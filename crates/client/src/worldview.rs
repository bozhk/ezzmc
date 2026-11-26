//! Связка клиент → арена террейна → render pass (M1.4): сетевые события → очередь перестроения →
//! rayon-меш → `TerrainArena` → отрисовка слоёв террейна. Камера свободная (M1.6, без физики).
use crate::audio;
use crate::netthread::{NetCmd, NetEvt, NetHandle};
use ezzmc_assets::block_atlas::BlockAtlas;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_assets::texture::Image;
use ezzmc_mesh::frustum::{look_matrix, Frustum};
use ezzmc_mesh::layers::RenderLayer;
use ezzmc_mesh::models::BlockModels;
use ezzmc_mesh::pool::MeshPool;
use ezzmc_mesh::queue::RebuildQueue;
use ezzmc_mesh::section::SectionMesh;
use ezzmc_mesh::section::{ModelProvider, Neighborhood, SectionPos};
use ezzmc_render::hiz::HiZ;
use ezzmc_render::lightmap::{LightmapParams, LightmapTexture};
use ezzmc_render::pipeline_mode::RenderPipelineMode;
use ezzmc_render::renderer::{CullCamera, Renderer};
use ezzmc_render::skydraw::{CelestialImages, SkyDraw};
use ezzmc_render::terrain::{TerrainArena, TerrainPipelines};
use ezzmc_render::uniforms::{ChunkSectionUniform, FogUniform, GlobalsUniform, ProjectionUniform};
use ezzmc_render::voxel_adapter::{split_mesh, Converted, RectTable};
use ezzmc_render::Gpu;
use ezzmc_world::block::BlockState;
use ezzmc_world::level::{Level, LevelEvent};
use glam::{Mat4, Vec3};
use rustc_hash::FxHashSet as HashSet;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::Arc;

/// Настройки рендера. `use_compute_renderer`: цельные кубы рисует compute-`Renderer`
/// (frustum + Hi-Z + indirect) вместо ванильной арены; по умолчанию выключено.
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderOptions {
    pub use_compute_renderer: bool,
    pub cutout_leaves: bool,
}

impl RenderOptions {
    /// `--compute-renderer` или `EZZMC_COMPUTE_RENDERER=1`.
    pub fn from_env_args() -> Self {
        let flag = std::env::args().any(|a| a == "--compute-renderer")
            || std::env::var("EZZMC_COMPUTE_RENDERER").is_ok_and(|v| v == "1" || v == "true");
        Self {
            use_compute_renderer: flag,
            cutout_leaves: true,
        }
    }
}

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
// VANILLA: Camera/GameRenderer — fov 70 по умолчанию.
const FOV_DEG: f32 = 70.0;
const NEAR: f32 = 0.05;
// rd=32 (~35k секций): ~78M вершин в пике (60M при 27k секций) — 80M переполнялось.
const VERTEX_CAPACITY: u32 = 120_000_000;
// Только прозрачный слой (вода/стекло): над океаном rd=32 10M индексов переполнялись.
const INDEX_ARENA_CAPACITY: u32 = 30_000_000;
const MAX_SECTIONS: u32 = 65_536;
const MAX_PENDING: usize = 128;
/// Предел пересортировок прозрачных слоёв за кадр.
const RESORT_PER_FRAME: usize = 16;
/// Повторных попыток загрузки секции в арену при нехватке места, прежде чем сдаться.
const ARENA_RETRIES: u8 = 3;

/// VANILLA: ItemStack#hasFoil для стека (блеск на 3D-предмете).
fn foil_of(st: &ezzmc_protocol::item::ItemStack) -> bool {
    crate::handitem::stack_ctx(st).is_some_and(|(n, _)| crate::hud::has_foil(n, &st.patch))
}

/// Слотов скинов (64×64) в атласе других игроков; слот 0 — Стив.
const SKIN_SLOTS: u32 = 32;

/// Неймтег игрока (собирается в `bake_remote_players`).
struct NameTag {
    anchor: [f64; 3],
    text: ezzmc_protocol::text::Component,
    discrete: bool,
    score: Option<ezzmc_protocol::text::Component>,
}

pub struct WorldView {
    /// 185: зажата ЛКМ (key.attack) — Minecraft#continueAttack.
    pub attack_held: bool,
    /// VANILLA: LivingEntity#attackStrengthTicker и последний предмет в руке (сброс при смене).
    pub psound: crate::playersound::PlayerSound,
    ambient: crate::ambient::Ambient,
    pub attack_ticker: i32,
    /// VANILLA: Minecraft#rightClickDelay (4 тика после использования) и #missTime (10 после промаха).
    pub right_click_delay: i32,
    pub miss_time: i32,
    /// ClientboundSetHealthPacket.food (VANILLA: FoodData#foodLevel).
    pub food: i32,
    pub saturation: f32,
    pub xp_progress: f32,
    pub xp_level: i32,
    /// Точки локатор-бара (VANILLA: ClientWaypointManager).
    pub waypoints: std::collections::HashMap<String, crate::hudbars::Wp>,
    /// VANILLA: Gui#experienceDisplayStartTick — когда последний раз менялся XP.
    pub xp_changed_at: Option<std::time::Instant>,
    xp_seen: bool,
    pub armor: f32,
    pub max_health: f32,
    pub hardcore: bool,
    /// VANILLA: LocalPlayer#showDeathScreen.
    pub show_death_screen: bool,
    /// VANILLA: Player#getScore.
    pub score: i32,
    /// Пришёл combat_kill: открыть DeathScreen (причина смерти).
    pub death_in: Option<Option<ezzmc_protocol::text::Component>>,
    /// Пришёл Respawn: экран смерти закрыть.
    pub respawn_in: bool,
    /// Пришёл Respawn: открыть LevelLoadingScreen с этой причиной (забирает App).
    pub loading_in: Option<crate::ui::LoadingReason>,
    /// После Respawn ещё не пришёл ни один чанк (VANILLA: LevelLoadTracker.WaitingForServer —
    /// старые чанки не считаются готовым уровнем).
    pub respawn_wait: bool,
    /// VANILLA: game_event WIN_GAME (4; в 26.3 param не проверяется) — показать End Poem / титры.
    pub win_game: bool,
    /// Экран смерти уже запрошен (до Respawn): повторные combat_kill/set_health(0) не открывают его снова.
    pub death_pending: bool,
    /// LocalPlayer#flashOnSetHealth: первый set_health только запоминает здоровье.
    health_seen: bool,
    /// VANILLA: Abilities#invulnerable (флаг 1).
    pub invulnerable: bool,
    /// VANILLA: LivingEntity#useItem/useItemRemaining — идёт еда/питьё.
    pub using: Option<Using>,
    attack_item: Option<u32>,
    digger: crate::dig::Digger,
    pub(crate) net: NetHandle,
    /// Клавиши последнего кадра ходьбы (для player_input).
    last_input: crate::physics::Input,
    /// GameType локального игрока (0 survival, 1 creative…).
    pub game_mode: i8,
    /// VANILLA: MultiPlayerGameMode#previousPlayerMode (-1 — нет).
    pub prev_game_mode: i8,
    /// Атрибуты локального игрока с сервера (UpdateAttributes); None — пакета ещё не было.
    block_range: Option<f64>,
    attack_speed: Option<f32>,
    entity_range: Option<f64>,
    mining_efficiency: Option<f32>,
    block_break_speed: f32,
    submerged_mining_speed: Option<f32>,
    /// Уровень прав оператора (entity event 24..28 → 0..4); 0 до первого события.
    pub op_level: u8,
    /// 230: инвентарь игрока и открытый контейнер.
    pub inv: crate::inventory::Inventory,
    models: Arc<BlockModels>,
    cutout_leaves: bool,
    pool: MeshPool,
    queue: RebuildQueue,
    arena: TerrainArena,
    pipes: TerrainPipelines,
    lightmap: LightmapTexture,
    sky: SkyDraw,
    clouds: ezzmc_render::clouddraw::CloudDraw,
    weather: ezzmc_render::weatherdraw::WeatherDraw,
    lightning: ezzmc_render::lightningdraw::LightningDraw,
    end_portal_draw: ezzmc_render::endportaldraw::EndPortalDraw,
    beacon_draw: ezzmc_render::beacondraw::BeaconDraw,
    beacons: crate::beacon::Beacons,
    /// Хранилища (vault) рядом: клиентский тик блок-сущности.
    vault_scan: crate::berscan::Scanner,
    beacon_batches: ezzmc_render::beacondraw::BeamBatches,
    end_portals: crate::endportal::EndPortals,
    end_portal_verts: Vec<ezzmc_render::endportaldraw::PortalVertex>,
    border_draw: ezzmc_render::borderdraw::BorderDraw,
    /// Модели сущностей (пока только аватар игрока от 3-го лица): скин Стива, слой одежды.
    entity_draw: ezzmc_render::particledraw::ParticleDraw,
    /// Другие игроки: атлас скинов 64×(64·SKIN_SLOTS), слот 0 — Стив.
    player_draw: ezzmc_render::particledraw::ParticleDraw,
    skin_slots: HashMap<String, u32>,
    player_model_slim: (
        ezzmc_render::entitymodel::Model,
        ezzmc_render::humanoid::HumanoidParts,
    ),
    skin_uploads: Vec<(u32, std::sync::Arc<Vec<u8>>)>,
    /// Броня игроков: отдельная текстура-атлас (`armor::ArmorKit`).
    armor_draw: ezzmc_render::particledraw::ParticleDraw,
    armor_kit: crate::armor::ArmorKit,
    /// Блеск зачарований брони (геометрия брони + текстура блеска).
    glint_draw: ezzmc_render::particledraw::ParticleDraw,
    /// Атласы для программного превью игрока (блоки — со старта, предметы — лениво).
    preview_blocks: std::sync::Arc<(Vec<u8>, u32, u32)>,
    preview_items: Option<std::sync::Arc<(Vec<u8>, u32, u32)>>,
    mobs: crate::mobs::Mobs,
    /// Id песни из level_event 1010 (Hud#setNowPlaying) — забирает main.
    pub now_playing: Option<i32>,
    chests: crate::chest::Chests,
    bells: crate::bell::Bells,
    tables: crate::enchtable::Tables,
    skulls: crate::skull::Skulls,
    shulker_boxes: crate::shulkerbox::ShulkerBoxes,
    banners: crate::banner::Banners,
    signs: crate::sign::Signs,
    /// Стрелы: текстуры arrow/arrow_spectral/arrow_tipped стопкой 32×96.
    arrow_draw: ezzmc_render::particledraw::ParticleDraw,
    /// VANILLA: PistonHeadRenderer — движущиеся блоки (атлас блоков).
    piston_draw: ezzmc_render::particledraw::ParticleDraw,
    /// Тени сущностей (shadow.png) и Level#getSkyDarken.
    shadow_draw: ezzmc_render::particledraw::ParticleDraw,
    sky_darken: i32,
    /// Головы, надетые на игроков в этом кадре: (предмет, корень модели, свет) — рисуются после `mobs.bake`.
    head_skulls: Vec<(&'static str, Mat4, (u8, u8))>,
    player_model: (
        ezzmc_render::entitymodel::Model,
        ezzmc_render::humanoid::HumanoidParts,
    ),
    particles: crate::particles::ParticleEngine,
    particle_draw: ezzmc_render::particledraw::ParticleDraw,
    last_particle_tick: Option<i64>,
    last_piston_tick: Option<i64>,
    /// Анимации чужих игроков по id сущности + game_time последнего тика анимаций.
    remote_anims: std::collections::HashMap<i32, crate::avatar::AvatarAnim>,
    remote_tick: Option<i64>,
    /// Entity#wasTouchingWater чужих сущностей (для брызг при входе в воду).
    wet_entities: std::collections::HashMap<i32, bool>,
    sky_state: Option<ezzmc_render::sky::SkyState>,
    /// Skybox.END: рисуется куб end_sky вместо неба Overworld.
    end_sky: bool,
    fog_state: ezzmc_render::fog::FogState,
    vignette: ezzmc_render::vignette::VignetteDraw,
    /// Hud#vignetteBrightness и накопитель тиков.
    vignette_b: f32,
    /// Сила красной виньетки у границы мира (0..1).
    vignette_warn: f32,
    vignette_acc: f32,
    /// EnvironmentAttributeProbe: (прошлый, новый) тик цветов [небо, туман, туман воды] и прогресс тика.
    probe_pair: Option<([u32; 3], [u32; 3])>,
    probe_acc: f32,
    probe_colors: [u32; 3],
    last_frame: Option<std::time::Instant>,
    #[allow(clippy::type_complexity)]
    lightmap_key: Option<(f32, [f32; 3], f32, f32, [f32; 3], [f32; 4])>,
    /// Активные эффекты локального игрока (UpdateMobEffect/RemoveMobEffect).
    pub effects: crate::effects::Effects,
    /// VANILLA: Player#getAbsorptionAmount.
    pub absorption: f32,
    group1: wgpu::BindGroup,
    projection_buf: wgpu::Buffer,
    /// Рука от 1-го лица: свой проход, своя проекция (70°, без глубины мира).
    hand_proj_buf: wgpu::Buffer,
    hand_draw: ezzmc_render::particledraw::ParticleDraw,
    hand_model: (
        ezzmc_render::entitymodel::Model,
        ezzmc_render::humanoid::HumanoidParts,
    ),
    hand_visible: bool,
    /// Предмет в руке: модели, отдельные отрисовщики (1-е лицо / F5), показанный стек (equip-анимация).
    hand_items: Option<crate::handitem::HandItems>,
    hand_rm: Option<ezzmc_assets::pack::ResourceManager>,
    hand_item_draw: Option<ezzmc_render::particledraw::ParticleDraw>,
    entity_item_draw: Option<ezzmc_render::particledraw::ParticleDraw>,
    /// Блеск предметов в руке (камера руки) и в мире.
    hand_glint_draw: Option<ezzmc_render::particledraw::ParticleDraw>,
    entity_glint_draw: Option<ezzmc_render::particledraw::ParticleDraw>,
    /// GPU-инстансинг дропов: модели на GPU (по указателю HandMesh), экземпляры текущего кадра.
    item_inst: Option<ezzmc_render::particledraw::ItemInstDraw>,
    item_inst_map: std::collections::HashMap<usize, usize>,
    item_inst_keep: Vec<std::sync::Arc<crate::handitem::HandMesh>>,
    item_inst_frame: Vec<ItemInstFrame>,
    /// VANILLA: ModelBakery#FIRE_0 / FIRE_1 (пламя горящих сущностей).
    fire_uv: Option<(ezzmc_assets::bake::SpriteUv, ezzmc_assets::bake::SpriteUv)>,
    /// VANILLA: ItemInHandRenderer#mainHandItem (предмет, который сейчас рисуется) и слот.
    hand_shown: Option<(ezzmc_protocol::item::ItemStack, u8)>,
    /// Видимый предмет второй руки и её высота (VANILLA: FirstPersonHandsAndItems#offHandItem/offHandHeight).
    off_shown: Option<ezzmc_protocol::item::ItemStack>,
    off_h: (f32, f32),
    hand_item_visible: bool,
    entity_item_visible: bool,
    /// Пирамида видимости текущего кадра для отсечения мобов (VANILLA: EntityRenderer#shouldRender).
    entity_frustum: Option<Frustum>,
    fog_buf: wgpu::Buffer,
    globals_buf: wgpu::Buffer,
    chunk_buf: wgpu::Buffer,
    group0: wgpu::BindGroup,
    depth: Option<(wgpu::TextureView, u32, u32)>,
    /// Отдельный буфер глубины руки: мировой `depth` после кадра остаётся для depth-теста неймтегов.
    hand_depth: Option<wgpu::TextureView>,
    gpu_queue: wgpu::Queue,
    color_format: wgpu::TextureFormat,
    atlas_view: wgpu::TextureView,
    atlas_tex: wgpu::Texture,
    atlas_anim: crate::atlasanim::AtlasAnim,
    /// Включён ли compute-рендер (флаг; без поддержки GPU остаётся `false`).
    use_compute: bool,
    vox: Option<Renderer>,
    rects: Arc<RectTable>,
    /// Секции, чьи цельные кубы лежат в `vox`.
    vox_resident: HashSet<SectionPos>,
    hiz: Option<HiZ>,
    /// Hi-Z и `prev` относятся к прошлому кадру того же размера.
    hiz_valid: bool,
    prev: Option<(Mat4, [f64; 3])>,
    atlas_size: (u32, u32),
    /// Секции, лежащие в арене.
    resident: HashSet<SectionPos>,
    /// Сколько раз секцию не удалось загрузить в арену (нехватка места) и её поставили на перестройку.
    arena_retry: std::collections::HashMap<SectionPos, u8>,
    min_y: i32,
    height: i32,
    view_radius: i32,
    /// VANILLA: ClientLevelData#isFlat (плоский мир: горизонт = minY, порог тьмы пустоты = 1).
    is_flat: bool,
    pub cam: [f64; 3],
    pub have_pos: bool,
    /// Поколение телепорта, которое главный поток уже применил (см. `NetHandle::tp_gen`).
    tp_gen_applied: u64,
    /// Последнее значение общей позиции, которое записал главный поток (или телепорт сервера).
    /// Если общая позиция другая — сетевой поток записал телепорт, его ещё не применили.
    last_pos_sent: Option<[f64; 6]>,
    /// Физика игрока (ходьба); `walking == false` — отладочный свободный полёт (EZZMC_FREECAM=1).
    pub player: crate::physics::Player,
    pub walking: bool,
    tick_acc: f64,
    /// Игрок сидит на транспорте (set_passengers): ходьба не тикает, шлём только поворот.
    riding: bool,
    /// Последний учтённый Level::own_flags_seq (синхронизация флага FALL_FLYING).
    own_flags_seen: u32,
    /// Нос лодки, на которой едет игрок (для клэмпа взгляда по мыши: Entity#turn → onPassengerTurned).
    pub ride_boat_yaw: Option<f32>,
    /// Поворот вида за последний тик в лодке (VANILLA: lerp(yRotO, yRot, partialTick) для камеры).
    ride_dr: f32,
    /// Сколько градусов вычли из `yaw` в этом кадре ради лерпа (main.rs возвращает при чтении).
    pub render_yaw_off: f32,
    /// Лодка, которой управляет игрок: id сущности и клиентская симуляция (TODO-206).
    boat: Option<(i32, crate::boat::BoatSim)>,
    /// Результат тика лодки для сетевого потока: вёсла, позиция, yaw, on_ground.
    boat_out: Option<crate::boat::BoatOut>,
    /// VANILLA: GameRenderer#fovModifier / oldFovModifier (сглаживание 0.5 за тик).
    fov_mod: (f32, f32),
    /// VANILLA: Camera#getFluidInCamera на последнем кадре (для modifyFovBasedOnDeathOrFluid).
    cam_fluid: ezzmc_render::fog::FogType,
    /// LocalPlayer#spinningEffectTime/Speed (Nausea, портал).
    spin: (f32, f32),
    /// LocalPlayer#oPortalEffectIntensity / portalEffectIntensity.
    portal_eff: (f32, f32),
    /// VANILLA: Player#sleepCounter (0..110), см. `tick_sleep`.
    sleep_timer: i32,
    /// Поза SLEEPING у локального игрока (Entity#isSleeping), обновляется в `tick_sleep`.
    pub sleeping: bool,
    /// Зум по удержанию (клавиша `key.zoom`): цель и текущий множитель FOV (1.0 — без зума).
    pub zoom_held: bool,
    pub zoom_factor: f32,
    /// VANILLA: Hud#scopeScale — масштаб оверлея подзорной трубы (0.5 → 1.125).
    pub scope_scale: f32,
    /// Снимок настроек (options.txt), обновляется из main каждый кадр.
    pub cfg: crate::settings::ViewCfg,
    /// Отладка (EZZMC_TOUR): не слать серверу позицию камеры.
    pub no_send: bool,
    pub yaw: f32,
    pub pitch: f32,
    /// VANILLA: CameraType (0 = FIRST_PERSON, 1 = THIRD_PERSON_BACK, 2 = THIRD_PERSON_FRONT).
    pub perspective: u8,
    pub status: Option<String>,
    pub joined: bool,
    /// VANILLA: MultiPlayerGameMode#carriedIndex — последний отправленный серверу слот хотбара.
    carried_sent: u8,
    /// Здоровье (0 — мёртв, показывается экран смерти).
    pub health: f32,
    /// Входящий чат для главного потока: (текст, индикатор; None — action bar).
    pub sugg_in: Vec<(i32, i32, i32, Vec<String>)>,
    pub tab: crate::tablist::TabList,
    pub boss: crate::bossbar::BossBars,
    /// VANILLA: GameRenderer#bossOverlayWorldDarkening / …O (тиковое, лерп по partialTick).
    boss_darken: f32,
    boss_darken_o: f32,
    /// Титры `/title` (VANILLA: Hud title/subtitle).
    pub titles: crate::titles::Titles,
    pub subtitles: crate::subtitles::Subtitles,
    pub advancements: crate::advancements::Advancements,
    /// ClientRecipeBook (TODO-323a).
    pub recipes: crate::recipebook::RecipeBook,
    /// Ghost-рецепт в окне крафта (GhostSlots): слоты меню с вариантами предметов.
    pub ghost: Vec<crate::recipebook::GhostSlot>,
    /// Статистика игрока (award_stats).
    pub stats: crate::stats::Stats,
    /// Достижения, по которым пора показать toast (язык есть только у main).
    pub toast_in: Vec<String>,
    pub toasts: crate::toasts::ToastManager,
    /// Строки субтитров последнего кадра (текст, стрелка, яркость) — для агента.
    pub subtitle_rows: Vec<(String, i8, u32)>,
    /// Scoreboard (цели/очки/команды). VANILLA: ClientPacketListener#scoreboard.
    pub scoreboard: crate::scoreboard::Scoreboard,
    /// Карты (TODO-525): данные и атлас `map_draw`.
    pub maps: crate::maps::Maps,
    map_draw: ezzmc_render::particledraw::ParticleDraw,
    /// Карта в руке: свой атлас (`maps::HAND_ATLAS`), проекция руки.
    hand_map_draw: ezzmc_render::particledraw::ParticleDraw,
    hand_map_visible: bool,
    /// Подписи значков карт: в рамках (координаты от камеры) и в руке (уже с проекцией руки).
    map_labels: Vec<crate::maps::Label>,
    hand_map_labels: Vec<crate::maps::Label>,
    /// (id, version) карт, залитых в `hand_map_draw` (основная, вторая рука).
    hand_map_seen: [Option<(i32, u32)>; 2],
    /// Неймтеги кадра: (позиция якоря, текст, присед) и матрица проекции этого кадра.
    /// Неймтеги: якорь, имя (с оформлением команды), присед, строка очков below_name.
    nametags: Vec<NameTag>,
    /// Ник локального игрока (команды/scoreboard).
    pub own_name: String,
    nt_view: (Mat4, [f64; 3], f32),
    /// ChatVisiblity id из настроек (выставляется из main каждый кадр).
    pub chat_vis: i32,
    /// VANILLA: PlayerSocialManager#hiddenPlayers — UUID, чьи сообщения скрыты (только в памяти).
    pub hidden_players: std::collections::HashSet<u128>,
    pub chat_in: Vec<(ezzmc_protocol::text::Component, Option<crate::chat::Tag>)>,
    /// Диалоги от сервера: Some — показать, None — clear_dialog.
    pub dialog_in: Vec<Option<Box<crate::dialog::Dialog>>>,
    /// open_sign_editor: (позиция, передняя сторона) — забирает main.
    pub sign_in: Vec<([i32; 3], bool)>,
    /// Клик по командному блоку (creative + op ≥ 2): (позиция, имя блока) — забирает main.
    pub cmd_in: Vec<([i32; 3], &'static str)>,
    /// block_entity_data пришёл для этих позиций — забирает main.
    pub be_changed: Vec<[i32; 3]>,
    /// ПКМ пером: (слот, страницы) — main открывает экран правки книги.
    pub book_in: Option<(i32, Vec<String>)>,
    lines: ezzmc_render::lines::LineDraw,
    /// Чёрная подложка контура блока при highContrastBlockOutline.
    outline_back: ezzmc_render::lines::LineDraw,
    /// F3: 3D-прицел (чёрная обводка + оси X/Y/Z); `Some(guiScale)` включает.
    /// VANILLA: DebugCrosshairRenderer.
    pub debug_crosshair: Option<u32>,
    /// F1: HUD скрыт — без руки (VANILLA: GameRenderer#renderItemInHand, isHudHidden).
    pub hud_hidden: bool,
    cross_lines: [ezzmc_render::lines::LineDraw; 4],
    /// F3+B: хитбоксы сущностей. VANILLA: EntityHitboxDebugRenderer.
    pub show_hitboxes: bool,
    /// Цвета: белый (бокс), красный (глаза), синий (взгляд), зелёный (части дракона).
    hit_lines: [ezzmc_render::lines::LineDraw; 4],
    /// F3+G: границы чанков. VANILLA: ChunkBorderRenderer.
    pub show_chunk_borders: bool,
    /// Группы: красные w4, жёлтые w1, бирюзовые w1, синие толстые w4, синие тонкие w1.
    border_lines: [ezzmc_render::lines::LineDraw; 5],
    /// Блок под прицелом (контур) и жидкость под прицелом (F3); обновляются в `update`.
    pub target: Option<crate::pick::Target>,
    pub fluid_target: Option<crate::pick::Target>,
    /// Дальность 20 (F3 «looking at»), без ограничения interaction range.
    pub look_target: Option<crate::pick::Target>,
    /// Блок для постановки/использования (ПКМ): выбирается в конце тика физики от глаз,
    /// которые уже ушли в пакет движения (VANILLA: Minecraft#tick → pick(1.0F)).
    pub place_target: Option<crate::pick::Target>,
    pub look_fluid: Option<crate::pick::Target>,
    /// Сущность под прицелом (id) — ближе блока (Minecraft#hitResult = EntityHitResult).
    pub entity_target: Option<i32>,
    /// Расстояние от глаз до точки попадания в `entity_target` (для `attack_range`).
    pub entity_dist: f64,
    /// Секций, прошедших frustum в последнем кадре (F3 `C:`).
    pub visible_sections: usize,
    /// Ключ последнего расчёта видимых секций/команд (матрица, камера, ревизия арены): кадры без
    /// изменений пропускают фильтр фрустума, пересортировку и сборку индирект-команд.
    draw_key: Option<([u32; 16], [u64; 3], u64)>,
}

fn buf(device: &wgpu::Device, label: &str, size: usize) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: size as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn upload_atlas(gpu: &Gpu, atlas: &BlockAtlas) -> (wgpu::Texture, wgpu::TextureView) {
    let levels = atlas.mips.len() as u32;
    let tex = gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("atlas.blocks"),
        size: wgpu::Extent3d {
            width: atlas.width,
            height: atlas.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: levels,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        // VANILLA: текстуры не sRGB, смешивание в «гамма-пространстве» — swapchain тоже не sRGB.
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    for (l, img) in atlas.mips.iter().enumerate() {
        write_mip(gpu, &tex, l as u32, img);
    }
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
    (tex, view)
}

fn write_mip(gpu: &Gpu, tex: &wgpu::Texture, level: u32, img: &Image) {
    gpu.queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: tex,
            mip_level: level,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &img.to_rgba8(),
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(img.width * 4),
            rows_per_image: Some(img.height),
        },
        wgpu::Extent3d {
            width: img.width,
            height: img.height,
            depth_or_array_layers: 1,
        },
    );
}

fn load_colormap(rm: &ResourceManager, name: &str) -> Vec<u32> {
    rm.read(&format!("assets/minecraft/textures/colormap/{name}.png"))
        .and_then(|d| Image::decode_png(&d).ok())
        .map(|i| i.px)
        .unwrap_or_default()
}

/// VANILLA: MoonPhase.index() — FULL, WANING_GIBBOUS, THIRD_QUARTER, WANING_CRESCENT, NEW,
/// WAXING_CRESCENT, FIRST_QUARTER, WAXING_GIBBOUS.
const MOON_FILES: [&str; 8] = [
    "full_moon",
    "waning_gibbous",
    "third_quarter",
    "waning_crescent",
    "new_moon",
    "waxing_crescent",
    "first_quarter",
    "waxing_gibbous",
];

fn load_celestials(rm: &ResourceManager) -> CelestialImages {
    let rd = |p: &str| {
        rm.read(&format!(
            "assets/minecraft/textures/environment/celestial/{p}.png"
        ))
        .and_then(|d| Image::decode_png(&d).ok())
    };
    let sun = rd("sun").unwrap_or_else(|| Image::new(32, 32));
    let (w, h) = (sun.width, sun.height);
    let fit = |i: Option<Image>| match i {
        Some(i) if i.width == w && i.height == h => i.to_rgba8(),
        _ => Image::new(w, h).to_rgba8(),
    };
    CelestialImages {
        width: w,
        height: h,
        sun: sun.to_rgba8(),
        moons: MOON_FILES.map(|n| fit(rd(&format!("moon/{n}")))),
        end_sky: rm
            .read("assets/minecraft/textures/environment/end_sky.png")
            .and_then(|d| Image::decode_png(&d).ok())
            .map_or_else(
                || (vec![0, 0, 0, 255], 1, 1),
                |i| (i.to_rgba8(), i.width, i.height),
            ),
    }
}

/// Плавание для корневой матрицы. VANILLA: AvatarRenderer#setupRotations (swimAmount > 0).
#[derive(Clone, Copy, Default, PartialEq)]
pub struct SwimPose {
    pub amount: f32,
    pub in_water: bool,
    pub visually: bool,
    pub x_rot: f32,
    /// Полёт на элитрах: перекрывает плавание (AvatarRenderer#setupRotations).
    pub fly: Option<crate::avatar::FlyPose>,
}

impl SwimPose {
    pub fn matrix(self) -> Mat4 {
        if let Some(f) = self.fly {
            // VANILLA: AvatarRenderer#setupRotations — XP(scale·(−90 − xRot)), затем YP(flyingYRot).
            return Mat4::from_rotation_x((f.scale * (-90.0 - self.x_rot)).to_radians())
                * Mat4::from_rotation_y(f.y_rot);
        }
        if self.amount <= 0.0 {
            return Mat4::IDENTITY;
        }
        let target = if self.in_water {
            -90.0 - self.x_rot
        } else {
            -90.0
        };
        let r = Mat4::from_rotation_x((self.amount * target).to_radians());
        if self.visually {
            r * Mat4::from_translation(Vec3::new(0.0, -1.0, 0.3))
        } else {
            r
        }
    }
}

/// VANILLA: Pose.SLEEPING (ordinal).
const POSE_SLEEPING: i32 = 2;

/// VANILLA: Player#tick, ветка sleepCounter.
fn next_sleep_timer(t: i32, sleeping: bool) -> i32 {
    if sleeping {
        (t + 1).min(100)
    } else if t > 0 {
        let n = t + 1;
        if n >= 110 {
            0
        } else {
            n
        }
    } else {
        0
    }
}

/// VANILLA: PlayerModel#translateToHand — у slim рука сдвигается на ±0.5 px по X до поворота.
fn slim_hand_matrix(arm: Mat4, root: Mat4, slim: bool, side: f32) -> Mat4 {
    if !slim {
        return arm;
    }
    root * Mat4::from_translation(Vec3::new(0.5 * side / 16.0, 0.0, 0.0)) * root.inverse() * arm
}

impl WorldView {
    /// VANILLA: Entity#updateFluidInteraction → doWaterSplashEffect (частицы) для чужих сущностей.
    fn entity_splashes(
        particles: &mut crate::particles::ParticleEngine,
        wet_entities: &mut std::collections::HashMap<i32, bool>,
        level: &Level,
    ) {
        let names = ezzmc_protocol::generated::r26_3::ENTITY_TYPE;
        let mut seen = std::collections::HashSet::new();
        for e in level.entities.iter() {
            seen.insert(e.id);
            let (w, h) = names
                .get(e.ty)
                .and_then(|n| ezzmc_world::entity_size::size(n))
                .unwrap_or((0.6, 1.8));
            let (w, h) = (f64::from(w), f64::from(h));
            let bb = [
                e.pos[0] - w / 2.0,
                e.pos[1],
                e.pos[2] - w / 2.0,
                e.pos[0] + w / 2.0,
                e.pos[1] + h,
                e.pos[2] + w / 2.0,
            ];
            let wet = crate::physics::fluid_interaction(level, bb)[0].0 > 0.0;
            // firstTick: новая сущность не плещется.
            let was = wet_entities.insert(e.id, wet);
            if wet && was == Some(false) {
                let mut vel = e.vel;
                if vel == [0.0; 3] {
                    vel = [
                        e.pos[0] - e.prev_pos[0],
                        e.pos[1] - e.prev_pos[1],
                        e.pos[2] - e.prev_pos[2],
                    ];
                }
                particles.water_splash_effect(e.pos, w, vel);
            }
        }
        wet_entities.retain(|id, _| seen.contains(id));
    }

    pub fn new(
        gpu: &Gpu,
        color: wgpu::TextureFormat,
        rm: &ResourceManager,
        net: NetHandle,
        mesh_threads: usize,
        opts: RenderOptions,
        prebuilt: Option<(BlockAtlas, BlockModels)>,
    ) -> Result<Self, String> {
        let (atlas, mut models) = match prebuilt {
            Some(p) => p,
            None => {
                let atlas = BlockAtlas::build(rm, 4)?;
                let models = BlockModels::build(rm, &atlas, opts.cutout_leaves);
                (atlas, models)
            }
        };
        models.tints.maps.grass = load_colormap(rm, "grass");
        models.tints.maps.foliage = load_colormap(rm, "foliage");
        models.tints.maps.dry_foliage = load_colormap(rm, "dry_foliage");
        let (atlas_tex, atlas_view) = upload_atlas(gpu, &atlas);
        let atlas_anim = crate::atlasanim::AtlasAnim::new(&atlas);
        let atlas_sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("atlas"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            // VANILLA: LevelRenderer#chunkLayerSampler — LINEAR/LINEAR (min = LINEAR_MIPMAP_LINEAR);
            // sampleNearest в шейдере только подгоняет UV, сглаживание вдали даёт линейный фильтр.
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });
        let lightmap = LightmapTexture::new(&gpu.device);
        lightmap.update(&gpu.queue, &LightmapParams::default());
        let lm_sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("lightmap"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // Не больше, чем позволяет видеокарта (вершина = 16 байт); дальность 127 требует десятков млн вершин.
        let vertex_capacity = VERTEX_CAPACITY
            .min((gpu.device.limits().max_buffer_size / 16).min(u64::from(u32::MAX)) as u32);
        tracing::info!("арена террейна: {vertex_capacity} вершин");
        let arena = TerrainArena::new(
            &gpu.device,
            &gpu.queue,
            vertex_capacity,
            INDEX_ARENA_CAPACITY
                .min((gpu.device.limits().max_buffer_size / 4).min(u64::from(u32::MAX)) as u32 / 2),
            MAX_SECTIONS,
        );
        let pipes = TerrainPipelines::new(&gpu.device, color, Some(DEPTH_FORMAT));
        let projection_buf = buf(&gpu.device, "projection", ProjectionUniform::SIZE);
        let fog_buf = buf(&gpu.device, "fog", FogUniform::SIZE);
        let globals_buf = buf(&gpu.device, "globals", GlobalsUniform::SIZE);
        let chunk_buf = buf(&gpu.device, "chunk", ChunkSectionUniform::SIZE);
        let group0 = pipes.bind_group0(
            &gpu.device,
            &projection_buf,
            &fog_buf,
            &globals_buf,
            &chunk_buf,
            &arena,
        );
        let group1 = pipes.bind_group1(
            &gpu.device,
            &atlas_view,
            &atlas_sampler,
            &lightmap.view,
            &lm_sampler,
        );
        let sky = SkyDraw::new(
            &gpu.device,
            &gpu.queue,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            &fog_buf,
            &load_celestials(rm),
        );
        let cloud_img = rm
            .read("assets/minecraft/textures/environment/clouds.png")
            .and_then(|d| Image::decode_png(&d).ok());
        let clouds = ezzmc_render::clouddraw::CloudDraw::new(
            &gpu.device,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            &fog_buf,
            cloud_img
                .as_ref()
                .map(|i| (i.px.as_slice(), i.width, i.height)),
        );
        let weather_img = |n: &str| {
            rm.read(&format!("assets/minecraft/textures/environment/{n}.png"))
                .and_then(|d| Image::decode_png(&d).ok())
                .map(|i| (i.to_rgba8(), i.width, i.height))
        };
        let (rain_img, snow_img) = (weather_img("rain"), weather_img("snow"));
        let forcefield = rm
            .read("assets/minecraft/textures/misc/forcefield.png")
            .and_then(|d| Image::decode_png(&d).ok())
            .map(|i| (i.to_rgba8(), i.width, i.height));
        let border_draw = ezzmc_render::borderdraw::BorderDraw::new(
            &gpu.device,
            &gpu.queue,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            forcefield.as_ref().map(|(d, w, h)| (d.as_slice(), *w, *h)),
        );
        let weather = ezzmc_render::weatherdraw::WeatherDraw::new(
            &gpu.device,
            &gpu.queue,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            &fog_buf,
            &lightmap.view,
            rain_img.as_ref().map(|(d, w, h)| (d.as_slice(), *w, *h)),
            snow_img.as_ref().map(|(d, w, h)| (d.as_slice(), *w, *h)),
        );
        let vignette_img = rm
            .read("assets/minecraft/textures/misc/vignette.png")
            .and_then(|d| Image::decode_png(&d).ok())
            .map(|i| (i.to_rgba8(), i.width, i.height));
        let (vpx, vw, vh) = vignette_img.unwrap_or((vec![0, 0, 0, 255], 1, 1));
        let vignette = ezzmc_render::vignette::VignetteDraw::new(
            &gpu.device,
            &gpu.queue,
            color,
            Some(DEPTH_FORMAT),
            &vpx,
            vw,
            vh,
        );
        let lightning = ezzmc_render::lightningdraw::LightningDraw::new(
            &gpu.device,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            &fog_buf,
        );
        let end_portal_draw = {
            let rd = |p: &str| {
                rm.read(p)
                    .and_then(|d| Image::decode_png(&d).ok())
                    .map_or_else(
                        || (vec![0, 0, 0, 255], 1, 1),
                        |i| (i.to_rgba8(), i.width, i.height),
                    )
            };
            let (sk, skw, skh) = rd("assets/minecraft/textures/environment/end_sky.png");
            let (pt, ptw, pth) = rd("assets/minecraft/textures/entity/end_portal/end_portal.png");
            ezzmc_render::endportaldraw::EndPortalDraw::new(
                &gpu.device,
                &gpu.queue,
                color,
                Some(DEPTH_FORMAT),
                &projection_buf,
                &fog_buf,
                &ezzmc_render::endportaldraw::Rgba {
                    data: &sk,
                    width: skw,
                    height: skh,
                },
                &ezzmc_render::endportaldraw::Rgba {
                    data: &pt,
                    width: ptw,
                    height: pth,
                },
            )
        };
        let beacon_draw = {
            let rd = |p: &str| {
                rm.read(p)
                    .and_then(|d| Image::decode_png(&d).ok())
                    .map_or_else(
                        || (vec![255, 255, 255, 255], 1, 1),
                        |i| (i.to_rgba8(), i.width, i.height),
                    )
            };
            let (b, bw, bh) = rd("assets/minecraft/textures/entity/beacon/beacon_beam.png");
            let (g, gw, gh) =
                rd("assets/minecraft/textures/entity/end_portal/end_gateway_beam.png");
            let img = |data, width, height| ezzmc_render::endportaldraw::Rgba {
                data,
                width,
                height,
            };
            ezzmc_render::beacondraw::BeaconDraw::new(
                &gpu.device,
                &gpu.queue,
                color,
                DEPTH_FORMAT,
                &projection_buf,
                &fog_buf,
                [&img(&b, bw, bh), &img(&g, gw, gh)],
            )
        };
        // VANILLA: ParticleResources — атлас `particles` и наборы спрайтов `particles/<name>.json`.
        let particle_atlas = BlockAtlas::build_named(rm, "particles", 0)
            .map_err(|e| tracing::warn!("атлас частиц: {e}"))
            .ok();
        let mut sets = crate::particles::SpriteSets::default();
        if let Some(pa) = &particle_atlas {
            for name in [
                "rain",
                "smoke",
                "flame",
                "soul_fire_flame",
                "copper_fire_flame",
                "lava",
                "underwater",
                "egg_crack",
                "small_flame",
                "sculk_charge_pop",
                "trial_spawner_detection",
                "vault_connection",
                "shriek",
                "sculk_charge",
                "dust",
                "note",
                "large_smoke",
                "dripping_water",
                "falling_water",
                "dripping_lava",
                "falling_lava",
                "landing_lava",
                "splash",
                "bubble",
                "portal",
                "dragon_breath",
                "end_rod",
                "crit",
                "explosion",
                "poof",
                "glow",
                "bubble_column_up",
                "current_down",
                "happy_villager",
                "effect",
                "instant_effect",
                "cloud",
                "white_smoke",
                "electric_spark",
                "wax_on",
                "wax_off",
                "scrape",
                "firework",
                "flash",
            ] {
                let list: Vec<_> = rm
                    .read(&format!("assets/minecraft/particles/{name}.json"))
                    .and_then(|d| serde_json::from_slice::<serde_json::Value>(&d).ok())
                    .and_then(|v| {
                        v.get("textures")?.as_array().map(|a| {
                            a.iter()
                                .filter_map(|t| t.as_str())
                                .filter_map(|t| {
                                    pa.sprites.get(&ezzmc_assets::atlas::Identifier::parse(t))
                                })
                                .copied()
                                .collect()
                        })
                    })
                    .unwrap_or_default();
                sets.insert(name, list);
            }
        }
        let particle_rgba = particle_atlas
            .as_ref()
            .map(|a| (a.mips[0].to_rgba8(), a.width, a.height));
        let particle_draw = ezzmc_render::particledraw::ParticleDraw::new(
            &gpu.device,
            &gpu.queue,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            &fog_buf,
            &lightmap.view,
            &atlas_view,
            particle_rgba
                .as_ref()
                .map(|(d, w, h)| (d.as_slice(), *w, *h)),
        );
        // VANILLA: DefaultPlayerSkin — оффлайн-клиент всегда Стив (wide/steve.png, 64×64).
        let skin = rm
            .read("assets/minecraft/textures/entity/player/wide/steve.png")
            .and_then(|d| Image::decode_png(&d).ok())
            .map(|i| (i.to_rgba8(), i.width, i.height));
        let entity_draw = ezzmc_render::particledraw::ParticleDraw::new(
            &gpu.device,
            &gpu.queue,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            &fog_buf,
            &lightmap.view,
            &atlas_view,
            skin.as_ref().map(|(d, w, h)| (d.as_slice(), *w, *h)),
        );
        let mut skin_atlas = vec![0u8; (64 * 64 * SKIN_SLOTS * 4) as usize];
        if let Some((d, _, _)) = skin.as_ref().filter(|(_, w, h)| *w == 64 && *h == 64) {
            skin_atlas[..d.len()].copy_from_slice(d);
        }
        let player_draw = ezzmc_render::particledraw::ParticleDraw::new(
            &gpu.device,
            &gpu.queue,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            &fog_buf,
            &lightmap.view,
            &atlas_view,
            Some((skin_atlas.as_slice(), 64, 64 * SKIN_SLOTS)),
        );
        let preview_blocks =
            std::sync::Arc::new((atlas.mips[0].to_rgba8(), atlas.width, atlas.height));
        let armor_kit = crate::armor::ArmorKit::new(rm);
        let armor_draw = ezzmc_render::particledraw::ParticleDraw::new(
            &gpu.device,
            &gpu.queue,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            &fog_buf,
            &lightmap.view,
            &atlas_view,
            Some((armor_kit.rgba.as_slice(), armor_kit.width, armor_kit.height)),
        );
        let glint_draw = {
            let glint = rm
                .read("assets/minecraft/textures/misc/enchanted_glint_armor.png")
                .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
                .map(|i| (i.to_rgba8(), i.width, i.height));
            ezzmc_render::particledraw::ParticleDraw::new_glint(
                &gpu.device,
                &gpu.queue,
                color,
                DEPTH_FORMAT,
                &projection_buf,
                &fog_buf,
                &lightmap.view,
                &atlas_view,
                glint.as_ref().map(|(d, w, h)| (d.as_slice(), *w, *h)),
            )
        };
        let arrow_tex = {
            let mut rgba = Vec::new();
            let mut ok = true;
            for n in ["arrow", "arrow_spectral", "arrow_tipped"] {
                match rm
                    .read(&format!(
                        "assets/minecraft/textures/entity/projectiles/{n}.png"
                    ))
                    .and_then(|d| Image::decode_png(&d).ok())
                {
                    Some(i) if i.width == 32 && i.height == 32 => rgba.extend(i.to_rgba8()),
                    _ => ok = false,
                }
            }
            ok.then_some(rgba)
        };
        let arrow_draw = ezzmc_render::particledraw::ParticleDraw::new_culled(
            &gpu.device,
            &gpu.queue,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            &fog_buf,
            &lightmap.view,
            &atlas_view,
            arrow_tex.as_ref().map(|d| (d.as_slice(), 32, 96)),
        );
        // VANILLA: RenderTypes#text (MapRenderer) — без отсечения граней.
        let map_draw = {
            let px = crate::maps::Maps::atlas_init(Some(rm));
            ezzmc_render::particledraw::ParticleDraw::new(
                &gpu.device,
                &gpu.queue,
                color,
                DEPTH_FORMAT,
                &projection_buf,
                &fog_buf,
                &lightmap.view,
                &atlas_view,
                Some((px.as_slice(), crate::maps::ATLAS, crate::maps::ATLAS)),
            )
        };
        let piston_draw = ezzmc_render::particledraw::ParticleDraw::new_culled(
            &gpu.device,
            &gpu.queue,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            &fog_buf,
            &lightmap.view,
            &atlas_view,
            None,
        );
        let shadow_tex = rm
            .read("assets/minecraft/textures/misc/shadow.png")
            .and_then(|d| Image::decode_png(&d).ok())
            .map(|i| (i.to_rgba8(), i.width, i.height));
        let shadow_draw = ezzmc_render::particledraw::ParticleDraw::new_shadow(
            &gpu.device,
            &gpu.queue,
            color,
            DEPTH_FORMAT,
            &projection_buf,
            &fog_buf,
            &lightmap.view,
            &atlas_view,
            shadow_tex.as_ref().map(|(d, w, h)| (d.as_slice(), *w, *h)),
        );
        let fire_uv = {
            let get = |n: &str| {
                atlas
                    .sprites
                    .get(&ezzmc_assets::atlas::Identifier::parse(n))
                    .copied()
            };
            get("block/fire_0").zip(get("block/fire_1"))
        };
        let hand_proj_buf = buf(&gpu.device, "hand projection", ProjectionUniform::SIZE);
        let hand_draw = ezzmc_render::particledraw::ParticleDraw::new(
            &gpu.device,
            &gpu.queue,
            color,
            DEPTH_FORMAT,
            &hand_proj_buf,
            &fog_buf,
            &lightmap.view,
            &atlas_view,
            skin.as_ref().map(|(d, w, h)| (d.as_slice(), *w, *h)),
        );
        let hand_model = {
            let (mut m, parts) = ezzmc_render::humanoid::player_mesh(false);
            for part in &mut m.parts {
                part.visible = matches!(
                    part.name,
                    "right_arm" | "right_sleeve" | "left_arm" | "left_sleeve"
                );
            }
            (m, parts)
        };
        let hand_map_draw = {
            let px = crate::maps::Maps::hand_atlas_init(rm);
            let (w, h) = crate::maps::HAND_ATLAS;
            ezzmc_render::particledraw::ParticleDraw::new(
                &gpu.device,
                &gpu.queue,
                color,
                DEPTH_FORMAT,
                &hand_proj_buf,
                &fog_buf,
                &lightmap.view,
                &atlas_view,
                Some((px.as_slice(), w, h)),
            )
        };
        let hand_items = crate::handitem::HandItems::new(rm, &atlas.sprites);
        let hand_rm = ezzmc_assets::install::GameInstall::locate("26.3")
            .ok()
            .and_then(|i| crate::packrepo::build_resource_manager(&i).ok());
        let item_glint_tex = rm
            .read("assets/minecraft/textures/misc/enchanted_glint_item.png")
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
            .map(|i| (i.to_rgba8(), i.width, i.height));
        let (hand_glint_draw, entity_glint_draw) = match &hand_items {
            Some(h) => {
                let (rgba, w, hh) = h.items_rgba();
                let mk = |proj: &wgpu::Buffer| {
                    ezzmc_render::particledraw::ParticleDraw::new_item_glint(
                        &gpu.device,
                        &gpu.queue,
                        color,
                        DEPTH_FORMAT,
                        proj,
                        &fog_buf,
                        &lightmap.view,
                        &atlas_view,
                        Some((rgba.as_slice(), w, hh)),
                        item_glint_tex
                            .as_ref()
                            .map(|(d, w, h)| (d.as_slice(), *w, *h)),
                    )
                };
                (Some(mk(&hand_proj_buf)), Some(mk(&projection_buf)))
            }
            None => (None, None),
        };
        let (hand_item_draw, entity_item_draw) = match &hand_items {
            Some(h) => {
                let (rgba, w, hh) = h.items_rgba();
                let mk = |proj: &wgpu::Buffer| {
                    ezzmc_render::particledraw::ParticleDraw::new(
                        &gpu.device,
                        &gpu.queue,
                        color,
                        DEPTH_FORMAT,
                        proj,
                        &fog_buf,
                        &lightmap.view,
                        &atlas_view,
                        Some((rgba.as_slice(), w, hh)),
                    )
                };
                (Some(mk(&hand_proj_buf)), Some(mk(&projection_buf)))
            }
            None => (None, None),
        };
        let item_inst = match (&entity_item_draw, &entity_glint_draw) {
            (Some(a), Some(b)) => Some(ezzmc_render::particledraw::ItemInstDraw::new(
                &gpu.device,
                a,
                b,
            )),
            _ => None,
        };
        let mut vox = None;
        let mut use_compute = false;
        if opts.use_compute_renderer {
            vox = Self::make_renderer(gpu, color);
            use_compute = vox.is_some();
        }
        let mut view = WorldView {
            gpu_queue: gpu.queue.clone(),
            color_format: color,
            atlas_view: atlas_view.clone(),
            atlas_tex,
            atlas_anim,
            use_compute,
            vox,
            rects: Arc::new(RectTable::default()),
            vox_resident: HashSet::default(),
            hiz: None,
            hiz_valid: false,
            prev: None,
            net,
            cutout_leaves: opts.cutout_leaves,
            models: Arc::new(models),
            pool: MeshPool::new(mesh_threads).map_err(|e| e.to_string())?,
            queue: RebuildQueue::default(),
            arena,
            pipes,
            lightmap,
            sky,
            clouds,
            weather,
            lightning,
            end_portal_draw,
            beacon_draw,
            beacons: crate::beacon::Beacons::default(),
            vault_scan: crate::berscan::Scanner::default(),
            beacon_batches: Default::default(),
            end_portals: crate::endportal::EndPortals::default(),
            end_portal_verts: Vec::new(),
            border_draw,
            entity_draw,
            player_draw,
            skin_slots: HashMap::new(),
            player_model_slim: ezzmc_render::humanoid::player_mesh(true),
            skin_uploads: Vec::new(),
            armor_draw,
            armor_kit,
            glint_draw,
            preview_blocks,
            preview_items: None,
            mobs: crate::mobs::Mobs::default(),
            now_playing: None,
            chests: crate::chest::Chests::default(),
            bells: crate::bell::Bells::default(),
            tables: crate::enchtable::Tables::default(),
            skulls: crate::skull::Skulls::default(),
            shulker_boxes: crate::shulkerbox::ShulkerBoxes::default(),
            banners: crate::banner::Banners::default(),
            signs: crate::sign::Signs::default(),
            arrow_draw,
            piston_draw,
            shadow_draw,
            sky_darken: 0,
            head_skulls: Vec::new(),
            player_model: ezzmc_render::humanoid::player_mesh(false),
            particles: crate::particles::ParticleEngine::new(sets),
            particle_draw,
            last_particle_tick: None,
            last_piston_tick: None,
            remote_anims: Default::default(),
            remote_tick: None,
            wet_entities: Default::default(),
            sky_state: None,
            end_sky: false,
            fog_state: ezzmc_render::fog::FogState::default(),
            vignette,
            vignette_b: 1.0,
            vignette_warn: 0.0,
            vignette_acc: 0.0,
            probe_pair: None,
            probe_acc: 0.0,
            probe_colors: [
                ezzmc_render::sky::DEFAULT_SKY_COLOR,
                ezzmc_render::fog::FogAttrs::default().base_fog,
                ezzmc_render::fog::FogAttrs::default().water_fog_color,
            ],
            last_frame: None,
            lightmap_key: None,
            effects: Default::default(),
            absorption: 0.0,
            group1,
            projection_buf,
            hand_proj_buf,
            hand_draw,
            hand_model,
            hand_visible: false,
            hand_items,
            hand_rm,
            hand_item_draw,
            entity_item_draw,
            hand_glint_draw,
            item_inst,
            item_inst_map: std::collections::HashMap::new(),
            item_inst_keep: Vec::new(),
            item_inst_frame: Vec::new(),
            entity_glint_draw,
            fire_uv,
            hand_shown: None,
            off_shown: None,
            off_h: (0.0, 0.0),
            hand_item_visible: false,
            entity_item_visible: false,
            entity_frustum: None,
            fog_buf,
            globals_buf,
            chunk_buf,
            group0,
            depth: None,
            hand_depth: None,
            atlas_size: (atlas.width, atlas.height),
            resident: HashSet::default(),
            arena_retry: std::collections::HashMap::default(),
            min_y: -64,
            height: 384,
            view_radius: 8,
            is_flat: false,
            cam: [0.0, 100.0, 0.0],
            have_pos: false,
            last_pos_sent: None,
            tp_gen_applied: 0,
            player: crate::physics::Player::new([0.0; 3]),
            last_input: crate::physics::Input::default(),
            game_mode: 0,
            prev_game_mode: -1,
            block_range: None,
            attack_speed: None,
            entity_range: None,
            mining_efficiency: None,
            block_break_speed: 1.0,
            submerged_mining_speed: None,
            op_level: 0,
            inv: crate::inventory::Inventory::default(),
            attack_held: false,
            psound: Default::default(),
            ambient: crate::ambient::Ambient::new(),
            attack_ticker: 0,
            right_click_delay: 0,
            miss_time: 0,
            food: 20,
            saturation: 5.0,
            xp_progress: 0.0,
            xp_level: 0,
            waypoints: std::collections::HashMap::new(),
            xp_changed_at: None,
            xp_seen: false,
            armor: 0.0,
            max_health: 20.0,
            hardcore: false,
            show_death_screen: true,
            score: 0,
            death_in: None,
            respawn_in: false,
            loading_in: None,
            respawn_wait: false,
            win_game: false,
            death_pending: false,
            health_seen: false,
            invulnerable: false,
            using: None,
            attack_item: None,
            digger: crate::dig::Digger::default(),
            walking: std::env::var("EZZMC_FREECAM").map_or(true, |v| v != "1"),
            tick_acc: 0.0,
            riding: false,
            own_flags_seen: 0,
            ride_boat_yaw: None,
            ride_dr: 0.0,
            render_yaw_off: 0.0,
            boat: None,
            boat_out: None,
            fov_mod: (1.0, 1.0),
            cam_fluid: ezzmc_render::fog::FogType::Atmospheric,
            spin: (0.0, 0.0),
            zoom_held: false,
            zoom_factor: 1.0,
            scope_scale: 0.5,
            cfg: crate::settings::ViewCfg::default(),
            no_send: false,
            yaw: 0.0,
            pitch: 0.0,
            perspective: 0,
            status: None,
            joined: false,
            carried_sent: 0,
            health: 20.0,
            sugg_in: Vec::new(),
            tab: Default::default(),
            boss: Default::default(),
            portal_eff: (0.0, 0.0),
            sleep_timer: 0,
            sleeping: false,
            boss_darken: 0.0,
            boss_darken_o: 0.0,
            titles: Default::default(),
            subtitles: Default::default(),
            advancements: Default::default(),
            recipes: Default::default(),
            ghost: Vec::new(),
            stats: Default::default(),
            toast_in: Vec::new(),
            toasts: Default::default(),
            subtitle_rows: Vec::new(),
            scoreboard: Default::default(),
            maps: Default::default(),
            map_draw,
            hand_map_draw,
            hand_map_visible: false,
            map_labels: Vec::new(),
            hand_map_labels: Vec::new(),
            hand_map_seen: [None; 2],
            nametags: Vec::new(),
            own_name: String::new(),
            nt_view: (Mat4::IDENTITY, [0.0; 3], 1.0),
            chat_vis: 0,
            hidden_players: Default::default(),
            chat_in: Vec::new(),
            dialog_in: Vec::new(),
            sign_in: Vec::new(),
            cmd_in: Vec::new(),
            be_changed: Vec::new(),
            book_in: None,
            lines: ezzmc_render::lines::LineDraw::new(&gpu.device, color, DEPTH_FORMAT),
            outline_back: ezzmc_render::lines::LineDraw::new(&gpu.device, color, DEPTH_FORMAT),
            debug_crosshair: None,
            hud_hidden: false,
            cross_lines: std::array::from_fn(|_| {
                ezzmc_render::lines::LineDraw::new(&gpu.device, color, DEPTH_FORMAT)
            }),
            show_hitboxes: false,
            show_chunk_borders: false,
            border_lines: std::array::from_fn(|_| {
                ezzmc_render::lines::LineDraw::new(&gpu.device, color, DEPTH_FORMAT)
            }),
            hit_lines: std::array::from_fn(|_| {
                ezzmc_render::lines::LineDraw::new(&gpu.device, color, DEPTH_FORMAT)
            }),
            target: None,
            fluid_target: None,
            look_target: None,
            place_target: None,
            look_fluid: None,
            entity_target: None,
            entity_dist: 0.0,
            visible_sections: 0,
            draw_key: None,
        };
        view.sync_atlas(gpu);
        Ok(view)
    }

    fn make_renderer(gpu: &Gpu, color: wgpu::TextureFormat) -> Option<Renderer> {
        let mode = RenderPipelineMode::ComputeIndirect;
        let r = Renderer::new(gpu, mode, color, VERTEX_CAPACITY, MAX_SECTIONS);
        match &r {
            Some(r) => tracing::info!("compute-рендер включён (compact={})", r.is_compact()),
            None => tracing::warn!("compute-рендер не поддерживается GPU — ванильный путь"),
        }
        r
    }

    /// Атлас и таблица UV-прямоугольников → GPU (при новых прямоугольниках).
    fn sync_atlas(&mut self, gpu: &Gpu) {
        if let Some(v) = &mut self.vox {
            if let Some(rects) = self.rects.take_dirty() {
                v.set_atlas(
                    gpu,
                    self.atlas_view.clone(),
                    self.lightmap.view.clone(),
                    &rects,
                );
            }
        }
    }

    /// Строка статуса рендера для заголовка окна/отладки.
    pub fn render_label(&self) -> &'static str {
        if self.use_compute {
            "RENDER: Compute (MDI / Hi-Z)"
        } else {
            "RENDER: Vanilla (Packed)"
        }
    }

    pub fn compute_renderer_enabled(&self) -> bool {
        self.use_compute
    }

    /// Живое переключение: вся геометрия перестраивается заново под выбранный путь.
    pub fn set_compute_renderer(&mut self, gpu: &Gpu, on: bool) {
        if on == self.use_compute {
            return;
        }
        let mut cols: HashSet<(i32, i32)> = HashSet::default();
        let arena_pos: Vec<SectionPos> = self.resident.drain().collect();
        let vox_pos: Vec<SectionPos> = self.vox_resident.drain().collect();
        for p in &arena_pos {
            cols.insert((p.x, p.z));
            self.arena.remove(*p);
        }
        if let Some(v) = &mut self.vox {
            for p in &vox_pos {
                cols.insert((p.x, p.z));
                v.remove_section(&self.gpu_queue, [p.x, p.y, p.z]);
            }
        }
        if on && self.vox.is_none() {
            self.vox = Self::make_renderer(gpu, self.color_format);
        }
        self.use_compute = on && self.vox.is_some();
        self.rects.mark_dirty();
        self.sync_atlas(gpu);
        self.hiz_valid = false;
        let _ = self.color_format;
        for (x, z) in cols {
            self.mark_column(x, z);
        }
    }

    /// Опция «See-Through Leaves»: пересобрать модели и перемешить всё видимое.
    /// Смена ресурспаков на лету (VANILLA: Minecraft#reloadResourcePacks): атлас блоков, модели, цвета
    /// биомов. Текстура атласа переписывается на месте, пока размеры совпадают (bind group'ы живы);
    /// иначе — Err, и вызывающий перезапускает клиент.
    pub fn reload_resources(&mut self, gpu: &Gpu, rm: &ResourceManager) -> Result<(), String> {
        let atlas = BlockAtlas::build(rm, 4)?;
        let cur = self.atlas_tex.size();
        if atlas.width != cur.width
            || atlas.height != cur.height
            || atlas.mips.len() as u32 != self.atlas_tex.mip_level_count()
        {
            return Err("размер атласа блоков изменился".into());
        }
        for (l, img) in atlas.mips.iter().enumerate() {
            write_mip(gpu, &self.atlas_tex, l as u32, img);
        }
        self.atlas_anim = crate::atlasanim::AtlasAnim::new(&atlas);
        let mut m = BlockModels::build(rm, &atlas, self.cutout_leaves);
        m.tints = self.models.tints.clone();
        m.tints.maps.grass = load_colormap(rm, "grass");
        m.tints.maps.foliage = load_colormap(rm, "foliage");
        m.tints.maps.dry_foliage = load_colormap(rm, "dry_foliage");
        self.models = Arc::new(m);
        let all: Vec<_> = self.resident.iter().copied().collect();
        for p in all {
            self.queue.mark(p);
        }
        Ok(())
    }

    pub fn set_cutout_leaves(&mut self, rm: &ResourceManager, on: bool) -> Result<(), String> {
        if self.cutout_leaves == on {
            return Ok(());
        }
        let atlas = BlockAtlas::build(rm, 4)?;
        let mut m = BlockModels::build(rm, &atlas, on);
        m.tints = self.models.tints.clone();
        self.models = Arc::new(m);
        self.cutout_leaves = on;
        let all: Vec<_> = self.resident.iter().copied().collect();
        for p in all {
            self.queue.mark(p);
        }
        Ok(())
    }

    fn mark_column(&mut self, cx: i32, cz: i32) {
        for sy in (self.min_y >> 4)..((self.min_y + self.height) >> 4) {
            self.queue.mark(SectionPos {
                x: cx,
                y: sy,
                z: cz,
            });
        }
    }

    /// Колонка поменялась: сама и 8 соседей (меш зависит от соседей).
    fn mark_column_area(&mut self, cx: i32, cz: i32) {
        for dz in -1..=1 {
            for dx in -1..=1 {
                self.mark_column(cx + dx, cz + dz);
            }
        }
    }

    fn drop_column(&mut self, cx: i32, cz: i32) {
        for sy in (self.min_y >> 4)..((self.min_y + self.height) >> 4) {
            let p = SectionPos {
                x: cx,
                y: sy,
                z: cz,
            };
            self.pool.cancel(&p);
            self.queue.remove(&p);
            if self.resident.remove(&p) {
                self.arena.remove(p);
            }
            if self.vox_resident.remove(&p) {
                if let Some(v) = &mut self.vox {
                    v.remove_section(&self.gpu_queue, [p.x, p.y, p.z]);
                }
            }
        }
    }

    fn drain_events(&mut self) {
        // VANILLA: свет, пересчитанный клиентским светодвижком → ремеш секций и соседей.
        if let Ok(mut g) = self.net.level.lock() {
            if let Some(l) = g.as_mut() {
                let dirty = std::mem::take(&mut l.chunks.light_dirty);
                drop(g);
                for (sx, sy, sz) in dirty {
                    for (dx, dy, dz) in [
                        (0, 0, 0),
                        (-1, 0, 0),
                        (1, 0, 0),
                        (0, -1, 0),
                        (0, 1, 0),
                        (0, 0, -1),
                        (0, 0, 1),
                    ] {
                        self.queue.mark(SectionPos {
                            x: sx + dx,
                            y: sy + dy,
                            z: sz + dz,
                        });
                    }
                }
            }
        }
        while let Ok(ev) = self.net.rx.try_recv() {
            match ev {
                NetEvt::Joined {
                    biomes,
                    min_y,
                    height,
                    view_radius,
                    is_flat,
                } => {
                    self.waypoints.clear();
                    // VANILLA: новый LocalPlayer после Login/Respawn — эффекты приходят заново.
                    self.effects.clear();
                    // Смена измерения (Respawn): старые меши выбрасываются.
                    if self.joined {
                        let cols: HashSet<(i32, i32)> = self
                            .resident
                            .iter()
                            .chain(self.vox_resident.iter())
                            .map(|p| (p.x, p.z))
                            .collect();
                        for (cx, cz) in cols {
                            self.drop_column(cx, cz);
                        }
                    }
                    if let Some(m) = Arc::get_mut(&mut self.models) {
                        m.tints.biomes = biomes;
                    }
                    self.min_y = min_y;
                    self.height = height;
                    self.view_radius = view_radius;
                    self.is_flat = is_flat;
                    self.joined = true;
                    tracing::info!(
                        "вход в мир: y {min_y}..{} радиус {view_radius} flat={is_flat}",
                        min_y + height
                    );
                }
                NetEvt::GameMode(m) => {
                    // VANILLA: MultiPlayerGameMode#setLocalMode — previous = текущий при смене.
                    if m != self.game_mode {
                        self.prev_game_mode = self.game_mode;
                    }
                    // VANILLA: LocalPlayer#onGameModeChanged — переход в SPECTATOR обнуляет вертикальную скорость.
                    // VANILLA-UNSURE: ваниль зовёт это из PlayerInfoUpdate(UPDATE_GAME_MODE), здесь — по смене режима.
                    if m == 3 && self.game_mode != 3 {
                        self.player.vel[1] = 0.0;
                    }
                    self.game_mode = m;
                    self.player.spectator = m == 3;
                    // VANILLA: Player#hasInfiniteMaterials — креатив.
                    self.inv.infinite_materials = m == 1;
                }
                NetEvt::Health(h, food, sat) => {
                    // VANILLA: LocalPlayer#hurtTo — не первый set_health и здоровье упало → hurtTime = 10.
                    if self.health_seen && h < self.health {
                        self.player.hurt(None);
                    }
                    self.health_seen = true;
                    self.player.dead = h <= 0.0;
                    self.health = h;
                    self.food = food;
                    self.saturation = sat;
                    // VANILLA: Minecraft#setScreen — игрок isDeadOrDying (вход мёртвым) → DeathScreen/respawn.
                    if h <= 0.0 && !self.death_pending {
                        self.death_pending = true;
                        if self.show_death_screen {
                            self.death_in = Some(None);
                        } else {
                            self.send(crate::netthread::NetCmd::Respawn);
                        }
                    }
                }
                NetEvt::Experience(p, l) => {
                    // VANILLA: LocalPlayer#setExperienceValues — только смена прогресса;
                    // первое изменение (experienceDisplayStartTick == MIN_VALUE) не показывается.
                    if p != self.xp_progress {
                        self.xp_changed_at = if self.xp_seen {
                            Some(std::time::Instant::now())
                        } else {
                            None
                        };
                        self.xp_seen = true;
                    }
                    self.xp_progress = p;
                    self.xp_level = l;
                    self.inv.xp_level = l;
                }
                NetEvt::Effect(id, amp, dur, fl) => self.effects.update(id, amp, dur, fl),
                NetEvt::EffectRemove(id) => self.effects.remove(id),
                NetEvt::Attribute(k, v) => match k {
                    "armor" => self.armor = v,
                    "absorption" => self.absorption = v,
                    "max_health" => self.max_health = v,
                    "attack_speed" => self.attack_speed = Some(v),
                    "block_interaction_range" => self.block_range = Some(f64::from(v)),
                    "entity_interaction_range" => self.entity_range = Some(f64::from(v)),
                    "mining_efficiency" => self.mining_efficiency = Some(v),
                    "block_break_speed" => self.block_break_speed = v,
                    "submerged_mining_speed" => self.submerged_mining_speed = Some(v),
                    _ => {}
                },
                NetEvt::Hardcore(h) => self.hardcore = h,
                NetEvt::ShowDeathScreen(b) => self.show_death_screen = b,
                // VANILLA: ClientPacketListener#handleGameEvent ARROW_HIT_PLAYER → playSound(.., PLAYERS, 0.18, 0.45).
                NetEvt::ArrowHitPlayer => crate::audio::play_at(
                    "entity.arrow.hit_player",
                    crate::audio::Cat::Players,
                    self.cam[0],
                    self.cam[1],
                    self.cam[2],
                    0.18,
                    0.45,
                ),
                NetEvt::Score(s) => self.score = s,
                NetEvt::WinGame => {
                    self.win_game = true;
                }
                NetEvt::Respawned(reason) => {
                    self.respawn_in = true;
                    self.loading_in = Some(reason);
                    self.respawn_wait = true;
                    self.death_pending = false;
                    // VANILLA: новый LocalPlayer — flashOnSetHealth, hurtTime, deathTime сброшены.
                    self.health_seen = false;
                    self.player.hurt_time = 0;
                    self.player.dead = false;
                    self.player.death_time = 0;
                    // VANILLA: handleRespawn — новый LocalPlayer с новым Inventory (selected = 0);
                    // PlayerList#respawn set_held_slot не шлёт, ensureHasSentCarriedItem отправит 0.
                    self.inv.selected = 0;
                    self.player.fall_flying = false;
                }
                NetEvt::CombatKill(msg) => {
                    // VANILLA: handlePlayerCombatKill — DeathScreen либо сразу player.respawn().
                    if let Some(prev) = self.death_in.as_mut() {
                        *prev = msg;
                    } else if !self.death_pending {
                        self.death_pending = true;
                        if self.show_death_screen {
                            self.death_in = Some(msg);
                        } else {
                            self.send(crate::netthread::NetCmd::Respawn);
                        }
                    }
                }
                NetEvt::Waypoint(op, wp) => {
                    // VANILLA: ClientWaypointManager#trackWaypoint/updateWaypoint/untrackWaypoint.
                    if op == 1 {
                        self.waypoints.remove(&wp.key);
                    } else {
                        self.waypoints.insert(wp.key.clone(), wp);
                    }
                }
                NetEvt::UseComplete => self.complete_using(),
                NetEvt::Chat(c, t, player, sender) => {
                    // VANILLA: ChatListener — canReceivePlayerMessages (chatVisibility FULL) /
                    // canReceiveSystemMessages (не HIDDEN).
                    let ok = if player {
                        self.chat_vis == 0
                    } else {
                        self.chat_vis < 2
                    };
                    // VANILLA: ChatListener#handlePlayerChatMessage — isHidden(sender) → пропуск.
                    let hidden = sender.is_some_and(|u| self.hidden_players.contains(&u));
                    if ok && !hidden {
                        self.chat_in.push((c, Some(t)));
                    }
                }
                NetEvt::Overlay(c) => self.chat_in.push((c, None)),
                NetEvt::Tab(e) => self.tab.apply(*e),
                NetEvt::Boss(id, op) => self.boss.apply(id, *op),
                NetEvt::Title(e) => self.titles.apply(e),
                NetEvt::Scoreboard(e) => self.scoreboard.apply(*e),
                NetEvt::Map(e) => self.maps.apply(*e),
                NetEvt::Advancements(u) => {
                    let ids = self.advancements.apply(*u);
                    self.toast_in.extend(ids);
                }
                NetEvt::RecipeBook(u) => {
                    for (cat, res) in self.recipes.apply(u) {
                        self.toasts.add_recipe(cat, res);
                    }
                    self.inv.book_open = self.recipes.settings.open_flags();
                }
                NetEvt::GhostRecipe(c, g) => {
                    // VANILLA: контейнер должен совпасть с открытым; в окне игрока id 0.
                    let open = self.inv.open.as_ref().map_or(0, |o| o.id);
                    if open == c {
                        let grid = if self.inv.open.is_some() { 3 } else { 2 };
                        let fuel = self
                            .inv
                            .open
                            .as_ref()
                            .and_then(|o| o.slots.get(1))
                            .is_some_and(|s| s.is_some());
                        self.ghost = g.slots(grid, fuel);
                    }
                }
                NetEvt::AdvTab(tab) => self.advancements.select(tab),
                NetEvt::Stats(u) => self.stats.apply(u),
                NetEvt::Dialog(d) => self.dialog_in.push(d),
                NetEvt::OpenSign(p, f) => self.sign_in.push((p, f)),
                NetEvt::Suggestions(id, s, l, v) => self.sugg_in.push((id, s, l, v)),
                NetEvt::Inventory(iev) => self.inv.apply(iev),
                NetEvt::Abilities { flags, fly_speed } => {
                    // VANILLA: ClientPacketListener#handlePlayerAbilities.
                    self.invulnerable = flags & 1 != 0;
                    self.player.mayfly = flags & 4 != 0;
                    self.player.flying = flags & 2 != 0;
                    self.player.fly_speed = fly_speed;
                }
                NetEvt::OpLevel(l) => self.op_level = l,
                NetEvt::Explosion(ex) => {
                    // VANILLA: ClientPacketListener#handleExplosion.
                    let c = ex.center;
                    match ex.particle {
                        "explosion_emitter" => {
                            self.particles.add_explosion_emitter(c[0], c[1], c[2])
                        }
                        _ => self.particles.add_huge(c[0], c[1], c[2], 1.0),
                    }
                    self.particles
                        .track_explosion(crate::particles::ExplosionInfo {
                            center: c,
                            radius: ex.radius,
                            block_count: ex.block_count,
                            particles: ex.block_particles,
                        });
                    if let Some(k) = ex.knockback {
                        // VANILLA: LocalPlayer#pushFromExplosion = addDeltaMovement.
                        for (v, kk) in self.player.vel.iter_mut().zip(k) {
                            *v += kk;
                        }
                    }
                }
                NetEvt::Ping(b) => {
                    let _ = self.net.cmd.send(crate::netthread::NetCmd::Pong(b));
                }
                NetEvt::Motion(v) => {
                    // VANILLA: ClientPacketListener#handleSetEntityMotion → Entity#lerpMotion.
                    self.player.vel = v;
                }
                NetEvt::Teleport {
                    pos,
                    vel,
                    yaw,
                    pitch,
                    gen,
                    id,
                    state,
                } => {
                    self.tp_gen_applied = gen;
                    let _ = self
                        .net
                        .cmd
                        .send(crate::netthread::NetCmd::AcceptTeleport(id, state));
                    // VANILLA: ClientPacketListener#handleMovePlayer — позиция сервера всегда применяется
                    // (раньше только первая: после /tp камера оставалась на старом месте и сервер
                    // перестал слать чанки).
                    tracing::debug!(
                        target: "dbg",
                        "server_teleport from={:.3?} to={:.3?} vel={:.3?} ground={} yaw={:.1} pitch={:.1}",
                        self.player.pos,
                        pos,
                        vel,
                        self.player.on_ground,
                        yaw,
                        pitch
                    );
                    // Отладка: что клиент видит в клетках под AABB до отката (сервер считал их твёрдыми).
                    if tracing::enabled!(target: "dbg", tracing::Level::DEBUG) {
                        let a = self.player.aabb();
                        let mut cells = Vec::new();
                        if let Ok(g) = self.net.level.lock() {
                            if let Some(l) = g.as_ref() {
                                for x in a[0].floor() as i32..=(a[3] - 1e-6).floor() as i32 {
                                    for y in a[1].floor() as i32..=(a[4] - 1e-6).floor() as i32 {
                                        for z in a[2].floor() as i32..=(a[5] - 1e-6).floor() as i32
                                        {
                                            let loaded = l.chunks.get(x >> 4, z >> 4).is_some();
                                            cells.push(format!(
                                                "({x},{y},{z})={}{}",
                                                l.chunks.block(x, y, z).name(),
                                                if loaded { "" } else { "[NO_CHUNK]" }
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        tracing::debug!(target: "dbg", "teleport_cells {}", cells.join(" "));
                    }
                    self.player.teleport(pos);
                    self.player.vel = vel;
                    self.cam = [pos[0], pos[1] + self.player.eye_height(), pos[2]];
                    self.yaw = yaw;
                    self.pitch = pitch;
                    self.have_pos = true;
                    // То же значение, что сетевой поток записал при телепорте (флаги = 0).
                    self.last_pos_sent = Some([
                        pos[0],
                        pos[1],
                        pos[2],
                        f64::from(yaw),
                        f64::from(pitch),
                        0.0,
                    ]);
                }
                NetEvt::Disconnected(why) => {
                    tracing::error!("отключён: {why}");
                    self.status = Some(why);
                }
                NetEvt::Level(l) => match l {
                    LevelEvent::ChunkLoaded { x, z } | LevelEvent::LightUpdated { x, z } => {
                        if matches!(l, LevelEvent::ChunkLoaded { .. }) {
                            self.respawn_wait = false;
                        }
                        self.mark_column_area(x, z);
                    }
                    LevelEvent::BlockEntityChanged(p) => self.be_changed.push([p.x, p.y, p.z]),
                    LevelEvent::ChunkDropped { x, z } => {
                        self.drop_column(x, z);
                        for (dx, dz) in [
                            (-1, 0),
                            (1, 0),
                            (0, -1),
                            (0, 1),
                            (-1, -1),
                            (1, 1),
                            (-1, 1),
                            (1, -1),
                        ] {
                            self.mark_column(x + dx, z + dz);
                        }
                    }
                    LevelEvent::BlocksChanged(v) => {
                        for p in v {
                            self.queue.mark_block(p.x, p.y, p.z);
                        }
                    }
                    LevelEvent::Effect {
                        ty: 2001,
                        pos,
                        data,
                    } => {
                        // VANILLA: LevelEvent.PARTICLES_DESTROY_BLOCK (+ breakSound)
                        if let Some(st) = BlockState::from_id(data as u32) {
                            crate::blocksound::brk(st.name(), pos.x, pos.y, pos.z);
                        }
                        if let (Ok(g), Some(st)) =
                            (self.net.level.lock(), BlockState::from_id(data as u32))
                        {
                            if let Some(level) = g.as_ref() {
                                let models = &self.models;
                                self.particles.destroy_block(pos, st, &mut |s, p| {
                                    Self::terrain_look(models, level, s, p)
                                });
                            }
                        }
                    }
                    LevelEvent::Effect {
                        ty: ty @ (2014 | 3008),
                        pos,
                        data,
                    } => {
                        // VANILLA: LevelEventHandler 2014 (addDestroyBlockEffect) / 3008 (+ звук кисти).
                        if let Some(st) = BlockState::from_id(data as u32) {
                            if ty == 3008 {
                                let ev = match st.name() {
                                    "suspicious_sand" => Some("item.brush.brushing.sand.complete"),
                                    "suspicious_gravel" => {
                                        Some("item.brush.brushing.gravel.complete")
                                    }
                                    _ => None,
                                };
                                if let Some(ev) = ev {
                                    crate::audio::play_at(
                                        ev,
                                        crate::audio::Cat::Players,
                                        f64::from(pos.x) + 0.5,
                                        f64::from(pos.y) + 0.5,
                                        f64::from(pos.z) + 0.5,
                                        1.0,
                                        1.0,
                                    );
                                }
                            }
                            if let Ok(g) = self.net.level.lock() {
                                if let Some(level) = g.as_ref() {
                                    let models = &self.models;
                                    self.particles.destroy_block(pos, st, &mut |s, p| {
                                        Self::terrain_look(models, level, s, p)
                                    });
                                }
                            }
                        }
                    }
                    LevelEvent::Effect {
                        ty: 2013,
                        pos,
                        data,
                    } => {
                        // VANILLA: LevelEventHandler 2013 → ParticleUtils#spawnSmashAttackParticles.
                        if let Ok(g) = self.net.level.lock() {
                            if let Some(level) = g.as_ref() {
                                let st = level.chunks.block(pos.x, pos.y, pos.z);
                                let models = &self.models;
                                self.particles.smash(pos, st, data, &mut |s, p| {
                                    Self::terrain_look(models, level, s, p)
                                });
                            }
                        }
                    }
                    LevelEvent::Effect {
                        ty: 3006,
                        pos,
                        data,
                    } if data >> 6 == 0 => {
                        // VANILLA: LevelEventHandler 3006 (count == 0): звук + sculk_charge_pop.
                        crate::audio::play_at(
                            "block.sculk.charge",
                            crate::audio::Cat::Blocks,
                            f64::from(pos.x) + 0.5,
                            f64::from(pos.y) + 0.5,
                            f64::from(pos.z) + 0.5,
                            1.0,
                            1.0,
                        );
                        let solid = match self.net.level.lock() {
                            Ok(g) => g.as_ref().is_some_and(|l| {
                                l.chunks
                                    .block(pos.x, pos.y, pos.z)
                                    .props()
                                    .is_some_and(|p| p.has(ezzmc_world::block::FLAG_CAN_OCCLUDE))
                            }),
                            Err(_) => false,
                        };
                        self.particles.sculk_pop([pos.x, pos.y, pos.z], solid);
                    }
                    LevelEvent::Effect {
                        ty: 3006,
                        pos,
                        data,
                    } => {
                        self.particles.sculk_charge([pos.x, pos.y, pos.z], data);
                    }
                    LevelEvent::Effect { ty: 3007, pos, .. } => {
                        // VANILLA: LevelEventHandler 3007 (SculkShriekerBlock.TOP_Y = 0.5).
                        let (x, y, z) = (
                            f64::from(pos.x) + 0.5,
                            f64::from(pos.y) + 0.5,
                            f64::from(pos.z) + 0.5,
                        );
                        for i in 0..10 {
                            self.particles.add_shriek(x, y, z, i * 5);
                        }
                        let waterlogged = self.net.level.lock().ok().is_some_and(|g| {
                            g.as_ref().is_some_and(|l| {
                                l.chunks.block(pos.x, pos.y, pos.z).property("waterlogged")
                                    == Some("true")
                            })
                        });
                        if !waterlogged {
                            let pitch = 0.6 + self.particles.rand3().0 as f32 * 0.4;
                            crate::audio::play_at(
                                "block.sculk_shrieker.shriek",
                                crate::audio::Cat::Blocks,
                                x,
                                y,
                                z,
                                2.0,
                                pitch,
                            );
                        }
                    }
                    LevelEvent::Effect { ty: 1504, pos, .. } => {
                        // VANILLA: LevelEventHandler 1504 → PointedDripstoneBlock#spawnDripParticle.
                        if let Ok(g) = self.net.level.lock() {
                            if let Some(level) = g.as_ref() {
                                self.particles.stalactite_drip(level, [pos.x, pos.y, pos.z]);
                            }
                        }
                    }
                    LevelEvent::Effect {
                        ty: ty @ (2019 | 2020),
                        pos,
                        data,
                    } => {
                        // VANILLA: LevelEventHandler 2019/2020 → ClientLevel#addBreakingBlockEffects.
                        if let Ok(g) = self.net.level.lock() {
                            if let Some(level) = g.as_ref() {
                                let st = level.chunks.block(pos.x, pos.y, pos.z);
                                if !st.is_air() {
                                    if ty == 2020 {
                                        crate::blocksound::brk(st.name(), pos.x, pos.y, pos.z);
                                    }
                                    let models = &self.models;
                                    self.particles.crack(
                                        pos,
                                        st,
                                        data.clamp(0, 5) as u8,
                                        &mut |s, p| Self::terrain_look(models, level, s, p),
                                    );
                                }
                            }
                        }
                    }
                    LevelEvent::GlobalEffect { ty, pos } => {
                        crate::levelsound::play_global(ty, [pos.x, pos.y, pos.z], self.cam);
                    }
                    LevelEvent::Effect { ty, pos, data } if ty != 2001 => {
                        self.particles.level_event(ty, [pos.x, pos.y, pos.z], data);
                        // VANILLA: LevelEventHandler#playJukeboxSong / stopJukeboxSongAndNotifyNearby.
                        if ty == 1010 {
                            self.now_playing = Some(data);
                        }
                        if ty == 1010 || ty == 1011 {
                            self.mobs.record_event([pos.x, pos.y, pos.z], ty == 1010);
                        }
                        crate::levelsound::play(ty, [pos.x, pos.y, pos.z], data);
                    }
                    LevelEvent::BlockEvent { pos, b0, b1, .. } => {
                        // VANILLA: ChestBlockEntity#triggerEvent (крышка сундука).
                        self.chests.block_event(
                            [pos.x, pos.y, pos.z],
                            i32::from(b0),
                            i32::from(b1),
                        );
                        // VANILLA: ShulkerBoxBlockEntity#triggerEvent (крышка шалкер-коробки).
                        self.shulker_boxes.block_event(
                            [pos.x, pos.y, pos.z],
                            i32::from(b0),
                            i32::from(b1),
                        );
                        self.beacons
                            .block_event([pos.x, pos.y, pos.z], i32::from(b0));
                        // VANILLA: BellBlockEntity#triggerEvent (качание колокола).
                        self.bells
                            .block_event([pos.x, pos.y, pos.z], i32::from(b0), i32::from(b1));
                        // VANILLA: PistonBaseBlock#triggerEvent (клиентская ветка).
                        let ch = match self.net.level.lock().as_deref_mut() {
                            Ok(Some(l)) => Some(l.piston_event(pos, b0, b1)),
                            _ => None,
                        };
                        if let Some(ch) = ch {
                            self.apply_piston(ch);
                        }
                        // VANILLA: NoteBlock#triggerEvent — частица ноты (звук приходит пакетом).
                        if let Ok(g) = self.net.level.lock() {
                            if let Some(level) = g.as_ref() {
                                let st = level.chunks.block(pos.x, pos.y, pos.z);
                                if st.name() == "note_block" {
                                    let tunable = !matches!(
                                        st.property("instrument"),
                                        Some(
                                            "zombie"
                                                | "skeleton"
                                                | "creeper"
                                                | "dragon"
                                                | "wither_skeleton"
                                                | "piglin"
                                                | "custom_head"
                                        )
                                    );
                                    if tunable {
                                        let note: i32 = st
                                            .property("note")
                                            .and_then(|n| n.parse().ok())
                                            .unwrap_or(0);
                                        self.particles.add_note(
                                            f64::from(pos.x) + 0.5,
                                            f64::from(pos.y) + 1.2,
                                            f64::from(pos.z) + 0.5,
                                            f64::from(note) / 24.0,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    LevelEvent::Particles(pk) => {
                        if let Ok(g) = self.net.level.lock() {
                            if let Some(level) = g.as_ref() {
                                let models = &self.models;
                                self.particles.handle_packet(&pk, &mut |s, p| {
                                    Self::terrain_look(models, level, s, p)
                                });
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
    }

    /// Спрайт и тинт осколка блока (VANILLA: TerrainParticle#<init>).
    fn terrain_look(
        models: &BlockModels,
        level: &Level,
        st: BlockState,
        pos: ezzmc_protocol::types::BlockPos,
    ) -> Option<crate::particles::TerrainLook> {
        let sprite = models.particle_sprite(st)?;
        let (cx, cz) = (pos.x >> 4, pos.z >> 4);
        let hood = Self::hood(level, cx, cz);
        let base = SectionPos {
            x: cx,
            y: pos.y >> 4,
            z: cz,
        };
        let c = models.particle_tint(st, &hood, base, pos.x, pos.y, pos.z);
        let ch = |s: u32| ((c >> s) & 255) as f32 / 255.0;
        Some(crate::particles::TerrainLook {
            sprite,
            tint: [ch(16), ch(8), ch(0)],
        })
    }

    /// Для MCP: (живые частицы, активные FireworkParticles.Starter).
    pub fn particle_stats(&self) -> (usize, usize, usize, usize) {
        (
            self.particles.count(),
            self.particles.starter_count(),
            self.particles.vault_connection_count(),
            self.particles.vault_count(),
        )
    }

    /// Время мира для шейдеров (GameTime): `(game_time, partial_tick)`.
    pub fn level_time(&self) -> (i64, f32) {
        match self.net.level.lock().as_deref() {
            Ok(Some(l)) => (l.game_time, l.partial_tick()),
            _ => (0, 0.0),
        }
    }

    /// Клиентские тики частиц по росту `game_time` (VANILLA: ParticleEngine#tick + tickWeatherEffects).
    /// VANILLA: Minecraft#getSituationalMusic: босс Энда (Musics.END_BOSS), иначе BACKGROUND_MUSIC
    /// в позиции камеры.
    pub fn situational_music(&self) -> Option<crate::music::Music> {
        let g = self.net.level.lock().ok()?;
        let level = g.as_ref()?;
        // VANILLA-UNSURE: dimension() == Level.END; ключ измерения не хранится → по skybox=end.
        if level.skybox == ezzmc_world::dimension::Skybox::End && self.boss.play_music() {
            return Some(crate::music::Music {
                sound: "music.dragon".into(),
                min_delay: 0,
                max_delay: 0,
                replace: true,
            });
        }
        let a = crate::ambient::Ambient::attrs_at(level, &self.models.tints.biomes, self.cam);
        let bg = a.music.as_ref().or(level.audio.music.as_ref())?;
        bg.select(self.game_mode == 1, self.player.under_water)
            .cloned()
    }

    /// Перерисовать изменённые клетки и показать разрушение блоков, раздавленных поршнем.
    fn apply_piston(&mut self, ch: ezzmc_world::piston::PistonChanges) {
        for p in &ch.changed {
            self.queue.mark_block(p.x, p.y, p.z);
        }
        if ch.destroyed.is_empty() {
            return;
        }
        if let Ok(g) = self.net.level.lock() {
            if let Some(level) = g.as_ref() {
                for (pos, st) in ch.destroyed {
                    // VANILLA: PistonBaseBlock#moveBlocks — levelEvent(2001) на клиенте.
                    crate::blocksound::brk(st.name(), pos.x, pos.y, pos.z);
                    let models = &self.models;
                    self.particles.destroy_block(pos, st, &mut |s, p| {
                        Self::terrain_look(models, level, s, p)
                    });
                }
            }
        }
    }

    /// VANILLA: PistonMovingBlockEntity#tick — по клиентским тикам (рост game_time).
    fn tick_pistons(&mut self) {
        let mut all = ezzmc_world::piston::PistonChanges::default();
        if let Ok(mut g) = self.net.level.lock() {
            let Some(level) = g.as_mut() else { return };
            let now = level.game_time;
            let n = match self.last_piston_tick.replace(now) {
                Some(prev) => (now - prev).clamp(0, 10),
                None => 0,
            };
            if level.pistons.is_empty() {
                return;
            }
            for _ in 0..n {
                crate::pistonview::push_player(level, &mut self.player);
                let c = level.tick_pistons();
                all.changed.extend(c.changed);
                all.destroyed.extend(c.destroyed);
            }
        }
        self.apply_piston(all);
    }

    fn tick_particles(&mut self) {
        let Ok(mut g) = self.net.level.lock() else {
            return;
        };
        let Some(level) = g.as_mut() else { return };
        // VANILLA: LightningBolt#tick (клиент, life==2) — гром и удар, дальность 10000.
        for p in std::mem::take(&mut level.thunder) {
            crate::levelsound::thunder(p);
        }
        // VANILLA: ThrownEnderpearl#onHit — 32 частицы portal в точке попадания.
        for p in std::mem::take(&mut level.pearl_hits) {
            self.particles.pearl_hit(p, self.cam);
        }
        // VANILLA: LivingEntity#handleDamageEvent/Player#animateHurt → hurtTime = 10 (тряска камеры).
        for dir in std::mem::take(&mut level.own_hurts) {
            self.player.hurt(dir);
        }
        // VANILLA: LivingEntity#handleDamageEvent / handleEntityEvent(3) → playSound(hurt/death, volume, ±0.2).
        for (ty, p, death) in std::mem::take(&mut level.mob_sounds) {
            let Some(name) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(ty) else {
                continue;
            };
            if let Some((ev, cat, vol)) = crate::levelsound::mob_hurt_sound(name, death) {
                let pitch = (crate::levelsound::rf() - crate::levelsound::rf()) * 0.2 + 1.0;
                crate::audio::play_at(&ev, cat, p[0], p[1], p[2], vol, pitch);
            }
        }
        for (p, w, h) in std::mem::take(&mut level.death_poofs) {
            self.particles.death_poof(p, w, h);
        }
        // VANILLA: FireworkRocketEntity#handleEntityEvent(17) → getExplosions() → createFireworks.
        for ev in std::mem::take(&mut level.firework_explosions) {
            let explosions = ev
                .item
                .as_ref()
                .and_then(|st| {
                    let id = ezzmc_protocol::generated::r26_3::DATA_COMPONENT_TYPE
                        .iter()
                        .position(|x| *x == "fireworks")? as u32;
                    st.patch.added.iter().find(|(i, _)| *i == id)
                })
                .map(|(_, b)| crate::particles::parse_fireworks(b))
                .unwrap_or_default();
            self.particles
                .create_fireworks(ev.pos, ev.vel, explosions, ev.play_sound);
        }
        let level: &Level = level;
        let now = level.game_time;
        crate::itemdyn::update(
            [self.cam[0], self.cam[1] - 1.62, self.cam[2]],
            self.yaw,
            level.day_time,
            now,
        );
        let n = match self.last_particle_tick.replace(now) {
            Some(prev) => (now - prev).clamp(0, 10),
            None => 0,
        };
        if n > 0 {
            self.vault_scan.update(
                level,
                self.cam,
                (self.queue.version, self.queue.block_version),
                is_vault,
            );
            let vaults = self
                .vault_scan
                .list
                .iter()
                .map(|&(p, st)| {
                    crate::particles::VaultInfo::from_block(
                        p,
                        st.property("facing").unwrap_or("north"),
                        st.property("ominous") == Some("true"),
                        crate::berscan::block_entity_tag(level, p),
                    )
                })
                .collect::<Vec<_>>();
            let players = if vaults.iter().any(|v| !v.connected.is_empty()) {
                // Level#getPlayerByUUID: свой игрок + чужие игроки из сущностей.
                let a = self.player.aabb_pub();
                let mut ps = vec![(level.own_uuid, self.player.pos, a[4] - a[1])];
                let pt = ezzmc_world::entities::player_type();
                ps.extend(
                    level
                        .entities
                        .iter()
                        .filter(|e| Some(e.ty) == pt)
                        .filter_map(|e| {
                            let (lo, hi) = e.hitbox()?;
                            Some((e.uuid, e.pos, hi[1] - lo[1]))
                        }),
                );
                ps
            } else {
                Vec::new()
            };
            self.particles.set_vaults(vaults, players);
        }
        for _ in 0..n {
            // VANILLA: Minecraft#tick → level.animateTick(camera block) каждый клиентский тик.
            self.particles.animate_tick(
                level,
                self.cam[0].floor() as i32,
                self.cam[1].floor() as i32,
                self.cam[2].floor() as i32,
            );
            self.particles.tick(level, self.cam);
            // VANILLA: VaultBlockEntity.Client#tick — для каждого хранилища рядом.
            self.particles.vault_tick(level.game_time);
            // VANILLA: FireworkRocketEntity#tick (клиент) — искра FIREWORK каждый тик
            // (life % 2 < 2 всегда истинно).
            for e in level.entities.iter().filter(|e| {
                ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty) == Some(&"firework_rocket")
            }) {
                let xa = self.particles.gaussian() * 0.05;
                let za = self.particles.gaussian() * 0.05;
                self.particles
                    .add_firework(e.pos[0], e.pos[1], e.pos[2], xa, -e.vel[1] * 0.5, za);
            }
            // VANILLA: AbstractHurtingProjectile#createParticleTrail — DragonFireball#getTrailParticle
            // = DRAGON_BREATH(power 1) в (x, y + 0.5, z) с нулевой скоростью, после шага.
            for e in level.entities.iter().filter(|e| e.proj == 4) {
                self.particles
                    .add_dragon_breath(e.pos[0], e.pos[1] + 0.5, e.pos[2], 0.0, 0.0, 0.0);
            }
            // VANILLA: AreaEffectCloud#clientTick — частицы по кругу радиуса (waiting: 2 шт. в 0.2,
            // через раз); у не-waiting скорость ((0.5 − r)·0.15, 0.01, (0.5 − r)·0.15).
            // VANILLA-UNSURE: поддержан только DRAGON_BREATH; ENTITY_EFFECT (зелья) ещё не портирован.
            for e in level
                .entities
                .iter()
                .filter(|e| e.cloud_particle == Some("dragon_breath"))
            {
                let (waiting, radius) = (e.cloud_waiting, e.cloud_radius);
                if waiting && self.particles.rand_f32() < 0.5 {
                    continue;
                }
                let (count, pr) = if waiting {
                    (2, 0.2_f32)
                } else {
                    (
                        (std::f32::consts::PI * radius * radius).ceil() as i32,
                        radius,
                    )
                };
                for _ in 0..count {
                    let angle = self.particles.rand_f32() * std::f32::consts::TAU;
                    let dist = self.particles.rand_f32().sqrt() * pr;
                    let x = e.pos[0] + f64::from(angle.cos() * dist);
                    let z = e.pos[2] + f64::from(angle.sin() * dist);
                    let (xa, ya, za) = if waiting {
                        (0.0, 0.0, 0.0)
                    } else {
                        let (a, _, b) = self.particles.rand3();
                        ((0.5 - a) * 0.15, f64::from(0.01_f32), (0.5 - b) * 0.15)
                    };
                    self.particles.add_dragon_breath(x, e.pos[1], z, xa, ya, za);
                }
            }
            // VANILLA: AbstractArrow#tick — 4 частицы CRIT вдоль хода крит-стрелы.
            for e in level.entities.iter().filter(|e| e.crit_arrow()) {
                let m = [
                    e.pos[0] - e.prev_pos[0],
                    e.pos[1] - e.prev_pos[1],
                    e.pos[2] - e.prev_pos[2],
                ];
                for i in 0..4 {
                    let k = f64::from(i) / 4.0;
                    self.particles.add_crit(
                        e.prev_pos[0] + m[0] * k,
                        e.prev_pos[1] + m[1] * k,
                        e.prev_pos[2] + m[2] * k,
                        -m[0],
                        -m[1] + 0.2,
                        -m[2],
                    );
                }
            }
            // VANILLA: EyeOfEnder#tick/spawnParticles (клиент) — после шага: origin = pos − Δ·0.25;
            // в воде 4 BUBBLE со скоростью Δ, иначе PORTAL (±0.3 по x/z, y−0.5).
            // VANILLA-UNSURE: isInWater — по точке origin (у ванили — пересечение AABB с водой).
            for e in level.entities.iter().filter(|e| e.proj == 3) {
                let v = e.vel;
                let o = [
                    e.pos[0] - v[0] * 0.25,
                    e.pos[1] - v[1] * 0.25,
                    e.pos[2] - v[2] * 0.25,
                ];
                let in_water = ezzmc_mesh::blockinfo::fluid(level.chunks.block(
                    o[0].floor() as i32,
                    o[1].floor() as i32,
                    o[2].floor() as i32,
                ))
                .is_some_and(|f| !f.lava);
                if in_water {
                    for _ in 0..4 {
                        self.particles
                            .add_bubble(o[0], o[1], o[2], v[0], v[1], v[2]);
                    }
                } else {
                    let (rx, _, rz) = self.particles.rand3();
                    self.particles.add_portal(
                        o[0] + rx * 0.6 - 0.3,
                        o[1] - 0.5,
                        o[2] + rz * 0.6 - 0.3,
                        v[0],
                        v[1],
                        v[2],
                    );
                }
            }
            // VANILLA: PrimedTnt#tick — клиентский дым SMOKE на y+0.5 каждый тик.
            for e in level.entities.iter().filter(|e| e.fuse > 0) {
                self.particles
                    .add_smoke(e.pos[0], e.pos[1] + 0.5, e.pos[2], 0.0, 0.0, 0.0);
            }
            for e in level.entities.iter() {
                let Some(n) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty) else {
                    continue;
                };
                if *n == "glow_squid" {
                    let (w, h) = ezzmc_world::entity_size::size(n).unwrap_or((0.8, 0.8));
                    let r = &mut self.particles;
                    let (w, h) = (f64::from(w), f64::from(h));
                    let (rx, ry, rz) = r.rand3();
                    r.add_glow_squid(
                        e.pos[0] + w * (2.0 * rx - 1.0) * 0.6,
                        e.pos[1] + h * ry,
                        e.pos[2] + w * (2.0 * rz - 1.0) * 0.6,
                    );
                }
                if matches!(*n, "blaze" | "enderman" | "endermite") {
                    let (w, h) = ezzmc_world::entity_size::size(n).unwrap_or((0.6, 1.8));
                    self.particles
                        .mob_ambient(n, e.pos, f64::from(w), f64::from(h));
                }
            }
            Self::entity_splashes(&mut self.particles, &mut self.wet_entities, level);
            self.ambient
                .tick(level, &self.models.tints.biomes, self.cam);
            if level.skybox == ezzmc_world::dimension::Skybox::Overworld {
                self.particles.tick_weather(
                    level,
                    &self.models.tints.biomes,
                    self.cam,
                    crate::weather::WEATHER_RADIUS,
                );
            }
        }
    }

    fn hood(level: &Level, cx: i32, cz: i32) -> Neighborhood {
        let mut h = Neighborhood::default();
        for dz in -1..=1 {
            for dx in -1..=1 {
                h.chunks[((dz + 1) * 3 + dx + 1) as usize] =
                    level.chunks.get(cx + dx, cz + dz).cloned();
            }
        }
        h
    }

    /// Флаги для сетевого потока (`netthread::FLAG_*`): земля, столкновение, спринт, клавиши (player_input).
    fn net_flags(&self) -> u32 {
        use crate::netthread::{
            FLAG_FLYING, FLAG_HCOL, FLAG_INPUT_SHIFT, FLAG_ON_GROUND, FLAG_RIDING, FLAG_SPRINTING,
        };
        if !self.walking {
            return 0;
        }
        let p = &self.player;
        let i = self.last_input;
        let mut bits = 0u8;
        for (on, bit) in [
            (i.forward, 1u8),
            (i.back, 2),
            (i.left, 4),
            (i.right, 8),
            (i.jump, 16),
            (i.sneak, 32),
            (i.sprint, 64),
        ] {
            if on {
                bits |= bit;
            }
        }
        (u32::from(p.on_ground) * FLAG_ON_GROUND)
            | (u32::from(p.horizontal_collision) * FLAG_HCOL)
            | (u32::from(p.sprinting) * FLAG_SPRINTING)
            | (u32::from(p.flying) * FLAG_FLYING)
            | (u32::from(bits) << FLAG_INPUT_SHIFT)
            | (u32::from(self.riding) * FLAG_RIDING)
    }

    /// Раз в кадр: события сети, постановка в меш, заливка готовых мешей в арену.
    pub fn update(&mut self, gpu: &Gpu) {
        // VANILLA: TextureManager#tick → анимированные спрайты атласа блоков.
        self.atlas_anim.update(&gpu.queue, &self.atlas_tex);
        // Лок до разбора событий: см. телепорт в netthread.rs.
        let shared_pos = self.net.pos.clone();
        let mut guard = shared_pos.lock().ok();
        self.drain_events();
        // VANILLA: SoundEngine#updateSource — слушатель в глазах камеры.
        audio::set_listener(self.cam, self.yaw, self.pitch);
        if self.have_pos && !self.no_send {
            // Сетевой поток шлёт это серверу как позицию игрока (иначе новые чанки не приходят).
            if let Some(g) = guard.as_mut() {
                // Сетевой поток уже записал телепорт, а событие ещё не разобрано: не затираем его
                // старой позицией (иначе уходит «moved too quickly» и сервер откатывает игрока).
                let net_teleported = matches!(**g, Some(cur) if Some(cur) != self.last_pos_sent);
                if !net_teleported {
                    let feet = if self.walking {
                        self.player.pos
                    } else {
                        [self.cam[0], self.cam[1] - 1.62, self.cam[2]]
                    };
                    let sent = [
                        feet[0],
                        feet[1],
                        feet[2],
                        f64::from(self.yaw),
                        f64::from(self.pitch),
                        f64::from(self.net_flags()),
                    ];
                    **g = Some(sent);
                    self.last_pos_sent = Some(sent);
                }
            }
        }
        drop(guard);
        self.tick_pistons();
        self.tick_particles();
        let n = MAX_PENDING.saturating_sub(self.pool.pending());
        if n > 0 && !self.queue.is_empty() {
            let look = look_dir(self.yaw, self.pitch);
            let cam = [self.cam[0] as f32, self.cam[1] as f32, self.cam[2] as f32];
            let take = self.queue.take(n, cam, look);
            if let Ok(g) = self.net.level.lock() {
                if let Some(level) = g.as_ref() {
                    for pos in take {
                        if level.chunks.get(pos.x, pos.z).is_none() {
                            continue;
                        }
                        let hood = Self::hood(level, pos.x, pos.z);
                        let models: Arc<dyn ModelProvider> = self.models.clone();
                        // compute: конвертация секции в рабочем потоке, а не на рендер-потоке
                        let post: Option<ezzmc_mesh::pool::Post> = if self.use_compute {
                            let rects = self.rects.clone();
                            Some(Arc::new(move |m: &ezzmc_mesh::section::SectionMesh| {
                                Box::new(split_mesh(m, &rects)) as Box<dyn std::any::Any + Send>
                            }))
                        } else {
                            None
                        };
                        self.pool.submit(pos, hood, models, post);
                    }
                }
            }
        }
        let prof_poll = std::time::Instant::now();
        let mut prof_vox = std::time::Duration::ZERO;
        for r in self.pool.poll() {
            let pos3 = [r.pos.x, r.pos.y, r.pos.z];
            if r.mesh.is_empty() {
                if self.resident.remove(&r.pos) {
                    self.arena.remove(r.pos);
                }
                if self.vox_resident.remove(&r.pos) {
                    if let Some(v) = &mut self.vox {
                        v.remove_section(&gpu.queue, pos3);
                    }
                }
                continue;
            }
            // compute-путь: цельные кубы → Renderer, остаток (и прозрачное) → ванильная арена
            let mut rest = None;
            if let (true, Some(v)) = (self.use_compute, self.vox.as_mut()) {
                let prof_t = std::time::Instant::now();
                let (conv, left) = match r
                    .extra
                    .and_then(|e| e.downcast::<(Converted, SectionMesh)>().ok())
                {
                    Some(b) => *b,
                    None => split_mesh(&r.mesh, &self.rects),
                };
                if v.upload_section(&gpu.queue, pos3, &conv.verts, &conv.quads) {
                    if conv.quads.is_empty() {
                        self.vox_resident.remove(&r.pos);
                    } else {
                        self.vox_resident.insert(r.pos);
                    }
                    rest = Some(left);
                } else {
                    tracing::warn!("compute-рендер: нет места под секцию, ванильный путь");
                    v.remove_section(&gpu.queue, pos3);
                    self.vox_resident.remove(&r.pos);
                }
                prof_vox += prof_t.elapsed();
            }
            let mesh = rest.as_ref().unwrap_or(&r.mesh);
            if mesh.is_empty() {
                if self.resident.remove(&r.pos) {
                    self.arena.remove(r.pos);
                }
                continue;
            }
            match self.arena.upload(&gpu.queue, r.pos, mesh, self.cam) {
                Ok(()) => {
                    self.arena_retry.remove(&r.pos);
                    self.resident.insert(r.pos);
                }
                Err(e) => {
                    // Старая геометрия осталась на месте; при нехватке места пробуем перестроить позже.
                    let tries = self.arena_retry.entry(r.pos).or_insert(0);
                    if *tries < ARENA_RETRIES {
                        *tries += 1;
                        self.queue.mark(r.pos);
                    } else {
                        self.arena_retry.remove(&r.pos);
                        let (used, cap, big, blocks) = self.arena.layout().vertex_stats();
                        tracing::warn!(
                            "арена: {e}; секция {:?} осталась со старой геометрией \
                             (занято {used}/{cap}, крупнейший свободный блок {big}, блоков {blocks}, секций {})",
                            r.pos,
                            self.arena.layout().len()
                        );
                    }
                }
            }
        }
        crate::mobs::prof_phase(0, "poll: всего", prof_poll.elapsed());
        crate::mobs::prof_phase(20, "poll: vox split+upload", prof_vox);
        self.sync_atlas(gpu);
        self.update_targets();
    }

    /// Свободный полёт. `axes` = (вперёд, вбок вправо, вверх) ∈ {−1,0,1}.
    /// Ходьба: фиксированные тики 20/с (`physics::Player`), камера — интерполяция между тиками.
    /// VANILLA: LocalPlayer#aiStep по тику клиента, Camera с partialTick.
    pub fn walk(&mut self, dt: f64, input: crate::physics::Input) {
        if !self.have_pos {
            return;
        }
        self.last_input = input;
        // VANILLA: Minecraft#runTick, не более 10 тиков за кадр (0.5 с) — иначе при хитче теряются тики езды.
        self.tick_acc += dt.min(0.5);
        let (mut yaw, pitch) = (self.yaw, self.pitch);
        self.player.pitch = pitch;
        let mut n = 0;
        while self.tick_acc >= 0.05 && n < 10 {
            self.tick_acc -= 0.05;
            n += 1;
            // VANILLA: LivingEntity#canGlide → canGlideUsing(CHEST): глайдер в нагрудном слоте.
            let glider = self
                .inv
                .slots
                .get(crate::inventory::ARMOR_START + 1)
                .and_then(|s| s.as_ref())
                .is_some_and(|s| {
                    crate::inventory::item_name(s.item)
                        .is_some_and(|n| crate::tooltip::can_glide_chest(&n, &s.patch))
                });
            let flags_seen = &mut self.own_flags_seen;
            let mut fall_fly_req = false;
            let p = &mut self.player;
            let mut self_fov = None;
            let mut riding_now = false;
            let mut mount_yaw: Option<f32> = None;
            let mut boat_yaw_now: Option<f32> = None;
            let was_riding = self.riding;
            let mut yaw_fix: Option<(f32, f32)> = None;
            let ran = self.net.level.lock().ok().and_then(|mut g| {
                g.as_mut().map(|l| {
                    // VANILLA: AbstractBoat#tick — лодка, где игрок первый пассажир, считается у нас.
                    let own = l.own_id;
                    let is_boat = |l: &ezzmc_world::level::Level, v: i32| {
                        l.entities.map.get(&v).is_some_and(|e| {
                            let n = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                                .get(e.ty)
                                .copied()
                                .unwrap_or("");
                            n.ends_with("_boat") || n.ends_with("_raft")
                        })
                    };
                    let ctl = l
                        .riders
                        .vehicle_of
                        .get(&own)
                        .copied()
                        .filter(|v| {
                            l.riders.passengers.get(v).and_then(|p| p.first()) == Some(&own)
                        })
                        .filter(|v| is_boat(l, *v));
                    if let Some((old, _)) = &self.boat {
                        if Some(*old) != ctl {
                            if let Some(e) = l.entities.map.get_mut(old) {
                                e.local_ctrl = false;
                                e.vel = [0.0; 3];
                            }
                            self.boat = None;
                            self.boat_out = None;
                        }
                    }
                    if let Some(v) = ctl {
                        if self.boat.is_none() {
                            if let Some(e) = l.entities.map.get_mut(&v) {
                                e.local_ctrl = true;
                                e.vel = [0.0; 3];
                            }
                            self.boat = Some((v, crate::boat::BoatSim::default()));
                        }
                        let sim = &mut self.boat.as_mut().expect("boat").1;
                        // VANILLA: commonTick → interpolate() до движения лодки.
                        if let Some(e) = l.entities.map.get_mut(&v) {
                            e.interpolate_step();
                        }
                        let body = l.entities.map.get(&v).map(|e| {
                            let n = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                                .get(e.ty)
                                .copied()
                                .unwrap_or("");
                            let (w, h) =
                                ezzmc_world::entity_size::size(n).unwrap_or((1.375, 0.5625));
                            crate::boat::BoatBody {
                                pos: e.pos,
                                vel: e.vel,
                                yaw: e.yaw,
                                on_ground: e.on_ground,
                                no_gravity: e.no_gravity,
                                width: w,
                                height: h,
                            }
                        });
                        if let Some(mut body) = body {
                            let old = (body.pos, body.yaw);
                            let level: &ezzmc_world::level::Level = l;
                            let paddle = sim.tick(&mut body, level, &|bb| {
                                let a = &crate::physics::fluid_accumulate(level, bb)[0];
                                (a.height, a.cur, a.n)
                            });
                            if let Some(e) = l.entities.map.get_mut(&v) {
                                e.prev_pos = old.0;
                                e.prev_yaw = old.1;
                                e.pos = body.pos;
                                e.yaw = body.yaw;
                                e.snap_target(body.pos);
                                e.vel = body.vel;
                                e.on_ground = body.on_ground;
                            }
                            // VANILLA: LocalPlayer#rideTick — ввод ставится после тика лодки.
                            sim.input = crate::boat::BoatInput {
                                left: input.left,
                                right: input.right,
                                up: input.forward,
                                down: input.back,
                            };
                            yaw_fix = Some((sim.delta_rotation, body.yaw));
                            self.boat_out = Some((paddle, body.pos, body.yaw, body.on_ground));
                        }
                    }
                    // VANILLA: LocalPlayer#rideTick — deltaMovement=0, позицию ставит транспорт.
                    let ride = ezzmc_world::entities::player_type()
                        .and_then(|t| l.ride_position(l.own_id, t));
                    if let Some(pos) = ride {
                        riding_now = true;
                        boat_yaw_now = l
                            .riders
                            .vehicle_of
                            .get(&l.own_id)
                            .and_then(|v| l.entities.map.get(v))
                            .filter(|v| {
                                let n = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                                    .get(v.ty)
                                    .copied()
                                    .unwrap_or("");
                                n.ends_with("_boat") || n.ends_with("_raft")
                            })
                            .map(|v| {
                                // VANILLA: рендер-yaw лодки по partialTick; чужая лодка (интерполяция
                                // сервера) иначе клэмпит камеру ступеньками 20 Гц.
                                if v.local_ctrl {
                                    v.yaw
                                } else {
                                    let d = (v.yaw - v.prev_yaw + 180.0).rem_euclid(360.0) - 180.0;
                                    v.prev_yaw + d * l.partial_tick()
                                }
                            });
                        if !was_riding {
                            // VANILLA: handleSetEntityPassengersPacket — на лодке взгляд по носу.
                            let vid = l.riders.vehicle_of.get(&l.own_id).copied().unwrap_or(-1);
                            if let Some(v) = l.entities.map.get(&vid) {
                                let name = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                                    .get(v.ty)
                                    .copied()
                                    .unwrap_or("");
                                if name.ends_with("_boat") || name.ends_with("_raft") {
                                    mount_yaw = Some(v.yaw);
                                }
                            }
                        }
                        p.prev_pos = p.pos;
                        p.pos = pos;
                        p.vel = [0.0; 3];
                        p.on_ground = false;
                        p.sprinting = false;
                        self_fov = Some(p.fov_target());
                    } else if crate::physics::Collider::has_chunk(
                        l,
                        p.pos[0].floor() as i32,
                        p.pos[2].floor() as i32,
                    ) {
                        let lvl = |n: &str| self.effects.get(n).map_or(0, |e| e.amplifier + 1);
                        p.fx = crate::physics::EffectLevels {
                            speed: lvl("speed"),
                            slowness: lvl("slowness"),
                            jump_boost: lvl("jump_boost"),
                            levitation: lvl("levitation"),
                            slow_falling: self.effects.has("slow_falling"),
                        };
                        let was_ground = p.on_ground;
                        p.anim.riding_body = None;
                        // VANILLA: SynchedEntityData — флаг 7 (FALL_FLYING) приходит от сервера.
                        if *flags_seen != l.own_flags_seq {
                            *flags_seen = l.own_flags_seq;
                            p.fall_flying = l.own_flags & 0x80 != 0;
                        }
                        p.glider = glider;
                        self.psound.before_tick(p);
                        p.tick(l, input, yaw);
                        fall_fly_req = p.fall_fly_request;
                        // VANILLA: FireworkRocketEntity#tick — ракеты, привязанные к нам (DATA_ATTACHED_TO_TARGET #9,
                        // OptionalInt: id+1), тикают после игрока и разгоняют его в полёте на элитрах.
                        let own = l.own_id;
                        let rockets = l
                            .entities
                            .iter()
                            .filter(|e| {
                                ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty)
                                    == Some(&"firework_rocket")
                                    && e.int_meta(9) == Some(own + 1)
                            })
                            .count();
                        for _ in 0..rockets {
                            p.firework_boost(yaw);
                        }
                        p.post_tick(yaw, pitch);
                        self.psound.after_tick(p, l, was_ground);
                        if let Some((pos, vel)) = self.psound.splash.take() {
                            // VANILLA: Entity#doWaterSplashEffect (игрок 0.6 шириной).
                            self.particles.water_splash_effect(pos, 0.6, vel);
                        }
                        if p.sprinting && !p.in_water {
                            // VANILLA: getOnPosLegacy — блок на 0.2 ниже ног.
                            let bp = ezzmc_protocol::types::BlockPos::new(
                                p.pos[0].floor() as i32,
                                (p.pos[1] - 0.2).floor() as i32,
                                p.pos[2].floor() as i32,
                            );
                            let st = l.chunks.block(bp.x, bp.y, bp.z);
                            let models = &self.models;
                            self.particles
                                .sprint(p.pos, p.vel, 0.6, st, bp, &mut |s, q| {
                                    Self::terrain_look(models, l, s, q)
                                });
                        }
                        self_fov = Some(p.fov_target());
                    }
                })
            });
            let _ = ran;
            if fall_fly_req {
                // VANILLA: LocalPlayer#aiStep — connection.send(START_FALL_FLYING).
                self.send(crate::netthread::NetCmd::StartFallFlying);
            }
            self.riding = riding_now;
            if let Some((dr, byaw)) = yaw_fix {
                // VANILLA: AbstractBoat#positionRider — yaw += deltaRotation, затем clampRotation.
                let before = self.yaw;
                self.yaw = crate::boat::clamp_passenger_yaw(self.yaw + dr, byaw);
                self.ride_dr = self.yaw - before;
                yaw = self.yaw;
            }
            self.ride_boat_yaw = boat_yaw_now;
            if boat_yaw_now.is_none() {
                self.ride_dr = 0.0;
            }
            if riding_now {
                // VANILLA: LocalPlayer#rideTick → Entity#tick: поворот головы/тела и взмах руки идут и в седле.
                if let Some(by) = boat_yaw_now {
                    self.yaw = crate::boat::clamp_passenger_yaw(self.yaw, by);
                    yaw = self.yaw;
                }
                self.player.anim.riding_body = boat_yaw_now;
                self.player.post_tick(yaw, pitch);
            }
            if riding_now && !was_riding {
                // VANILLA: handleSetEntityPassengersPacket — подсказка «Нажмите %s, чтобы слезть».
                use ezzmc_protocol::text::{Component, Content};
                let mut c = Component::text("");
                c.content = Content::Translatable {
                    key: "mount.onboard".into(),
                    fallback: None,
                    args: vec![Component {
                        content: Content::Keybind("key.sneak".into()),
                        style: Default::default(),
                        siblings: Vec::new(),
                    }],
                };
                self.chat_in.push((c, None));
            }
            if let Some(y) = mount_yaw {
                self.yaw = y;
                yaw = y;
            }
            if self_fov.is_some() {
                // VANILLA: LocalPlayer#tick — dropSpamThrottler.tick().
                self.inv.drop_spam = (self.inv.drop_spam - 1).max(0);
                // VANILLA: LivingEntity#tick attackStrengthTicker++; Player#tick — сброс при смене предмета.
                self.attack_ticker = self.attack_ticker.saturating_add(1);
                let cur = self.inv.held().map(|s| s.item);
                if cur != self.attack_item {
                    self.attack_item = cur;
                    self.attack_ticker = 0;
                }
                // VANILLA: Minecraft#tick — rightClickDelay/missTime уменьшаются каждый тик.
                self.right_click_delay = (self.right_click_delay - 1).max(0);
                self.miss_time = (self.miss_time - 1).max(0);
                self.effects.tick();
                // VANILLA: GameRenderer#tick — +0.05 за тик при darkenScreen, спад −0.0125.
                self.boss_darken_o = self.boss_darken;
                if self.boss.darken_screen() {
                    self.boss_darken = (self.boss_darken + 0.05).min(1.0);
                } else if self.boss_darken > 0.0 {
                    self.boss_darken -= 0.0125;
                }
                // VANILLA: MultiPlayerGameMode#tick → ensureHasSentCarriedItem: слот, выбранный сервером
                // (set_held_slot при входе), уходит как set_carried_item (иначе Grim считает слот 0).
                if self.inv.selected != self.carried_sent {
                    self.carried_sent = self.inv.selected;
                    let _ = self
                        .net
                        .cmd
                        .send(crate::netthread::NetCmd::SetCarried(i16::from(
                            self.inv.selected,
                        )));
                }
                self.tick_sleep();
                self.tick_using();
                crate::cooldown::tick();
                self.tick_hand();
                self.tick_dig();
                // VANILLA: LocalPlayer#handlePortalTransitionEffect (Transition.CONFUSION у nether_portal).
                self.portal_eff.0 = self.portal_eff.1;
                let step = if self.player.inside_portal {
                    if self.portal_eff.1 == 0.0 {
                        // VANILLA: SoundEvents.PORTAL_TRIGGER, forLocalAmbience(volume 0.25).
                        crate::audio::play_ui(
                            "block.portal.trigger",
                            crate::audio::Cat::Ambient,
                            0.25,
                            crate::levelsound::rf() * 0.4 + 0.8,
                        );
                    }
                    0.0125
                } else if self.portal_eff.1 > 0.0 {
                    -0.05
                } else {
                    0.0
                };
                self.portal_eff.1 = (self.portal_eff.1 + step).clamp(0.0, 1.0);
                // VANILLA: LocalPlayer#tickSpinningEffect.
                let portal = self.portal_eff.1;
                let nausea = self.effects.blend_factor("nausea", 1.0);
                if portal > 0.0 || nausea > 0.0 {
                    self.spin.1 = (portal * 20.0 + nausea * 7.0) / (portal + nausea);
                    self.spin.0 += self.spin.1;
                } else {
                    self.spin.1 = 0.0;
                }
            }
            if let Some(t) = self_fov {
                // VANILLA: AbstractClientPlayer#getFieldOfViewModifier — lerp(fovEffectScale, 1, modifier);
                // от первого лица со зрительной трубой возвращает 0.1 напрямую.
                let t = if self.is_scoping() {
                    SPYGLASS_ZOOM_FOV_MODIFIER
                } else {
                    1.0 + (t - 1.0) * self.cfg.fov_effect
                };
                let (_, cur) = self.fov_mod;
                self.fov_mod = (cur, cur + (t - cur) * 0.5);
                self.update_place_target(yaw, pitch);
            }
            // VANILLA: LocalPlayer#sendChanges — один пакет движения на каждый тик физики.
            if self_fov.is_some() && self.have_pos && !self.no_send {
                self.send_vehicle_tick();
                self.send_phys_tick(yaw, pitch);
            }
        }
        if n == 5 {
            self.tick_acc = 0.0;
        }
        // Каждый кадр, а не только в тик: нос чужой лодки по partialTick (клэмп камеры без ступенек 20 Гц).
        if self.riding && self.ride_boat_yaw.is_some() {
            let smooth = self.net.level.lock().ok().and_then(|g| {
                let l = g.as_ref()?;
                let v = l
                    .riders
                    .vehicle_of
                    .get(&l.own_id)
                    .and_then(|v| l.entities.map.get(v))
                    .filter(|v| !v.local_ctrl)?;
                let d = (v.yaw - v.prev_yaw + 180.0).rem_euclid(360.0) - 180.0;
                Some(v.prev_yaw + d * l.partial_tick())
            });
            if let Some(by) = smooth {
                self.ride_boat_yaw = Some(by);
                self.yaw = crate::boat::clamp_passenger_yaw(self.yaw, by);
            }
        }
        self.cam = self.player.eye_lerp(self.tick_acc / 0.05);
        if self.sleeping && self.perspective == 0 {
            // VANILLA: Camera#setup — move(0, 0.15, 0) спящего.
            self.cam[1] += 0.15;
        }
        // VANILLA: Camera → getViewYRot(partialTick): поворот лодки за тик размазываем по кадрам.
        self.render_yaw_off = if self.riding && self.ride_boat_yaw.is_some() {
            self.ride_dr * (1.0 - (self.tick_acc / 0.05).clamp(0.0, 1.0) as f32)
        } else {
            0.0
        };
        self.yaw -= self.render_yaw_off;
        if let Ok(mut g) = self.net.level.lock() {
            if let Some(l) = g.as_mut() {
                l.local_ctrl_pt = (self.tick_acc / 0.05).clamp(0.0, 1.0) as f32;
            }
        }
    }

    /// VANILLA: Minecraft#tick → pick(1.0F) после тика: цель клика от глаз конца тика.
    /// Сервер проверяет луч от позиции последнего пакета движения, поэтому берём именно её,
    /// а не кадровую (интерполированную вперёд) камеру `self.cam`.
    fn update_place_target(&mut self, yaw: f32, pitch: f32) {
        self.place_target = None;
        if !self.have_pos || self.game_mode == 3 {
            return;
        }
        let eye = self.player.eye_lerp(1.0);
        let d = look_dir(yaw, pitch);
        let reach = self.block_reach();
        if let Ok(g) = self.net.level.lock() {
            if let Some(l) = g.as_ref() {
                self.place_target = crate::pick::raycast_t(l, eye, d, false, reach).map(|(t, _)| t);
            }
        }
    }

    /// Тик лодки для сетевого потока (paddle_boat перед player_input, move_vehicle после Rot).
    fn send_vehicle_tick(&mut self) {
        let gen = self.net.tp_gen.load(Ordering::SeqCst);
        if gen != self.tp_gen_applied {
            return;
        }
        if let Some((paddle, pos, yaw, on_ground)) = self.boat_out.take() {
            let _ = self.net.cmd.send(NetCmd::VehicleTick {
                paddle,
                pos,
                yaw,
                on_ground,
            });
        }
    }

    /// Отправляет сетевому потоку состояние после тика физики. Тики до применения телепорта не шлём.
    fn send_phys_tick(&self, yaw: f32, pitch: f32) {
        let gen = self.net.tp_gen.load(Ordering::SeqCst);
        if gen != self.tp_gen_applied {
            return;
        }
        let _ = self.net.cmd.send(NetCmd::PhysTick {
            pos: self.player.pos,
            yaw,
            pitch,
            flags: self.net_flags(),
            gen,
        });
    }

    pub fn set_cfg(&mut self, cfg: crate::settings::ViewCfg) {
        self.cfg = cfg;
        crate::armor::set_glint_speed(cfg.glint_speed);
        ezzmc_render::particledraw::set_glint_strength(cfg.glint_strength);
        self.particles.status = cfg.particles;
        self.player.sprint_window = cfg.sprint_window;
        self.player.auto_jump = cfg.auto_jump;
    }

    /// VANILLA: GameRenderer#bobHurt (всегда) · bobView (только при options.bobView).
    fn bob(&self, pt: f32) -> Mat4 {
        let hurt = self.player.hurt_matrix(pt, self.cfg.damage_tilt);
        let m = if self.cfg.bob_view {
            hurt * self.player.bob_matrix(pt)
        } else {
            hurt
        };
        // VANILLA: GameRenderer#renderLevel — вращающийся перекос проекции (Nausea × screenEffectScale²).
        let nausea = self.effects.blend_factor("nausea", pt);
        let k =
            nausea.max(self.portal_intensity(pt)) * self.cfg.screen_effect * self.cfg.screen_effect;
        if k > 0.0 {
            let skew = 5.0 / (k * k + 5.0) - k * 0.04;
            let skew = skew * skew;
            let axis = glam::Vec3::new(
                0.0,
                std::f32::consts::SQRT_2 / 2.0,
                std::f32::consts::SQRT_2 / 2.0,
            );
            let ang = (self.spin.0 + pt * self.spin.1).to_radians();
            m * Mat4::from_axis_angle(axis, ang)
                * Mat4::from_scale(glam::Vec3::new(1.0 / skew, 1.0, 1.0))
                * Mat4::from_axis_angle(axis, -ang)
        } else {
            m
        }
    }

    /// VANILLA: Mth.lerp(pt, oPortalEffectIntensity, portalEffectIntensity).
    pub fn portal_intensity(&self, pt: f32) -> f32 {
        self.portal_eff.0 + (self.portal_eff.1 - self.portal_eff.0) * pt
    }

    /// VANILLA: LevelExtractor — waterOverlay: глаза в воде, от первого лица, не спектатор → яркость в точке глаз.
    pub fn water_overlay(&self) -> Option<f32> {
        if !self.player.under_water || self.perspective != 0 || self.game_mode == 3 {
            return None;
        }
        let g = self.net.level.lock().ok()?;
        let l = g.as_ref()?;
        let (bl, sl) = crate::particles::light_at(l, self.cam[0], self.cam[1], self.cam[2]);
        // VANILLA-UNSURE: Lightmap.getBrightness без ambient измерения: v / (4 − 3v).
        let v = f32::from(bl.max(sl)) / 15.0;
        Some(v / (4.0 - 3.0 * v))
    }

    /// VANILLA: PlayerRenderState.isOnFire (Entity#isOnFire — бит 0 флагов, от первого лица, не спектатор, не спальня).
    /// Возвращает номер кадра анимации fire_1 (frametime 1).
    pub fn fire_overlay_frame(&self) -> Option<u32> {
        if self.perspective != 0 || self.game_mode == 3 {
            return None;
        }
        let g = self.net.level.lock().ok()?;
        let l = g.as_ref()?;
        (l.own_flags & 1 != 0).then_some(self.effects.ticks as u32)
    }

    /// VANILLA: LevelExtractor#getViewBlockingState — 8 точек вокруг глаз; блок «душит» (isSuffocating):
    /// по умолчанию blocksMotion + полный куб, исключения — листва, стекло, корни мангров, часть вечно «душащих».
    /// VANILLA-UNSURE: nearPlaneBB не проверяется; закрытые шалкеры/поршни не разобраны.
    /// Возвращает спрайт частицы блока и атлас.
    pub fn block_overlay(&self) -> Option<ezzmc_assets::bake::SpriteUv> {
        if self.game_mode == 3 {
            return None;
        }
        let g = self.net.level.lock().ok()?;
        let l = g.as_ref()?;
        let eye = self.player.pos[1] + self.player.eye_height();
        let half = f64::from(0.6f32 * 0.8);
        for i in 0..8 {
            let x = self.player.pos[0] + (f64::from(i & 1) - 0.5) * half;
            let y = eye + (f64::from((i >> 1) & 1) - 0.5) * 0.1;
            let z = self.player.pos[2] + (f64::from((i >> 2) & 1) - 0.5) * half;
            let (bx, by, bz) = (x.floor() as i32, y.floor() as i32, z.floor() as i32);
            let st = l.chunks.block(bx, by, bz);
            let name = st.name();
            let always = matches!(name, "farmland" | "soul_sand" | "dirt_path" | "mud");
            let never = name.ends_with("leaves")
                || name.ends_with("glass")
                || name == "mangrove_roots"
                || name == "moving_piston";
            let full = {
                use crate::physics::Collider;
                l.boxes(bx, by, bz) == [[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]]
            };
            if always || (!never && full) {
                if let Some(sp) = self.models.particle_sprite(st) {
                    return Some(sp);
                }
            }
        }
        None
    }

    /// Атлас блоков (RGBA) для экранных оверлеев.
    pub fn blocks_atlas(&self) -> &(Vec<u8>, u32, u32) {
        &self.preview_blocks
    }

    /// VANILLA: Hud#extractCameraOverlays — Equippable#cameraOverlay (тыква на голове, слот 5), от первого лица.
    /// VANILLA-UNSURE: компонент equippable не читается, захардкожен carved_pumpkin → misc/pumpkinblur.
    pub fn pumpkin_overlay(&self) -> bool {
        self.perspective == 0
            && self
                .inv
                .slots
                .get(5)
                .and_then(|s| s.as_ref())
                .and_then(|s| crate::inventory::item_name(s.item))
                .is_some_and(|n| n.trim_start_matches("minecraft:") == "carved_pumpkin")
    }

    /// VANILLA: Player#isScoping (+ первое лицо) — используется подзорная труба.
    pub fn is_scoping(&self) -> bool {
        self.perspective == 0
            && self.using.is_some()
            && self
                .inv
                .held()
                .and_then(|s| crate::inventory::item_name(s.item))
                .is_some_and(|n| n.trim_start_matches("minecraft:") == "spyglass")
    }

    /// VANILLA: Player#getPercentFrozen = min(ticksFrozen, 140) / 140 (0 — не рисуем).
    pub fn frozen_percent(&self) -> f32 {
        let Ok(g) = self.net.level.lock() else {
            return 0.0;
        };
        g.as_ref()
            .map_or(0.0, |l| l.own_frozen.clamp(0, 140) as f32 / 140.0)
    }

    /// То же с partialTick текущего кадра.
    /// VANILLA: Player#tick — sleepCounter растёт во сне (до 100), после пробуждения доходит до 110 и сбрасывается.
    fn tick_sleep(&mut self) {
        let sleeping = if let Ok(Some(l)) = self.net.level.lock().as_deref() {
            l.own_pose == POSE_SLEEPING
        } else {
            false
        };
        self.sleeping = sleeping;
        self.player.sleeping = sleeping;
        self.sleep_timer = next_sleep_timer(self.sleep_timer, sleeping);
    }

    /// VANILLA: Camera#setup — спящий: вид по ориентации кровати (`toYRot − 180`, pitch 0), глаза +0.15.
    /// Без кровати под sleeping_pos — yaw 0.
    pub fn sleep_camera(&self) -> Option<(f32, f32)> {
        if !self.sleeping || self.perspective != 0 {
            return None;
        }
        let g = self.net.level.lock().ok()?;
        let l = g.as_ref()?;
        let yaw = l
            .own_sleep_pos
            .map(|p| l.chunks.block(p[0], p[1], p[2]))
            .filter(|s| s.name().ends_with("_bed"))
            .and_then(|s| s.property("facing"))
            .map_or(0.0, |f| match f {
                "south" => -180.0,
                "west" => -90.0,
                "north" => 0.0,
                _ => 90.0,
            });
        Some((yaw, 0.0))
    }

    /// VANILLA: Player#getSleepTimer.
    pub fn sleep_timer(&self) -> i32 {
        self.sleep_timer
    }

    pub fn portal_intensity_now(&self) -> f32 {
        self.portal_intensity((self.tick_acc / 0.05).clamp(0.0, 1.0) as f32)
    }

    /// Номер кадра анимации спрайта nether_portal (frametime 2) по тикам клиента.
    pub fn portal_frame(&self) -> u32 {
        self.effects.ticks as u32 / 2
    }

    /// Сущности в поле зрения камеры (для MCP `visible`): проекция центра хитбокса по матрице последнего кадра.
    /// `x`/`y` — доля экрана 0..1 (0,0 — левый верх); `in_view` — центр внутри кадра и впереди камеры.
    pub fn visible_entities(&self, level: &Level) -> Vec<serde_json::Value> {
        let (vp, cam, _) = self.nt_view;
        let mut out = Vec::new();
        for e in level.entities.iter() {
            let Some(name) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty) else {
                continue;
            };
            let (bw, bh) = ezzmc_world::entity_size::size(name)
                .map_or((0.5, 0.5), |s| (f64::from(s.0), f64::from(s.1)));
            let h = bh * 0.5;
            // дистанция до ближайшей точки хитбокса (как ванильный reach), а не до центра
            let near = (0..3)
                .map(|i| {
                    let (lo, hi) = if i == 1 {
                        (e.pos[1], e.pos[1] + bh)
                    } else {
                        (e.pos[i] - bw / 2.0, e.pos[i] + bw / 2.0)
                    };
                    (cam[i].clamp(lo, hi) - cam[i]).powi(2)
                })
                .sum::<f64>()
                .sqrt();
            let d = [e.pos[0] - cam[0], e.pos[1] + h - cam[1], e.pos[2] - cam[2]];
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            let c = vp * glam::Vec4::new(d[0] as f32, d[1] as f32, d[2] as f32, 1.0);
            let (nx, ny) = if c.w > 0.0 {
                (c.x / c.w, c.y / c.w)
            } else {
                (f32::NAN, f32::NAN)
            };
            let in_view = c.w > 0.05 && nx.abs() <= 1.0 && ny.abs() <= 1.0;
            out.push(serde_json::json!({
                "id": e.id,
                "type": name,
                "pos": e.pos,
                "dist": (dist * 10.0).round() / 10.0,
                "reach_dist": (near * 10.0).round() / 10.0,
                "in_reach": near <= 3.0,
                "in_view": in_view,
                "behind": c.w <= 0.05,
                "x": if nx.is_nan() { serde_json::Value::Null } else { serde_json::json!(nx * 0.5 + 0.5) },
                "y": if ny.is_nan() { serde_json::Value::Null } else { serde_json::json!(0.5 - ny * 0.5) },
            }));
        }
        out.sort_by(|a, b| {
            a["dist"]
                .as_f64()
                .partial_cmp(&b["dist"].as_f64())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        out
    }

    /// VANILLA: EntityRenderer#submitNameTag — текст 0.025 блока/пиксель шрифта, по центру над якорем,
    /// подложка 0x40000000 (отступ 1 px). Тусклый see-through слой рисуется всегда, яркий —
    /// отдельным слоем с depth-тестом по буферу мира (`flush_nametags`).
    pub fn draw_nametags(
        &self,
        t: &mut ezzmc_render::text::TextRenderer,
        lang: &crate::ui::lang::Lang,
        gw: f32,
        gh: f32,
    ) {
        use crate::titles::{draw_spans, spans_width};
        let (vp, cam, fy) = self.nt_view;
        // (x, y, масштаб, w, z, спаны, присед)
        type Tag = (f32, f32, f32, f32, f32, Vec<crate::chat::Span>, bool);
        let mut tags: Vec<Tag> = Vec::new();
        for nt in &self.nametags {
            // VANILLA: EntityRenderer#submitNameDisplay — scoreText на якоре, имя выше на 9·1.15·0.025.
            let lines = nt.score.iter().map(|c| (c, 0.0)).chain(std::iter::once((
                &nt.text,
                if nt.score.is_some() {
                    9.0 * 1.15 * 0.025
                } else {
                    0.0
                },
            )));
            for (c, dy) in lines {
                let p = nt.anchor;
                let v = vp
                    * glam::Vec4::new(
                        (p[0] - cam[0]) as f32,
                        (p[1] + dy - cam[1]) as f32,
                        (p[2] - cam[2]) as f32,
                        1.0,
                    );
                if v.w <= 0.05 {
                    continue;
                }
                let (x, y) = ((v.x / v.w * 0.5 + 0.5) * gw, (0.5 - v.y / v.w * 0.5) * gh);
                tags.push((
                    x,
                    y,
                    0.025 * fy * (gh / 2.0) / v.w,
                    v.w,
                    v.z / v.w,
                    crate::chat::flatten(c, lang, true),
                    nt.discrete,
                ));
            }
        }
        tags.sort_by(|a, b| b.3.total_cmp(&a.3));
        for (x, y, sc, _, z, spans, sneak) in tags {
            let w = spans_width(t, &spans);
            t.set_transform(Some((x, y, 0.0, sc)));
            t.set_depth(None);
            t.fill(-w / 2.0 - 1.0, -1.0, w / 2.0, 8.0, 0x4000_0000);
            // VANILLA: слой see-through 0x20FFFFFF (виден всегда) + яркий 0xFFFFFFFF с depth-тестом
            // (у приседающего яркого слоя нет). Цвета компонента сохраняются, alpha — от слоя.
            draw_spans(t, &spans, -w / 2.0, 0.0, 32.0 / 255.0, false);
            if !sneak {
                // Чуть ближе к камере, чтобы не бороться с глубиной самого якоря.
                t.set_depth(Some((z - 1e-5).max(0.0)));
                draw_spans(t, &spans, -w / 2.0, 0.0, 1.0, false);
                t.set_depth(None);
            }
        }
        t.set_transform(None);
    }

    /// Надписи на табличках (мировая плоскость, слой с depth-тестом; сбрасывается в `flush_nametags`).
    pub fn draw_signs(&self, t: &mut ezzmc_render::text::TextRenderer, w: u32, h: u32) {
        let (vp, cam, _) = self.nt_view;
        self.signs.draw(t, vp, cam, (w as f32, h as f32));
    }

    /// Подписи значков карт (TODO-526): в рамках — с depth-тестом по миру; в руке — поверх
    /// (у руки своя очищенная глубина, сдвиг 2.0 прижимает к ближней плоскости).
    pub fn draw_map_labels(
        &self,
        t: &mut ezzmc_render::text::TextRenderer,
        lang: &crate::ui::lang::Lang,
        w: u32,
        h: u32,
    ) {
        let (vp, _, _) = self.nt_view;
        let size = (w as f32, h as f32);
        let sd = self.sky_darken;
        // Как у табличек: кривая LightTexture по max(block, sky − skyDarken).
        let k = |b: u16, s: u16| {
            let v = (i32::from(b).max(i32::from(s) - sd)).clamp(0, 15) as f32 / 15.0;
            (v / (4.0 - 3.0 * v)).max(0.04)
        };
        crate::maps::draw_labels(t, lang, &self.map_labels, vp, size, 3.0e-5, k);
        if !self.hud_hidden {
            crate::maps::draw_labels(
                t,
                lang,
                &self.hand_map_labels,
                Mat4::IDENTITY,
                size,
                2.0,
                |_, _| 1.0,
            );
        }
    }

    /// Яркий слой неймтегов с depth-тестом по мировому буферу кадра (после `render`).
    pub fn flush_nametags(
        &self,
        t: &mut ezzmc_render::text::TextRenderer,
        gpu: &Gpu,
        view: &wgpu::TextureView,
        w: u32,
        h: u32,
    ) {
        if let Some((d, dw, dh)) = &self.depth {
            if (*dw, *dh) == (w, h) {
                t.flush_depth(&gpu.device, &gpu.queue, view, d, DEPTH_FORMAT, w, h);
            }
        }
    }

    /// Текущий FOV с динамическим множителем (спринт/полёт), интерполяция по partial tick.
    pub fn fov_deg(&self) -> f32 {
        let pt = (self.tick_acc / 0.05) as f32;
        let (o, c) = self.fov_mod;
        self.fov_death_or_fluid(self.cfg.fov * (o + (c - o) * pt) * self.zoom_factor)
    }

    /// VANILLA: Camera#modifyFovBasedOnDeathOrFluid — смерть сужает FOV, вода/лава ×lerp(fovEffectScale, 1, 6/7).
    fn fov_death_or_fluid(&self, mut fov: f32) -> f32 {
        use ezzmc_render::fog::FogType;
        if self.player.dead {
            let pt = (self.tick_acc / 0.05) as f32;
            let dur = (self.player.death_time as f32 + pt).min(20.0);
            fov /= (1.0 - 500.0 / (dur + 500.0)) * 2.0 + 1.0;
        }
        if matches!(self.cam_fluid, FogType::Water | FogType::Lava) {
            fov *= 1.0 + (0.857_142_87 - 1.0) * self.cfg.fov_effect;
        }
        fov
    }

    /// Плавный зум: к 1/4 FOV при удержании клавиши, обратно при отпускании.
    pub fn tick_zoom(&mut self, dt: f64) {
        // VANILLA: Hud#extractCameraOverlays — scopeScale = lerp(0.5·Δтиков, scopeScale, 1.125); без трубы 0.5.
        if self.is_scoping() {
            let k = (0.5 * dt * 20.0).clamp(0.0, 1.0) as f32;
            self.scope_scale += (1.125 - self.scope_scale) * k;
        } else {
            self.scope_scale = 0.5;
        }
        let target = if self.zoom_held { 0.25 } else { 1.0 };
        let k = 1.0 - (-14.0 * dt.clamp(0.0, 0.1)).exp() as f32;
        self.zoom_factor += (target - self.zoom_factor) * k;
        if (self.zoom_factor - target).abs() < 0.002 {
            self.zoom_factor = target;
        }
    }

    pub fn fly(&mut self, dt: f64, axes: (f32, f32, f32), sprint: bool) {
        // VANILLA: творческий полёт ≈ 10.9 бл/с, спринт ×2.
        let speed = 10.89 * if sprint { 2.0 } else { 1.0 } * dt;
        let yaw = f64::from(self.yaw).to_radians();
        // yaw 0 → +Z; вправо = −X при yaw 0.
        let (fx, fz) = (-yaw.sin(), yaw.cos());
        let (rx, rz) = (-fz, fx);
        let (f, s, u) = (f64::from(axes.0), f64::from(axes.1), f64::from(axes.2));
        self.cam[0] += (fx * f + rx * s) * speed;
        self.cam[2] += (fz * f + rz * s) * speed;
        self.cam[1] += u * speed;
    }

    fn ensure_depth(&mut self, gpu: &Gpu, w: u32, h: u32) {
        if self.depth.as_ref().is_some_and(|d| d.1 == w && d.2 == h) {
            return;
        }
        let t = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth"),
            size: wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        self.depth = Some((t.create_view(&Default::default()), w, h));
        let t = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("hand depth"),
            size: wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        self.hand_depth = Some(t.create_view(&Default::default()));
    }

    /// Рисует мир в `target`. `clear` — цвет неба (M1.5 заменит настоящим небом).
    /// VANILLA: CameraType#cycle (key.togglePerspective).
    pub fn cycle_perspective(&mut self) {
        self.perspective = (self.perspective + 1) % 3;
    }

    /// VANILLA: Camera#setup для detached-камеры: разворот на 180° для вида спереди,
    /// отъезд на `getMaxZoom(4.0)` назад вдоль взгляда.
    /// VANILLA-UNSURE: ClipContext.Block.VISUAL заменён контуром блока (OUTLINE).
    fn third_person_view(&self) -> ([f64; 3], f32, f32) {
        let (yaw, pitch) = if self.perspective == 2 {
            (self.yaw + 180.0, -self.pitch)
        } else {
            (self.yaw, self.pitch)
        };
        let f = look_dir(yaw, pitch);
        let mut dist = 4.0f32;
        if let Ok(g) = self.net.level.lock() {
            if let Some(l) = g.as_ref() {
                let back = [-f[0], -f[1], -f[2]];
                for i in 0..8 {
                    let off = |b: i32| f64::from(((i >> b) & 1) * 2 - 1) * 0.1;
                    let from = [
                        self.cam[0] + off(0),
                        self.cam[1] + off(1),
                        self.cam[2] + off(2),
                    ];
                    if let Some((_, t)) =
                        crate::pick::raycast_t(l, from, back, false, f64::from(dist))
                    {
                        // Расстояние считается от позиции камеры, не от `from`.
                        let hit = [
                            from[0] + f64::from(back[0]) * t,
                            from[1] + f64::from(back[1]) * t,
                            from[2] + f64::from(back[2]) * t,
                        ];
                        let d2 = (hit[0] - self.cam[0]).powi(2)
                            + (hit[1] - self.cam[1]).powi(2)
                            + (hit[2] - self.cam[2]).powi(2);
                        if d2 < f64::from(dist).powi(2) {
                            dist = d2.sqrt() as f32;
                        }
                    }
                }
            }
        }
        let d = f64::from(dist);
        (
            [
                self.cam[0] - f64::from(f[0]) * d,
                self.cam[1] - f64::from(f[1]) * d,
                self.cam[2] - f64::from(f[2]) * d,
            ],
            yaw,
            pitch,
        )
    }

    pub fn render(
        &mut self,
        gpu: &Gpu,
        target: &wgpu::TextureView,
        w: u32,
        h: u32,
        clear: [f64; 3],
    ) {
        if self.perspective == 0 {
            return self.render_view(gpu, target, w, h, clear);
        }
        // Камера отъезжает только на время кадра; позиция игрока (`cam`) не меняется.
        let saved = (self.cam, self.yaw, self.pitch);
        let (c, y, p) = self.third_person_view();
        (self.cam, self.yaw, self.pitch) = (c, y, p);
        self.render_view(gpu, target, w, h, clear);
        (self.cam, self.yaw, self.pitch) = saved;
    }

    /// VANILLA: LivingEntityRenderer#submit для игрока: R_y(180−bodyRot)·S(−1,−1,1)·S(0.9375)·T(0,−1.501,0),
    /// поза из `HumanoidModel#setupAnim`, сдвиг вниз на 2/16 при приседе (AvatarRenderer#getRenderOffset).
    fn prepare_player_model(&mut self, gpu: &Gpu, mv: Mat4) {
        use ezzmc_render::humanoid::setup_anim;
        let mut data = Vec::new();
        let mut armor_data = Vec::new();
        let mut glint_data = Vec::new();
        let mut shadow = Vec::new();
        self.entity_item_visible = false;
        let held_mesh = self.inv.held().cloned().and_then(|st| {
            let (name, cx) = crate::handitem::stack_ctx(&st)?;
            let cx = self.use_ctx(cx);
            let rm = self.hand_rm.as_ref()?;
            self.hand_items.as_mut()?.get(rm, name, &cx)
        });
        let left_mesh = self.offhand_mesh(true);
        let held_foil = self.inv.held().is_some_and(foil_of);
        let left_foil = self
            .inv
            .slots
            .get(crate::inventory::OFFHAND)
            .and_then(|s| s.as_ref())
            .is_some_and(foil_of);
        let mut item_data = (Vec::new(), Vec::new());
        if self.perspective != 0 && self.walking && self.have_pos {
            let pt = (self.tick_acc / 0.05).clamp(0.0, 1.0) as f32;
            let p = &self.player;
            let lerp = |a: f64, b: f64| a + (b - a) * f64::from(pt);
            let pos = [
                lerp(p.prev_pos[0], p.pos[0]),
                lerp(p.prev_pos[1], p.pos[1]),
                lerp(p.prev_pos[2], p.pos[2]),
            ];
            let mut state = p.anim.humanoid_state(pt, p.crouching);
            if self.riding {
                // VANILLA: LivingEntityRenderer#extractRenderState — пассажир: walkAnimation = 0.
                state.passenger = true;
                state.walk_pos = 0.0;
                state.walk_speed = 0.0;
            }
            state.item_right = self.inv.held().is_some();
            state.item_left = left_mesh.is_some();
            let body = p.anim.body_rot(pt);
            let light = match self.net.level.lock().as_deref() {
                Ok(Some(l)) => {
                    crate::particles::light_at(l, pos[0], pos[1] + p.eye_height(), pos[2])
                }
                _ => (0, 15),
            };
            let swim = SwimPose {
                amount: state.swim_amount,
                in_water: p.in_water,
                visually: p.swimming || p.crawling,
                x_rot: state.x_rot,
                fly: p.anim.fly_pose(pt),
            };
            let root = self.avatar_root(pos, body, p.crouching, swim);
            let (model, parts) = &mut self.player_model;
            crate::settings::apply_parts(model, self.cfg.parts);
            setup_anim(model, parts, &state);
            model.bake(root, [255; 4], light, &mut data);
            if let Some(n) = self
                .inv
                .slots
                .get(crate::inventory::ARMOR_START)
                .and_then(|s| s.as_ref())
                .and_then(crate::skull::worn_name)
            {
                let head = model.part_matrices(root)[parts.head];
                self.head_skulls
                    .push((n, crate::skull::worn_root(head), light));
            }
            let looks = crate::armor::looks_of_menu(&self.inv.slots);
            self.armor_kit.fall_flying = self.player.fall_flying;
            self.armor_kit.bake(
                &looks,
                &state,
                root,
                [255; 4],
                light,
                &mut armor_data,
                &mut glint_data,
            );
            // VANILLA: AvatarRenderer — shadowRadius 0.5; тень не рисуется у невидимого.
            if let Ok(Some(l)) = self.net.level.lock().as_deref() {
                crate::entityshadow::bake(l, pos, self.cam, 0.5, 1.0, self.sky_darken, &mut shadow);
            }
            if let Some(mesh) = held_mesh {
                // VANILLA: ItemInHandLayer — рука, X−90°, Y180°, translate(1/16, 2/16, −10/16).
                let arm = model.part_matrices(root)[parts.right_arm]
                    * Mat4::from_rotation_x((-90.0f32).to_radians())
                    * Mat4::from_rotation_y(180.0f32.to_radians())
                    * Mat4::from_translation(Vec3::new(1.0 / 16.0, 0.125, -0.625));
                mesh.bake(
                    arm,
                    crate::handitem::View::ThirdPerson,
                    light,
                    held_foil,
                    &mut item_data.0,
                    &mut item_data.1,
                );
                self.entity_item_visible = true;
            }
            if let Some(mesh) = left_mesh {
                let arm = model.part_matrices(root)[parts.left_arm]
                    * Mat4::from_rotation_x((-90.0f32).to_radians())
                    * Mat4::from_rotation_y(180.0f32.to_radians())
                    * Mat4::from_translation(Vec3::new(-1.0 / 16.0, 0.125, -0.625));
                mesh.bake(
                    arm,
                    crate::handitem::View::ThirdPersonLeft,
                    light,
                    left_foil,
                    &mut item_data.0,
                    &mut item_data.1,
                );
                self.entity_item_visible = true;
            }
        }
        crate::mobs::prof_at(5, "t до bake_remote_players");
        let mut player_data = Vec::new();
        self.bake_remote_players(
            &mut player_data,
            &mut armor_data,
            &mut glint_data,
            &mut shadow,
            &mut item_data,
        );
        if let Ok(g) = self.net.level.lock() {
            if let Some(l) = g.as_ref() {
                self.mobs.bake(
                    l,
                    self.cam,
                    self.sky_darken,
                    self.entity_frustum.as_ref(),
                    &mut shadow,
                );
                for fx in std::mem::take(&mut self.mobs.dragon_fx) {
                    self.particles
                        .add_dragon_breath_burst(fx.head, fx.look, fx.motion);
                }
                // VANILLA: EnderDragon#tickDeath / aiStep — смещение (±4, 2±2, ±4) от позиции дракона.
                for (p, emitter) in std::mem::take(&mut self.mobs.dragon_booms) {
                    let ps = &mut self.particles;
                    let xo = f64::from((ps.rand_f32() - 0.5) * 8.0);
                    let yo = f64::from((ps.rand_f32() - 0.5) * 4.0);
                    let zo = f64::from((ps.rand_f32() - 0.5) * 8.0);
                    let (x, y, z) = (p[0] + xo, p[1] + 2.0 + yo, p[2] + zo);
                    if emitter {
                        ps.add_explosion_emitter(x, y, z);
                    } else {
                        // HugeExplosionParticle.Provider: xAux = 0 → quadSize 2.
                        ps.add_huge(x, y, z, 0.0);
                    }
                }
                let t_chest = std::time::Instant::now();
                self.chests.bake(
                    l,
                    self.cam,
                    (self.queue.version, self.queue.block_version),
                    &mut self.mobs,
                    self.entity_frustum.as_ref(),
                );
                crate::mobs::prof_phase(16, "chests.bake", t_chest.elapsed());
                self.bells.bake(
                    l,
                    self.cam,
                    (self.queue.version, self.queue.block_version),
                    &mut self.mobs,
                );
                crate::painting::bake(l, self.cam, &mut self.mobs);
                self.tables.bake(
                    l,
                    self.cam,
                    (self.queue.version, self.queue.block_version),
                    &mut self.mobs,
                );
                self.skulls.bake(
                    l,
                    self.cam,
                    (self.queue.version, self.queue.block_version),
                    &mut self.mobs,
                );
                self.shulker_boxes.bake(
                    l,
                    self.cam,
                    (self.queue.version, self.queue.block_version),
                    &mut self.mobs,
                );
                self.banners.bake(
                    l,
                    self.cam,
                    (self.queue.version, self.queue.block_version),
                    &mut self.mobs,
                );
                self.signs.bake(
                    l,
                    self.cam,
                    (self.queue.version, self.queue.block_version),
                    self.sky_darken,
                );
            }
        }
        // VANILLA: ItemInHandLayer у мобов — рука + X−90°, Y180°, translate(±1/16, 2/16, −10/16).
        for (st, hand, light, left) in std::mem::take(&mut self.mobs.held_items) {
            let Some((name, cx)) = crate::handitem::stack_ctx(&st) else {
                continue;
            };
            let Some(rm) = self.hand_rm.as_ref() else {
                continue;
            };
            let Some(mesh) = self.hand_items.as_mut().and_then(|h| h.get(rm, name, &cx)) else {
                continue;
            };
            let dx = if left { -1.0 } else { 1.0 };
            let m = hand
                * Mat4::from_rotation_x((-90.0f32).to_radians())
                * Mat4::from_rotation_y(180.0f32.to_radians())
                * Mat4::from_translation(Vec3::new(dx / 16.0, 0.125, -0.625));
            let view = if left {
                crate::handitem::View::ThirdPersonLeft
            } else {
                crate::handitem::View::ThirdPerson
            };
            mesh.bake(
                m,
                view,
                light,
                foil_of(&st),
                &mut item_data.0,
                &mut item_data.1,
            );
            self.entity_item_visible = true;
        }
        self.bake_dragon_fireballs();
        for (name, root, light) in std::mem::take(&mut self.head_skulls) {
            if let Some((m, tex)) = crate::skull::item_model(name) {
                m.bake(root, [255; 4], light, self.mobs.out(tex));
            }
        }
        if let Some(rm) = self.hand_rm.as_ref() {
            let (color, projection, fog, lightmap, atlas) = (
                self.color_format,
                &self.projection_buf,
                &self.fog_buf,
                &self.lightmap.view,
                &self.atlas_view,
            );
            let mut make = |tex: &str| {
                let mask = tex == crate::mobs::MASK_TEX;
                // VANILLA: RenderTypes#waterMask — без текстуры; берём сплошную белую (в boat/oak есть
                // прозрачные пиксели, и depth-only проход отбрасывал бы их — вода просвечивала).
                let (rgba, tw, th) = if mask {
                    (vec![255u8; 4 * 16 * 16], 16, 16)
                } else {
                    // `@путь` — от textures/ (картины), иначе от textures/entity/.
                    let tex = tex.strip_suffix("#dissolve").unwrap_or(tex);
                    let path = match tex.strip_prefix('@') {
                        Some(p) => format!("assets/minecraft/textures/{p}.png"),
                        None => format!("assets/minecraft/textures/entity/{tex}.png"),
                    };
                    let img = rm.read(&path).and_then(|d| Image::decode_png(&d).ok())?;
                    (img.to_rgba8(), img.width, img.height)
                };
                if tex == crate::mobs::DRAGON_DISSOLVE_TEX {
                    let path = format!(
                        "assets/minecraft/textures/entity/{}.png",
                        crate::mobs::DRAGON_EXPLODING_TEX
                    );
                    let m = rm.read(&path).and_then(|d| Image::decode_png(&d).ok())?;
                    let mrgba = m.to_rgba8();
                    return Some(ezzmc_render::particledraw::ParticleDraw::new_dissolve(
                        &gpu.device,
                        &gpu.queue,
                        color,
                        DEPTH_FORMAT,
                        projection,
                        fog,
                        lightmap,
                        atlas,
                        Some((rgba.as_slice(), tw, th)),
                        (mrgba.as_slice(), m.width, m.height),
                    ));
                }
                let ctor = if mask {
                    ezzmc_render::particledraw::ParticleDraw::new_mask
                } else if tex == crate::mobs::SWIRL_TEX {
                    ezzmc_render::particledraw::ParticleDraw::new_additive
                } else if tex == crate::mobs::BEAM_TEX {
                    ezzmc_render::particledraw::ParticleDraw::new_repeat
                } else {
                    ezzmc_render::particledraw::ParticleDraw::new
                };
                Some(ctor(
                    &gpu.device,
                    &gpu.queue,
                    color,
                    DEPTH_FORMAT,
                    projection,
                    fog,
                    lightmap,
                    atlas,
                    Some((rgba.as_slice(), tw, th)),
                ))
            };
            let t_prep = std::time::Instant::now();
            self.mobs.prepare(&gpu.device, &gpu.queue, mv, &mut make);
            crate::mobs::prof_prepare(t_prep.elapsed());
        }
        crate::mobs::prof_at(6, "t после mobs.prepare");
        self.bake_item_entities(&mut shadow);
        crate::mobs::prof_at(11, "t после bake_item_entities");
        let mut arrow_data = Vec::new();
        self.bake_projectiles(&mut item_data, &mut arrow_data);
        let mut map_data = Vec::new();
        self.bake_item_frames(&mut item_data, &mut map_data);
        self.maps.upload(&gpu.queue, &self.map_draw);
        self.map_draw
            .prepare_raw(&gpu.device, &gpu.queue, mv, &map_data);
        crate::mobs::prof_at(12, "t после bake_projectiles");
        self.arrow_draw
            .prepare_raw(&gpu.device, &gpu.queue, mv, &arrow_data);
        self.bake_flames(&mut item_data.0);
        crate::mobs::prof_at(9, "t после bake_flames");
        self.entity_draw
            .prepare_raw(&gpu.device, &gpu.queue, mv, &data);
        for (slot, rgba) in self.skin_uploads.drain(..) {
            self.player_draw
                .write_region(&gpu.queue, 0, slot * 64, 64, 64, &rgba);
        }
        self.player_draw
            .prepare_raw(&gpu.device, &gpu.queue, mv, &player_data);
        self.armor_draw
            .prepare_raw(&gpu.device, &gpu.queue, mv, &armor_data);
        self.glint_draw
            .set_texture_mat(crate::armor::glint_matrix(0.16));
        self.glint_draw
            .prepare_raw(&gpu.device, &gpu.queue, mv, &glint_data);
        let mut pdata = Vec::new();
        if let Ok(g) = self.net.level.lock() {
            if let Some(l) = g.as_ref() {
                crate::pistonview::bake(l, &self.models, self.cam, l.partial_tick(), &mut pdata);
                crate::pistonview::bake_tnt(
                    l,
                    &self.models,
                    self.cam,
                    l.partial_tick(),
                    &mut pdata,
                    self.sky_darken,
                    &mut shadow,
                );
                self.mobs.bake_mushrooms(&self.models, &mut pdata);
                self.mobs.bake_antenna(&self.models, &mut pdata);
                self.mobs.bake_carried(&self.models, &mut pdata);
                crate::pistonview::bake_falling(
                    l,
                    &self.models,
                    self.cam,
                    l.partial_tick(),
                    &mut pdata,
                    self.sky_darken,
                    &mut shadow,
                );
            }
        }
        self.piston_draw
            .prepare_split(&gpu.device, &gpu.queue, mv, &pdata, &[]);
        // VANILLA: options.entityShadows=false — тени не рисуются.
        if !self.cfg.entity_shadows {
            shadow.clear();
        }
        self.shadow_draw
            .prepare_raw(&gpu.device, &gpu.queue, mv, &shadow);
        crate::mobs::prof_phase(
            20,
            &format!(
                "item_data блоки {} КБ / предметы {} КБ",
                item_data.0.len() / 1024,
                item_data.1.len() / 1024
            ),
            std::time::Duration::ZERO,
        );
        let t_up = std::time::Instant::now();
        if let Some(d) = self.entity_item_draw.as_mut() {
            d.prepare_split(&gpu.device, &gpu.queue, mv, &item_data.0, &item_data.1);
        }
        crate::mobs::prof_phase(21, "upload item_draw", t_up.elapsed());
        let t_up = std::time::Instant::now();
        if let Some(d) = self.entity_glint_draw.as_mut() {
            d.set_texture_mat(crate::armor::glint_matrix(8.0));
            d.prepare_split(&gpu.device, &gpu.queue, mv, &item_data.0, &item_data.1);
        }
        crate::mobs::prof_phase(22, "upload glint_draw", t_up.elapsed());
        self.upload_item_inst(gpu, mv);
    }

    /// Заливает экземпляры дропов (GPU-инстансинг): новые модели — на GPU, матрицы+свет — в storage-буфер.
    fn upload_item_inst(&mut self, gpu: &Gpu, mv: Mat4) {
        let t = std::time::Instant::now();
        let Some(inst) = self.item_inst.as_mut() else {
            return;
        };
        let frame = std::mem::take(&mut self.item_inst_frame);
        let mut data: Vec<f32> = Vec::new();
        let mut ranges = Vec::with_capacity(frame.len());
        for f in &frame {
            let key = std::sync::Arc::as_ptr(&f.mesh) as usize;
            let mesh = *self.item_inst_map.entry(key).or_insert_with(|| {
                let (b, i) = f.mesh.static_mesh();
                self.item_inst_keep.push(f.mesh.clone());
                inst.add_mesh(&gpu.device, &b, &i)
            });
            let first = (data.len() / ezzmc_render::particledraw::ITEM_INST_FLOATS) as u32;
            data.extend_from_slice(&f.normal);
            data.extend_from_slice(&f.foil);
            let n = (f.normal.len() + f.foil.len()) / ezzmc_render::particledraw::ITEM_INST_FLOATS;
            ranges.push(ezzmc_render::particledraw::ItemInstRange {
                mesh,
                first,
                count: n as u32,
                foil: (f.foil.len() / ezzmc_render::particledraw::ITEM_INST_FLOATS) as u32,
            });
        }
        inst.prepare(&gpu.device, &gpu.queue, bytemuck_cast(&data), &ranges);
        if let Some(d) = self.entity_item_draw.as_ref() {
            d.set_model_view(&gpu.queue, mv);
        }
        if let Some(d) = self.entity_glint_draw.as_ref() {
            d.set_model_view(&gpu.queue, mv);
        }
        crate::mobs::prof_phase(23, "дропы: upload instancing", t.elapsed());
    }

    /// VANILLA: LivingEntityRenderer#submit — R_y(180−bodyRot)·S(−1,−1,1)·S(0.9375)·T(0,−1.501,0),
    /// сдвиг вниз на 2/16 при приседе (AvatarRenderer#getRenderOffset).
    fn avatar_root(&self, pos: [f64; 3], body: f32, crouching: bool, swim: SwimPose) -> Mat4 {
        let off = Vec3::new(
            (pos[0] - self.cam[0]) as f32,
            (pos[1] - self.cam[1]) as f32 + if crouching { -2.0 / 16.0 } else { 0.0 },
            (pos[2] - self.cam[2]) as f32,
        );
        Mat4::from_translation(off)
            * Mat4::from_rotation_y((180.0 - body).to_radians())
            * swim.matrix()
            * Mat4::from_scale(Vec3::new(-1.0, -1.0, 1.0))
            * Mat4::from_scale(Vec3::splat(0.9375))
            * Mat4::from_translation(Vec3::new(0.0, -1.501, 0.0))
    }

    /// VANILLA: LivingEntityRenderer#submit/setupRotations для Pose.SLEEPING: сдвиг от кровати на
    /// `eyeHeight − 0.1` против ориентации, затем YP(sleepDirectionToRotation), ZP(90), YP(270).
    fn avatar_root_sleeping(&self, pos: [f64; 3], facing: &str) -> Mat4 {
        let (angle, sx, sz) = match facing {
            "south" => (90.0f32, 0.0f32, 1.0f32),
            "west" => (0.0, -1.0, 0.0),
            "north" => (270.0, 0.0, -1.0),
            _ => (180.0, 1.0, 0.0),
        };
        let head_off = 1.62 - 0.1;
        let off = Vec3::new(
            (pos[0] - self.cam[0]) as f32 - sx * head_off,
            (pos[1] - self.cam[1]) as f32,
            (pos[2] - self.cam[2]) as f32 - sz * head_off,
        );
        Mat4::from_translation(off)
            * Mat4::from_rotation_y(angle.to_radians())
            * Mat4::from_rotation_z(90.0f32.to_radians())
            * Mat4::from_rotation_y(270.0f32.to_radians())
            * Mat4::from_scale(Vec3::new(-1.0, -1.0, 1.0))
            * Mat4::from_scale(Vec3::splat(0.9375))
            * Mat4::from_translation(Vec3::new(0.0, -1.501, 0.0))
    }

    /// Рука от 1-го лица. VANILLA: FirstPersonHandsAndItemsRenderer#renderPlayerArm (пустая правая рука),
    /// AvatarRenderer#renderHand (rightArm.zRot = 0.1), GameRenderer#renderItemInHand
    /// (проекция 70° + bobView, model-view = единица, глубина очищена).
    fn prepare_hand(&mut self, gpu: &Gpu, aspect: f32, far: f32) {
        self.hand_visible = false;
        self.hand_item_visible = false;
        self.hand_map_visible = false;
        self.hand_map_labels.clear();
        // VANILLA: GameRenderer#renderItemInHand — не в режиме SPECTATOR.
        if self.perspective != 0 || !self.walking || !self.have_pos || self.game_mode == 3 {
            return;
        }
        let pt = (self.tick_acc / 0.05).clamp(0.0, 1.0) as f32;
        let p = &self.player;
        let attack = p.anim.humanoid_state(pt, p.crouching).swing;
        let lerp = |a: f32, b: f32| a + (b - a) * pt;
        let inverse_h = 1.0 - lerp(p.hand_h_o, p.hand_h);
        let (x_bob, y_bob) = (lerp(p.x_bob_o, p.x_bob), lerp(p.y_bob_o, p.y_bob));
        let pi = std::f32::consts::PI;
        let sqrt_a = attack.sqrt();
        let xs = -0.3 * (sqrt_a * pi).sin();
        let ys = 0.4 * (sqrt_a * pi * 2.0).sin();
        let zs = -0.4 * (attack * pi).sin();
        let z_rot = (attack * attack * pi).sin();
        let y_rot = (sqrt_a * pi).sin();
        let m = Mat4::from_rotation_x(((self.pitch - x_bob) * 0.1).to_radians())
            * Mat4::from_rotation_y(((self.yaw - y_bob) * 0.1).to_radians())
            * Mat4::from_translation(Vec3::new(
                xs + 0.640_000_05,
                ys - 0.6 + inverse_h * -0.6,
                zs - 0.719_999_97,
            ))
            * Mat4::from_rotation_y(45.0f32.to_radians())
            * Mat4::from_rotation_y((y_rot * 70.0).to_radians())
            * Mat4::from_rotation_z((z_rot * -20.0).to_radians())
            * Mat4::from_translation(Vec3::new(-1.0, 3.6, 3.5))
            * Mat4::from_rotation_z(120.0f32.to_radians())
            * Mat4::from_rotation_x(200.0f32.to_radians())
            * Mat4::from_rotation_y((-135.0f32).to_radians())
            * Mat4::from_translation(Vec3::new(5.6, 0.0, 0.0));
        let light = match self.net.level.lock().as_deref() {
            Ok(Some(l)) => crate::particles::light_at(l, self.cam[0], self.cam[1], self.cam[2]),
            _ => (0, 15),
        };
        let (mut arm_data, mut map_data) = (Vec::new(), Vec::new());
        let main_map = self.bake_hand_maps(
            gpu,
            pt,
            inverse_h,
            attack,
            x_bob,
            y_bob,
            light,
            &mut arm_data,
            &mut map_data,
        );
        let mesh = if main_map { None } else { self.shown_mesh() };
        if let (Some(mesh), Some(_d)) = (mesh, self.hand_item_draw.as_mut()) {
            // VANILLA: ItemInHandRenderer#renderArmWithItem (одноручный предмет) + applyItemArmAttackTransform.
            let sa = attack.sqrt();
            let g = (sa * pi).sin();
            let f = (attack * attack * pi).sin();
            // VANILLA: applyItemArmAttackTransform — i = (arm == RIGHT) ? 1 : -1 (mainHand=left зеркалит).
            let i = if self.cfg.left_handed { -1.0f32 } else { 1.0 };
            let look = Mat4::from_rotation_x(((self.pitch - x_bob) * 0.1).to_radians())
                * Mat4::from_rotation_y(((self.yaw - y_bob) * 0.1).to_radians());
            let place =
                Mat4::from_translation(Vec3::new(i * 0.56, -0.52 + inverse_h * -0.6, -0.72));
            let root = if let Some(u) = self.using.as_ref().filter(|u| u.bow) {
                // VANILLA: FirstPersonHandsAndItemsRenderer — ItemUseAnimation.BOW.
                let held = (u.duration - u.remaining) as f32 + pt;
                let mut power = held / 20.0;
                power = ((power * power + power * 2.0) / 3.0).min(1.0);
                let mut b =
                    Mat4::from_translation(Vec3::new(i * -0.278_568_2, 0.183_443_87, 0.157_315_31))
                        * Mat4::from_rotation_x((-13.935f32).to_radians())
                        * Mat4::from_rotation_y((i * 35.3).to_radians())
                        * Mat4::from_rotation_z((i * -9.785).to_radians());
                if power > 0.1 {
                    let shake = ((held - 0.1) * 1.3).sin() * (power - 0.1);
                    b *= Mat4::from_translation(Vec3::new(0.0, shake * 0.004, 0.0));
                }
                b *= Mat4::from_translation(Vec3::new(0.0, 0.0, power * 0.04))
                    * Mat4::from_scale(Vec3::new(1.0, 1.0, 1.0 + power * 0.2))
                    * Mat4::from_rotation_y((-i * 45.0).to_radians());
                look * place * b
            } else if let Some(u) = self.using.as_ref().filter(|u| u.remaining > 0) {
                // VANILLA: FirstPersonHandsAndItemsRenderer#applyEatTransform (до applyItemArmTransform).
                let curr = u.remaining as f32 - pt + 1.0;
                let scaled = curr / u.duration as f32;
                let mut e = Mat4::IDENTITY;
                if scaled < 0.8 {
                    e *= Mat4::from_translation(Vec3::new(
                        0.0,
                        (curr / 4.0 * pi).cos().abs() * 0.1,
                        0.0,
                    ));
                }
                let j = 1.0 - scaled.powi(27);
                e *= Mat4::from_translation(Vec3::new(j * 0.6 * i, j * -0.5, 0.0))
                    * Mat4::from_rotation_y((i * j * 90.0).to_radians())
                    * Mat4::from_rotation_x((j * 10.0).to_radians())
                    * Mat4::from_rotation_z((i * j * 30.0).to_radians());
                look * e * place
            } else {
                look * Mat4::from_translation(Vec3::new(
                    i * -0.4 * g,
                    0.2 * (sa * pi * 2.0).sin(),
                    -0.2 * (attack * pi).sin(),
                )) * place
                    * Mat4::from_rotation_y((i * (45.0 + f * -20.0)).to_radians())
                    * Mat4::from_rotation_z((i * g * -20.0).to_radians())
                    * Mat4::from_rotation_x((g * -80.0).to_radians())
                    * Mat4::from_rotation_y((i * -45.0f32).to_radians())
            };
            let (mut blocks, mut items) = (Vec::new(), Vec::new());
            mesh.bake(
                root,
                if self.cfg.left_handed {
                    crate::handitem::View::FirstPersonLeft
                } else {
                    crate::handitem::View::FirstPerson
                },
                light,
                self.hand_shown.as_ref().is_some_and(|(s, _)| foil_of(s)),
                &mut blocks,
                &mut items,
            );
            self.bake_offhand_fp(light, x_bob, y_bob, &mut blocks, &mut items);
            let Some(d) = self.hand_item_draw.as_mut() else {
                return;
            };
            d.prepare_split(&gpu.device, &gpu.queue, Mat4::IDENTITY, &blocks, &items);
            if let Some(g) = self.hand_glint_draw.as_mut() {
                g.set_texture_mat(crate::armor::glint_matrix(8.0));
                g.prepare_split(&gpu.device, &gpu.queue, Mat4::IDENTITY, &blocks, &items);
            }
            self.hand_item_visible = true;
        } else if self.hand_shown.is_none() || main_map {
            if !main_map {
                self.bake_arm(m, true, light, &mut arm_data);
            }
            let (mut blocks, mut items) = (Vec::new(), Vec::new());
            self.bake_offhand_fp(light, x_bob, y_bob, &mut blocks, &mut items);
            if let Some(d) = self.hand_item_draw.as_mut() {
                d.prepare_split(&gpu.device, &gpu.queue, Mat4::IDENTITY, &blocks, &items);
                if let Some(g) = self.hand_glint_draw.as_mut() {
                    g.set_texture_mat(crate::armor::glint_matrix(8.0));
                    g.prepare_split(&gpu.device, &gpu.queue, Mat4::IDENTITY, &blocks, &items);
                }
                self.hand_item_visible = !blocks.is_empty() || !items.is_empty();
            }
        }
        if !arm_data.is_empty() {
            self.hand_draw
                .prepare_raw(&gpu.device, &gpu.queue, Mat4::IDENTITY, &arm_data);
            self.hand_visible = true;
        }
        if !map_data.is_empty() {
            self.hand_map_draw
                .prepare_raw(&gpu.device, &gpu.queue, Mat4::IDENTITY, &map_data);
            self.hand_map_visible = true;
        }
        // VANILLA: Camera#calculateHudFov = modifyFovBasedOnDeathOrFluid(70).
        let hud_fov = self.fov_death_or_fluid(70.0);
        let proj = Mat4::perspective_rh(hud_fov.to_radians(), aspect, NEAR, far) * self.bob(pt);
        for l in &mut self.hand_map_labels {
            l.m = proj * l.m;
        }
        gpu.queue.write_buffer(
            &self.hand_proj_buf,
            0,
            &ProjectionUniform { proj_mat: proj }.to_bytes(),
        );
    }

    /// Одна рука модели игрока. VANILLA: AvatarRenderer#renderHand (zRot ±0.1).
    fn bake_arm(&mut self, m: Mat4, right: bool, light: (u8, u8), out: &mut Vec<u8>) {
        let (model, parts) = &mut self.hand_model;
        model.reset_pose();
        model.parts[parts.right_arm].pose.z_rot = 0.1;
        model.parts[parts.left_arm].pose.z_rot = -0.1;
        let keep: &dyn Fn(&str) -> bool = if right {
            &|n| matches!(n, "right_arm" | "right_sleeve")
        } else {
            &|n| matches!(n, "left_arm" | "left_sleeve")
        };
        model.bake_parts(m, [255; 4], light, out, keep);
    }

    /// Карты в руках от 1-го лица. VANILLA: FirstPersonHandsAndItemsRenderer#renderTwoHandedMap /
    /// renderOneHandedMap / renderMapHand / renderMap (26.3). Возвращает true, если в основной руке карта.
    #[allow(clippy::too_many_arguments)]
    fn bake_hand_maps(
        &mut self,
        gpu: &Gpu,
        pt: f32,
        inverse_h: f32,
        attack: f32,
        x_bob: f32,
        y_bob: f32,
        light: (u8, u8),
        arms: &mut Vec<u8>,
        out: &mut Vec<u8>,
    ) -> bool {
        use crate::maps;
        let has = |s: Option<&ezzmc_protocol::item::ItemStack>| {
            s.filter(|s| s.count > 0).and_then(maps::map_id)
        };
        let main = has(self.hand_shown.as_ref().map(|(s, _)| s));
        let off = has(self.off_shown.as_ref());
        if main.is_none() && off.is_none() {
            return false;
        }
        let pi = std::f32::consts::PI;
        let look = Mat4::from_rotation_x(((self.pitch - x_bob) * 0.1).to_radians())
            * Mat4::from_rotation_y(((self.yaw - y_bob) * 0.1).to_radians());
        let l240 = (u16::from(light.0) * 16, u16::from(light.1) * 16);
        // Заливка текстур карт в руке — только при смене карты или её содержимого.
        for (slot, id) in [main, off].into_iter().enumerate() {
            let cur = id.and_then(|id| self.maps.map.get(&id).map(|m| (id, m.version)));
            if cur.is_some() && cur != self.hand_map_seen[slot] {
                if let Some(px) = cur.and_then(|(id, _)| self.maps.rgba(id)) {
                    self.hand_map_draw.write_region(
                        &gpu.queue,
                        slot as u32 * 128,
                        0,
                        128,
                        128,
                        &px,
                    );
                }
                self.hand_map_seen[slot] = cur;
            }
        }
        // VANILLA: renderMap.
        let maps_ref = &self.maps;
        let hand_labels = std::cell::RefCell::new(Vec::new());
        let render_map = |m: Mat4, slot: usize, id: i32, out: &mut Vec<u8>| {
            let m = m
                * Mat4::from_rotation_y(pi)
                * Mat4::from_rotation_z(pi)
                * Mat4::from_scale(Vec3::splat(0.38))
                * Mat4::from_translation(Vec3::new(-0.5, -0.5, 0.0))
                * Mat4::from_scale(Vec3::splat(0.0078125));
            let has_data = maps_ref.map.contains_key(&id);
            let [u0, v0, u1, v1] = maps::hand_background(has_data);
            for (p, uv) in [
                ([-7.0, 135.0, 0.0], [u0, v1]),
                ([135.0, 135.0, 0.0], [u1, v1]),
                ([135.0, -7.0, 0.0], [u1, v0]),
                ([-7.0, -7.0, 0.0], [u0, v0]),
            ] {
                let q = m.transform_point3(Vec3::from(p));
                ezzmc_render::particledraw::push_vertex_raw(out, q.to_array(), uv, [255; 4], l240);
            }
            maps_ref.bake(
                id,
                m,
                false,
                l240,
                Some(maps::hand_layout(slot)),
                out,
                &mut hand_labels.borrow_mut(),
            );
        };
        // VANILLA: renderPlayerArm (invert = ±1).
        let player_arm = |inv: f32, inv_h: f32, a: f32| {
            let sq = a.sqrt();
            Mat4::from_translation(Vec3::new(
                inv * (-0.3 * (sq * pi).sin() + 0.640_000_05),
                0.4 * (sq * pi * 2.0).sin() - 0.6 + inv_h * -0.6,
                -0.4 * (a * pi).sin() - 0.719_999_97,
            )) * Mat4::from_rotation_y((inv * 45.0).to_radians())
                * Mat4::from_rotation_y((inv * (sq * pi).sin() * 70.0).to_radians())
                * Mat4::from_rotation_z((inv * (a * a * pi).sin() * -20.0).to_radians())
                * Mat4::from_translation(Vec3::new(-inv, 3.6, 3.5))
                * Mat4::from_rotation_z((inv * 120.0).to_radians())
                * Mat4::from_rotation_x(200.0f32.to_radians())
                * Mat4::from_rotation_y((inv * -135.0).to_radians())
                * Mat4::from_translation(Vec3::new(inv * 5.6, 0.0, 0.0))
        };
        let mut arm_list: Vec<(Mat4, bool)> = Vec::new();
        // VANILLA: renderOneHandedMap.
        let one_handed = |arm_list: &mut Vec<(Mat4, bool)>,
                          inv: f32,
                          inv_h: f32,
                          a: f32,
                          slot: usize,
                          id: i32,
                          out: &mut Vec<u8>| {
            let b = look * Mat4::from_translation(Vec3::new(inv * 0.125, -0.125, 0.0));
            arm_list.push((
                b * Mat4::from_rotation_z((inv * 10.0).to_radians()) * player_arm(inv, inv_h, a),
                inv > 0.0,
            ));
            let sq = a.sqrt();
            let xs = (sq * pi).sin();
            let m = b
                * Mat4::from_translation(Vec3::new(inv * 0.51, -0.08 + inv_h * -1.2, -0.75))
                * Mat4::from_translation(Vec3::new(
                    inv * -0.5 * xs,
                    0.4 * (sq * pi * 2.0).sin() - 0.3 * xs,
                    -0.3 * (a * pi).sin(),
                ))
                * Mat4::from_rotation_x((xs * -45.0).to_radians())
                * Mat4::from_rotation_y((inv * xs * -30.0).to_radians());
            render_map(m, slot, id, out);
        };
        let main_inv = if self.cfg.left_handed { -1.0f32 } else { 1.0 };
        if let Some(id) = main {
            if self.off_shown.as_ref().is_none_or(|s| s.count <= 0) {
                // VANILLA: renderTwoHandedMap + calculateMapTilt.
                let sq = attack.sqrt();
                let tilt = (1.0 - self.pitch / 45.0 + 0.1).clamp(0.0, 1.0);
                let tilt = -(tilt * pi).cos() * 0.5 + 0.5;
                let base =
                    look * Mat4::from_translation(Vec3::new(
                        0.0,
                        0.2 * (attack * pi).sin() / 2.0,
                        -0.4 * (sq * pi).sin(),
                    )) * Mat4::from_translation(Vec3::new(
                        0.0,
                        0.04 + inverse_h * -1.2 + tilt * -0.5,
                        -0.72,
                    )) * Mat4::from_rotation_x((tilt * -85.0).to_radians());
                for inv in [1.0f32, -1.0] {
                    // VANILLA: renderMapHand.
                    arm_list.push((
                        base * Mat4::from_rotation_y(90f32.to_radians())
                            * Mat4::from_rotation_y(92f32.to_radians())
                            * Mat4::from_rotation_x(45f32.to_radians())
                            * Mat4::from_rotation_z((inv * -41.0).to_radians())
                            * Mat4::from_translation(Vec3::new(inv * 0.3, -1.1, 0.45)),
                        inv > 0.0,
                    ));
                }
                let m = base
                    * Mat4::from_rotation_x(((sq * pi).sin() * 20.0).to_radians())
                    * Mat4::from_scale(Vec3::splat(2.0));
                render_map(m, 0, id, out);
            } else {
                one_handed(&mut arm_list, main_inv, inverse_h, attack, 0, id, out);
            }
        }
        if let Some(id) = off {
            let inv_h = 1.0 - (self.off_h.0 + (self.off_h.1 - self.off_h.0) * pt);
            one_handed(&mut arm_list, -main_inv, inv_h, 0.0, 1, id, out);
        }
        self.hand_map_labels = hand_labels.into_inner();
        for (m, right) in arm_list {
            self.bake_arm(m, right, light, arms);
        }
        main.is_some()
    }

    /// VANILLA: renderArmWithItem(OFF_HAND): translate(−0.56, −0.52, −0.72), attack = 0.
    // TODO: у ванили у второй руки есть подъём/опускание (offHandHeight) — не сделано.
    fn bake_offhand_fp(
        &mut self,
        light: (u8, u8),
        xb: f32,
        yb: f32,
        b: &mut Vec<u8>,
        i: &mut Vec<u8>,
    ) {
        // Карта во второй руке рисуется в `bake_hand_maps`.
        if self
            .off_shown
            .as_ref()
            .is_some_and(|s| crate::maps::map_id(s).is_some())
        {
            return;
        }
        if let Some(m) = self.offhand_mesh(false) {
            let pt = (self.tick_acc / 0.05).clamp(0.0, 1.0) as f32;
            let inv_h = 1.0 - (self.off_h.0 + (self.off_h.1 - self.off_h.0) * pt);
            let root = Mat4::from_rotation_x(((self.pitch - xb) * 0.1).to_radians())
                * Mat4::from_rotation_y(((self.yaw - yb) * 0.1).to_radians())
                * Mat4::from_translation(Vec3::new(
                    if self.cfg.left_handed { 0.56 } else { -0.56 },
                    -0.52 + inv_h * -0.6,
                    -0.72,
                ));
            let view = if self.cfg.left_handed {
                crate::handitem::View::FirstPerson
            } else {
                crate::handitem::View::FirstPersonLeft
            };
            let foil = self.off_shown.as_ref().is_some_and(foil_of);
            m.bake(root, view, light, foil, b, i);
        }
    }

    /// Состояние игрока для превью в инвентаре. VANILLA: InventoryScreen рисует LocalPlayer как есть
    /// (присед, плавание/ползание, предметы в руках).
    pub fn preview_snapshot(&mut self) -> crate::charpreview::Snap {
        let pt = (self.tick_acc / 0.05).clamp(0.0, 1.0) as f32;
        let p = &self.player;
        let crouching = p.crouching;
        let mut state = p.anim.humanoid_state(pt, crouching);
        if self.riding {
            // VANILLA: LivingEntityRenderer#extractRenderState — пассажир: поза сидя, walkAnimation = 0.
            state.passenger = true;
            state.walk_pos = 0.0;
            state.walk_speed = 0.0;
        }
        let swim = SwimPose {
            amount: state.swim_amount,
            in_water: p.in_water,
            visually: p.swimming || p.crawling,
            x_rot: 0.0,
            fly: None,
        };
        let held = self.inv.held().cloned().and_then(|st| {
            let (name, cx) = crate::handitem::stack_ctx(&st)?;
            let cx = self.use_ctx(cx);
            let rm = self.hand_rm.as_ref()?;
            self.hand_items.as_mut()?.get(rm, name, &cx)
        });
        let left = self.offhand_mesh(true);
        state.item_right = held.is_some();
        state.item_left = left.is_some();
        if self.preview_items.is_none() {
            self.preview_items = self
                .hand_items
                .as_ref()
                .map(|h| std::sync::Arc::new(h.items_rgba()));
        }
        crate::charpreview::Snap {
            state,
            crouching,
            swim,
            right: held,
            left,
            blocks: self.preview_blocks.clone(),
            items: self.preview_items.clone(),
            foil: (
                self.inv.held().is_some_and(foil_of),
                self.inv
                    .slots
                    .get(crate::inventory::OFFHAND)
                    .and_then(|s| s.as_ref())
                    .is_some_and(foil_of),
            ),
        }
    }

    fn offhand_mesh(&mut self, real: bool) -> Option<std::sync::Arc<crate::handitem::HandMesh>> {
        let st = if real {
            self.inv.slots.get(crate::inventory::OFFHAND)?.clone()?
        } else {
            self.off_shown.clone()?
        };
        let (name, cx) = crate::handitem::stack_ctx(&st)?;
        let rm = self.hand_rm.as_ref()?;
        self.hand_items.as_mut()?.get(rm, name, &cx)
    }

    /// Контекст предмета в основной руке: добавляет тики натяжения лука.
    fn use_ctx(&self, mut cx: crate::itemicon::Ctx) -> crate::itemicon::Ctx {
        if let Some(u) = self.using.as_ref().filter(|u| u.bow) {
            cx.using = Some((u.duration - u.remaining).clamp(0, 30) as u8);
        }
        cx
    }

    fn shown_mesh(&mut self) -> Option<std::sync::Arc<crate::handitem::HandMesh>> {
        let (stack, _) = self.hand_shown.as_ref()?;
        let (name, cx) = crate::handitem::stack_ctx(stack)?;
        let cx = self.use_ctx(cx);
        let rm = self.hand_rm.as_ref()?;
        self.hand_items.as_mut()?.get(rm, name, &cx)
    }

    /// Чужие игроки (TODO-211, часть): модель Стива, анимации по тикам `game_time`.
    /// VANILLA-UNSURE: приседание/плавание/скины/слой плаща по метаданным не разобраны.
    /// VANILLA: ItemEntityRenderer#submit (+ submitMultipleFromCount) — предметы на земле:
    /// покачивание `sin(age/10+bob)·0.1+0.1`, вращение `age/20+bob`, копии по количеству, тень 0.15/0.75.
    /// VANILLA: FlameFeatureRenderer#prepare для горящих сущностей (предметы, игроки).
    fn bake_flames(&mut self, out: &mut Vec<u8>) {
        use ezzmc_render::particledraw::push_vertex_raw;
        let Some((fire0, fire1)) = self.fire_uv else {
            return;
        };
        let Ok(guard) = self.net.level.lock() else {
            return;
        };
        let Some(level) = guard.as_ref() else {
            return;
        };
        let item_ty = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
            .iter()
            .position(|n| *n == "item");
        let player_ty = ezzmc_world::entities::player_type();
        let pt = level.partial_tick();
        // Поворот вокруг Y к камере: локальная +Z → назад от взгляда.
        let yaw = self.yaw.to_radians();
        let (sy, cy) = (yaw.sin(), yaw.cos());
        let ax = [-cy, 0.0, -sy];
        let az = [sy, 0.0, -cy];
        for e in level.entities.iter() {
            if e.flags & 1 == 0 {
                continue;
            }
            let (bw, bh) = if Some(e.ty) == item_ty {
                (0.25f32, 0.25f32)
            } else if Some(e.ty) == player_ty {
                (0.6, 1.8)
            } else if let Some(sz) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                .get(e.ty)
                .and_then(|n| ezzmc_world::entity_size::size(n))
            {
                // VANILLA: EntityRenderer#submit → displayFireAnimation для любых сущностей.
                sz
            } else {
                continue;
            };
            let lerp = |a: f64, b: f64| a + (b - a) * f64::from(pt);
            let pos = [
                lerp(e.prev_pos[0], e.pos[0]),
                lerp(e.prev_pos[1], e.pos[1]),
                lerp(e.prev_pos[2], e.pos[2]),
            ];
            let rel = [
                (pos[0] - self.cam[0]) as f32,
                (pos[1] - self.cam[1]) as f32,
                (pos[2] - self.cam[2]) as f32,
            ];
            if rel[0] * rel[0] + rel[1] * rel[1] + rel[2] * rel[2] > 64.0 * 64.0 {
                continue;
            }
            let (_, sky) = crate::particles::light_at(level, pos[0], pos[1] + 0.2, pos[2]);
            let light = (240u16, u16::from(sky) * 16);
            let s = bw * 1.4;
            let mut r = 0.5f32;
            let mut h = bh / s;
            let mut yo = 0.0f32;
            let mut zo = 0.3 - (h as i32) as f32 * 0.02;
            let mut n = 0;
            while h > 0.0 {
                let t = if n % 2 == 0 { fire0 } else { fire1 };
                let (mut u0, mut u1) = (t.u0, t.u1);
                if (n / 2) % 2 == 0 {
                    std::mem::swap(&mut u0, &mut u1);
                }
                let v = |x: f32, y: f32| {
                    let (lx, ly, lz) = (x * s, y * s, zo * s);
                    [
                        rel[0] + ax[0] * lx + az[0] * lz,
                        rel[1] + ly,
                        rel[2] + ax[2] * lx + az[2] * lz,
                    ]
                };
                let c = [255; 4];
                push_vertex_raw(out, v(-r, 0.0 - yo), [u1, t.v1], c, light);
                push_vertex_raw(out, v(r, 0.0 - yo), [u0, t.v1], c, light);
                push_vertex_raw(out, v(r, 1.4 - yo), [u0, t.v0], c, light);
                push_vertex_raw(out, v(-r, 1.4 - yo), [u1, t.v0], c, light);
                h -= 0.45;
                yo -= 0.45;
                r *= 0.9;
                zo -= 0.03;
                n += 1;
            }
            self.entity_item_visible = true;
        }
    }

    fn bake_item_entities(&mut self, shadow: &mut Vec<u8>) {
        use ezzmc_world::noise::LegacyRandom;
        let Some(item_ty) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
            .iter()
            .position(|n| *n == "item")
        else {
            return;
        };
        struct Drop {
            id: i32,
            pos: [f64; 3],
            age: f32,
            light: (u8, u8),
            stack: ezzmc_protocol::item::ItemStack,
        }
        let t_collect = std::time::Instant::now();
        let mut drops = Vec::new();
        {
            let Ok(guard) = self.net.level.lock() else {
                return;
            };
            let Some(level) = guard.as_ref() else {
                return;
            };
            let pt = level.partial_tick();
            for e in level.entities.iter().filter(|e| e.ty == item_ty) {
                let Some(stack) = e.item.clone() else {
                    continue;
                };
                let lerp = |a: f64, b: f64| a + (b - a) * f64::from(pt);
                let pos = [
                    lerp(e.prev_pos[0], e.pos[0]),
                    lerp(e.prev_pos[1], e.pos[1]),
                    lerp(e.prev_pos[2], e.pos[2]),
                ];
                let d2 = (pos[0] - self.cam[0]).powi(2)
                    + (pos[1] - self.cam[1]).powi(2)
                    + (pos[2] - self.cam[2]).powi(2);
                if d2 > 64.0 * 64.0 {
                    continue;
                }
                // VANILLA: EntityRenderer#shouldRender — AABB (0.25×0.25) .inflate(0.5) против Frustum.
                if let Some(fr) = self.entity_frustum.as_ref() {
                    let lo = [pos[0] - 0.625, pos[1] - 0.5, pos[2] - 0.625];
                    let hi = [pos[0] + 0.625, pos[1] + 0.75, pos[2] + 0.625];
                    if !fr.is_visible(lo, hi) {
                        continue;
                    }
                }
                crate::entityshadow::bake(
                    level,
                    pos,
                    self.cam,
                    0.15,
                    0.75,
                    self.sky_darken,
                    shadow,
                );
                drops.push(Drop {
                    id: e.id,
                    pos,
                    age: e.age as f32 + pt,
                    light: crate::particles::light_at(level, pos[0], pos[1] + 0.2, pos[2]),
                    stack,
                });
            }
            // VANILLA: ItemPickupParticleGroup — предмет летит к цели за 3 тика, (life+pt)/3 в квадрате.
            for p in &level.pickups {
                let f = f64::from((p.life as f32 + pt) / 3.0).powi(2);
                let lerp = |a: f64, b: f64| a + (b - a) * f64::from(pt);
                let target = match level.entities.map.get(&p.target) {
                    Some(t) => {
                        let ty = lerp(t.prev_pos[1], t.pos[1]);
                        [
                            lerp(t.prev_pos[0], t.pos[0]),
                            (ty + ty + 1.62) / 2.0,
                            lerp(t.prev_pos[2], t.pos[2]),
                        ]
                    }
                    None => {
                        let eye = self.player.eye_lerp(f64::from(pt));
                        let ty = lerp(self.player.prev_pos[1], self.player.pos[1]);
                        [
                            lerp(self.player.prev_pos[0], self.player.pos[0]),
                            (ty + eye[1]) / 2.0,
                            lerp(self.player.prev_pos[2], self.player.pos[2]),
                        ]
                    }
                };
                let pos = [0, 1, 2].map(|i| p.pos[i] + (target[i] - p.pos[i]) * f);
                drops.push(Drop {
                    id: p.id,
                    pos,
                    age: p.age,
                    light: crate::particles::light_at(level, pos[0], pos[1] + 0.2, pos[2]),
                    stack: p.stack.clone(),
                });
            }
        }
        crate::mobs::prof_phase(17, "дропы: сбор (тень+свет)", t_collect.elapsed());
        let t_bake = std::time::Instant::now();
        let n_drops = drops.len();
        let _prof_bake = ProfDrop(t_bake, n_drops);
        // Фаза 1 (последовательно): меши и матрицы всех копий каждого дропа.
        self.item_inst_frame.clear();
        let mut mesh_ix: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        let mut bounds: Vec<(f32, f32)> = Vec::new();
        let mut frame: Vec<ItemInstFrame> = Vec::new();
        for d in drops {
            let Some(rm) = self.hand_rm.as_ref() else {
                return;
            };
            let Some((name, cx)) = crate::handitem::stack_ctx(&d.stack) else {
                continue;
            };
            let Some(mesh) = self.hand_items.as_mut().and_then(|h| h.get(rm, name, &cx)) else {
                continue;
            };
            self.entity_item_visible = true;
            let foil = foil_of(&d.stack);
            let mi = *mesh_ix
                .entry(std::sync::Arc::as_ptr(&mesh) as usize)
                .or_insert_with(|| {
                    bounds.push(mesh.ground_bounds());
                    frame.push(ItemInstFrame {
                        mesh: mesh.clone(),
                        normal: Vec::new(),
                        foil: Vec::new(),
                    });
                    frame.len() - 1
                });
            let (min_y, depth) = bounds[mi];
            let bob_offs =
                LegacyRandom::new(i64::from(d.id)).next_float() * std::f32::consts::PI * 2.0;
            let bob = (d.age / 10.0 + bob_offs).sin() * 0.1 + 0.1;
            let spin = d.age / 20.0 + bob_offs;
            let base = Mat4::from_translation(Vec3::new(
                (d.pos[0] - self.cam[0]) as f32,
                (d.pos[1] - self.cam[1]) as f32 + bob - min_y + 0.0625,
                (d.pos[2] - self.cam[2]) as f32,
            )) * Mat4::from_rotation_y(spin);
            // VANILLA: ItemClusterRenderState#getRenderedAmount / getSeedForItemStack.
            let count = match d.stack.count {
                c if c >= 48 => 5,
                c if c >= 32 => 4,
                c if c >= 16 => 3,
                c if c > 1 => 2,
                _ => 1,
            };
            let mut rng = LegacyRandom::new(i64::from(d.stack.item));
            let ground = frame[mi].mesh.ground_matrix();
            let mut push = |m: Mat4| {
                let f = &mut frame[mi];
                let v = if foil { &mut f.foil } else { &mut f.normal };
                v.extend_from_slice(&(m * ground).to_cols_array());
                v.extend_from_slice(&[f32::from(d.light.0), f32::from(d.light.1), 0.0, 0.0]);
            };
            if depth > 0.0625 {
                push(base);
                for _ in 1..count {
                    let o = Vec3::new(
                        (rng.next_float() * 2.0 - 1.0) * 0.15,
                        (rng.next_float() * 2.0 - 1.0) * 0.15,
                        (rng.next_float() * 2.0 - 1.0) * 0.15,
                    );
                    push(base * Mat4::from_translation(o));
                }
            } else {
                let off_z = depth * 1.5;
                let mut m = base
                    * Mat4::from_translation(Vec3::new(
                        0.0,
                        0.0,
                        -(off_z * (count - 1) as f32 / 2.0),
                    ));
                push(m);
                m *= Mat4::from_translation(Vec3::new(0.0, 0.0, off_z));
                for _ in 1..count {
                    let o = Vec3::new(
                        (rng.next_float() * 2.0 - 1.0) * 0.075,
                        (rng.next_float() * 2.0 - 1.0) * 0.075,
                        0.0,
                    );
                    push(m * Mat4::from_translation(o));
                    m *= Mat4::from_translation(Vec3::new(0.0, 0.0, off_z));
                }
            }
        }
        self.item_inst_frame = frame;
    }

    /// Стрелы и брошенные предметы. VANILLA: ArrowRenderer#submit (Y yRot−90, Z xRot, ArrowModel);
    /// ThrownItemRenderer#submit (предмет GROUND, повёрнутый в сторону камеры).
    // VANILLA-UNSURE: tipped-стрела (цвет эффекта) не различается — метаданные не разбираются; тряска shake не воспроизводится.
    fn bake_projectiles(&mut self, item_data: &mut (Vec<u8>, Vec<u8>), arrows: &mut Vec<u8>) {
        use ezzmc_protocol::generated::r26_3::ENTITY_TYPE;
        struct P {
            kind: &'static str,
            pos: [f64; 3],
            rot: (f32, f32),
            light: (u8, u8),
            stack: Option<ezzmc_protocol::item::ItemStack>,
            shake: f32,
        }
        let mut list = Vec::new();
        {
            let Ok(guard) = self.net.level.lock() else {
                return;
            };
            let Some(level) = guard.as_ref() else {
                return;
            };
            let pt = level.partial_tick();
            for e in level.entities.iter().filter(|e| e.proj != 0) {
                let Some(name) = ENTITY_TYPE.get(e.ty).copied() else {
                    continue;
                };
                if !matches!(
                    name,
                    "arrow"
                        | "spectral_arrow"
                        | "ender_pearl"
                        | "snowball"
                        | "egg"
                        | "experience_bottle"
                        | "eye_of_ender"
                ) {
                    continue;
                }
                let lerp = |a: f64, b: f64| a + (b - a) * f64::from(pt);
                let pos = [
                    lerp(e.prev_pos[0], e.pos[0]),
                    lerp(e.prev_pos[1], e.pos[1]),
                    lerp(e.prev_pos[2], e.pos[2]),
                ];
                let d2 = (pos[0] - self.cam[0]).powi(2)
                    + (pos[1] - self.cam[1]).powi(2)
                    + (pos[2] - self.cam[2]).powi(2);
                if d2 > 64.0 * 64.0 {
                    continue;
                }
                // VANILLA: EyeOfEnder#shouldRenderAtSqrDistance — первые 2 тика не видно у камеры.
                if name == "eye_of_ender" && e.age < 2 && d2 < 12.25 {
                    continue;
                }
                let lr = |a: f32, b: f32| a + (b - a) * pt;
                let mut light = crate::particles::light_at(level, pos[0], pos[1] + 0.1, pos[2]);
                // VANILLA: ThrownItemRenderer(fullBright = true) у EYE_OF_ENDER — блок-свет 15.
                if name == "eye_of_ender" {
                    light.0 = 15;
                }
                let dy = (e.yaw - e.prev_yaw + 540.0).rem_euclid(360.0) - 180.0;
                list.push(P {
                    kind: name,
                    pos,
                    rot: (e.prev_yaw + dy * pt, lr(e.prev_pitch, e.pitch)),
                    light,
                    stack: e.item.clone(),
                    shake: e.shake as f32 - pt,
                });
            }
        }
        for p in list {
            let rel = Vec3::new(
                (p.pos[0] - self.cam[0]) as f32,
                (p.pos[1] - self.cam[1]) as f32,
                (p.pos[2] - self.cam[2]) as f32,
            );
            if p.kind.ends_with("arrow") {
                let root = Mat4::from_translation(rel)
                    * Mat4::from_rotation_y((p.rot.0 - 90.0).to_radians())
                    * Mat4::from_rotation_z(p.rot.1.to_radians());
                let row = u32::from(p.kind == "spectral_arrow");
                ezzmc_render::entitymodel::bake_arrow(root, row, p.light, p.shake, arrows);
                continue;
            }
            let Some(rm) = self.hand_rm.as_ref() else {
                continue;
            };
            let (name, cx) = match p.stack.as_ref().and_then(crate::handitem::stack_ctx) {
                Some(v) => v,
                // VANILLA: EyeOfEnder#getDefaultItem = ENDER_EYE (у типа и предмета разные имена).
                None if p.kind == "eye_of_ender" => ("ender_eye", crate::itemicon::Ctx::default()),
                None => (p.kind, crate::itemicon::Ctx::default()),
            };
            let Some(mesh) = self.hand_items.as_mut().and_then(|h| h.get(rm, name, &cx)) else {
                continue;
            };
            // VANILLA: camera.orientation = rotationYXZ(π − yRot, −xRot, 0).
            let orient = Mat4::from_euler(
                glam::EulerRot::YXZ,
                std::f32::consts::PI - self.yaw.to_radians(),
                -self.pitch.to_radians(),
                0.0,
            );
            mesh.bake(
                Mat4::from_translation(rel) * orient,
                crate::handitem::View::Ground,
                p.light,
                p.stack.as_ref().is_some_and(foil_of),
                &mut item_data.0,
                &mut item_data.1,
            );
            self.entity_item_visible = true;
        }
    }

    /// VANILLA: DragonFireballRenderer#submit (26.3): scale(2), rotate(camera.orientation), квад
    /// (x − 0.5, y − 0.25, 0) с UV (0,1)(1,1)(1,0)(0,0), entityCutout, блок-свет 15, нормаль (0,1,0).
    fn bake_dragon_fireballs(&mut self) {
        let mut list = Vec::new();
        {
            let Ok(guard) = self.net.level.lock() else {
                return;
            };
            let Some(level) = guard.as_ref() else {
                return;
            };
            let pt = f64::from(level.partial_tick());
            for e in level.entities.iter().filter(|e| {
                ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty) == Some(&"dragon_fireball")
            }) {
                let pos: [f64; 3] =
                    std::array::from_fn(|i| e.prev_pos[i] + (e.pos[i] - e.prev_pos[i]) * pt);
                let mut light = crate::particles::light_at(level, pos[0], pos[1], pos[2]);
                light.0 = 15;
                list.push((pos, light));
            }
        }
        if list.is_empty() {
            return;
        }
        // VANILLA: camera.orientation = rotationYXZ(π − yRot, −xRot, 0).
        let orient = Mat4::from_euler(
            glam::EulerRot::YXZ,
            std::f32::consts::PI - self.yaw.to_radians(),
            -self.pitch.to_radians(),
            0.0,
        );
        let shade =
            ezzmc_render::entitymodel::mix_light(orient.transform_vector3(Vec3::Y).normalize());
        let c = (255.0 * shade).round() as u8;
        let out = self.mobs.out(crate::mobs::DRAGON_FIREBALL_TEX);
        for (pos, light) in list {
            let rel = Vec3::new(
                (pos[0] - self.cam[0]) as f32,
                (pos[1] - self.cam[1]) as f32,
                (pos[2] - self.cam[2]) as f32,
            );
            let m = Mat4::from_translation(rel) * Mat4::from_scale(Vec3::splat(2.0)) * orient;
            for (x, y, u, v) in [
                (0.0, 0.0, 0.0, 1.0),
                (1.0, 0.0, 1.0, 1.0),
                (1.0, 1.0, 1.0, 0.0),
                (0.0, 1.0, 0.0, 0.0),
            ] {
                let p = m.transform_point3(Vec3::new(x - 0.5, y - 0.25, 0.0));
                ezzmc_render::particledraw::push_vertex(
                    out,
                    p.to_array(),
                    [u, v],
                    [c, c, c, 255],
                    light,
                );
            }
        }
    }

    /// Рамки (обычные и светящиеся) с предметом. VANILLA: ItemFrameRenderer#submit/extractRenderState
    /// (26.3): модель блока `item_frame[_map]` (BlockModelResolver#updateForItemFrame), предмет FIXED
    /// ×0.5 с поворотом rotation·45°; у светящейся — свет блока ≥5, предмет полной яркости.
    fn bake_item_frames(&mut self, item_data: &mut (Vec<u8>, Vec<u8>), map_data: &mut Vec<u8>) {
        self.map_labels.clear();
        use ezzmc_protocol::generated::r26_3::ENTITY_TYPE;
        struct F {
            pos: [f64; 3],
            dir: i32,
            glow: bool,
            invisible: bool,
            rotation: i32,
            light: (u8, u8),
            stack: Option<ezzmc_protocol::item::ItemStack>,
        }
        let mut list = Vec::new();
        {
            let Ok(guard) = self.net.level.lock() else {
                return;
            };
            let Some(level) = guard.as_ref() else {
                return;
            };
            for e in level.entities.iter() {
                let glow = match ENTITY_TYPE.get(e.ty).copied() {
                    Some("item_frame") => false,
                    Some("glow_item_frame") => true,
                    _ => continue,
                };
                let d2 = (e.pos[0] - self.cam[0]).powi(2)
                    + (e.pos[1] - self.cam[1]).powi(2)
                    + (e.pos[2] - self.cam[2]).powi(2);
                if d2 > 64.0 * 64.0 {
                    continue;
                }
                let (bl, sl) = crate::particles::light_at(level, e.pos[0], e.pos[1], e.pos[2]);
                list.push(F {
                    pos: e.pos,
                    // VANILLA: HangingEntity#DATA_DIRECTION (#8), иначе data из add_entity.
                    dir: e.int_meta(8).unwrap_or(e.data),
                    glow,
                    invisible: e.flags & 0x20 != 0,
                    rotation: e.int_meta(10).unwrap_or(0),
                    light: (if glow { bl.max(5) } else { bl }, sl),
                    stack: e.item.clone().filter(|s| s.count > 0),
                });
            }
        }
        let Some(rm) = self.hand_rm.as_ref() else {
            return;
        };
        for f in list {
            let step = match f.dir {
                0 => [0.0, -1.0, 0.0],
                1 => [0.0, 1.0, 0.0],
                2 => [0.0, 0.0, -1.0],
                3 => [0.0, 0.0, 1.0],
                4 => [-1.0, 0.0, 0.0],
                _ => [1.0, 0.0, 0.0],
            };
            let (x_rot, y_rot) = match f.dir {
                0 => (90.0f32, 180.0f32),
                1 => (-90.0, 180.0),
                // Direction#toYRot: S 0, W 90, N 180, E 270.
                2 => (0.0, 0.0),
                3 => (0.0, 180.0),
                4 => (0.0, 180.0 - 90.0),
                _ => (0.0, 180.0 - 270.0),
            };
            let o = 0.46875;
            let mut m = Mat4::from_translation(Vec3::new(
                (f.pos[0] - self.cam[0]) as f32 + step[0] * o,
                (f.pos[1] - self.cam[1]) as f32 + step[1] * o,
                (f.pos[2] - self.cam[2]) as f32 + step[2] * o,
            )) * Mat4::from_rotation_x(x_rot.to_radians())
                * Mat4::from_rotation_y(y_rot.to_radians());
            // VANILLA: ItemFrameRenderer#extractRenderState — карта только при MAP_ID и наличии данных.
            let map_id = f
                .stack
                .as_ref()
                .and_then(crate::maps::map_id)
                .filter(|id| self.maps.map.contains_key(id));
            let is_map = map_id.is_some();
            if !f.invisible {
                let model = match (f.glow, is_map) {
                    (false, false) => "minecraft:block/item_frame",
                    (false, true) => "minecraft:block/item_frame_map",
                    (true, false) => "minecraft:block/glow_item_frame",
                    (true, true) => "minecraft:block/glow_item_frame_map",
                };
                if let Some(mesh) = self
                    .hand_items
                    .as_mut()
                    .and_then(|h| h.block_model(rm, model))
                {
                    mesh.bake(
                        m,
                        crate::handitem::View::Ground,
                        f.light,
                        false,
                        &mut item_data.0,
                        &mut item_data.1,
                    );
                    self.entity_item_visible = true;
                }
            }
            m *=
                Mat4::from_translation(Vec3::new(0.0, 0.0, if f.invisible { 0.5 } else { 0.4375 }));
            let Some(stack) = f.stack else { continue };
            if let Some(id) = map_id {
                // VANILLA: ItemFrameRenderer#submit — ветка mapId; свет светящейся рамки 15728850.
                let mm = m
                    * Mat4::from_rotation_z(((f.rotation % 4 * 2) as f32 * 45.0).to_radians())
                    * Mat4::from_rotation_z(180f32.to_radians())
                    * Mat4::from_scale(Vec3::splat(0.0078125))
                    * Mat4::from_translation(Vec3::new(-64.0, -64.0, -1.0));
                let light = if f.glow {
                    (210, 240)
                } else {
                    (u16::from(f.light.0) * 16, u16::from(f.light.1) * 16)
                };
                self.maps
                    .bake(id, mm, true, light, None, map_data, &mut self.map_labels);
                continue;
            }
            let Some((name, cx)) = crate::handitem::stack_ctx(&stack) else {
                continue;
            };
            let Some(mesh) = self.hand_items.as_mut().and_then(|h| h.get(rm, name, &cx)) else {
                continue;
            };
            let m = m
                * Mat4::from_rotation_z((f.rotation as f32 * 45.0).to_radians())
                * Mat4::from_scale(Vec3::splat(0.5));
            let light = if f.glow { (15, 15) } else { f.light };
            mesh.bake(
                m,
                crate::handitem::View::Fixed,
                light,
                foil_of(&stack),
                &mut item_data.0,
                &mut item_data.1,
            );
            self.entity_item_visible = true;
        }
    }

    /// Слот атласа скинов для игрока (VANILLA: SkinManager — пока скин не пришёл, Стив).
    fn skin_slot(
        tab: &crate::tablist::TabList,
        slots: &mut HashMap<String, u32>,
        uploads: &mut Vec<(u32, std::sync::Arc<Vec<u8>>)>,
        uuid: &u128,
    ) -> u32 {
        let Some(url) = tab.entries.get(uuid).and_then(|t| t.skin_url.clone()) else {
            return 0;
        };
        if let Some(&s) = slots.get(&url) {
            return s;
        }
        let sk = crate::skins::global();
        sk.request(&url);
        let next = slots.len() as u32 + 1;
        if next >= SKIN_SLOTS {
            return 0;
        }
        match sk.get(&url) {
            Some(img) => {
                slots.insert(url, next);
                uploads.push((next, img));
                next
            }
            None => 0,
        }
    }

    fn bake_remote_players(
        &mut self,
        player_data: &mut Vec<u8>,
        armor_data: &mut Vec<u8>,
        glint_data: &mut Vec<u8>,
        shadow: &mut Vec<u8>,
        item_data: &mut (Vec<u8>, Vec<u8>),
    ) {
        use ezzmc_render::humanoid::setup_anim;
        let Some(player_ty) = ezzmc_world::entities::player_type() else {
            return;
        };
        let Ok(guard) = self.net.level.lock() else {
            return;
        };
        let Some(level) = guard.as_ref() else {
            return;
        };
        let now = level.game_time;
        let ticks = match self.remote_tick.replace(now) {
            Some(prev) if now >= prev => (now - prev).min(5),
            _ => 0,
        };
        let pt = level.partial_tick();
        self.remote_anims
            .retain(|id, _| level.entities.map.contains_key(id));
        self.nametags.clear();
        for e in level.entities.iter().filter(|e| e.ty == player_ty) {
            let anim = self.remote_anims.entry(e.id).or_default();
            anim.swimming = e.swimming();
            anim.fall_flying = e.fall_flying();
            anim.motion = e.vel;
            // VANILLA: handleAnimate → swing(); один взмах на пакет.
            if anim.swing_seen != e.swing_seq {
                anim.swing_seen = e.swing_seq;
                anim.swing();
            }
            // VANILLA: Entity#isInvisible → LivingEntityRenderer не рисует модель (без брони/предметов).
            // VANILLA: AbstractBoat#clampRotation — тело пассажира лодки = yRot лодки, крутится только голова.
            anim.riding_body = level
                .riders
                .vehicle_of
                .get(&e.id)
                .and_then(|v| level.entities.map.get(v))
                .filter(|v| {
                    let n = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                        .get(v.ty)
                        .copied()
                        .unwrap_or("");
                    n.ends_with("_boat") || n.ends_with("_raft")
                })
                .map(|v| v.yaw);
            // Анимацию тикаем, чтобы при появлении не было скачка.
            for _ in 0..ticks {
                anim.tick(e.pos, e.prev_pos, e.head_yaw, e.pitch);
            }
            let lerp = |a: f64, b: f64| a + (b - a) * f64::from(pt);
            let pos = [
                lerp(e.prev_pos[0], e.pos[0]),
                lerp(e.prev_pos[1], e.pos[1]),
                lerp(e.prev_pos[2], e.pos[2]),
            ];
            let light = crate::particles::light_at(level, pos[0], pos[1] + 1.62, pos[2]);
            // Плавание по общим флагам (Entity#isSwimming); «в воде» — по блоку у ног.
            let swimming = e.swimming();
            let wet = ezzmc_mesh::blockinfo::fluid(level.chunks.block(
                pos[0].floor() as i32,
                pos[1].floor() as i32,
                pos[2].floor() as i32,
            ))
            .is_some_and(|f| !f.lava);
            anim.in_water = wet;
            if e.invisible() {
                continue;
            }
            let crouching = e.crouching() && !swimming;
            // VANILLA: AvatarRenderer#shouldShowName — дальность 64 (присед 32), не у невидимого;
            // якорь EntityAttachment.NAME_TAG = высота хитбокса + 0.5 (1.8 / присед 1.5).
            {
                let d2 = (pos[0] - self.cam[0]).powi(2)
                    + (pos[1] - self.cam[1]).powi(2)
                    + (pos[2] - self.cam[2]).powi(2);
                let range = if crouching { 32.0 } else { 64.0 };
                if d2 < range * range {
                    if let Some(name) = self.tab.profiles.get(&e.uuid) {
                        let sb = &self.scoreboard;
                        let team = sb.team_of(name);
                        // VANILLA: LivingEntityRenderer#shouldShowName — правило nameTagVisibility
                        // команды (0 always, 1 never, 2 hideForOtherTeams, 3 hideForOwnTeam).
                        let show = team.is_none_or(|tm| {
                            let mine = sb.team_of(&self.own_name);
                            let allied = mine.is_some_and(|m| std::ptr::eq(m, tm));
                            match tm.name_tag_visibility {
                                1 => false,
                                2 => mine.is_none() || allied,
                                3 => mine.is_none() || !allied,
                                _ => true,
                            }
                        });
                        if show {
                            let h = if crouching { 1.5 } else { 1.8 };
                            let anchor = [pos[0], pos[1] + h + 0.5, pos[2]];
                            // VANILLA: Player#getDisplayName — PlayerTeam.formatNameForTeam.
                            let text = ezzmc_protocol::text::Component::text(name.clone());
                            let text = match team {
                                Some(tm) => tm.format_name(text),
                                None => text,
                            };
                            // VANILLA: Entity#belowNameDisplay — ближе BELOW_NAME_DISTANCE (10).
                            let score = (d2 < 100.0).then(|| sb.below_name(name)).flatten();
                            self.nametags.push(NameTag {
                                anchor,
                                text,
                                discrete: crouching,
                                score,
                            });
                        }
                    }
                }
            }
            let mut state = anim.humanoid_state(pt, crouching);
            if level.riders.vehicle_of.contains_key(&e.id) {
                state.passenger = true;
                state.walk_pos = 0.0;
                state.walk_speed = 0.0;
            }
            let mut mesh_of = |st: &Option<ezzmc_protocol::item::ItemStack>| {
                let (name, cx) = crate::handitem::stack_ctx(st.as_ref()?)?;
                let rm = self.hand_rm.as_ref()?;
                self.hand_items.as_mut()?.get(rm, name, &cx)
            };
            let right_mesh = mesh_of(&e.held);
            let left_mesh = mesh_of(&e.held_off);
            state.item_right = e.held.is_some();
            state.item_left = e.held_off.is_some();
            let body = anim.body_rot(pt);
            let swim = SwimPose {
                amount: state.swim_amount,
                in_water: wet,
                visually: swimming,
                x_rot: state.x_rot,
                fly: anim.fly_pose(pt),
            };
            let bed_facing = (e.pose == POSE_SLEEPING)
                .then_some(e.sleep_pos)
                .flatten()
                .map(|p| level.chunks.block(p[0], p[1], p[2]))
                .filter(|s| s.name().ends_with("_bed"))
                .and_then(|s| s.property("facing"));
            let root = match bed_facing {
                Some(f) => self.avatar_root_sleeping(pos, f),
                None => self.avatar_root(pos, body, crouching, swim),
            };
            let slot = Self::skin_slot(
                &self.tab,
                &mut self.skin_slots,
                &mut self.skin_uploads,
                &e.uuid,
            );
            let slim = self.tab.entries.get(&e.uuid).is_some_and(|t| t.skin_slim);
            let (model, parts) = if slim {
                &mut self.player_model_slim
            } else {
                &mut self.player_model
            };
            crate::settings::apply_parts(model, 0x7F);
            setup_anim(model, parts, &state);
            // Скин игрока: слот атласа по URL из свойства `textures` (нет/не скачан — Стив, слот 0).
            let first = player_data.len();
            model.bake(root, [255; 4], light, player_data);
            for v in player_data[first..].chunks_exact_mut(32) {
                let t = f32::from_le_bytes([v[16], v[17], v[18], v[19]]);
                let t = (slot as f32 + t) / SKIN_SLOTS as f32;
                v[16..20].copy_from_slice(&t.to_le_bytes());
            }
            if let Some(n) = e.armor[0].as_ref().and_then(crate::skull::worn_name) {
                let head = model.part_matrices(root)[parts.head];
                self.head_skulls
                    .push((n, crate::skull::worn_root(head), light));
            }
            let looks = crate::armor::looks_of_entity(&e.armor);
            self.armor_kit.fall_flying = e.fall_flying();
            self.armor_kit.bake(
                &looks, &state, root, [255; 4], light, armor_data, glint_data,
            );
            crate::entityshadow::bake(level, pos, self.cam, 0.5, 1.0, self.sky_darken, shadow);
            // VANILLA: ItemInHandLayer — рука, X−90°, Y180°, translate(±1/16, 2/16, −10/16).
            let (rf, lf) = (
                e.held.as_ref().is_some_and(foil_of),
                e.held_off.as_ref().is_some_and(foil_of),
            );
            for (mesh, arm_idx, dx, view, foil) in [
                (
                    right_mesh,
                    parts.right_arm,
                    1.0,
                    crate::handitem::View::ThirdPerson,
                    rf,
                ),
                (
                    left_mesh,
                    parts.left_arm,
                    -1.0,
                    crate::handitem::View::ThirdPersonLeft,
                    lf,
                ),
            ] {
                if let Some(mesh) = mesh {
                    let arm = slim_hand_matrix(model.part_matrices(root)[arm_idx], root, slim, dx)
                        * Mat4::from_rotation_x((-90.0f32).to_radians())
                        * Mat4::from_rotation_y(180.0f32.to_radians())
                        * Mat4::from_translation(Vec3::new(dx / 16.0, 0.125, -0.625));
                    mesh.bake(arm, view, light, foil, &mut item_data.0, &mut item_data.1);
                    self.entity_item_visible = true;
                }
            }
        }
    }

    fn render_view(
        &mut self,
        gpu: &Gpu,
        target: &wgpu::TextureView,
        w: u32,
        h: u32,
        clear: [f64; 3],
    ) {
        let t_rv = std::time::Instant::now();
        // TODO-149: цвет неба по времени суток и погоде (sky_state); пока без диска/светил,
        // поэтому он же служит фоном и цветом тумана.
        // Цвет фона/тумана теперь даёт compute_fog (FogRenderer); тут — небо и lightmap.
        let _sky_color = match self.net.level.lock().as_deref() {
            Ok(Some(l)) => {
                // VANILLA: биомный атрибут visual/sky_color в точке камеры.
                // VANILLA: EnvironmentAttributeProbe — гауссово смешивание биомов (probe_colors).
                let base_sky = self.probe_colors[0];
                let mut st =
                    ezzmc_render::sky::sky_state(l.day_time, l.rain_level, l.thunder(), base_sky);
                // VANILLA: ClientLevel#addEnvironmentAttributeLayers — вспышка молнии:
                // SKY_COLOR = srgbLerp(0.22, sky, (0.8, 0.8, 1.0)), SKY_LIGHT_FACTOR = 1.
                // VANILLA-UNSURE: цвет тумана (compute_fog) вспышку не учитывает.
                if l.sky_flash_time > 0 && !self.cfg.hide_lightning_flashes {
                    st.sky_color = ezzmc_render::sky::srgb_lerp(0.22, st.sky_color, 0xFFCC_CCFF);
                    st.sky_light_factor = 1.0;
                }
                let partial = l.partial_tick();
                let prev = ezzmc_render::sky::sky_state(
                    l.prev_day_time,
                    l.rain_level,
                    l.thunder(),
                    base_sky,
                );
                let drawn = ezzmc_render::sky::lerp_angles(&prev, &st, partial);
                let c = st.sky_color;
                // VANILLA: LightmapRenderStateExtractor — sky_factor/sky_light_color из атрибутов времени/погоды.
                // VANILLA-UNSURE: Level#skyDarken = 15 − SKY_LIGHT_LEVEL; уровень выводим из sky_light_factor
                // (день 1.0 → 15, ночь 0.24 → 4).
                self.sky_darken = 15
                    - (4.0 + (st.sky_light_factor - 0.24) / 0.76 * 11.0)
                        .round()
                        .clamp(4.0, 15.0) as i32;
                // VANILLA: без дневного таймлайна (Nether/End) SKY_LIGHT_* и AMBIENT берутся из атрибутов
                // измерения; у Overworld ambient_light_color = #0a0a0a и ambient_light 0 — это дефолты.
                let day_cycle = l.skybox == ezzmc_world::dimension::Skybox::Overworld;
                let dim = l.dimension.as_ref();
                let rgb01 = |c: u32| {
                    [
                        ((c >> 16) & 255) as f32 / 255.0,
                        ((c >> 8) & 255) as f32 / 255.0,
                        (c & 255) as f32 / 255.0,
                    ]
                };
                let (sky_factor, sky_light_color) = if day_cycle {
                    (st.sky_light_factor, st.sky_light_color)
                } else {
                    (
                        dim.and_then(|d| d.attr_f32("visual/sky_light_factor"))
                            .unwrap_or(1.0),
                        dim.and_then(|d| d.attr_color("visual/sky_light_color"))
                            .map_or([1.0; 3], rgb01),
                    )
                };
                // VANILLA: CardinalLighting по cardinal_light измерения (затенение граней в Nether).
                self.models.set_nether_shading(dim.is_some_and(|d| {
                    d.cardinal_light == ezzmc_world::dimension::CardinalLight::Nether
                }));
                let ambient_light = dim.map_or(0.0, |d| d.ambient_light);
                let ambient_color = dim
                    .and_then(|d| d.attr_color("visual/ambient_light_color"))
                    .map_or([10.0 / 255.0; 3], rgb01);
                // VANILLA: LightmapRenderStateExtractor#extract — эффекты Darkness/NightVision/Conduit.
                // VANILLA: GameRenderer#extract вызывает lightmap extract(state, 1.0F), а не покадровый pt:
                // иначе (tickCount - pt) идёт назад внутри тика и скачет на новом (пила вместо плавной волны).
                let pt = 1.0;
                let darkness_opt = self.cfg.darkness_effect;
                let darkness_effect = self.effects.blend_factor("darkness", pt) * darkness_opt;
                let darkness_scale = ezzmc_render::lightmap::darkness_scale(
                    self.effects.ticks as f32,
                    pt,
                    darkness_effect,
                ) * darkness_opt;
                let night_vision = if self.effects.has("night_vision") {
                    self.effects.night_vision_scale(pt)
                } else if self.fog_state.water_vision(self.player.under_water) > 0.0
                    && self.effects.has("conduit_power")
                {
                    self.fog_state.water_vision(self.player.under_water)
                } else {
                    0.0
                };
                // VANILLA: LightmapRenderStateExtractor — bossOverlayWorldDarkening(partialTicks).
                let darken_world = self.boss_darken(pt);
                let key = (
                    sky_factor,
                    sky_light_color,
                    self.cfg.gamma,
                    ambient_light,
                    ambient_color,
                    [darkness_effect, darkness_scale, night_vision, darken_world],
                );
                if self.lightmap_key != Some(key) {
                    self.lightmap_key = Some(key);
                    self.lightmap.update(
                        &gpu.queue,
                        &LightmapParams {
                            sky_factor,
                            sky_light_color,
                            gamma: key.2,
                            ambient_light,
                            ambient_color,
                            darkness_effect,
                            darkness_scale,
                            night_vision,
                            darken_world,
                            ..LightmapParams::default()
                        },
                    );
                }
                // VANILLA: небо рисуется только при skybox=overworld (Nether: none, End: end_sky — TODO).
                // VANILLA: GameRenderer#renderLevel — shouldRenderSky = !shouldCreateWorldFog.
                self.sky_state = (l.skybox == ezzmc_world::dimension::Skybox::Overworld
                    && !self.boss.world_fog())
                .then_some(drawn);
                self.end_sky =
                    l.skybox == ezzmc_world::dimension::Skybox::End && !self.boss.world_fog();
                [
                    f64::from((c >> 16) & 255) / 255.0,
                    f64::from((c >> 8) & 255) / 255.0,
                    f64::from(c & 255) / 255.0,
                ]
            }
            _ => {
                self.sky_state = None;
                self.end_sky = false;
                clear
            }
        };
        self.ensure_depth(gpu, w, h);
        let aspect = w as f32 / h.max(1) as f32;
        // VANILLA: ChunkMap#setServerViewDistance — сервер меняет радиус (смена renderDistance в настройках).
        if let Ok(Some(l)) = self.net.level.lock().as_deref() {
            // Туман и дальняя плоскость: не дальше, чем сервер присылает (min серверного и renderDistance клиента).
            self.view_radius = l.chunks.view_radius().min(self.cfg.render_distance);
        }
        // VANILLA: Camera#update — depthFar = max(renderDistance*16*4, cloudRange*16); cloudRange 64 (fancy).
        let far = ((self.view_radius.max(2) as f32) * 16.0 * 4.0).max(64.0 * 16.0);
        // glam perspective_rh: глубина клипа 0..1 (wgpu).
        let proj = Mat4::perspective_rh(self.fov_deg().to_radians(), aspect, NEAR, far);
        let mv = look_matrix(self.yaw, self.pitch);
        // VANILLA: bobHurt/bobView входят в проекцию мира — надписи/неймтеги обязаны их разделять.
        self.nt_view = (
            proj * self.bob((self.tick_acc / 0.05) as f32) * mv,
            self.cam,
            proj.y_axis.y,
        );
        // VANILLA: CloudRenderer#prepare (cloudHeight = visual/cloud_height, дефолт 192.33; cloudRange 64 (fancy), FANCY).
        if let Ok(Some(l)) = self.net.level.lock().as_deref() {
            let cloud_height = l
                .dimension
                .as_ref()
                .and_then(|d| d.attr_f32("visual/cloud_height"))
                .unwrap_or(192.33);
            let cc = ezzmc_render::sky::cloud_color(l.day_time, l.rain_level, l.thunder());
            if cc >> 24 > 0 && l.skybox == ezzmc_world::dimension::Skybox::Overworld {
                self.clouds.prepare(
                    &gpu.device,
                    &gpu.queue,
                    mv,
                    cc,
                    if self.cfg.graphics == 0 {
                        ezzmc_render::clouddraw::CloudStatus::Fast
                    } else {
                        ezzmc_render::clouddraw::CloudStatus::Fancy
                    },
                    cloud_height,
                    64,
                    self.cam,
                    l.game_time,
                    l.partial_tick(),
                );
            } else {
                self.clouds.hide();
            }
        }
        // VANILLA: WeatherEffectRenderer#extractRenderState + prepare.
        {
            let cols = match self.net.level.lock().as_deref() {
                Ok(Some(l)) if l.skybox == ezzmc_world::dimension::Skybox::Overworld => Some((
                    crate::weather::collect(
                        l,
                        &self.models.tints.biomes,
                        self.cam,
                        crate::weather::WEATHER_RADIUS,
                        l.partial_tick(),
                    ),
                    l.rain_level,
                )),
                _ => None,
            };
            let (c, intensity) = cols.unwrap_or_default();
            self.weather.prepare(
                &gpu.device,
                &gpu.queue,
                mv,
                self.cam,
                &c.rain,
                &c.snow,
                crate::weather::WEATHER_RADIUS,
                intensity,
            );
        }
        // VANILLA: ParticleEngine#render — квады частиц (оси камеры из yaw/pitch).
        {
            let (y, p) = (self.yaw.to_radians(), self.pitch.to_radians());
            let right = [-y.cos(), 0.0, -y.sin()];
            let fwd = [-y.sin() * p.cos(), -p.sin(), y.cos() * p.cos()];
            let up = [
                right[1] * fwd[2] - right[2] * fwd[1],
                right[2] * fwd[0] - right[0] * fwd[2],
                right[0] * fwd[1] - right[1] * fwd[0],
            ];
            let quads = match self.net.level.lock().as_deref() {
                Ok(Some(l)) => self.particles.extract(l, self.cam, l.partial_tick()),
                _ => Vec::new(),
            };
            self.particle_draw
                .prepare(&gpu.device, &gpu.queue, mv, right, up, &quads);
        }
        // VANILLA: TheEndPortalRenderer / TheEndGatewayRenderer.
        {
            let versions = (self.queue.version, self.queue.block_version);
            let mut verts = std::mem::take(&mut self.end_portal_verts);
            let (gt, pt) = match self.net.level.lock().as_deref() {
                Ok(Some(l)) => {
                    self.end_portals.build(l, self.cam, versions, &mut verts);
                    (l.game_time, l.partial_tick())
                }
                _ => {
                    verts.clear();
                    (0, 0.0)
                }
            };
            self.end_portal_draw
                .prepare(&gpu.device, &gpu.queue, mv, &verts, gt, pt);
            self.end_portal_verts = verts;
        }
        // VANILLA: BeaconRenderer.
        {
            let versions = (self.queue.version, self.queue.block_version);
            let mut batches = std::mem::take(&mut self.beacon_batches);
            batches.iter_mut().flatten().for_each(Vec::clear);
            if let Ok(Some(l)) = self.net.level.lock().as_deref() {
                self.beacons.build(l, self.cam, versions, &mut batches);
            }
            self.beacon_draw
                .prepare(&gpu.device, &gpu.queue, mv, &batches);
            self.beacon_batches = batches;
        }
        // VANILLA: WorldBorderRenderer#extract/prepare — renderDistance = опция·16, depthFar.
        {
            let rd = f64::from(self.view_radius.max(2)) * 16.0;
            let st = match self.net.level.lock().as_deref() {
                Ok(Some(l)) => {
                    ezzmc_render::borderdraw::extract(&l.border, l.partial_tick(), self.cam, rd)
                }
                _ => Default::default(),
            };
            let millis = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_millis() as u64);
            self.border_draw
                .prepare(&gpu.queue, &st, self.cam, rd, far, mv, millis);
        }
        crate::mobs::prof_at(7, "t до prepare_hand");
        self.prepare_hand(gpu, aspect, far);
        // VANILLA: AvatarRenderer — игрок виден только от 3-го лица (в 1-м лице камера в голове).
        let mut fr = Frustum::new(proj, mv, true);
        fr.prepare(self.cam[0], self.cam[1], self.cam[2]);
        self.entity_frustum = Some(fr);
        self.prepare_player_model(gpu, mv);
        // После мобов: лучи смерти дракона собираются в Mobs::bake.
        // VANILLA: LightningBoltRenderer — сущности молний (позиции относительно камеры).
        {
            let bolts: Vec<_> = match self.net.level.lock().as_deref() {
                Ok(Some(l)) if !l.bolts.list.is_empty() => l
                    .bolts
                    .list
                    .iter()
                    .map(|b| ezzmc_render::lightningdraw::BoltInstance {
                        pos: b.pos,
                        seed: b.seed,
                    })
                    .collect(),
                _ => Vec::new(),
            };
            let rays = std::mem::take(&mut self.mobs.dragon_rays);
            self.lightning
                .prepare(&gpu.device, &gpu.queue, mv, self.cam, &bolts, &rays);
        }
        let q = &gpu.queue;
        q.write_buffer(
            &self.projection_buf,
            0,
            &ProjectionUniform {
                proj_mat: if self.walking {
                    proj * self.bob((self.tick_acc / 0.05) as f32)
                } else {
                    proj
                },
            }
            .to_bytes(),
        );
        let fog = self.compute_fog(aspect, clear);
        q.write_buffer(&self.fog_buf, 0, &fog.to_bytes());
        // Compute-путь (F8) рисует своим шейдером — ему туман нужен отдельно.
        if let Some(v) = &self.vox {
            v.set_fog(q, &fog);
        }
        let bp = [
            self.cam[0].floor() as i32,
            self.cam[1].floor() as i32,
            self.cam[2].floor() as i32,
        ];
        // terrain.wgsl: pos = local + (origin − camera_block_pos) + camera_offset → offset = −frac(cam).
        let off = [
            (f64::from(bp[0]) - self.cam[0]) as f32,
            (f64::from(bp[1]) - self.cam[1]) as f32,
            (f64::from(bp[2]) - self.cam[2]) as f32,
        ];
        q.write_buffer(
            &self.globals_buf,
            0,
            &GlobalsUniform {
                camera_block_pos: bp,
                camera_offset: off,
                screen_size: [w as f32, h as f32],
                glint_alpha: 0.0,
                game_time: 0.0,
                menu_blur_radius: 0,
                use_rgss: 0,
            }
            .to_bytes(),
        );
        q.write_buffer(
            &self.chunk_buf,
            0,
            &ChunkSectionUniform {
                model_view_mat: mv,
                chunk_visibility: 1.0,
                texture_size: [self.atlas_size.0 as i32, self.atlas_size.1 as i32],
                chunk_position: [0, 0, 0],
            }
            .to_bytes(),
        );
        let vp = (proj * mv).to_cols_array().map(f32::to_bits);
        let key = (vp, self.cam.map(f64::to_bits), self.arena.revision());
        if self.draw_key != Some(key) {
            let mut fr = Frustum::new(proj, mv, true);
            fr.prepare(self.cam[0], self.cam[1], self.cam[2]);
            // VANILLA-UNSURE: в ванили пересортировка идёт асинхронно в воркерах; здесь — не больше
            // RESORT_PER_FRAME секций за кадр (остальные добираются в следующих: needs_resort остаётся истинным).
            // Отсечение, сортировка по расстоянию и сбор команд — по плотному массиву слотов, без хеширования.
            let (n_visible, exhausted) =
                self.arena
                    .cull_prepare(q, self.cam, RESORT_PER_FRAME, |x, y, z| {
                        fr.section_visible(x, y, z)
                    });
            self.visible_sections = n_visible;
            // Бюджет пересортировки исчерпан — на следующем кадре продолжаем, кэш не фиксируем.
            self.draw_key = (!exhausted).then_some(key);
        }
        let Some(depth) = self.depth.as_ref().map(|d| d.0.clone()) else {
            return;
        };
        crate::mobs::prof_at(10, "t до encoder");
        let compute = self.use_compute && self.vox.is_some();
        // VANILLA: GameRenderer#renderLevel — projection.mul(bobHurt·bobView): качание входит в матрицу
        // всего мира, в том числе compute-пути (MDI/Hi-Z), контуров и отладочных линий.
        let view_proj = if self.walking {
            proj * self.bob((self.tick_acc / 0.05) as f32) * mv
        } else {
            proj * mv
        };
        let mut enc = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        if compute {
            if self.hiz.as_ref().is_none_or(|z| z.size() != (w, h)) {
                self.hiz = Some(HiZ::new(&gpu.device, w, h));
                self.hiz_valid = false;
            }
            let (pvp, ppos) = match (self.hiz_valid, self.prev) {
                (true, Some(p)) => p,
                _ => (view_proj, self.cam),
            };
            let cam = CullCamera {
                view_proj,
                pos: self.cam,
                prev_view_proj: pvp,
                prev_pos: ppos,
            };
            let hiz = if self.hiz_valid && std::env::var("EZZMC_NOHIZ").is_err() {
                self.hiz.as_ref()
            } else {
                None
            };
            if let Some(v) = &self.vox {
                v.encode_cull(gpu, &mut enc, &cam, hiz);
            }
        }
        let color_att = |load: wgpu::LoadOp<wgpu::Color>| wgpu::RenderPassColorAttachment {
            view: target,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load,
                store: wgpu::StoreOp::Store,
            },
        };
        let depth_att = |load: wgpu::LoadOp<f32>| wgpu::RenderPassDepthStencilAttachment {
            view: &depth,
            depth_ops: Some(wgpu::Operations {
                load,
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };
        let sky = wgpu::Color {
            r: f64::from(fog.color[0]),
            g: f64::from(fog.color[1]),
            b: f64::from(fog.color[2]),
            a: 1.0,
        };
        // Compute-режим: проход A (непрозрачное) → Hi-Z из глубины БЕЗ прозрачных слоёв → проход B
        // (Translucent/Tripwire поверх). Иначе вода в глубине «закрыла» бы секции за ней.
        let first_layers: &[RenderLayer] = if compute {
            &[RenderLayer::Solid, RenderLayer::Cutout]
        } else {
            &RenderLayer::ALL
        };
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("world"),
                color_attachments: &[Some(color_att(wgpu::LoadOp::Clear(sky)))],
                depth_stencil_attachment: Some(depth_att(wgpu::LoadOp::Clear(1.0))),
                ..Default::default()
            });
            // VANILLA: LevelRenderer#addSkyPass — не при Blindness/Darkness.
            if let Some(st) = self
                .sky_state
                .as_ref()
                .filter(|_| !self.effects.blocks_sky())
            {
                // VANILLA: SkyRenderer#shouldRenderDarkDisc — глаза ниже getHorizonHeight
                // (ClientLevelData: flat ? minY : 63) и не под водой.
                let horizon = if self.is_flat {
                    f64::from(self.min_y)
                } else {
                    63.0
                };
                let dark_disc = self.cam[1] < horizon && !self.player.under_water;
                self.sky.prepare(&gpu.queue, mv, st, dark_disc);
                self.sky.draw(&mut pass);
            }
            // VANILLA: SkyRenderer#renderEndSky (skybox=end); вспышка End flash — TODO.
            if self.end_sky && !self.effects.blocks_sky() {
                self.sky.draw_end(&gpu.queue, &mut pass, mv);
            }
            if compute {
                if let Some(v) = &self.vox {
                    v.draw(&mut pass);
                }
            }
            pass.set_bind_group(0, &self.group0, &[]);
            pass.set_bind_group(1, &self.group1, &[]);
            // VANILLA: сущности рисуются ДО прозрачного слоя террейна (иначе вода, пишущая
            // глубину, скрывает предметы под ней).
            for &layer in first_layers {
                if !compute && layer == RenderLayer::Translucent {
                    self.draw_entities(&mut pass);
                }
                if self.arena.draw_count(layer) == 0 {
                    continue;
                }
                pass.set_pipeline(self.pipes.pipeline(layer));
                self.arena.draw(&mut pass, layer);
            }
            if compute {
                // Compute: Solid/Cutout уже нарисованы — сущности и частицы тут же (до воды).
                self.draw_entities(&mut pass);
            } else {
                self.draw_post_world(&gpu.queue, &mut pass);
            }
        }
        let nohiz = std::env::var("EZZMC_NOHIZ").is_ok();
        if compute {
            if let Some(z) = self.hiz.as_ref().filter(|_| !nohiz) {
                z.build(&gpu.device, &mut enc, &depth);
            }
            let rest = [RenderLayer::Translucent, RenderLayer::Tripwire];
            {
                let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("world translucent"),
                    color_attachments: &[Some(color_att(wgpu::LoadOp::Load))],
                    depth_stencil_attachment: Some(depth_att(wgpu::LoadOp::Load)),
                    ..Default::default()
                });
                pass.set_bind_group(0, &self.group0, &[]);
                pass.set_bind_group(1, &self.group1, &[]);
                for layer in rest {
                    if self.arena.draw_count(layer) == 0 {
                        continue;
                    }
                    pass.set_pipeline(self.pipes.pipeline(layer));
                    self.arena.draw(&mut pass, layer);
                }
                self.draw_post_world(&gpu.queue, &mut pass);
            }
            self.prev = Some((view_proj, self.cam));
            self.hiz_valid = true;
        }
        // VANILLA: LevelRenderer#submitBlockOutline — чёрный контур ARGB.black(102), толщина
        // Window#getAppropriateLineWidth = max(2.5, ширина/1920·2.5), поверх террейна.
        if let Some(t) = self.target {
            let off = [
                (f64::from(t.pos[0]) - self.cam[0]) as f32,
                (f64::from(t.pos[1]) - self.cam[1]) as f32,
                (f64::from(t.pos[2]) - self.cam[2]) as f32,
            ];
            let o = ezzmc_mesh::blockinfo::outline_offset(t.state, t.pos[0], t.pos[2]);
            let off = [off[0] + o[0], off[1] + o[1], off[2] + o[2]];
            let segs: Vec<_> = ezzmc_mesh::blockinfo::outline_boxes(t.state)
                .iter()
                .flat_map(|b| ezzmc_render::lines::box_edges(b, off))
                .collect();
            let width = (w as f32 / 1920.0 * 2.5).max(2.5);
            // VANILLA: боббинг входит в матрицу проекции — контур качается вместе с миром.
            let outline_vp = view_proj;
            // VANILLA: LevelRenderer#submitBlockOutline — highContrast: чёрная подложка (ширина 7)
            // и цвет 0xFF57FFE1 (-11010079) вместо чёрного 40%.
            let hc = self.cfg.high_contrast_outline;
            if hc {
                self.outline_back.set(
                    &gpu.queue,
                    outline_vp,
                    [w as f32, h as f32],
                    [0.0, 0.0, 0.0, 1.0],
                    7.0,
                    &segs,
                );
            } else {
                self.outline_back.clear();
            }
            self.lines.set(
                &gpu.queue,
                outline_vp,
                [w as f32, h as f32],
                if hc {
                    [0x57 as f32 / 255.0, 1.0, 0xE1 as f32 / 255.0, 1.0]
                } else {
                    [0.0, 0.0, 0.0, 0.4]
                },
                width,
                &segs,
            );
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("outline"),
                color_attachments: &[Some(color_att(wgpu::LoadOp::Load))],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            self.outline_back.draw(&mut pass);
            self.lines.draw(&mut pass);
        } else {
            self.outline_back.clear();
            self.lines.clear();
        }
        if self.show_hitboxes {
            // VANILLA: EntityHitboxDebugRenderer#showHitboxes — белый бокс, красный слой на высоте глаз
            // (LivingEntity), зелёные части дракона, синий вектор взгляда длиной 2.
            let segs = self.hitbox_segments();
            let colors = [
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0, 1.0],
                [0.25, 1.0, 0.0, 1.0],
            ];
            let width = (w as f32 / 1920.0 * 2.5).max(2.5);
            for i in 0..4 {
                self.hit_lines[i].set(
                    &gpu.queue,
                    view_proj,
                    [w as f32, h as f32],
                    colors[i],
                    width,
                    &segs[i],
                );
            }
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("hitboxes"),
                color_attachments: &[Some(color_att(wgpu::LoadOp::Load))],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            for l in &self.hit_lines {
                l.draw(&mut pass);
            }
        }
        if self.show_chunk_borders {
            let segs = self.chunk_border_segments();
            let colors = [
                [1.0, 0.0, 0.0, 0.5],
                [1.0, 1.0, 0.0, 1.0],
                [0.0, 155.0 / 255.0, 155.0 / 255.0, 1.0],
                [0.25, 0.25, 1.0, 1.0],
                [0.25, 0.25, 1.0, 1.0],
            ];
            let k = (w as f32 / 1920.0).max(1.0);
            let widths = [4.0 * k, k, k, 4.0 * k, k];
            for i in 0..5 {
                self.border_lines[i].set(
                    &gpu.queue,
                    view_proj,
                    [w as f32, h as f32],
                    colors[i],
                    widths[i],
                    &segs[i],
                );
            }
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("chunk borders"),
                color_attachments: &[Some(color_att(wgpu::LoadOp::Load))],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            for l in &self.border_lines {
                l.draw(&mut pass);
            }
        }
        if let (Some(gs), 0) = (self.debug_crosshair, self.perspective) {
            // VANILLA: DebugCrosshairRenderer#render — оси мира на расстоянии 1 перед глазами,
            // длина 0.01·guiScale; чёрная обводка (ширина 4) под цветными осями (ширина 2).
            let d = look_dir(self.yaw, self.pitch);
            let s = 0.01 * gs as f32;
            let axes = [[s, 0.0, 0.0], [0.0, s, 0.0], [0.0, 0.0, s]];
            let seg = |a: [f32; 3]| (d, [d[0] + a[0], d[1] + a[1], d[2] + a[2]]);
            let screen = [w as f32, h as f32];
            let black: Vec<_> = axes.iter().map(|a| seg(*a)).collect();
            // VANILLA: GameRenderer#render3dHud — прицел рисуется с hud-проекцией (bobHurt/bobView
            // входят только в проекцию мира), поэтому качание камеры его не двигает.
            let view_proj = proj * mv;
            let colors = [
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 1.0],
                [0.5, 0.5, 1.0, 1.0],
            ];
            self.cross_lines[0].set(
                &gpu.queue,
                view_proj,
                screen,
                [0.0, 0.0, 0.0, 1.0],
                4.0,
                &black,
            );
            for i in 0..3 {
                self.cross_lines[i + 1].set(
                    &gpu.queue,
                    view_proj,
                    screen,
                    colors[i],
                    2.0,
                    &[seg(axes[i])],
                );
            }
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("3d crosshair"),
                color_attachments: &[Some(color_att(wgpu::LoadOp::Load))],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            for l in &self.cross_lines {
                l.draw(&mut pass);
            }
        }
        if (self.hand_visible || self.hand_item_visible || self.hand_map_visible)
            && !self.hud_hidden
            && !self.is_scoping()
        {
            // VANILLA: рука рисуется после мира с очищенной глубиной.
            let hand_depth = self.hand_depth.as_ref().unwrap_or(&depth);
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("hand"),
                color_attachments: &[Some(color_att(wgpu::LoadOp::Load))],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: hand_depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_bind_group(0, &self.group0, &[]);
            pass.set_bind_group(1, &self.group1, &[]);
            if self.hand_visible {
                self.hand_draw.draw(&mut pass);
            }
            if self.hand_map_visible {
                self.hand_map_draw.draw(&mut pass);
            }
            if let (true, Some(d)) = (self.hand_item_visible, &self.hand_item_draw) {
                d.draw(&mut pass);
                if let Some(g) = &self.hand_glint_draw {
                    g.draw(&mut pass);
                }
            }
        }
        crate::mobs::prof_at(8, "t до submit");
        let t_sub = std::time::Instant::now();
        gpu.queue.submit([enc.finish()]);
        crate::mobs::prof_phase(2, "submit", t_sub.elapsed());
        crate::mobs::prof_phase(1, "render_view всего", t_rv.elapsed());
        let _ = &self.lightmap;
    }

    /// VANILLA: KeyboardHandler (F3+A) — LevelExtractor#allChanged: перестроить все чанки.
    pub fn reload_chunks(&mut self) {
        let all: Vec<_> = self.resident.iter().copied().collect();
        for p in all {
            self.queue.mark(p);
        }
        self.debug_feedback("Reloaded chunks");
    }

    /// VANILLA: KeyboardHandler (F3+C) — текст `/execute in <dim> run tp @s x y z yaw pitch`.
    pub fn location_command(&self) -> String {
        let p = self.player.pos;
        format!(
            "/execute in {} run tp @s {:.2} {:.2} {:.2} {:.2} {:.2}",
            crate::itemdyn::dimension(),
            p[0],
            p[1],
            p[2],
            self.yaw,
            self.pitch
        )
    }

    /// VANILLA: KeyboardHandler#debugFeedback — «[Debug]: <текст>» (жёлтый жирный префикс) в чат.
    pub fn debug_feedback(&mut self, text: &str) {
        let mut c = ezzmc_protocol::text::Component::text("[Debug]:");
        c.style.color = Some(0xFFFF55);
        c.style.bold = Some(true);
        c.siblings
            .push(ezzmc_protocol::text::Component::text(format!(" {text}")));
        self.chat_in.push((c, Some(crate::chat::Tag::System)));
    }

    /// VANILLA: KeyboardHandler (F3+B) — переключить хитбоксы и написать в чат «[Debug]: Hitboxes: shown/hidden».
    pub fn toggle_hitboxes(&mut self) {
        self.show_hitboxes = !self.show_hitboxes;
        self.debug_feedback(if self.show_hitboxes {
            "Hitboxes: shown"
        } else {
            "Hitboxes: hidden"
        });
    }

    /// VANILLA: ChunkBorderRenderer#emitGizmos — линии секции под камерой (координаты относительно камеры).
    fn chunk_border_segments(&self) -> [Vec<([f32; 3], [f32; 3])>; 5] {
        let mut out: [Vec<([f32; 3], [f32; 3])>; 5] = Default::default();
        let Ok(g) = self.net.level.lock() else {
            return out;
        };
        let Some(l) = g.as_ref() else {
            return out;
        };
        let f = l.chunks.format();
        let (ymin, ymax) = (f.min_y, f.min_y + f.height);
        let px = self.player.pos;
        let xs = ((px[0].floor() as i32) >> 4) * 16;
        let zs = ((px[2].floor() as i32) >> 4) * 16;
        let sy = (px[1].floor() as i32) >> 4;
        let rel = |x: i32, y: i32, z: i32| {
            [
                (f64::from(x) - self.cam[0]) as f32,
                (f64::from(y) - self.cam[1]) as f32,
                (f64::from(z) - self.cam[2]) as f32,
            ]
        };
        let vline = |x: i32, z: i32| (rel(x, ymin, z), rel(x, ymax, z));
        for x in [-16, 0, 16, 32] {
            for z in [-16, 0, 16, 32] {
                out[0].push(vline(xs + x, zs + z));
            }
        }
        for x in (2..16).step_by(2) {
            let i = if x % 4 == 0 { 2 } else { 1 };
            out[i].push(vline(xs + x, zs));
            out[i].push(vline(xs + x, zs + 16));
            out[i].push(vline(xs, zs + x));
            out[i].push(vline(xs + 16, zs + x));
        }
        let ring = |o: &mut Vec<_>, y: i32| {
            o.push((rel(xs, y, zs), rel(xs, y, zs + 16)));
            o.push((rel(xs, y, zs + 16), rel(xs + 16, y, zs + 16)));
            o.push((rel(xs + 16, y, zs + 16), rel(xs + 16, y, zs)));
            o.push((rel(xs + 16, y, zs), rel(xs, y, zs)));
        };
        for y in (ymin..=ymax).step_by(2) {
            let i = if y % 8 == 0 { 2 } else { 1 };
            ring(&mut out[i], y);
        }
        for x in [0, 16] {
            for z in [0, 16] {
                out[3].push(vline(xs + x, zs + z));
            }
        }
        for y in (ymin..=ymax).step_by(16) {
            ring(&mut out[3], y);
        }
        // Куб секции под игроком.
        let (y0, y1) = (sy * 16, sy * 16 + 16);
        for y in [y0, y1] {
            ring(&mut out[4], y);
        }
        for x in [0, 16] {
            for z in [0, 16] {
                out[4].push((rel(xs + x, y0, zs + z), rel(xs + x, y1, zs + z)));
            }
        }
        out
    }

    /// VANILLA: KeyboardHandler (F3+N) — creative↔spectator через change_game_mode (нужен op ≥ 2).
    pub fn debug_spectate(&mut self) {
        if self.op_level < 2 {
            self.debug_feedback("Unable to switch game mode, no permission");
        } else if self.game_mode != 3 {
            self.send(crate::netthread::NetCmd::ChangeGameMode(3));
        } else {
            let m = if self.prev_game_mode < 0 {
                1
            } else {
                self.prev_game_mode
            };
            self.send(crate::netthread::NetCmd::ChangeGameMode(m));
        }
    }

    /// VANILLA: KeyboardHandler (F3+G) — границы чанков.
    pub fn toggle_chunk_borders(&mut self) {
        self.show_chunk_borders = !self.show_chunk_borders;
        self.debug_feedback(if self.show_chunk_borders {
            "Chunk borders: shown"
        } else {
            "Chunk borders: hidden"
        });
    }

    /// VANILLA: EntityHitboxDebugRenderer — отрезки (относительно камеры) по цветам:
    /// [бокс, глаза, взгляд, части дракона]; ближайшие сущности в пределах 64 блоков.
    fn hitbox_segments(&self) -> [Vec<([f32; 3], [f32; 3])>; 4] {
        let mut out: [Vec<([f32; 3], [f32; 3])>; 4] = Default::default();
        let Ok(g) = self.net.level.lock() else {
            return out;
        };
        let Some(l) = g.as_ref() else {
            return out;
        };
        let pt = f64::from(l.partial_tick());
        let rel = |p: [f64; 3]| {
            [
                (p[0] - self.cam[0]) as f32,
                (p[1] - self.cam[1]) as f32,
                (p[2] - self.cam[2]) as f32,
            ]
        };
        let mut ents: Vec<_> = l
            .entities
            .map
            .values()
            .map(|e| {
                let d = [
                    e.pos[0] - self.cam[0],
                    e.pos[1] - self.cam[1],
                    e.pos[2] - self.cam[2],
                ];
                (d[0] * d[0] + d[1] * d[1] + d[2] * d[2], e)
            })
            .filter(|(d, _)| *d < 64.0 * 64.0)
            .collect();
        ents.sort_by(|a, b| a.0.total_cmp(&b.0));
        for (_, e) in ents.into_iter().take(60) {
            let Some((lo, hi)) = e.hitbox() else { continue };
            let Some(name) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty) else {
                continue;
            };
            let cur = [
                e.prev_pos[0] + (e.pos[0] - e.prev_pos[0]) * pt,
                e.prev_pos[1] + (e.pos[1] - e.prev_pos[1]) * pt,
                e.prev_pos[2] + (e.pos[2] - e.prev_pos[2]) * pt,
            ];
            let sh = [cur[0] - e.pos[0], cur[1] - e.pos[1], cur[2] - e.pos[2]];
            let b = [
                lo[0] + sh[0],
                lo[1] + sh[1],
                lo[2] + sh[2],
                hi[0] + sh[0],
                hi[1] + sh[1],
                hi[2] + sh[2],
            ];
            let off = rel([0.0; 3]);
            out[0].extend(ezzmc_render::lines::box_edges(&b, off));
            // VANILLA-UNSURE: eyeHeight = 0.85·высота (EntityDimensions#defaultEyeHeight), без точечных
            // переопределений типов; игрок — по позе.
            let height = hi[1] - lo[1];
            let eye = if *name == "player" {
                if e.flags & 16 != 0 {
                    0.4
                } else if e.flags & 1 != 0 {
                    1.27
                } else {
                    1.62
                }
            } else {
                height * 0.85
            };
            if is_living_type(name) {
                let eb = [b[0], b[1] + eye - 0.01, b[2], b[3], b[1] + eye + 0.01, b[5]];
                out[1].extend(ezzmc_render::lines::box_edges(&eb, off));
            }
            if *name == "ender_dragon" {
                for (_, pb) in self
                    .mobs
                    .dragon_parts()
                    .iter()
                    .filter(|(id, _)| (e.id + 1..=e.id + 8).contains(id))
                {
                    out[3].extend(ezzmc_render::lines::box_edges(pb, off));
                }
            }
            let yaw = if is_living_type(name) {
                e.head_yaw
            } else {
                e.yaw
            };
            let d = look_dir(yaw, e.pitch);
            let eye_pos = [cur[0], b[1] + eye, cur[2]];
            let a = rel(eye_pos);
            out[2].push((a, [a[0] + d[0] * 2.0, a[1] + d[1] * 2.0, a[2] + d[2] * 2.0]));
        }
        out
    }

    /// VANILLA: FogRenderer#setupFog — тип среды по камере, атрибуты биома, цвет/дистанции.
    /// Сущности/частицы слоя OPAQUE — до прозрачного террейна (общий код ванильного и compute-пути).
    fn draw_entities(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.entity_draw.draw(pass);
        self.player_draw.draw(pass);
        self.armor_draw.draw(pass);
        self.glint_draw.draw(pass);
        self.mobs.draw(pass);
        self.arrow_draw.draw(pass);
        self.map_draw.draw(pass);
        self.piston_draw.draw(pass);
        self.shadow_draw.draw(pass);
        if let (true, Some(d)) = (self.entity_item_visible, &self.entity_item_draw) {
            if let (Some(i), Some(g)) = (&self.item_inst, &self.entity_glint_draw) {
                i.draw(pass, d, g);
            }
            d.draw(pass);
            if let Some(g) = &self.entity_glint_draw {
                g.draw(pass);
            }
        }
        // VANILLA: частицы слоя OPAQUE (почти все) рисуются ДО прозрачного террейна —
        // вода просвечивает над ними (glow кальмара под водой).
        self.particle_draw.draw(pass);
        pass.set_bind_group(0, &self.group0, &[]);
        pass.set_bind_group(1, &self.group1, &[]);
    }

    /// Молнии, облака, погода, виньетка — после прозрачного слоя террейна.
    fn draw_post_world(&mut self, queue: &wgpu::Queue, pass: &mut wgpu::RenderPass<'_>) {
        // VANILLA: облака рисуются после прозрачного слоя террейна.
        self.end_portal_draw.draw(pass);
        self.beacon_draw.draw(pass, false);
        self.beacon_draw.draw(pass, true);
        self.lightning.draw(pass);
        self.clouds.draw(pass);
        self.weather.draw(pass);
        // VANILLA: LevelRenderer — граница мира сразу после погоды.
        self.border_draw.draw(pass);
        // VANILLA: Hud#extractVignette (опция vignette:true) — поверх мира.
        self.vignette
            .prepare(queue, self.vignette_b, self.vignette_warn);
        self.vignette.draw(pass);
    }

    fn compute_fog(&mut self, aspect: f32, clear: [f64; 3]) -> FogUniform {
        use ezzmc_render::fog::{setup_fog, FogAttrs, FogInput};
        let now = std::time::Instant::now();
        let dt = self
            .last_frame
            .replace(now)
            .map_or(0.0, |t| now.duration_since(t).as_secs_f32());
        let (bx, by, bz) = (
            self.cam[0].floor() as i32,
            self.cam[1].floor() as i32,
            self.cam[2].floor() as i32,
        );
        let guard = self.net.level.lock();
        let Ok(Some(l)) = guard.as_deref() else {
            return FogUniform {
                color: [clear[0] as f32, clear[1] as f32, clear[2] as f32, 1.0],
                environmental_start: 0.0,
                environmental_end: 1.0e9,
                render_distance_start: 1.0e9,
                render_distance_end: 1.0e9,
                sky_end: 1.0e9,
                clouds_end: 1.0e9,
            };
        };
        let fog_type = fluid_in_camera(l, self.cam, self.yaw, self.pitch, aspect);
        self.cam_fluid = fog_type;
        let mut attrs = FogAttrs::default();
        // VANILLA: атрибуты измерения (visual/fog_*_distance; у Overworld их нет → дефолты).
        if let Some(d) = l.dimension.as_ref() {
            attrs.fog_start = d
                .attr_f32("visual/fog_start_distance")
                .unwrap_or(attrs.fog_start);
            attrs.fog_end = d
                .attr_f32("visual/fog_end_distance")
                .unwrap_or(attrs.fog_end);
        }
        let day_cycle = l.skybox == ezzmc_world::dimension::Skybox::Overworld;
        let mut sky_light = 15u8;
        let mut block_light = 0u8;
        if let Some(c) = l.chunks.get(bx >> 4, bz >> 4) {
            let (lx, lz) = ((bx & 15) as usize, (bz & 15) as usize);
            sky_light = c.sky_light(lx, by, lz).unwrap_or(15);
            block_light = c.block_light(lx, by, lz).unwrap_or(0);
        }
        // VANILLA: Camera#tick → EnvironmentAttributeProbe#tick раз в тик; getValue лерпит между
        // прошлым и новым значением по partialTick.
        self.probe_acc += dt * 20.0;
        let cam = self.cam;
        let sample = |me: &Self| probe_colors(l, &me.models.tints.biomes, cam);
        let mut pair = self.probe_pair.unwrap_or_else(|| {
            let v = sample(self);
            (v, v)
        });
        while self.probe_acc >= 1.0 {
            self.probe_acc -= 1.0;
            pair = (pair.1, sample(self));
        }
        self.probe_pair = Some(pair);
        let t = self.probe_acc.clamp(0.0, 1.0);
        for k in 0..3 {
            self.probe_colors[k] =
                ezzmc_render::sky::srgb_lerp(t, pair.0[k], pair.1[k]) & 0x00FF_FFFF;
        }
        attrs.base_sky = self.probe_colors[0];
        attrs.base_fog = self.probe_colors[1];
        attrs.water_fog_color = self.probe_colors[2];
        // VANILLA: Hud#updateVignetteBrightness — раз в тик, по getMaxLocalRawBrightness камеры.
        self.vignette_acc += dt * 20.0;
        let raw = i32::from(block_light)
            .max(i32::from(sky_light) - ezzmc_render::vignette::sky_darken(l.day_time));
        while self.vignette_acc >= 1.0 {
            self.vignette_acc -= 1.0;
            self.vignette_b = ezzmc_render::vignette::tick_brightness(self.vignette_b, raw, 0.0);
        }
        // VANILLA: Hud#extractVignette — borderWarningStrength по расстоянию до границы мира.
        self.vignette_warn = l.border.warning_strength(self.cam[0], self.cam[2]);
        // VANILLA-UNSURE: Biome#hasPrecipitation не разбирается из реестра → считаем true;
        // WATER_FOG_END ×0.85 у болот/мангров не применён; биомная интерполяция атрибутов не делалась.
        self.fog_state
            .update_rain(l.rain_level, sky_light, true, dt * 20.0);
        let f = look_dir(self.yaw, self.pitch);
        let pt = (self.tick_acc / 0.05).clamp(0.0, 1.0) as f32;
        let in_water = matches!(fog_type, ezzmc_render::fog::FogType::Water);
        self.fog_state.tick_water_vision(in_water, dt * 20.0, false);
        let water_vision = self.fog_state.water_vision(in_water);
        setup_fog(
            &FogInput {
                effects: ezzmc_render::fog::FogEffects {
                    blindness: self.effects.get("blindness").map(|e| e.duration),
                    darkness: self
                        .effects
                        .has("darkness")
                        .then(|| self.effects.blend_factor("darkness", pt)),
                    night_vision: self.effects.fog_night_vision(pt),
                },
                fog_type,
                attrs,
                day_time: l.day_time,
                rain: l.rain_level,
                thunder: l.thunder(),
                view_chunks: self.view_radius.max(2),
                forward_x: f[0],
                spectator: false,
                cam_y: self.cam[1],
                min_y: self.min_y,
                // VANILLA: ClientLevelData#voidDarknessOnsetRange (flat ? 1 : 32).
                void_darkness_onset: if self.is_flat { 1.0 } else { 32.0 },
                water_vision,
                cloud_range: 64,
                day_cycle,
                darken_world: self.boss_darken(pt),
                boss_fog: self.boss.world_fog(),
            },
            &self.fog_state,
        )
    }

    /// VANILLA: GameRenderer#bossOverlayWorldDarkening(partialTicks).
    pub fn boss_darken(&self, pt: f32) -> f32 {
        self.boss_darken_o + (self.boss_darken - self.boss_darken_o) * pt
    }

    pub fn view_radius(&self) -> i32 {
        self.view_radius
    }

    /// Доступ к уровню под локом (`None`, пока мир не создан).
    pub fn with_level<R>(&self, f: impl FnOnce(&Level) -> R) -> Option<R> {
        let g = self.net.level.lock().ok()?;
        g.as_ref().map(f)
    }

    /// VANILLA: Minecraft#continueAttack → MultiPlayerGameMode#continueDestroyBlock.
    /// VANILLA: Minecraft#startAttack — клик по сущности: ServerboundAttackPacket + swing.
    pub fn click_attack(&mut self) {
        // VANILLA: Minecraft#startAttack — при missTime > 0 клик игнорируется (без взмаха).
        if self.miss_time > 0 {
            return;
        }
        if let Some(id) = self.entity_target {
            // VANILLA: Minecraft#startAttack — ATTACK_RANGE предмета: вне [min, max] удара нет.
            if self.attack_range_ok() {
                // VANILLA: Player#attack → RemotePlayer#hurtClient = true → causeExtraKnockback:
                // удар спринтом (knockbackAttack: sprinting && attackStrengthScale(0.5) > 0.9) гасит свою
                // скорость ×0.6 по XZ и сбрасывает спринт. Для остальных сущностей hurtClient = false.
                // VANILLA-UNSURE: чары Knockback/оружие (getKnockback) не учтены, как и baseDamage > 0.
                let remote_player = self.with_level(|l| {
                    l.entities.iter().find(|e| e.id == id).is_some_and(|e| {
                        ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                            .get(e.ty)
                            .is_some_and(|n| *n == "player")
                    })
                });
                if remote_player == Some(true)
                    && self.player.sprinting
                    && self.attack_strength(0.5) > 0.9
                {
                    self.player.vel[0] *= 0.6;
                    self.player.vel[2] *= 0.6;
                    self.player.sprinting = false;
                }
                let _ = self
                    .net
                    .cmd
                    .send(crate::netthread::NetCmd::Attack { entity_id: id });
                self.attack_ticker = 0;
            }
        } else if self.target.is_none() {
            // Промах: в не-креативе missTime = 10; resetAttackStrengthTicker.
            if self.game_mode != 1 {
                self.miss_time = 10;
            }
            self.attack_ticker = 0;
        }
        if self.target.is_some() && self.entity_target.is_none() {
            self.click_dig();
        }
        // VANILLA: startAttack → player.swing(MAIN_HAND) (+ ServerboundSwingPacket).
        self.player.swing();
        let _ = self.net.cmd.send(crate::netthread::NetCmd::Swing);
    }

    /// VANILLA: LivingEntity#updateUsingItem (клиентская часть): смена предмета → стоп; звук каждые 4 тика.
    fn tick_using(&mut self) {
        let Some(u) = self.using.as_mut() else {
            self.player.using_item = false;
            return;
        };
        let held = self.inv.held().map(|s| s.item);
        if held != Some(u.item) || self.inv.selected != u.slot || self.game_mode == 3 {
            self.stop_using();
            return;
        }
        // VANILLA: Consumable#shouldEmitParticlesAndSounds.
        let used = u.duration - u.remaining;
        if let Some(c) = u.c {
            if u.remaining > 0
                && used > (u.duration as f32 * 0.21875) as i32
                && u.remaining % 4 == 0
            {
                Self::emit_consume_sound(&mut u.rng, c, self.cam);
            }
        }
        if u.remaining > 0 {
            u.remaining -= 1;
        }
    }

    /// VANILLA: Consumable#emitParticlesAndSounds (звук; частицы предмета — отдельная подзадача).
    fn emit_consume_sound(
        rng: &mut ezzmc_world::noise::LegacyRandom,
        c: crate::itemdata::Consume,
        p: [f64; 3],
    ) {
        let (vol, pitch) = if c.drink {
            (0.5, 0.9 + 0.1 * rng.next_float())
        } else {
            let v = if rng.next_boolean() { 0.5 } else { 1.0 };
            (v, 1.0 + (rng.next_float() - rng.next_float()) * 0.2)
        };
        crate::audio::play_at(
            c.sound,
            crate::audio::Cat::Players,
            p[0],
            p[1],
            p[2],
            vol,
            pitch,
        );
    }

    /// VANILLA: LivingEntity#stopUsingItem (локально).
    pub fn stop_using(&mut self) {
        // VANILLA: SpyglassItem#releaseUsing → stopUsing: звук (в любом лице).
        let spy = self.using.is_some()
            && self
                .inv
                .held()
                .and_then(|s| crate::inventory::item_name(s.item))
                .is_some_and(|n| n.trim_start_matches("minecraft:") == "spyglass");
        if spy {
            crate::audio::play_at(
                "item.spyglass.stop_using",
                crate::audio::Cat::Players,
                self.cam[0],
                self.cam[1],
                self.cam[2],
                1.0,
                1.0,
            );
        }
        self.using = None;
        self.player.using_item = false;
    }

    /// VANILLA: entity event 9 → LivingEntity#completeUsingItem: звук завершения и стоп.
    fn complete_using(&mut self) {
        if let Some(mut u) = self.using.take() {
            if let Some(c) = u.c {
                Self::emit_consume_sound(&mut u.rng, c, self.cam);
            }
        }
        self.player.using_item = false;
    }

    /// VANILLA: MultiPlayerGameMode#releaseUsingItem — пакет RELEASE_USE_ITEM + локальный стоп.
    pub fn release_using(&mut self) {
        let _ = self.net.cmd.send(crate::netthread::NetCmd::PlayerAction {
            action: 5,
            pos: [0, 0, 0],
            face: 0,
            seq: 0,
        });
        self.stop_using();
    }

    /// VANILLA: ItemInHandRenderer#tick — высота руки тянется к attackStrength³ (или 0 при смене предмета).
    // VANILLA-UNSURE: re-equip при смене типа предмета или слота хотбара.
    fn tick_hand(&mut self) {
        let cur = self.inv.held().cloned();
        let sel = self.inv.selected;
        let same = match (&self.hand_shown, &cur) {
            (None, None) => true,
            (Some((a, s)), Some(b)) => a.item == b.item && *s == sel,
            _ => false,
        };
        let f = self.attack_strength(1.0);
        let target = if same { f * f * f } else { 0.0 };
        let p = &mut self.player;
        p.hand_h_o = p.hand_h;
        p.hand_h += (target - p.hand_h).clamp(-0.4, 0.4);
        if same {
            self.hand_shown = cur.map(|c| (c, sel));
        } else if p.hand_h < 0.1 {
            self.hand_shown = cur.map(|c| (c, sel));
        }
        // VANILLA: FirstPersonHandsAndItems#tick, вторая рука: цель 1, при смене предмета 0.
        let next = self
            .inv
            .slots
            .get(crate::inventory::OFFHAND)
            .cloned()
            .flatten();
        let same = self.off_shown.as_ref().map(|s| s.item) == next.as_ref().map(|s| s.item);
        let (_, h) = self.off_h;
        let nh = h + ((if same { 1.0 } else { 0.0 }) - h).clamp(-0.4, 0.4);
        self.off_h = (h, nh);
        if same || nh < 0.1 {
            self.off_shown = next;
        }
    }

    /// VANILLA: Player#getCurrentItemAttackStrengthDelay = 1 / attack_speed * 20.
    /// Скорость — атрибут `attack_speed` с сервера (модификаторы предмета в руке уже учтены);
    /// до первого пакета — запасная таблица по имени предмета.
    pub fn attack_delay(&self) -> f32 {
        if let Some(v) = self.attack_speed.filter(|v| *v > 0.0) {
            return 1.0 / v * 20.0;
        }
        let name = self
            .inv
            .held()
            .and_then(|s| ezzmc_protocol::item::ClientRegistry::Item.name_of(s.item))
            .map_or("", |n| n.rsplit(':').next().unwrap_or(n));
        let speed = match name {
            n if n.ends_with("_sword") => 1.6,
            n if n.ends_with("_pickaxe") => 1.2,
            n if n.ends_with("_shovel") => 1.0,
            "wooden_axe" | "stone_axe" => 0.8,
            "iron_axe" => 0.9,
            "diamond_axe" | "golden_axe" | "netherite_axe" | "copper_axe" => 1.0,
            "wooden_hoe" => 1.0,
            "stone_hoe" => 2.0,
            "iron_hoe" => 3.0,
            "trident" => 1.1,
            "mace" => 0.6,
            _ => 4.0,
        };
        1.0 / speed * 20.0
    }

    /// VANILLA: AttackRange#isInRange для компонента ATTACK_RANGE предмета в руке (копья);
    /// без компонента — всегда true. Значения копий — default components 26.3 (одинаковы у всех).
    pub fn attack_range_ok(&self) -> bool {
        let spear = self
            .inv
            .held()
            .and_then(|s| ezzmc_protocol::item::ClientRegistry::Item.name_of(s.item))
            .is_some_and(|n| n.ends_with("_spear"));
        if !spear {
            return true;
        }
        let (min, max, margin) = if self.game_mode == 1 {
            (2.0, 6.5, 0.125)
        } else {
            (2.0, 4.5, 0.125)
        };
        self.entity_dist >= min - margin && self.entity_dist <= max + margin
    }

    /// VANILLA: Hud#extractCrosshair — цель LivingEntity и isAlive (смерть = entity_event 3).
    pub fn target_living_alive(&self) -> bool {
        let Some(id) = self.entity_target else {
            return false;
        };
        let Ok(g) = self.net.level.lock() else {
            return false;
        };
        self.attack_range_ok()
            && g.as_ref().is_some_and(|l| {
                l.entities.iter().find(|e| e.id == id).is_some_and(|e| {
                    e.death_at.is_none()
                        && !e.dead
                        && ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                            .get(e.ty)
                            .is_some_and(|n| is_living_type(n))
                })
            })
    }

    /// VANILLA: Player#getAttackStrengthScale.
    pub fn attack_strength(&self, partial: f32) -> f32 {
        ((self.attack_ticker as f32 + partial) / self.attack_delay()).clamp(0.0, 1.0)
    }

    pub fn send(&self, cmd: crate::netthread::NetCmd) {
        let _ = self.net.cmd.send(cmd);
    }

    /// Клик по слоту: предсказание + пакет с заявленным результатом (слоты и курсор), как в ванили.
    pub fn click_slot(&mut self, slot: i16, button: i8, mode: i32) {
        let before: Vec<_> = self.inv.menu_slots().to_vec();
        let state_id = self.inv.state_id;
        self.inv.predict_click(slot, button, mode);
        let changed = self.changed_since(&before);
        self.send_click_claims(state_id, slot, button, mode, changed);
    }

    fn changed_since(&self, before: &[crate::inventory::Slot]) -> Vec<(i16, Option<(u32, i32)>)> {
        self.inv
            .menu_slots()
            .iter()
            .enumerate()
            .filter(|(i, s)| before.get(*i) != Some(*s))
            .map(|(i, s)| (i as i16, crate::invscreen::claim(s.as_ref())))
            .collect()
    }

    fn send_click_claims(
        &self,
        state_id: i32,
        slot: i16,
        button: i8,
        input: i32,
        changed: Vec<(i16, Option<(u32, i32)>)>,
    ) {
        self.send(crate::netthread::NetCmd::ContainerClick {
            container: self.inv.open.as_ref().map_or(0, |o| o.id),
            state_id,
            slot,
            button,
            input,
            changed,
            carried: crate::invscreen::claim(self.inv.cursor.as_ref()),
        });
    }

    /// Перетаскивание стека (QUICK_CRAFT): start / add×N / end + предсказание.
    pub fn quick_craft(&mut self, slots: &[usize], ty: u8) {
        use crate::invscreen::{click_cmd, OUTSIDE, QUICK_CRAFT};
        let t = (ty as i8) << 2;
        // Начало и добавления: курсор ещё не изменён (как в ванили).
        self.send(click_cmd(&self.inv, OUTSIDE, t, QUICK_CRAFT));
        for s in slots {
            self.send(click_cmd(&self.inv, *s as i16, t | 1, QUICK_CRAFT));
        }
        let before: Vec<_> = self.inv.menu_slots().to_vec();
        let state_id = self.inv.state_id;
        self.inv.predict_quick_craft(slots, ty);
        let changed = self.changed_since(&before);
        self.send_click_claims(state_id, OUTSIDE, t | 2, QUICK_CRAFT, changed);
    }

    /// VANILLA: AnvilScreen#onNameChanged → ServerboundRenameItemPacket.
    /// VANILLA: MultiPlayerGameMode#handlePlaceRecipe (контейнер 0 — инвентарь игрока).
    pub fn send_place_recipe(&self, recipe: i32, use_max: bool) {
        let id = self.inv.open.as_ref().map_or(0, |o| o.id);
        self.send(crate::netthread::NetCmd::PlaceRecipe(id, recipe, use_max));
    }

    /// VANILLA: RecipeBookComponent#toggleVisibility + sendUpdateSettings (книга крафта).
    pub fn toggle_crafting_book(&mut self, k: crate::recipebook::BookKind) {
        let t = self.recipes.settings.of_mut(k);
        t.open = !t.open;
        self.inv.book_open = self.recipes.settings.open_flags();
        self.send_book_settings(k);
    }

    /// VANILLA: RecipeBookComponent#sendUpdateSettings (тип: 0 крафт, 1 печь, 2 плавильня, 3 коптильня).
    pub fn send_book_settings(&self, k: crate::recipebook::BookKind) {
        let s = self.recipes.settings.of(k);
        self.send(crate::netthread::NetCmd::RecipeBookSettings(
            k.idx() as i32,
            s.open,
            s.filtering,
        ));
    }

    /// VANILLA: ServerboundClientCommandPacket(PERFORM_RESPAWN).
    pub fn send_respawn(&self) {
        self.send(crate::netthread::NetCmd::Respawn);
    }

    pub fn send_rename_item(&self, name: String) {
        self.send(crate::netthread::NetCmd::RenameItem(name));
    }

    /// VANILLA: BundleMouseActions#toggleSelectedBundleItem.
    pub fn send_select_bundle(&self, slot: i32, index: i32) {
        self.send(crate::netthread::NetCmd::SelectBundle(slot, index));
    }

    pub fn send_container_close(&self) {
        let id = self.inv.open.as_ref().map_or(0, |o| o.id);
        self.send(crate::netthread::NetCmd::ContainerClose(id));
    }

    /// VANILLA: Minecraft#handleKeybinds — key.drop: DROP_ITEM (Ctrl: DROP_ALL_ITEMS), не в наблюдателе.
    pub fn drop_item(&mut self, all: bool) {
        if self.game_mode != 3 {
            // VANILLA: Minecraft#handleKeybinds — if (player.drop(ctrl)) player.swing(MAIN_HAND) (локально).
            if self.inv.held().is_some() {
                self.player.swing();
                // VANILLA: LocalPlayer#drop → Inventory#removeFromSelected(all) — предсказание на клиенте.
                let i = crate::inventory::HOTBAR_START + usize::from(self.inv.selected);
                if let Some(st) = self.inv.slots[i].as_mut() {
                    if all || st.count <= 1 {
                        self.inv.slots[i] = None;
                    } else {
                        st.count -= 1;
                    }
                }
            }
            self.send(crate::netthread::NetCmd::PlayerAction {
                action: if all { 3 } else { 4 },
                pos: [0, 0, 0],
                face: 0,
                seq: 0,
            });
        }
    }

    /// VANILLA: key.swapOffhand → SWAP_ITEM_WITH_OFFHAND.
    pub fn swap_offhand(&mut self) {
        if self.game_mode != 3 {
            self.send(crate::netthread::NetCmd::PlayerAction {
                action: 6,
                pos: [0, 0, 0],
                face: 0,
                seq: 0,
            });
        }
    }

    /// VANILLA: Inventory#setSelectedSlot + ClientPacketListener#ensureHasSentCarriedItem.
    pub fn select_slot(&mut self, slot: u8) {
        if slot < 9 && self.inv.selected != slot {
            self.inv.selected = slot;
            self.carried_sent = slot;
            let _ = self
                .net
                .cmd
                .send(crate::netthread::NetCmd::SetCarried(i16::from(slot)));
        }
    }

    /// VANILLA: Inventory#swapPaint — колесо листает хотбар по кругу (вверх = влево).
    pub fn scroll_hotbar(&mut self, dy: f64) {
        let d = if dy > 0.0 {
            1
        } else if dy < 0.0 {
            -1
        } else {
            return;
        };
        let s = (i32::from(self.inv.selected) - d).rem_euclid(9);
        self.select_slot(s as u8);
    }

    /// VANILLA: Minecraft#startUseItem → MultiPlayerGameMode#useItemOn (рука пуста, предсказания нет).
    pub fn use_item(&mut self) {
        // VANILLA: Minecraft#startUseItem — rightClickDelay = 4 при каждом использовании.
        self.right_click_delay = 4;
        if !self.have_pos || self.game_mode == 3 {
            return;
        }
        let held_name = self.inv.held().and_then(|s| {
            ezzmc_protocol::generated::r26_3::ITEM
                .get(s.item as usize)
                .copied()
        });
        let mut consumed = false;
        // VANILLA: Minecraft#startUseItem → MultiPlayerGameMode#interact — ПКМ по сущности
        // (ServerboundInteractPacket с точкой попадания относительно сущности).
        if let Some(id) = self.entity_target {
            if self.entity_dist <= self.entity_reach() {
                let d = look_dir(self.yaw, self.pitch);
                let hit = [
                    self.cam[0] + f64::from(d[0]) * self.entity_dist,
                    self.cam[1] + f64::from(d[1]) * self.entity_dist,
                    self.cam[2] + f64::from(d[2]) * self.entity_dist,
                ];
                let info = self.with_level(|l| {
                    l.entities.map.get(&id).map(|e| {
                        let n = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                            .get(e.ty)
                            .copied()
                            .unwrap_or("");
                        (e.pos, n.ends_with("_boat") || n.ends_with("_raft"))
                    })
                });
                if let Some(Some((epos, vehicle))) = info {
                    self.send(crate::netthread::NetCmd::Interact {
                        entity_id: id,
                        loc: [hit[0] - epos[0], hit[1] - epos[1], hit[2] - epos[2]],
                        secondary: self.player.crouching,
                    });
                    if vehicle {
                        // VANILLA: AbstractBoat#interact → InteractionResult.SUCCESS → player.swing(hand).
                        self.player.swing();
                        let _ = self.net.cmd.send(crate::netthread::NetCmd::Swing);
                        return;
                    }
                }
            }
        }
        if let Some(t) = self.place_target {
            let seq = self.next_seq();
            let rel = [
                (t.hit[0] - f64::from(t.pos[0])) as f32,
                (t.hit[1] - f64::from(t.pos[1])) as f32,
                (t.hit[2] - f64::from(t.pos[2])) as f32,
            ];
            // DIAG-PLACE: камера кадра и взгляд, с которыми выбран блок (сравнить с sent_* в netthread).
            // DIAG-PLACE: имя состояния и смещения блока (block_offset — рендер, outline_offset — контур).
            let diag_off = ezzmc_mesh::blockinfo::block_offset(t.state, t.pos[0], t.pos[2]);
            let diag_outline = ezzmc_mesh::blockinfo::outline_offset(t.state, t.pos[0], t.pos[2]);
            tracing::info!(
                target: "place",
                "target hit={:.6?} pos={:?} face={} state={} block_offset={:.4?} outline_offset={:.4?} cam={:.6?} frame_yaw={:.4} frame_pitch={:.4}",
                t.hit, t.pos, t.face, t.state.name(), diag_off, diag_outline, self.cam, self.yaw, self.pitch
            );
            let _ = self.net.cmd.send(crate::netthread::NetCmd::UseItemOn {
                pos: t.pos,
                face: t.face,
                rel,
                inside: t.inside,
                seq,
            });
            // VANILLA: MultiPlayerGameMode#useItemOn — Shift + что-то в руках гасит использование блока.
            let have_in_hands =
                held_name.is_some() || self.inv.slots[crate::inventory::OFFHAND].is_some();
            let suppress = self.player.crouching && have_in_hands;
            // VANILLA: RedStoneWireBlock#useWithoutItem — клик только по точке или кресту
            // (isDot/isCross) и только если состояние изменится, иначе PASS (клик не поглощается, в руке блок ставится как обычно).
            let wire_passes = t.state.name() == "redstone_wire" && {
                let changes = self.net.level.lock().ok().is_some_and(|g| {
                    g.as_ref().is_some_and(|l| {
                        let get = |x: i32, y: i32, z: i32| l.chunks.block(x, y, z);
                        crate::support::wire_click_changes(t.state, &get, t.pos)
                    })
                });
                !changes
            };
            // VANILLA: CommandBlock#useWithoutItem — Player#canUseGameMasterBlocks (creative + op ≥ 2).
            let cmd_block = matches!(
                t.state.name(),
                "command_block" | "chain_command_block" | "repeating_command_block"
            ) && self.game_mode == 1
                && self.op_level >= 2;
            if cmd_block && !suppress {
                self.cmd_in.push((t.pos, t.state.name()));
            }
            // VANILLA: StructureBlockEntity#usedBy — canUseGameMasterBlocks, экран открывается на клиенте.
            let structure =
                t.state.name() == "structure_block" && self.game_mode == 1 && self.op_level >= 2;
            if structure && !suppress {
                self.cmd_in.push((t.pos, "structure_block"));
            }
            // VANILLA: JigsawBlock#useWithoutItem — canUseGameMasterBlocks, экран на клиенте.
            let jigsaw = t.state.name() == "jigsaw" && self.game_mode == 1 && self.op_level >= 2;
            if jigsaw && !suppress {
                self.cmd_in.push((t.pos, "jigsaw"));
            }
            // VANILLA: TestBlock#useWithoutItem — canUseGameMasterBlocks, openTestBlock на клиенте.
            let testblock =
                t.state.name() == "test_block" && self.game_mode == 1 && self.op_level >= 2;
            if testblock && !suppress {
                self.cmd_in.push((t.pos, "test_block"));
            }
            let kind = if suppress || wire_passes {
                None
            } else if cmd_block || structure || jigsaw || testblock {
                Some(BlockUse::Menu)
            } else {
                // VANILLA: FlowerPotBlock#useItemOn/useWithoutItem — пустой горшок с растением в руке: SUCCESS
                // (взмах); без растения клик поглощён (CONSUME, без взмаха).
                block_use_kind(t.state.name()).or_else(|| {
                    (t.state.name() == "flower_pot").then(|| {
                        let potable = held_name.map(item_block_name).is_some_and(|n| {
                            ezzmc_protocol::generated::r26_3::BLOCK
                                .contains(&format!("potted_{n}").as_str())
                        });
                        if potable {
                            BlockUse::Click
                        } else {
                            BlockUse::Menu
                        }
                    })
                })
            };
            // VANILLA-UNSURE: локального useOn нет; блок-интерактив «съедает» клик (consumed),
            // блочный предмет — если клетка постановки свободна; иначе PASS → use_item ниже.
            // Swing: PREDICTED только у кликов без меню (рычаг, дверь…) и при постановке блока;
            // меню (SUCCESS_SERVER) руку качает сервер.
            // VANILLA: BlockItem с другим именем блока (Items.REDSTONE → redstone_wire и т. п.).
            let held_block = held_name.map(item_block_name);
            let is_block =
                held_block.is_none_or(|n| ezzmc_protocol::generated::r26_3::BLOCK.contains(&n));
            let place_cell = held_block
                .filter(|_| self.adventure_may_place(t.state))
                .and_then(|n| self.place_target(t.pos, t.face, t.hit, n));
            let placeable = place_cell.is_some();
            // Отладка расхождений (EZZMC_LOG=dbg=debug): почему есть/нет взмаха при постановке.
            tracing::debug!(
                target: "dbg",
                "use_on pos={:?} face={} held={:?} block_click={} is_block={} placeable={} crouch={} ground={} pos={:.3?} vel={:.3?} aabb={:.3?}",
                t.pos,
                t.face,
                held_name,
                kind.is_some(),
                is_block,
                placeable,
                self.player.crouching,
                self.player.on_ground,
                self.player.pos,
                self.player.vel,
                self.player.aabb()
            );
            if let Some(k) = kind {
                consumed = true;
                if k == BlockUse::Click {
                    crate::blocksound::use_block(t.state.name(), t.state, t.pos);
                    self.predict_toggle(t.state, t.pos);
                    self.player.swing();
                    let _ = self.net.cmd.send(crate::netthread::NetCmd::Swing);
                } else if menu_swings(t.state.name()) {
                    // VANILLA: useWithoutItem → InteractionResult.SUCCESS (клиентская ветка) →
                    // MultiPlayerGameMode#useItemOn → player.swing(hand).
                    self.player.swing();
                    let _ = self.net.cmd.send(crate::netthread::NetCmd::Swing);
                }
            } else if held_name == Some("ender_eye")
                && t.state.name() == "end_portal_frame"
                && t.state.property("eye") == Some("false")
            {
                // VANILLA: EnderEyeItem#useOn — пустая рамка: клиент сразу SUCCESS (взмах), глаз
                // вставляет сервер (level event 1503 + блок).
                consumed = true;
                self.player.swing();
                let _ = self.net.cmd.send(crate::netthread::NetCmd::Swing);
            } else if is_block && held_name.is_some() && placeable {
                consumed = true;
                self.player.swing();
                let _ = self.net.cmd.send(crate::netthread::NetCmd::Swing);
                // VANILLA: BlockItem#place → SoundType.getPlaceSound в клетке постановки.
                if let (Some(n), Some(c)) = (held_block, place_cell) {
                    crate::blocksound::place(n, c[0], c[1], c[2]);
                    // Предсказанный блок сразу в меш (иначе контур есть, а модели ещё нет).
                    self.queue.mark_block(c[0], c[1], c[2]);
                    self.queue.mark_block(c[0], c[1] + 1, c[2]);
                    if n.ends_with("_door") {
                        self.predict_door(n, c, t.hit);
                    } else {
                        self.predict_place(
                            n,
                            c,
                            crate::placement::Ctx {
                                pos: c,
                                face: t.face,
                                click: t.hit,
                                yaw: self.yaw,
                                pitch: self.pitch,
                                replace_clicked: c == t.pos,
                                here: self
                                    .net
                                    .level
                                    .lock()
                                    .ok()
                                    .and_then(|g| {
                                        g.as_ref().map(|l| l.chunks.block(c[0], c[1], c[2]))
                                    })
                                    .unwrap_or_default(),
                            },
                        );
                    }
                }
            } else {
                consumed = is_block;
            }
        }
        // VANILLA: Minecraft#startUseItem → MultiPlayerGameMode#useItem, если useItemOn не consumed.
        // Взмах — только у предметов, чьё использование клиент предсказывает (метательные);
        // еда/лук/щит/меч и прочее руку не качают (swingSource NONE / сервер пришлёт сам).
        if !consumed && held_name.is_some() {
            // VANILLA: WritableBookItem#use → Player#openItemGui (клиент: BookEditScreen), пакет use_item уходит всё равно.
            if held_name == Some("writable_book") {
                let pages = self
                    .inv
                    .held()
                    .map(crate::bookview::writable_pages)
                    .unwrap_or_default();
                self.book_in = Some((i32::from(self.inv.selected), pages));
            }
            let seq = self.next_seq();
            let _ = self.net.cmd.send(crate::netthread::NetCmd::UseItem {
                seq,
                yaw: self.yaw,
                pitch: self.pitch,
            });
            // VANILLA: MultiPlayerGameMode#useItem — предмет на кулдауне: пакет уходит, результат PASS
            // (ни взмаха, ни use()).
            if held_name.is_some_and(crate::cooldown::active) {
                return;
            }
            // VANILLA: ItemStack#use → applyAfterUseComponentSideEffects → UseCooldown#apply
            // (клиент ставит кулдаун сам, не дожидаясь ClientboundCooldownPacket).
            if let Some(n) = held_name {
                if let Some(t) = crate::cooldown::use_cooldown_ticks(n) {
                    crate::cooldown::add(n, t);
                }
            }
            const THROWN: &[&str] = &[
                "ender_pearl",
                "snowball",
                "egg",
                "blue_egg",
                "brown_egg",
                "experience_bottle",
                "splash_potion",
                "lingering_potion",
                "wind_charge",
                "fishing_rod",
                "firework_rocket",
            ];
            if held_name.is_some_and(|n| THROWN.contains(&n)) {
                self.player.swing();
                let _ = self.net.cmd.send(crate::netthread::NetCmd::Swing);
            }
            // VANILLA: BowItem#use — натяжение, если есть стрела (или креатив): startUsingItem, 72000 тиков.
            if held_name == Some("bow") && self.using.is_none() {
                let has_arrow = self.game_mode == 1
                    || self.inv.slots.iter().flatten().any(|s| {
                        ezzmc_protocol::generated::r26_3::ITEM
                            .get(s.item as usize)
                            .is_some_and(|n| n.ends_with("arrow"))
                    });
                if has_arrow {
                    self.using = Some(Using {
                        item: self.inv.held().map_or(0, |s| s.item),
                        slot: self.inv.selected,
                        remaining: 72000,
                        duration: 72000,
                        c: None,
                        bow: true,
                        rng: ezzmc_world::noise::LegacyRandom::new(seed_now()),
                    });
                    self.player.using_item = true;
                }
            }
            // VANILLA: SpyglassItem#use — звук + startUsingItem (1200 тиков).
            if held_name == Some("spyglass") && self.using.is_none() {
                self.using = Some(Using {
                    item: self.inv.held().map_or(0, |s| s.item),
                    slot: self.inv.selected,
                    remaining: 1200,
                    duration: 1200,
                    c: None,
                    bow: false,
                    rng: ezzmc_world::noise::LegacyRandom::new(seed_now()),
                });
                self.player.using_item = true;
                crate::audio::play_at(
                    "item.spyglass.use",
                    crate::audio::Cat::Players,
                    self.cam[0],
                    self.cam[1],
                    self.cam[2],
                    1.0,
                    1.0,
                );
            }
            // VANILLA: Consumable#startConsuming → LivingEntity#startUsingItem (canEat для еды).
            if let Some(c) = held_name.and_then(crate::itemdata::consumable) {
                if self.using.is_none() && (self.invulnerable || c.always || self.food < 20) {
                    self.using = Some(Using {
                        item: self.inv.held().map_or(0, |s| s.item),
                        slot: self.inv.selected,
                        remaining: c.ticks,
                        duration: c.ticks,
                        c: Some(c),
                        bow: false,
                        rng: ezzmc_world::noise::LegacyRandom::new(seed_now()),
                    });
                    self.player.using_item = true;
                }
            }
        }
    }

    /// VANILLA: BlockItem#useOn → BlockPlaceContext: клетка постановки.
    /// Если кликнутый блок заменяемый (трава, папоротник, снег…) — блок встаёт В НЕГО (`replaceClicked`),
    /// иначе — в соседнюю клетку по грани. Возвращает клетку, если постановка возможна.
    fn place_target(&self, pos: [i32; 3], face: u8, hit: [f64; 3], item: &str) -> Option<[i32; 3]> {
        let d: [i32; 3] = match face {
            0 => [0, -1, 0],
            1 => [0, 1, 0],
            2 => [0, 0, -1],
            3 => [0, 0, 1],
            4 => [-1, 0, 0],
            _ => [1, 0, 0],
        };
        let adjacent = [pos[0] + d[0], pos[1] + d[1], pos[2] + d[2]];
        let Ok(g) = self.net.level.lock() else {
            return Some(adjacent);
        };
        let Some(l) = g.as_ref() else {
            return Some(adjacent);
        };
        let clicked = l.chunks.block(pos[0], pos[1], pos[2]);
        // VANILLA: BlockBehaviour#canBeReplaced(ctx) — заменяемый и не тот же блок, что в руке.
        // VANILLA: SlabBlock#canBeReplaced — та же плита (не double) заменяется по грани/высоте клика.
        let slab_merge = |st: ezzmc_world::block::BlockState, clicked_here: bool| {
            if st.name() != item || !item.ends_with("_slab") {
                return false;
            }
            let ty = st.property("type");
            if ty == Some("double") {
                return false;
            }
            if !clicked_here {
                return true;
            }
            let above = self
                .target
                .is_some_and(|t| t.hit[1] - f64::from(pos[1]) > 0.5);
            let horiz = face >= 2;
            if ty == Some("bottom") {
                face == 1 || (above && horiz)
            } else {
                face == 0 || (!above && horiz)
            }
        };
        // VANILLA: VineBlock#canBeReplaced — лоза в клетке заменяема лозой, пока заняты не все 5 граней.
        let vine_merge = |st: ezzmc_world::block::BlockState| {
            item == "vine"
                && st.name() == "vine"
                && ["up", "north", "east", "south", "west"]
                    .iter()
                    .any(|f| st.property(f) != Some("true"))
        };
        let replace_clicked = (is_replaceable(clicked.name()) && clicked.name() != item)
            || slab_merge(clicked, true)
            || vine_merge(clicked);
        let target = if replace_clicked { pos } else { adjacent };
        let st = l.chunks.block(target[0], target[1], target[2]);
        let free = st.is_air()
            || is_fluid_block(st)
            || is_replaceable(st.name())
            || slab_merge(st, replace_clicked)
            || vine_merge(st);
        if !free {
            return None;
        }
        // VANILLA: AbstractBedBlock#getStateForPlacement — клетка головы должна быть заменяемой.
        let bed_head = item
            .ends_with("_bed")
            .then(|| crate::placement::bed_head_pos(target, self.yaw));
        if let Some(h) = bed_head {
            let hs = l.chunks.block(h[0], h[1], h[2]);
            if !(hs.is_air() || is_fluid_block(hs) || is_replaceable(hs.name())) {
                return None;
            }
        }
        // VANILLA: BlockItem#canPlace → mustSurvive: state.canSurvive(level, pos); getPlacementState
        // даёт null (нет подходящей опоры) — постановки нет.
        // Состояние, чья коллизия проверяется на пересечение с игроком (isUnobstructed считает по
        // состоянию постановки: слияние плит даёт double = полный блок, верхняя плита — верх и т. п.).
        let mut placed_state = None;
        if !item.ends_with("_door") {
            let ctx = crate::placement::Ctx {
                pos: target,
                face,
                click: hit,
                yaw: self.yaw,
                pitch: self.pitch,
                replace_clicked: target == pos,
                here: st,
            };
            let get = |x: i32, y: i32, z: i32| l.chunks.block(x, y, z);
            let ok =
                |s: ezzmc_world::block::BlockState| crate::support::can_survive(s, &get, target);
            let placed = crate::placement::fluid_plant(item, &ctx, &get, &ok)
                .unwrap_or_else(|| crate::placement::place_state(item, &ctx, &ok));
            match placed {
                crate::placement::Placed::Never => return None,
                crate::placement::Placed::State(s) => {
                    placed_state = Some(crate::placement::with_stairs_shape(s, target, &get))
                }
            }
        }
        // VANILLA: SlabBlock#getStateForPlacement — та же плита → type=double.
        if st.name() == item && item.ends_with("_slab") {
            placed_state = st.with("type", "double");
        }
        // VANILLA: BlockItem#placeBlock → Level#setBlock возвращает false, если состояние не изменилось
        // (лоза без новой грани) → InteractionResult.FAIL: ни звука, ни взмаха.
        if placed_state == Some(st) {
            return None;
        }
        // VANILLA: BlockItem#canPlace → Level#isUnobstructed(state, pos, collisionContext):
        // коллизия нового блока не должна пересекать сущности (здесь — сам игрок; спектатор не мешает).
        if self.game_mode == 3 {
            return Some(target);
        }
        let new = match placed_state {
            Some(s) => s,
            None => {
                let Some(info) = ezzmc_world::generated::b26_3::BLOCKS
                    .iter()
                    .find(|b| b.name == item)
                else {
                    return Some(target);
                };
                let Some(n) = ezzmc_world::block::BlockState::from_id(info.default_state) else {
                    return Some(target);
                };
                n
            }
        };
        let a = self.player.aabb();
        // Нога и (у кровати) голова: setPlacedBy ставит вторую клетку, коллизия та же.
        let head_state = new.with("part", "head");
        let hits = |cell: [i32; 3], st: ezzmc_world::block::BlockState| {
            let Some(props) = st.props() else {
                return false;
            };
            let (bx, by, bz) = (f64::from(cell[0]), f64::from(cell[1]), f64::from(cell[2]));
            ezzmc_mesh::blockinfo::collision_boxes_of(props.collision_shape)
                .iter()
                .any(|b| {
                    a[0] < bx + b[3]
                        && a[3] > bx + b[0]
                        && a[1] < by + b[4]
                        && a[4] > by + b[1]
                        && a[2] < bz + b[5]
                        && a[5] > bz + b[2]
                })
        };
        let blocked =
            hits(target, new) || bed_head.zip(head_state).is_some_and(|(h, hs)| hits(h, hs));
        (!blocked).then_some(target)
    }

    /// VANILLA: DoorBlock/TrapDoorBlock#useWithoutItem — переключение `open` (у двери обе половины).
    fn predict_toggle(&self, st: ezzmc_world::block::BlockState, pos: [i32; 3]) {
        let name = st.name();
        let door = name.ends_with("_door");
        if !(door || name.ends_with("_trapdoor")) {
            return;
        }
        let Some(open) = st.property("open") else {
            return;
        };
        let v = if open == "true" { "false" } else { "true" };
        let Ok(mut g) = self.net.level.lock() else {
            return;
        };
        let Some(l) = g.as_mut() else {
            return;
        };
        let oy = if st.property("half") == Some("upper") {
            pos[1] - 1
        } else {
            pos[1] + 1
        };
        let other = l.chunks.block(pos[0], oy, pos[2]);
        let mut set = |p: [i32; 3], s: ezzmc_world::block::BlockState| {
            if let Some(n) = s.with("open", v) {
                l.predict_block(ezzmc_protocol::types::BlockPos::new(p[0], p[1], p[2]), n);
            }
        };
        set(pos, st);
        if door && other.block_id() == st.block_id() {
            set([pos[0], oy, pos[2]], other);
        }
    }

    /// VANILLA: DoorBlock#getStateForPlacement + setPlacedBy — обе половины сразу (предсказание).
    fn predict_door(&self, item: &str, pos: [i32; 3], hit: [f64; 3]) {
        use ezzmc_mesh::blockinfo as bi;
        use ezzmc_world::block::BlockState;
        let Some(info) = ezzmc_world::generated::b26_3::BLOCKS
            .iter()
            .find(|b| b.name == item)
        else {
            return;
        };
        let Some(base) = BlockState::from_id(info.default_state) else {
            return;
        };
        let Ok(mut g) = self.net.level.lock() else {
            return;
        };
        let Some(l) = g.as_mut() else {
            return;
        };
        // Level#isInWorldBounds / canSurvive (опора снизу) / верхняя клетка свободна.
        let above = l.chunks.block(pos[0], pos[1] + 1, pos[2]);
        let below = l.chunks.block(pos[0], pos[1] - 1, pos[2]);
        let free = |s: BlockState| s.is_air() || is_fluid_block(s);
        if !free(above) || !bi::has(below, bi::COLLISION_FULL) {
            return;
        }
        // Direction#fromYRot: 0 юг, 1 запад, 2 север, 3 восток.
        let q = ((self.yaw / 90.0 + 0.5).floor() as i32).rem_euclid(4);
        let (facing, step, cw, ccw) = match q {
            0 => ("south", [0, 1], [-1, 0], [1, 0]),
            1 => ("west", [-1, 0], [0, -1], [0, 1]),
            2 => ("north", [0, -1], [1, 0], [-1, 0]),
            _ => ("east", [1, 0], [0, 1], [0, -1]),
        };
        let at = |l: &Level, o: [i32; 2], dy: i32| {
            l.chunks.block(pos[0] + o[0], pos[1] + dy, pos[2] + o[1])
        };
        let (l_st, l_up, r_st, r_up) = (at(l, ccw, 0), at(l, ccw, 1), at(l, cw, 0), at(l, cw, 1));
        let full = |s: BlockState| i32::from(bi::has(s, bi::COLLISION_FULL));
        let n = -full(l_st) - full(l_up) + full(r_st) + full(r_up);
        let is_door =
            |s: BlockState| s.name().ends_with("_door") && s.property("half") == Some("lower");
        let (ld, rd) = (is_door(l_st), is_door(r_st));
        let right = if (ld && !rd) || n > 0 {
            true
        } else if (rd && !ld) || n < 0 {
            false
        } else {
            let (dx, dz) = (hit[0] - f64::from(pos[0]), hit[2] - f64::from(pos[2]));
            (step[0] < 0 && dz < 0.0)
                || (step[0] > 0 && dz > 0.0)
                || (step[1] < 0 && dx > 0.0)
                || (step[1] > 0 && dx < 0.0)
        };
        let Some(lower) = base
            .with("facing", facing)
            .and_then(|s| s.with("hinge", if right { "right" } else { "left" }))
            .and_then(|s| s.with("half", "lower"))
        else {
            return;
        };
        let Some(upper) = lower.with("half", "upper") else {
            return;
        };
        // sequence уже взят у UseItemOn (next_seq), pending_seq тот же.
        let bp = |y: i32| ezzmc_protocol::types::BlockPos::new(pos[0], y, pos[2]);
        l.predict_block(bp(pos[1]), lower);
        l.predict_block(bp(pos[1] + 1), upper);
    }

    /// VANILLA: BlockItem#place под BlockStatePredictionHandler — клетка меняется сразу, до ответа сервера.
    /// Без этого игрок входит в клетку, которую сервер уже занял (откаты при постановке в траву).
    /// Состояние — `placement::place_state` (поворот, ось, половина, canSurvive), иначе default_state.
    /// VANILLA-UNSURE: без соседей (форма ступенек, тип сундука, сигнал).
    fn predict_place(&self, item: &str, c: [i32; 3], ctx: crate::placement::Ctx) {
        if let Ok(mut g) = self.net.level.lock() {
            if let Some(l) = g.as_mut() {
                let placed = {
                    let get = |x: i32, y: i32, z: i32| l.chunks.block(x, y, z);
                    let ok =
                        |s: ezzmc_world::block::BlockState| crate::support::can_survive(s, &get, c);
                    crate::placement::place_state(item, &ctx, &ok)
                };
                let crate::placement::Placed::State(mut state) = placed else {
                    return;
                };
                let state0 = state;
                state = crate::placement::with_stairs_shape(state0, c, &|x, y, z| {
                    l.chunks.block(x, y, z)
                });
                // VANILLA: getStateForPlacement → WATERLOGGED = fluidState.is(Fluids.WATER) (только источник).
                let here = l.chunks.block(c[0], c[1], c[2]);
                // VANILLA: SlabBlock#getStateForPlacement — та же плита → type=double, waterlogged=false.
                if here.name() == item && item.ends_with("_slab") {
                    if let Some(s) = here
                        .with("type", "double")
                        .and_then(|s| s.with("waterlogged", "false"))
                    {
                        l.predict_block(ezzmc_protocol::types::BlockPos::new(c[0], c[1], c[2]), s);
                        return;
                    }
                }
                if ezzmc_mesh::blockinfo::fluid(here).is_some_and(|f| !f.lava && f.source) {
                    if let Some(s) = state.with("waterlogged", "true") {
                        state = s;
                    }
                }
                // sequence уже взят у UseItemOn (next_seq), pending_seq тот же.
                l.predict_block(
                    ezzmc_protocol::types::BlockPos::new(c[0], c[1], c[2]),
                    state,
                );
                // VANILLA: BedBlock#setPlacedBy — голова в pos.relative(facing).
                if item.ends_with("_bed") {
                    if let Some(head) = state.with("part", "head") {
                        let h = crate::placement::bed_head_pos(c, self.yaw);
                        l.predict_block(
                            ezzmc_protocol::types::BlockPos::new(h[0], h[1], h[2]),
                            head,
                        );
                    }
                }
            }
        }
    }

    fn next_seq(&self) -> i32 {
        self.net
            .level
            .lock()
            .ok()
            .and_then(|mut g| g.as_mut().map(ezzmc_world::level::Level::start_prediction))
            .unwrap_or(0)
    }

    fn dig_aim(&self) -> (Option<crate::dig::Aim>, crate::dig::Body) {
        use crate::dig::{Aim, Body};
        // VANILLA: Minecraft#tick → pick(1.0F) в конце тика; handleKeybinds следующего тика ломает по этому hitResult
        // (глаза и поворот последнего отправленного пакета движения). Кадровый `self.target` (интерполированная
        // камера, свежий yaw/pitch) расходится с лучом сервера → Grim RotationBreak при махании камерой.
        let tick_hit = if self.entity_target.is_some() {
            None
        } else {
            self.place_target
        };
        // VANILLA: startAttack/continueDestroyBlock смотрят на ТЕКУЩЕЕ состояние блока: уже сломанный (предсказание
        // клиента) — воздух, цели нет (иначе START_DESTROY по воздуху → Grim AirLiquidBreak).
        let tick_hit = tick_hit.and_then(|mut t| {
            let g = self.net.level.lock().ok()?;
            let st = g.as_ref()?.chunks.block(t.pos[0], t.pos[1], t.pos[2]);
            if st.is_air() {
                return None;
            }
            t.state = st;
            Some(t)
        });
        let aim = tick_hit.filter(|_| self.have_pos).map(|t| Aim {
            pos: t.pos,
            face: t.face,
            state: t.state,
        });
        let body = Body {
            on_ground: self.player.on_ground,
            under_water: self.player.under_water,
            gear: self.dig_gear(),
        };
        (aim, body)
    }

    /// VANILLA: Player#blockInteractionRange — атрибут с сервера; до пакета 4.5 (креатив 5.0).
    fn block_reach(&self) -> f64 {
        self.block_range.unwrap_or(if self.game_mode == 1 {
            crate::pick::CREATIVE_BLOCK_REACH
        } else {
            crate::pick::BLOCK_REACH
        })
    }

    /// VANILLA: Player#entityInteractionRange — атрибут с сервера; до пакета 3.0 (креатив 5.0).
    fn entity_reach(&self) -> f64 {
        self.entity_range
            .unwrap_or(if self.game_mode == 1 { 5.0 } else { 3.0 })
    }

    /// Предмет в руке, чары и эффекты для `toolspeed::destroy_speed`.
    fn dig_gear(&self) -> crate::toolspeed::Gear {
        let held = self.inv.held();
        let tool = held
            .and_then(crate::handitem::stack_ctx)
            .map_or("", |(n, _)| n);
        let amp = |n: &str| self.effects.get(n).map_or(0, |e| e.amplifier + 1);
        // Значения атрибутов приходят с сервера (чары уже учтены); до первого пакета — по чарам локально.
        let head_aqua = self
            .inv
            .slots
            .get(crate::inventory::ARMOR_START)
            .and_then(|s| s.as_ref())
            .is_some_and(|s| crate::tooltip::enchant_level(&s.patch, "aqua_affinity") > 0);
        let eff_level = held.map_or(0, |s| crate::tooltip::enchant_level(&s.patch, "efficiency"));
        crate::toolspeed::Gear {
            tool,
            efficiency: self
                .mining_efficiency
                .unwrap_or_else(|| crate::toolspeed::efficiency_attr(eff_level)),
            break_speed: self.block_break_speed,
            submerged: self
                .submerged_mining_speed
                .unwrap_or(if head_aqua { 1.0 } else { 0.2 }),
            haste: amp("haste").max(amp("conduit_power")),
            fatigue: amp("mining_fatigue"),
        }
    }

    /// VANILLA: Item#canDestroyBlock для предмета в основной руке (креатив: TOOL.canDestroyBlocksInCreative).
    fn held_breaks_creative(&self) -> bool {
        self.inv.held().is_none_or(|s| {
            crate::handitem::stack_ctx(s).is_none_or(|(n, _)| crate::dig::breaks_in_creative(n))
        })
    }

    /// VANILLA: Player#blockActionRestricted — в приключении ломать можно, только если `can_break`
    /// предмета в основной руке совпал с блоком; тогда копаем как в выживании.
    fn dig_mode(&self, target: Option<ezzmc_world::block::BlockState>) -> i8 {
        if self.game_mode != 2 {
            return self.game_mode;
        }
        match (target, self.inv.held()) {
            (Some(st), Some(h)) if crate::adventure::allows(&h.patch, "can_break", st) => 0,
            _ => 2,
        }
    }

    /// VANILLA: BlockItem#useOn — в приключении ставить можно, только если `can_place_on` совпал с кликнутым блоком.
    fn adventure_may_place(&self, clicked: ezzmc_world::block::BlockState) -> bool {
        self.game_mode != 2
            || self
                .inv
                .held()
                .is_some_and(|h| crate::adventure::allows(&h.patch, "can_place_on", clicked))
    }

    fn tick_dig(&mut self) {
        let (aim, body) = self.dig_aim();
        let ok = self.held_breaks_creative();
        let mode = self.dig_mode(aim.as_ref().map(|a| a.state));
        let r = self.digger.tick(self.attack_held, aim, mode, body, ok);
        if std::mem::take(&mut self.digger.hit_sound) {
            if let Some(a) = aim {
                crate::blocksound::hit(a.state.name(), a.pos[0], a.pos[1], a.pos[2]);
            }
        }
        // VANILLA: continueAttack → player.swing(MAIN_HAND) каждый тик удержания (+ ServerboundSwingPacket
        // ПОСЛЕ PlayerAction этого тика; без него Grim NoSwingBreak на STOP_DESTROY_BLOCK).
        let swung = std::mem::take(&mut self.digger.swing);
        if swung {
            self.player.swing();
            // VANILLA: continueDestroyBlock → ParticleEngine#crack каждый тик.
            let guard = self.net.level.lock().unwrap_or_else(|e| e.into_inner());
            if let (Some(a), Some(l)) = (aim, guard.as_ref()) {
                let models = &self.models;
                self.particles.crack(
                    ezzmc_protocol::types::BlockPos::new(a.pos[0], a.pos[1], a.pos[2]),
                    a.state,
                    a.face,
                    &mut |s, q| Self::terrain_look(models, l, s, q),
                );
            }
        }
        self.apply_dig(r);
        if swung {
            let _ = self.net.cmd.send(crate::netthread::NetCmd::Swing);
        }
    }

    /// VANILLA: Minecraft#startAttack по блоку — каждый клик = startDestroyBlock (без destroyDelay).
    fn click_dig(&mut self) {
        let (aim, body) = self.dig_aim();
        let ok = self.held_breaks_creative();
        let mode = self.dig_mode(aim.as_ref().map(|a| a.state));
        let r = self.digger.click(aim, mode, body, ok);
        self.apply_dig(r);
    }

    fn apply_dig(&mut self, (cmds, broken): (Vec<crate::dig::DigCmd>, Vec<[i32; 3]>)) {
        if cmds.is_empty() && broken.is_empty() {
            return;
        }
        let n = cmds.len();
        if let Ok(mut g) = self.net.level.lock() {
            if let Some(l) = g.as_mut() {
                for (i, c) in cmds.into_iter().enumerate() {
                    // VANILLA: startPrediction на каждую команду; ломаемый блок — под sequence последней.
                    // VANILLA: MultiPlayerGameMode — ABORT_DESTROY_BLOCK шлётся без startPrediction (sequence 0);
                    // prediction только у START/STOP (иначе Grim BadPacketsH: expected=0).
                    let seq = if c.action == crate::dig::ABORT_DESTROY_BLOCK {
                        0
                    } else {
                        l.start_prediction()
                    };
                    if i + 1 == n {
                        for p in &broken {
                            // VANILLA: MultiPlayerGameMode#destroyBlock → Level#levelEvent(2001) (эффект ломания у самого игрока)
                            let bp = ezzmc_protocol::types::BlockPos::new(p[0], p[1], p[2]);
                            let old = l.chunks.block(p[0], p[1], p[2]);
                            if old != ezzmc_world::block::BlockState::AIR {
                                crate::blocksound::brk(old.name(), p[0], p[1], p[2]);
                                let models = &self.models;
                                self.particles.destroy_block(bp, old, &mut |s, q| {
                                    Self::terrain_look(models, l, s, q)
                                });
                            }
                            l.predict_block(
                                ezzmc_protocol::types::BlockPos::new(p[0], p[1], p[2]),
                                ezzmc_world::block::BlockState::AIR,
                            );
                            // VANILLA: DoorBlock#updateShape — вторая половина без пары исчезает сразу.
                            if old.name().ends_with("_door") {
                                let oy = if old.property("half") == Some("upper") {
                                    p[1] - 1
                                } else {
                                    p[1] + 1
                                };
                                let other = l.chunks.block(p[0], oy, p[2]);
                                if other.block_id() == old.block_id() {
                                    l.predict_block(
                                        ezzmc_protocol::types::BlockPos::new(p[0], oy, p[2]),
                                        ezzmc_world::block::BlockState::AIR,
                                    );
                                }
                            }
                        }
                    }
                    let _ = self.net.cmd.send(crate::netthread::NetCmd::PlayerAction {
                        action: c.action,
                        pos: c.pos,
                        face: c.face,
                        seq,
                    });
                }
            }
        }
        if !broken.is_empty() {
            // Свинг не здесь: его шлют startAttack (клик) и continueAttack (удержание), по одному на вызов.
            for p in broken {
                self.queue.mark_block(p[0], p[1], p[2]);
            }
        }
    }

    /// Луч взгляда → блок и жидкость под прицелом.
    fn update_targets(&mut self) {
        let (o, d) = (self.cam, look_dir(self.yaw, self.pitch));
        let (mut t, mut f) = (None, None);
        if self.have_pos {
            if let Ok(g) = self.net.level.lock() {
                if let Some(l) = g.as_ref() {
                    t = crate::pick::raycast(l, o, d, false);
                    f = crate::pick::raycast(l, o, d, true)
                        .filter(|h| ezzmc_mesh::blockinfo::fluid(h.state).is_some());
                }
            }
        }
        self.look_target = t;
        self.look_fluid = f;
        // VANILLA: Player#blockInteractionRange (4.5, креатив 5.0): дальше блок не выбирается.
        let block_range = self.block_reach();
        let within = |h: &crate::pick::Target| {
            let v = [h.hit[0] - o[0], h.hit[1] - o[1], h.hit[2] - o[2]];
            (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt() <= block_range
        };
        t = t.filter(within);
        f = f.filter(within);
        // VANILLA: GameRenderer#pick — сущность выбирается, если ближе блока; дальность 3.0 (креатив 5.0).
        let mut ent = None;
        if self.have_pos && self.game_mode != 3 {
            let reach = self.entity_reach();
            let parts = self.mobs.dragon_parts();
            if let Ok(g) = self.net.level.lock() {
                if let Some(l) = g.as_ref() {
                    if let Some((id, dist)) = crate::pick::raycast_entity(l, o, d, reach, &parts) {
                        let block_d = t.map_or(f64::INFINITY, |b| {
                            let v = [b.hit[0] - o[0], b.hit[1] - o[1], b.hit[2] - o[2]];
                            (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
                        });
                        if dist < block_d {
                            ent = Some(id);
                            self.entity_dist = dist;
                        }
                    }
                }
            }
        }
        if ent.is_some() {
            t = None;
        }
        self.entity_target = ent;
        self.target = t;
        self.fluid_target = f;
    }

    pub fn resident_sections(&self) -> usize {
        self.resident.len() + self.vox_resident.len()
    }
}

/// Сама жидкость (вода/лава), а не блок с жидкостью внутри (келп, морская трава, waterlogged):
/// VANILLA: BlockState#canBeReplaced — вода/лава заменяемы, келп и пр. нет.
/// VANILLA: EnvironmentAttributeProbe#tick — GaussianSampler по квартам (pos×0.25), веса по биому
/// копятся в порядке первого появления (Reference2DoubleArrayMap), затем последовательный srgbLerp
/// (SpatialAttributeInterpolator#applyAttributeLayer). Возвращает [sky, fog, water_fog].
fn probe_colors(
    l: &ezzmc_world::level::Level,
    biomes: &[Option<ezzmc_world::biome_color::BiomeColors>],
    cam: [f64; 3],
) -> [u32; 3] {
    const KERNEL: [f64; 7] = [0.0, 1.0, 4.0, 6.0, 4.0, 1.0, 0.0];
    let defaults = ezzmc_render::fog::FogAttrs::default();
    let p = [
        cam[0] * 0.25 - 0.5,
        cam[1] * 0.25 - 0.5,
        cam[2] * 0.25 - 0.5,
    ];
    let fl = [p[0].floor(), p[1].floor(), p[2].floor()];
    let rel = [p[0] - fl[0], p[1] - fl[1], p[2] - fl[2]];
    let w = |r: f64, i: usize| KERNEL[i + 1] + (KERNEL[i] - KERNEL[i + 1]) * r;
    let mut acc: Vec<(u32, f64)> = Vec::new();
    for z in 0..6usize {
        let qz = fl[2] as i32 - 2 + z as i32;
        for x in 0..6usize {
            let qx = fl[0] as i32 - 2 + x as i32;
            let Some(c) = l.chunks.get((qx * 4) >> 4, (qz * 4) >> 4) else {
                continue;
            };
            for y in 0..6usize {
                let qy = fl[1] as i32 - 2 + y as i32;
                let weight = w(rel[0], x) * w(rel[1], y) * w(rel[2], z);
                let id = c
                    .chunk
                    .biome(((qx * 4) & 15) as usize, qy * 4, ((qz * 4) & 15) as usize);
                match acc.iter_mut().find(|e| e.0 == id) {
                    Some(e) => e.1 += weight,
                    None => acc.push((id, weight)),
                }
            }
        }
    }
    // VANILLA: EnvironmentAttributeSystem — слой измерения (dimension_type.attributes) под
    // биомным: у биомов Энда нет visual/fog_color/sky_color → #181318 / #000000 из the_end.
    let dim = l.dimension.as_ref();
    let dim_sky = dim
        .and_then(|d| d.attr_color("visual/sky_color"))
        .unwrap_or(ezzmc_render::sky::DEFAULT_SKY_COLOR);
    let dim_fog = dim
        .and_then(|d| d.attr_color("visual/fog_color"))
        .unwrap_or(defaults.base_fog);
    let of = |id: u32| -> [u32; 3] {
        match biomes.get(id as usize) {
            Some(Some(b)) => [
                b.sky_color.unwrap_or(dim_sky),
                b.fog_color.unwrap_or(dim_fog),
                b.water_fog_color.unwrap_or(defaults.water_fog_color),
            ],
            _ => [dim_sky, dim_fog, defaults.water_fog_color],
        }
    };
    let mut result: Option<[u32; 3]> = None;
    let mut total = 0.0f64;
    for (id, weight) in acc {
        if weight <= 0.0 {
            continue;
        }
        let v = of(id);
        total += weight;
        result = Some(match result {
            None => v,
            Some(r) => {
                let t = (weight / total) as f32;
                [
                    ezzmc_render::sky::srgb_lerp(t, r[0], v[0]),
                    ezzmc_render::sky::srgb_lerp(t, r[1], v[1]),
                    ezzmc_render::sky::srgb_lerp(t, r[2], v[2]),
                ]
            }
        });
    }
    result.unwrap_or_else(|| of(u32::MAX))
}

fn is_fluid_block(s: BlockState) -> bool {
    matches!(s.name(), "water" | "lava" | "bubble_column")
}

fn look_dir(yaw: f32, pitch: f32) -> [f32; 3] {
    let (y, p) = (yaw.to_radians(), pitch.to_radians());
    [-y.sin() * p.cos(), -p.sin(), y.cos() * p.cos()]
}

/// VANILLA: FluidState#getHeightForCamera (упрощённо: источник под сплошным блоком = 1).
fn fluid_height_for_camera(l: &Level, p: [i32; 3], fl: ezzmc_mesh::blockinfo::Fluid) -> f32 {
    use ezzmc_mesh::blockinfo as bi;
    let above = l.chunks.block(p[0], p[1] + 1, p[2]);
    if fl.source && bi::has(above, bi::COLLISION_FULL) {
        return 1.0;
    }
    // FlowingFluid#getHeight: жидкость того же типа сверху → 1.
    if bi::fluid(above).is_some_and(|a| a.lava == fl.lava) {
        return 1.0;
    }
    fl.own_height()
}

/// VANILLA: Camera#getFluidInCamera (+ точки ближней плоскости).
fn fluid_in_camera(
    l: &Level,
    cam: [f64; 3],
    yaw: f32,
    pitch: f32,
    aspect: f32,
) -> ezzmc_render::fog::FogType {
    use ezzmc_mesh::blockinfo as bi;
    use ezzmc_render::fog::FogType;
    let b = [
        cam[0].floor() as i32,
        cam[1].floor() as i32,
        cam[2].floor() as i32,
    ];
    if let Some(fl) = bi::fluid(l.chunks.block(b[0], b[1], b[2])) {
        if !fl.lava && cam[1] < f64::from(b[1]) + f64::from(fluid_height_for_camera(l, b, fl)) {
            return FogType::Water;
        }
    }
    // Ближняя плоскость: forward, 4 угла (Camera#getNearPlane).
    let fwd = look_dir(yaw, pitch);
    let up = look_dir(yaw, pitch - 90.0);
    let right = [
        fwd[1] * up[2] - fwd[2] * up[1],
        fwd[2] * up[0] - fwd[0] * up[2],
        fwd[0] * up[1] - fwd[1] * up[0],
    ];
    let h = f64::from(NEAR) * f64::from((FOV_DEG.to_radians() / 2.0).tan());
    let w = h * f64::from(aspect);
    let n = f64::from(NEAR);
    let mut pts = vec![[0.0f64; 3]];
    for (sx, sy) in [(-1.0, 1.0), (1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)] {
        pts.push([
            sx * w * f64::from(right[0]) + sy * h * f64::from(up[0]),
            sx * w * f64::from(right[1]) + sy * h * f64::from(up[1]),
            sx * w * f64::from(right[2]) + sy * h * f64::from(up[2]),
        ]);
    }
    for p in pts {
        let o = [
            cam[0] + n * f64::from(fwd[0]) + p[0],
            cam[1] + n * f64::from(fwd[1]) + p[1],
            cam[2] + n * f64::from(fwd[2]) + p[2],
        ];
        let c = [
            o[0].floor() as i32,
            o[1].floor() as i32,
            o[2].floor() as i32,
        ];
        let st = l.chunks.block(c[0], c[1], c[2]);
        if let Some(fl) = bi::fluid(st) {
            if fl.lava && o[1] <= f64::from(fluid_height_for_camera(l, c, fl)) + f64::from(c[1]) {
                return FogType::Lava;
            }
        } else if st.name() == "powder_snow" {
            return FogType::PowderSnow;
        }
    }
    FogType::Atmospheric
}

/// Блок «съедает» ПКМ: `Menu` — открывает интерфейс (swing от сервера), `Click` — мгновенное действие.
#[derive(Clone, Copy, PartialEq, Eq)]
enum BlockUse {
    Menu,
    Click,
}

/// VANILLA: Item → Block для BlockItem с разными именами (Items.REDSTONE → redstone_wire и т. п.).
fn item_block_name(item: &str) -> &str {
    match item {
        "redstone" => "redstone_wire",
        "string" => "tripwire",
        "wheat_seeds" => "wheat",
        "beetroot_seeds" => "beetroots",
        "melon_seeds" => "melon_stem",
        "pumpkin_seeds" => "pumpkin_stem",
        "carrot" => "carrots",
        "potato" => "potatoes",
        "sweet_berries" => "sweet_berry_bush",
        "glow_berries" => "cave_vines",
        "cocoa_beans" => "cocoa",
        "torchflower_seeds" => "torchflower_crop",
        "pitcher_pod" => "pitcher_crop",
        other => other,
    }
}

/// VANILLA: `.replaceable()` в Blocks.java — блоки, в которые можно поставить блок без очистки.
/// Цветы сюда не входят (ваниль их не заменяет).
fn is_replaceable(name: &str) -> bool {
    const REPLACEABLE: &[&str] = &[
        "air",
        "cave_air",
        "void_air",
        "water",
        "lava",
        "short_grass",
        "tall_grass",
        "fern",
        "large_fern",
        "dead_bush",
        "short_dry_grass",
        "tall_dry_grass",
        "bush",
        "seagrass",
        "tall_seagrass",
        "snow",
        "fire",
        "soul_fire",
        "vine",
        "glow_lichen",
        "hanging_roots",
        "leaf_litter",
        "light",
        "nether_sprouts",
        "crimson_roots",
        "warped_roots",
        "resin_clump",
        "bubble_column",
        "structure_void",
    ];
    REPLACEABLE.contains(&name)
}

/// Меню-блоки, у которых useWithoutItem на клиенте безусловно возвращает SUCCESS (взмах руки).
/// Условные (лекторий, командные блоки, компостер, полка, якорь возрождения) сюда не входят.
fn menu_swings(name: &str) -> bool {
    matches!(
        name,
        "chest"
            | "trapped_chest"
            | "ender_chest"
            | "barrel"
            | "furnace"
            | "blast_furnace"
            | "smoker"
            | "crafting_table"
            | "anvil"
            | "chipped_anvil"
            | "damaged_anvil"
            | "enchanting_table"
            | "brewing_stand"
            | "grindstone"
            | "stonecutter"
            | "loom"
            | "cartography_table"
            | "smithing_table"
            | "beacon"
            | "hopper"
            | "dropper"
            | "dispenser"
            | "crafter"
    ) || name.ends_with("shulker_box")
        || name.ends_with("copper_chest")
}

/// VANILLA: BlockBehaviour#useWithoutItem (SUCCESS / SUCCESS_SERVER) — по имени блока.
/// VANILLA-UNSURE: список по именам, а не по классам; предметозависимые блоки (котёл,
/// горшок, костёр, рамка портала) не входят — их useItemOn идёт на сервере.
fn block_use_kind(name: &str) -> Option<BlockUse> {
    const MENU: &[&str] = &[
        "chest",
        "trapped_chest",
        "ender_chest",
        "barrel",
        "furnace",
        "blast_furnace",
        "smoker",
        "crafting_table",
        "anvil",
        "chipped_anvil",
        "damaged_anvil",
        "enchanting_table",
        "brewing_stand",
        "grindstone",
        "stonecutter",
        "loom",
        "cartography_table",
        "smithing_table",
        "beacon",
        "hopper",
        "dropper",
        "dispenser",
        "crafter",
        "lectern",
        "command_block",
        "chain_command_block",
        "repeating_command_block",
        "structure_block",
        "jigsaw",
        "test_block",
        "composter",
        "chiseled_bookshelf",
        "respawn_anchor",
    ];
    const CLICK: &[&str] = &[
        "lever",
        "repeater",
        "comparator",
        "daylight_detector",
        "note_block",
        "dragon_egg",
        "bell",
        "cake",
        "jukebox",
        "redstone_wire",
        "sweet_berry_bush",
        "cave_vines",
        "cave_vines_plant",
    ];
    if MENU.contains(&name) || name.ends_with("shulker_box") || name.ends_with("copper_chest") {
        return Some(BlockUse::Menu);
    }
    if CLICK.contains(&name)
        || name.ends_with("_button")
        || name.ends_with("_bed")
        || name.ends_with("_sign")
        || name.ends_with("_candle_cake")
        || name.starts_with("potted_")
        || name == "candle_cake"
        || (name.ends_with("_door") && name != "iron_door")
        || (name.ends_with("_trapdoor") && name != "iron_trapdoor")
        || name.ends_with("_fence_gate")
    {
        return Some(BlockUse::Click);
    }
    None
}

/// VANILLA: состояние использования предмета (еда/питьё).
/// VANILLA: SpyglassItem#ZOOM_FOV_MODIFIER.
const SPYGLASS_ZOOM_FOV_MODIFIER: f32 = 0.1;

pub struct Using {
    pub item: u32,
    pub slot: u8,
    pub remaining: i32,
    pub duration: i32,
    /// None — лук/арбалет (натяжение), без звуков еды.
    pub c: Option<crate::itemdata::Consume>,
    pub bow: bool,
    rng: ezzmc_world::noise::LegacyRandom,
}

fn seed_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos() as i64)
}

/// Тип сущности — LivingEntity (мобы, игроки, армор-стенд). Остальное: предметы, снаряды,
/// транспорт, блоки, рамки/картины, display/marker и пр.
fn is_living_type(name: &str) -> bool {
    const NOT: &[&str] = &[
        "item",
        "experience_orb",
        "falling_block",
        "tnt",
        "painting",
        "item_frame",
        "glow_item_frame",
        "leash_knot",
        "end_crystal",
        "marker",
        "interaction",
        "area_effect_cloud",
        "evoker_fangs",
        "lightning_bolt",
        "fishing_bobber",
        "eye_of_ender",
        "block_display",
        "item_display",
        "text_display",
        "firework_rocket",
        "ominous_item_spawner",
        "experience_bottle",
    ];
    const NOT_SUFFIX: &[&str] = &[
        "_boat",
        "_raft",
        "_chest_boat",
        "minecart",
        "arrow",
        "fireball",
        "_charge",
        "_potion",
        "trident",
        "snowball",
        "egg",
        "ender_pearl",
        "wind_charge",
        "skull",
        "shulker_bullet",
        "llama_spit",
        "spear",
    ];
    let n = name.strip_prefix("minecraft:").unwrap_or(name);
    !NOT.contains(&n)
        && !NOT_SUFFIX.iter().any(|s| n.ends_with(s))
        && n != "boat"
        && n != "chest_boat"
}

#[cfg(test)]
mod block_use_tests {
    #[test]
    fn sleep_timer_follows_vanilla() {
        let mut t = 0;
        for _ in 0..150 {
            t = next_sleep_timer(t, true);
        }
        assert_eq!(t, 100);
        let mut n = 0;
        while t != 0 {
            t = next_sleep_timer(t, false);
            n += 1;
        }
        assert_eq!(n, 10);
    }

    use super::*;

    #[test]
    fn block_use_kinds() {
        assert!(block_use_kind("copper_chest") == Some(BlockUse::Menu));
        assert!(block_use_kind("oxidized_copper_chest") == Some(BlockUse::Menu));
        assert!(block_use_kind("dragon_egg") == Some(BlockUse::Click));
        assert!(block_use_kind("potted_oak_sapling") == Some(BlockUse::Click));
        assert!(block_use_kind("iron_door").is_none());
        assert!(block_use_kind("stone").is_none());
    }
}

/// PROF: время запекания дропов и их число (по Drop).
/// Экземпляры одной модели за кадр (20 f32: матрица + свет): обычные, затем с блеском.
struct ItemInstFrame {
    mesh: std::sync::Arc<crate::handitem::HandMesh>,
    normal: Vec<f32>,
    foil: Vec<f32>,
}

fn bytemuck_cast(v: &[f32]) -> &[u8] {
    // SAFETY: f32 → u8 того же буфера, длина ×4, выравнивание 1.
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast::<u8>(), std::mem::size_of_val(v)) }
}

struct ProfDrop(std::time::Instant, usize);
impl Drop for ProfDrop {
    fn drop(&mut self) {
        crate::mobs::prof_phase(18, "дропы: запекание мешей", self.0.elapsed());
        crate::mobs::prof_phase(19, &format!("дропов {}", self.1), std::time::Duration::ZERO);
    }
}

fn is_vault(id: u32) -> bool {
    ezzmc_world::block::BlockState::from_id(id).is_some_and(|s| s.name() == "vault")
}
