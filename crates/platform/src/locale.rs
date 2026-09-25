//! Operating-system UI language, limited to languages the app can show.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiLanguage {
	English,
	Portuguese,
	Spanish,
}

pub fn ui_language() -> UiLanguage {
	language_from_tag(&ui_language_tag())
}

pub(crate) fn language_from_tag(tag: &str) -> UiLanguage {
	let tag = tag.trim().to_ascii_lowercase();
	if tag.starts_with("pt") {
		UiLanguage::Portuguese
	} else if tag.starts_with("es") {
		UiLanguage::Spanish
	} else {
		UiLanguage::English
	}
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn ui_language_tag() -> String {
	#[link(name = "kernel32")]
	unsafe extern "system" {
		fn GetUserDefaultUILanguage() -> u16;
	}
	// SAFETY: GetUserDefaultUILanguage takes no pointers and returns a LANGID.
	let id = unsafe { GetUserDefaultUILanguage() };
	match id & 0x3ff {
		0x16 => "pt".to_owned(),
		0x0a => "es".to_owned(),
		_ => "en".to_owned(),
	}
}

#[cfg(not(windows))]
fn ui_language_tag() -> String {
	std::env::var("LC_ALL")
		.or_else(|_| std::env::var("LANG"))
		.unwrap_or_default()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn tags_map_to_supported_languages() {
		assert_eq!(language_from_tag("pt-BR"), UiLanguage::Portuguese);
		assert_eq!(language_from_tag("es-MX"), UiLanguage::Spanish);
		assert_eq!(language_from_tag("en-US"), UiLanguage::English);
		assert_eq!(language_from_tag("fr-FR"), UiLanguage::English);
	}
}
