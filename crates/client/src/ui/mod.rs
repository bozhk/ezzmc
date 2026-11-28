//! GUI главного меню (VANILLA: TitleScreen, JoinMultiplayerScreen, OptionsScreen и подэкраны).
//! Режим «немедленного рисования»: каждый кадр экран объявляет виджеты, клик обрабатывается на месте.
pub mod bookedit;
pub mod cmdblock;
pub mod dialog;
pub mod jigsaw;
pub mod lang;
pub mod options;
pub mod ping;
pub mod servers;
pub mod signedit;
pub mod social;
pub mod structblock;
pub mod testblock;

use crate::editbox::{EditBox, Move};
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;
use lang::Lang;
use options::{Def, Kind, Options};
use servers::Server;
use std::path::PathBuf;

const WHITE: u32 = 0xFFFFFF;
const GRAY: u32 = 0xA0A0A0;

/// Текстуры GUI: (имя в рисовалке, путь внутри `assets/minecraft/textures/gui/`).
pub const TEXTURES: &[(&str, &str)] = &[
    ("button", "sprites/widget/button"),
    ("button_hl", "sprites/widget/button_highlighted"),
    ("button_off", "sprites/widget/button_disabled"),
    ("slider", "sprites/widget/slider"),
    ("slider_hl", "sprites/widget/slider_highlighted"),
    ("handle", "sprites/widget/slider_handle"),
    ("handle_hl", "sprites/widget/slider_handle_highlighted"),
    ("logo", "title/minecraft"),
    ("edition", "title/edition"),
    ("overlay", "title/background/panorama_overlay"),
    ("list_bg", "menu_list_background"),
    ("header_sep", "header_separator"),
    ("footer_sep", "footer_separator"),
    ("menu_bg", "menu_background"),
    ("iw_menu_bg", "inworld_menu_background"),
    ("iw_list_bg", "inworld_menu_list_background"),
    ("iw_header_sep", "inworld_header_separator"),
    ("iw_footer_sep", "inworld_footer_separator"),
    ("dlg_warn", "sprites/dialog/warning_button"),
    ("dlg_warn_hl", "sprites/dialog/warning_button_highlighted"),
    ("dlg_check", "sprites/widget/checkbox"),
    ("dlg_check_hl", "sprites/widget/checkbox_highlighted"),
    ("dlg_check_on", "sprites/widget/checkbox_selected"),
    (
        "dlg_check_on_hl",
        "sprites/widget/checkbox_selected_highlighted",
    ),
    ("sign_acacia", "signs/acacia"),
    ("hsign_acacia", "hanging_signs/acacia"),
    ("sign_bamboo", "signs/bamboo"),
    ("hsign_bamboo", "hanging_signs/bamboo"),
    ("sign_birch", "signs/birch"),
    ("hsign_birch", "hanging_signs/birch"),
    ("sign_cherry", "signs/cherry"),
    ("hsign_cherry", "hanging_signs/cherry"),
    ("sign_crimson", "signs/crimson"),
    ("hsign_crimson", "hanging_signs/crimson"),
    ("sign_dark_oak", "signs/dark_oak"),
    ("hsign_dark_oak", "hanging_signs/dark_oak"),
    ("sign_jungle", "signs/jungle"),
    ("hsign_jungle", "hanging_signs/jungle"),
    ("sign_mangrove", "signs/mangrove"),
    ("hsign_mangrove", "hanging_signs/mangrove"),
    ("sign_oak", "signs/oak"),
    ("hsign_oak", "hanging_signs/oak"),
    ("sign_pale_oak", "signs/pale_oak"),
    ("hsign_pale_oak", "hanging_signs/pale_oak"),
    ("sign_poplar", "signs/poplar"),
    ("hsign_poplar", "hanging_signs/poplar"),
    ("sign_spruce", "signs/spruce"),
    ("hsign_spruce", "hanging_signs/spruce"),
    ("sign_warped", "signs/warped"),
    ("hsign_warped", "hanging_signs/warped"),
    ("srv_join", "sprites/server_list/join"),
    ("srv_join_hl", "sprites/server_list/join_highlighted"),
    ("srv_up", "sprites/server_list/move_up"),
    ("srv_up_hl", "sprites/server_list/move_up_highlighted"),
    ("srv_down", "sprites/server_list/move_down"),
    ("srv_down_hl", "sprites/server_list/move_down_highlighted"),
    ("srv_incompatible", "sprites/server_list/incompatible"),
    ("srv_unreachable", "sprites/server_list/unreachable"),
    ("srv_ping_1", "sprites/server_list/ping_1"),
    ("srv_ping_2", "sprites/server_list/ping_2"),
    ("srv_ping_3", "sprites/server_list/ping_3"),
    ("srv_ping_4", "sprites/server_list/ping_4"),
    ("srv_ping_5", "sprites/server_list/ping_5"),
    ("srv_pinging_1", "sprites/server_list/pinging_1"),
    ("srv_pinging_2", "sprites/server_list/pinging_2"),
    ("srv_pinging_3", "sprites/server_list/pinging_3"),
    ("srv_pinging_4", "sprites/server_list/pinging_4"),
    ("srv_pinging_5", "sprites/server_list/pinging_5"),
    ("icon_language", "sprites/icon/language"),
    ("icon_access", "sprites/icon/accessibility"),
    ("panorama_0", "title/background/panorama_0"),
    ("panorama_1", "title/background/panorama_1"),
    ("panorama_2", "title/background/panorama_2"),
    ("panorama_3", "title/background/panorama_3"),
    ("panorama_4", "title/background/panorama_4"),
    ("panorama_5", "title/background/panorama_5"),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Video,
    Sound,
    Skin,
    Chat,
    Accessibility,
    Controls,
    Mouse,
    KeyBinds,
    Language,
    ResourcePacks,
    Telemetry,
    Credits,
}

impl Page {
    fn title(self) -> &'static str {
        match self {
            Page::Video => "options.videoTitle",
            Page::Sound => "options.sounds.title",
            Page::Skin => "options.skinCustomisation.title",
            Page::Chat => "options.chat.title",
            Page::Accessibility => "options.accessibility.title",
            Page::Controls => "controls.title",
            Page::Mouse => "options.mouse_settings.title",
            Page::KeyBinds => "controls.keybinds.title",
            Page::Language => "options.language.title",
            Page::ResourcePacks => "resourcePack.title",
            Page::Telemetry => "telemetry_info.screen.title",
            Page::Credits => "credits_and_attribution.screen.title",
        }
    }

    fn defs(self) -> &'static [Def] {
        match self {
            Page::Video => options::VIDEO,
            Page::Sound => options::SOUND,
            Page::Skin => options::SKIN,
            Page::Chat => options::CHAT,
            Page::Accessibility => options::ACCESSIBILITY,
            Page::Controls => options::CONTROLS,
            Page::Mouse => options::MOUSE,
            _ => &[],
        }
    }
}

/// VANILLA: LevelLoadingScreen.Reason — фон экрана загрузки уровня.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LoadingReason {
    /// Спрайт nether_portal на весь экран.
    NetherPortal,
    /// END_PORTAL fill (end_sky + end_portal) на весь экран.
    EndPortal,
    /// Панорама + фон меню.
    #[default]
    Other,
}

impl LoadingReason {
    /// VANILLA: ClientPacketListener#determineLevelLoadingReason.
    pub fn determine(player_died: bool, dim: &str, old_dim: &str) -> Self {
        const NETHER: &str = "minecraft:the_nether";
        const END: &str = "minecraft:the_end";
        if player_died {
            Self::Other
        } else if dim == NETHER || old_dim == NETHER {
            Self::NetherPortal
        } else if dim == END || old_dim == END {
            Self::EndPortal
        } else {
            Self::Other
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Title,
    Multiplayer,
    Direct,
    /// `None` — новый сервер.
    EditServer(Option<usize>),
    Singleplayer,
    Options,
    Page(Page),
    /// Меню паузы (в игре).
    Pause,
    /// VANILLA: DisconnectedScreen — «Connection Lost», причина, «Back to Server List».
    Disconnected,
    /// Диалог сервера (данные — `State.dlg`).
    Dialog,
    /// VANILLA: ConfirmLinkScreen (untrusted) — URL в `State.link`.
    LinkConfirm,
    /// VANILLA: SignEditScreen/HangingSignEditScreen (данные — `State.sign`).
    Sign,
    /// VANILLA: BookEditScreen/BookSignScreen (данные — `State.book`).
    Book,
    /// VANILLA: CommandBlockEditScreen (данные — `State.cmd`).
    CmdBlock,
    /// VANILLA: StructureBlockEditScreen (данные — `State.structure`).
    Structure,
    /// VANILLA: JigsawBlockEditScreen (данные — `State.jigsaw`).
    Jigsaw,
    /// VANILLA: TestBlockEditScreen (данные — `State.testblock`).
    TestBlock,
    /// VANILLA: SocialInteractionsScreen (данные — `State.social`).
    Social,
    /// VANILLA: DeathScreen (данные — `State.death`).
    Death,
    /// VANILLA: DeathScreen.TitleConfirmScreen — «Вы уверены, что хотите выйти?».
    DeathConfirm,
    /// VANILLA: ConnectScreen — «Connecting to the server...» и «Cancel».
    Connecting,
    /// VANILLA: LevelLoadingScreen — «Loading terrain...» до готовности уровня.
    Loading,
}

/// Рабочий порт локального сервера (xtask server 26.3 25611).
pub const DEFAULT_SERVER: &str = "localhost:25611";

pub enum Action {
    None,
    /// Подключиться к `host[:port]`.
    Join(String),
    Quit,
    /// Закрыть меню паузы и вернуться в игру.
    Resume,
    /// VANILLA: LocalPlayer#respawn — ServerboundClientCommand(PERFORM_RESPAWN).
    Respawn,
    /// Выйти с сервера в главное меню.
    Disconnect,
    /// VANILLA: LanguageSelectScreen#onDone — применить язык (код `ru_ru`).
    SetLanguage(String),
    /// Набор ресурспаков изменён — перезагрузить ресурсы (перезапуск процесса).
    ReloadResources,
    /// Последний диалог закрыт — вернуться в игру (без пересборки ресурсов).
    CloseDialog,
    /// Экран правки таблички закрыт (`take_sign_out` отдаёт текст).
    CloseSign,
    /// Экран командного блока закрыт (`take_cmd_out` отдаёт данные, если Готово).
    CloseCmdBlock,
    /// Экран структурного блока закрыт (`take_structure_out`).
    CloseStructure,
    CloseJigsaw,
    CloseTestBlock,
    CloseSocial,
    /// Экран книги закрыт (`take_book_out` отдаёт страницы, если сохранено).
    CloseBook,
    /// VANILLA: ConfirmLinkScreen → Blaze3D.openUri.
    OpenLink(String),
    /// VANILLA: PauseScreen → диалог Dialogs.SERVER_LINKS.
    OpenServerLinks,
    /// VANILLA: PauseScreen → AdvancementsScreen.
    OpenAdv,
    /// VANILLA: PauseScreen → StatsScreen.
    OpenStats,
    /// VANILLA: ConnectScreen «Cancel» — оборвать подключение, назад к списку серверов.
    CancelConnect,
}

struct State {
    stack: Vec<Screen>,
    servers: Vec<Server>,
    sel: Option<usize>,
    scroll: f32,
    fields: [EditBox; 2],
    focus: usize,
    last_click: (Option<usize>, u64),
    confirm_delete: bool,
    servers_path: PathBuf,
    pinger: ping::Pinger,
    splash: String,
    /// Список доступных ресурспаков (скан `resourcepacks/` при входе на экран).
    packs_avail: Option<Vec<crate::packrepo::PackInfo>>,
    /// Строка поиска, пак на подтверждении (несовместимый), выбор при входе на экран, ввод кадра.
    pack_search: EditBox,
    /// Экран языка: список (код, «Название (Регион)»), выбранный код.
    langs: Vec<(String, String)>,
    lang_sel: String,
    pack_confirm: Option<String>,
    pack_initial: Option<String>,
    pack_typed: String,
    pack_keys: Vec<UiKey>,
    /// Текст для системного буфера (Ctrl+C/X в полях меню); забирает main.
    clip_out: Option<String>,
    link: Option<String>,
    /// Причина отключения для `Screen::Disconnected`.
    kick: String,
    /// Заголовок `connect.failed` (отказ до входа в игру) вместо `disconnect.lost`.
    kick_connect_failed: bool,
    /// Текущий диалог сервера и исходящие действия его кнопок.
    dlg: Option<dialog::Scr>,
    dlg_out: Vec<dialog::Out>,
    sign: Option<signedit::SignEdit>,
    book: Option<bookedit::BookEdit>,
    cmd: Option<cmdblock::CmdBlock>,
    cmd_out: Option<cmdblock::CmdOut>,
    structure: Option<structblock::StructBlock>,
    structure_out: Option<structblock::StructOut>,
    jigsaw: Option<jigsaw::Jigsaw>,
    jigsaw_out: Option<jigsaw::JigsawOut>,
    testblock: Option<testblock::TestBlock>,
    social: Option<social::Social>,
    /// Игроки для экрана (main обновляет каждый кадр) и скрытые UUID.
    social_players: Vec<social::Player>,
    social_hidden: std::collections::HashSet<u128>,
    /// Запросы скрыть/показать: (uuid, скрыть, имя) — разбирает main.
    social_toggle: Vec<(u128, bool, String)>,
    testblock_out: Option<testblock::TestOut>,
    book_out: Option<bookedit::BookOut>,
    sign_out: Option<([i32; 3], bool, [String; 4])>,
    /// VANILLA: DeathScreen — причина (строка с кодами `§`), hardcore, счёт, момент открытия (кнопки активны через 20 тиков).
    death: Option<Death>,
}

struct Death {
    cause: Option<String>,
    hardcore: bool,
    score: i32,
    opened: std::time::Instant,
    /// Нажата «Respawn»/«Spectate World» (`button.active = false`).
    sent: bool,
    /// Момент открытия экрана подтверждения.
    confirm: std::time::Instant,
}

pub struct Gui {
    st: State,
    lang: Lang,
    pub opts: Options,
    mx: f32,
    my: f32,
    click: bool,
    down: bool,
    drag: Option<u32>,
    wheel: f32,
    pending_keys: Vec<UiKey>,
    pending_text: String,
    spin: f32,
    in_game: bool,
    /// Фон `Screen::Loading` (VANILLA: LevelLoadingScreen#reason).
    loading_reason: LoadingReason,
    /// Строки экрана привязок клавиш (App обновляет, пока `wants_binds`).
    pub binds: Vec<BindRow>,
    /// Привязка, ожидающая нажатия клавиши/кнопки (её ловит App).
    pub listening: Option<&'static str>,
    pub reset_keys: bool,
    pub reset_one: Option<&'static str>,
}

/// Строка экрана привязок. `key` — ключ перевода (`key.keyboard.w`).
pub struct BindRow {
    pub name: &'static str,
    pub category: &'static str,
    pub key: String,
    pub conflict: bool,
    pub default: bool,
}

#[derive(Clone, Copy)]
pub enum UiKey {
    Escape,
    Enter,
    Up,
    Down,
    Tab,
    BackTab,
    /// Backspace/Delete: (направление, по слову).
    Erase(i32, bool),
    /// Стрелки/Home/End: (куда, по слову, с выделением).
    Move(Move, bool, bool),
    SelectAll,
    Copy,
    Cut,
}

fn edit_with(s: &str) -> EditBox {
    let mut e = EditBox::new(128);
    e.set(s);
    e
}

/// VANILLA: EditBox#keyPressed + charTyped для одного поля.
fn edit_apply(e: &mut EditBox, keys: &[UiKey], typed: &str, clip: &mut Option<String>) {
    for k in keys {
        match *k {
            UiKey::Erase(d, w) => {
                e.erase(d, w);
            }
            UiKey::Move(m, w, s) => e.move_cursor(m, w, s),
            UiKey::SelectAll => e.select_all(),
            UiKey::Copy => *clip = Some(e.selected_text()),
            UiKey::Cut => *clip = Some(e.cut()),
            _ => {}
        }
    }
    // type_text("") удалил бы выделение (и Ctrl+A, Shift+стрелки «стирали» бы текст).
    if !typed.is_empty() {
        e.type_text(typed);
    }
}

fn cur(st: &State) -> Screen {
    *st.stack.last().unwrap_or(&Screen::Title)
}

impl Gui {
    pub fn new(lang_json: &[u8], splashes: &str, dir: &std::path::Path) -> Self {
        let servers_path = dir.join("servers.dat");
        // Первый запуск: локальный сервер в списке по умолчанию.
        if !servers_path.exists() {
            servers::save(
                &servers_path,
                &[servers::Server {
                    name: "Local Server".into(),
                    ip: DEFAULT_SERVER.into(),
                }],
            );
        }
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos() as usize);
        let lines: Vec<&str> = splashes.lines().filter(|l| !l.trim().is_empty()).collect();
        let splash = if lines.is_empty() {
            String::new()
        } else {
            lines[seed % lines.len()].to_owned()
        };
        // Отладка: EZZMC_MENU_SCREEN=options|video|sound|language|multiplayer — открыть экран сразу.
        let mut stack = vec![Screen::Title];
        match std::env::var("EZZMC_MENU_SCREEN").as_deref() {
            Ok("options") => stack.push(Screen::Options),
            Ok("video") => stack.extend([Screen::Options, Screen::Page(Page::Video)]),
            Ok("sound") => stack.extend([Screen::Options, Screen::Page(Page::Sound)]),
            Ok("language") => stack.extend([Screen::Page(Page::Language)]),
            Ok("controls") => stack.extend([Screen::Options, Screen::Page(Page::Controls)]),
            Ok("keybinds") => stack.extend([
                Screen::Options,
                Screen::Page(Page::Controls),
                Screen::Page(Page::KeyBinds),
            ]),
            Ok("mouse") => stack.extend([
                Screen::Options,
                Screen::Page(Page::Controls),
                Screen::Page(Page::Mouse),
            ]),
            Ok("packs") => stack.extend([Screen::Options, Screen::Page(Page::ResourcePacks)]),
            Ok("multiplayer") => stack.push(Screen::Multiplayer),
            _ => {}
        }
        Self {
            st: State {
                stack,
                servers: servers::load(&servers_path),
                sel: None,
                scroll: 0.0,
                fields: [EditBox::new(128), edit_with(DEFAULT_SERVER)],
                focus: 0,
                last_click: (None, 0),
                confirm_delete: false,
                servers_path,
                pinger: ping::Pinger::default(),
                splash,
                packs_avail: None,
                pack_search: EditBox::new(64),
                langs: Vec::new(),
                lang_sel: String::new(),
                pack_confirm: None,
                pack_initial: None,
                pack_typed: String::new(),
                pack_keys: Vec::new(),
                clip_out: None,
                link: None,
                kick: String::new(),
                kick_connect_failed: false,
                death: None,
                dlg: None,
                dlg_out: Vec::new(),
                sign: None,
                sign_out: None,
                cmd: None,
                cmd_out: None,
                structure: None,
                structure_out: None,
                jigsaw: None,
                jigsaw_out: None,
                testblock: None,
                social: None,
                social_players: Vec::new(),
                social_hidden: Default::default(),
                social_toggle: Vec::new(),
                testblock_out: None,
                book: None,
                book_out: None,
            },
            lang: Lang::load(lang_json),
            opts: Options::load(&options::path_in(dir)),
            mx: 0.0,
            my: 0.0,
            click: false,
            down: false,
            drag: None,
            wheel: 0.0,
            pending_keys: Vec::new(),
            pending_text: String::new(),
            spin: 0.0,
            in_game: false,
            loading_reason: LoadingReason::Other,
            binds: Vec::new(),
            listening: None,
            reset_keys: false,
            reset_one: None,
        }
    }

    /// Список языков из `pack.mcmeta` индекса ассетов (VANILLA: LanguageManager#getLanguages).
    pub fn set_languages(&mut self, v: Vec<(String, String)>) {
        self.st.langs = v;
    }

    pub fn set_lang(&mut self, l: Lang) {
        self.lang = l;
    }

    /// Показать диалог сервера (VANILLA: ClientPacketListener#showDialog).
    pub fn open_dialog(&mut self, d: Box<crate::dialog::Dialog>) {
        self.st.dlg = Some(dialog::open(*d));
        self.st.stack = vec![Screen::Dialog];
        self.st.scroll = 0.0;
        self.in_game = true;
    }

    /// clear_dialog. true — экран диалога был открыт и закрыт (надо вернуться в игру).
    pub fn open_server_links(&mut self) {
        if let Some(d) = crate::dialog::server_links_dialog() {
            self.open_dialog(Box::new(d));
        }
    }

    /// VANILLA: ChatScreen#handleComponentClicked → open_url → ConfirmLinkScreen(untrusted).
    /// VANILLA-UNSURE: ваниль возвращает в чат после выбора, мы — сразу в игру.
    pub fn open_link_confirm(&mut self, url: String) {
        self.st.link = Some(url);
        self.st.stack = vec![Screen::LinkConfirm];
        self.in_game = true;
    }

    pub fn close_dialog(&mut self) -> bool {
        let had = self.st.dlg.take().is_some();
        let shown = had && cur(&self.st) == Screen::Dialog;
        if shown {
            self.st.stack.clear();
        }
        shown
    }

    /// Для MCP: (заголовок, вид, подписи кнопок) открытого диалога.
    pub fn dialog_info(&self) -> Option<(String, &'static str, Vec<String>)> {
        let scr = self
            .st
            .dlg
            .as_ref()
            .filter(|_| cur(&self.st) == Screen::Dialog)?;
        Some(dialog::info(scr, &self.lang))
    }

    /// Для MCP: нажать кнопку диалога по индексу.
    pub fn dialog_press(&mut self, n: usize) -> bool {
        if cur(&self.st) != Screen::Dialog {
            return false;
        }
        self.st.dlg.as_mut().is_some_and(|s| {
            dialog::press(s, n);
            true
        })
    }

    /// Открыть редактор таблички (VANILLA: ClientPacketListener#handleOpenSignEditor).
    pub fn open_sign(
        &mut self,
        pos: [i32; 3],
        front: bool,
        name: &str,
        lines: &[String; 4],
        color: u32,
    ) {
        self.st.dlg = None;
        self.st.sign = Some(signedit::open(pos, front, name, lines, color));
        self.st.stack = vec![Screen::Sign];
        self.in_game = true;
    }

    /// Открыть экран командного блока (VANILLA: LocalPlayer#openCommandBlock).
    pub fn open_cmd_block(&mut self, pos: [i32; 3], name: &str) {
        self.st.dlg = None;
        self.st.cmd = Some(cmdblock::open(pos, name));
        self.st.stack = vec![Screen::CmdBlock];
        self.in_game = true;
    }

    /// Пришёл block_entity_data для открытого командного блока (VANILLA: updateGui).
    pub fn cmd_block_data(
        &mut self,
        pos: [i32; 3],
        conditional: bool,
        tag: Option<&ezzmc_protocol::nbt::Tag>,
    ) {
        if let Some(c) = self.st.cmd.as_mut().filter(|c| c.pos == pos) {
            c.update(conditional, tag);
        }
    }

    /// Позиция открытого командного блока.
    pub fn cmd_block_pos(&self) -> Option<[i32; 3]> {
        self.st.cmd.as_ref().map(|c| c.pos)
    }

    pub fn take_cmd_out(&mut self) -> Option<cmdblock::CmdOut> {
        self.st.cmd_out.take()
    }

    pub fn close_cmd_block(&mut self) {
        self.st.cmd = None;
        if cur(&self.st) == Screen::CmdBlock {
            self.st.stack.clear();
        }
    }

    /// VANILLA: LocalPlayer#openStructureBlock.
    pub fn open_structure(&mut self, pos: [i32; 3], tag: Option<&ezzmc_protocol::nbt::Tag>) {
        self.st.dlg = None;
        let mut s = structblock::open(pos);
        s.update(tag);
        self.st.structure = Some(s);
        self.st.stack = vec![Screen::Structure];
        self.in_game = true;
    }

    pub fn take_structure_out(&mut self) -> Option<structblock::StructOut> {
        self.st.structure_out.take()
    }

    pub fn close_structure(&mut self) {
        self.st.structure = None;
        if cur(&self.st) == Screen::Structure {
            self.st.stack.clear();
        }
    }

    /// VANILLA: LocalPlayer#openJigsawBlock.
    pub fn open_jigsaw(
        &mut self,
        pos: [i32; 3],
        orientation: &str,
        tag: Option<&ezzmc_protocol::nbt::Tag>,
    ) {
        self.st.dlg = None;
        self.st.jigsaw = Some(jigsaw::open(pos, orientation, tag));
        self.st.stack = vec![Screen::Jigsaw];
        self.in_game = true;
    }

    /// VANILLA: Minecraft#handleKeybinds — key.socialInteractions → SocialInteractionsScreen.
    pub fn open_social(&mut self, me: u128, server: String) {
        self.st.dlg = None;
        self.st.social = Some(social::open(me, server));
        self.st.stack = vec![Screen::Social];
        self.in_game = true;
    }

    /// Обновить данные открытого экрана; true — экран открыт.
    pub fn social_sync(
        &mut self,
        players: Vec<social::Player>,
        hidden: &std::collections::HashSet<u128>,
    ) -> bool {
        if self.st.social.is_none() {
            return false;
        }
        self.st.social_players = players;
        self.st.social_hidden = hidden.clone();
        true
    }

    pub fn take_social_toggles(&mut self) -> Vec<(u128, bool, String)> {
        std::mem::take(&mut self.st.social_toggle)
    }

    pub fn is_social(&self) -> bool {
        self.st.social.is_some()
    }

    pub fn close_social(&mut self) {
        self.st.social = None;
        if cur(&self.st) == Screen::Social {
            self.st.stack.clear();
        }
    }

    /// VANILLA: LocalPlayer#openTestBlock.
    pub fn open_testblock(&mut self, pos: [i32; 3], tag: Option<&ezzmc_protocol::nbt::Tag>) {
        self.st.dlg = None;
        self.st.testblock = Some(testblock::open(pos, tag));
        self.st.stack = vec![Screen::TestBlock];
        self.in_game = true;
    }

    pub fn take_testblock_out(&mut self) -> Option<testblock::TestOut> {
        self.st.testblock_out.take()
    }

    pub fn close_testblock(&mut self) {
        self.st.testblock = None;
        if cur(&self.st) == Screen::TestBlock {
            self.st.stack.clear();
        }
    }

    pub fn take_jigsaw_out(&mut self) -> Option<jigsaw::JigsawOut> {
        self.st.jigsaw_out.take()
    }

    pub fn close_jigsaw(&mut self) {
        self.st.jigsaw = None;
        if cur(&self.st) == Screen::Jigsaw {
            self.st.stack.clear();
        }
    }

    /// Открыт экран правки книги (для агента).
    pub fn is_book(&self) -> bool {
        cur(&self.st) == Screen::Book
    }

    /// Открыть экран правки книги с пером (VANILLA: LocalPlayer#openItemGui → BookEditScreen).
    pub fn open_book(&mut self, slot: i32, pages: Vec<String>, author: &str) {
        self.st.dlg = None;
        self.st.book = Some(bookedit::open(slot, pages, author));
        self.st.stack = vec![Screen::Book];
        self.in_game = true;
    }

    /// Сохранённая книга (Готово/Подписать): отправить ServerboundEditBookPacket.
    pub fn take_book_out(&mut self) -> Option<bookedit::BookOut> {
        self.st.book_out.take()
    }

    /// Закрыть экран книги (после `Action::CloseBook`).
    pub fn close_book(&mut self) {
        self.st.book = None;
        if cur(&self.st) == Screen::Book {
            self.st.stack.clear();
        }
    }

    /// Текст закрытой таблички: (позиция, передняя сторона, строки) — отправить ServerboundSignUpdate.
    pub fn take_sign_out(&mut self) -> Option<([i32; 3], bool, [String; 4])> {
        self.st.sign_out.take()
    }

    /// Закрыть экран правки (после `Action::CloseSign`).
    pub fn close_sign(&mut self) {
        self.st.sign = None;
        if cur(&self.st) == Screen::Sign {
            self.st.stack.clear();
        }
    }

    /// Исходящие действия кнопок диалога (команды, custom click).
    pub fn take_dialog_out(&mut self) -> Vec<dialog::Out> {
        std::mem::take(&mut self.st.dlg_out)
    }

    pub fn wants_binds(&self) -> bool {
        // Строки нужны и на странице Controls, и на экране самих привязок (иначе они не обновляются).
        matches!(cur(&self.st), Screen::Page(Page::Controls | Page::KeyBinds))
    }

    /// VANILLA: Minecraft#pauseGame — меню паузы поверх мира.
    pub fn open_pause(&mut self) {
        self.st.dlg = None;
        self.st.stack = vec![Screen::Pause];
        self.st.scroll = 0.0;
        self.in_game = true;
    }

    /// VANILLA: ConnectScreen — подключение (родитель — список серверов).
    pub fn open_connecting(&mut self) {
        self.st.dlg = None;
        self.st.stack = vec![Screen::Title, Screen::Multiplayer, Screen::Connecting];
        self.st.scroll = 0.0;
        self.in_game = false;
    }

    /// VANILLA: LevelLoadingScreen — «Loading terrain...».
    pub fn open_loading(&mut self) {
        self.open_loading_reason(LoadingReason::Other);
    }

    /// VANILLA: ClientPacketListener#startWaitingForNewLevel — открыть LevelLoadingScreen
    /// (уже открыт — LevelLoadingScreen#update меняет только причину).
    pub fn open_loading_reason(&mut self, reason: LoadingReason) {
        self.loading_reason = reason;
        self.st.dlg = None;
        self.st.stack = vec![Screen::Loading];
        self.st.scroll = 0.0;
        self.in_game = false;
    }

    /// Фон текущего экрана загрузки, если он открыт и рисуется не меню (портал Нижнего мира/Энда).
    pub fn loading_portal(&self) -> Option<LoadingReason> {
        (cur(&self.st) == Screen::Loading && self.loading_reason != LoadingReason::Other)
            .then_some(self.loading_reason)
    }

    /// Экран подключения/загрузки уровня сейчас активен.
    pub fn is_connecting(&self) -> bool {
        matches!(cur(&self.st), Screen::Connecting | Screen::Loading)
    }

    /// Меню паузы сейчас активно (NowPlayingToast рисуется внутри него).
    pub fn is_pause(&self) -> bool {
        matches!(cur(&self.st), Screen::Pause)
    }

    /// Вернуться к списку серверов (после «Cancel»).
    pub fn back_to_servers(&mut self) {
        self.st.dlg = None;
        self.st.stack = vec![Screen::Title, Screen::Multiplayer];
        self.in_game = false;
    }

    /// VANILLA: DisconnectedScreen — показать причину отключения (родитель — список серверов).
    pub fn open_disconnected(&mut self, reason: String, connect_failed: bool) {
        self.st.dlg = None;
        self.st.stack = vec![Screen::Title, Screen::Multiplayer, Screen::Disconnected];
        self.st.kick = reason;
        self.st.kick_connect_failed = connect_failed;
        self.st.scroll = 0.0;
        self.st.sel = None;
        self.in_game = false;
    }

    /// VANILLA: ClientPacketListener#handlePlayerCombatKill → `new DeathScreen(message, hardcore, player)`.
    pub fn open_death(
        &mut self,
        cause: Option<&ezzmc_protocol::text::Component>,
        hardcore: bool,
        score: i32,
    ) {
        self.st.dlg = None;
        let cause = cause.map(|c| {
            crate::chat::spans_to_legacy(&crate::chat::flatten(c, &self.lang, true), WHITE)
        });
        let now = std::time::Instant::now();
        self.st.death = Some(Death {
            cause,
            hardcore,
            score,
            opened: now,
            sent: false,
            confirm: now,
        });
        self.st.stack = vec![Screen::Death];
        self.st.scroll = 0.0;
        self.in_game = true;
    }

    /// Закрыть экран смерти (респавн состоялся).
    pub fn close_death(&mut self) {
        self.st.death = None;
        self.st.stack = vec![Screen::Pause];
    }

    /// Вернуться в главное меню (после выхода с сервера).
    pub fn reset_title(&mut self) {
        self.st.dlg = None;
        self.st.stack = vec![Screen::Title];
        self.in_game = false;
    }

    /// Курсор в физических пикселях.
    pub fn on_cursor(&mut self, x: f64, y: f64) {
        self.mx = x as f32;
        self.my = y as f32;
    }

    pub fn on_left_button(&mut self, pressed: bool) {
        if pressed {
            self.click = true;
        } else {
            self.drag = None;
        }
        self.down = pressed;
    }

    pub fn on_wheel(&mut self, y: f32) {
        self.wheel += y;
    }

    pub fn on_key(&mut self, k: UiKey) {
        self.pending_keys.push(k);
    }

    /// Файл брошен в окно (VANILLA: PackSelectionScreen#onFilesDrop) — только на экране паков.
    pub fn on_drop(&mut self, p: &std::path::Path) {
        if cur(&self.st) == Screen::Page(Page::ResourcePacks) && crate::packrepo::import(p) {
            self.st.packs_avail = None;
        }
    }

    /// Текст для системного буфера обмена после Ctrl+C/X в поле меню.
    pub fn take_clip(&mut self) -> Option<String> {
        self.st.clip_out.take()
    }

    pub fn on_text(&mut self, s: &str) {
        self.pending_text
            .extend(s.chars().filter(|c| !c.is_control()));
    }

    fn push(&mut self, s: Screen) {
        self.st.stack.push(s);
        self.st.scroll = 0.0;
        self.st.sel = None;
        self.st.confirm_delete = false;
        self.st.packs_avail = None;
    }

    fn pop(&mut self) {
        if self.st.stack.len() > 1 {
            self.st.stack.pop();
        }
        self.st.scroll = 0.0;
        self.st.confirm_delete = false;
        self.st.packs_avail = None;
        self.opts.save();
    }

    /// Рисует текущий экран. Возвращает действие для приложения.
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &mut self,
        dev: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        w: u32,
        h: u32,
        t: &mut TextRenderer,
        p: &mut GuiPainter,
        dt_s: f32,
        now_ms: u64,
    ) -> Action {
        let speed = self.opts.f32(
            options::ACCESSIBILITY
                .iter()
                .find(|d| d.key == "panoramaScrollSpeed")
                .expect("panorama"),
        );
        self.spin = (self.spin + dt_s * 20.0 * speed * 0.1) % 360.0;
        let guiscale = self.opts.f32(&options::VIDEO[options::GUI_SCALE]) as u32;
        t.set_scale_option(guiscale);
        t.begin(w, h);
        p.begin(t.scale());
        let k = t.scale() as f32;
        let (gw, gh) = (w as f32 / k, h as f32 / k);
        let mut action = Action::None;
        let keys = std::mem::take(&mut self.pending_keys);
        let typed = std::mem::take(&mut self.pending_text);
        if cur(&self.st) == Screen::Page(Page::Language) {
            self.st.pack_typed = typed.clone();
            self.st.pack_keys = keys.clone();
        }
        if cur(&self.st) == Screen::Page(Page::ResourcePacks) {
            let st = &mut self.st;
            if st.packs_avail.is_none() {
                let list = crate::packrepo::scan();
                for pk in &list {
                    if let Some((px, iw, ih)) = &pk.icon {
                        p.add_texture(
                            dev,
                            queue,
                            &format!("packicon:{}", pk.id),
                            px,
                            *iw,
                            *ih,
                            false,
                        );
                    }
                }
                st.packs_avail = Some(list);
            }
            if st.pack_initial.is_none() {
                st.pack_initial = Some(
                    self.opts
                        .raw("resourcePacks")
                        .unwrap_or("[\"vanilla\"]")
                        .to_owned(),
                );
            }
            st.pack_typed = typed.clone();
            st.pack_keys = keys.clone();
        }
        self.st.pinger.poll(&self.lang, dev, queue, p);
        let wheel = std::mem::take(&mut self.wheel);
        let click = std::mem::take(&mut self.click);
        let mut esc = false;
        if matches!(
            cur(&self.st),
            Screen::Sign
                | Screen::Book
                | Screen::CmdBlock
                | Screen::Structure
                | Screen::Jigsaw
                | Screen::TestBlock
        ) {
            // VANILLA: Screen#extractTransparentBackground — fillGradient(0xC0101010 → 0xD0101010).
            let (gw0, gh0) = (w as f32 / k, h as f32 / k);
            t.fill_gradient(0.0, 0.0, gw0, gh0, 0xC010_1010, 0xD010_1010);
            t.flush(dev, queue, view, w, h);
            t.begin(w, h);
            p.begin(t.scale());
        }
        if matches!(cur(&self.st), Screen::Death | Screen::DeathConfirm) {
            // VANILLA: DeathScreen#extractDeathBackground — fillGradient(0x60500000 → 0xA0803030);
            // отдельным проходом, чтобы градиент лёг под кнопки (спрайты флашатся раньше текста).
            let (gw0, gh0) = (w as f32 / k, h as f32 / k);
            t.fill_gradient(0.0, 0.0, gw0, gh0, 0x6050_0000, 0xA080_3030);
            t.flush(dev, queue, view, w, h);
            t.begin(w, h);
            p.begin(t.scale());
        }
        {
            let mut f = Frame {
                t,
                p,
                lang: &self.lang,
                mx: self.mx / k,
                my: self.my / k,
                click,
                down: self.down,
                drag: &mut self.drag,
                id: 0,
                w: gw,
                h: gh,
                in_game: self.in_game,
            };
            let st = &mut self.st;
            let opts = &mut self.opts;
            let screen = cur(st);
            // VANILLA: LevelLoadingScreen#extractBackground — у порталов фон рисует App до GUI.
            let portal_bg =
                screen == Screen::Loading && self.loading_reason != LoadingReason::Other;
            // Фон: панорама + затемнение (VANILLA: Screen#extractPanorama; размытие не делаем).
            if !self.in_game && !portal_bg {
                f.p.blit(
                    "overlay",
                    0.0,
                    0.0,
                    gw,
                    gh,
                    [0.0, 0.0, 1.0, 1.0],
                    0xFFFFFFFF,
                );
            }
            if matches!(
                screen,
                Screen::Death
                    | Screen::DeathConfirm
                    | Screen::Sign
                    | Screen::Book
                    | Screen::CmdBlock
                    | Screen::Structure
                    | Screen::Jigsaw
                    | Screen::TestBlock
            ) || portal_bg
            {
                // фон нарисован до Frame (см. выше) / порталом
            } else if screen != Screen::Title {
                // VANILLA: Screen#extractMenuBackground — тайл 32×32 (menu_background / inworld_).
                let bg = if self.in_game {
                    "iw_menu_bg"
                } else {
                    "menu_bg"
                };
                f.p.tile(bg, 0.0, 0.0, gw, gh, 32.0, 0xFFFFFFFF);
            }
            let mut next: Option<Screen> = None;
            let mut back = false;
            match screen {
                Screen::Title => title(&mut f, st, now_ms, &mut next, &mut action),
                Screen::Multiplayer => multiplayer(
                    &mut f,
                    st,
                    wheel,
                    now_ms,
                    &keys,
                    &mut next,
                    &mut back,
                    &mut action,
                ),
                Screen::Direct => form(
                    &mut f,
                    st,
                    &keys,
                    &typed,
                    now_ms,
                    true,
                    &mut back,
                    &mut action,
                    &mut next,
                ),
                Screen::EditServer(_) => form(
                    &mut f,
                    st,
                    &keys,
                    &typed,
                    now_ms,
                    false,
                    &mut back,
                    &mut action,
                    &mut next,
                ),
                Screen::Singleplayer => {
                    f.center("selectWorld.title", 8.0, WHITE);
                    f.center_raw("(single-player: later)", gh / 2.0 - 4.0, GRAY);
                    if f.button(
                        gw / 2.0 - 100.0,
                        gh - 27.0,
                        200.0,
                        20.0,
                        &f.lang.tr("gui.back"),
                        true,
                    ) {
                        back = true;
                    }
                }
                Screen::Pause => pause(&mut f, &mut next, &mut action),
                Screen::Disconnected => {
                    disconnected(&mut f, &st.kick, st.kick_connect_failed, &mut back)
                }
                Screen::Connecting => {
                    // VANILLA: ConnectScreen#extractRenderState / #init.
                    let key = match ezzmc_net::session::LOGIN_STATUS
                        .load(std::sync::atomic::Ordering::Relaxed)
                    {
                        1 => "connect.authorizing",
                        2 => "connect.encrypting",
                        _ => "connect.connecting",
                    };
                    f.center(key, gh / 2.0 - 50.0, WHITE);
                    if f.button(
                        gw / 2.0 - 100.0,
                        (gh / 4.0).floor() + 120.0 + 12.0,
                        200.0,
                        20.0,
                        &f.lang.tr("gui.cancel"),
                        true,
                    ) {
                        action = Action::CancelConnect;
                    }
                }
                // VANILLA: LevelLoadingScreen — без статуса чанков (на клиенте его нет): текст y = h/2 − 50.
                Screen::Loading => {
                    f.center("multiplayer.downloadingTerrain", gh / 2.0 - 50.0, WHITE)
                }
                Screen::Death => death(&mut f, st, &mut next, &mut action),
                Screen::DeathConfirm => death_confirm(&mut f, st, &mut action),
                Screen::Sign => signedit::screen(&mut f, st, &keys, &typed, now_ms, &mut action),
                Screen::Book => bookedit::screen(&mut f, st, &keys, &typed, now_ms, &mut action),
                Screen::CmdBlock => {
                    cmdblock::screen(&mut f, st, &keys, &typed, now_ms, &mut action)
                }
                Screen::Structure => {
                    structblock::screen(&mut f, st, &keys, &typed, now_ms, &mut action)
                }
                Screen::Jigsaw => jigsaw::screen(&mut f, st, &keys, &typed, now_ms, &mut action),
                Screen::TestBlock => {
                    testblock::screen(&mut f, st, &keys, &typed, now_ms, &mut action)
                }
                Screen::Dialog => {
                    dialog::screen(&mut f, st, &keys, &typed, wheel, now_ms, &mut action);
                    // VANILLA: Screen#defaultHandleGameClickEvent → open_url → ConfirmLinkScreen(trusted=false).
                    if let Some(i) = st
                        .dlg_out
                        .iter()
                        .position(|o| matches!(o, dialog::Out::OpenUrl(_)))
                    {
                        if let dialog::Out::OpenUrl(u) = st.dlg_out.remove(i) {
                            st.link = Some(u);
                            next = Some(Screen::LinkConfirm);
                        }
                    }
                }
                Screen::Social => {
                    social::screen(&mut f, st, &keys, &typed, wheel, now_ms, &mut action)
                }
                Screen::LinkConfirm => link_confirm(&mut f, st, &mut back, &mut action),
                Screen::Options => options_hub(&mut f, st, opts, &mut next, &mut back),
                Screen::Page(Page::KeyBinds) => controls(
                    &mut f,
                    st,
                    &self.binds,
                    &mut self.listening,
                    &mut self.reset_keys,
                    &mut self.reset_one,
                    wheel,
                    &mut back,
                ),
                Screen::Page(Page::ResourcePacks) => {
                    resource_packs(&mut f, st, opts, wheel, &mut back)
                }
                Screen::Page(Page::Language) => {
                    if let Some(code) = language_screen(&mut f, st, opts, wheel, &mut back) {
                        action = Action::SetLanguage(code);
                    }
                }
                Screen::Page(pg) => page(&mut f, st, opts, pg, wheel, &mut next, &mut back),
            }
            // VANILLA: DisconnectedScreen#shouldCloseOnEsc = false.
            if keys.iter().any(|k| matches!(k, UiKey::Escape))
                && screen != Screen::Title
                && screen != Screen::Disconnected
                && screen != Screen::Connecting
                && screen != Screen::Loading
                && screen != Screen::Dialog
                && screen != Screen::Sign
                && screen != Screen::Book
                && screen != Screen::CmdBlock
                && screen != Screen::Structure
                && screen != Screen::Jigsaw
                && screen != Screen::TestBlock
                && screen != Screen::Social
                && screen != Screen::Death
                && screen != Screen::DeathConfirm
                && self.listening.is_none()
            {
                if screen == Screen::Pause {
                    action = Action::Resume;
                } else {
                    esc = true;
                }
            }
            if let Some(n) = next {
                self.push(n);
                if matches!(n, Screen::EditServer(_) | Screen::Direct) {
                    prepare_form(&mut self.st, n);
                }
            } else if back || esc {
                if screen == Screen::Page(Page::Language) {
                    self.st.pack_search = EditBox::new(64);
                }
                if screen == Screen::Page(Page::ResourcePacks) {
                    let now = self.opts.raw("resourcePacks").unwrap_or("[\"vanilla\"]");
                    if self.st.pack_initial.as_deref().is_some_and(|i| i != now) {
                        action = Action::ReloadResources;
                    }
                    self.st.pack_initial = None;
                    self.st.pack_search = EditBox::new(64);
                    self.st.pack_confirm = None;
                }
                self.pop();
            }
        }
        // Панорама (куб) + спрайты, затем текст/заливки.
        let pano = if self.in_game || self.loading_portal().is_some() {
            None
        } else {
            Some((10.0, -self.spin))
        };
        p.flush(dev, queue, view, w, h, pano);
        t.flush(dev, queue, view, w, h);
        action
    }
}

fn prepare_form(st: &mut State, s: Screen) {
    st.focus = 0;
    let (a, b) = match s {
        Screen::EditServer(Some(i)) => st
            .servers
            .get(i)
            .map_or((String::new(), String::new()), |s| {
                (s.name.clone(), s.ip.clone())
            }),
        Screen::EditServer(None) => (String::new(), String::new()),
        _ => (st.fields[1].text.clone(), String::new()),
    };
    st.fields = [edit_with(&a), edit_with(&b)];
}

struct Frame<'a> {
    t: &'a mut TextRenderer,
    p: &'a mut GuiPainter,
    lang: &'a Lang,
    mx: f32,
    my: f32,
    click: bool,
    down: bool,
    drag: &'a mut Option<u32>,
    id: u32,
    w: f32,
    h: f32,
    in_game: bool,
}

impl Frame<'_> {
    fn hover(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.mx >= x && self.my >= y && self.mx < x + w && self.my < y + h
    }

    fn center_raw(&mut self, s: &str, y: f32, rgb: u32) {
        let x = (self.w - self.t.text_width(s)) / 2.0;
        self.t.draw(s, x, y, rgb, true);
    }

    fn center(&mut self, key: &str, y: f32, rgb: u32) {
        let s = self.lang.tr(key);
        self.center_raw(&s, y, rgb);
    }

    /// VANILLA: AbstractWidget#extractScrollingStringOverContents (margin 2) +
    /// ActiveTextCollector#defaultScrollingHelper — длинная подпись обрезается и ездит туда-обратно.
    fn scrolling_label(&mut self, label: &str, x: f32, y: f32, w: f32, h: f32, col: u32) {
        let ty = ((y + y + h - 9.0) / 2.0).floor() + 1.0;
        self.scrolling_text(label, x + 2.0, x + w - 2.0, ty, col, true);
    }

    /// Строка в `[left, right]`: влезает — по центру (`center`) или слева, иначе обрезка и бегущая строка.
    fn scrolling_text(
        &mut self,
        label: &str,
        left: f32,
        right: f32,
        ty: f32,
        col: u32,
        center: bool,
    ) {
        let lw = self.t.text_width(label);
        let avail = right - left;
        if lw > avail {
            let max_pos = f64::from(lw - avail);
            let time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0.0, |d| (d.as_millis() % 86_400_000) as f64 / 1000.0);
            let period = (max_pos * 0.5).max(3.0);
            let a = (std::f64::consts::FRAC_PI_2 * (std::f64::consts::TAU * time / period).cos())
                .sin()
                / 2.0
                + 0.5;
            let pos = (a * max_pos).trunc() as f32;
            self.t.set_clip_x(Some((left, right)));
            self.t.draw(label, left - pos, ty, col, true);
            self.t.set_clip_x(None);
        } else if center {
            let cx = ((left + right) / 2.0).clamp(left + lw / 2.0, right - lw / 2.0);
            self.t.draw(label, cx - lw / 2.0, ty, col, true);
        } else {
            self.t.draw(label, left, ty, col, true);
        }
    }

    /// VANILLA: Button — спрайты button / highlighted / disabled (nine_slice 200×20, граница 3).
    fn button(&mut self, x: f32, y: f32, w: f32, h: f32, label: &str, active: bool) -> bool {
        if self.hover(x, y, w, h) {
            crate::cursor::request(if active {
                crate::cursor::Kind::PointingHand
            } else {
                crate::cursor::Kind::NotAllowed
            });
        }
        let hov = self.hover(x, y, w, h) && active;
        let sprite = if !active {
            "button_off"
        } else if hov {
            "button_hl"
        } else {
            "button"
        };
        self.p
            .nine_slice(sprite, x, y, w, h, [200.0, 20.0], [3.0; 4], 0xFFFFFFFF);
        let col = if active { WHITE } else { GRAY };
        self.scrolling_label(label, x, y, w, h, col);
        if hov && self.click {
            self.click = false;
            // VANILLA: AbstractButton#playDownSound → SimpleSoundInstance.forUI(UI_BUTTON_CLICK, 1.0F).
            crate::audio::play_ui("ui.button.click", crate::audio::Cat::Master, 0.25, 1.0);
            return true;
        }
        false
    }

    /// VANILLA: AbstractSliderButton. `v` в 0..1; возвращает true, если значение менялось.
    fn slider(&mut self, x: f32, y: f32, w: f32, h: f32, label: &str, v: &mut f32) -> bool {
        self.id += 1;
        let mut clicked = false;
        let hov = self.hover(x, y, w, h);
        if hov || *self.drag == Some(self.id) {
            crate::cursor::request(if *self.drag == Some(self.id) && self.down {
                crate::cursor::Kind::ResizeEw
            } else {
                crate::cursor::Kind::PointingHand
            });
        }
        if hov && self.click {
            self.click = false;
            crate::audio::play_ui("ui.button.click", crate::audio::Cat::Master, 0.25, 1.0);
            *self.drag = Some(self.id);
            clicked = true;
        }
        let mut changed = false;
        // VANILLA: AbstractSliderButton#onClick — значение ставится сразу по клику.
        if *self.drag == Some(self.id) && (self.down || clicked) {
            let nv = ((self.mx - (x + 4.0)) / (w - 8.0)).clamp(0.0, 1.0);
            if (nv - *v).abs() > f32::EPSILON {
                *v = nv;
                changed = true;
            }
        }
        let hl = hov || *self.drag == Some(self.id);
        self.p.nine_slice(
            if hl { "slider_hl" } else { "slider" },
            x,
            y,
            w,
            h,
            [200.0, 20.0],
            [1.0; 4],
            0xFFFFFFFF,
        );
        let hx = x + (w - 8.0) * *v;
        self.p.nine_slice(
            if hl { "handle_hl" } else { "handle" },
            hx.round(),
            y,
            8.0,
            h,
            [8.0, 20.0],
            [2.0, 2.0, 2.0, 3.0],
            0xFFFFFFFF,
        );
        self.scrolling_label(label, x, y, w, h, WHITE);
        changed
    }

    /// Фон списка между шапкой и подвалом (VANILLA: menu_list_background + separators).
    fn list_bg(&mut self, top: f32, bottom: f32) {
        let (w, h) = (self.w, bottom - top);
        // VANILLA: AbstractSelectionList#extractListBackground/Separators (texture 32×32 → ячейка 32).
        let (lb, hs, fs) = if self.in_game {
            ("iw_list_bg", "iw_header_sep", "iw_footer_sep")
        } else {
            ("list_bg", "header_sep", "footer_sep")
        };
        self.p.tile(lb, 0.0, top, w, h, 32.0, 0xFFFFFFFF);
        // Разделители — текстуры 32×2: по вертикали UV 0..1 (обе строки), по горизонтали тайл 32.
        for (tex, y) in [(hs, top - 2.0), (fs, bottom)] {
            let mut cx = 0.0;
            while cx < w {
                let cw = 32.0_f32.min(w - cx);
                self.p
                    .blit(tex, cx, y, cw, 2.0, [0.0, 0.0, cw / 32.0, 1.0], 0xFFFFFFFF);
                cx += 32.0;
            }
        }
    }

    /// VANILLA: EditBox#renderWidget/mouseClicked/mouseDragged: прокрутка по курсору,
    /// подсветка выделения, клик ставит курсор, зажатая ЛКМ растягивает выделение.
    #[allow(clippy::too_many_arguments)]
    fn edit_box(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        e: &mut EditBox,
        focused: bool,
        caret: bool,
    ) -> bool {
        let border = if focused { 0xFFFFFFFF } else { 0xFFA0A0A0 };
        self.t
            .fill(x - 1.0, y - 1.0, x + w + 1.0, y + h + 1.0, border);
        self.t.fill(x, y, x + w, y + h, 0xFF000000);
        let cs: Vec<char> = e.text.chars().collect();
        let sub = |a: usize, b: usize| cs[a..b].iter().collect::<String>();
        e.scroll = e.scroll.min(cs.len()).min(e.cursor);
        while e.scroll < e.cursor && self.t.text_width(&sub(e.scroll, e.cursor)) > w - 8.0 {
            e.scroll += 1;
        }
        let mut end = e.scroll;
        while end < cs.len() && self.t.text_width(&sub(e.scroll, end + 1)) <= w - 8.0 {
            end += 1;
        }
        let tx = x + 4.0;
        let ty = y + (h - 8.0) / 2.0;
        let (a, b) = e.selection();
        let (a, b) = (a.clamp(e.scroll, end), b.clamp(e.scroll, end));
        if a < b {
            let x1 = tx + self.t.text_width(&sub(e.scroll, a));
            let x2 = tx + self.t.text_width(&sub(e.scroll, b));
            self.t.fill(x1, ty - 1.0, x2, ty + 10.0, 0xFF3A6FD8);
        }
        self.t.draw(&sub(e.scroll, end), tx, ty, 0xE0E0E0, true);
        // VANILLA: EditBox#extractWidgetRenderState + TextCursorUtils — мигание 300 мс; в конце текста
        // каретка «_», иначе полоска 1×(9+1) цветом текста (вставка).
        let _ = caret;
        let blink = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_millis() / 300);
        if focused && blink % 2 == 0 && e.cursor >= e.scroll && e.cursor <= end {
            let cx = tx + self.t.text_width(&sub(e.scroll, e.cursor));
            if e.cursor == cs.len() && cs.len() < e.max {
                self.t.draw("_", cx, ty, 0xE0E0E0, true);
            } else {
                self.t.fill(cx, ty - 1.0, cx + 1.0, ty + 10.0, 0xFFE0E0E0);
            }
        }
        let over = self.hover(x, y, w, h);
        if over {
            crate::cursor::request(crate::cursor::Kind::Ibeam);
        }
        // Индекс символа под курсором мыши (по середине символа).
        let at = |s: &Self| {
            let mut i = e.scroll;
            while i < end
                && tx + s.t.text_width(&sub(e.scroll, i + 1)) - 0.5 * s.t.text_width(&sub(i, i + 1))
                    < s.mx
            {
                i += 1;
            }
            i
        };
        let hit = over && self.click;
        if hit {
            self.click = false;
            e.cursor = at(self);
            e.sel = e.cursor;
            e.dragging = true;
        } else if e.dragging {
            if self.down {
                e.cursor = if self.mx < tx {
                    e.scroll.saturating_sub(1)
                } else {
                    at(self)
                };
                if self.mx >= x + w - 4.0 {
                    e.cursor = (end + 1).min(cs.len());
                }
            } else {
                e.dragging = false;
            }
        }
        hit
    }
}

/// VANILLA: TitleScreen — логотип, сплэш, кнопки, копирайт.
fn title(
    f: &mut Frame,
    st: &mut State,
    now_ms: u64,
    next: &mut Option<Screen>,
    action: &mut Action,
) {
    let (w, h) = (f.w, f.h);
    // Логотип 256×44 (текстура 256×64), edition 128×14 (текстура 128×16).
    f.p.blit(
        "logo",
        (w / 2.0).floor() - 128.0,
        30.0,
        256.0,
        44.0,
        [0.0, 0.0, 1.0, 44.0 / 64.0],
        0xFFFFFFFF,
    );
    f.p.blit(
        "edition",
        (w / 2.0).floor() - 64.0,
        30.0 + 44.0 - 7.0,
        128.0,
        14.0,
        [0.0, 0.0, 1.0, 14.0 / 16.0],
        0xFFFFFFFF,
    );
    // VANILLA: SplashRenderer — центр (w/2+123, 69), поворот −20°, пульсация масштаба.
    if !st.splash.is_empty() && !HIDE_SPLASH.load(std::sync::atomic::Ordering::Relaxed) {
        let tw = f.t.text_width(&st.splash);
        let ph = 1.8
            - ((now_ms % 1000) as f32 / 1000.0 * std::f32::consts::TAU)
                .sin()
                .abs()
                * 0.1;
        let sc = ph * 100.0 / (tw + 32.0);
        f.t.set_transform(Some((w / 2.0 + 123.0, 69.0, -20.0, sc)));
        f.t.draw(&st.splash, -((tw / 2.0).floor()), -8.0, 0xFFFF00, true);
        f.t.set_transform(None);
    }
    let top = (h / 4.0).floor() + 48.0;
    let cx = (w / 2.0).floor();
    if f.button(
        cx - 100.0,
        top,
        200.0,
        20.0,
        &f.lang.tr("menu.singleplayer"),
        true,
    ) {
        *next = Some(Screen::Singleplayer);
    }
    if f.button(
        cx - 100.0,
        top + 24.0,
        200.0,
        20.0,
        &f.lang.tr("menu.multiplayer"),
        true,
    ) {
        *next = Some(Screen::Multiplayer);
    }
    let row = top + 72.0;
    if f.button(
        cx - 100.0,
        row,
        98.0,
        20.0,
        &f.lang.tr("menu.options"),
        true,
    ) {
        *next = Some(Screen::Options);
    }
    if f.button(cx + 2.0, row, 98.0, 20.0, &f.lang.tr("menu.quit"), true) {
        *action = Action::Quit;
    }
    // Иконки «язык» / «спец. возможности» слева и справа от ряда.
    if f.button(cx - 124.0, row, 20.0, 20.0, "", true) {
        *next = Some(Screen::Page(Page::Language));
    }
    f.p.blit(
        "icon_language",
        cx - 124.0 + 2.5,
        row + 2.5,
        15.0,
        15.0,
        [0.0, 0.0, 1.0, 1.0],
        0xFFFFFFFF,
    );
    if f.button(cx + 104.0, row, 20.0, 20.0, "", true) {
        *next = Some(Screen::Page(Page::Accessibility));
    }
    f.p.blit(
        "icon_access",
        cx + 104.0 + 2.5,
        row + 2.5,
        15.0,
        15.0,
        [0.0, 0.0, 1.0, 1.0],
        0xFFFFFFFF,
    );
    let c = f.lang.tr("title.credits");
    let cw = f.t.text_width(&c);
    f.t.draw(&c, w - cw - 2.0, h - 10.0, WHITE, true);
    f.t.draw("Minecraft 26.3 (ezzmc)", 2.0, h - 10.0, WHITE, true);
}

/// VANILLA: JoinMultiplayerScreen + ServerSelectionList (строка 36 px, двойной клик — вход).
#[allow(clippy::too_many_arguments)]
fn multiplayer(
    f: &mut Frame,
    st: &mut State,
    wheel: f32,
    now_ms: u64,
    keys: &[UiKey],
    next: &mut Option<Screen>,
    back: &mut bool,
    action: &mut Action,
) {
    let (w, h) = (f.w, f.h);
    let (top, bottom) = (32.0, h - 64.0);
    f.center("multiplayer.title", 20.0 - 4.0, WHITE);
    f.list_bg(top, bottom);
    let rows = st.servers.len() as f32;
    let max_scroll = (rows * 36.0 - (bottom - top) + 4.0).max(0.0);
    st.scroll = (st.scroll - wheel * 18.0).clamp(0.0, max_scroll);
    if st.confirm_delete {
        // Подтверждение удаления (VANILLA: ConfirmScreen).
        f.t.fill(0.0, 0.0, w, h, 0xA0000000);
        f.center("selectServer.deleteQuestion", h / 2.0 - 30.0, WHITE);
        if let Some(s) = st.sel.and_then(|i| st.servers.get(i)) {
            let l = format!("'{}' will be lost forever! (A long time!)", s.name);
            f.center_raw(&l, h / 2.0 - 16.0, GRAY);
        }
        if f.button(
            w / 2.0 - 105.0,
            h / 2.0 + 4.0,
            100.0,
            20.0,
            &f.lang.tr("selectServer.deleteButton"),
            true,
        ) {
            if let Some(i) = st.sel.take().filter(|i| *i < st.servers.len()) {
                st.servers.remove(i);
                servers::save(&st.servers_path, &st.servers);
            }
            st.confirm_delete = false;
        }
        if f.button(
            w / 2.0 + 5.0,
            h / 2.0 + 4.0,
            100.0,
            20.0,
            &f.lang.tr("gui.cancel"),
            true,
        ) {
            st.confirm_delete = false;
        }
        return;
    }
    let mut join = None;
    let mut swap: Option<(usize, usize)> = None;
    let mut tip: Option<Vec<(String, u32)>> = None;
    let n = st.servers.len();
    let (cx, cw) = (w / 2.0 - 150.0, 300.0);
    for i in 0..n {
        let y = top + 4.0 - st.scroll + i as f32 * 36.0;
        if y + 36.0 < top || y > bottom {
            continue;
        }
        let (name, ip) = (st.servers[i].name.clone(), st.servers[i].ip.clone());
        // VANILLA: пинг стартует при первой отрисовке строки (State.INITIAL → PINGING).
        st.pinger.ensure(&ip, f.lang);
        let in_view = f.my >= top && f.my < bottom;
        if st.sel == Some(i) {
            f.t.fill(cx - 2.0, y - 2.0, cx + cw + 2.0, y + 34.0, 0xFF808080);
            f.t.fill(cx - 1.0, y - 1.0, cx + cw + 1.0, y + 33.0, 0xFF000000);
        }
        let hovered = f.hover(cx, y, cw, 32.0) && in_view;
        let (rx, ry) = (f.mx - cx, f.my - y);
        let over_icon = hovered && rx < 32.0;
        let (right, top_left, bot_left) = (
            over_icon && rx > 16.0,
            over_icon && rx < 16.0 && ry < 16.0,
            over_icon && rx < 16.0 && ry > 16.0,
        );
        if hovered && f.click {
            f.click = false;
            if right {
                join = Some(i);
            } else if top_left && i > 0 {
                swap = Some((i, i - 1));
            } else if bot_left && i + 1 < n {
                swap = Some((i, i + 1));
            } else if st.last_click.0 == Some(i) && now_ms.saturating_sub(st.last_click.1) < 250 {
                join = Some(i);
            }
            st.sel = Some(i);
            st.last_click = (Some(i), now_ms);
        }
        let info = st.pinger.map.get(&ip);
        f.t.draw(&name, cx + 35.0, y + 1.0, WHITE, true);
        if let Some(info) = info {
            for (l, line) in wrap(f.t, &info.motd, cw - 34.0).iter().take(2).enumerate() {
                f.t.draw(line, cx + 35.0, y + 12.0 + 9.0 * l as f32, 0x808080, true);
            }
        }
        let icon = ping::icon_name(&ip);
        let icon = if f.p.has(&icon) {
            icon
        } else {
            "unknown_server".to_owned()
        };
        f.p.blit(&icon, cx, y, 32.0, 32.0, [0.0, 0.0, 1.0, 1.0], 0xFFFFFFFF);
        // Иконка связи и текст «online/max» (VANILLA: statusIcon / statusIconTooltip).
        let sx = cx + cw - 15.0;
        if let Some(info) = info {
            use ping::PingState::*;
            let sprite = match info.state {
                Pinging => {
                    let mut k = (now_ms / 100 + i as u64 * 2) & 7;
                    if k > 4 {
                        k = 8 - k;
                    }
                    format!("srv_pinging_{}", k + 1)
                }
                Incompatible => "srv_incompatible".to_owned(),
                Unreachable => "srv_unreachable".to_owned(),
                Successful => format!(
                    "srv_ping_{}",
                    match info.ping {
                        0..=149 => 5,
                        150..=299 => 4,
                        300..=599 => 3,
                        600..=999 => 2,
                        _ => 1,
                    }
                ),
            };
            f.p.blit(&sprite, sx, y, 10.0, 8.0, [0.0, 0.0, 1.0, 1.0], 0xFFFFFFFF);
            let status = if info.state == Incompatible {
                format!("§c{}", info.version)
            } else {
                info.status.clone()
            };
            let sw = f.t.text_width(&status);
            let stx = sx - sw - 5.0;
            f.t.draw(&status, stx, y + 1.0, 0x808080, true);
            if in_view && f.mx >= sx && f.mx <= sx + 10.0 && f.my >= y && f.my <= y + 8.0 {
                let key = match info.state {
                    Pinging => Some("multiplayer.status.pinging"),
                    Incompatible => Some("multiplayer.status.incompatible"),
                    Unreachable => Some("multiplayer.status.no_connection"),
                    Successful => None,
                };
                let text = match key {
                    Some(k) => f.lang.tr(k),
                    None => f
                        .lang
                        .tr("multiplayer.status.ping")
                        .replace("%s", &info.ping.to_string()),
                };
                tip = Some(vec![(text, WHITE)]);
            } else if in_view
                && !info.players.is_empty()
                && f.mx >= stx
                && f.mx <= stx + sw
                && f.my >= y
                && f.my <= y + 8.0
            {
                tip = Some(info.players.iter().map(|p| (p.clone(), WHITE)).collect());
            }
        }
        if hovered {
            f.t.fill(cx, y, cx + 32.0, y + 32.0, 0xA0909090);
            let hl = |on: bool, a: &'static str, b: &'static str| if on { b } else { a };
            f.p.blit(
                hl(right, "srv_join", "srv_join_hl"),
                cx,
                y,
                32.0,
                32.0,
                [0.0, 0.0, 1.0, 1.0],
                0xFFFFFFFF,
            );
            if i > 0 {
                f.p.blit(
                    hl(top_left, "srv_up", "srv_up_hl"),
                    cx,
                    y,
                    32.0,
                    32.0,
                    [0.0, 0.0, 1.0, 1.0],
                    0xFFFFFFFF,
                );
            }
            if i + 1 < n {
                f.p.blit(
                    hl(bot_left, "srv_down", "srv_down_hl"),
                    cx,
                    y,
                    32.0,
                    32.0,
                    [0.0, 0.0, 1.0, 1.0],
                    0xFFFFFFFF,
                );
            }
        }
    }
    if let Some((a, b)) = swap {
        // VANILLA: JoinMultiplayerScreen#onSelectedChange / swap серверов в списке + сохранение.
        st.servers.swap(a, b);
        st.sel = Some(b);
        servers::save(&st.servers_path, &st.servers);
    }
    if let Some(lines) = tip {
        crate::tooltip::draw_colored(f.t, &lines, f.mx, f.my, w, h);
    }
    if st.servers.is_empty() {
        f.center_raw("-", top + 12.0, GRAY);
    }
    // Нижние кнопки (VANILLA: два ряда по h-52 и h-28).
    let sel = st.sel.filter(|i| *i < st.servers.len());
    let (r1, r2) = (h - 52.0, h - 28.0);
    if f.button(
        w / 2.0 - 154.0,
        r1,
        100.0,
        20.0,
        &f.lang.tr("selectServer.select"),
        sel.is_some(),
    ) {
        join = sel;
    }
    if f.button(
        w / 2.0 - 50.0,
        r1,
        100.0,
        20.0,
        &f.lang.tr("selectServer.direct"),
        true,
    ) {
        *next = Some(Screen::Direct);
    }
    if f.button(
        w / 2.0 + 4.0 + 50.0,
        r1,
        100.0,
        20.0,
        &f.lang.tr("selectServer.add"),
        true,
    ) {
        *next = Some(Screen::EditServer(None));
    }
    if f.button(
        w / 2.0 - 154.0,
        r2,
        70.0,
        20.0,
        &f.lang.tr("selectServer.edit"),
        sel.is_some(),
    ) {
        *next = sel.map(|i| Screen::EditServer(Some(i)));
    }
    if f.button(
        w / 2.0 - 74.0,
        r2,
        70.0,
        20.0,
        &f.lang.tr("selectServer.delete"),
        sel.is_some(),
    ) {
        st.confirm_delete = true;
    }
    if f.button(
        w / 2.0 + 4.0,
        r2,
        70.0,
        20.0,
        &f.lang.tr("selectServer.refresh"),
        true,
    ) {
        st.pinger.refresh();
    }
    if f.button(
        w / 2.0 + 4.0 + 76.0,
        r2,
        75.0,
        20.0,
        &f.lang.tr("gui.back"),
        true,
    ) {
        *back = true;
    }
    if keys.iter().any(|k| matches!(k, UiKey::Enter)) {
        join = join.or(sel);
    }
    if let Some(i) = join {
        *action = Action::Join(st.servers[i].ip.clone());
    }
}

/// Перенос строки по ширине с переносом активных `§`-кодов на следующую строку.
fn wrap(t: &TextRenderer, s: &str, max: f32) -> Vec<String> {
    wrap_n(t, s, max, 2)
}

/// Как `wrap`, но не больше `limit` строк.
pub(crate) fn wrap_n(t: &TextRenderer, s: &str, max: f32, limit: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut active = String::new();
    for para in s.split('\n') {
        let mut line = active.clone();
        let mut last_space: Option<usize> = None;
        let chars: Vec<char> = para.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            if c == '§' && i + 1 < chars.len() {
                let code = chars[i + 1].to_ascii_lowercase();
                line.push('§');
                line.push(code);
                if code == 'r' {
                    active.clear();
                } else {
                    active.push('§');
                    active.push(code);
                }
                i += 2;
                continue;
            }
            i += 1;
            line.push(c);
            if c == ' ' {
                last_space = Some(line.len());
            }
            if t.text_width(&line) > max {
                let (first, rest) = match last_space {
                    Some(p) if p < line.len() => {
                        (line[..p].trim_end().to_owned(), line[p..].to_owned())
                    }
                    Some(_) => (line.trim_end().to_owned(), String::new()),
                    None => {
                        let cut = line.len() - c.len_utf8();
                        (line[..cut].to_owned(), format!("{active}{c}"))
                    }
                };
                out.push(first);
                line = format!("{active}{rest}");
                last_space = None;
                if out.len() >= limit {
                    return out;
                }
            }
        }
        out.push(line);
        if out.len() >= limit {
            return out;
        }
    }
    out
}

/// Общая форма: «Прямое подключение» (`direct`) и «Правка сервера».
#[allow(clippy::too_many_arguments)]
fn form(
    f: &mut Frame,
    st: &mut State,
    keys: &[UiKey],
    typed: &str,
    now_ms: u64,
    direct: bool,
    back: &mut bool,
    action: &mut Action,
    _next: &mut Option<Screen>,
) {
    let (w, h) = (f.w, f.h);
    let n = if direct { 1 } else { 2 };
    for k in keys {
        match k {
            UiKey::Tab => st.focus = (st.focus + 1) % n,
            UiKey::BackTab => st.focus = (st.focus + n - 1) % n,
            _ => {}
        }
    }
    let focus = st.focus;
    edit_apply(&mut st.fields[focus], keys, typed, &mut st.clip_out);
    let caret = (now_ms / 500) % 2 == 0;
    let cx = w / 2.0 - 100.0;
    let (title, y0) = if direct {
        ("selectServer.direct", 116.0)
    } else {
        ("addServer.title", 66.0)
    };
    f.center(title, 17.0, WHITE);
    let ip_idx = n - 1;
    if !direct {
        f.t.draw(&f.lang.tr("addServer.enterName"), cx, 53.0, GRAY, true);
        if f.edit_box(cx, y0, 200.0, 20.0, &mut st.fields[0], st.focus == 0, caret) {
            st.focus = 0;
        }
    }
    let ip_y = if direct { y0 } else { 106.0 };
    f.t.draw(&f.lang.tr("addServer.enterIp"), cx, ip_y - 13.0, GRAY, true);
    if f.edit_box(
        cx,
        ip_y,
        200.0,
        20.0,
        &mut st.fields[usize::from(!direct)],
        st.focus == ip_idx,
        caret,
    ) {
        st.focus = ip_idx;
    }
    let ip = st.fields[usize::from(!direct)].text.trim().to_owned();
    let ok = !ip.is_empty();
    let done_key = if direct {
        "selectServer.select"
    } else {
        "addServer.add"
    };
    let by = (h / 4.0).floor() + 96.0 + 18.0;
    let enter = keys.iter().any(|k| matches!(k, UiKey::Enter));
    if f.button(cx, by, 200.0, 20.0, &f.lang.tr(done_key), ok) || (enter && ok) {
        if direct {
            *action = Action::Join(ip);
        } else {
            let name = if st.fields[0].text.trim().is_empty() {
                f.lang.tr("selectServer.defaultName")
            } else {
                st.fields[0].text.trim().to_owned()
            };
            let s = Server { name, ip };
            match cur(st) {
                Screen::EditServer(Some(i)) if i < st.servers.len() => st.servers[i] = s,
                _ => st.servers.push(s),
            }
            servers::save(&st.servers_path, &st.servers);
            *back = true;
        }
    }
    if f.button(cx, by + 24.0, 200.0, 20.0, &f.lang.tr("gui.cancel"), true) {
        *back = true;
    }
}

/// VANILLA: OptionsScreen — FOV, «Online», сетка 2×N кнопок, «Done».
/// VANILLA: PauseScreen (Back to Game / Options / Disconnect).
/// VANILLA: DisconnectedScreen#init — колонка по центру: заголовок, причина (макс. ширина w−50), кнопка 200.
fn disconnected(f: &mut Frame, reason: &str, connect_failed: bool, back: &mut bool) {
    let lines = wrap(f.t, reason, f.w - 50.0);
    let n = lines.len() as f32;
    let total = 29.0 + 9.0 * n + 20.0 + 24.0;
    let top = ((f.h - total) / 2.0).floor();
    // VANILLA: ConnectScreen → CONNECT_FAILED до входа, иначе «Connection Lost»
    // (VANILLA-UNSURE: кик в Configuration у ванили тоже «Connection Lost», у нас — connect.failed).
    f.center(
        if connect_failed {
            "connect.failed"
        } else {
            "disconnect.lost"
        },
        top + 10.0,
        WHITE,
    );
    for (i, l) in lines.iter().enumerate() {
        f.center_raw(l, top + 39.0 + 9.0 * i as f32, WHITE);
    }
    let y = top + 29.0 + 9.0 * n + 20.0 + 2.0;
    if f.button(
        f.w / 2.0 - 100.0,
        y,
        200.0,
        20.0,
        &f.lang.tr("gui.toMenu"),
        true,
    ) {
        *back = true;
    }
}

/// VANILLA: DeathScreen — заголовок ×2, причина, счёт; «Respawn»/«Spectate World» и «Title Screen»
/// (активны через 20 тиков).
fn death(f: &mut Frame, st: &mut State, next: &mut Option<Screen>, action: &mut Action) {
    let Some(d) = &mut st.death else { return };
    let active = d.opened.elapsed() >= std::time::Duration::from_millis(1000);
    let (w, h) = (f.w, f.h);
    let title = f.lang.tr(if d.hardcore {
        "deathScreen.title.hardcore"
    } else {
        "deathScreen.title"
    });
    // Масштаб 2 вокруг (w/2, 60): текст рисуется в «своих» координатах (middleLine/2, 30).
    f.t.set_transform(Some((w / 2.0, 60.0, 0.0, 2.0)));
    let tw = f.t.text_width(&title);
    f.t.draw(&title, -tw / 2.0, 0.0, WHITE, true);
    f.t.set_transform(None);
    if let Some(c) = &d.cause {
        f.center_raw(c, 85.0, WHITE);
    }
    let score = f
        .lang
        .fmt("deathScreen.score.value", &[&format!("§e{}§r", d.score)]);
    f.center_raw(&score, 100.0, WHITE);
    let x = w / 2.0 - 100.0;
    let y = (h / 4.0).floor() + 72.0;
    let label = f.lang.tr(if d.hardcore {
        "deathScreen.spectate"
    } else {
        "deathScreen.respawn"
    });
    if f.button(x, y, 200.0, 20.0, &label, active && !d.sent) {
        d.sent = true;
        *action = Action::Respawn;
    }
    if f.button(
        x,
        y + 24.0,
        200.0,
        20.0,
        &f.lang.tr("deathScreen.titleScreen"),
        active,
    ) {
        if d.hardcore {
            *action = Action::Disconnect;
        } else {
            d.confirm = std::time::Instant::now();
            *next = Some(Screen::DeathConfirm);
        }
    }
}

/// VANILLA: DeathScreen.TitleConfirmScreen (ConfirmScreen: заголовок, пустое сообщение, 2 кнопки 150×20, задержка 20 тиков).
fn death_confirm(f: &mut Frame, st: &mut State, action: &mut Action) {
    let Some(d) = &mut st.death else { return };
    let active = d.confirm.elapsed() >= std::time::Duration::from_millis(1000);
    // Вертикальный LinearLayout (spacing 8): заголовок 9, сообщение 0, ряд кнопок (padding 16 + 20).
    let top = ((f.h - 61.0) / 2.0).floor();
    f.center("deathScreen.quit.confirm", top, WHITE);
    let (y, x) = (top + 41.0, ((f.w - 304.0) / 2.0).floor());
    if f.button(
        x,
        y,
        150.0,
        20.0,
        &f.lang.tr("deathScreen.titleScreen"),
        active,
    ) {
        *action = Action::Disconnect;
    }
    if f.button(
        x + 154.0,
        y,
        150.0,
        20.0,
        &f.lang.tr("deathScreen.respawn"),
        active,
    ) {
        // VANILLA: callback(false) — player.respawn(), setScreen(null).
        d.sent = true;
        *action = Action::Respawn;
    }
}

/// VANILLA: ConfirmLinkScreen(trusted=false): заголовок, URL, предупреждение, [Да][Копировать][Нет].
fn link_confirm(f: &mut Frame, st: &mut State, back: &mut bool, action: &mut Action) {
    let Some(url) = st.link.clone() else {
        *back = true;
        return;
    };
    // LinearLayout vertical spacing 8: 9 + 9 + 9 + ряд кнопок (padding 16 + 20).
    let top = ((f.h - 87.0) / 2.0).floor();
    f.center("chat.link.confirm", top, WHITE);
    f.center_raw(&url, top + 17.0, WHITE);
    f.center("chat.link.warning", top + 34.0, 0xFFFFCC);
    let (y, x) = (top + 67.0, ((f.w - 308.0) / 2.0).floor());
    if f.button(x, y, 100.0, 20.0, &f.lang.tr("gui.yes"), true) {
        *action = Action::OpenLink(url.clone());
        *back = true;
    }
    if f.button(x + 104.0, y, 100.0, 20.0, &f.lang.tr("chat.copy"), true) {
        st.clip_out = Some(url);
        *back = true;
    }
    if f.button(x + 208.0, y, 100.0, 20.0, &f.lang.tr("gui.no"), true) {
        *back = true;
    }
}

fn pause(f: &mut Frame, next: &mut Option<Screen>, action: &mut Action) {
    let (w, h) = (f.w, f.h);
    f.center("menu.game", 40.0, WHITE);
    let x = w / 2.0 - 102.0;
    let y0 = (h / 4.0).floor() + 24.0;
    if f.button(x, y0, 204.0, 20.0, &f.lang.tr("menu.returnToGame"), true) {
        *action = Action::Resume;
    }
    // VANILLA: PauseScreen — ряд Advancements / Statistics (по 98).
    if f.button(
        x,
        y0 + 24.0,
        98.0,
        20.0,
        &f.lang.tr("gui.advancements"),
        true,
    ) {
        *action = Action::OpenAdv;
    }
    if f.button(
        x + 106.0,
        y0 + 24.0,
        98.0,
        20.0,
        &f.lang.tr("gui.stats"),
        true,
    ) {
        *action = Action::OpenStats;
    }
    // VANILLA: PauseScreen#getCustomAdditions — «Ссылки сервера», если сервер прислал ссылки.
    let links = crate::dialog::SERVER_LINKS
        .lock()
        .is_ok_and(|g| !g.is_empty());
    let mut dy = 0.0;
    if links {
        if f.button(
            x,
            y0 + 48.0,
            204.0,
            20.0,
            &f.lang.tr("menu.server_links"),
            true,
        ) {
            *action = Action::OpenServerLinks;
        }
        dy = 24.0;
    }
    if f.button(
        x,
        y0 + 48.0 + dy,
        204.0,
        20.0,
        &f.lang.tr("menu.options"),
        true,
    ) {
        *next = Some(Screen::Options);
    }
    if f.button(
        x,
        y0 + 96.0 + dy,
        204.0,
        20.0,
        &f.lang.tr("menu.disconnect"),
        true,
    ) {
        *action = Action::Disconnect;
    }
}

fn options_hub(
    f: &mut Frame,
    _st: &mut State,
    opts: &mut Options,
    next: &mut Option<Screen>,
    back: &mut bool,
) {
    let (w, h) = (f.w, f.h);
    f.center("options.title", 12.0, WHITE);
    option_row(f, opts, &options::FOV, w / 2.0 - 155.0, 29.0, 150.0);
    f.button(
        w / 2.0 + 5.0,
        29.0,
        150.0,
        20.0,
        &f.lang.tr("options.online"),
        false,
    );
    let items: [(&str, Option<Page>); 10] = [
        ("options.skinCustomisation", Some(Page::Skin)),
        ("options.sounds", Some(Page::Sound)),
        ("options.video", Some(Page::Video)),
        ("options.controls", Some(Page::Controls)),
        ("options.language", Some(Page::Language)),
        ("options.chat", Some(Page::Chat)),
        ("options.resourcepack", Some(Page::ResourcePacks)),
        ("options.accessibility", Some(Page::Accessibility)),
        ("options.telemetry", Some(Page::Telemetry)),
        ("options.credits_and_attribution", Some(Page::Credits)),
    ];
    for (i, (key, pg)) in items.iter().enumerate() {
        let (x, y) = (
            w / 2.0 + if i % 2 == 0 { -154.0 } else { 4.0 },
            65.0 + (i / 2) as f32 * 24.0,
        );
        // VANILLA: «Telemetry» неактивна (нет аккаунта) — как при отключённой телеметрии.
        let active = *pg != Some(Page::Telemetry);
        if f.button(x, y, 150.0, 20.0, &f.lang.tr(key), active) {
            *next = pg.map(Screen::Page);
        }
    }
    if f.button(
        w / 2.0 - 100.0,
        h - 27.0,
        200.0,
        20.0,
        &f.lang.tr("gui.done"),
        true,
    ) {
        *back = true;
    }
}

/// VANILLA: OptionsSubScreen — шапка 33 px, прокручиваемый список в 2 колонки, «Done».
/// VANILLA: KeyBindsScreen/KeyBindsList — категории, кнопка клавиши (75), Reset (50).
#[allow(clippy::too_many_arguments)]
fn controls(
    f: &mut Frame,
    st: &mut State,
    binds: &[BindRow],
    listening: &mut Option<&'static str>,
    reset_all: &mut bool,
    reset_one: &mut Option<&'static str>,
    wheel: f32,
    back: &mut bool,
) {
    let (w, h) = (f.w, f.h);
    let (top, bottom) = (33.0, h - 33.0);
    f.list_bg(top, bottom);
    // Высота: заголовок категории 20, строка 24.
    let mut total = 4.0;
    let mut last = "";
    for b in binds {
        if b.category != last {
            total += 20.0;
            last = b.category;
        }
        total += 24.0;
    }
    let max_scroll = (total - (bottom - top)).max(0.0);
    st.scroll = (st.scroll - wheel * 12.0).clamp(0.0, max_scroll);
    let mut y = top + 4.0 - st.scroll;
    let mut last = "";
    for b in binds {
        if b.category != last {
            last = b.category;
            if y >= top - 10.0 && y + 10.0 <= bottom {
                let s = f.lang.tr(b.category);
                f.center_raw(&s, y + 9.0, WHITE);
            }
            y += 20.0;
        }
        if y >= top && y + 20.0 <= bottom {
            let name_label = f.lang.tr(b.name);
            let ty = y + 6.0;
            if f.button(
                w / 2.0 + 105.0,
                y,
                50.0,
                20.0,
                &f.lang.tr("controls.reset"),
                !b.default,
            ) {
                *reset_one = Some(b.name);
            }
            // VANILLA: InputConstants.Type.KEYBOARD#displayTextSupplier — один символ → заглавная буква.
            let key = match b.key.strip_prefix("key.keyboard.") {
                Some(c) if c.chars().count() == 1 => c.to_uppercase(),
                _ => f.lang.tr(&b.key),
            };
            let label = if *listening == Some(b.name) {
                format!("§e> §f{key}§e <")
            } else if b.conflict {
                format!("§c[ §f{key}§c ]")
            } else {
                key
            };
            if f.button(w / 2.0 + 25.0, y, 75.0, 20.0, &label, true) {
                *listening = Some(b.name);
            }
            // Отход от ваниля (по просьбе владельца): длинное имя обрезается до кнопки и ездит.
            f.scrolling_text(
                &name_label,
                w / 2.0 - 155.0,
                w / 2.0 + 17.0,
                ty,
                WHITE,
                false,
            );
        }
        y += 24.0;
    }
    f.center("controls.keybinds.title", 15.0, WHITE);
    if f.button(
        w / 2.0 - 155.0,
        h - 27.0,
        150.0,
        20.0,
        &f.lang.tr("controls.resetAll"),
        true,
    ) {
        *reset_all = true;
    }
    if f.button(
        w / 2.0 + 5.0,
        h - 27.0,
        150.0,
        20.0,
        &f.lang.tr("gui.done"),
        true,
    ) {
        *listening = None;
        *back = true;
    }
}

fn page(
    f: &mut Frame,
    st: &mut State,
    opts: &mut Options,
    pg: Page,
    wheel: f32,
    next: &mut Option<Screen>,
    back: &mut bool,
) {
    let (w, h) = (f.w, f.h);
    let (top, bottom) = (33.0, h - 33.0);
    f.list_bg(top, bottom);
    let defs = pg.defs();
    // VANILLA: ControlsScreen — первая строка: Mouse Settings / Key Binds, дальше опции.
    let head = usize::from(pg == Page::Controls) * 2;
    // VANILLA: OptionsList#addBig/addSmall — «большая» опция занимает всю строку (310), малые идут парами.
    let wide =
        |d: &Def| pg == Page::Sound && matches!(d.key, "soundCategory_master" | "soundDevice");
    let (mut row, mut col) = (head / 2, head % 2);
    let mut pos = Vec::with_capacity(defs.len());
    for d in defs {
        if wide(d) {
            if col == 1 {
                row += 1;
                col = 0;
            }
            pos.push((row, 0, true));
            row += 1;
        } else {
            pos.push((row, col, false));
            col ^= 1;
            if col == 0 {
                row += 1;
            }
        }
    }
    let rows = (row + usize::from(col == 1)) as f32;
    let max_scroll = (rows * 24.0 + 4.0 - (bottom - top)).max(0.0);
    st.scroll = (st.scroll - wheel * 12.0).clamp(0.0, max_scroll);
    for (n, (key, target)) in [
        ("options.mouse_settings", Page::Mouse),
        ("controls.keybinds", Page::KeyBinds),
    ]
    .into_iter()
    .take(head)
    .enumerate()
    {
        let x = w / 2.0 + if n == 0 { -155.0 } else { 5.0 };
        let y = top + 4.0 - st.scroll;
        if y >= top && y + 20.0 <= bottom && f.button(x, y, 150.0, 20.0, &f.lang.tr(key), true) {
            *next = Some(Screen::Page(target));
        }
    }
    for (d, &(r, c, big)) in defs.iter().zip(&pos) {
        let y = top + 4.0 - st.scroll + r as f32 * 24.0;
        if y < top || y + 20.0 > bottom {
            continue;
        }
        if big {
            option_row(f, opts, d, w / 2.0 - 155.0, y, 310.0);
        } else {
            let x = w / 2.0 + if c == 0 { -155.0 } else { 5.0 };
            option_row(f, opts, d, x, y, 150.0);
        }
    }
    if defs.is_empty() && head == 0 {
        f.center_raw("-", h / 2.0 - 4.0, GRAY);
    }
    f.center(pg.title(), 15.0, WHITE);
    if f.button(
        w / 2.0 - 100.0,
        h - 27.0,
        200.0,
        20.0,
        &f.lang.tr("gui.done"),
        true,
    ) {
        *back = true;
    }
}

/// VANILLA: LanguageSelectScreen — поиск, список (строки по 18, ширина 270), «Font…»/«Done».
/// VANILLA-UNSURE: вертикальные отступы шапки (заголовок y=4, поиск y=17) выведены из раскладки.
fn language_screen(
    f: &mut Frame,
    st: &mut State,
    opts: &mut Options,
    wheel: f32,
    back: &mut bool,
) -> Option<String> {
    let (w, h) = (f.w, f.h);
    let (top, bottom) = (36.0, h - 53.0);
    let (pk, pt) = (
        std::mem::take(&mut st.pack_keys),
        std::mem::take(&mut st.pack_typed),
    );
    edit_apply(&mut st.pack_search, &pk, &pt, &mut st.clip_out);
    if st.lang_sel.is_empty() {
        st.lang_sel = opts.raw("lang").unwrap_or("en_us").to_owned();
    }
    let q = st.pack_search.text.to_lowercase();
    let shown: Vec<(String, String)> = st
        .langs
        .iter()
        .filter(|(_, n)| q.is_empty() || n.to_lowercase().contains(&q))
        .cloned()
        .collect();
    let max_scroll = (shown.len() as f32 * 18.0 + 4.0 - (bottom - top)).max(0.0);
    st.scroll = (st.scroll - wheel * 9.0).clamp(0.0, max_scroll);
    f.list_bg(top, bottom);
    let (rw, rx) = (270.0, w / 2.0 - 135.0);
    let mut applied = None;
    for (i, (code, name)) in shown.iter().enumerate() {
        let y = top + 2.0 - st.scroll + i as f32 * 18.0;
        if y + 18.0 < top || y > bottom {
            continue;
        }
        let (vt, vb) = (y.max(top), (y + 18.0).min(bottom));
        let sel = *code == st.lang_sel;
        if sel {
            f.t.fill(rx, vt, rx + rw, vb, 0xFFFFFFFF);
            f.t.fill(rx + 1.0, vt, rx + rw - 1.0, vb, 0xFF000000);
        }
        if f.click && f.hover(rx, vt, rw, vb - vt) && f.my >= top && f.my < bottom {
            st.lang_sel = code.clone();
            f.click = false;
            // Двойной клик (VANILLA: mouseClicked(doubleClick) → onDone).
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_millis() as u64);
            if st.last_click.0 == Some(i) && now.saturating_sub(st.last_click.1) < 250 {
                applied = Some(code.clone());
                *back = true;
            }
            st.last_click = (Some(i), now);
        }
        let ty = y + 5.0;
        if ty >= top && ty + 8.0 <= bottom {
            f.center_raw(name, ty, WHITE);
        }
    }
    // Ползунок (AbstractSelectionList#scrollBarX = rowRight + 6 + 2), перетаскивание — пока кнопка зажата на нём.
    if max_scroll > 0.0 {
        let (sbx, lh) = (rx + rw + 8.0, bottom - top);
        let sh = (lh * lh / (shown.len() as f32 * 18.0 + 4.0)).clamp(32.0, (lh - 8.0).max(32.0));
        if f.down && f.mx >= sbx && f.mx < sbx + 6.0 && f.my >= top && f.my < bottom {
            let n = ((f.my - top - sh / 2.0) / (lh - sh)).clamp(0.0, 1.0);
            st.scroll = n * max_scroll;
        }
        let sy = top + st.scroll * (lh - sh) / max_scroll;
        f.p.nine_slice(
            "st_scroller_bg",
            sbx,
            top,
            6.0,
            lh,
            [6.0, 32.0],
            [1.0; 4],
            0xFFFFFFFF,
        );
        f.p.nine_slice(
            "st_scroller",
            sbx,
            sy,
            6.0,
            sh,
            [6.0, 32.0],
            [1.0; 4],
            0xFFFFFFFF,
        );
    }
    // Шапка поверх прокручиваемого списка.
    f.center(&f.lang.tr("options.language.title"), 4.0, WHITE);
    let caret = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis())
        / 500)
        % 2
        == 0;
    f.edit_box(
        w / 2.0 - 100.0,
        17.0,
        200.0,
        14.0,
        &mut st.pack_search,
        true,
        caret,
    );
    // VANILLA: футер — предупреждение (withColor -4539718 = 0xBABABA) и кнопки.
    f.center_raw(
        &f.lang.tr("options.languageAccuracyWarning"),
        h - 45.0,
        0xBABABA,
    );
    // Font Settings (FontOptionsScreen) не реализован — кнопка неактивна.
    let _ = f.button(
        w / 2.0 - 154.0,
        h - 28.0,
        150.0,
        20.0,
        &f.lang.tr("options.font"),
        false,
    );
    if f.button(
        w / 2.0 + 4.0,
        h - 28.0,
        150.0,
        20.0,
        &f.lang.tr("gui.done"),
        true,
    ) {
        applied = Some(st.lang_sel.clone());
        *back = true;
    }
    if let Some(c) = &applied {
        opts.set("lang", c.clone());
    }
    applied
}

/// Обрезка строки по ширине с «...».
fn clip(f: &Frame, s: &str, max: f32) -> String {
    if f.t.text_width(s) <= max {
        return s.to_owned();
    }
    let mut o: String = s.to_owned();
    while !o.is_empty() && f.t.text_width(&format!("{o}...")) > max {
        o.pop();
    }
    format!("{o}...")
}

/// VANILLA: PackSelectionScreen — «Available» слева, «Selected» справа (сверху — высший приоритет).
/// Клик по доступному — включить; по выбранному — выключить; ^/v — порядок. Применяется после перезапуска.
fn resource_packs(f: &mut Frame, st: &mut State, opts: &mut Options, wheel: f32, back: &mut bool) {
    use crate::packrepo::{self as pr, PackInfo};
    let (w, h) = (f.w, f.h);
    let (top, bottom) = (64.0, h - 48.0);
    let avail_all = st.packs_avail.clone().unwrap_or_default();
    // Ввод в поле поиска (VANILLA: PackSelectionScreen — EditBox фильтрует оба списка).
    let (pk, pt) = (
        std::mem::take(&mut st.pack_keys),
        std::mem::take(&mut st.pack_typed),
    );
    edit_apply(&mut st.pack_search, &pk, &pt, &mut st.clip_out);
    let q = st.pack_search.text.to_lowercase();
    let matches = |title: &str, desc: &str| {
        q.is_empty() || title.to_lowercase().contains(&q) || desc.to_lowercase().contains(&q)
    };
    let mut sel = pr::parse_selected(opts.raw("resourcePacks").unwrap_or("[\"vanilla\"]"));
    sel.retain(|id| id == pr::VANILLA || avail_all.iter().any(|p| &p.id == id));
    let avail: Vec<&PackInfo> = avail_all
        .iter()
        .filter(|p| !sel.contains(&p.id) && matches(&p.title, &p.desc))
        .collect();
    let vanilla_name = f.lang.tr("resourcePack.vanilla.name");
    let vanilla_desc = f.lang.tr("resourcePack.vanilla.description");
    let shown_sel: Vec<usize> = (0..sel.len())
        .rev()
        .filter(|&i| {
            let id = &sel[i];
            if id == pr::VANILLA {
                matches(&vanilla_name, &vanilla_desc)
            } else {
                avail_all
                    .iter()
                    .find(|p| &p.id == id)
                    .is_some_and(|p| matches(&p.title, &p.desc))
            }
        })
        .collect();
    let modal = st.pack_confirm.is_some();
    let mclick = f.click && modal;
    if modal {
        f.click = false;
    }
    let rows = avail.len().max(shown_sel.len()) as f32;
    let max_scroll = (rows * 36.0 + 4.0 - (bottom - top)).max(0.0);
    st.scroll = (st.scroll - wheel * 12.0).clamp(0.0, max_scroll);
    f.list_bg(top, bottom);
    let (lx, rx, cw) = (w / 2.0 - 205.0, w / 2.0 + 5.0, 200.0);
    let draw_row =
        |f: &mut Frame, x: f32, y: f32, icon: &str, title: &str, desc: &str, red: bool| {
            f.t.fill(x, y, x + cw, y + 32.0, 0x80000000);
            let icon = if f.p.has(icon) { icon } else { "pack_unknown" };
            f.p.blit(icon, x, y, 32.0, 32.0, [0.0, 0.0, 1.0, 1.0], 0xFFFFFFFF);
            f.t.draw(
                &clip(f, title, cw - 36.0 - 28.0),
                x + 36.0,
                y + 3.0,
                WHITE,
                true,
            );
            let col = if red { 0xFF5555 } else { GRAY };
            f.t.draw(&clip(f, desc, cw - 40.0), x + 36.0, y + 15.0, col, true);
        };
    let compat_text = |f: &Frame, p: &PackInfo| match p.compat {
        Some(ezzmc_assets::pack::Compat::TooOld) => (f.lang.tr("pack.incompatible.old"), true),
        Some(ezzmc_assets::pack::Compat::TooNew) => (f.lang.tr("pack.incompatible.new"), true),
        None => (f.lang.tr("pack.incompatible.unknown"), true),
        _ => (p.desc.clone(), false),
    };
    let compatible = |p: &PackInfo| {
        !matches!(
            p.compat,
            Some(ezzmc_assets::pack::Compat::TooOld)
                | Some(ezzmc_assets::pack::Compat::TooNew)
                | None
        )
    };
    let mut new_sel: Option<Vec<String>> = None;
    for (i, p) in avail.iter().enumerate() {
        let y = top + 4.0 - st.scroll + i as f32 * 36.0;
        if y < top || y + 32.0 > bottom {
            continue;
        }
        let (d, red) = compat_text(f, p);
        let src = f.lang.tr("pack.source.local");
        draw_row(
            f,
            lx,
            y,
            &format!("packicon:{}", p.id),
            &f.lang.fmt("pack.nameAndSource", &[&p.title, &src]),
            &d,
            red,
        );
        if !modal && f.hover(lx, y, cw, 32.0) {
            crate::cursor::request(crate::cursor::Kind::PointingHand);
            if f.click {
                f.click = false;
                if compatible(p) {
                    let mut s = sel.clone();
                    s.push(p.id.clone());
                    new_sel = Some(s);
                } else {
                    // VANILLA: PackSelectionScreen — подтверждение несовместимого пака.
                    st.pack_confirm = Some(p.id.clone());
                }
            }
        }
    }
    // Выбранные: сверху — последний в списке (высший приоритет).
    for (n, &idx) in shown_sel.iter().enumerate() {
        let y = top + 4.0 - st.scroll + n as f32 * 36.0;
        if y < top || y + 32.0 > bottom {
            continue;
        }
        let id = sel[idx].clone();
        if id == pr::VANILLA {
            let src = f.lang.tr("pack.source.builtin");
            draw_row(
                f,
                rx,
                y,
                "pack_vanilla",
                &f.lang.fmt("pack.nameAndSource", &[&vanilla_name, &src]),
                &vanilla_desc,
                false,
            );
            continue;
        }
        let Some(p) = avail_all.iter().find(|p| p.id == id) else {
            continue;
        };
        let (d, red) = compat_text(f, p);
        let src = f.lang.tr("pack.source.local");
        draw_row(
            f,
            rx,
            y,
            &format!("packicon:{}", p.id),
            &f.lang.fmt("pack.nameAndSource", &[&p.title, &src]),
            &d,
            red,
        );
        // Кнопки порядка: ^ — выше (к концу списка), v — ниже (но не под vanilla).
        let (bx, by) = (rx + cw - 22.0, y + 1.0);
        let up_ok = idx + 1 < sel.len();
        let down_ok = idx > 1;
        if f.button(bx, by, 10.0, 14.0, "^", up_ok) && up_ok {
            let mut s = sel.clone();
            s.swap(idx, idx + 1);
            new_sel = Some(s);
        }
        if f.button(bx + 11.0, by, 10.0, 14.0, "v", down_ok) && down_ok {
            let mut s = sel.clone();
            s.swap(idx, idx - 1);
            new_sel = Some(s);
        }
        if f.hover(rx, y + 16.0, cw, 16.0) && f.click {
            f.click = false;
            let mut s = sel.clone();
            s.remove(idx);
            new_sel = Some(s);
        }
    }
    // Шапка: заголовок, подсказка про drag-and-drop, поиск, заголовки колонок.
    f.center("resourcePack.title", 8.0, WHITE);
    f.center_raw(&f.lang.tr("pack.dropInfo"), 20.0, GRAY);
    let caret = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis())
        / 500)
        % 2
        == 0;
    f.edit_box(
        w / 2.0 - 100.0,
        32.0,
        200.0,
        14.0,
        &mut st.pack_search,
        true,
        caret,
    );
    for (x, key) in [(lx, "pack.available.title"), (rx, "pack.selected.title")] {
        let t = f.lang.tr(key);
        let tx = x + (cw - f.t.text_width(&t)) / 2.0;
        f.t.draw(&t, tx, 51.0, WHITE, true);
    }
    // VANILLA: PackSelectionScreen — футер: «Open Pack Folder» (Blaze3D.openPath) + «Done», по 150 px, зазор 8.
    f.center_raw(&f.lang.tr("pack.folderInfo"), h - 40.0, GRAY);
    if f.button(
        w / 2.0 - 154.0,
        h - 27.0,
        150.0,
        20.0,
        &f.lang.tr("pack.openFolder"),
        !modal,
    ) && !modal
    {
        let dir = pr::dir();
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::process::Command::new("xdg-open").arg(&dir).spawn();
    }
    if f.button(
        w / 2.0 + 4.0,
        h - 27.0,
        150.0,
        20.0,
        &f.lang.tr("gui.done"),
        !modal,
    ) && !modal
    {
        *back = true;
    }
    if let Some(id) = st.pack_confirm.clone() {
        // Модальный диалог (VANILLA: ConfirmScreen).
        f.click = mclick;
        f.t.fill(0.0, 0.0, w, h, 0xC0101010);
        let msg_key = match avail_all.iter().find(|p| p.id == id).and_then(|p| p.compat) {
            Some(ezzmc_assets::pack::Compat::TooOld) => "pack.incompatible.confirm.old",
            Some(ezzmc_assets::pack::Compat::TooNew) => "pack.incompatible.confirm.new",
            _ => "pack.incompatible.confirm.unknown",
        };
        f.center("pack.incompatible.confirm.title", h / 2.0 - 40.0, WHITE);
        // Перенос по словам (VANILLA: ConfirmScreen — MultiLineLabel).
        let (msg, mut line, mut ly) = (f.lang.tr(msg_key), String::new(), h / 2.0 - 26.0);
        for word in msg.split(' ') {
            let try_line = if line.is_empty() {
                word.to_owned()
            } else {
                format!("{line} {word}")
            };
            if f.t.text_width(&try_line) > w - 40.0 && !line.is_empty() {
                f.center_raw(&line, ly, WHITE);
                ly += 10.0;
                line = word.to_owned();
            } else {
                line = try_line;
            }
        }
        f.center_raw(&line, ly, WHITE);
        if f.button(
            w / 2.0 - 155.0,
            h / 2.0 + 4.0,
            150.0,
            20.0,
            &f.lang.tr("gui.yes"),
            true,
        ) {
            let mut s = sel.clone();
            s.push(id);
            new_sel = Some(s);
            st.pack_confirm = None;
        } else if f.button(
            w / 2.0 + 5.0,
            h / 2.0 + 4.0,
            150.0,
            20.0,
            &f.lang.tr("gui.no"),
            true,
        ) {
            st.pack_confirm = None;
        }
    }
    if let Some(s) = new_sel {
        opts.set("resourcePacks", pr::encode_selected(&s));
    }
}

/// Значение опции для подписи (VANILLA: OptionInstance#valueSetter/captions).
fn value_text(lang: &Lang, d: &Def, raw: &str) -> String {
    let v: f32 = raw.parse().unwrap_or(0.0);
    match d.kind {
        Kind::Toggle => lang.tr(if raw == "true" {
            "options.on"
        } else {
            "options.off"
        }),
        Kind::Cycle(c) => c
            .iter()
            .find(|(val, _)| *val == raw)
            .map_or_else(|| raw.to_owned(), |(_, k)| lang.tr(k)),
        Kind::Log(..) => format!("{v:.2}"),
        Kind::Slider(min, max, step) => match d.key {
            // VANILLA: Options#sensitivity — 0 «*yawn*», 1 «HYPERSPEED!!!», иначе 2·value в процентах.
            "mouseSensitivity" if v <= 0.0 => lang.tr("options.sensitivity.min"),
            "mouseSensitivity" if v >= 1.0 => lang.tr("options.sensitivity.max"),
            "mouseSensitivity" => format!("{}%", (2.0 * v * 100.0).round()),
            "sprintWindow" if v <= 0.0 => lang.tr("options.off"),
            "fov" if v == 70.0 => lang.tr("options.fov.min"),
            "fov" if v >= max => lang.tr("options.fov.max"),
            "maxFps" if v >= max => lang.tr("options.framerateLimit.max"),
            "maxFps" => lang.fmt("options.framerate", &[&format!("{v}")]),
            "guiScale" if v < 1.0 => lang.tr("options.guiScale.auto"),
            "renderDistance" | "simulationDistance" | "cloudRange" => {
                lang.fmt("options.chunks", &[&format!("{v}")])
            }
            "weatherRadius" => lang.fmt("options.blocks", &[&format!("{v}")]),
            // VANILLA: Options#biomeBlendRadius — подпись по ширине (2v+1).
            "biomeBlendRadius" => {
                lang.tr(&format!("options.biomeBlendRadius.{}", v as i32 * 2 + 1))
            }
            "menuBackgroundBlurriness" if v <= 0.0 => lang.tr("options.off"),
            // VANILLA: Options#percentValueOrOffLabel.
            "screenEffectScale"
            | "fovEffectScale"
            | "darknessEffectScale"
            | "damageTiltStrength"
            | "glintSpeed"
            | "glintStrength"
                if v <= 0.0 =>
            {
                lang.tr("options.off")
            }
            "chatDelay" if v <= 0.0 => lang.tr("options.chat.delay_none"),
            "chatDelay" => lang.fmt("options.chat.delay", &[&format!("{v:.1}")]),
            "notificationDisplayTime" => lang.fmt("options.multiplier", &[&format!("{v}")]),
            "mipmapLevels" if v <= 0.0 => lang.tr("options.off"),
            "maxAnisotropyBit" => {
                lang.fmt("options.multiplier", &[&format!("{}", 1 << (v as u32))])
            }
            "gamma" if v <= min => lang.tr("options.gamma.min"),
            "gamma" if (v - 0.5).abs() < 0.005 => lang.tr("options.gamma.default"),
            "gamma" if v >= max => lang.tr("options.gamma.max"),
            _ if step >= 1.0 => format!("{}", v.round()),
            _ if d.key.starts_with("soundCategory") && v <= 0.0 => lang.tr("options.off"),
            _ => format!("{}%", (v * 100.0).round()),
        },
    }
}

fn option_row(f: &mut Frame, opts: &mut Options, d: &Def, x: f32, y: f32, w: f32) {
    let raw = opts.get(d).to_owned();
    let caption = f.lang.tr(d.lang);
    let label = if d.key == "chunkSectionFadeInTime" {
        // VANILLA: Options#chunkSectionFadeInTime — подпись целиком, без «caption: value».
        let v: f32 = raw.parse().unwrap_or(0.0);
        if v <= 0.0 {
            f.lang.tr("options.chunkFade.none")
        } else {
            f.lang
                .fmt("options.chunkFade.seconds", &[&format!("{v:.2}")])
        }
    } else if matches!(
        d.key,
        "musicFrequency" | "musicToast" | "backgroundForChatOnly" | "chatDelay"
    ) {
        // VANILLA: Options#musicFrequency/musicToast — подпись = только значение.
        value_text(f.lang, d, &raw)
    } else {
        f.lang.fmt(
            "options.generic_value",
            &[&caption, &value_text(f.lang, d, &raw)],
        )
    };
    match d.kind {
        Kind::Toggle => {
            if f.button(x, y, w, 20.0, &label, true) {
                opts.set(d.key, (raw != "true").to_string());
            }
        }
        Kind::Cycle(c) => {
            if f.button(x, y, w, 20.0, &label, true) {
                let i = c.iter().position(|(v, _)| *v == raw).map_or(0, |i| i + 1) % c.len();
                opts.set(d.key, c[i].0.to_owned());
            }
        }
        Kind::Log(min, max) => {
            let cur = (f64::from(opts.f32(d)).max(1e-3).log10() * 100.0).round() as f32;
            let mut n = ((cur - min as f32) / (max - min) as f32).clamp(0.0, 1.0);
            if f.slider(x, y, w, 20.0, &label, &mut n) {
                let s = (min as f32 + n * (max - min) as f32).round();
                opts.set(d.key, format!("{}", 10f64.powf(f64::from(s) / 100.0)));
            }
        }
        Kind::Slider(min, max, step) => {
            // VANILLA: ClampingLazyMaxIntRange — максимум GUI Scale = calculateScale(0)+1 по текущему окну.
            let max = if d.key == "guiScale" {
                let k = f.t.scale() as f32;
                ezzmc_render::text::gui_scale((f.w * k) as u32, (f.h * k) as u32) as f32 + 1.0
            } else {
                max
            };
            // VANILLA: UnitDouble.xmap(Mth::square, Math::sqrt) — положение ползунка = √значения.
            let squared = matches!(d.key, "fovEffectScale" | "darknessEffectScale");
            let cur = if squared {
                opts.f32(d).max(0.0).sqrt()
            } else {
                opts.f32(d)
            };
            let mut n = ((cur - min) / (max - min)).clamp(0.0, 1.0);
            if f.slider(x, y, w, 20.0, &label, &mut n) {
                let mut v = min + n * (max - min);
                if squared {
                    v *= v;
                }
                if step > 0.0 {
                    v = (min + ((v - min) / step).round() * step).clamp(min, max);
                }
                opts.set(d.key, options::format_num(v, step));
            }
        }
    }
}

/// Options#hideSplashTexts (обновляется каждый кадр из options).
pub static HIDE_SPLASH: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(test)]
mod loading_reason_tests {
    use super::LoadingReason;

    #[test]
    fn determine_matches_vanilla() {
        let d = LoadingReason::determine;
        assert_eq!(
            d(false, "minecraft:the_nether", "minecraft:overworld"),
            LoadingReason::NetherPortal
        );
        assert_eq!(
            d(false, "minecraft:overworld", "minecraft:the_nether"),
            LoadingReason::NetherPortal
        );
        assert_eq!(
            d(false, "minecraft:the_end", "minecraft:overworld"),
            LoadingReason::EndPortal
        );
        assert_eq!(
            d(false, "minecraft:overworld", "minecraft:the_end"),
            LoadingReason::EndPortal
        );
        // Nether проверяется раньше End.
        assert_eq!(
            d(false, "minecraft:the_end", "minecraft:the_nether"),
            LoadingReason::NetherPortal
        );
        assert_eq!(
            d(true, "minecraft:the_nether", "minecraft:the_end"),
            LoadingReason::Other
        );
        assert_eq!(
            d(false, "minecraft:overworld", "minecraft:overworld"),
            LoadingReason::Other
        );
    }
}
