//! Ephemeral, provider-bounded web media player used from chat embeds.
//! It never shares Discord credentials, accepts downloads, or grants device permissions.
use std::sync::Arc;

pub const HEADER_HEIGHT: f32 = 56.0;

fn safe_url(value: &str) -> Option<url::Url> {
	let url = url::Url::parse(value).ok()?;
	if url.scheme() != "https"
		|| !url.username().is_empty()
		|| url.password().is_some()
		|| url.port().is_some_and(|port| port != 443)
	{
		return None;
	}
	Some(url)
}

fn video_id(value: &str) -> Option<&str> {
	(!value.is_empty()
		&& value.len() <= 32
		&& value
			.bytes()
			.all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_')))
	.then_some(value)
}

pub fn normalize(value: &str) -> Option<String> {
	let url = safe_url(value)?;
	let host = url.host_str()?.trim_start_matches("www.").to_ascii_lowercase();
	match host.as_str() {
		"youtu.be" => {
			let id = video_id(url.path_segments()?.next()?)?;
			Some(format!("https://www.youtube-nocookie.com/embed/{id}?autoplay=1"))
		}
		"youtube.com" | "m.youtube.com" => {
			let id = if url.path() == "/watch" {
				url.query_pairs().find_map(|(key, value)| {
					(key == "v").then(|| value.into_owned())
				})?
			} else {
				let mut parts = url.path_segments()?;
				let kind = parts.next()?;
				if !matches!(kind, "shorts" | "embed") {
					return None;
				}
				parts.next()?.to_owned()
			};
			let id = video_id(&id)?;
			Some(format!("https://www.youtube-nocookie.com/embed/{id}?autoplay=1"))
		}
		"x.com" | "twitter.com" => {
			let parts: Vec<_> = url.path_segments()?.filter(|part| !part.is_empty()).take(4).collect();
			if parts.len() < 3 || parts[1] != "status" || !parts[2].bytes().all(|b| b.is_ascii_digit()) {
				return None;
			}
			Some(format!("https://x.com/{}/status/{}", parts[0], parts[2]))
		}
		"vimeo.com" => {
			let id = url.path_segments()?.find(|part| part.bytes().all(|b| b.is_ascii_digit()))?;
			Some(format!("https://player.vimeo.com/video/{id}?autoplay=1"))
		}
		"player.vimeo.com" => {
			let mut parts = url.path_segments()?;
			if parts.next()? != "video" {
				return None;
			}
			let id = parts.next()?;
			if !id.bytes().all(|b| b.is_ascii_digit()) {
				return None;
			}
			Some(format!("https://player.vimeo.com/video/{id}?autoplay=1"))
		}
		_ => None,
	}
}

fn navigation_allowed(value: &str) -> bool {
	let Some(url) = safe_url(value) else { return false };
	let Some(host) = url.host_str().map(|host| host.to_ascii_lowercase()) else {
		return false;
	};
	matches!(
		host.as_str(),
		"youtube.com"
			| "www.youtube.com"
			| "m.youtube.com"
			| "youtube-nocookie.com"
			| "www.youtube-nocookie.com"
			| "x.com"
			| "www.x.com"
			| "twitter.com"
			| "www.twitter.com"
			| "vimeo.com"
			| "www.vimeo.com"
			| "player.vimeo.com"
	)
}

#[cfg(not(target_os = "linux"))]
pub struct WebMediaView {
	view: wry::WebView,
}
#[cfg(not(target_os = "linux"))]
impl WebMediaView {
	pub fn open(
		parent: Arc<winit::window::Window>,
		value: &str,
		_wake: impl Fn() + Send + Sync + 'static,
	) -> Result<Self, &'static str> {
		let url = normalize(value).ok_or("Unsupported web video provider or URL")?;
		let view = wry::WebViewBuilder::new()
			.with_url(&url)
			.with_incognito(true)
			.with_visible(true)
			.with_focused(true)
			.with_autoplay(true)
			.with_devtools(false)
			.with_permission_handler(|kind| {
				if matches!(kind, wry::PermissionKind::Autoplay) {
					wry::PermissionResponse::Allow
				} else {
					wry::PermissionResponse::Deny
				}
			})
			.with_navigation_handler(|url| navigation_allowed(&url))
			.with_new_window_req_handler(|_, _| wry::NewWindowResponse::Deny)
			.with_download_started_handler(|_, _| false)
			.with_bounds(bounds(&parent))
			.build_as_child(parent.as_ref())
			.map_err(|_| "Could not create the isolated web media player")?;
		view.set_visible(true)
			.map_err(|_| "Could not show the web media player")?;
		let _ = view.focus();
		Ok(Self { view })
	}
	pub fn embedded(&self) -> bool { true }
	pub fn closed(&self) -> bool { false }
	pub fn pump(&self) {}
	pub fn resize(&self, parent: &winit::window::Window) {
		let _ = self.view.set_bounds(bounds(parent));
	}
}
#[cfg(not(target_os = "linux"))]
fn bounds(parent: &winit::window::Window) -> wry::Rect {
	let size = parent.inner_size();
	let header = (HEADER_HEIGHT as f64 * parent.scale_factor()).round() as u32;
	wry::Rect {
		position: wry::dpi::PhysicalPosition::new(0, header as i32).into(),
		size: wry::dpi::PhysicalSize::new(size.width, size.height.saturating_sub(header)).into(),
	}
}

#[cfg(target_os = "linux")]
pub struct WebMediaView {
	view: webkit6::WebView,
	window: gtk4::Window,
	display: gtk4::gdk::Display,
	closed: std::rc::Rc<std::cell::Cell<bool>>,
}
#[cfg(target_os = "linux")]
impl WebMediaView {
	pub fn open(
		_parent: Arc<winit::window::Window>,
		value: &str,
		wake: impl Fn() + Send + Sync + 'static,
	) -> Result<Self, &'static str> {
		use webkit6::{glib, prelude::*};
		gtk4::init().map_err(|_| "Linux web media window unavailable")?;
		crate::ensure_gtk_application_id();
		let url = normalize(value).ok_or("Unsupported web video provider or URL")?;
		let session = webkit6::NetworkSession::new_ephemeral();
		session.set_persistent_credential_storage_enabled(false);
		session.set_tls_errors_policy(webkit6::TLSErrorsPolicy::Fail);
		session.connect_download_started(|_, download| download.cancel());
		let settings = webkit6::Settings::new();
		settings.set_enable_developer_extras(false);
		settings.set_enable_write_console_messages_to_stdout(false);
		settings.set_javascript_can_open_windows_automatically(false);
		settings.set_javascript_can_access_clipboard(false);
		settings.set_enable_media_stream(false);
		settings.set_enable_webrtc(false);
		let view = webkit6::WebView::builder()
			.network_session(&session)
			.settings(&settings)
			.build();
		view.set_hexpand(true);
		view.set_vexpand(true);
		view.connect_decide_policy(|_, decision, kind| {
			let allowed = match kind {
				webkit6::PolicyDecisionType::NavigationAction => decision
					.downcast_ref::<webkit6::NavigationPolicyDecision>()
					.and_then(|decision| decision.navigation_action())
					.and_then(|action| action.request())
					.and_then(|request| request.uri())
					.is_some_and(|uri| navigation_allowed(&uri)),
				webkit6::PolicyDecisionType::Response => decision
					.downcast_ref::<webkit6::ResponsePolicyDecision>()
					.is_some_and(|response| {
						response.is_mime_type_supported()
							&& (!response.is_main_frame_main_resource()
								|| response
									.request()
									.and_then(|request| request.uri())
									.is_some_and(|uri| navigation_allowed(&uri)))
					}),
				_ => false,
			};
			if allowed { decision.use_(); } else { decision.ignore(); }
			true
		});
		view.connect_create(|_, _| None);
		view.connect_permission_request(|_, request| {
			request.deny();
			true
		});
		view.connect_run_file_chooser(|_, request| {
			request.cancel();
			true
		});
		view.connect_context_menu(|_, _, _| true);
		let window = gtk4::Window::builder()
			.title("Web media preview · SereinExt")
			.default_width(960)
			.default_height(720)
			.child(&view)
			.build();
		let closed = std::rc::Rc::new(std::cell::Cell::new(false));
		let closed_request = closed.clone();
		let weak_view = view.downgrade();
		let wake: Arc<dyn Fn() + Send + Sync> = Arc::new(wake);
		let close_wake = wake.clone();
		window.connect_close_request(move |_| {
			closed_request.set(true);
			if let Some(view) = weak_view.upgrade() {
				view.stop_loading();
				view.terminate_web_process();
			}
			close_wake();
			glib::Propagation::Proceed
		});
		let crashed = closed.clone();
		let crash_wake = wake.clone();
		view.connect_web_process_terminated(move |_, _| {
			crashed.set(true);
			crash_wake();
		});
		window.present();
		view.grab_focus();
		view.load_uri(&url);
		let display = gtk4::prelude::WidgetExt::display(&window);
		Ok(Self { view, window, display, closed })
	}
	pub fn embedded(&self) -> bool { false }
	pub fn closed(&self) -> bool { self.closed.get() }
	pub fn resize(&self, _parent: &winit::window::Window) {}
	pub fn pump(&self) {
		use webkit6::glib;
		let context = glib::MainContext::default();
		let started = std::time::Instant::now();
		for _ in 0..16 {
			if started.elapsed() >= std::time::Duration::from_millis(2) || !context.pending() {
				break;
			}
			context.iteration(false);
		}
		use webkit6::prelude::*;
		self.display.flush();
	}
}
#[cfg(target_os = "linux")]
impl Drop for WebMediaView {
	fn drop(&mut self) {
		use webkit6::prelude::*;
		self.view.stop_loading();
		self.view.terminate_web_process();
		self.window.set_child(None::<&gtk4::Widget>);
		self.window.destroy();
		use webkit6::prelude::*;
		self.display.flush();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn normalizes_supported_media_without_open_redirects() {
		assert_eq!(
			normalize("https://youtu.be/dQw4w9WgXcQ?si=tracking").as_deref(),
			Some("https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ?autoplay=1")
		);
		assert_eq!(
			normalize("https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=ignored").as_deref(),
			Some("https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ?autoplay=1")
		);
		assert_eq!(
			normalize("https://x.com/user/status/123?utm_source=ignored").as_deref(),
			Some("https://x.com/user/status/123")
		);
		assert!(normalize("https://youtube.com.evil.test/watch?v=dQw4w9WgXcQ").is_none());
		assert!(normalize("http://x.com/user/status/123").is_none());
	}
}
