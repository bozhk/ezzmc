//! Связка клиент → арена террейна → render pass (M1.4): сетевые события → очередь перестроения →
//! rayon-меш → `TerrainArena` → отрисовка слоёв террейна. Камера свободная (M1.6, без физики).
use crate::netthread::{NetEvt, NetHandle};
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
use std::collections::HashSet;
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
const VERTEX_CAPACITY: u32 = 20_000_000;
const INDEX_ARENA_CAPACITY: u32 = 10_000_000;
const MAX_SECTIONS: u32 = 16_384;
const MAX_PENDING: usize = 48;

pub struct WorldView {
    /// 185: зажата ЛКМ (key.attack) — Minecraft#continueAttack.
    pub attack_held: bool,
    digger: crate::dig::Digger,
    net: NetHandle,
    /// Клавиши последнего кадра ходьбы (для player_input).
    last_input: crate::physics::Input,
    /// GameType локального игрока (0 survival, 1 creative…).
    pub game_mode: i8,
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
    player_model: (
        ezzmc_render::entitymodel::Model,
        ezzmc_render::humanoid::HumanoidParts,
    ),
    particles: crate::particles::ParticleEngine,
    particle_draw: ezzmc_render::particledraw::ParticleDraw,
    last_particle_tick: Option<i64>,
    /// Анимации чужих игроков по id сущности + game_time последнего тика анимаций.
    remote_anims: std::collections::HashMap<i32, crate::avatar::AvatarAnim>,
    remote_tick: Option<i64>,
    sky_state: Option<ezzmc_render::sky::SkyState>,
    fog_state: ezzmc_render::fog::FogState,
    vignette: ezzmc_render::vignette::VignetteDraw,
    /// Hud#vignetteBrightness и накопитель тиков.
    vignette_b: f32,
    vignette_acc: f32,
    last_frame: Option<std::time::Instant>,
    lightmap_key: Option<(f32, [f32; 3])>,
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
    fog_buf: wgpu::Buffer,
    globals_buf: wgpu::Buffer,
    chunk_buf: wgpu::Buffer,
    group0: wgpu::BindGroup,
    depth: Option<(wgpu::TextureView, u32, u32)>,
    gpu_queue: wgpu::Queue,
    color_format: wgpu::TextureFormat,
    atlas_view: wgpu::TextureView,
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
    min_y: i32,
    height: i32,
    view_radius: i32,
    pub cam: [f64; 3],
    pub have_pos: bool,
    /// Физика игрока (ходьба); `walking == false` — отладочный свободный полёт (EZZMC_FREECAM=1).
    pub player: crate::physics::Player,
    pub walking: bool,
    tick_acc: f64,
    /// VANILLA: GameRenderer#fovModifier / oldFovModifier (сглаживание 0.5 за тик).
    fov_mod: (f32, f32),
    /// Отладка (EZZMC_TOUR): не слать серверу позицию камеры.
    pub no_send: bool,
    pub yaw: f32,
    pub pitch: f32,
    /// VANILLA: CameraType (0 = FIRST_PERSON, 1 = THIRD_PERSON_BACK, 2 = THIRD_PERSON_FRONT).
    pub perspective: u8,
    pub status: Option<String>,
    pub joined: bool,
    lines: ezzmc_render::lines::LineDraw,
    /// Блок под прицелом (контур) и жидкость под прицелом (F3); обновляются в `update`.
    pub target: Option<crate::pick::Target>,
    pub fluid_target: Option<crate::pick::Target>,
    /// Сущность под прицелом (id) — ближе блока (Minecraft#hitResult = EntityHitResult).
    pub entity_target: Option<i32>,
    /// Секций, прошедших frustum в последнем кадре (F3 `C:`).
    pub visible_sections: usize,
}

fn buf(device: &wgpu::Device, label: &str, size: usize) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: size as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn upload_atlas(gpu: &Gpu, atlas: &BlockAtlas) -> wgpu::TextureView {
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
    tex.create_view(&wgpu::TextureViewDescriptor::default())
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
#[derive(Clone, Copy, Default)]
struct SwimPose {
    amount: f32,
    in_water: bool,
    visually: bool,
    x_rot: f32,
}

impl SwimPose {
    fn matrix(self) -> Mat4 {
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
        let atlas_view = upload_atlas(gpu, &atlas);
        let atlas_sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("atlas"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
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
        let arena = TerrainArena::new(
            &gpu.device,
            &gpu.queue,
            VERTEX_CAPACITY,
            INDEX_ARENA_CAPACITY,
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
            for name in ["rain", "smoke"] {
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
            use_compute,
            vox,
            rects: RectTable::default(),
            vox_resident: HashSet::new(),
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
            player_model: ezzmc_render::humanoid::player_mesh(false),
            particles: crate::particles::ParticleEngine::new(sets),
            particle_draw,
            last_particle_tick: None,
            remote_anims: Default::default(),
            remote_tick: None,
            sky_state: None,
            fog_state: ezzmc_render::fog::FogState::default(),
            vignette,
            vignette_b: 1.0,
            vignette_acc: 0.0,
            last_frame: None,
            lightmap_key: None,
            group1,
            projection_buf,
            hand_proj_buf,
            hand_draw,
            hand_model,
            hand_visible: false,
            fog_buf,
            globals_buf,
            chunk_buf,
            group0,
            depth: None,
            atlas_size: (atlas.width, atlas.height),
            resident: HashSet::new(),
            min_y: -64,
            height: 384,
            view_radius: 8,
            cam: [0.0, 100.0, 0.0],
            have_pos: false,
            player: crate::physics::Player::new([0.0; 3]),
            last_input: crate::physics::Input::default(),
            game_mode: 0,
            inv: crate::inventory::Inventory::default(),
            attack_held: false,
            digger: crate::dig::Digger::default(),
            walking: std::env::var("EZZMC_FREECAM").map_or(true, |v| v != "1"),
            tick_acc: 0.0,
            fov_mod: (1.0, 1.0),
            no_send: false,
            yaw: 0.0,
            pitch: 0.0,
            perspective: 0,
            status: None,
            joined: false,
            lines: ezzmc_render::lines::LineDraw::new(&gpu.device, color, DEPTH_FORMAT),
            target: None,
            fluid_target: None,
            entity_target: None,
            visible_sections: 0,
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
        let mut cols: HashSet<(i32, i32)> = HashSet::new();
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
        while let Ok(ev) = self.net.rx.try_recv() {
            match ev {
                NetEvt::Joined {
                    biomes,
                    min_y,
                    height,
                    view_radius,
                } => {
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
                    self.joined = true;
                    tracing::info!(
                        "вход в мир: y {min_y}..{} радиус {view_radius}",
                        min_y + height
                    );
                }
                NetEvt::GameMode(m) => self.game_mode = m,
                NetEvt::Inventory(iev) => self.inv.apply(iev),
                NetEvt::Abilities { flags, fly_speed } => {
                    // VANILLA: ClientPacketListener#handlePlayerAbilities.
                    self.player.mayfly = flags & 4 != 0;
                    self.player.flying = flags & 2 != 0;
                    self.player.fly_speed = fly_speed;
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
                } => {
                    // VANILLA: ClientPacketListener#handleMovePlayer — позиция сервера всегда применяется
                    // (раньше только первая: после /tp камера оставалась на старом месте и сервер
                    // перестал слать чанки).
                    self.player.teleport(pos);
                    self.player.vel = vel;
                    self.cam = [pos[0], pos[1] + self.player.eye_height(), pos[2]];
                    self.yaw = yaw;
                    self.pitch = pitch;
                    self.have_pos = true;
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
                        // VANILLA: LevelEvent.PARTICLES_DESTROY_BLOCK
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
    fn tick_particles(&mut self) {
        let Ok(g) = self.net.level.lock() else { return };
        let Some(level) = g.as_ref() else { return };
        let now = level.game_time;
        let n = match self.last_particle_tick.replace(now) {
            Some(prev) => (now - prev).clamp(0, 10),
            None => 0,
        };
        for _ in 0..n {
            self.particles.tick(level);
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
            FLAG_FLYING, FLAG_HCOL, FLAG_INPUT_SHIFT, FLAG_ON_GROUND, FLAG_SPRINTING,
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
    }

    /// Раз в кадр: события сети, постановка в меш, заливка готовых мешей в арену.
    pub fn update(&mut self, gpu: &Gpu) {
        // Лок до разбора событий: см. телепорт в netthread.rs.
        let shared_pos = self.net.pos.clone();
        let mut guard = shared_pos.lock().ok();
        self.drain_events();
        if self.have_pos && !self.no_send {
            // Сетевой поток шлёт это серверу как позицию игрока (иначе новые чанки не приходят).
            if let Some(g) = guard.as_mut() {
                let feet = if self.walking {
                    self.player.pos
                } else {
                    [self.cam[0], self.cam[1] - 1.62, self.cam[2]]
                };
                **g = Some([
                    feet[0],
                    feet[1],
                    feet[2],
                    f64::from(self.yaw),
                    f64::from(self.pitch),
                    f64::from(self.net_flags()),
                ]);
            }
        }
        drop(guard);
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
                    self.resident.insert(r.pos);
                }
                Err(e) => tracing::warn!("арена: {e}"),
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
        self.tick_acc += dt.min(0.25);
        let (yaw, pitch) = (self.yaw, self.pitch);
        self.player.pitch = pitch;
        let mut n = 0;
        while self.tick_acc >= 0.05 && n < 5 {
            self.tick_acc -= 0.05;
            n += 1;
            let p = &mut self.player;
            let mut self_fov = None;
            let ran = self.net.level.lock().ok().and_then(|g| {
                g.as_ref().map(|l| {
                    // Мир под игроком ещё не пришёл — стоим (ваниль показывает экран загрузки).
                    if crate::physics::Collider::has_chunk(
                        l,
                        p.pos[0].floor() as i32,
                        p.pos[2].floor() as i32,
                    ) {
                        p.tick(l, input, yaw);
                        p.post_tick(yaw, pitch);
                        self_fov = Some(p.fov_target());
                    }
                })
            });
            let _ = ran;
            if self_fov.is_some() {
                self.tick_dig();
            }
            if let Some(t) = self_fov {
                let (_, cur) = self.fov_mod;
                self.fov_mod = (cur, cur + (t - cur) * 0.5);
            }
        }
        if n == 5 {
            self.tick_acc = 0.0;
        }
        self.cam = self.player.eye_lerp(self.tick_acc / 0.05);
    }

    /// Текущий FOV с динамическим множителем (спринт/полёт), интерполяция по partial tick.
    fn fov_deg(&self) -> f32 {
        let pt = (self.tick_acc / 0.05) as f32;
        let (o, c) = self.fov_mod;
        FOV_DEG * (o + (c - o) * pt)
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
        if self.perspective != 0 && self.walking && self.have_pos {
            let pt = (self.tick_acc / 0.05).clamp(0.0, 1.0) as f32;
            let p = &self.player;
            let lerp = |a: f64, b: f64| a + (b - a) * f64::from(pt);
            let pos = [
                lerp(p.prev_pos[0], p.pos[0]),
                lerp(p.prev_pos[1], p.pos[1]),
                lerp(p.prev_pos[2], p.pos[2]),
            ];
            let state = p.anim.humanoid_state(pt, p.crouching);
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
                visually: p.swimming,
                x_rot: state.x_rot,
            };
            let root = self.avatar_root(pos, body, p.crouching, swim);
            let (model, parts) = &mut self.player_model;
            setup_anim(model, parts, &state);
            model.bake(root, [255; 4], light, &mut data);
        }
        self.bake_remote_players(&mut data);
        self.entity_draw
            .prepare_raw(&gpu.device, &gpu.queue, mv, &data);
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
        let (model, parts) = &mut self.hand_model;
        model.reset_pose();
        model.parts[parts.right_arm].pose.z_rot = 0.1;
        let mut data = Vec::new();
        model.bake(m, [255; 4], light, &mut data);
        self.hand_draw
            .prepare_raw(&gpu.device, &gpu.queue, Mat4::IDENTITY, &data);
        let proj = Mat4::perspective_rh(70.0f32.to_radians(), aspect, NEAR, far)
            * self.player.bob_matrix(pt);
        gpu.queue.write_buffer(
            &self.hand_proj_buf,
            0,
            &ProjectionUniform { proj_mat: proj }.to_bytes(),
        );
        self.hand_visible = true;
    }

    /// Чужие игроки (TODO-211, часть): модель Стива, анимации по тикам `game_time`.
    /// VANILLA-UNSURE: приседание/плавание/скины/слой плаща по метаданным не разобраны.
    fn bake_remote_players(&mut self, data: &mut Vec<u8>) {
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
            // VANILLA: Entity#isInvisible → LivingEntityRenderer не рисует модель (без брони/предметов).
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
            let state = anim.humanoid_state(pt, crouching);
            let body = anim.body_rot(pt);
            let swim = SwimPose {
                amount: state.swim_amount,
                in_water: wet,
                visually: swimming,
                x_rot: state.x_rot,
            };
            let root = self.avatar_root(pos, body, crouching, swim);
            let (model, parts) = &mut self.player_model;
            setup_anim(model, parts, &state);
            model.bake(root, [255; 4], light, data);
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
                // VANILLA-UNSURE: без сглаживания CubicSampler по соседним биомам.
                let (bx, by, bz) = (
                    self.cam[0].floor() as i32,
                    self.cam[1].floor() as i32,
                    self.cam[2].floor() as i32,
                );
                let base_sky = l
                    .chunks
                    .get(bx >> 4, bz >> 4)
                    .and_then(|c| {
                        let id = c.chunk.biome((bx & 15) as usize, by, (bz & 15) as usize);
                        self.models
                            .tints
                            .biomes
                            .get(id as usize)?
                            .as_ref()?
                            .sky_color
                    })
                    .unwrap_or(ezzmc_render::sky::DEFAULT_SKY_COLOR);
                let mut st =
                    ezzmc_render::sky::sky_state(l.day_time, l.rain_level, l.thunder(), base_sky);
                // VANILLA: ClientLevel#addEnvironmentAttributeLayers — вспышка молнии:
                // SKY_COLOR = srgbLerp(0.22, sky, (0.8, 0.8, 1.0)), SKY_LIGHT_FACTOR = 1.
                // VANILLA-UNSURE: цвет тумана (compute_fog) вспышку не учитывает.
                if l.sky_flash_time > 0 {
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
                let key = (st.sky_light_factor, st.sky_light_color);
                if self.lightmap_key != Some(key) {
                    self.lightmap_key = Some(key);
                    self.lightmap.update(
                        &gpu.queue,
                        &LightmapParams {
                            sky_factor: key.0,
                            sky_light_color: key.1,
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
                    ezzmc_render::clouddraw::CloudStatus::Fancy,
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
                    proj * self.player.bob_matrix((self.tick_acc / 0.05) as f32)
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
        let mut fr = Frustum::new(proj, mv, true);
        fr.prepare(self.cam[0], self.cam[1], self.cam[2]);
        let visible: Vec<SectionPos> = self
            .resident
            .iter()
            .copied()
            .filter(|p| fr.section_visible(p.x, p.y, p.z))
            .collect();
        self.visible_sections = visible.len();
        // пересортировка прозрачных квадов у видимых секций
        for p in &visible {
            self.arena.resort(q, *p, self.cam);
        }
        self.arena.prepare_draws(q, &visible, self.cam);
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
            if let Some(st) = &self.sky_state {
                // VANILLA: shouldRenderDarkDisc — глаза ниже горизонта (63 в Overworld).
                self.sky.prepare(&gpu.queue, mv, st, self.cam[1] < 63.0);
                self.sky.draw(&mut pass);
            }
            if compute {
                if let Some(v) = &self.vox {
                    v.draw(&mut pass);
                }
            }
            pass.set_bind_group(0, &self.group0, &[]);
            pass.set_bind_group(1, &self.group1, &[]);
            for &layer in first_layers {
                if self.arena.draw_count(layer) == 0 {
                    continue;
                }
                pass.set_pipeline(self.pipes.pipeline(layer));
                self.arena.draw(&mut pass, layer);
            }
            if !compute {
                // VANILLA: облака рисуются после прозрачного слоя террейна.
                self.entity_draw.draw(&mut pass);
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
            let o = ezzmc_mesh::blockinfo::block_offset(t.state, t.pos[0], t.pos[2]);
            let off = [off[0] + o[0], off[1] + o[1], off[2] + o[2]];
            let segs: Vec<_> = ezzmc_mesh::blockinfo::outline_boxes(t.state)
                .iter()
                .flat_map(|b| ezzmc_render::lines::box_edges(b, off))
                .collect();
            let width = (w as f32 / 1920.0 * 2.5).max(2.5);
            // VANILLA: боббинг входит в матрицу проекции — контур качается вместе с миром.
            let outline_vp = if self.walking {
                proj * self.player.bob_matrix((self.tick_acc / 0.05) as f32) * mv
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
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            self.lines.draw(&mut pass);
        } else {
            self.lines.clear();
        }
        if self.hand_visible {
            // VANILLA: рука рисуется после мира с очищенной глубиной.
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("hand"),
                color_attachments: &[Some(color_att(wgpu::LoadOp::Load))],
                depth_stencil_attachment: Some(depth_att(wgpu::LoadOp::Clear(1.0))),
                ..Default::default()
            });
            pass.set_bind_group(0, &self.group0, &[]);
            pass.set_bind_group(1, &self.group1, &[]);
            self.hand_draw.draw(&mut pass);
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
        let mut sky_light = 15u8;
        let mut block_light = 0u8;
        if let Some(c) = l.chunks.get(bx >> 4, bz >> 4) {
            let (lx, lz) = ((bx & 15) as usize, (bz & 15) as usize);
            let id = c.chunk.biome(lx, by, lz) as usize;
            if let Some(Some(b)) = self.models.tints.biomes.get(id) {
                attrs.base_sky = b.sky_color.unwrap_or(attrs.base_sky);
                attrs.base_fog = b.fog_color.unwrap_or(attrs.base_fog);
                attrs.water_fog_color = b.water_fog_color.unwrap_or(attrs.water_fog_color);
            }
            sky_light = c.sky_light(lx, by, lz).unwrap_or(15);
            block_light = c.block_light(lx, by, lz).unwrap_or(0);
        }
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
        let in_water = matches!(fog_type, ezzmc_render::fog::FogType::Water);
        self.fog_state.tick_water_vision(in_water, dt * 20.0, false);
        let water_vision = self.fog_state.water_vision(in_water);
        setup_fog(
            &FogInput {
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
                void_darkness_onset: 32.0,
                water_vision,
                cloud_range: 64,
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
        if let Some(id) = self.entity_target {
            let _ = self
                .net
                .cmd
                .send(crate::netthread::NetCmd::Attack { entity_id: id });
            let _ = self.net.cmd.send(crate::netthread::NetCmd::Swing);
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
        let Some(t) = self.target.filter(|_| self.have_pos && self.game_mode != 3) else {
            return;
        };
        let seq = self
            .net
            .level
            .lock()
            .ok()
            .and_then(|mut g| g.as_mut().map(ezzmc_world::level::Level::start_prediction))
            .unwrap_or(0);
        let rel = [
            (t.hit[0] - f64::from(t.pos[0])) as f32,
            (t.hit[1] - f64::from(t.pos[1])) as f32,
            (t.hit[2] - f64::from(t.pos[2])) as f32,
        ];
        let _ = self.net.cmd.send(crate::netthread::NetCmd::UseItemOn {
            pos: t.pos,
            face: t.face,
            rel,
            inside: t.inside,
            seq,
        });
    }

    fn tick_dig(&mut self) {
        use crate::dig::{Aim, Body};
        let aim = self.target.filter(|_| self.have_pos).map(|t| Aim {
            pos: t.pos,
            face: t.face,
            state: t.state,
        });
        let body = Body {
            on_ground: self.player.on_ground,
            under_water: self.player.under_water,
        };
        let (cmds, broken) = self
            .digger
            .tick(self.attack_held, aim, self.game_mode, body);
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
                            l.predict_block(
                                ezzmc_protocol::types::BlockPos::new(p[0], p[1], p[2]),
                                ezzmc_world::block::BlockState::AIR,
                            );
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
        // VANILLA: GameRenderer#pick — сущность выбирается, если ближе блока; дальность 3.0 (креатив 5.0).
        let mut ent = None;
        if self.have_pos && self.game_mode != 3 {
            let reach = if self.game_mode == 1 { 5.0 } else { 3.0 };
            if let Ok(g) = self.net.level.lock() {
                if let Some(l) = g.as_ref() {
                    if let Some((id, dist)) = crate::pick::raycast_entity(l, o, d, reach) {
                        let block_d = t.map_or(f64::INFINITY, |b| {
                            let v = [b.hit[0] - o[0], b.hit[1] - o[1], b.hit[2] - o[2]];
                            (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
                        });
                        if dist < block_d {
                            ent = Some(id);
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
