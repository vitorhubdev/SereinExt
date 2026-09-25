//! Worker-owned speech processing; never runs in device callbacks.
use model::voice_settings::{NoiseSuppression, Processing, VoiceProcessing};
use nnnoiseless::DenoiseState;
use sonora::{
	AudioProcessing, Config, StreamConfig,
	config::{AdaptiveDigital, EchoCanceller, GainController2, NoiseSuppressionLevel},
};
use std::{
	sync::mpsc,
	time::{Duration, Instant},
};

/// Share of each 10 ms frame DeepFilterNet may spend before the machine counts as too slow.
const DEEP_BUDGET: f64 = 0.6;
/// Consecutive slow seconds tolerated, so one scheduler hiccup does not switch models.
const DEEP_SLOW_SECONDS: u8 = 3;
const FRAMES_PER_SECOND: u32 = 100;

#[derive(Default)]
struct Load {
	busy: Duration,
	frames: u32,
	slow_seconds: u8,
	warmed_up: bool,
}
impl Load {
	/// Records one 10 ms frame's processing time; false once the budget is exceeded for long.
	fn record(&mut self, elapsed: Duration) -> bool {
		self.busy += elapsed;
		self.frames += 1;
		if self.frames < FRAMES_PER_SECOND {
			return true;
		}
		let load = self.busy.as_secs_f64() / (f64::from(self.frames) * 0.010);
		self.busy = Duration::ZERO;
		self.frames = 0;
		// The first second includes tract's plan warm-up and is not representative.
		if !std::mem::replace(&mut self.warmed_up, true) {
			return true;
		}
		self.slow_seconds = if load > DEEP_BUDGET {
			self.slow_seconds + 1
		} else {
			0
		};
		self.slow_seconds < DEEP_SLOW_SECONDS
	}
}

fn deep_model() -> Option<df::tract::DfTract> {
	let params = df::tract::DfParams::default();
	let runtime = df::tract::RuntimeParams::default_with_ch(1);
	let model = df::tract::DfTract::new(params, &runtime).ok()?;
	(model.hop_size == 480 && model.sr == 48_000).then_some(model)
}

fn deep_frame(model: &mut df::tract::DfTract, chunk: [f32; 480]) -> Option<[f32; 480]> {
	let mut output = [0.0f32; 480];
	let input = ndarray::ArrayView2::from_shape((1, 480), &chunk[..]).ok()?;
	let view = ndarray::ArrayViewMut2::from_shape((1, 480), &mut output[..]).ok()?;
	model.process(input, view).ok()?;
	Some(output)
}

enum Step {
	Loading,
	Done,
	Failed,
}

/// DeepFilterNet on its own thread: tract state is not `Send`, and loading takes long
/// enough that doing it on the audio worker would drop call audio.
struct Deep {
	requests: mpsc::SyncSender<[f32; 480]>,
	responses: mpsc::Receiver<Option<[f32; 480]>>,
	loaded: mpsc::Receiver<bool>,
	ready: bool,
	load: Load,
}
impl Deep {
	fn start() -> Option<Self> {
		let (requests, incoming) = mpsc::sync_channel::<[f32; 480]>(1);
		let (reply, responses) = mpsc::sync_channel(1);
		let (report, loaded) = mpsc::sync_channel(1);
		std::thread::Builder::new()
			.name("voice-deepfilter".into())
			.spawn(move || {
				let Some(mut model) = deep_model() else {
					let _ = report.send(false);
					return;
				};
				let _ = report.send(true);
				while let Ok(chunk) = incoming.recv() {
					if reply.send(deep_frame(&mut model, chunk)).is_err() {
						break;
					}
				}
			})
			.ok()?;
		Some(Self {
			requests,
			responses,
			loaded,
			ready: false,
			load: Load::default(),
		})
	}
	fn process(&mut self, chunk: &mut [f32; 480]) -> Step {
		if !self.ready {
			match self.loaded.try_recv() {
				Ok(true) => self.ready = true,
				Err(mpsc::TryRecvError::Empty) => return Step::Loading,
				Ok(false) | Err(mpsc::TryRecvError::Disconnected) => return Step::Failed,
			}
		}
		let start = Instant::now();
		if self.requests.send(*chunk).is_err() {
			return Step::Failed;
		}
		match self.responses.recv_timeout(Duration::from_millis(50)) {
			Ok(Some(output)) => *chunk = output,
			_ => return Step::Failed,
		}
		if self.load.record(start.elapsed()) {
			Step::Done
		} else {
			Step::Failed
		}
	}
}

pub struct Echo {
	processor: AudioProcessing,
	gain: Option<AudioProcessing>,
	noise: Option<Box<DenoiseState<'static>>>,
	deep: Option<Deep>,
	/// Set when DeepFilterNet was chosen but could not load or keep up; RNNoise runs instead.
	deep_fallback: bool,
	settings: Processing,
}

fn processor(config: Config) -> AudioProcessing {
	AudioProcessing::builder()
		.config(config)
		.capture_config(StreamConfig::new(48_000, 1))
		.render_config(StreamConfig::new(48_000, 1))
		.build()
}
fn noise_state() -> Box<DenoiseState<'static>> {
	let mut noise = DenoiseState::new();
	noise.process_frame(&mut [0.0; 480], &[0.0; 480]);
	noise
}
impl Echo {
	pub fn new() -> Self {
		Self {
			processor: processor(Config {
				echo_canceller: Some(EchoCanceller::default()),
				..Default::default()
			}),
			gain: None,
			noise: None,
			deep: None,
			deep_fallback: false,
			settings: VoiceProcessing::from_legacy(false).effective(),
		}
	}

	/// DeepFilterNet was selected but this machine could not load it or keep up.
	pub fn deep_fallback(&self) -> bool {
		self.deep_fallback
	}

	fn sync_rnnoise(&mut self) {
		let wanted = match self.settings.suppression {
			NoiseSuppression::RnNoise => true,
			// RNNoise covers the model's load time as well as a fallback.
			NoiseSuppression::DeepFilter => !self.deep.as_ref().is_some_and(|deep| deep.ready),
			NoiseSuppression::Off | NoiseSuppression::WebRtc => false,
		};
		if wanted && self.noise.is_none() {
			self.noise = Some(noise_state());
		} else if !wanted {
			self.noise = None;
		}
	}

	pub fn configure(&mut self, settings: Processing) -> Result<(), &'static str> {
		if !settings.is_valid() {
			return Err("Invalid microphone processing settings");
		}
		if settings == self.settings {
			return Ok(());
		}
		if settings.suppression != NoiseSuppression::DeepFilter {
			self.deep = None;
			self.deep_fallback = false;
		} else if self.deep.is_none() && !self.deep_fallback {
			self.deep = Deep::start();
			self.deep_fallback = self.deep.is_none();
		}
		if settings.echo_cancellation != self.settings.echo_cancellation
			|| (settings.suppression == NoiseSuppression::WebRtc)
				!= (self.settings.suppression == NoiseSuppression::WebRtc)
			|| settings.suppression_level != self.settings.suppression_level
		{
			self.processor.apply_config(Config {
				echo_canceller: settings.echo_cancellation.then(EchoCanceller::default),
				noise_suppression: (settings.suppression == NoiseSuppression::WebRtc).then(|| {
					sonora::config::NoiseSuppression {
						level: match settings.suppression_level {
							0 => NoiseSuppressionLevel::Low,
							1 => NoiseSuppressionLevel::Moderate,
							2 => NoiseSuppressionLevel::High,
							_ => NoiseSuppressionLevel::VeryHigh,
						},
						..Default::default()
					}
				}),
				..Default::default()
			});
		}
		if settings.automatic_gain && self.gain.is_none() {
			// Digital-only AGC runs after the chosen denoiser. It never changes OS mic gain.
			self.gain = Some(processor(Config {
				gain_controller2: Some(GainController2 {
					adaptive_digital: Some(AdaptiveDigital {
						max_gain_db: 20.0,
						initial_gain_db: 0.0,
						..Default::default()
					}),
					..Default::default()
				}),
				..Default::default()
			}));
		} else if !settings.automatic_gain {
			self.gain = None;
		}
		self.settings = settings;
		self.sync_rnnoise();
		Ok(())
	}

	pub fn reset(&mut self) {
		let config = StreamConfig::new(48_000, 1);
		self.processor.initialize(config, config, config, config);
		if let Some(gain) = &mut self.gain {
			gain.initialize(config, config, config, config);
		}
		if self.noise.is_some() {
			self.noise = Some(noise_state());
		}
	}

	pub fn render(&mut self, frame: &[f32; 960]) -> Result<(), &'static str> {
		if !self.settings.echo_cancellation {
			return Ok(());
		}
		let mut output = [0.0; 480];
		for chunk in frame.as_chunks::<480>().0 {
			self.processor
				.process_render_f32(&[chunk], &mut [&mut output])
				.map_err(|_| "Echo cancellation could not process speaker audio")?;
		}
		Ok(())
	}

	pub fn capture(
		&mut self,
		frame: &mut [f32; 960],
		time_noise: bool,
	) -> Result<Duration, &'static str> {
		for sample in frame.iter_mut() {
			*sample = if sample.is_finite() {
				sample.clamp(-1.0, 1.0)
			} else {
				0.0
			};
		}
		let mut noise_time = Duration::ZERO;
		for chunk in frame.as_chunks_mut::<480>().0 {
			let mut output = *chunk;
			if self.settings.echo_cancellation
				|| self.settings.suppression == NoiseSuppression::WebRtc
			{
				// AEC3 estimates render/capture delay internally. Forcing 0 ms here
				// misrepresents the real device/callback buffering and can destabilize
				// cancellation on otherwise healthy audio paths.
				self.processor
					.process_capture_f32(&[chunk], &mut [&mut output])
					.map_err(|_| "Microphone processing failed")?;
			}
			let start = time_noise.then(Instant::now);
			match self.deep.as_mut().map(|deep| deep.process(&mut output)) {
				Some(Step::Done) => {
					if let Some(start) = start {
						noise_time += start.elapsed();
					}
					if self.noise.is_some() {
						self.sync_rnnoise();
					}
				}
				Some(Step::Failed) => {
					self.deep = None;
					self.deep_fallback = true;
					self.sync_rnnoise();
				}
				Some(Step::Loading) | None => {}
			}
			if let Some(noise) = &mut self.noise
				&& !self.deep.as_ref().is_some_and(|deep| deep.ready)
			{
				let start = time_noise.then(Instant::now);
				let input = output.map(|s| (s * 32768.0).clamp(-32768.0, 32767.0));
				noise.process_frame(&mut output, &input);
				for sample in &mut output {
					*sample = (*sample / 32768.0).clamp(-1.0, 1.0);
				}
				if let Some(start) = start {
					noise_time += start.elapsed();
				}
			}
			chunk.copy_from_slice(&output);
		}
		if let Some(gain) = &mut self.gain {
			for chunk in frame.as_chunks_mut::<480>().0 {
				let mut output = [0.0; 480];
				gain.process_capture_f32(&[chunk], &mut [&mut output])
					.map_err(|_| "Automatic gain processing failed")?;
				chunk.copy_from_slice(&output);
			}
		}
		for sample in frame {
			*sample = if sample.is_finite() {
				sample.clamp(-1.0, 1.0)
			} else {
				0.0
			};
		}
		Ok(noise_time)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn second(load: &mut Load, per_frame: Duration) -> bool {
		(0..FRAMES_PER_SECOND).all(|_| load.record(per_frame))
	}

	#[test]
	fn deep_load_tolerates_warm_up_and_short_spikes() {
		let mut load = Load::default();
		assert!(
			second(&mut load, Duration::from_millis(9)),
			"warm-up second is ignored"
		);
		assert!(second(&mut load, Duration::from_millis(9)));
		assert!(second(&mut load, Duration::from_millis(9)));
		assert!(
			second(&mut load, Duration::from_millis(2)),
			"a fast second clears the streak"
		);
		assert!(second(&mut load, Duration::from_millis(9)));
		assert!(second(&mut load, Duration::from_millis(9)));
		assert!(
			!second(&mut load, Duration::from_millis(9)),
			"three slow seconds give up"
		);
	}

	#[test]
	fn deep_filter_denoises_and_falls_back_to_rnnoise() {
		let mut seed = 0x2545_f491_u32;
		let mut noisy = |index: usize| -> [f32; 960] {
			std::array::from_fn(|i| {
				seed ^= seed << 13;
				seed ^= seed >> 17;
				seed ^= seed << 5;
				let hiss = (seed as f32 / u32::MAX as f32 - 0.5) * 0.05;
				hiss + ((index * 960 + i) as f32 * 0.03).sin() * 0.2
			})
		};
		let started = Instant::now();
		let mut dsp = Echo::new();
		dsp.configure(Processing {
			suppression: NoiseSuppression::DeepFilter,
			..Processing::default()
		})
		.unwrap();
		assert!(
			started.elapsed() < Duration::from_millis(100),
			"loading must not block audio"
		);
		assert!(dsp.noise.is_some(), "RNNoise covers the model's load time");
		while !dsp.deep.as_ref().is_some_and(|deep| deep.ready) {
			assert!(started.elapsed() < Duration::from_secs(60) && !dsp.deep_fallback());
			dsp.capture(&mut noisy(0), false).unwrap();
			std::thread::sleep(Duration::from_millis(10));
		}
		let load_time = started.elapsed();
		assert!(
			dsp.noise.is_none(),
			"RNNoise stops once DeepFilterNet is ready"
		);
		let started = Instant::now();
		for index in 0..200 {
			let mut frame = noisy(index);
			dsp.capture(&mut frame, false).unwrap();
			assert!(frame.iter().all(|s| s.is_finite() && s.abs() <= 1.0));
		}
		let per_frame = started.elapsed() / 400;
		println!("DeepFilterNet ready after {load_time:?}, {per_frame:?} per 10 ms frame");
		assert!(
			!dsp.deep_fallback(),
			"two seconds cannot trip the three-second guard"
		);

		dsp.deep = None;
		dsp.deep_fallback = true;
		dsp.sync_rnnoise();
		assert!(
			dsp.noise.is_some(),
			"fallback keeps suppression on with RNNoise"
		);
		dsp.configure(Processing::default()).unwrap();
		assert!(!dsp.deep_fallback() && dsp.noise.is_some());
	}
}
