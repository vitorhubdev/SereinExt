//! Small built-in locale catalog. Unknown keys deliberately fall back to English so
//! incremental translation cannot make a control disappear or become unusable.
use model::Language;

pub fn text(language: Language, english: &'static str) -> &'static str {
	match language {
		Language::English => english,
		Language::PortugueseBrazil => portuguese_brazil(english).unwrap_or(english),
		Language::Spanish => spanish(english).unwrap_or(english),
	}
}

fn portuguese_brazil(key: &str) -> Option<&'static str> {
	Some(match key {
		"User settings" => "Configurações do usuário",
		"App settings" => "Configurações do aplicativo",
		"Customization" => "Personalização",
		"My Account" => "Minha conta",
		"Profile" => "Perfil",
		"General" => "Geral",
		"Appearance" => "Aparência",
		"Chat" => "Conversas",
		"Messaging Permissions" => "Permissões de mensagens",
		"Notifications" => "Notificações",
		"Game Activity" => "Atividade de jogos",
		"Voice & Video" => "Voz e vídeo",
		"Keybinds" => "Atalhos de teclado",
		"Data & Privacy" => "Dados e privacidade",
		"Updates" => "Atualizações",
		"Extensions" => "Extensões",
		"Themes" => "Temas",
		"The Discord account signed in on this device." => "A conta do Discord conectada neste dispositivo.",
		"Choose how you appear across Discord." => "Escolha como você aparece no Discord.",
		"Startup, window and graphics behavior on this device." => "Inicialização, janela e gráficos neste dispositivo.",
		"Theme, colours, window effects and layout." => "Tema, cores, efeitos da janela e layout.",
		"How messages, media, links and scrolling behave." => "Como mensagens, mídia, links e rolagem se comportam.",
		"Control who can contact you and how messages are filtered." => "Controle quem pode falar com você e como as mensagens são filtradas.",
		"Choose which notifications you receive and how they appear." => "Escolha quais notificações você recebe e como elas aparecem.",
		"Show others what you are playing." => "Mostre aos outros o que você está jogando.",
		"Microphone, speakers, camera and voice processing." => "Microfone, alto-falantes, câmera e processamento de voz.",
		"Keyboard shortcuts for Serein." => "Atalhos de teclado do Serein.",
		"What Serein keeps on this device." => "O que o Serein mantém neste dispositivo.",
		"Keep Serein up to date on this device." => "Mantenha o Serein atualizado neste dispositivo.",
		"Manage community plugins." => "Gerencie plugins da comunidade.",
		"Choose a community theme." => "Escolha um tema da comunidade.",
		"Language" => "Idioma",
		"App language" => "Idioma do aplicativo",
		"Changes apply immediately and are saved on this device." => "As alterações são aplicadas imediatamente e salvas neste dispositivo.",
		"Startup" => "Inicialização",
		"Open Serein when your computer starts" => "Abrir o Serein ao iniciar o computador",
		"Serein signs in and connects in the background." => "O Serein entra na conta e conecta em segundo plano.",
		"Start minimized" => "Iniciar minimizado",
		"Start in the background, out of your way." => "Iniciar em segundo plano, sem ocupar a tela.",
		"Automatic startup is available on Windows and macOS." => "A inicialização automática está disponível no Windows e macOS.",
		"Window" => "Janela",
		"Hide Serein title bar" => "Ocultar a barra de título do Serein",
		"Use the system title bar and window buttons instead." => "Use a barra de título e os botões de janela do sistema.",
		"Keep Serein in the menu bar" => "Manter o Serein na barra de menus",
		"Keep Serein in the system tray" => "Manter o Serein na bandeja do sistema",
		"The tray is unavailable on this platform." => "A bandeja do sistema não está disponível nesta plataforma.",
		"Graphics" => "Gráficos",
		"Render with" => "Renderizar com",
		"No settings found" => "Nenhuma configuração encontrada",
		"Try theme, notifications, voice, or cache." => "Tente tema, notificações, voz ou cache.",
		_ => return None,
	})
}

fn spanish(key: &str) -> Option<&'static str> {
	Some(match key {
		"User settings" => "Ajustes de usuario",
		"App settings" => "Ajustes de la aplicación",
		"Customization" => "Personalización",
		"My Account" => "Mi cuenta",
		"Profile" => "Perfil",
		"General" => "General",
		"Appearance" => "Apariencia",
		"Chat" => "Chat",
		"Messaging Permissions" => "Permisos de mensajería",
		"Notifications" => "Notificaciones",
		"Game Activity" => "Actividad de juegos",
		"Voice & Video" => "Voz y video",
		"Keybinds" => "Atajos de teclado",
		"Data & Privacy" => "Datos y privacidad",
		"Updates" => "Actualizaciones",
		"Extensions" => "Extensiones",
		"Themes" => "Temas",
		"The Discord account signed in on this device." => "La cuenta de Discord conectada en este dispositivo.",
		"Choose how you appear across Discord." => "Elige cómo apareces en Discord.",
		"Startup, window and graphics behavior on this device." => "Inicio, ventana y gráficos en este dispositivo.",
		"Theme, colours, window effects and layout." => "Tema, colores, efectos de ventana y diseño.",
		"How messages, media, links and scrolling behave." => "Cómo se comportan los mensajes, medios, enlaces y desplazamiento.",
		"Control who can contact you and how messages are filtered." => "Controla quién puede contactarte y cómo se filtran los mensajes.",
		"Choose which notifications you receive and how they appear." => "Elige qué notificaciones recibes y cómo aparecen.",
		"Show others what you are playing." => "Muestra a los demás a qué estás jugando.",
		"Microphone, speakers, camera and voice processing." => "Micrófono, altavoces, cámara y procesamiento de voz.",
		"Keyboard shortcuts for Serein." => "Atajos de teclado de Serein.",
		"What Serein keeps on this device." => "Lo que Serein guarda en este dispositivo.",
		"Keep Serein up to date on this device." => "Mantén Serein actualizado en este dispositivo.",
		"Manage community plugins." => "Administra plugins de la comunidad.",
		"Choose a community theme." => "Elige un tema de la comunidad.",
		"Language" => "Idioma",
		"App language" => "Idioma de la aplicación",
		"Changes apply immediately and are saved on this device." => "Los cambios se aplican inmediatamente y se guardan en este dispositivo.",
		"Startup" => "Inicio",
		"Open Serein when your computer starts" => "Abrir Serein al iniciar el equipo",
		"Serein signs in and connects in the background." => "Serein inicia sesión y se conecta en segundo plano.",
		"Start minimized" => "Iniciar minimizado",
		"Start in the background, out of your way." => "Iniciar en segundo plano, sin ocupar la pantalla.",
		"Automatic startup is available on Windows and macOS." => "El inicio automático está disponible en Windows y macOS.",
		"Window" => "Ventana",
		"Hide Serein title bar" => "Ocultar la barra de título de Serein",
		"Use the system title bar and window buttons instead." => "Usa la barra de título y los botones de ventana del sistema.",
		"Keep Serein in the menu bar" => "Mantener Serein en la barra de menús",
		"Keep Serein in the system tray" => "Mantener Serein en la bandeja del sistema",
		"The tray is unavailable on this platform." => "La bandeja del sistema no está disponible en esta plataforma.",
		"Graphics" => "Gráficos",
		"Render with" => "Renderizar con",
		"No settings found" => "No se encontraron ajustes",
		"Try theme, notifications, voice, or cache." => "Prueba tema, notificaciones, voz o caché.",
		_ => return None,
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn locales_fall_back_to_english_without_empty_controls() {
		assert_eq!(text(Language::PortugueseBrazil, "Voice & Video"), "Voz e vídeo");
		assert_eq!(text(Language::Spanish, "Voice & Video"), "Voz y video");
		assert_eq!(text(Language::Spanish, "Untranslated sentinel"), "Untranslated sentinel");
	}
}
