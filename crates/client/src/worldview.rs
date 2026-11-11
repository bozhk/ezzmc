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
use ezzmc_mesh::section::{ModelProvider, Neighborhood, SectionPos};
use ezzmc_render::hiz::HiZ;
use ezzmc_render::lightmap::{LightmapParams, LightmapTexture};
use ezzmc_render::pipeline_mode::RenderPipelineMode;
use ezzmc_render::renderer::{CullCamera, Renderer};
use ezzmc_render::skydraw::{CelestialImages, SkyDraw};
use ezzmc_render::terrain::{TerrainArena, TerrainPipelines};
use ezzmc_render::uniforms::{ChunkSectionUniform, FogUniform, GlobalsUniform, ProjectionUniform};
use ezzmc_render::voxel_adapter::{split_mesh, RectTable};
use ezzmc_render::Gpu;
use ezzmc_world::block::BlockState;
use ezzmc_world::level::{Level, LevelEvent};
use glam::{Mat4, Vec3};
use rustc_hash::FxHashSet as HashSet;
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
    /// VANILLA: Abilities#invulnerable (флаг 1).
    pub invulnerable: bool,
    /// VANILLA: LivingEntity#useItem/useItemRemaining — идёт еда/питьё.
    pub using: Option<Using>,
    attack_item: Option<u32>,
    digger: crate::dig::Digger,
    net: NetHandle,
    /// Клавиши последнего кадра ходьбы (для player_input).
    last_input: crate::physics::Input,
    /// GameType локального игрока (0 survival, 1 creative…).
    pub game_mode: i8,
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
    /// Модели сущностей (пока только аватар игрока от 3-го лица): скин Стива, слой одежды.
    entity_draw: ezzmc_render::particledraw::ParticleDraw,
    /// Броня игроков: отдельная текстура-атлас (`armor::ArmorKit`).
    armor_draw: ezzmc_render::particledraw::ParticleDraw,
    armor_kit: crate::armor::ArmorKit,
    /// Блеск зачарований брони (геометрия брони + текстура блеска).
    glint_draw: ezzmc_render::particledraw::ParticleDraw,
    /// Атласы для программного превью игрока (блоки — со старта, предметы — лениво).
    preview_blocks: std::rc::Rc<(Vec<u8>, u32, u32)>,
    preview_items: Option<std::rc::Rc<(Vec<u8>, u32, u32)>>,
    mobs: crate::mobs::Mobs,
    chests: crate::chest::Chests,
    bells: crate::bell::Bells,
    /// Стрелы: текстуры arrow/arrow_spectral/arrow_tipped стопкой 32×96.
    arrow_draw: ezzmc_render::particledraw::ParticleDraw,
    /// VANILLA: PistonHeadRenderer — движущиеся блоки (атлас блоков).
    piston_draw: ezzmc_render::particledraw::ParticleDraw,
    /// Тени сущностей (shadow.png) и Level#getSkyDarken.
    shadow_draw: ezzmc_render::particledraw::ParticleDraw,
    sky_darken: i32,
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
    fog_state: ezzmc_render::fog::FogState,
    vignette: ezzmc_render::vignette::VignetteDraw,
    /// Hud#vignetteBrightness и накопитель тиков.
    vignette_b: f32,
    vignette_acc: f32,
    /// EnvironmentAttributeProbe: (прошлый, новый) тик цветов [небо, туман, туман воды] и прогресс тика.
    probe_pair: Option<([u32; 3], [u32; 3])>,
    probe_acc: f32,
    probe_colors: [u32; 3],
    last_frame: Option<std::time::Instant>,
    #[allow(clippy::type_complexity)]
    lightmap_key: Option<(f32, [f32; 3], f32, f32, [f32; 3], [f32; 3])>,
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
    /// VANILLA: ModelBakery#FIRE_0 / FIRE_1 (пламя горящих сущностей).
    fire_uv: Option<(ezzmc_assets::bake::SpriteUv, ezzmc_assets::bake::SpriteUv)>,
    /// VANILLA: ItemInHandRenderer#mainHandItem (предмет, который сейчас рисуется) и слот.
    hand_shown: Option<(ezzmc_protocol::item::ItemStack, u8)>,
    /// Видимый предмет второй руки и её высота (VANILLA: FirstPersonHandsAndItems#offHandItem/offHandHeight).
    off_shown: Option<ezzmc_protocol::item::ItemStack>,
    off_h: (f32, f32),
    hand_item_visible: bool,
    entity_item_visible: bool,
    fog_buf: wgpu::Buffer,
    globals_buf: wgpu::Buffer,
    chunk_buf: wgpu::Buffer,
    group0: wgpu::BindGroup,
    depth: Option<(wgpu::TextureView, u32, u32)>,
    gpu_queue: wgpu::Queue,
    color_format: wgpu::TextureFormat,
    atlas_view: wgpu::TextureView,
    atlas_tex: wgpu::Texture,
    atlas_anim: crate::atlasanim::AtlasAnim,
    /// Включён ли compute-рендер (флаг; без поддержки GPU остаётся `false`).
    use_compute: bool,
    vox: Option<Renderer>,
    rects: RectTable,
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
    /// Зум по удержанию (клавиша `key.zoom`): цель и текущий множитель FOV (1.0 — без зума).
    pub zoom_held: bool,
    pub zoom_factor: f32,
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
    /// Здоровье (0 — мёртв, показывается экран смерти).
    pub health: f32,
    /// Входящий чат для главного потока: (текст, индикатор; None — action bar).
    pub sugg_in: Vec<(i32, i32, i32, Vec<String>)>,
    pub chat_in: Vec<(ezzmc_protocol::text::Component, Option<crate::chat::Tag>)>,
    lines: ezzmc_render::lines::LineDraw,
    /// F3: 3D-прицел (чёрная обводка + оси X/Y/Z); `Some(guiScale)` включает.
    /// VANILLA: DebugCrosshairRenderer.
    pub debug_crosshair: Option<u32>,
    /// F1: HUD скрыт — без руки (VANILLA: GameRenderer#renderItemInHand, isHudHidden).
    pub hud_hidden: bool,
    cross_lines: [ezzmc_render::lines::LineDraw; 4],
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
    }
}

/// Плавание для корневой матрицы. VANILLA: AvatarRenderer#setupRotations (swimAmount > 0).
#[derive(Clone, Copy, Default, PartialEq)]
pub struct SwimPose {
    pub amount: f32,
    pub in_water: bool,
    pub visually: bool,
    pub x_rot: f32,
}

impl SwimPose {
    pub fn matrix(self) -> Mat4 {
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
    ) -> Result<Self, String> {
        let atlas = BlockAtlas::build(rm, 4)?;
        let mut models = BlockModels::build(rm, &atlas, opts.cutout_leaves);
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
            DEPTH_FORMAT,
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
                "crit",
                "explosion",
                "poof",
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
        let preview_blocks =
            std::rc::Rc::new((atlas.mips[0].to_rgba8(), atlas.width, atlas.height));
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
        let piston_draw = ezzmc_render::particledraw::ParticleDraw::new(
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
                part.visible = matches!(part.name, "right_arm" | "right_sleeve");
            }
            (m, parts)
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
            rects: RectTable::default(),
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
            entity_draw,
            armor_draw,
            armor_kit,
            glint_draw,
            preview_blocks,
            preview_items: None,
            mobs: crate::mobs::Mobs::default(),
            chests: crate::chest::Chests::default(),
            bells: crate::bell::Bells::default(),
            arrow_draw,
            piston_draw,
            shadow_draw,
            sky_darken: 0,
            player_model: ezzmc_render::humanoid::player_mesh(false),
            particles: crate::particles::ParticleEngine::new(sets),
            particle_draw,
            last_particle_tick: None,
            last_piston_tick: None,
            remote_anims: Default::default(),
            remote_tick: None,
            wet_entities: Default::default(),
            sky_state: None,
            fog_state: ezzmc_render::fog::FogState::default(),
            vignette,
            vignette_b: 1.0,
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
            entity_glint_draw,
            fire_uv,
            hand_shown: None,
            off_shown: None,
            off_h: (0.0, 0.0),
            hand_item_visible: false,
            entity_item_visible: false,
            fog_buf,
            globals_buf,
            chunk_buf,
            group0,
            depth: None,
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
            invulnerable: false,
            using: None,
            attack_item: None,
            digger: crate::dig::Digger::default(),
            walking: std::env::var("EZZMC_FREECAM").map_or(true, |v| v != "1"),
            tick_acc: 0.0,
            riding: false,
            ride_boat_yaw: None,
            ride_dr: 0.0,
            render_yaw_off: 0.0,
            boat: None,
            boat_out: None,
            fov_mod: (1.0, 1.0),
            zoom_held: false,
            zoom_factor: 1.0,
            cfg: crate::settings::ViewCfg::default(),
            no_send: false,
            yaw: 0.0,
            pitch: 0.0,
            perspective: 0,
            status: None,
            joined: false,
            health: 20.0,
            sugg_in: Vec::new(),
            chat_in: Vec::new(),
            lines: ezzmc_render::lines::LineDraw::new(&gpu.device, color, DEPTH_FORMAT),
            debug_crosshair: None,
            hud_hidden: false,
            cross_lines: std::array::from_fn(|_| {
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
            if self.rects.dirty {
                v.set_atlas(
                    gpu,
                    self.atlas_view.clone(),
                    self.lightmap.view.clone(),
                    &self.rects.rects,
                );
                self.rects.dirty = false;
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
        self.rects.dirty = true;
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
                NetEvt::GameMode(m) => self.game_mode = m,
                NetEvt::Health(h, food, sat) => {
                    self.health = h;
                    self.food = food;
                    self.saturation = sat;
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
                NetEvt::Waypoint(op, wp) => {
                    // VANILLA: ClientWaypointManager#trackWaypoint/updateWaypoint/untrackWaypoint.
                    if op == 1 {
                        self.waypoints.remove(&wp.key);
                    } else {
                        self.waypoints.insert(wp.key.clone(), wp);
                    }
                }
                NetEvt::UseComplete => self.complete_using(),
                NetEvt::Chat(c, t) => self.chat_in.push((c, Some(t))),
                NetEvt::Overlay(c) => self.chat_in.push((c, None)),
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
                } => {
                    self.tp_gen_applied = gen;
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
                        self.mark_column_area(x, z);
                    }
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
                    LevelEvent::Effect { ty, pos, data } if ty != 2001 => {
                        if ty == 2000 {
                            self.particles.shoot_smoke(data, pos.x, pos.y, pos.z);
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

    /// Клиентские тики частиц по росту `game_time` (VANILLA: ParticleEngine#tick + tickWeatherEffects).
    /// VANILLA: Minecraft#getSituationalMusic (без босса Энда): BACKGROUND_MUSIC в позиции камеры.
    pub fn situational_music(&self) -> Option<crate::music::Music> {
        let g = self.net.level.lock().ok()?;
        let level = g.as_ref()?;
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
            self.particles.pearl_hit(p);
        }
        for (p, w, h) in std::mem::take(&mut level.death_poofs) {
            self.particles.death_poof(p, w, h);
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
        for _ in 0..n {
            // VANILLA: Minecraft#tick → level.animateTick(camera block) каждый клиентский тик.
            self.particles.animate_tick(
                level,
                self.cam[0].floor() as i32,
                self.cam[1].floor() as i32,
                self.cam[2].floor() as i32,
            );
            self.particles.tick(level);
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
            // VANILLA: PrimedTnt#tick — клиентский дым SMOKE на y+0.5 каждый тик.
            for e in level.entities.iter().filter(|e| e.fuse > 0) {
                self.particles
                    .add_smoke(e.pos[0], e.pos[1] + 0.5, e.pos[2], 0.0, 0.0, 0.0);
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
                        self.pool.submit(pos, hood, models);
                    }
                }
            }
        }
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
                let (conv, left) = split_mesh(&r.mesh, &mut self.rects);
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
                            .map(|v| v.yaw);
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
                        self.psound.before_tick(p);
                        p.tick(l, input, yaw);
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
                self.tick_using();
                crate::cooldown::tick();
                self.tick_hand();
                self.tick_dig();
            }
            if let Some(t) = self_fov {
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
        self.cam = self.player.eye_lerp(self.tick_acc / 0.05);
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
        self.particles.status = cfg.particles;
        self.player.sprint_window = cfg.sprint_window;
        self.player.auto_jump = cfg.auto_jump;
    }

    /// VANILLA: GameRenderer#renderLevel — bobView только при options.bobView.
    fn bob(&self, pt: f32) -> Mat4 {
        if self.cfg.bob_view {
            self.player.bob_matrix(pt)
        } else {
            Mat4::IDENTITY
        }
    }

    /// Текущий FOV с динамическим множителем (спринт/полёт), интерполяция по partial tick.
    pub fn fov_deg(&self) -> f32 {
        let pt = (self.tick_acc / 0.05) as f32;
        let (o, c) = self.fov_mod;
        self.cfg.fov * (o + (c - o) * pt) * self.zoom_factor
    }

    /// Плавный зум: к 1/4 FOV при удержании клавиши, обратно при отпускании.
    pub fn tick_zoom(&mut self, dt: f64) {
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
            };
            let root = self.avatar_root(pos, body, p.crouching, swim);
            let (model, parts) = &mut self.player_model;
            crate::settings::apply_parts(model, self.cfg.parts);
            setup_anim(model, parts, &state);
            model.bake(root, [255; 4], light, &mut data);
            let looks = crate::armor::looks_of_menu(&self.inv.slots);
            self.armor_kit.fall_flying = false;
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
        self.bake_remote_players(
            &mut data,
            &mut armor_data,
            &mut glint_data,
            &mut shadow,
            &mut item_data,
        );
        if let Ok(g) = self.net.level.lock() {
            if let Some(l) = g.as_ref() {
                self.mobs.bake(l, self.cam, self.sky_darken, &mut shadow);
                self.chests.bake(
                    l,
                    self.cam,
                    (self.queue.version, self.queue.block_version),
                    &mut self.mobs,
                );
                self.bells.bake(
                    l,
                    self.cam,
                    (self.queue.version, self.queue.block_version),
                    &mut self.mobs,
                );
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
                    let img = rm
                        .read(&format!("assets/minecraft/textures/entity/{tex}.png"))
                        .and_then(|d| Image::decode_png(&d).ok())?;
                    (img.to_rgba8(), img.width, img.height)
                };
                let ctor = if mask {
                    ezzmc_render::particledraw::ParticleDraw::new_mask
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
            self.mobs.prepare(&gpu.device, &gpu.queue, mv, &mut make);
        }
        self.bake_item_entities(&mut item_data, &mut shadow);
        let mut arrow_data = Vec::new();
        self.bake_projectiles(&mut item_data, &mut arrow_data);
        self.arrow_draw
            .prepare_raw(&gpu.device, &gpu.queue, mv, &arrow_data);
        self.bake_flames(&mut item_data.0);
        self.entity_draw
            .prepare_raw(&gpu.device, &gpu.queue, mv, &data);
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
        if let Some(d) = self.entity_item_draw.as_mut() {
            d.prepare_split(&gpu.device, &gpu.queue, mv, &item_data.0, &item_data.1);
        }
        if let Some(d) = self.entity_glint_draw.as_mut() {
            d.set_texture_mat(crate::armor::glint_matrix(8.0));
            d.prepare_split(&gpu.device, &gpu.queue, mv, &item_data.0, &item_data.1);
        }
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

    /// Рука от 1-го лица. VANILLA: FirstPersonHandsAndItemsRenderer#renderPlayerArm (пустая правая рука),
    /// AvatarRenderer#renderHand (rightArm.zRot = 0.1), GameRenderer#renderItemInHand
    /// (проекция 70° + bobView, model-view = единица, глубина очищена).
    fn prepare_hand(&mut self, gpu: &Gpu, aspect: f32, far: f32) {
        self.hand_visible = false;
        self.hand_item_visible = false;
        if self.perspective != 0 || !self.walking || !self.have_pos {
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
        let mesh = self.shown_mesh();
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
        } else if self.hand_shown.is_none() {
            let (model, parts) = &mut self.hand_model;
            model.reset_pose();
            model.parts[parts.right_arm].pose.z_rot = 0.1;
            let mut data = Vec::new();
            model.bake(m, [255; 4], light, &mut data);
            self.hand_draw
                .prepare_raw(&gpu.device, &gpu.queue, Mat4::IDENTITY, &data);
            self.hand_visible = true;
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
        let proj = Mat4::perspective_rh(70.0f32.to_radians(), aspect, NEAR, far) * self.bob(pt);
        gpu.queue.write_buffer(
            &self.hand_proj_buf,
            0,
            &ProjectionUniform { proj_mat: proj }.to_bytes(),
        );
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
                .map(|h| std::rc::Rc::new(h.items_rgba()));
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

    fn offhand_mesh(&mut self, real: bool) -> Option<std::rc::Rc<crate::handitem::HandMesh>> {
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

    fn shown_mesh(&mut self) -> Option<std::rc::Rc<crate::handitem::HandMesh>> {
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

    fn bake_item_entities(&mut self, item_data: &mut (Vec<u8>, Vec<u8>), shadow: &mut Vec<u8>) {
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
            let (min_y, depth) = mesh.ground_bounds();
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
            let view = crate::handitem::View::Ground;
            if depth > 0.0625 {
                mesh.bake(
                    base,
                    view,
                    d.light,
                    foil,
                    &mut item_data.0,
                    &mut item_data.1,
                );
                for _ in 1..count {
                    let o = Vec3::new(
                        (rng.next_float() * 2.0 - 1.0) * 0.15,
                        (rng.next_float() * 2.0 - 1.0) * 0.15,
                        (rng.next_float() * 2.0 - 1.0) * 0.15,
                    );
                    mesh.bake(
                        base * Mat4::from_translation(o),
                        view,
                        d.light,
                        foil,
                        &mut item_data.0,
                        &mut item_data.1,
                    );
                }
            } else {
                let off_z = depth * 1.5;
                let mut m = base
                    * Mat4::from_translation(Vec3::new(
                        0.0,
                        0.0,
                        -(off_z * (count - 1) as f32 / 2.0),
                    ));
                mesh.bake(m, view, d.light, foil, &mut item_data.0, &mut item_data.1);
                m *= Mat4::from_translation(Vec3::new(0.0, 0.0, off_z));
                for _ in 1..count {
                    let o = Vec3::new(
                        (rng.next_float() * 2.0 - 1.0) * 0.075,
                        (rng.next_float() * 2.0 - 1.0) * 0.075,
                        0.0,
                    );
                    mesh.bake(
                        m * Mat4::from_translation(o),
                        view,
                        d.light,
                        foil,
                        &mut item_data.0,
                        &mut item_data.1,
                    );
                    m *= Mat4::from_translation(Vec3::new(0.0, 0.0, off_z));
                }
            }
        }
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
                let lr = |a: f32, b: f32| a + (b - a) * pt;
                let dy = (e.yaw - e.prev_yaw + 540.0).rem_euclid(360.0) - 180.0;
                list.push(P {
                    kind: name,
                    pos,
                    rot: (e.prev_yaw + dy * pt, lr(e.prev_pitch, e.pitch)),
                    light: crate::particles::light_at(level, pos[0], pos[1] + 0.1, pos[2]),
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

    fn bake_remote_players(
        &mut self,
        data: &mut Vec<u8>,
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
        for e in level.entities.iter().filter(|e| e.ty == player_ty) {
            let anim = self.remote_anims.entry(e.id).or_default();
            anim.swimming = e.swimming();
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
            };
            let root = self.avatar_root(pos, body, crouching, swim);
            let (model, parts) = &mut self.player_model;
            crate::settings::apply_parts(model, 0x7F);
            setup_anim(model, parts, &state);
            model.bake(root, [255; 4], light, data);
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
                    let arm = model.part_matrices(root)[arm_idx]
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
                // darknessEffectScale (доступность) = 1.0: опции нет.
                // VANILLA: GameRenderer#extract вызывает lightmap extract(state, 1.0F), а не покадровый pt:
                // иначе (tickCount - pt) идёт назад внутри тика и скачет на новом (пила вместо плавной волны).
                let pt = 1.0;
                let darkness_effect = self.effects.blend_factor("darkness", pt);
                let darkness_scale = ezzmc_render::lightmap::darkness_scale(
                    self.effects.ticks as f32,
                    pt,
                    darkness_effect,
                );
                let night_vision = if self.effects.has("night_vision") {
                    self.effects.night_vision_scale(pt)
                } else if self.fog_state.water_vision(self.player.under_water) > 0.0
                    && self.effects.has("conduit_power")
                {
                    self.fog_state.water_vision(self.player.under_water)
                } else {
                    0.0
                };
                let key = (
                    sky_factor,
                    sky_light_color,
                    self.cfg.gamma,
                    ambient_light,
                    ambient_color,
                    [darkness_effect, darkness_scale, night_vision],
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
                            ..LightmapParams::default()
                        },
                    );
                }
                // VANILLA: небо рисуется только при skybox=overworld (Nether: none, End: end_sky — TODO).
                self.sky_state =
                    (l.skybox == ezzmc_world::dimension::Skybox::Overworld).then_some(drawn);
                [
                    f64::from((c >> 16) & 255) / 255.0,
                    f64::from((c >> 8) & 255) / 255.0,
                    f64::from(c & 255) / 255.0,
                ]
            }
            _ => {
                self.sky_state = None;
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
        // VANILLA: CloudRenderer#prepare (cloudHeight 192.33, cloudRange 64 (GraphicsPreset fancy), FANCY).
        if let Ok(Some(l)) = self.net.level.lock().as_deref() {
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
                    192.33,
                    64,
                    self.cam,
                    l.game_time,
                    l.partial_tick(),
                );
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
            self.lightning
                .prepare(&gpu.device, &gpu.queue, mv, self.cam, &bolts);
        }
        self.prepare_hand(gpu, aspect, far);
        // VANILLA: AvatarRenderer — игрок виден только от 3-го лица (в 1-м лице камера в голове).
        self.prepare_player_model(gpu, mv);
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
        let compute = self.use_compute && self.vox.is_some();
        let view_proj = proj * mv;
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
            let hiz = if self.hiz_valid {
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
                    self.entity_draw.draw(&mut pass);
                    self.armor_draw.draw(&mut pass);
                    self.glint_draw.draw(&mut pass);
                    self.mobs.draw(&mut pass);
                    self.arrow_draw.draw(&mut pass);
                    self.piston_draw.draw(&mut pass);
                    self.shadow_draw.draw(&mut pass);
                    if let (true, Some(d)) = (self.entity_item_visible, &self.entity_item_draw) {
                        d.draw(&mut pass);
                        if let Some(g) = &self.entity_glint_draw {
                            g.draw(&mut pass);
                        }
                    }
                    pass.set_bind_group(0, &self.group0, &[]);
                    pass.set_bind_group(1, &self.group1, &[]);
                }
                if self.arena.draw_count(layer) == 0 {
                    continue;
                }
                pass.set_pipeline(self.pipes.pipeline(layer));
                self.arena.draw(&mut pass, layer);
            }
            if !compute {
                // VANILLA: облака рисуются после прозрачного слоя террейна.
                self.lightning.draw(&mut pass);
                self.clouds.draw(&mut pass);
                self.weather.draw(&mut pass);
                self.particle_draw.draw(&mut pass);
                // VANILLA: Hud#extractVignette (опция vignette:true) — поверх мира.
                self.vignette.prepare(&gpu.queue, self.vignette_b);
                self.vignette.draw(&mut pass);
            }
        }
        if compute {
            if let Some(z) = &self.hiz {
                z.build(&gpu.device, &mut enc, &depth);
            }
            let rest = [RenderLayer::Translucent, RenderLayer::Tripwire];
            if rest.iter().any(|l| self.arena.draw_count(*l) > 0) {
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
            let outline_vp = if self.walking {
                proj * self.bob((self.tick_acc / 0.05) as f32) * mv
            } else {
                view_proj
            };
            self.lines.set(
                &gpu.queue,
                outline_vp,
                [w as f32, h as f32],
                [0.0, 0.0, 0.0, 0.4],
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
            self.lines.draw(&mut pass);
        } else {
            self.lines.clear();
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
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            for l in &self.cross_lines {
                l.draw(&mut pass);
            }
        }
        if (self.hand_visible || self.hand_item_visible) && !self.hud_hidden {
            // VANILLA: рука рисуется после мира с очищенной глубиной.
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("hand"),
                color_attachments: &[Some(color_att(wgpu::LoadOp::Load))],
                depth_stencil_attachment: Some(depth_att(wgpu::LoadOp::Clear(1.0))),
                ..Default::default()
            });
            pass.set_bind_group(0, &self.group0, &[]);
            pass.set_bind_group(1, &self.group1, &[]);
            if self.hand_visible {
                self.hand_draw.draw(&mut pass);
            }
            if let (true, Some(d)) = (self.hand_item_visible, &self.hand_item_draw) {
                d.draw(&mut pass);
                if let Some(g) = &self.hand_glint_draw {
                    g.draw(&mut pass);
                }
            }
        }
        gpu.queue.submit([enc.finish()]);
        let _ = &self.lightmap;
    }

    /// VANILLA: FogRenderer#setupFog — тип среды по камере, атрибуты биома, цвет/дистанции.
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
            },
            &self.fog_state,
        )
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
            let kind = if suppress || wire_passes {
                None
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
            let seq = self.next_seq();
            let _ = self.net.cmd.send(crate::netthread::NetCmd::UseItem {
                seq,
                yaw: self.yaw,
                pitch: self.pitch,
            });
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
        let aim = self.target.filter(|_| self.have_pos).map(|t| Aim {
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
        // VANILLA: continueAttack → player.swing(MAIN_HAND) каждый тик удержания (пакет Punch здесь не реализован).
        if std::mem::take(&mut self.digger.swing) {
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
                    let seq = l.start_prediction();
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
            let _ = self.net.cmd.send(crate::netthread::NetCmd::Swing);
            self.player.swing();
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
            if let Ok(g) = self.net.level.lock() {
                if let Some(l) = g.as_ref() {
                    if let Some((id, dist)) = crate::pick::raycast_entity(l, o, d, reach) {
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
    let of = |id: u32| -> [u32; 3] {
        match biomes.get(id as usize) {
            Some(Some(b)) => [
                b.sky_color.unwrap_or(ezzmc_render::sky::DEFAULT_SKY_COLOR),
                b.fog_color.unwrap_or(defaults.base_fog),
                b.water_fog_color.unwrap_or(defaults.water_fog_color),
            ],
            _ => [
                ezzmc_render::sky::DEFAULT_SKY_COLOR,
                defaults.base_fog,
                defaults.water_fog_color,
            ],
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
