# Сверка ezzmc с ванилью 26.3 (стенд бок о бок) — 2026-11-08

Слева на скриншотах владельца — ezzmc, справа — чистая ваниль 26.3.

## Стенд (как повторить)
- Сервер 26.3 уже запущен (`reference/_server/26.3`, `online-mode=false`), к нему подключён `ezzmc-client --connect 127.0.0.1:25611` под ником `EzzBot`.
- Ваниль запускается скриптом `/tmp/claude-1000/-home-bozhk-PETS-ezzmc/3eab4a0f-0527-48fe-bb97-28efe1478cf6/scratchpad/vlaunch.sh`. Скрипт собирает classpath из `~/.minecraft/versions/26.3` и берёт Java 25 из TLauncher (`java-runtime-epsilon`). Ник `Vanilla`, `--quickPlayMultiplayer 127.0.0.1:25611`, gameDir `.../scratchpad/vgame`. Если /tmp очищен, скрипт придётся пересоздать. Ключевые пункты: `-Djava.library.path` = `~/.minecraft/versions/26.3/natives/lwjgl/3.4.3+4/x64`, `--assetIndex 34`.
- Консоль сервера принимает команды через fifo `/tmp/claude-1000/-home-bozhk-PETS-ezzmc/2acd9955-7815-48cc-81ff-37560bb2859e/scratchpad/in` (`echo "cmd" > fifo`). Он привязан к старому процессу сервера.
- Настройки стенда: `time set 12700`, `gamerule advance_time false`, `weather clear`. Команда `nocol` с `collisionRule never`, оба игрока невидимы (`invisibility`). `Vanilla` в режиме наблюдателя, `spectate EzzBot Vanilla` после каждого переподключения `EzzBot`. Камера ванилы идёт от глаз `EzzBot`.
- ezzmc теперь не рисует невидимых игроков: `Entity::invisible()` (флаг 32) в `world/entities.rs` и `continue` в `bake_remote_players` (`worldview.rs`).
- Важно: ванила смотрит из режима наблюдателя, поэтому `camera.entity()` не `LocalPlayer`. Из-за этого `brightenFactor`/`getWaterVision` для воды у неё равен 1.0. ezzmc ведёт себя как `LocalPlayer`, и там вода ярчеет постепенно (до 600 тиков). Для сравнения яркости воды подождите 30 секунд под водой или сравнивайте при `water_vision = 1`.
- Остановить: `pkill -f "[v]sync.sh"` (скрипт `vsync.sh` больше не нужен). Не использовать `pkill -f vsync.sh`: он убивает и шелл.

## Выводы по замечаниям владельца

| # | Замечание | Статус |
|---|---|---|
| 0 | Силуэты чанков над водой на закате | **Ваниль.** Правок не нужно (туман, веер заката и порядок слоёв совпадают с 26.3). |
| 1 | Освещение/яркость цветов и листвы (ночью ezzmc темнее) | **Исправлено.** `lightmap.rs::texel` был по формуле 1.21.11. Теперь порт `core/lightmap.fsh` 26.3: `AmbientColor` (#0a0a0a) и `NightVisionColor`, `BlockLightTint` (#FFD88C) с `parabolicMixFactor`, `BossOverlayWorldDarkening`, без финального `mix(0.75, 0.04)`. `block_factor` = flicker + 1.4 (было 1.5). Тесты в `lightmap.rs` обновлены и проходят. **Визуально не проверено.** |
| 2 | Вода: цвет тумана под водой (ezzmc насыщенно-синий и тёмный, ваниль светло-голубой) | **Исправлено, не проверено.** (a) Дефолт `water_fog_color` был 0x3F76E4 (это цвет воды), в 26.3 дефолт 0x050533 (`EnvironmentAttributes.WATER_FOG_COLOR`); (b) добавлено `FogRenderer#computeFogColor`: яркость воды растёт с `getWaterVision`, цвет нормируется по max-компоненте; (c) `WaterFogEnvironment`: конец тумана × `max(0.25, getWaterVision)`; (d) `FogState::{tick_water_vision, water_vision}` (рост +1 тик, спад −10, 0..600), `FogInput.water_vision`. Тесты fog зелёные. |
| 3 | Вид под водой №2: у ваниль видна поверхность воды снизу (полосы), у ezzmc вместо неё небо/силуэт берега и красный веер заката | **НЕ сделано.** Причина не найдена. Заднюю грань поверхности (`backward_up` в `mesh/fluid.rs`) порт делает. Подозрения: дальняя поверхность воды не рисуется (проверить `cull_mode: Back` в `render/terrain.rs:723` для translucent и логику `backward_up`); в ванили небо рисуется под водой (кроме лавы/снега/эффектов, `LevelRenderer#addSkyPass`), но веер закрыт поверхностью воды. Проверить после правок п.2: часть эффекта могла быть из-за неверного тумана. |
| 4 | Облака: дальность | **НЕ сделано.** В ванили `cloudRange` по пресету графики: fast=32, fancy=**64**, fabulous=128 (`GraphicsPreset.java:52/73/94`). Дефолт `Options.cloudRange` = 128, но пресет «fancy» ставит 64 (в `options.txt` ванили `cloudRange:64`). У нас в `worldview.rs:~1301` жёстко 128 — привести к 64. |
| 5 | Свечение солнца накрывает облака в ванили, у нас нет | **НЕ сделано.** `clouds.wgsl` фейдит только альфу (`linear_fog_value(dist, 0, clouds_end)`), цвет не подмешивается. Проверить в `reference/26.3` (`CloudRenderer`, шейдеры `clouds.fsh`/`cloud*.glsl`), как туман и закат влияют на цвет облаков (скорее всего: цвет тумана с подмешиванием заката у горизонта красит облака, либо веер рисуется после облаков). Эталон: `reference/26.3/client/net/minecraft/client/renderer/CloudRenderer.java`, `SkyRenderer.java`, `LevelRenderer.java`. |
| 6 | Виньетка | **НЕ разобрано.** В коде ezzmc виньетки нет (`grep vig` пусто). У слева на скриншоте тёмный градиент по краям окна: возможно, это тень окна компоситора, а не рендер. В ванили `Gui` рисует виньетку только у границы мира (и опция `vignette:true`). Проверить визуально на полноэкранном ezzmc; если градиент реальный — найти источник в `crates/client/src/ui/`. |
| 7 | Листва: «яркость» и вид | **Частично** (яркость должна вырасти после п.1). Вид листвы (ванильные настройки `cutoutLeaves:true` при пресете fancy, дырявая против сплошной) не сверен. Компьют-рендер заморожен (CLAUDE.md), не трогать. |

## Что ещё не сделано / не проверено
- Всё в этом файле **не проверено визуально**: после правок нужен новый скриншот-стенд (перезапустить `ezzmc-client`, затем `spectate EzzBot Vanilla`).
- `crates/render/src/shaders/lightmap.wgsl` (GPU-вариант) остался по формуле 1.21.11 и не используется (карта освещения считается на CPU в `lightmap.rs`). При желании привести в соответствие или удалить.
- Туман не учитывает вспышку молнии (VANILLA-UNSURE, старое).
- Записи в `STATUS.md`/`CODEMAP.md`/`TASKS.md` по этим правкам **не внесены**.
- `cargo fmt`/`clippy` на изменения не гонялись, из тестов запущены только `-p ezzmc-render lightmap` и `fog`.

## Изменённые файлы
- `crates/render/src/lightmap.rs` — формула 26.3, дефолты, тесты.
- `crates/render/src/fog.rs` — вода (цвет, дальность, яркость), `water_vision`, тесты.
- `crates/client/src/worldview.rs` — вызов `tick_water_vision`/`water_vision` в `compute_fog`; пропуск невидимых игроков.
- `crates/world/src/entities.rs` — `Entity::invisible()`.

---

## ПРОМПТ ДЛЯ СЛЕДУЮЩЕГО ИИ

Контекст: проект ezzmc (`/home/bozhk/PETS/ezzmc`), читай `CLAUDE.md`, затем `docs/ref/vanilla-compare-2026-11-08.md` (этот файл). Версия-цель 26.3, эталон в `reference/26.3/`. Береги токены владельца: смотри в ванили только нужное, не запускай субагентов.

Владелец сравнивает ezzmc (слева) с чистой ванилью 26.3 (справа) на скриншотах и просит исправить различия. Стенд описан выше. Уже сделаны освещение (формула 26.3) и туман воды (п.1, п.2), но визуально не проверены. Сделай по порядку:
1. Облака: `cloudRange` 64 вместо 128 (fancy-пресет), см. `worldview.rs` около строки 1301.
2. Подмешивание свечения солнца в облака: найди в `reference/26.3` (`CloudRenderer`, шейдеры облаков, `LevelRenderer`), как закат/цвет тумана влияет на облака, и перенеси (`crates/render/src/clouddraw.rs`, `shaders/clouds.wgsl`).
3. Вид под водой №2: почему не видна поверхность воды снизу (см. п.3 таблицы). Проверь `cull_mode`, `backward_up`, порядок слоёв и небо под водой.
4. Листва: сверь вид и яркость после п.1 (cutout/опция `cutoutLeaves`), компьют-рендер не трогать.
5. Виньетка: убедись, что в ezzmc её нет, и не добавляй; если на полном экране градиент реальный, найди источник.
6. Запусти `cargo fmt`, `clippy`, тесты `ezzmc-render`; обнови `STATUS.md`, `CODEMAP.md`, `TASKS.md` по правилам `CLAUDE.md`. Явно пиши в журнале, что проверено только тестами, а не на живом сервере.
Не выдумывай поведение: при сомнениях сверяйся с `reference/26.3`, а при расхождении с ванилью проси новый скриншот у владельца.

---

## ДОПОЛНЕНИЕ (вторая половина сессии) — состояние на момент остановки

### Сделано после первой части
- `cloudRange` 64 в `worldview.rs` (~стр. 1301) + `FogInput.cloud_range`; `cloud_end = min(cloudRange*16, cloud_fog_end)` (`render/fog.rs`).
- **Виньетка портирована** (владелец был прав: она есть в 26.3, `Hud#extractVignette`): новый `crates/render/src/vignette.rs` (`VignetteDraw`, `sky_darken`, `level_brightness`, `tick_brightness`, 2 теста зелёные). В `worldview.rs`: поля `vignette`, `vignette_b`, `vignette_acc`; яркость обновляется раз в тик в `compute_fog` по `max(block_light, sky_light − sky_darken)`; рисуется в конце основного прохода (не-compute). Не сделано: красный сигнал у границы мира, учёт дождя в `sky_darken`, скрытие по F1 (в ezzmc нет HUD и F1). Визуально не проверено.
- Релизная сборка `ezzmc-client` собиралась успешно после этих правок (до начала работы над меню).
- В части 1 этого файла строка про виньетку («только у границы мира») — **неверна**, см. выше.

### Важная находка по листве (не исправлена!)
`worldview.rs:278`: `BlockModels::build(rm, &atlas, false)` — ezzmc строит модели с `cutout_leaves = false` (быстрая листва: слой Solid, грани между листьями скрыты), а в ванили `cutoutLeaves:true` (fancy). Вероятно, это и есть расхождение листвы на скриншотах. Исправление: передавать `true` (или читать ключ `cutoutLeaves` из `options.txt`; значение запекается в модели при загрузке мира, смена на лету требует пересборки — применять при следующем подключении).

### Что я начал и НЕ закончил (кода по меню пока НЕ написано)
Владелец не может открыть настройки: в ezzmc нет меню паузы (Esc только отпускает курсор). Решение владельца: «проще сделать меню паузы и менять в нём». План (ничего из этого ещё не в коде):
1. `ui/mod.rs`: `Screen::Pause`, `Gui::open_pause()`, `Action::Resume`, параметр `ingame: bool` в `Gui::render` (без панорамы и без blit «overlay», `p.flush(.., None)`, затемнение `0x70000000` оставить); Esc на корне стека Pause → `Action::Resume`. Кнопки как в ваниль: `menu.returnToGame`, `menu.options`, `menu.disconnect` (пока = выход), заголовок `menu.game`.
2. `ui/options.rs` (список `VIDEO`): добавить `cutoutLeaves` (Toggle, "true") и `vignette` (Toggle, "true") — ключи ванили.
3. `main.rs`: поле `paused`; Esc в игре (нажатие, не повтор) → `paused = true`, `gui.open_pause()`, release мыши, `input.release_all()`/`keys.release_all()`; ветки `KeyboardInput`, `MouseInput`, `MouseWheel` с условием `menu_active || paused`; в замыкании `draw_and_present` после `v.render(..)` при `paused` вызвать `gui.render(.., ingame=true)`; `Action::Resume` → `paused=false` + grab мыши.
4. `worldview.rs`: читать `cutoutLeaves`/`vignette` из `options.txt` при создании мира (`ui::options::Options::load("options.txt")`).
Настройки сохраняются в `./options.txt` (`Options::save` вызывается в `Gui::pop`).

### Нерешённое (без изменений)
Подводный вид №2 (веер заката/поверхность воды снизу), свечение солнца над облаками, `lightmap.wgsl` устарел. Код неба/облаков/веера сверен с 26.3 — расхождений не найдено; нужен новый скриншот после перезапуска.
Стенд ванили: `improvedTransparency:false`, `cutoutLeaves:true`, `cloudRange:64`, `vignette:true` в `scratchpad/vgame/options.txt`.
STATUS.md/CODEMAP.md/TASKS.md: в STATUS добавлена одна строка журнала (часть 1); про виньетку, `cloud_range` и находку по листве записи нет. CODEMAP/TASKS не обновлялись.
