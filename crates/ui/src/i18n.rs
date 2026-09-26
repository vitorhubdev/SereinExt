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
		"The Discord account signed in on this device." => {
			"A conta do Discord conectada neste dispositivo."
		}
		"Choose how you appear across Discord." => "Escolha como você aparece no Discord.",
		"Startup, window and graphics behavior on this device." => {
			"Inicialização, janela e gráficos neste dispositivo."
		}
		"Theme, colours, window effects and layout." => "Tema, cores, efeitos da janela e layout.",
		"How messages, media, links and scrolling behave." => {
			"Como mensagens, mídia, links e rolagem se comportam."
		}
		"Control who can contact you and how messages are filtered." => {
			"Controle quem pode falar com você e como as mensagens são filtradas."
		}
		"Choose which notifications you receive and how they appear." => {
			"Escolha quais notificações você recebe e como elas aparecem."
		}
		"Show others what you are playing." => "Mostre aos outros o que você está jogando.",
		"Microphone, speakers, camera and voice processing." => {
			"Microfone, alto-falantes, câmera e processamento de voz."
		}
		"Keyboard shortcuts for SereinExt." => "Atalhos de teclado do SereinExt.",
		"What SereinExt keeps on this device." => "O que o SereinExt mantém neste dispositivo.",
		"Keep SereinExt up to date on this device." => {
			"Mantenha o SereinExt atualizado neste dispositivo."
		}
		"Manage community plugins." => "Gerencie plugins da comunidade.",
		"Choose a community theme." => "Escolha um tema da comunidade.",
		"Language" => "Idioma",
		"App language" => "Idioma do aplicativo",
		"Changes apply immediately and are saved on this device." => {
			"As alterações são aplicadas imediatamente e salvas neste dispositivo."
		}
		"Startup" => "Inicialização",
		"Open SereinExt when your computer starts" => "Abrir o SereinExt ao iniciar o computador",
		"SereinExt signs in and connects in the background." => {
			"O SereinExt entra na conta e conecta em segundo plano."
		}
		"Start minimized" => "Iniciar minimizado",
		"Start in the background, out of your way." => {
			"Iniciar em segundo plano, sem ocupar a tela."
		}
		"Automatic startup is available on Windows and macOS." => {
			"A inicialização automática está disponível no Windows e macOS."
		}
		"Window" => "Janela",
		"Hide SereinExt title bar" => "Ocultar a barra de título do SereinExt",
		"Use the system title bar and window buttons instead." => {
			"Use a barra de título e os botões de janela do sistema."
		}
		"Keep SereinExt in the menu bar" => "Manter o SereinExt na barra de menus",
		"Keep SereinExt in the system tray" => "Manter o SereinExt na bandeja do sistema",
		"The tray is unavailable on this platform." => {
			"A bandeja do sistema não está disponível nesta plataforma."
		}
		"Graphics" => "Gráficos",
		"Render with" => "Renderizar com",
		"Search" => "Buscar",
		"Close settings (Esc)" => "Fechar configurações (Esc)",
		"Unofficial · not endorsed by Discord" => "Não oficial · não endossado pelo Discord",
		"Exit preview" => "Sair da prévia",
		"Log out" => "Sair da conta",
		"Your account" => "Sua conta",
		"Offline preview · synthetic account" => "Prévia offline · conta sintética",
		"Signed in with your Discord account" => "Conectado com sua conta do Discord",
		"Display name" => "Nome de exibição",
		"Email, password and security" => "E-mail, senha e segurança",
		"Managed in Discord" => "Gerenciado no Discord",
		"Edit profile" => "Editar perfil",
		"Session" => "Sessão",
		"Closes the offline fixture. Nothing is stored for the preview." => {
			"Fecha a prévia offline. Nada é armazenado para a prévia."
		}
		"Removes the saved login and clears this account's local cache and drafts." => {
			"Remove o login salvo e limpa o cache local e os rascunhos desta conta."
		}
		"Theme" => "Tema",
		"Accent" => "Cor de destaque",
		"Primary color" => "Cor primária",
		"The active theme brings its own accent; it takes over while the theme is in use." => {
			"O tema ativo traz sua própria cor de destaque e ela é usada enquanto o tema estiver ativo."
		}
		"Used for buttons, selection and message highlights." => {
			"Usada em botões, seleção e destaques de mensagens."
		}
		"Reset" => "Redefinir",
		"Choose primary color" => "Escolher cor primária",
		"Window effects" => "Efeitos da janela",
		"Transparency & blur" => "Transparência e desfoque",
		"Restart SereinExt after changing this. Themes can customize effects while enabled." => {
			"Reinicie o SereinExt após alterar isto. Temas podem personalizar os efeitos enquanto estiverem ativos."
		}
		"Transparency" => "Transparência",
		"Blur" => "Desfoque",
		"Zero disables blur; the native compositor controls its exact strength." => {
			"Zero desativa o desfoque; o compositor nativo controla a intensidade exata."
		}
		"Apply to all surfaces" => "Aplicar a todas as superfícies",
		"Include sidebars, server rail, headers, and composer." => {
			"Inclui barras laterais, trilho de servidores, cabeçalhos e compositor."
		}
		"Channel list" => "Lista de canais",
		"Show hidden channels" => "Mostrar canais ocultos",
		"Show channels you cannot currently access." => {
			"Mostra canais aos quais você não tem acesso no momento."
		}
		"Colour preset" => "Predefinição de cores",
		"Share game activity" => "Compartilhar atividade de jogo",
		"Detect running games and ask Discord to share them as activity." => {
			"Detecta jogos em execução e pede ao Discord para compartilhá-los como atividade."
		}
		"Enable on Discord" => "Ativar no Discord",
		"Check again" => "Verificar novamente",
		"Looking for a running game" => "Procurando um jogo em execução",
		"Activity sharing is off" => "O compartilhamento de atividade está desativado",
		"Synthetic activity, never shared or saved." => {
			"Atividade sintética, nunca compartilhada nem salva."
		}
		"Local storage" => "Armazenamento local",
		"Clear cache" => "Limpar cache",
		"Removes cached messages and media. Drafts and your login stay." => {
			"Remove mensagens e mídias em cache. Rascunhos e seu login permanecem."
		}
		"Messages and drafts are cached on this device inside bounded, account-isolated files. Cache data is not encrypted by SereinExt; saved login tokens use the OS credential store." => {
			"Mensagens e rascunhos ficam em cache neste dispositivo em arquivos limitados e isolados por conta. Os dados de cache não são criptografados pelo SereinExt; tokens de login salvos usam o armazenamento de credenciais do sistema."
		}
		"Your privacy" => "Sua privacidade",
		"SereinExt does not collect telemetry or upload diagnostics. Discord retains service-side data according to its own policies." => {
			"O SereinExt não coleta telemetria nem envia diagnósticos. O Discord mantém dados do serviço de acordo com as próprias políticas."
		}
		"Offline preview · changes stay in this session and are never sent." => {
			"Prévia offline · as alterações ficam nesta sessão e nunca são enviadas."
		}
		"Closing the window keeps SereinExt in the menu bar. Quit from its menu to exit." => {
			"Fechar a janela mantém o SereinExt na barra de menus. Use o menu para encerrar."
		}
		"Closing keeps SereinExt running. Use the tray to show, minimize or quit." => {
			"Fechar mantém o SereinExt em execução. Use a bandeja para mostrar, minimizar ou encerrar."
		}
		"Closing the window keeps SereinExt in the notification area. Quit from its menu to exit." => {
			"Fechar a janela mantém o SereinExt na área de notificação. Use o menu para encerrar."
		}
		"Takes effect the next time SereinExt starts." => {
			"Entra em vigor na próxima vez que o SereinExt iniciar."
		}
		"Online" => "Online",
		"Idle" => "Ausente",
		"Do Not Disturb" => "Não perturbe",
		"Invisible" => "Invisível",
		"Don't clear" => "Não limpar",
		"30 minutes" => "30 minutos",
		"1 hour" => "1 hora",
		"4 hours" => "4 horas",
		"Today" => "Hoje",
		"Switch to" => "Alternar para",
		"Forget" => "Esquecer",
		"SereinExt clears it" => "O SereinExt limpa o status",
		"You" => "Você",
		"Custom status" => "Status personalizado",
		"Shown next to your name across Discord." => "Exibido ao lado do seu nome no Discord.",
		"Switch accounts" => "Alternar contas",
		"Add an account" => "Adicionar uma conta",
		"Forget this account on this device" => "Esquecer esta conta neste dispositivo",
		"Loading profile…" => "Carregando perfil…",
		"Reload profile" => "Recarregar perfil",
		"You will not receive desktop notifications" => {
			"Você não receberá notificações na área de trabalho"
		}
		"You will appear offline" => "Você aparecerá offline",
		"Edit custom status" => "Editar status personalizado",
		"Set a custom status" => "Definir status personalizado",
		"No custom status" => "Sem status personalizado",
		"Status text" => "Texto do status",
		"What's on your mind?" => "No que você está pensando?",
		"Clear after" => "Limpar depois",
		"Use up to 128 characters without control characters." => {
			"Use até 128 caracteres sem caracteres de controle."
		}
		"Clear" => "Limpar",
		"Apply" => "Aplicar",
		"No settings found" => "Nenhuma configuração encontrada",
		"Try theme, notifications, voice, or cache." => "Tente tema, notificações, voz ou cache.",
		"Use a different account" => "Usar outra conta",
		"Waiting for Discord…" => "Aguardando o Discord…",
		"Use another account" => "Usar outra conta",
		"Continue with Discord" => "Continuar com o Discord",
		"Welcome back" => "Bem-vindo de volta",
		"Welcome to SereinExt" => "Bem-vindo ao SereinExt",
		"Continue with a saved account, or sign in with another one." => {
			"Continue com uma conta salva ou entre com outra."
		}
		"Sign in with Discord." => "Entre com o Discord.",
		"Saved accounts" => "Contas salvas",
		"This is my account" => "Esta é a minha conta",
		"Check this to continue." => "Marque isto para continuar.",
		"Independent and open source. Not affiliated with Discord." => {
			"Independente e de código aberto. Não afiliado ao Discord."
		}
		"Message" => "Mensagem",
		"Unread messages" => "Mensagens não lidas",
		"Mark as read" => "Marcar como lida",
		"Jump to unread" => "Ir para não lidas",
		"Copy" => "Copiar",
		"Copy message" => "Copiar mensagem",
		"Copy download link" => "Copiar link de download",
		"Reply" => "Responder",
		"Forward" => "Encaminhar",
		"Forward message" => "Encaminhar mensagem",
		"Create Thread…" => "Criar tópico…",
		"View reactions" => "Ver reações",
		"Mark read through here" => "Marcar como lida até aqui",
		"Mark Unread" => "Marcar como não lida",
		"Unpin message" => "Desafixar mensagem",
		"Pin message" => "Fixar mensagem",
		"Edit message" => "Editar mensagem",
		"Remove from delete selection" => "Tirar da seleção",
		"Select for batch delete" => "Selecionar para apagar",
		"You can select up to 5 messages at a time." => {
			"Dá para selecionar até 5 mensagens por vez."
		}
		"Delete message…" => "Apagar mensagem…",
		"Delete message immediately" => "Apagar mensagem agora",
		"Message history is unavailable with current permission information." => {
			"O histórico não está disponível com as permissões atuais."
		}
		"Mute" => "Silenciar",
		"Unmute" => "Ativar microfone",
		"Deafen" => "Silenciar áudio",
		"Undeafen" => "Ativar áudio",
		"Disconnect" => "Desconectar",
		"Dismiss call" => "Fechar chamada",
		"Reconnect to call" => "Reconectar à chamada",
		"Recent call" => "Chamada recente",
		"You were in this call recently" => "Você estava nesta chamada recentemente",
		"Dismiss" => "Dispensar",
		"Share your screen" => "Compartilhar tela",
		"Stop sharing" => "Parar de compartilhar",
		"Turn on camera" => "Ligar câmera",
		"Turn off camera" => "Desligar câmera",
		"Turn on microphone" => "Ligar microfone",
		"Turn off microphone" => "Desligar microfone",
		"Turn on incoming audio" => "Ouvir a chamada",
		"Turn off incoming audio" => "Deixar de ouvir a chamada",
		"Speaking is unavailable in this channel." => "Não é possível falar neste canal.",
		"Voice settings" => "Configurações de voz",
		"Microphone and speaker settings" => "Microfone e alto-falantes",
		"Noise suppression" => "Supressão de ruído",
		"Removes background noise from your microphone before anyone else hears it." => {
			"Tira o barulho de fundo do seu microfone antes que os outros escutem."
		}
		"Bot" => "Bot",
		"Screens" => "Telas",
		"Apps" => "Apps",
		"Options" => "Opções",
		"None open" => "Nenhum aberto",
		"open" => "abertos",
		"Resolution" => "Resolução",
		"Frame rate" => "Quadros/s",
		"Choose a screen or window" => "Escolha uma tela ou janela",
		"Looking for screens and windows…" => "Procurando telas e janelas…",
		"Offline preview · no screen is captured" => "Prévia offline · nenhuma tela é capturada",
		"In a call" => "Em chamada",
		"In a call · microphone muted" => "Em chamada · microfone mudo",
		"In a call · deafened" => "Em chamada · áudio desligado",
		"unread mentions" => "menções não lidas",
		"User volume" => "Volume da pessoa",
		"Reset volume" => "Restaurar volume",
		"Silent" => "Sem som",
		"Normal" => "Normal",
		"Louder than normal" => "Mais alto que o normal",
		"5% quieter" => "5% mais baixo",
		"5% louder" => "5% mais alto",
		"Bots start at 50% to protect your hearing. You can still raise it here." => {
			"Bots começam em 50% para proteger sua audição. Você ainda pode aumentar aqui."
		}
		"Start bots at 50% volume" => "Bots começam com 50% de volume",
		"Protects your hearing from bots that join very loud. Right-click a bot in the call to change its volume." => {
			"Protege sua audição de bots que entram muito altos. Clique com o botão direito num bot na chamada para mudar o volume dele."
		}
		"Light" => "Leve",
		"Maximum" => "Máxima",
		"Recommended" => "Recomendado",
		"All voice settings" => "Todas as configurações de voz",
		"No filter. For studio microphones or when playing music." => {
			"Sem filtro. Para microfones de estúdio ou quando for tocar música."
		}
		"Steady hum like fans or air conditioning. Lightest on your PC." => {
			"Zumbido constante, como ventilador ou ar-condicionado. O mais leve para o PC."
		}
		"Keyboard, clicks and everyday home noise. Works well for most people." => {
			"Teclado, cliques e barulhos do dia a dia em casa. Funciona bem para a maioria."
		}
		"Very noisy home, or friends complain about your background noise. Uses more of your PC." => {
			"Casa muito barulhenta ou amigos reclamando do seu barulho. Usa mais o PC."
		}
		"PC usage: none" => "Uso do PC: nenhum",
		"PC usage: very low" => "Uso do PC: muito baixo",
		"PC usage: low" => "Uso do PC: baixo",
		"PC usage: medium" => "Uso do PC: médio",
		"Friends complaining about noise? Choose Maximum. If your voice cuts out or your PC slows down, go back to Standard." => {
			"Amigos reclamando do barulho? Escolha Máxima. Se sua voz começar a picotar ou o PC ficar lento, volte para Padrão."
		}
		"Your PC couldn't keep up with Maximum, so SereinExt switched to Standard to keep your voice smooth." => {
			"Seu PC não acompanhou a Máxima, então o SereinExt voltou para Padrão para sua voz não travar."
		}
		"The defaults work for most people. Change these only if something sounds wrong." => {
			"Os padrões servem para a maioria. Mude só se algo estiver soando errado."
		}
		"Click to turn on or off · right-click to choose the level" => {
			"Clique para ligar ou desligar · botão direito para escolher o nível"
		}
		"Noise suppression is unavailable in this build or preview." => {
			"A supressão de ruído não está disponível nesta versão ou prévia."
		}
		"Share a screen or window" => "Compartilhar uma tela ou janela",
		"Stop sharing your screen" => "Parar de compartilhar a tela",
		"Stop sharing your camera" => "Parar de compartilhar a câmera",
		"Share your selected camera with this call" => {
			"Compartilhar a câmera escolhida nesta chamada"
		}
		"Turn off noise suppression" => "Desligar supressão de ruído",
		"Turn on noise suppression" => "Ligar supressão de ruído",
		"Voice processing" => "Processamento de voz",
		"Voice processing & input mode" => "Processamento de voz e modo de entrada",
		"Mode" => "Modo",
		"Off" => "Desligado",
		"Standard" => "Padrão",
		"Echo cancellation" => "Cancelamento de eco",
		"Recommended when speakers can be picked up by your microphone." => {
			"Recomendado quando o alto-falante pode ser captado pelo microfone."
		}
		"Automatic microphone volume" => "Volume automático do microfone",
		"Keeps speech at a more consistent loudness without changing your output volume." => {
			"Mantém a fala num volume mais estável, sem mudar o volume de saída."
		}
		"Push to talk" => "Apertar para falar",
		"When enabled, your microphone transmits only while the configured shortcut is held." => {
			"Com isso ligado, o microfone só transmite enquanto o atalho estiver pressionado."
		}
		"Mute and deafen always take priority." => {
			"Silenciar o microfone e o áudio sempre tem prioridade."
		}
		"Hold your configured shortcut when you want to speak." => {
			"Segure o atalho configurado quando quiser falar."
		}
		"Deafen turns off incoming audio and mutes your microphone with it." => {
			"Ensurdecer desliga o áudio da chamada e silencia o microfone junto."
		}
		"Advanced input settings" => "Ajustes avançados de entrada",
		"Voice activity threshold" => "Limite de atividade de voz",
		"Only transmit sound above the threshold." => "Só transmite som acima do limite.",
		"Open voice activity; mute and push to talk still apply." => {
			"Microfone aberto; silenciar e apertar para falar continuam valendo."
		}
		"Input level" => "Nível de entrada",
		"Light suppression strength" => "Intensidade do nível Leve",
		"Higher levels remove more noise but can affect natural voice detail." => {
			"Níveis mais altos tiram mais ruído, mas podem mudar o detalhe natural da voz."
		}
		"Low" => "Baixo",
		"Moderate" => "Moderado",
		"High" => "Alto",
		"Very high" => "Muito alto",
		"Recommended defaults" => "Padrões recomendados",
		"Raw microphone" => "Microfone sem tratamento",
		"Devices & levels" => "Dispositivos e volumes",
		"Input device" => "Dispositivo de entrada",
		"Output device" => "Dispositivo de saída",
		"Microphone gain" => "Ganho do microfone",
		"Speaker volume" => "Volume do alto-falante",
		"100% is the original level. Higher levels may distort." => {
			"100% é o nível original. Acima disso pode distorcer."
		}
		"Rescan devices" => "Procurar dispositivos de novo",
		"Reset levels" => "Restaurar volumes",
		"System default follows your operating-system choice. Select a device only when you want SereinExt to stay pinned to it." => {
			"O padrão do sistema segue a escolha do sistema operacional. Escolha um dispositivo só quando quiser que o SereinExt fique nele."
		}
		"System default (recommended)" => "Padrão do sistema (recomendado)",
		"Device unavailable — choose another" => "Dispositivo indisponível — escolha outro",
		"Looking for audio devices..." => "Procurando dispositivos de áudio...",
		"Looking for audio devices…" => "Procurando dispositivos de áudio…",
		"Could not start audio device discovery" => "Não foi possível procurar os dispositivos de áudio",
		"Audio devices loaded · headphones avoid microphone echo" => {
			"Dispositivos de áudio carregados · fone evita eco do microfone"
		}
		"Audio device discovery stopped" => "A busca de dispositivos de áudio parou",
		"One selected audio device is unavailable. Choose System default or rescan devices." => {
			"Um dispositivo de áudio escolhido não está disponível. Use o padrão do sistema ou procure de novo."
		}
		"Microphone unavailable · choose another input. You are still connected." => {
			"Microfone indisponível · escolha outra entrada. Você continua na chamada."
		}
		"Microphone unavailable · still connected. Choose another input in Audio settings." => {
			"Microfone indisponível · você continua na chamada. Escolha outra entrada em Áudio."
		}
		"Camera" => "Câmera",
		"Voice privacy code" => "Código de privacidade da voz",
		"Compare with the other participants. This code changes with the encrypted call group." => {
			"Compare com os outros participantes. Este código muda com o grupo criptografado da chamada."
		}
		"Audio preferences are saved on this device. Your microphone starts only when you join a call or start testing." => {
			"As preferências de áudio ficam neste dispositivo. O microfone só liga quando você entra numa chamada ou começa um teste."
		}
		"Install a voice-enabled build to use these controls." => {
			"Use uma versão com voz para estes controles."
		}
		"Friends" => "Amigos",
		"Add Friend" => "Adicionar amigo",
		"You can add friends with their Discord username." => {
			"Você pode adicionar amigos pelo nome de usuário do Discord."
		}
		"Username" => "Nome de usuário",
		"Enter a username" => "Digite um nome de usuário",
		"Sending…" => "Enviando…",
		"Send Friend Request" => "Enviar pedido de amizade",
		"Offline demo · actions are simulated." => "Demonstração offline · as ações são simuladas.",
		"Reconnect before sending a friend request." => {
			"Reconecte antes de enviar um pedido de amizade."
		}
		"All" => "Todos",
		"Pending" => "Pendentes",
		"Blocked & Ignored" => "Bloqueados e ignorados",
		"All friends" => "Todos os amigos",
		"Blocked & ignored" => "Bloqueados e ignorados",
		"Blocked and ignored users are not available yet." => {
			"Usuários bloqueados e ignorados ainda não estão disponíveis."
		}
		"Friends are not available yet." => "Os amigos ainda não estão disponíveis.",
		"No blocked or ignored users match your search." => {
			"Nenhum usuário bloqueado ou ignorado corresponde à busca."
		}
		"No friends match your search." => "Nenhum amigo corresponde à busca.",
		"No blocked or ignored users." => "Nenhum usuário bloqueado ou ignorado.",
		"No friends yet." => "Nenhum amigo ainda.",
		"No friends are currently online." => "Nenhum amigo está online agora.",
		"Some friends’ online status and activity couldn’t be loaded. The Online list may be incomplete." => {
			"O status e a atividade de alguns amigos não carregaram. A lista Online pode estar incompleta."
		}
		"Dismiss friend status warning" => "Dispensar aviso de status dos amigos",
		"Direct Messages" => "Mensagens diretas",
		"Mark As Read" => "Marcar como lida",
		"Server Settings" => "Configurações do servidor",
		"Create invite" => "Criar convite",
		"Leave server" => "Sair do servidor",
		"Leave server?" => "Sair do servidor?",
		"Are you sure you want to leave" => "Tem certeza que quer sair de",
		"You will not be able to rejoin this server unless you are re-invited." => {
			"Você não poderá voltar a este servidor sem um novo convite."
		}
		"Leaving…" => "Saindo…",
		"Leave Server" => "Sair do servidor",
		"Close" => "Fechar",
		"Offline preview · no server changes" => "Prévia offline · nenhuma mudança no servidor",
		"Remove From Favorites" => "Remover dos favoritos",
		"Add To Favorites" => "Adicionar aos favoritos",
		"Favorites are saved on this device." => "Os favoritos ficam salvos neste dispositivo.",
		"Invite to Channel" => "Convidar para o canal",
		"Copy Link" => "Copiar link",
		"Unmute Channel" => "Reativar canal",
		"Mute Channel" => "Silenciar canal",
		"For 15 Minutes" => "Por 15 minutos",
		"For 1 Hour" => "Por 1 hora",
		"For 3 Hours" => "Por 3 horas",
		"For 8 Hours" => "Por 8 horas",
		"For 24 Hours" => "Por 24 horas",
		"Until I Turn It Back On" => "Até eu reativar",
		"Copy Channel ID" => "Copiar ID do canal",
		"Hide Muted Channels" => "Ocultar canais silenciados",
		"Restart to update" => "Reiniciar para atualizar",
		"Updating…" => "Atualizando…",
		"Update available" => "Atualização disponível",
		"Navigation" => "Navegação",
		"Move around Serein without reaching for the mouse." => {
			"Navegue pelo Serein sem usar o mouse."
		}
		"Messages" => "Mensagens",
		"Composer shortcuts are only active while you are writing." => {
			"Os atalhos do compositor só valem enquanto você está escrevendo."
		}
		"Text Formatting" => "Formatação de texto",
		"Apply or remove formatting in the composer." => {
			"Aplica ou remove formatação no compositor."
		}
		"Global availability" => "Disponibilidade global",
		"Show Keyboard Shortcuts List" => "Mostrar lista de atalhos",
		"Switch Conversation" => "Trocar de conversa",
		"Close Settings or Dialog" => "Fechar configurações ou diálogo",
		"Send Message" => "Enviar mensagem",
		"Insert New Line" => "Inserir nova linha",
		"Edit Last Editable Message" => "Editar a última mensagem editável",
		"Bold" => "Negrito",
		"Italic" => "Itálico",
		"Underline" => "Sublinhado",
		"Strikethrough" => "Tachado",
		"Inline Code" => "Código em linha",
		"Code Block" => "Bloco de código",
		"Spoiler" => "Spoiler",
		"Push to Talk" => "Apertar para falar",
		"Toggle Mute" => "Alternar mudo",
		"Toggle Deafen" => "Alternar áudio",
		"Voice" => "Voz",
		"Control your microphone and incoming audio during a connected call." => {
			"Controla o microfone e o áudio da chamada enquanto você está conectado."
		}
		"Already bound to" => "Já usado por",
		"Brazil" => "Brasil",
		"United States" => "Estados Unidos",
		"Canada" => "Canadá",
		"United Kingdom" => "Reino Unido",
		"Germany" => "Alemanha",
		"Netherlands" => "Países Baixos",
		"France" => "França",
		"Spain" => "Espanha",
		"Poland" => "Polônia",
		"Finland" => "Finlândia",
		"Sweden" => "Suécia",
		"Singapore" => "Singapura",
		"Japan" => "Japão",
		"Hong Kong" => "Hong Kong",
		"Australia" => "Austrália",
		"India" => "Índia",
		"South Africa" => "África do Sul",
		"Chile" => "Chile",
		"Argentina" => "Argentina",
		"South Korea" => "Coreia do Sul",
		"Europe" => "Europa",
		"Russia" => "Rússia",
		"People in this call will see what you pick." => {
			"As pessoas nesta chamada verão o que você escolher."
		}
		"Looking for your screens…" => "Procurando suas telas…",
		"Entire screen" => "Tela inteira",
		"Share audio" => "Compartilhar áudio",
		"Also send sound from other apps. Your microphone stays as it is." => {
			"Também envia o som de outros apps. O microfone continua como está."
		}
		"Share an app" => "Compartilhar um app",
		"No apps are open to share." => "Não há apps abertos para compartilhar.",
		"App" => "App",
		"Refresh" => "Atualizar",
		"Quality" => "Qualidade",
		"Show cursor" => "Mostrar cursor",
		"Include the pointer in the shared video." => "Inclui o ponteiro no vídeo compartilhado.",
		"Share Screen" => "Compartilhar tela",
		"Cancel" => "Cancelar",
		"Before you use SereinExt" => "Antes de usar o SereinExt",
		"SereinExt is an unofficial app for your own Discord account: it is not Discord, it is not endorsed by Discord, and Discord's rules still apply to your account. It also includes other people's work (libraries, fonts and icons) under their own licenses, and accepting here does not waive those licenses or shift their copyright. Continuing confirms that you understand both points." => {
			"O SereinExt é um aplicativo não oficial para a sua própria conta do Discord: não é o Discord, não é endossado pelo Discord, e as regras do Discord continuam valendo para a sua conta. Ele também inclui trabalho de outras pessoas (bibliotecas, fontes e ícones) sob as licenças delas, e aceitar aqui não anula essas licenças nem transfere o direito autoral delas. Ao continuar, você confirma que entendeu esses dois pontos."
		}
		"Your app preferences could not be read, so SereinExt cannot tell whether you accepted this before." => {
			"Não foi possível ler as preferências do aplicativo, então o SereinExt não sabe se você já aceitou isto antes."
		}
		"View full licenses" => "Ver licenças completas",
		"Hide full licenses" => "Ocultar licenças completas",
		"App preferences were not saved. If your acceptance of the terms was not recorded, SereinExt will ask again next launch." => {
			"As preferências do aplicativo não foram salvas. Se o seu aceite dos termos não foi registrado, o SereinExt vai perguntar de novo na próxima vez que abrir."
		}
		"Your session expired; sign in again to continue." => "Sua sessão expirou; entre de novo para continuar.",
		"I understand — continue" => "Entendi — continuar",
		"Zoom" => "Zoom",
		"Scales text and controls across the app." => {
			"Ajusta o texto e os controles em todo o aplicativo."
		}
		"Layout" => "Layout",
		"Reset layout" => "Redefinir layout",
		"Sidebar width" => "Largura da barra lateral",
		"Channel and conversation list width in wide windows." => {
			"Largura da lista de canais e conversas em janelas largas."
		}
		"Show People in wide windows" => "Mostrar Pessoas em janelas largas",
		"Keep the member list open whenever the window is wide enough." => {
			"Mantém a lista de membros aberta quando a janela é larga o suficiente."
		}
		"Messages and media" => "Mensagens e mídia",
		"Reset chat" => "Redefinir conversas",
		"Animate GIFs" => "Animar GIFs",
		"Visible chat GIFs play automatically." => {
			"Os GIFs visíveis no chat são reproduzidos automaticamente."
		}
		"Hide image and GIF links" => "Ocultar links de imagem e GIF",
		"Hide standalone links when their image or GIF preview is shown." => {
			"Oculta links soltos quando a prévia da imagem ou do GIF aparece."
		}
		"Links" => "Links",
		"Confirm before opening links" => "Confirmar antes de abrir links",
		"Ask before opening external links. Discord links always open directly." => {
			"Pergunta antes de abrir links externos. Links do Discord sempre abrem direto."
		}
		"Scrolling" => "Rolagem",
		"Smooth scrolling" => "Rolagem suave",
		"Animate wheel movement and jumps between messages." => {
			"Anima o movimento da roda e os saltos entre mensagens."
		}
		"Scrolling speed" => "Velocidade da rolagem",
		"Mouse wheel and trackpad movement. 100% is the default." => {
			"Movimento da roda e do trackpad. 100% é o padrão."
		}
		"Retry saving reading settings" => "Tentar salvar as preferências de leitura de novo",
		"Overview" => "Visão geral",
		"Sounds" => "Sons",
		"Badges" => "Emblemas",
		"Enable Desktop Notifications" => "Ativar notificações na área de trabalho",
		"For per-channel or per-server notifications, right-click the channel or server and select Notification Settings." => {
			"Para notificações por canal ou servidor, clique com o botão direito no canal ou servidor e escolha Configurações de notificação."
		}
		"Sound Volume" => "Volume dos sons",
		"Adjusts the volume of all notification sounds and ringtones." => {
			"Ajusta o volume de todos os sons de notificação e toques."
		}
		"Disable All Notification Sounds" => "Desativar todos os sons de notificação",
		"Disables notification sounds. Your individual sound preferences are saved and restored when you turn this off." => {
			"Desativa os sons de notificação. Suas preferências individuais são salvas e voltam quando você desliga isto."
		}
		"New Message" => "Nova mensagem",
		"New Message in the channel I'm currently reading" => {
			"Nova mensagem no canal que estou lendo"
		}
		"Incoming Ring" => "Toque de entrada",
		"Outgoing Ring" => "Toque de saída",
		"Microphone Muted" => "Microfone silenciado",
		"Microphone Unmuted" => "Microfone ativado",
		"Camera On" => "Câmera ligada",
		"Screen Share Started" => "Compartilhamento de tela iniciado",
		"Call Joined" => "Entrou na chamada",
		"User Left Call" => "Saiu da chamada",
		"Preview Sound" => "Ouvir som",
		"Ringtones, call devices and microphone processing." => {
			"Toques, dispositivos da chamada e processamento do microfone."
		}
		"Open" => "Abrir",
		"Enable Unread Message Badge" => "Mostrar emblema de mensagens não lidas",
		"Shows a red badge on the app icon when you have unread messages." => {
			"Mostra um emblema vermelho no ícone do app quando há mensagens não lidas."
		}
		"App icon badges are not available on this platform yet." => {
			"Emblemas no ícone do app ainda não estão disponíveis nesta plataforma."
		}
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
		"The Discord account signed in on this device." => {
			"La cuenta de Discord conectada en este dispositivo."
		}
		"Choose how you appear across Discord." => "Elige cómo apareces en Discord.",
		"Startup, window and graphics behavior on this device." => {
			"Inicio, ventana y gráficos en este dispositivo."
		}
		"Theme, colours, window effects and layout." => {
			"Tema, colores, efectos de ventana y diseño."
		}
		"How messages, media, links and scrolling behave." => {
			"Cómo se comportan los mensajes, medios, enlaces y desplazamiento."
		}
		"Control who can contact you and how messages are filtered." => {
			"Controla quién puede contactarte y cómo se filtran los mensajes."
		}
		"Choose which notifications you receive and how they appear." => {
			"Elige qué notificaciones recibes y cómo aparecen."
		}
		"Show others what you are playing." => "Muestra a los demás a qué estás jugando.",
		"Microphone, speakers, camera and voice processing." => {
			"Micrófono, altavoces, cámara y procesamiento de voz."
		}
		"Keyboard shortcuts for SereinExt." => "Atajos de teclado de SereinExt.",
		"What SereinExt keeps on this device." => "Lo que SereinExt guarda en este dispositivo.",
		"Keep SereinExt up to date on this device." => {
			"Mantén SereinExt actualizado en este dispositivo."
		}
		"Manage community plugins." => "Administra plugins de la comunidad.",
		"Choose a community theme." => "Elige un tema de la comunidad.",
		"Language" => "Idioma",
		"App language" => "Idioma de la aplicación",
		"Changes apply immediately and are saved on this device." => {
			"Los cambios se aplican inmediatamente y se guardan en este dispositivo."
		}
		"Startup" => "Inicio",
		"Open SereinExt when your computer starts" => "Abrir SereinExt al iniciar el equipo",
		"SereinExt signs in and connects in the background." => {
			"SereinExt inicia sesión y se conecta en segundo plano."
		}
		"Start minimized" => "Iniciar minimizado",
		"Start in the background, out of your way." => {
			"Iniciar en segundo plano, sin ocupar la pantalla."
		}
		"Automatic startup is available on Windows and macOS." => {
			"El inicio automático está disponible en Windows y macOS."
		}
		"Window" => "Ventana",
		"Hide SereinExt title bar" => "Ocultar la barra de título de SereinExt",
		"Use the system title bar and window buttons instead." => {
			"Usa la barra de título y los botones de ventana del sistema."
		}
		"Keep SereinExt in the menu bar" => "Mantener SereinExt en la barra de menús",
		"Keep SereinExt in the system tray" => "Mantener SereinExt en la bandeja del sistema",
		"The tray is unavailable on this platform." => {
			"La bandeja del sistema no está disponible en esta plataforma."
		}
		"Graphics" => "Gráficos",
		"Render with" => "Renderizar con",
		"Search" => "Buscar",
		"Close settings (Esc)" => "Cerrar ajustes (Esc)",
		"Unofficial · not endorsed by Discord" => "No oficial · no respaldado por Discord",
		"Exit preview" => "Salir de la vista previa",
		"Log out" => "Cerrar sesión",
		"Your account" => "Tu cuenta",
		"Offline preview · synthetic account" => "Vista previa sin conexión · cuenta sintética",
		"Signed in with your Discord account" => "Sesión iniciada con tu cuenta de Discord",
		"Display name" => "Nombre para mostrar",
		"Email, password and security" => "Correo, contraseña y seguridad",
		"Managed in Discord" => "Administrado en Discord",
		"Edit profile" => "Editar perfil",
		"Session" => "Sesión",
		"Closes the offline fixture. Nothing is stored for the preview." => {
			"Cierra la vista previa sin conexión. No se guarda nada para la vista previa."
		}
		"Removes the saved login and clears this account's local cache and drafts." => {
			"Elimina el inicio de sesión guardado y borra la caché local y los borradores de esta cuenta."
		}
		"Theme" => "Tema",
		"Accent" => "Color de acento",
		"Primary color" => "Color principal",
		"The active theme brings its own accent; it takes over while the theme is in use." => {
			"El tema activo trae su propio color de acento y se usa mientras el tema esté activo."
		}
		"Used for buttons, selection and message highlights." => {
			"Se usa en botones, selección y resaltados de mensajes."
		}
		"Reset" => "Restablecer",
		"Choose primary color" => "Elegir color principal",
		"Window effects" => "Efectos de ventana",
		"Transparency & blur" => "Transparencia y desenfoque",
		"Restart SereinExt after changing this. Themes can customize effects while enabled." => {
			"Reinicia SereinExt después de cambiar esto. Los temas pueden personalizar los efectos mientras estén activos."
		}
		"Transparency" => "Transparencia",
		"Blur" => "Desenfoque",
		"Zero disables blur; the native compositor controls its exact strength." => {
			"Cero desactiva el desenfoque; el compositor nativo controla su intensidad exacta."
		}
		"Apply to all surfaces" => "Aplicar a todas las superficies",
		"Include sidebars, server rail, headers, and composer." => {
			"Incluye barras laterales, barra de servidores, encabezados y compositor."
		}
		"Channel list" => "Lista de canales",
		"Show hidden channels" => "Mostrar canales ocultos",
		"Show channels you cannot currently access." => {
			"Muestra canales a los que no puedes acceder actualmente."
		}
		"Colour preset" => "Preajuste de color",
		"Share game activity" => "Compartir actividad de juego",
		"Detect running games and ask Discord to share them as activity." => {
			"Detecta juegos en ejecución y pide a Discord que los comparta como actividad."
		}
		"Enable on Discord" => "Activar en Discord",
		"Check again" => "Comprobar de nuevo",
		"Looking for a running game" => "Buscando un juego en ejecución",
		"Activity sharing is off" => "El uso compartido de actividad está desactivado",
		"Synthetic activity, never shared or saved." => {
			"Actividad sintética, nunca compartida ni guardada."
		}
		"Local storage" => "Almacenamiento local",
		"Clear cache" => "Borrar caché",
		"Removes cached messages and media. Drafts and your login stay." => {
			"Elimina mensajes y medios en caché. Los borradores y tu inicio de sesión se conservan."
		}
		"Messages and drafts are cached on this device inside bounded, account-isolated files. Cache data is not encrypted by SereinExt; saved login tokens use the OS credential store." => {
			"Los mensajes y borradores se guardan en caché en este dispositivo, en archivos limitados y aislados por cuenta. SereinExt no cifra los datos de caché; los tokens de inicio de sesión guardados usan el almacén de credenciales del sistema."
		}
		"Your privacy" => "Tu privacidad",
		"SereinExt does not collect telemetry or upload diagnostics. Discord retains service-side data according to its own policies." => {
			"SereinExt no recopila telemetría ni envía diagnósticos. Discord conserva los datos del servicio según sus propias políticas."
		}
		"Offline preview · changes stay in this session and are never sent." => {
			"Vista previa sin conexión · los cambios permanecen en esta sesión y nunca se envían."
		}
		"Closing the window keeps SereinExt in the menu bar. Quit from its menu to exit." => {
			"Cerrar la ventana mantiene SereinExt en la barra de menús. Sal desde su menú para terminar."
		}
		"Closing keeps SereinExt running. Use the tray to show, minimize or quit." => {
			"Cerrar mantiene SereinExt en ejecución. Usa la bandeja para mostrar, minimizar o salir."
		}
		"Closing the window keeps SereinExt in the notification area. Quit from its menu to exit." => {
			"Cerrar la ventana mantiene SereinExt en el área de notificación. Sal desde su menú para terminar."
		}
		"Takes effect the next time SereinExt starts." => {
			"Se aplica la próxima vez que se inicie SereinExt."
		}
		"Online" => "En línea",
		"Idle" => "Ausente",
		"Do Not Disturb" => "No molestar",
		"Invisible" => "Invisible",
		"Don't clear" => "No borrar",
		"30 minutes" => "30 minutos",
		"1 hour" => "1 hora",
		"4 hours" => "4 horas",
		"Today" => "Hoy",
		"Switch to" => "Cambiar a",
		"Forget" => "Olvidar",
		"SereinExt clears it" => "SereinExt borra el estado",
		"You" => "Tú",
		"Custom status" => "Estado personalizado",
		"Shown next to your name across Discord." => "Se muestra junto a tu nombre en Discord.",
		"Switch accounts" => "Cambiar de cuenta",
		"Add an account" => "Añadir una cuenta",
		"Forget this account on this device" => "Olvidar esta cuenta en este dispositivo",
		"Loading profile…" => "Cargando perfil…",
		"Reload profile" => "Recargar perfil",
		"You will not receive desktop notifications" => "No recibirás notificaciones de escritorio",
		"You will appear offline" => "Aparecerás sin conexión",
		"Edit custom status" => "Editar estado personalizado",
		"Set a custom status" => "Establecer un estado personalizado",
		"No custom status" => "Sin estado personalizado",
		"Status text" => "Texto del estado",
		"What's on your mind?" => "¿Qué tienes en mente?",
		"Clear after" => "Borrar después",
		"Use up to 128 characters without control characters." => {
			"Usa hasta 128 caracteres sin caracteres de control."
		}
		"Clear" => "Borrar",
		"Apply" => "Aplicar",
		"No settings found" => "No se encontraron ajustes",
		"Try theme, notifications, voice, or cache." => "Prueba tema, notificaciones, voz o caché.",
		"Use a different account" => "Usar otra cuenta",
		"Waiting for Discord…" => "Esperando a Discord…",
		"Use another account" => "Usar otra cuenta",
		"Continue with Discord" => "Continuar con Discord",
		"Welcome back" => "Bienvenido de nuevo",
		"Welcome to SereinExt" => "Bienvenido a SereinExt",
		"Continue with a saved account, or sign in with another one." => {
			"Continúa con una cuenta guardada o entra con otra."
		}
		"Sign in with Discord." => "Entra con Discord.",
		"Saved accounts" => "Cuentas guardadas",
		"This is my account" => "Esta es mi cuenta",
		"Check this to continue." => "Marca esto para continuar.",
		"Independent and open source. Not affiliated with Discord." => {
			"Independiente y de código abierto. No afiliado a Discord."
		}
		"Message" => "Mensaje",
		"Unread messages" => "Mensajes no leídos",
		"Mark as read" => "Marcar como leído",
		"Jump to unread" => "Ir a no leídos",
		"Copy" => "Copiar",
		"Copy message" => "Copiar mensaje",
		"Copy download link" => "Copiar enlace de descarga",
		"Reply" => "Responder",
		"Forward" => "Reenviar",
		"Forward message" => "Reenviar mensaje",
		"Create Thread…" => "Crear hilo…",
		"View reactions" => "Ver reacciones",
		"Mark read through here" => "Marcar como leído hasta aquí",
		"Mark Unread" => "Marcar como no leído",
		"Unpin message" => "Desfijar mensaje",
		"Pin message" => "Fijar mensaje",
		"Edit message" => "Editar mensaje",
		"Remove from delete selection" => "Quitar de la selección",
		"Select for batch delete" => "Seleccionar para borrar",
		"You can select up to 5 messages at a time." => {
			"Puedes seleccionar hasta 5 mensajes a la vez."
		}
		"Delete message…" => "Borrar mensaje…",
		"Delete message immediately" => "Borrar mensaje ahora",
		"Message history is unavailable with current permission information." => {
			"El historial no está disponible con los permisos actuales."
		}
		"Mute" => "Silenciar",
		"Unmute" => "Activar micrófono",
		"Deafen" => "Ensordecer",
		"Undeafen" => "Activar audio",
		"Disconnect" => "Desconectar",
		"Dismiss call" => "Cerrar llamada",
		"Reconnect to call" => "Reconectar a la llamada",
		"Recent call" => "Llamada reciente",
		"You were in this call recently" => "Estuviste en esta llamada hace poco",
		"Dismiss" => "Descartar",
		"Share your screen" => "Compartir pantalla",
		"Stop sharing" => "Dejar de compartir",
		"Turn on camera" => "Activar cámara",
		"Turn off camera" => "Desactivar cámara",
		"Turn on microphone" => "Activar micrófono",
		"Turn off microphone" => "Desactivar micrófono",
		"Turn on incoming audio" => "Escuchar la llamada",
		"Turn off incoming audio" => "Dejar de escuchar la llamada",
		"Speaking is unavailable in this channel." => "No se puede hablar en este canal.",
		"Voice settings" => "Ajustes de voz",
		"Microphone and speaker settings" => "Micrófono y altavoces",
		"Noise suppression" => "Supresión de ruido",
		"Removes background noise from your microphone before anyone else hears it." => {
			"Quita el ruido de fondo de tu micrófono antes de que los demás lo escuchen."
		}
		"Bot" => "Bot",
		"Screens" => "Pantallas",
		"Apps" => "Apps",
		"Options" => "Opciones",
		"None open" => "Ninguna abierta",
		"open" => "abiertas",
		"Resolution" => "Resolución",
		"Frame rate" => "Fotogramas/s",
		"Choose a screen or window" => "Elige una pantalla o ventana",
		"Looking for screens and windows…" => "Buscando pantallas y ventanas…",
		"Offline preview · no screen is captured" => "Vista previa sin conexión · no se captura ninguna pantalla",
		"In a call" => "En llamada",
		"In a call · microphone muted" => "En llamada · micrófono silenciado",
		"In a call · deafened" => "En llamada · audio desactivado",
		"unread mentions" => "menciones sin leer",
		"User volume" => "Volumen de la persona",
		"Reset volume" => "Restablecer volumen",
		"Silent" => "Sin sonido",
		"Normal" => "Normal",
		"Louder than normal" => "Más alto de lo normal",
		"5% quieter" => "5% más bajo",
		"5% louder" => "5% más alto",
		"Bots start at 50% to protect your hearing. You can still raise it here." => {
			"Los bots empiezan al 50% para proteger tu audición. Aún puedes subirlo aquí."
		}
		"Start bots at 50% volume" => "Los bots empiezan al 50% de volumen",
		"Protects your hearing from bots that join very loud. Right-click a bot in the call to change its volume." => {
			"Protege tu audición de bots que entran muy fuertes. Haz clic derecho en un bot de la llamada para cambiar su volumen."
		}
		"Light" => "Ligera",
		"Maximum" => "Máxima",
		"Recommended" => "Recomendado",
		"All voice settings" => "Todos los ajustes de voz",
		"No filter. For studio microphones or when playing music." => {
			"Sin filtro. Para micrófonos de estudio o cuando pones música."
		}
		"Steady hum like fans or air conditioning. Lightest on your PC." => {
			"Zumbido constante, como ventilador o aire acondicionado. Lo más ligero para el PC."
		}
		"Keyboard, clicks and everyday home noise. Works well for most people." => {
			"Teclado, clics y ruidos cotidianos de casa. Funciona bien para la mayoría."
		}
		"Very noisy home, or friends complain about your background noise. Uses more of your PC." => {
			"Casa muy ruidosa o amigos que se quejan de tu ruido. Usa más el PC."
		}
		"PC usage: none" => "Uso del PC: ninguno",
		"PC usage: very low" => "Uso del PC: muy bajo",
		"PC usage: low" => "Uso del PC: bajo",
		"PC usage: medium" => "Uso del PC: medio",
		"Friends complaining about noise? Choose Maximum. If your voice cuts out or your PC slows down, go back to Standard." => {
			"¿Tus amigos se quejan del ruido? Elige Máxima. Si tu voz se corta o el PC va lento, vuelve a Estándar."
		}
		"Your PC couldn't keep up with Maximum, so SereinExt switched to Standard to keep your voice smooth." => {
			"Tu PC no pudo con Máxima, así que SereinExt volvió a Estándar para que tu voz no se corte."
		}
		"The defaults work for most people. Change these only if something sounds wrong." => {
			"Los valores predeterminados sirven a la mayoría. Cámbialos solo si algo suena mal."
		}
		"Click to turn on or off · right-click to choose the level" => {
			"Haz clic para activar o desactivar · clic derecho para elegir el nivel"
		}
		"Noise suppression is unavailable in this build or preview." => {
			"La supresión de ruido no está disponible en esta versión o vista previa."
		}
		"Share a screen or window" => "Compartir una pantalla o ventana",
		"Stop sharing your screen" => "Dejar de compartir la pantalla",
		"Stop sharing your camera" => "Dejar de compartir la cámara",
		"Share your selected camera with this call" => {
			"Compartir la cámara elegida en esta llamada"
		}
		"Turn off noise suppression" => "Desactivar supresión de ruido",
		"Turn on noise suppression" => "Activar supresión de ruido",
		"Voice processing" => "Procesamiento de voz",
		"Voice processing & input mode" => "Procesamiento de voz y modo de entrada",
		"Mode" => "Modo",
		"Off" => "Desactivado",
		"Standard" => "Estándar",
		"Echo cancellation" => "Cancelación de eco",
		"Recommended when speakers can be picked up by your microphone." => {
			"Recomendado cuando el altavoz puede entrar por el micrófono."
		}
		"Automatic microphone volume" => "Volumen automático del micrófono",
		"Keeps speech at a more consistent loudness without changing your output volume." => {
			"Mantiene la voz en un volumen más estable, sin cambiar el volumen de salida."
		}
		"Push to talk" => "Pulsar para hablar",
		"When enabled, your microphone transmits only while the configured shortcut is held." => {
			"Con esto activado, el micrófono solo transmite mientras mantienes el atajo."
		}
		"Mute and deafen always take priority." => {
			"Silenciar el micrófono y el audio siempre tiene prioridad."
		}
		"Hold your configured shortcut when you want to speak." => {
			"Mantén el atajo configurado cuando quieras hablar."
		}
		"Deafen turns off incoming audio and mutes your microphone with it." => {
			"Ensordecer apaga el audio de la llamada y silencia el micrófono a la vez."
		}
		"Advanced input settings" => "Ajustes avanzados de entrada",
		"Voice activity threshold" => "Umbral de actividad de voz",
		"Only transmit sound above the threshold." => "Solo transmite sonido por encima del umbral.",
		"Open voice activity; mute and push to talk still apply." => {
			"Micrófono abierto; silenciar y pulsar para hablar siguen aplicando."
		}
		"Input level" => "Nivel de entrada",
		"Light suppression strength" => "Intensidad del nivel Ligera",
		"Higher levels remove more noise but can affect natural voice detail." => {
			"Los niveles más altos quitan más ruido, pero pueden cambiar el detalle natural de la voz."
		}
		"Low" => "Bajo",
		"Moderate" => "Moderado",
		"High" => "Alto",
		"Very high" => "Muy alto",
		"Recommended defaults" => "Valores recomendados",
		"Raw microphone" => "Micrófono sin procesar",
		"Devices & levels" => "Dispositivos y volumen",
		"Input device" => "Dispositivo de entrada",
		"Output device" => "Dispositivo de salida",
		"Microphone gain" => "Ganancia del micrófono",
		"Speaker volume" => "Volumen del altavoz",
		"100% is the original level. Higher levels may distort." => {
			"100% es el nivel original. Por encima puede distorsionar."
		}
		"Rescan devices" => "Buscar dispositivos de nuevo",
		"Reset levels" => "Restablecer volumen",
		"System default follows your operating-system choice. Select a device only when you want SereinExt to stay pinned to it." => {
			"El predeterminado del sistema sigue la elección del sistema operativo. Elige un dispositivo solo si quieres que SereinExt se quede en él."
		}
		"System default (recommended)" => "Predeterminado del sistema (recomendado)",
		"Device unavailable — choose another" => "Dispositivo no disponible — elige otro",
		"Looking for audio devices..." => "Buscando dispositivos de audio...",
		"Looking for audio devices…" => "Buscando dispositivos de audio…",
		"Could not start audio device discovery" => "No se pudieron buscar los dispositivos de audio",
		"Audio devices loaded · headphones avoid microphone echo" => {
			"Dispositivos de audio cargados · los auriculares evitan el eco del micrófono"
		}
		"Audio device discovery stopped" => "La búsqueda de dispositivos de audio se detuvo",
		"One selected audio device is unavailable. Choose System default or rescan devices." => {
			"Un dispositivo de audio elegido no está disponible. Usa el predeterminado del sistema o busca de nuevo."
		}
		"Microphone unavailable · choose another input. You are still connected." => {
			"Micrófono no disponible · elige otra entrada. Sigues en la llamada."
		}
		"Microphone unavailable · still connected. Choose another input in Audio settings." => {
			"Micrófono no disponible · sigues en la llamada. Elige otra entrada en Audio."
		}
		"Camera" => "Cámara",
		"Voice privacy code" => "Código de privacidad de voz",
		"Compare with the other participants. This code changes with the encrypted call group." => {
			"Compáralo con los demás participantes. Este código cambia con el grupo cifrado de la llamada."
		}
		"Audio preferences are saved on this device. Your microphone starts only when you join a call or start testing." => {
			"Las preferencias de audio se guardan en este dispositivo. El micrófono solo se enciende al entrar en una llamada o al empezar una prueba."
		}
		"Install a voice-enabled build to use these controls." => {
			"Usa una versión con voz para estos controles."
		}
		"Friends" => "Amigos",
		"Add Friend" => "Añadir amigo",
		"You can add friends with their Discord username." => {
			"Puedes añadir amigos con su nombre de usuario de Discord."
		}
		"Username" => "Nombre de usuario",
		"Enter a username" => "Escribe un nombre de usuario",
		"Sending…" => "Enviando…",
		"Send Friend Request" => "Enviar solicitud de amistad",
		"Offline demo · actions are simulated." => "Vista previa sin conexión · las acciones son simuladas.",
		"Reconnect before sending a friend request." => {
			"Reconéctate antes de enviar una solicitud de amistad."
		}
		"All" => "Todos",
		"Pending" => "Pendientes",
		"Blocked & Ignored" => "Bloqueados e ignorados",
		"All friends" => "Todos los amigos",
		"Blocked & ignored" => "Bloqueados e ignorados",
		"Blocked and ignored users are not available yet." => {
			"Los usuarios bloqueados e ignorados todavía no están disponibles."
		}
		"Friends are not available yet." => "Los amigos todavía no están disponibles.",
		"No blocked or ignored users match your search." => {
			"Ningún usuario bloqueado o ignorado coincide con la búsqueda."
		}
		"No friends match your search." => "Ningún amigo coincide con la búsqueda.",
		"No blocked or ignored users." => "No hay usuarios bloqueados o ignorados.",
		"No friends yet." => "Todavía no hay amigos.",
		"No friends are currently online." => "Ningún amigo está en línea ahora.",
		"Some friends’ online status and activity couldn’t be loaded. The Online list may be incomplete." => {
			"No se pudo cargar el estado y la actividad de algunos amigos. La lista En línea puede estar incompleta."
		}
		"Dismiss friend status warning" => "Descartar aviso de estado de amigos",
		"Direct Messages" => "Mensajes directos",
		"Mark As Read" => "Marcar como leído",
		"Server Settings" => "Ajustes del servidor",
		"Create invite" => "Crear invitación",
		"Leave server" => "Salir del servidor",
		"Leave server?" => "¿Salir del servidor?",
		"Are you sure you want to leave" => "¿Seguro que quieres salir de",
		"You will not be able to rejoin this server unless you are re-invited." => {
			"No podrás volver a este servidor sin una nueva invitación."
		}
		"Leaving…" => "Saliendo…",
		"Leave Server" => "Salir del servidor",
		"Close" => "Cerrar",
		"Offline preview · no server changes" => "Vista previa sin conexión · sin cambios en el servidor",
		"Remove From Favorites" => "Quitar de favoritos",
		"Add To Favorites" => "Añadir a favoritos",
		"Favorites are saved on this device." => "Los favoritos se guardan en este dispositivo.",
		"Invite to Channel" => "Invitar al canal",
		"Copy Link" => "Copiar enlace",
		"Unmute Channel" => "Reactivar canal",
		"Mute Channel" => "Silenciar canal",
		"For 15 Minutes" => "Durante 15 minutos",
		"For 1 Hour" => "Durante 1 hora",
		"For 3 Hours" => "Durante 3 horas",
		"For 8 Hours" => "Durante 8 horas",
		"For 24 Hours" => "Durante 24 horas",
		"Until I Turn It Back On" => "Hasta que yo lo reactive",
		"Copy Channel ID" => "Copiar ID del canal",
		"Hide Muted Channels" => "Ocultar canales silenciados",
		"Restart to update" => "Reiniciar para actualizar",
		"Updating…" => "Actualizando…",
		"Update available" => "Actualización disponible",
		"Navigation" => "Navegación",
		"Move around Serein without reaching for the mouse." => {
			"Muévete por Serein sin usar el ratón."
		}
		"Messages" => "Mensajes",
		"Composer shortcuts are only active while you are writing." => {
			"Los atajos del compositor solo funcionan mientras escribes."
		}
		"Text Formatting" => "Formato de texto",
		"Apply or remove formatting in the composer." => {
			"Aplica o quita formato en el compositor."
		}
		"Global availability" => "Disponibilidad global",
		"Show Keyboard Shortcuts List" => "Mostrar lista de atajos",
		"Switch Conversation" => "Cambiar de conversación",
		"Close Settings or Dialog" => "Cerrar ajustes o diálogo",
		"Send Message" => "Enviar mensaje",
		"Insert New Line" => "Insertar nueva línea",
		"Edit Last Editable Message" => "Editar el último mensaje editable",
		"Bold" => "Negrita",
		"Italic" => "Cursiva",
		"Underline" => "Subrayado",
		"Strikethrough" => "Tachado",
		"Inline Code" => "Código en línea",
		"Code Block" => "Bloque de código",
		"Spoiler" => "Spoiler",
		"Push to Talk" => "Pulsar para hablar",
		"Toggle Mute" => "Alternar silencio",
		"Toggle Deafen" => "Alternar audio",
		"Voice" => "Voz",
		"Control your microphone and incoming audio during a connected call." => {
			"Controla el micrófono y el audio de la llamada mientras estás conectado."
		}
		"Already bound to" => "Ya usado por",
		"Brazil" => "Brasil",
		"United States" => "Estados Unidos",
		"Canada" => "Canadá",
		"United Kingdom" => "Reino Unido",
		"Germany" => "Alemania",
		"Netherlands" => "Países Bajos",
		"France" => "Francia",
		"Spain" => "España",
		"Poland" => "Polonia",
		"Finland" => "Finlandia",
		"Sweden" => "Suecia",
		"Singapore" => "Singapur",
		"Japan" => "Japón",
		"Hong Kong" => "Hong Kong",
		"Australia" => "Australia",
		"India" => "India",
		"South Africa" => "Sudáfrica",
		"Chile" => "Chile",
		"Argentina" => "Argentina",
		"South Korea" => "Corea del Sur",
		"Europe" => "Europa",
		"Russia" => "Rusia",
		"People in this call will see what you pick." => {
			"Las personas en esta llamada verán lo que elijas."
		}
		"Looking for your screens…" => "Buscando tus pantallas…",
		"Entire screen" => "Pantalla completa",
		"Share audio" => "Compartir audio",
		"Also send sound from other apps. Your microphone stays as it is." => {
			"También envía el sonido de otras apps. El micrófono sigue igual."
		}
		"Share an app" => "Compartir una app",
		"No apps are open to share." => "No hay apps abiertas para compartir.",
		"App" => "App",
		"Refresh" => "Actualizar",
		"Quality" => "Calidad",
		"Show cursor" => "Mostrar cursor",
		"Include the pointer in the shared video." => "Incluye el puntero en el video compartido.",
		"Share Screen" => "Compartir pantalla",
		"Cancel" => "Cancelar",
		"Before you use SereinExt" => "Antes de usar SereinExt",
		"SereinExt is an unofficial app for your own Discord account: it is not Discord, it is not endorsed by Discord, and Discord's rules still apply to your account. It also includes other people's work (libraries, fonts and icons) under their own licenses, and accepting here does not waive those licenses or shift their copyright. Continuing confirms that you understand both points." => {
			"SereinExt es una aplicación no oficial para tu propia cuenta de Discord: no es Discord, Discord no la respalda y las reglas de Discord siguen aplicando a tu cuenta. También incluye trabajo de otras personas (bibliotecas, fuentes e iconos) bajo sus propias licencias, y aceptar aquí no deja sin efecto esas licencias ni transfiere sus derechos de autor. Al continuar, confirmas que entiendes ambos puntos."
		}
		"Your app preferences could not be read, so SereinExt cannot tell whether you accepted this before." => {
			"No se pudieron leer las preferencias de la aplicación, así que SereinExt no puede saber si ya aceptaste esto antes."
		}
		"View full licenses" => "Ver licencias completas",
		"Hide full licenses" => "Ocultar licencias completas",
		"App preferences were not saved. If your acceptance of the terms was not recorded, SereinExt will ask again next launch." => {
			"Las preferencias de la aplicación no se guardaron. Si tu aceptación de los términos no quedó registrada, SereinExt volverá a preguntar la próxima vez que se abra."
		}
		"Your session expired; sign in again to continue." => "Tu sesión expiró; inicia sesión de nuevo para continuar.",
		"I understand — continue" => "Entendido — continuar",
		"Zoom" => "Zoom",
		"Scales text and controls across the app." => {
			"Escala el texto y los controles en toda la aplicación."
		}
		"Layout" => "Diseño",
		"Reset layout" => "Restablecer diseño",
		"Sidebar width" => "Ancho de la barra lateral",
		"Channel and conversation list width in wide windows." => {
			"Ancho de la lista de canales y conversaciones en ventanas anchas."
		}
		"Show People in wide windows" => "Mostrar Personas en ventanas anchas",
		"Keep the member list open whenever the window is wide enough." => {
			"Mantiene la lista de miembros abierta cuando la ventana es lo bastante ancha."
		}
		"Messages and media" => "Mensajes y medios",
		"Reset chat" => "Restablecer chat",
		"Animate GIFs" => "Animar GIFs",
		"Visible chat GIFs play automatically." => {
			"Los GIFs visibles del chat se reproducen solos."
		}
		"Hide image and GIF links" => "Ocultar enlaces de imagen y GIF",
		"Hide standalone links when their image or GIF preview is shown." => {
			"Oculta enlaces sueltos cuando se muestra la vista previa de la imagen o el GIF."
		}
		"Links" => "Enlaces",
		"Confirm before opening links" => "Confirmar antes de abrir enlaces",
		"Ask before opening external links. Discord links always open directly." => {
			"Pregunta antes de abrir enlaces externos. Los enlaces de Discord siempre se abren directo."
		}
		"Scrolling" => "Desplazamiento",
		"Smooth scrolling" => "Desplazamiento suave",
		"Animate wheel movement and jumps between messages." => {
			"Anima el movimiento de la rueda y los saltos entre mensajes."
		}
		"Scrolling speed" => "Velocidad de desplazamiento",
		"Mouse wheel and trackpad movement. 100% is the default." => {
			"Movimiento de la rueda y del trackpad. 100% es el valor predeterminado."
		}
		"Retry saving reading settings" => "Reintentar guardar las preferencias de lectura",
		"Overview" => "Resumen",
		"Sounds" => "Sonidos",
		"Badges" => "Insignias",
		"Enable Desktop Notifications" => "Activar notificaciones de escritorio",
		"For per-channel or per-server notifications, right-click the channel or server and select Notification Settings." => {
			"Para notificaciones por canal o servidor, haz clic derecho en el canal o servidor y elige Ajustes de notificaciones."
		}
		"Sound Volume" => "Volumen de los sonidos",
		"Adjusts the volume of all notification sounds and ringtones." => {
			"Ajusta el volumen de todos los sonidos de notificación y tonos."
		}
		"Disable All Notification Sounds" => "Desactivar todos los sonidos de notificación",
		"Disables notification sounds. Your individual sound preferences are saved and restored when you turn this off." => {
			"Desactiva los sonidos de notificación. Tus preferencias individuales se guardan y vuelven cuando lo apagas."
		}
		"New Message" => "Mensaje nuevo",
		"New Message in the channel I'm currently reading" => {
			"Mensaje nuevo en el canal que estoy leyendo"
		}
		"Incoming Ring" => "Tono entrante",
		"Outgoing Ring" => "Tono saliente",
		"Microphone Muted" => "Micrófono silenciado",
		"Microphone Unmuted" => "Micrófono activado",
		"Camera On" => "Cámara activada",
		"Screen Share Started" => "Pantalla compartida iniciada",
		"Call Joined" => "Se unió a la llamada",
		"User Left Call" => "Salió de la llamada",
		"Preview Sound" => "Escuchar sonido",
		"Ringtones, call devices and microphone processing." => {
			"Tonos, dispositivos de la llamada y procesamiento del micrófono."
		}
		"Open" => "Abrir",
		"Enable Unread Message Badge" => "Mostrar insignia de mensajes no leídos",
		"Shows a red badge on the app icon when you have unread messages." => {
			"Muestra una insignia roja en el icono de la app cuando hay mensajes no leídos."
		}
		"App icon badges are not available on this platform yet." => {
			"Las insignias del icono de la app aún no están disponibles en esta plataforma."
		}
		_ => return None,
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn migrated_ui_catalog_is_complete_for_both_locales() {
		const KEYS: &[&str] = &[
			"Online",
			"Idle",
			"Do Not Disturb",
			"Invisible",
			"Don't clear",
			"30 minutes",
			"1 hour",
			"4 hours",
			"Today",
			"Switch to",
			"Forget",
			"SereinExt clears it",
			"You",
			"Custom status",
			"Shown next to your name across Discord.",
			"Switch accounts",
			"Add an account",
			"Forget this account on this device",
			"Loading profile…",
			"Reload profile",
			"You will not receive desktop notifications",
			"You will appear offline",
			"Edit custom status",
			"Set a custom status",
			"No custom status",
			"Status text",
			"What's on your mind?",
			"Clear after",
			"Use up to 128 characters without control characters.",
			"Clear",
			"Apply",
			"Search",
			"Close settings (Esc)",
			"Unofficial · not endorsed by Discord",
			"Exit preview",
			"Log out",
			"Your account",
			"Offline preview · synthetic account",
			"Signed in with your Discord account",
			"Display name",
			"Email, password and security",
			"Managed in Discord",
			"Edit profile",
			"Session",
			"Closes the offline fixture. Nothing is stored for the preview.",
			"Removes the saved login and clears this account's local cache and drafts.",
			"Theme",
			"Accent",
			"Primary color",
			"The active theme brings its own accent; it takes over while the theme is in use.",
			"Used for buttons, selection and message highlights.",
			"Reset",
			"Choose primary color",
			"Window effects",
			"Transparency & blur",
			"Restart SereinExt after changing this. Themes can customize effects while enabled.",
			"Transparency",
			"Blur",
			"Zero disables blur; the native compositor controls its exact strength.",
			"Apply to all surfaces",
			"Include sidebars, server rail, headers, and composer.",
			"Channel list",
			"Show hidden channels",
			"Show channels you cannot currently access.",
			"Colour preset",
			"Share game activity",
			"Detect running games and ask Discord to share them as activity.",
			"Enable on Discord",
			"Check again",
			"Looking for a running game",
			"Activity sharing is off",
			"Synthetic activity, never shared or saved.",
			"Local storage",
			"Clear cache",
			"Removes cached messages and media. Drafts and your login stay.",
			"Messages and drafts are cached on this device inside bounded, account-isolated files. Cache data is not encrypted by SereinExt; saved login tokens use the OS credential store.",
			"Your privacy",
			"SereinExt does not collect telemetry or upload diagnostics. Discord retains service-side data according to its own policies.",
			"Offline preview · changes stay in this session and are never sent.",
			"Closing the window keeps SereinExt in the menu bar. Quit from its menu to exit.",
			"Closing keeps SereinExt running. Use the tray to show, minimize or quit.",
			"Closing the window keeps SereinExt in the notification area. Quit from its menu to exit.",
			"Takes effect the next time SereinExt starts.",
			"Use a different account",
			"Waiting for Discord…",
			"Use another account",
			"Continue with Discord",
			"Welcome back",
			"Welcome to SereinExt",
			"Continue with a saved account, or sign in with another one.",
			"Sign in with Discord.",
			"Saved accounts",
			"This is my account",
			"Check this to continue.",
			"Independent and open source. Not affiliated with Discord.",
			"Message",
			"Unread messages",
			"Mark as read",
			"Jump to unread",
			"Copy",
			"Copy message",
			"Copy download link",
			"Reply",
			"Forward",
			"Forward message",
			"Create Thread…",
			"View reactions",
			"Mark read through here",
			"Mark Unread",
			"Unpin message",
			"Pin message",
			"Edit message",
			"Remove from delete selection",
			"Select for batch delete",
			"You can select up to 5 messages at a time.",
			"Delete message…",
			"Delete message immediately",
			"Message history is unavailable with current permission information.",
			"Mute",
			"Unmute",
			"Deafen",
			"Undeafen",
			"Disconnect",
			"Dismiss call",
			"Reconnect to call",
			"Recent call",
			"You were in this call recently",
			"Dismiss",
			"Share your screen",
			"Stop sharing",
			"Turn on camera",
			"Turn off camera",
			"Turn on microphone",
			"Turn off microphone",
			"Turn on incoming audio",
			"Turn off incoming audio",
			"Speaking is unavailable in this channel.",
			"Voice settings",
			"Microphone and speaker settings",
			"Noise suppression",
			"Share a screen or window",
			"Stop sharing your screen",
			"Stop sharing your camera",
			"Share your selected camera with this call",
			"Turn off noise suppression",
			"Turn on noise suppression",
			"Voice processing",
			"Voice processing & input mode",
			"Removes background noise from your microphone before anyone else hears it.",
			"Mode",
			"Off",
			"Light",
			"Standard",
			"Maximum",
			"Bot",
			"Screens",
			"Apps",
			"Options",
			"None open",
			"open",
			"Resolution",
			"Frame rate",
			"Choose a screen or window",
			"Looking for screens and windows…",
			"Offline preview · no screen is captured",
			"In a call",
			"In a call · microphone muted",
			"In a call · deafened",
			"unread mentions",
			"User volume",
			"Reset volume",
			"Silent",
			"Normal",
			"Louder than normal",
			"5% quieter",
			"5% louder",
			"Bots start at 50% to protect your hearing. You can still raise it here.",
			"Start bots at 50% volume",
			"Protects your hearing from bots that join very loud. Right-click a bot in the call to change its volume.",
			"Recommended",
			"All voice settings",
			"No filter. For studio microphones or when playing music.",
			"Steady hum like fans or air conditioning. Lightest on your PC.",
			"Keyboard, clicks and everyday home noise. Works well for most people.",
			"Very noisy home, or friends complain about your background noise. Uses more of your PC.",
			"PC usage: none",
			"PC usage: very low",
			"PC usage: low",
			"PC usage: medium",
			"Friends complaining about noise? Choose Maximum. If your voice cuts out or your PC slows down, go back to Standard.",
			"Your PC couldn't keep up with Maximum, so SereinExt switched to Standard to keep your voice smooth.",
			"The defaults work for most people. Change these only if something sounds wrong.",
			"Click to turn on or off · right-click to choose the level",
			"Noise suppression is unavailable in this build or preview.",
			"Echo cancellation",
			"Recommended when speakers can be picked up by your microphone.",
			"Automatic microphone volume",
			"Keeps speech at a more consistent loudness without changing your output volume.",
			"Push to talk",
			"When enabled, your microphone transmits only while the configured shortcut is held.",
			"Mute and deafen always take priority.",
			"Hold your configured shortcut when you want to speak.",
			"Deafen turns off incoming audio and mutes your microphone with it.",
			"Advanced input settings",
			"Voice activity threshold",
			"Only transmit sound above the threshold.",
			"Open voice activity; mute and push to talk still apply.",
			"Input level",
			"Light suppression strength",
			"Higher levels remove more noise but can affect natural voice detail.",
			"Low",
			"Moderate",
			"High",
			"Very high",
			"Recommended defaults",
			"Raw microphone",
			"Devices & levels",
			"Input device",
			"Output device",
			"Microphone gain",
			"Speaker volume",
			"100% is the original level. Higher levels may distort.",
			"Rescan devices",
			"Reset levels",
			"System default follows your operating-system choice. Select a device only when you want SereinExt to stay pinned to it.",
			"System default (recommended)",
			"Device unavailable — choose another",
			"Looking for audio devices...",
			"Looking for audio devices…",
			"Could not start audio device discovery",
			"Audio devices loaded · headphones avoid microphone echo",
			"Audio device discovery stopped",
			"One selected audio device is unavailable. Choose System default or rescan devices.",
			"Microphone unavailable · choose another input. You are still connected.",
			"Microphone unavailable · still connected. Choose another input in Audio settings.",
			"Camera",
			"Voice privacy code",
			"Compare with the other participants. This code changes with the encrypted call group.",
			"Audio preferences are saved on this device. Your microphone starts only when you join a call or start testing.",
			"Install a voice-enabled build to use these controls.",
			"Friends",
			"Add Friend",
			"You can add friends with their Discord username.",
			"Username",
			"Enter a username",
			"Sending…",
			"Send Friend Request",
			"Offline demo · actions are simulated.",
			"Reconnect before sending a friend request.",
			"All",
			"Pending",
			"Blocked & Ignored",
			"All friends",
			"Blocked & ignored",
			"Blocked and ignored users are not available yet.",
			"Friends are not available yet.",
			"No blocked or ignored users match your search.",
			"No friends match your search.",
			"No blocked or ignored users.",
			"No friends yet.",
			"No friends are currently online.",
			"Some friends’ online status and activity couldn’t be loaded. The Online list may be incomplete.",
			"Dismiss friend status warning",
			"Direct Messages",
			"Mark As Read",
			"Server Settings",
			"Create invite",
			"Leave server",
			"Leave server?",
			"Are you sure you want to leave",
			"You will not be able to rejoin this server unless you are re-invited.",
			"Leaving…",
			"Leave Server",
			"Close",
			"Offline preview · no server changes",
			"Remove From Favorites",
			"Add To Favorites",
			"Favorites are saved on this device.",
			"Invite to Channel",
			"Copy Link",
			"Unmute Channel",
			"Mute Channel",
			"For 15 Minutes",
			"For 1 Hour",
			"For 3 Hours",
			"For 8 Hours",
			"For 24 Hours",
			"Until I Turn It Back On",
			"Copy Channel ID",
			"Hide Muted Channels",
			"Restart to update",
			"Updating…",
			"Update available",
			"Navigation",
			"Move around Serein without reaching for the mouse.",
			"Messages",
			"Composer shortcuts are only active while you are writing.",
			"Text Formatting",
			"Apply or remove formatting in the composer.",
			"Global availability",
			"Show Keyboard Shortcuts List",
			"Switch Conversation",
			"Close Settings or Dialog",
			"Send Message",
			"Insert New Line",
			"Edit Last Editable Message",
			"Bold",
			"Italic",
			"Underline",
			"Strikethrough",
			"Inline Code",
			"Code Block",
			"Spoiler",
			"Push to Talk",
			"Toggle Mute",
			"Toggle Deafen",
			"Voice",
			"Control your microphone and incoming audio during a connected call.",
			"Already bound to",
			"Brazil",
			"United States",
			"Canada",
			"United Kingdom",
			"Germany",
			"Netherlands",
			"France",
			"Spain",
			"Poland",
			"Finland",
			"Sweden",
			"Singapore",
			"Japan",
			"Hong Kong",
			"Australia",
			"India",
			"South Africa",
			"Chile",
			"Argentina",
			"South Korea",
			"Europe",
			"Russia",
			"People in this call will see what you pick.",
			"Looking for your screens…",
			"Entire screen",
			"Share audio",
			"Also send sound from other apps. Your microphone stays as it is.",
			"Share an app",
			"No apps are open to share.",
			"App",
			"Refresh",
			"Quality",
			"Show cursor",
			"Include the pointer in the shared video.",
			"Share Screen",
			"Cancel",
			"Before you use SereinExt",
			"SereinExt is an unofficial app for your own Discord account: it is not Discord, it is not endorsed by Discord, and Discord's rules still apply to your account. It also includes other people's work (libraries, fonts and icons) under their own licenses, and accepting here does not waive those licenses or shift their copyright. Continuing confirms that you understand both points.",
			"Your app preferences could not be read, so SereinExt cannot tell whether you accepted this before.",
			"View full licenses",
			"Hide full licenses",
			"App preferences were not saved. If your acceptance of the terms was not recorded, SereinExt will ask again next launch.",
			"Your session expired; sign in again to continue.",
			"I understand — continue",
		];
		for key in KEYS {
			assert!(portuguese_brazil(key).is_some(), "missing pt-BR: {key}");
			assert!(spanish(key).is_some(), "missing es: {key}");
		}
	}

	#[test]
	fn locales_fall_back_to_english_without_empty_controls() {
		assert_eq!(
			text(Language::PortugueseBrazil, "Voice & Video"),
			"Voz e vídeo"
		);
		assert_eq!(text(Language::Spanish, "Voice & Video"), "Voz y video");
		assert_eq!(
			text(Language::Spanish, "Untranslated sentinel"),
			"Untranslated sentinel"
		);
	}
}
