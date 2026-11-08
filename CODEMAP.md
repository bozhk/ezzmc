# CODEMAP.md — карта кода ezzmc

> **Зачем:** чтобы не искать код по всему проекту. Нужен кусок логики → смотришь сюда → идёшь сразу в нужный файл.
> **Правила ведения — `GOAL.md §7.9`.** Кратко: создал/переместил/переименовал/удалил файл или добавил важную функцию/тип — **обнови эту карту в том же коммите**. Карта устарела = баг (`[codemap]` в STATUS.md → Баги).
> Читай `GOAL.md` и `STATUS.md` первыми; CODEMAP — следующий шаг перед поиском по коду.

## Как пользоваться
1. Ищи по ключевым словам (`Ctrl+F`) в разделе «Индекс по темам» ниже.
2. Нашёл файл → открывай только нужный диапазон, не весь файл.
3. Не нашёл в карте → ищи по коду (`rg`), **затем добавь найденное сюда**.
4. Не пиши в карту код. Только: путь, что внутри, ключевые элементы, связи.

## Формат записи
```
### `путь/к/файлу.rs`
- **Что:** одна строка — назначение файла.
- **Ключевое:** `Тип`, `fn имя()` — по 1 строке на важное (не перечисляй всё).
- **Ванильный эталон:** `net.minecraft...Класс#метод` (если порт ванили; иначе —)
- **Связи:** кто вызывает / что использует (крейты/файлы).
- **Статус:** ✅ готово · 🚧 в работе · ⚠️ есть баги (BUG-NNN) · 📝 заглушка (TODO-NNN)
```

Правила записей: пути от корня репозитория; одна запись на файл; запись ≤ 6 строк; сортировка по крейтам; при удалении файла — удалить запись (не оставлять мёртвые).

---

## Индекс по темам
(тема → где искать; добавляй строки по мере появления кода)

| Тема | Файл / крейт |
|---|---|
| Таблица версий и protocol number | `data/versions.toml` ✅ |
| Кодогенерация из `--reports` | `xtask/` (reports: `cargo xtask gen-reports <ver>` → `reference/<ver>/reports/`; `gen-protocol <ver>` → `crates/protocol/src/generated/v*.rs`; `gen-registries <ver>` → `generated/r*.rs`; `gen-blockstates <ver>` → `crates/world/src/generated/b*.rs`; `gen-blockprops <ver>` → `generated/p*.rs`, свойства/формы блоков; `gen-tags <ver>` → `generated/t*.rs`, теги) |
| Эталонный исходник (не в git) | `reference/<версия>/client/` (`cargo xtask fetch-reference <ver>`), выжимки — `docs/ref/` |
| Запись/воспроизведение трафика | `crates/net/src/replay.rs` (`.ezrec`, `record_proxy`, `replay_clientbound`); `cargo xtask record` ✅ |
| Сетевой фрейминг / VarInt | — |
| Шифрование / компрессия | — |
| Кодеки пакетов по версиям | — |
| Трансляторы версий | — |
| Чанки / палитры / свет | — |
| Меш и рендер | — |
| Физика / движение | — |
| Инвентарь / UI | — |
| Тесты, golden-данные | `tests-integration/` |
| Бенчмарки | `benches/` (`cargo xtask bench`) ✅ |

---

## Карта по крейтам

### Корень
- `CLAUDE.md` / `AGENTS.md` — входная инструкция агентам (CLAUDE.md главная).
- `GOAL.md` — цели, ванильность, правила агентов, лимиты и штрафы.
- `STATUS.md` — журнал: фокус, баги, TODO, решения, матрица версий.
- `TASKS.md` — бэклог по главам 0–21, нумерация TODO-NNN.
- `CODEMAP.md` — этот файл.
- `Cargo.toml` — workspace, профили сборки, `[workspace.dependencies]` внутренних крейтов ✅
- `benches/` — criterion-бенчи (`benches/smoke.rs`), член workspace ✅
- `.github/workflows/ci.yml` — fmt + clippy + test ✅
- `rust-toolchain.toml` — stable + rustfmt/clippy ✅

### `crates/protocol/`
- **Что:** `src/install.rs` (`GameInstall::locate(ver)`: env `EZZMC_CLIENT_JAR`/`EZZMC_MINECRAFT_DIR`, `.minecraft`, dev-кэш `reference/_cache`; `index_path`) ✅ 089; `src/source.rs` (трейт `ResourceSource` read/exists/list; `ZipSource` mmap+libdeflater, `DirSource`, `ObjectStore` по hash индекса; защита от `..`) ✅ 090; `src/pack.rs` (`PackMeta`, `Pack`, `ResourceManager`, `CURRENT_FORMAT`) ✅ 091; `src/stitcher.rs` (`stitch`, `Entry`, `Stitched`) ✅ 092; `src/texture.rs` (`Image`, `generate_mip_levels`, `MipmapStrategy`, `solidify`) ✅ 092; `src/atlas.rs` (`Identifier`, `load_sources`, `resolve`, `SpriteLoad`, `missing_image`) ✅ 093; `src/error.rs`.

### `crates/protocol/wire.rs`
- `wire.rs`: `Reader`/`Writer` (VarInt/VarLong, string, uuid, block_pos, angle, bitset, optional) ✅ 048
- `nbt.rs`: `Tag`, бинарный named/network, SNBT (разбор+вывод), MUTF-8, лимиты ✅ 055
- `text.rs`: `Component` из JSON/NBT, `Style`, click/hover, `to_plain` с translate ✅ 056
- `types.rs`: `BlockPos`, `ChunkPos`, `Vec3`, `GameProfile`, `offline_uuid` ✅ 057a

### `crates/net/`
- **Что:** `error.rs` (`NetError`/`NetResult`) ✅ 025, `replay.rs` ✅ 023, `frame.rs` (`Codec`: фрейм+zlib, лимиты 2^21/8MiB) ✅ 049/050, `conn.rs` (`Connection`: connect/handshake/send/recv, автомат состояний, `Intent`) ✅ 047/052/053, `session.rs` (`PacketIds`, `login_offline`, `configure` → `ServerConfig`: реестры/теги/features/known packs, `tag_contains`; живой тест `#[ignore]`) ✅ 059/062ч/064/070/080–084, `play.rs` (`PlayHandler`: вход в Play, телепорт-подтверждение, keep-alive, chunk_batch_received; `PlayEvent`; `MoveTracker`, `send_move`/`send_player_input`/`send_sprint`, `read_lp_vec3`, `PlayEvent::Motion`) ✅ 457, 169–171, `srv.rs` (парсинг адреса, SRV через свой UDP-DNS) ✅ 047

### `crates/world/`
- **Что:** `src/install.rs` (`GameInstall::locate(ver)`: env `EZZMC_CLIENT_JAR`/`EZZMC_MINECRAFT_DIR`, `.minecraft`, dev-кэш `reference/_cache`; `index_path`) ✅ 089; `src/source.rs` (трейт `ResourceSource` read/exists/list; `ZipSource` mmap+libdeflater, `DirSource`, `ObjectStore` по hash индекса; защита от `..`) ✅ 090; `src/error.rs`.

### `crates/mesh/`
- **137–146:** `section.rs` (ModelProvider/Neighborhood/`mesh_section`), `lighting.rs`+`ao_table.rs` (138/139), `layers.rs`+`layers_table.rs` (141), `sort.rs` (142), `frustum.rs`+`occlusion.rs`+`vis.rs` (143), `fluid.rs` (145), `blockinfo.rs` (флаги, `Fluid`, `block_offset` — 146), `models.rs`/`padded.rs`, `assets/block_atlas.rs`.
- **Что (M1.4):** `src/vertex.rs` (`PackedVertex` 16 Б: pos u16×3 `(c+8)*2048`, light u16 block|sky<<4|ao<<8, uv unorm16×2, цвет u8×4; `pack_quad`) ✅ 135; `src/cull.rs` (`should_render_face` — порт Block#shouldRenderFace, `skip_rendering`, перекрытие граней по форме, кэш thread-local) ✅ 136; `src/section.rs` (`SectionPos`, `ModelProvider`, `Neighborhood` 3×3, `mesh_section`, `SectionMesh` по 3 слоям) ✅ 133; `src/queue.rs` (`RebuildQueue`: ближние первыми, штраф за спиной, `mark_block` с соседями) ✅ 133; `src/pool.rs` (`MeshPool`: rayon + crossbeam, поколения отбрасывают устаревшее) ✅ 134.
- **Что:** `src/install.rs` (`GameInstall::locate(ver)`: env `EZZMC_CLIENT_JAR`/`EZZMC_MINECRAFT_DIR`, `.minecraft`, dev-кэш `reference/_cache`; `index_path`) ✅ 089; `src/source.rs` (трейт `ResourceSource` read/exists/list; `ZipSource` mmap+libdeflater, `DirSource`, `ObjectStore` по hash индекса; защита от `..`) ✅ 090; `src/error.rs`.

### `crates/render/`
- **490:** `src/gpu_cull.rs` (`GpuCuller::new/cull/draw`, `gpu_layer_mask`) + `src/shaders/cull.wgsl` (frustum-тест секций в compute, вывод компактный+счётчик при `MULTI_DRAW_INDIRECT_COUNT`, иначе слот→слот); `TerrainArena`: `src_cmds_buf`, `flags_buf`, `set_visible`, `flush_removed`, `slot_count`. ✅ 490 (прозрачный слой — только CPU).
- **144/140:** `src/terrain.rs` (`RangeAllocator`, `ArenaLayout`, `TerrainArena` upload/resort/prepare_draws/draw, `TerrainPipelines` по слоям, `vertex_layout`) ✅ 144; `src/lightmap.rs` (`compute`, `LightmapParams`, `Flicker`, `LightmapTexture`) ✅ 140.
- **Что:** `gpu.rs` — `Gpu` (instance/adapter/device/queue/surface), `BackendChoice` (env `EZZMC_BACKEND`), цепочка fallback PRIMARY→GL→программный, `Gpu::headless` для тестов. ✅ TODO-032; `swapchain.rs` — `Swapchain` (configure/resize/`set_present_mode`/`acquire`→`Acquire::{Frame,Skip,DeviceLost}`, `clear_and_present`), `PresentMode::{Vsync,Mailbox,Immediate}.resolve` ✅ TODO-033

### `crates/protocol/`
- **Что:** `error.rs` (`ProtocolError`/`ProtocolResult`, thiserror), типы `State`/`Direction`/`PacketInfo` (`lib.rs`) + таблицы id пакетов `generated/v<ver>.rs` и реестров `generated/r<ver>.rs` (`ITEM[id]` → имя) (@generated, не править). ✅ для 1.21.11

### `crates/world/`
- **Что:** `registry.rs` (`Registry`, `Registries`: id↔name, динамические из registry_data) ✅ 113; `block.rs` (`BlockState(u16)` ✅ 112, `BlockInfo`/`PropInfo`, `decode_state`, `StateProps`, `in_tag`) + `generated/{b,p,t}<ver>.rs` (@generated); `palette.rs` (`PalettedContainer`, `Kind::{BlockStates,Biomes}`, `ceil_log2`, read/write) ✅ 114; `chunk.rs` (`ChunkFormat`, `Section`, `Chunk`: block/set_block/biome) ✅ 115/116; `chunk_packet.rs` (`LevelChunkPacket::read`, `HeightmapType`, `BlockEntityInfo`) ✅ 117; `light.rs` (`DataLayer`, `LightUpdate`) ✅ 121; `storage.rs` (`ChunkCache` кольцо, `ClientChunk`, `ChunkLight`, sky_light/block_light) ✅ 118/120; `level.rs` (`Level::handle(name, body)` → `LevelEvent`; время, погода) ✅ 118/119/129ч; `dimension.rs` (`DimensionType::from_nbt`, `chunk_format`) ✅ 130. Живой тест — `net/session.rs::live_world_chunks` (`--ignored`).

### `crates/assets/`
- **Что:** `src/install.rs` (`GameInstall::locate(ver)`: env `EZZMC_CLIENT_JAR`/`EZZMC_MINECRAFT_DIR`, `.minecraft`, dev-кэш `reference/_cache`; `index_path`) ✅ 089; `src/source.rs` (трейт `ResourceSource` read/exists/list; `ZipSource` mmap+libdeflater, `DirSource`, `ObjectStore` по hash индекса; защита от `..`) ✅ 090; `src/error.rs`.

### `crates/client/`
- **Что:** `src/main.rs` (winit `ApplicationHandler`, флаг `--log-packets`), `src/threads.rs` (модель потоков: `Worker<Cmd,Evt>` на crossbeam, `mesh_pool` rayon ядра−2; main=winit+тик+рендер) ✅ TODO-035, `src/input.rs` (`Mouse`: сырая дельта, grab/release, `sensitivity_factor` (s*0.6+0.2)³·8, `apply_turn` pitch±90; `InputState`; keymap winit↔GLFW↔`key.keyboard.*`) ✅ TODO-036, `src/keymap.rs` (`KeyBindings`: ванильные умолчания Options.key*, `rebind`/`conflicts`/`reset_all`, down/click как KeyMapping, `to_options_lines`/`load_options` в формате `key_<name>:key.keyboard.x`) ✅ TODO-037, `src/focus.rs` (`Focus`: windowActive, автопауза через 500 мс при `pause_on_lost_focus`) ✅ TODO-039, `src/display.rs` (`FramerateLimit` — троттлинг как FramerateLimitTracker, `FrameLimiter`, `calculate_scale`/`scaled_size` GUI-масштаб, `present_mode(vsync)`) ✅ TODO-040, `src/screenshot.rs` (`save`, `unique_path`, `timestamp_name` — `screenshots/yyyy-MM-dd_HH.mm.ss[_N].png`; F2 из keymap) ✅ TODO-041, `src/clipboard.rs` (`Clipboard` get/set поверх arboard, лениво, локальный запас при сбое; трейт `TextClipboard` для тестов) ✅ TODO-043, `src/timer.rs` (`Timer` — порт DeltaTracker.Timer: `advance_time`→тики, `partial_tick`, pause/freeze; `Clock`), `src/window.rs` (`WindowState` — fullscreen/размер окна + options-строки, `toggle_fullscreen` на F11 ✅ TODO-042, `WindowConfig`, `create`, `framebuffer_size`, иконка; 854×480 как в ванили) ✅ TODO-031, `src/log.rs` (tracing: `init`, `build_filter`; env `EZZMC_LOG`/`RUST_LOG`, файл-лог, цель `packets` по флагу). ✅ TODO-024

### `xtask/`
- **Что:** `cargo xtask <cmd>` (алиас `.cargo/config.toml`).
- **Ключевое:** `gen-reports` (`gen_reports.rs`), `gen-protocol` (`gen_protocol.rs`), `gen-registries` (`gen_registries.rs`), `gen-blockstates` (`gen_blockstates.rs`), `gen-blockprops` (`gen_blockprops.rs`), `gen-tags` (`gen_tags.rs`), `capture-golden` (`capture_golden.rs`), `server <ver> [порт] [--reset]` (`server.rs`: ванильный offline-сервер в `reference/_server/<ver>/`); `record <proto> <listen> <target> <out.ezrec>` (прокси-рекордер); `bench`; `fetch-reference <ver>` (`fetch_reference.rs`: скачивание+remap+decompile; до 1.14.4 — `legacy_named_jar`, Yarn; `mappings.rs`: ProGuard→Tiny, тест). 🚧 TODO-016+

### `tests-integration/`
- **Что:** крейт `ezzmc-tests-integration`: `src/lib.rs` (формат и правила golden, `load`/`cases`/`same_f32`), `golden/<ver>/<name>.json` (данные с ванили), `golden/tools/<Name>.java` (снятие), `tests/golden_format.rs`.
- **Снять:** `cargo xtask capture-golden <ver> <Name>` (`capture_golden.rs`; библиотеки клиента докачиваются в `reference/_cache/<ver>/libs`). Пример: `MthSin` (Mth.sin/cos, float-биты). ✅

### `benches/`
(пока нет кода)

---

## Журнал карты
- 2026-11-07 — Создан каркас CODEMAP.md. Кода нет.

### Дополнение M1.3/M1.4 (2026-11-07)
- `crates/assets/src/`: `animation.rs` (.mcmeta анимации) 094; `model.rs` (JSON-модели, parent, слоты текстур) 095; `blockstate.rs` (variants/multipart, веса) 096; `bake.rs` (FaceBakery: `bake_elements`, `ModelState`, uvlock, `QuadCollection`) 098.
- `crates/world/src/`: `noise.rs` (LegacyRandom, Simplex/PerlinSimplexNoise) и `biome_color.rs` (`ColorMaps`, `BiomeColors::from_nbt`, `GrassColorModifier`, `blend_tint`) — 101.
- `crates/render/src/`: `shaders.rs` (`Shader`, naga-валидация) + `shaders/*.wgsl` (common, terrain, sky, stars, clouds, lightmap, position_color, position_tex_color, particle) и `uniforms.rs` (8 uniform-структур с проверкой размеров) — 103.

- ⚠ Compute-путь ниже заморожен (DEC-013): по умолчанию не включается, не развивать.
- `crates/render/src/` (воксельный путь ComputeIndirect; в клиенте под флагом `--compute-renderer`): `pipeline_mode.rs` (`GpuVendor`, `RenderPipelineMode::select/detect`), `voxel_vertex.rs` (`VoxelVertex` u32 + `QuadMeta`), `hiz.rs` (`HiZ`: R32Float мип-цепочка max-глубины из Depth32Float), `renderer.rs` (`Renderer`: монолитный storage-буфер + `RangeAllocator`, `upload_section/remove_section`, `encode_cull` frustum+Hi-Z, `draw` multi-draw-indirect[_count], `supported(gpu)`), `voxel_adapter.rs` (`split_mesh`: ванильный `SectionMesh` → `Converted`+остаток, `RectTable`), `shaders/voxel_cull.wgsl`, `voxel_draw.wgsl` (vertex pulling), `hiz_build.wgsl`, `voxel_mesh_stub.txt` (заглушка task/mesh). Тесты: `crates/render/tests/voxel_renderer.rs`.
- `crates/client/src/worldview.rs`: `RenderOptions` (флаг compute-рендера), ветвление в `update`/`render`, F8 — переключатель (main.rs).
- `crates/render/src/gui.rs` (+`shaders/gui.wgsl`): `GuiPainter` — спрайты/nine_slice/панорама. `crates/client/src/ui/{mod,lang,options,servers}.rs`: GUI главного меню (экраны, виджеты, options.txt, servers.dat); подключено в `main.rs` (`init_gui`, `menu_active`).
- `crates/client/src/netthread.rs` (сетевой поток, `NetEvt`, `SharedLevel`), `crates/client/src/worldview.rs` (`WorldView`: update/fly/render; связка net→mesh→arena→pass). Запуск: `cargo xtask server 1.21.11 25599` + `cargo run -p ezzmc-client --release -- --connect 127.0.0.1:25599`.
- `crates/client/src/particles.rs` (155): `ParticleEngine` (tick/extract/handle_packet/tick_weather/destroy_block), `SpriteSets`, `TerrainLook`; подключён в `worldview.rs` (`terrain_look`, `tick_particles`); `render/src/particledraw.rs` — отрисовка квадов.
- `crates/render/src/weatherdraw.rs` (151): `WeatherDraw::prepare/draw` (колонки дождя/снега, шейдер particle.wgsl); `client/src/weather.rs::collect` (высота MOTION_BLOCKING, осадки, сид колонок); `world/src/precipitation.rs` (Biome#getPrecipitationAt, шум температуры); `ClientChunk::height`.
- `crates/render/src/clouddraw.rs` (149): `CloudDraw::prepare/draw` (облака, ячейки 12×4×12); `sky::cloud_color`.
- `crates/render/src/fog.rs` (150): `setup_fog(FogInput, FogState)` → `FogUniform` (Atmospheric/Water/Lava/PowderSnow, дождь, void-darkness); клиент: `worldview.rs::compute_fog`/`fluid_in_camera`.
- `crates/render/src/sky.rs` (149, CPU): `sky_state(day_time, rain, thunder, base_sky)` → `SkyState`; `sample_track`/`CubicBezier`; `build_stars/build_sky_disc/build_sunrise_fan`. 
- `crates/client/src/hud.rs`: HUD-хотбар и иконки предметов (`draw_hotbar`).
- `crates/client/src/inventory.rs`: модель инвентаря и разбор пакетов контейнеров.
- `crates/render/src/text.rs` (+`shaders/text.wgsl`): `TextRenderer` — ванильный ascii.png, `begin/draw/flush`, `gui_scale`; HUD строится в `client/main.rs` (RedrawRequested).
- `crates/render/src/skydraw.rs` (149, GPU): `SkyDraw::new/prepare/draw` — диск неба, веер заката, солнце, луна (8 фаз), звёзды, тёмный диск; рисуется первым в проходе мира из `client/worldview.rs` (`load_celestials` берёт `textures/environment/celestial/*`).

- `crates/client/src/debugscreen.rs` — F3 (раскладка 26.3); `pick.rs` — луч взгляда/цель (`raycast_t` — с расстоянием; F5-камера в `worldview.rs::third_person_view`); `crates/render/src/lines.rs` + `shaders/lines.wgsl` — толстые линии (контур блока); `text.rs::fill` + `§`-коды.

- `crates/world/src/lightning.rs` — клиентские молнии (Bolts, тик); `Level.bolts/sky_flash_time`, разбор add_entity/remove_entities в `level.rs`. `crates/render/src/lightningdraw.rs` + `shaders/lightning.wgsl` — рендер молнии (`build_bolt`).

- `crates/client/src/physics.rs` — игрок: тик движения, коллизии (Entity#collide), прыжок/спринт/sneak/полёт, вода+лава (`update_fluid_state`, `travel_in_water/lava`, течение), лестницы (`on_climbable`); `Collider` trait (тесты без мира).
- `crates/render/src/entitymodel.rs`: база моделей сущностей (`Model::add/reset_pose/part_matrices/bake`, `BoxDef`, `Pose`) — для всех будущих мобов; `keyframe.rs`: AnimationDefinition/Channel/Keyframe (apply, apply_walk, catmullrom); `humanoid.rs`: `player_mesh`/`humanoid_mesh`/`setup_anim`/`HumanoidState`. `particledraw.rs::prepare_raw` — сырые квады моделей.
- `crates/world/src/entities.rs`: хранилище сущностей `Level.entities` (add/move/teleport/sync/rotate_head, лерп); `Entity.flags` (метаданные #0, `set_entity_data` в `level.rs`: присед/плавание); рисуются в `worldview.rs::bake_remote_players`.
- `crates/client/src/avatar.rs`: `AvatarAnim` (тело/голова/ходьба/взмах), в `physics::Player.anim`; рендер — `worldview.rs::prepare_player_model`, `entity_draw`. Плавание: `swim_amount`; рука 1-го лица — `worldview.rs::prepare_hand` (`hand_draw`, проход «hand»); вода — `physics.rs::update_fluid_state`/`travel_in_water`.
- `crates/client/src/dig.rs` — `Digger`/`destroy_progress` (185, подключён: `WorldView::tick_dig`); `pick.rs` — `Target{face,hit,inside}`, `BLOCK_REACH`; `protocol/item.rs` — ItemStack/компоненты (230a); `client/inventory.rs` — Inventory+пакеты (230b, не подключён); `world/level.rs` — предсказание блоков (187); `world/entity_size.rs` — хитбоксы типов; `pick::raycast_entity`; `netthread.rs` — `NetCmd` (main→net), `NetEvt::{GameMode,Abilities,Motion}`, `FLAG_*` в SharedPos.
