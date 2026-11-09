# TASKS.md — полный список задач ezzmc (от окна до убийства дракона)

> Это **бэклог проекта**, разбитый на главы. Журнал, баги, решения — в `STATUS.md`; цели и правила — `GOAL.md`.
> **Правила работы с задачами (GOAL.md §7.10):**
> - Берёшь задачу **сверху вниз** из самой ранней главы, где есть незакрытые P0/P1. Прыгать по главам нельзя без записи `DEC`.
> - Взял → пометь `[~]` + `(чат/агент, дата)`. Закончил → `[x]` + `(дата, коммит)`. Не смог → вернуть `[ ]` и записать причину в `STATUS.md → Блокеры`.
> - Каждая задача портируется с **эталона** (GOAL.md §2А), поведение **ванильное** (§0). Для игровой логики в задаче подразумевается `// VANILLA:` и golden-тест, где применимо.
> - Новая задача, найденная по ходу → добавь в **конец подходящей главы** с новым сквозным номером (следующий свободный TODO-NNN).
> - **Следующий свободный номер: TODO-498.** Обновляй эту строку при добавлении задач.
> - Приоритет: **P0** блокирует всё · **P1** нужна для играбельности · **P2** нужна для полной ваниль-паритета · **P3** полировка.
> - Размер задачи S/M/L (GOAL.md §7А.1) указывать не обязательно, но L-задачи надо **дробить**, если не влезают в потолок.
> - **Режим проекта (GOAL.md §0Б): приватный, оффлайн, без аккаунтов.** Отменённые задачи помечены `[-]` и не берутся.
> - Главы 0–21 идут по порядку зависимостей. Версия-цель в главах 1–17: **последняя (каноническая)**. Старые версии — глава 18.

Веха «**Убить дракона**» = все P0/P1 глав 0–17 + приёмочный сценарий главы 21.

---

> ⚠ Решение владельца 2026-11-07: графику рисуем **только на ванильном пути (Packed/TerrainArena)**. Compute-рендер (MDI / Hi-Z, `--compute-renderer`, F8) заморожен: не развивать и не чинить, пока не сделано всё остальное (возможно — в самом конце). Ярлык запускает без флага. Задачи гл. 19А про compute/Hi-Z/mesh shaders (TODO-413, 491, 492, 493) — отложены в самый конец, не брать.

## ⭐ ВЕХА M1 — «Видимый ванильный мир» (ПРИОРИТЕТ НАД ВСЕМ ОСТАЛЬНЫМ)

Решение владельца: **сначала — подключиться к локальному offline-серверу и увидеть мир, выглядящий как в ванили (рендер, свет, небо, туман, биомные цвета), потом всё остальное.** Пока M1 не закрыта, задачи вне списка ниже **не берутся** (исключение — блокирующие баги). Идём **строго по порядку шагов**, внутри шага — по номерам.
Версия-цель: **26.3** (DEC-014; данные 1.21.11 — запасные). Без физики и взаимодействия: камера летает свободно.

| Шаг | Что даёт | Задачи (по порядку) |
|---|---|---|
| **M1.1 Соединение** | Клиент логинится и проходит Configuration | 047, 048, 049, 050, 052, 053, 055, 056, 057, 064, 070, 080, 081, 082, 083, 084, 059, 062, 457 |
| **M1.2 Данные мира** | Чанки приходят и хранятся | 112, 113, 114, 115, 116, 117, 118, 120, 121, 119, 129, 130 |
| **M1.3 Ресурсы** | Текстуры/модели/блокстейты из установленной игры | 089, 090, 091, 092, 093, 094, 095, 096, 098, 101, 103 |
| **M1.4 Меш и рендер** | Земля видна, ванильное освещение | 133, 134, 135, 136, 137, 143, 144, 139, 138, 140, 141, 142, 145, 146 |
| **M1.5 Окружение** | Небо, туман, облака, погода, дальность | 149, 150, 151, 154, 458 |
| **M1.6 Свободная камера** | Можно летать и смотреть мир | 455 (free-fly), 162, затем ходьба: 163–171 |
| **M1.7 Приёмка M1** | Сверка с ванилью по скриншотам | 456 |

Отложено до закрытия M1: 026–030, 044–046, 060, 063, 065–069, 074–078, 085–088, 100, 102, 104–111, главы 8 (кроме 163–171), 9–17, 19–22 (кроме сквозной оптимизации, не ломающей ваниль). Глава 18 (мульти-версии) — **сразу после M1** (это Столп №2), до остальных глав.

---

## Глава 0. Фундамент проекта
- [x] TODO-009 [P0] Cargo workspace и структура крейтов (protocol, net, world, mesh, render, assets, client, xtask)
- [x] TODO-010 [P0] `rust-toolchain.toml`, edition, зафиксировать MSRV
- [x] TODO-011 [P0] Профили сборки: release (lto fat, codegen-units=1, panic=abort), dev с opt-level для горячих крейтов, profile `bench`, profile `profiling` (debuginfo)
- [x] TODO-012 [P0] `data/versions.toml`: все версии 1.5.2 → latest (имя, protocol, data version, дата, эра, config-state, формат чанков)
- [x] TODO-013 [P0] CI: fmt + clippy -D warnings + test + сборка под Linux/Windows/macOS
- [x] TODO-014 [P0] Каркас бенчмарков (criterion/divan) и `xtask bench`
- [x] TODO-015 [P0] `xtask fetch-reference <ver>`: manifest → jar → маппинги → декомпиляция в `reference/` (gitignored)
- [x] TODO-016 [P0] `xtask fetch-reference` для версий до 1.14 (MCP/сторонние маппинги)
- [x] TODO-017 [P0] `xtask gen-reports <ver>`: запуск data generator (--reports/--server) → `reference/<ver>/reports`
- [x] TODO-018 [P0] `xtask gen-protocol`: reports → Rust-таблицы пакетов (id, направление, состояние)
- [x] TODO-019 [P0] `xtask gen-registries`: блоки/предметы/сущности/биомы/звуки/эффекты/энчанты → Rust
- [x] TODO-020 [P0] `xtask gen-blockstates`: все состояния блоков, id ↔ свойства (из reports/blocks.json)
- [x] TODO-020b [P0] Свойства блоков вне отчёта: shape/collision, hardness, resistance, tool-tag, light, опасность — из исходника Mojang/дампа (в `blocks.json` их нет)
- [x] TODO-020c [P1] Теги инструментов/материалов блоков (mineable/*, needs_*_tool) из датапака (`--server` дата-генератор) → таблица; сейчас в `StateProps` только `requires_correct_tool`
- [x] TODO-021 [P0] Каркас golden-тестов (`tests-integration/golden/`), формат данных, как снимать с ванили
- [x] TODO-022 [P0] Стенд: скрипт запуска ванильного сервера нужной версии (offline, отладочный) для тестов
- [x] TODO-023 [P0] Запись/воспроизведение трафика (`replay`): формат, рекордер-прокси, плеер для тестов
- [x] TODO-024 [P1] Логирование (tracing), уровни, лог в файл, лог пакетов (по флагу)
- [x] TODO-025 [P1] Единая система ошибок (thiserror), политика «паник от сети нет»
- [~] TODO-026 [P1] Конфиг клиента (options.txt-совместимый формат), каталог данных (`.minecraft`-совместимые пути)
- [~] TODO-027 [P1] Фича-флаги Cargo: `profiling`, `tracy`, `dev-tools`
- [~] TODO-028 [P1] Pre-commit проверки, шаблон коммита, проверка актуальности CODEMAP/STATUS в CI (скрипт-линтер)
- [~] TODO-029 [P2] Fuzz-каркас (cargo-fuzz) для парсеров пакетов/NBT/чанков
- [~] TODO-452 [P1] Проверить protocol для 1.7.3 (нет в minecraft-data) и добавить в `data/versions.toml`
- [~] TODO-030 [P2] Miri-прогон для unsafe-кода в CI

## Глава 1. Окно, главный цикл, ввод
- [x] TODO-031 [P0] Окно (winit): создание, размер, DPI, fullscreen/borderless, иконка, заголовок
- [x] TODO-032 [P0] wgpu: инициализация instance/adapter/device, выбор бэкенда (Vulkan/DX12/Metal), fallback
- [x] TODO-033 [P0] Swapchain, resize, present mode (vsync/mailbox/immediate), потеря устройства
- [x] TODO-034 [P0] Главный цикл: фиксированный тик 20 TPS + интерполяция рендера (partialTick как в ваниле)
- [x] TODO-035 [P0] Разделение потоков: main/render, net, mesh-workers, io; каналы между ними
- [x] TODO-036 [P0] Ввод клавиатуры/мыши: сырой ввод, захват курсора, чувствительность (ваниль-кривая), keymap
- [x] TODO-037 [P0] Биндинги клавиш по умолчанию (как в ванили) и переназначение, сохранение в options
- [x] TODO-038 [P1] Геймпад не нужен (нет в Java Edition) — зафиксировать как не-цель
- [x] TODO-039 [P1] Фокус окна: пауза при потере фокуса (как в ванили), курсор освобождается
- [x] TODO-040 [P1] Ограничение FPS, меню «макс. FPS», вертикальная синхронизация, GUI-масштаб
- [x] TODO-041 [P1] Скриншоты (F2), сохранение PNG в `screenshots/` с ванильным именем
- [x] TODO-042 [P1] Полноэкранный режим F11, переключение, запоминание размера окна
- [x] TODO-043 [P1] Системный буфер обмена (чат, поля ввода)
- [~] TODO-044 [P2] Отладочный оверлей по F3 (каркас, наполнение — глава 12)
- [~] TODO-045 [P2] Профайлер кадра (F3+L как в ваниле) + внутренний трейс (tracy)
- [~] TODO-046 [P3] HiDPI/Wayland/X11 специфика, XWayland-проблемы, Windows IME ввод

- [~] [P1] TODO-453 Выбор оконного бэкенда Linux: флаг `--backend=wayland|x11|auto` + env `WINIT_UNIX_BACKEND`, без пересборки; в логе/F3 показывать активный бэкенд — (после TODO-031/032)
- [~] [P1] TODO-454 Замер Wayland vs X11 (XWayland): frame time (avg/1%low/max), present mode (Fifo/Mailbox/Immediate), задержка ввода, захват курсора; результат в `STATUS.md → Бенчмарки`, выбрать default по факту и записать DEC — (после TODO-453)

- [x] [P0] TODO-455 Свободная камера (debug fly): WASD/пробел/Shift, без физики и коллизий, скорость по колесу, для просмотра мира (M1.6)
- [~] [P0] TODO-456 Приёмка M1: тот же seed/позиция в ванили и у нас, скриншот-сравнение (свет, AO, небо, туман, цвет трав/воды/листвы), список расхождений в `[vanilla-diff]`, FPS по бюджету
- [x] [P0] TODO-457 Play-вход: Login (join game), Respawn, Game Event (start waiting for level chunks), Set Default Spawn, Synchronize Player Position + Teleport Confirm, Player Loaded — минимум, чтобы сервер начал слать чанки
- [x] [P1] TODO-458 Время суток/освещение неба: Update Time → цвет неба, яркость, солнце/луна, звёзды (связка с 129 и 149)

## Глава 2. Сетевое ядро (последняя версия)
- [x] TODO-047 [P0] TCP-клиент, async-рантайм (tokio или свой mio-поток), таймауты, DNS, SRV `_minecraft._tcp`
- [x] TODO-048 [P0] VarInt/VarLong, строки, UUID, позиции (packed long), углы, bitset, optional/enum-кодирование
- [x] TODO-049 [P0] Фрейминг пакетов (length-prefixed), буферы без аллокаций, потоковый парсер
- [x] TODO-050 [P0] Компрессия (threshold, zlib через libdeflate/zlib-ng), лимиты размеров
- [~] TODO-051 [P3] Шифрование AES/CFB8 (нужно только для online-mode серверов — вне цели по §0Б, опционально)
- [x] TODO-052 [P0] Состояния: Handshaking, Status, Login, Configuration, Play — конечный автомат
- [x] TODO-053 [P0] Handshake пакет, Intent (status/login/transfer), protocol number, адрес/порт
- [~] TODO-054 [P0] Server List Ping: status request/response, ping/pong, парсинг JSON (версия, игроки, MOTD, favicon, enforcesSecureChat)
- [x] TODO-055 [P0] NBT: бинарный формат, named/nameless root (сетевой), SNBT, сериализация/десериализация
- [x] TODO-056 [P0] Text Component: JSON/NBT-форма, стили, translate/with, selector/score/nbt/keybind, клик/hover события
- [~] TODO-057 [P0] Структуры данных протокола: Slot/ItemStack (data components), BlockPos, ChunkPos, Vec3, GameProfile — 057a ✅ (BlockPos/ChunkPos/Vec3/GameProfile); 057b Slot/ItemStack + data components (кодеки компонентов 1.21.11) — не сделано
- [~] TODO-058 [P0] Data Components: все типы компонентов предмета (сериализация сети), patched components
- [x] TODO-059 [P0] Реестры в сети: Registry Data, tags, known packs (Select Known Packs), синхронизация
- [~] TODO-060 [P1] Brigadier command tree: парсер пакета Commands, все типы аргументов + suggestions
- [~] TODO-061 [P1] Plugin channels / custom payload: brand, register, discovery (минимум brand=vanilla)
- [~] TODO-062 [P1] Keep-alive, ping, pong, disconnect (причина как Component), reconnect — Configuration: keep-alive/ping/disconnect/transfer ✅; Play-часть и reconnect-UI — позже
- [~] TODO-063 [P1] Transfer-пакет (перенос между серверами), cookies (store/request)
- [x] TODO-064 [P1] Лимиты и защита: макс. размер пакета, глубина NBT, длина строк, количество элементов — никаких паник
- [~] TODO-065 [P1] Очередь исходящих пакетов, флаш, Nagle off, приоритеты (позиция/ввод)
- [~] TODO-066 [P1] Метрики сети (байт/с, пакетов/с, лаг) для F3
- [~] TODO-067 [P2] Ping-fuzz и replay-прогон всех пакетов состояния Play
- [x] TODO-068 [P2] Серверный `chunk batch`: обратная связь скорости (chunk_batch_received)
- [~] TODO-069 [P3] IPv6, прокси (SOCKS) — опционально, не ванильно, не делаем без DEC

## Глава 3. Вход (только offline, без аккаунтов — GOAL.md §0Б)
- [x] TODO-070 [P0] Offline-режим логина: Login Start (name, uuid), Login Success, Login Acknowledged
- [-] TODO-071 ОТМЕНЕНО (§0Б): online-режим/sessionserver не нужен
- [-] TODO-072 ОТМЕНЕНО (§0Б): Microsoft/Xbox/Minecraft авторизации нет
- [-] TODO-073 ОТМЕНЕНО (§0Б): токенов нет
- [~] TODO-074 [P1] Локальные профили: ник + offline-UUID (`OfflinePlayer:<name>`), несколько профилей в конфиге, выбор активного
- [~] TODO-075 [P1] Скины: локальный PNG (classic/slim) или дефолтный Steve/Alex по UUID; без сетевой загрузки
- [~] TODO-076 [P1] Login plugin request/response (custom query), compression set, disconnect на логине
- [-] TODO-077 ОТМЕНЕНО (§0Б): ключей чата/сертификатов нет, чат unsigned
- [~] TODO-078 [P2] Quick Play, ввод адреса из командной строки (`--server`, `--username`, `--uuid` как у лаунчера)
- [-] TODO-079 ОТМЕНЕНО (§0Б): аккаунтных ошибок нет

## Глава 4. Конфигурация и регистры
- [x] TODO-080 [P0] Состояние Configuration: Client Information, Plugin Message, Finish Configuration/Acknowledge
- [x] TODO-081 [P0] Registry Data: dimension_type, biome, damage_type, painting_variant, banner_pattern, wolf_variant, trim, jukebox_song, enchantment, chat_type и др.
- [x] TODO-082 [P0] Известные паки (Known Packs) и «core» пак: подмена данных регистров по версии
- [x] TODO-083 [P0] Теги: Update Tags, структура, запросы `is_in_tag`
- [x] TODO-084 [P0] Feature Flags (Update Enabled Features): vanilla/trade_rebalance/redstone_experiments и пр.
- [~] TODO-085 [P1] Server resource pack: Add/Remove, загрузка, SHA1, prompt, принудительный, применение
- [~] TODO-086 [P1] Reset Chat, Server Links, Custom Report Details, Clear Dialog/Show Dialog (если есть в версии)
- [x] TODO-087 [P1] Client Information: язык, view distance, chat mode, цвета, skin parts, main hand, text filtering, listing
- [~] TODO-088 [P2] Reconfiguration (возврат Play → Config), смена мира/ресурспака на лету

## Глава 5. Ресурсы (assets)
- [x] TODO-089 [P0] Поиск и чтение установленной Minecraft: `.minecraft/versions/<v>/<v>.jar`, `assets/indexes`, `assets/objects`
- [x] TODO-090 [P0] Чтение jar (zip), index json, объекты по hash, ленивая загрузка, mmap
- [x] TODO-091 [P0] Ресурспак-система: слои паков, `pack.mcmeta`, pack_format, overlays, порядок и перекрытие, фильтры
- [x] TODO-092 [P0] Загрузка текстур: PNG → атлас блоков/предметов (stitching как в ванили, mipmaps)
- [x] TODO-093 [P0] Атлас: `atlases/*.json` (blocks, items, particles, paintings, mob_effects, gui, banner_patterns, shield_patterns, signs, shulker_boxes, chests, decorated_pot…)
- [x] TODO-094 [P0] Анимированные текстуры `.mcmeta` (frametime, frames, interpolate), tick-обновление
- [x] TODO-095 [P0] Модели блоков/предметов JSON: parent, elements, faces, rotation, display, textures, ambientocclusion, gui_light
- [x] TODO-096 [P0] Blockstates: variants, multipart, weights/random rotation, uvlock
- [~] TODO-097 [P0] Item model definitions (`items/*.json`, новая система item models: condition/select/range_dispatch/composite…)
- [x] TODO-098 [P0] Компиляция моделей → baked quads (позиция, UV, tint, shade, cull-face)
- [~] TODO-099 [P1] Языковые файлы: `lang/*.json`, fallback en_us, форматирование `%s/%1$s`, plurals не нужны
- [~] TODO-100 [P1] Шрифты: `font/*.json` providers (bitmap, ttf, space, unihex, legacy_unicode, reference)
- [x] TODO-101 [P1] Цвета биомов: colormap `grass.png/foliage.png/dry_foliage.png`, water color, fog/sky цвета из биомов
- [~] TODO-102 [P1] Звуки: `sounds.json`, события, веса, pitch/volume, stream, `.ogg`
- [x] TODO-103 [P1] Шейдеры ванили → собственные WGSL-эквиваленты (core: position_tex_color, terrain, entity, sky, stars, clouds, particles, gui…)
- [~] TODO-104 [P1] Пост-эффекты: `post_effect`/`shaders/post` (прозрачность, outline — spectator/glow)
- [~] TODO-105 [P1] Текстуры сущностей, equipment (armor/trim/layers), элитры, плащи, скины игрока
- [~] TODO-106 [P1] Splash-тексты, панорама title screen, GUI sprites (`gui/sprites`, 9-slice метаданные)
- [x] TODO-107 [P2] Ресурспак-перезагрузка на лету: блоки, модели, GUI (шрифт, спрайты, панорама), colormap — проверено вживую; не обновляются иконки предметов, рука, частицы, небо/облака, сущности, звуки, lang (нужен рестарт; F3+T и экран загрузки — нет)
- [~] TODO-108 [P2] Датапаки клиентской части: нет (только серверные регистры по сети)
- [~] TODO-109 [P2] Кэш предобработанных ресурсов (бинарный кэш атласов и моделей) для быстрого старта
- [~] TODO-110 [P2] Поддержка `optifine`-стиля CTM/CIT — **не делаем** (не ванильно)
- [~] TODO-111 [P3] Загрузка ресурсов с сервера по URL с проверкой хэша и кэшем

## Глава 6. Данные мира
- [x] TODO-112 [P0] Block state: id → (block, properties), `BlockState` компактный (u16/u32), таблицы из генератора
- [x] TODO-113 [P0] Реестры в рантайме: Block, Item, EntityType, BlockEntityType, Fluid, Biome — id ↔ name
- [x] TODO-114 [P0] Секция чанка 16×16×16: palette (single/indirect/direct), bit-packed хранение, как в протоколе
- [x] TODO-115 [P0] Чанк: колонка секций по высоте измерения (min_y/height из dimension_type), BlockPos↔секция
- [x] TODO-116 [P0] Биомы 4×4×4 (paletted container), доступ с шумом/blend для цветов
- [x] TODO-117 [P0] Пакет Chunk Data and Update Light: heightmaps (NBT), данные секций, block entities, light data
- [x] TODO-118 [P0] Unload Chunk, Chunk Batch Start/Finished, Set Center Chunk, View Distance/Simulation Distance
- [x] TODO-119 [P0] Block Update, Multi-Block Change (Section Update), Block Entity Data, Block Event, Block Destruction
- [x] TODO-120 [P0] Хранилище чанков клиента: HashMap/плоская сетка вокруг центра, соседи, удаление вне радиуса
- [x] TODO-121 [P0] Свет: небесный/блочный, пакет light data, обновления, битовые маски, нибблы
- [~] TODO-122 [P1] Клиентское распространение света (как в ванили) при Block Update до прихода света с сервера (если ваниль делает)
- [~] TODO-123 [P1] Heightmaps (MOTION_BLOCKING, WORLD_SURFACE и др.) для дождя/снега/частиц
- [~] TODO-124 [P1] Форма столкновений и выбора блока: VoxelShape (collision/outline/interaction/occlusion) из генератора
- [~] TODO-125 [P1] Свойства блока: hardness, blast resistance, friction, speed/jump factor, light emission, opacity, occlusion
- [~] TODO-126 [P1] Жидкости: Fluid state (height, flow direction), вода/лава, источник/поток, falling
- [~] TODO-127 [P1] Block entities данные: sign, chest, banner, skull, bed, beacon, spawner, lectern, jukebox, brewing stand, decorated pot, trial spawner, vault, crafter, sculk-sensors…
- [~] TODO-128 [P2] Тики блоков на клиенте: random display tick (частицы факелов, дождя, лавы), анимации
- [x] TODO-129 [P2] Ванильный `Level` на клиенте: время суток (day time/game time/rain/thunder), difficulty, world border, spawn
- [~] TODO-129b [P2] Level: difficulty (change_difficulty), world border (initialize_border/set_border_*), default spawn — остаток 129 (интерполяция rain/thunder: в ванили на клиенте вырождена, сделано thunder×rain)
- [x] TODO-130 [P2] Измерения: dimension_type свойства (ultrawarm, natural, has_skylight, ceiling, coordinate_scale, ambient_light, effects)
- [~] TODO-131 [P2] Random: Java LCG (`RandomSource`, Xoroshiro128++ новые, `LegacyRandomSource`), позиционные seeds для вариаций блоков/частиц
- [~] TODO-132 [P2] Позиционный seed ванили для случайного поворота моделей (`Mth.getSeed`) — бит-в-бит

## Глава 7. Меш и рендер мира
- [x] TODO-133 [P0] Секционный меш: разбиение чанка на 16³, очереди пересборки, приоритет по расстоянию/взгляду
- [x] TODO-134 [P0] Многопоточный мешер (rayon/свой пул), без блокировки render-потока
- [x] TODO-135 [P0] Упакованные вершины (u32/u64): позиция, UV, свет, цвет, нормаль, AO
- [x] TODO-136 [P0] Face culling: сосед непрозрачен → грань не рисуется (как `Block.shouldRenderFace`)
- [x] TODO-137 [P0] Рендер полных кубов с текстурами из baked-моделей, tint (биом, редстоун, стебли, ...)
- [x] TODO-138 [P0] Ambient Occlusion (smooth lighting) — **точно как в ванили** (`ModelBlockRenderer` AO-веса и facing-brightness)
- [x] TODO-139 [P0] Освещение граней: face shade (up 1.0, down 0.5, north/south 0.8, east/west 0.6), light map, ночное зрение/gamma
- [x] TODO-140 [P0] Lightmap текстура: sky/block light, дневное освещение, эффект ночного зрения, гамма, dimension effects
- [x] TODO-141 [P0] Render layers: solid, cutout, cutout_mipped, translucent, tripwire — порядок и сортировка
- [x] TODO-142 [P0] Сортировка прозрачных граней (translucent sorting по расстоянию в секции, пересортировка при движении)
- [x] TODO-143 [P0] Frustum culling по секциям, затем occlusion culling (portal/visibility graph как в ванили «cave culling»)
- [x] TODO-144 [P0] Индирект-рендер: большие вершинные/индексные буферы, аллокатор по секциям, multi-draw-indirect
- [x] TODO-145 [P1] Жидкости: меш воды/лавы (высоты углов, поток, текстуры still/flow, tint воды биомом, подводный оверлей)
- [x] TODO-146 [P1] Блоки с нестандартным видом: растения (cross), двери, лестницы, ограды, стёкла-панели, огонь, плиты, лианы, лозы, лилии, ковры, снег-слои
- [~] TODO-147 [P1] Блоки-сущности (BER): сундуки, таблички, кровати, головы, баннеры, маяк, колокол, спавнер, декор-горшок и др. — отдельный рендер
- [~] TODO-148 [P1] Текст на табличках (в т.ч. glow, двусторонний), висячие таблички
- [~] TODO-149 [P1] Небо: цвет неба по времени/биому, закат/восход, солнце, луна и фазы, звёзды, облака (fast/fancy)
- [~] TODO-150 [P1] Туман: terrain fog, подводный, в лаве, в пещере/darkness, blindness, слепота/темнота эффект, туман Нижнего мира, Энда
- [x] TODO-151 [P1] Погода: дождь/снег (текстуры, анимация по heightmap), небо/туман/облака при дожде и грозе (thunder×rain). Остаток разнесён: 151a, 151b
- [~] TODO-151a [P2] (код готов, болт проверен вживую; вспышка неба не подтверждена; нет звука) Молния: сущность lightning_bolt (LightningBoltRenderer), вспышка неба (skyFlashTime=2), звуки грома — после главы 10 (сущности) и 288
- [x] TODO-151b [P2] Брызги дождя (ClientLevel#tickWeatherEffects: частицы rain/smoke, звук) — после 155 (частицы) и 288
- [~] TODO-152 [P1] Выделение блока (outline), трещины ломания (destroy stages), overlay
- [~] TODO-153 [P1] Ночное видение, тьма (Darkness effect/Warden), спектральный режим
- [x] TODO-154 [P1] Дальность прорисовки, плавная подгрузка, fade чанков (если ваниль не фейдит — не добавлять), порядок загрузки
- [~] TODO-155 [P2] Частицы: ядро системы, текстурные частицы (блок-осколки, дым, пламя, редстоун, вода, лава, сердечки, магия, громкость, портал…), все типы 1:1
- [~] TODO-156 [P2] Видимость границ (F3+G, чанк-границы), F3+B хитбоксы, F3+A перезагрузка чанков
- [~] TODO-157 [P2] Рендер мира в FBO/render targets для пост-эффектов (прозрачность, outline, entity glow)
- [~] TODO-158 [P2] Обновление анимированных текстур на GPU без CPU-копирования
- [~] TODO-159 [P2] Mipmaps уровень по настройке, анизотропия (ваниль: нет → не включать, mipmaps — да)
- [~] TODO-160 [P3] Режимы графики Fast/Fancy: листва, облака, прозрачность — в ванили; реализовать как ванильные опции
- [~] TODO-161 [P3] Улучшенная прозрачность (ванильный `transparency` post-effect)

## Глава 8. Игрок: движение и физика
- [~] TODO-162 [P0] Камера: FOV, поворот (pitch ±90), интерполяция, third-person (F5, 3 режима), bobbing — готово: pitch ±90, F5 (3 режима, отъезд 4 бл. с 8-лучевым getMaxZoom); не готово: динамический FOV (спринт/эффекты), bobbing (нужен игрок, 163+), модель игрока в 3-м лице (гл.10)
- [~] TODO-163 [P0] Игрок как `LocalPlayer`: позиция, скорость, on_ground, yaw/pitch, bounding box, eye height, позы
- [~] TODO-164 [P0] Тик движения `LivingEntity.travel`/`aiStep` — порядок как в ваниле, float/double точность
- [x] TODO-165 [P0] Коллизии: `Entity.collide` по осям (Y → X → Z порядок), step height, VoxelShape-пересечения
- [~] TODO-166 [P0] Ходьба/бег/спринт/подкрадывание/плавание/ползание (позы), скорости и атрибуты (movement_speed, modifiers)
- [~] TODO-167 [P0] Прыжок (jump power, jump boost, honey, slime), гравитация, drag, воздух/земля, friction блока
- [x] TODO-168 [P0] Падение: fall distance, урон (на сервере) + клиентские эффекты, no-fall логика не нужна
- [x] TODO-169 [P0] Пакеты движения: Move Player Pos/Rot/Pos+Rot/StatusOnly, Player Input, Player Command (sprint/sneak/bed leave…)
- [x] TODO-170 [P0] Синхронизация: Teleport (Synchronize Player Position + confirm), Set Entity Motion, rubberband, предсказание
- [x] TODO-171 [P0] Частота отправки позиции как в ванили (раз в тик/каждые 20 тиков heartbeat, пороги дельты)
- [~] TODO-172 [P1] Плавание в воде/лаве, дельфин-граци, подводное дыхание (пузыри в HUD), скорость
- [x] TODO-173 [P1] Лестницы/лианы/леса/строительные леса: climbing, прилипание, спуск при Shift
- [~] TODO-174 [P1] Особые блоки: лёд (friction), слизь (bounce), мёд (slowdown, wall slide), паутина, ягодный куст, кровать (bounce), души-песок/ soul speed, магма
- [~] TODO-175 [P1] Полёт: creative flight (двойной пробел, скорость), элитры (глайд, firework boost, коллизии), spectator no-clip
- [~] TODO-176 [P1] Езда: лодки, вагонетки (рельсы, физика), лошади/ослики/мулы (прыжок, управление), свиньи, страйдер, верблюд, паук-жокей
- [~] TODO-177 [P1] Управление транспортом: Player Input (vehicle), Move Vehicle, Steer Boat, Set Passengers
- [~] TODO-178 [P2] Позы: стояние/присед/плавание/спин/сон/смерть/элитры/дудочка (pose, hitbox размеры)
- [x] TODO-179 [P2] Предсказание сброса позиции сервером, «moved wrongly», «moved too quickly» не ломать
- [~] TODO-180 [P2] Swim-sneak, crawl под блоком, 1.5-блочные проёмы, step-up 0.6, sneaking edge-safety (не падать с края)
- [~] TODO-181 [P2] Респавн: экран смерти, Respawn пакет, запрос статистики, смена измерения при респавне
- [~] TODO-182 [P2] Pushing сущностей (entity collision), толкание игроков и мобов, boat-push
- [~] TODO-183 [P3] Точность дрейфа float между клиентом/сервером на длинных дистанциях, far-lands поведение 1:1

## Глава 9. Взаимодействие с блоками
- [~] TODO-184 [P0] Raycast блока/сущности из глаз (дальность 4.5/5, creative), `ClipContext`, VoxelShape
- [~] TODO-185 [P0] Ломание блока: Player Action start/abort/finish dig, прогресс (`destroyProgress`), скорость по инструменту/эффектам/воде/воздуху — подключено в WorldView::tick_dig (ЛКМ → PlayerAction + Swing + локальное предсказание); КПС: клик = startDestroyBlock без destroyDelay (Digger::click), удержание — с задержкой 5 (проверено владельцем 2026-11-08); частицы-осколки есть; нет инструментов/эффектов/анимации трещин
- [~] TODO-186 [P0] Постановка блока: Use Item On (hand, hit result, inside, sequence), предсказание и ack Block Changed Ack — отправка Use Item On готова (ПКМ → `WorldView::use_item`, `send_use_item_on`), без предсказания блока (нет инвентаря), ack Block Changed не разбирается; seq общий (187)
- [x] TODO-187 [P0] Блок-предсказание: локальная смена блока и откат при отказе сервера — `Level::{start_prediction,predict_block}` + `block_changed_ack` (откат к серверному состоянию), единый seq для dig/use; предсказание постановки появится с инвентарём
- [ ] TODO-188 [P0] Use Item (пища, зелья, лук, щит, удочка, ender pearl), удержание ПКМ, прогресс использования
- [~] TODO-189 [P0] Атака сущности: Interact (attack/interact/interact at), кулдаун атаки (attack strength), sweep, crit — атака готова: луч по сущностям (`pick::raycast_entity`, хитбоксы `world/entity_size.rs` из EntityTypes 26.3, дальность 3.0/5.0 креатив), ЛКМ → `ServerboundAttack`+Swing. Нет: Interact/InteractAt (ПКМ; нужен LpVec3-writer), кулдаун атаки, sweep/crit (нужен инвентарь)
- [ ] TODO-190 [P0] Pick Block (СКМ) в creative/survival, Set Creative Mode Slot
- [ ] TODO-191 [P1] Звуки блоков: step, break, place, hit, fall — по SoundType (volume/pitch как в ванили)
- [ ] TODO-192 [P1] Частицы ломания/удара/хождения блока, анимации руки (swing), смена предмета (equip animation)
- [~] TODO-193 [P1] Руки/предмет от первого лица: позиция, swing, use-анимации (eat, bow, block, spyglass, trident, brush, goat horn), map in hand — eat/drink сделано 2026-11-09 (частицы item и остальные use-анимации открыты)
- [ ] TODO-194 [P1] Рендер предмета в руке, в 3-м лице, на земле (item entity), в рамке, на ArmorStand, GUI
- [ ] TODO-195 [P1] Инструментальные эффекты: Efficiency, Haste, Mining Fatigue, aqua affinity, подводное ломание, не на земле (÷5)
- [ ] TODO-196 [P1] Контейнеры по клику: сундук, печь, верстак, наковальня и т.д. — открытие через Open Screen
- [ ] TODO-197 [P2] Блок-взаимодействия: двери, люки, калитки, кнопки, рычаги, нотный блок, колокол, кровать, якорь возрождения, переключатели
- [ ] TODO-198 [P2] Adventure/Spectator ограничения: can_break/can_place, невозможность ломать
- [~] TODO-199 [P2] (cooldown предметов ✔ пакет+плашка в иконках, не проверено вживую) Cooldown предметов (ender pearl, shield), использование щита (blocking), shield disable axe
- [ ] TODO-200 [P2] Block interaction range и entity interaction range (атрибуты, 1.20.5+)
- [ ] TODO-201 [P3] Креатив: мгновенное ломание, drop creative mode, сброс предметов

## Глава 10. Сущности
- [ ] TODO-202 [P0] Реестр EntityType (все типы, размеры хитбокса, eye height, tracking range, update interval)
- [~] TODO-203 [P0] (готово: add/remove/move/teleport/sync/rotate_head; нет: velocity) Пакеты: Spawn Entity, Remove Entities, Entity Position Sync, Move Entity Pos/Rot/Both, Rotate Head, Entity Velocity, Entity Teleport
- [ ] TODO-204 [P0] Entity Metadata (Synched Data): serializers, все типы (byte/int/float/string/component/slot/…/pose/villager data/optional), индексы по классам
- [ ] TODO-205 [P0] Entity Event (статусы), Animate (swing, hurt, crit), Hurt Animation, Damage Event, Entity Sound
- [ ] TODO-206 [P0] Set Equipment, Set Passengers, Set Entity Link (leash), Attach Entity, Update Attributes, Entity Effect/Remove Effect
- [~] TODO-207 [P0] (готово: хранилище, лерп 3 шага, head rot) Клиентский учёт сущностей: хранилище, интерполяция позиции (`lerp steps`), head rotation, body yaw
- [~] TODO-208 [P0] (готово: Model/Part/Cube/Pose/bake, гуманоид; Keyframe-анимации готовы (keyframe.rs); нет: texture-atlas слоёв, RenderType) Entity model system: Baked model layers, parts, cubes, UV, texture size, poses, animation definitions (Keyframe)
- [ ] TODO-209 [P0] Рендер сущностей: EntityRenderDispatcher, layer renderers, RenderType, освещение (block/sky), hurt overlay (красный), death tilt
- [~] TODO-210 [P0] (готово: модель Стива wide + слои одежды, ходьба/свинг/присед; нет: slim, плащ, элитры, другие игроки) Игроки: модель (classic/slim), слои одежды, плащ, элитры, голова, скелет движений (walk/swing)
- [ ] TODO-211 [P0] Другие игроки: Player Info Update/Remove (add/remove/gamemode/latency/display name/listed/chat session), tab list
- [ ] TODO-212 [P1] Item entity (предмет на земле): bobbing, вращение, стопка визуально, подбор (Pickup Item анимация)
- [ ] TODO-213 [P1] XP-орб, стрела, снаряды (снежок, яйцо, жемчуг, зелье, трезубец, огненный шар, череп визера, фейерверк), падающий блок, TNT
- [ ] TODO-214 [P1] Мобы пассивные: курица, корова, свинья, овца (шерсть, стрижка), лошади, кролик, лиса, волк, кошка, оцелот, попугай, панда, лама, летучая мышь, черепаха, дельфин, жабы, аксолотль, пчела, ослик, мул, страйдер, верблюд, армадилло, олень… — каждый: модель, текстуры, вариации
- [ ] TODO-215 [P1] Мобы враждебные: зомби (вариации), скелет (вариации), крипер, паук, пещерный паук, эндермен, слизень, магмакуб, гаст, блейз, визер-скелет, ведьма, разбойники, вексы, дрифтед, фантом, утопленник, хоглин, пиглин, страж, древний страж, шалкер, силверфиш, эндермит, варден, бриз, бологд…
- [ ] TODO-216 [P1] Жители/торговцы: деревенский житель (профессии, уровни, биомы), странствующий торговец, zombie villager, golem (железный/снежный), pillager, ravager
- [ ] TODO-217 [P1] Босс-мобы: **Эндер-дракон**, Визер, Страж (Elder Guardian), Warden — модели, анимации, босс-бары
- [ ] TODO-218 [P1] Армор-стенд, лодка/плот/сундучная лодка, вагонетки (все виды), рамка/светящаяся рамка, картина, поводок-узел, маркер, interaction, display (block/item/text display)
- [ ] TODO-219 [P1] Броня на сущностях: equipment layers, trims, окраска кожаной, elytra, glint
- [ ] TODO-220 [P1] Тени, имя над головой (nametag, видимость, sneaking), scoreboard теги, команды-цвет, glow outline
- [ ] TODO-221 [P1] Эффекты состояния: горение (оверлей), урон (красный), смерть (поворот), невидимость, светящийся, левитация, медленное падение
- [ ] TODO-222 [P2] Анимации мобов: ходьба/атака/спец (крипер раздувается, эндермен рот/сундук, ghast, shulker, bee, fox sleeping, panda sitting…)
- [ ] TODO-223 [P2] Вариации по Registry (wolf/cat/frog/painting/pig/cow/chicken variants), декор-горшок черепки
- [ ] TODO-224 [P2] Звуки мобов: ambient/hurt/death/step, привязка к событиям Entity Event/Sound
- [ ] TODO-225 [P2] Спавн-яйца рендер, шерсть/цвета овец (jeb_), ошейники, сёдла, ковры лам, броня лошади, шляпы
- [ ] TODO-226 [P2] Display entities: интерполяция transform, billboard, brightness, text, view range, glow
- [ ] TODO-227 [P2] Interpolation-движение транспорта и головы как в ванили (`lerpTo`, steps 3/4/10)
- [ ] TODO-228 [P3] Статистика производительности: батчинг/инстансинг однотипных сущностей без изменения вида

## Глава 11. Инвентарь, предметы, контейнеры
- [ ] TODO-229 [P0] Реестр предметов и данных (stack size, durability, attributes, компоненты по умолчанию)
- [~] TODO-230 [P0] Инвентарь игрока: хотбар (9), главный (27), броня (4), оффхэнд, крафт 2×2, курсор — ГОТОВО: (a) `protocol/src/item.rs` — грамматика ItemStack/DataComponentPatch 26.3 (формы компонентов из pomme, `read_stack/read_template/read_patch/write_stack`, тест); (b) `client/src/inventory.rs` — `Inventory` (46 слотов InventoryMenu, cursor, selected, state_id, open), `parse_packet` (container_set_content/set_slot, set_cursor_item, set_player_inventory, set_held_slot, open_screen, container_close), тест. (c) подключено в netthread/WorldView (`NetEvt::Inventory`, `WorldView.inv`); (d) слот 1–9/колесо → `set_carried_item` готово. НЕ СДЕЛАНО: (e) частично готово: HUD-хотбар + плоские иконки + количество (`client/hud.rs`, проверено вживую); осталось: иконки по items/*.json, 3D-блоки; (f) экран инвентаря + клики `container_click` (HashedStack: item varint, count, HashedPatchMap — хэши компонентов CRC32C не реализованы, для стеков с компонентами слать пусто → сервер пересинхронизирует); (g) креатив: `set_creative_mode_slot` (short + OPTIONAL_UNTRUSTED stack) и креатив-вкладки; (h) Use Item (188), предсказание постановки блока из руки (186), скорость ломания по инструменту (185)
- [x] TODO-231 [P0] Window/Menu система: Open Screen, Container Set Content/Slot/Property, Set Cursor Item, Close Container
- [~] TODO-232 [P0] (см. STATUS 2026-11-08: честные changedSlots/carried — дописать main.rs) Клики по слотам: Click Container (ModeClickType: pickup, quick move, swap, clone, throw, quick craft, pickup all), state_id, changed slots
- [x] TODO-233 [P0] Колесо/цифры хотбара, Set Carried Item, Drop (Q, Ctrl+Q), смена рук (F)
- [~] TODO-234 [P0] Рендер предметов в GUI (3D блоки/плоские), количество, durability bar, cooldown overlay, glint — glint в GUI проверен вживую 2026-11-08 (хотбар); durability bar/cooldown не начаты
- [~] TODO-235 [P0] (имя/редкость/чары/lore готовы 2026-11-08) Tooltip предмета: имя, lore, атрибуты, энчанты, durability, компоненты, F3+H advanced
- [ ] TODO-236 [P1] Экран инвентаря выживания + персонаж-превью, recipe book кнопка
- [ ] TODO-237 [P1] Верстак, печь/коптильня/плавильня, сундук/большой/эндер/бочка/шалкер, раздатчик/выбрасыватель, воронка, наковальня, точило, зачарователь, ткацкий станок, картографический, камнерез, кузнечный стол, кузница, варочная стойка, маяк, лектерн, жители-торговля, лошадь/лама, крафтер, нейтральные
- [ ] TODO-238 [P1] Рецепты: синхронизация Recipe Book Add/Remove/Settings, подсветка, «place recipe» (Place Ghost Recipe), группы
- [~] TODO-239 [P1] Creative inventory: вкладки (building, colored, natural, functional, redstone, tools, combat, food, ingredients, spawn eggs, op, search, saved hotbars), порядок как в ванили
- [x] TODO-239a [P1] Креатив: данные вкладок генерировать из `CreativeModeTabs.java` 26.3 (цепи, цветные ступени/плиты, новые предметы вроде Sulfur Cube); счёт предметов сверить
- [~] TODO-239b [P1] (данные и выдача готовы, иконки с тинтом нет) Креатив: генерируемые предметы (tipped arrows, зелья, зачарованные книги…) с компонентами + иконки
- [ ] TODO-239c [P1] Иконки: spear, свет gui-трансформа (слишком тёмные), компас/часы вращаются, special-модели (сундук и др.)
- [ ] TODO-239d [P1] Креатив: вкладка Saved Hotbars, тултип и курсор (рука) на вкладках, OP-вкладка
- [ ] TODO-239e [P1] Тултип предметов полный (категория, атрибуты, прочность, чары, lore, редкость)
- [ ] TODO-239f [P1] Креатив: визуал размазывания (подсветка + число), СКМ-размазывание по макс. стаку, Shift по хотбару стирает, Shift-выдача макс. стака; разобрать Shift по корзине
- [ ] TODO-239g [P1] Креатив/инвентарь: (значки пустых слотов — готово 2026-11-08); Q и Shift+ЛКМ по корзине — проверено 2026-11-08; число слота поверх курсора и лимит размазывания — собрано, не проверено (см. STATUS)
- [ ] TODO-239h [P1] Вторая рука: смена на F (проверить), рендер второй руки от первого лица, механика (использование, анимации)
- [ ] TODO-240 [P1] Зачарование: GUI, клики, энчант-подсказки, сид игрока, уровни
- [ ] TODO-241 [P1] Торговля с жителями: Merchant Offers, выбор рецепта, Select Trade, скидки, звёзды уровня
- [ ] TODO-242 [P2] Data components UI: имя, lore, custom model data, attribute modifiers, enchantments, stored enchantments, trim, dyed color, banner patterns, potion contents, firework, written book, map, food, tool, container, bundle…
- [ ] TODO-243 [P2] Bundle (интерфейс, прокрутка), шалкер-ящик в инвентаре, книга и перо (редактор), подписанная книга (читалка)
- [ ] TODO-244 [P2] Карты: Map Data пакет, цвета (палитра базовых цветов ×4 оттенка), рендер карты в руке/рамке/GUI, баннеры/маркеры
- [ ] TODO-245 [P2] Anvil rename (Rename Item), beacon (Set Beacon), command block UI, structure block UI, jigsaw block UI, sign editor, lectern book page, test instance, crafter slots toggling
- [ ] TODO-246 [P2] Quick craft drag (ЛКМ/ПКМ-распределение), double-click collect, shift-click маршруты по меню — 1:1 с `AbstractContainerMenu`
- [ ] TODO-247 [P2] Чат-команда подсказки `/give` не нужны; клиент только отображает
- [ ] TODO-248 [P3] Hotbar saving (C/X в творч.), `hotbar.nbt`, loading
- [ ] TODO-249 [P3] Предметы-«заглушки»: unknown item отображение, missing texture (магент/чёрный)

## Глава 12. Интерфейс (GUI/HUD/меню)
- [ ] TODO-250 [P0] GUI-слой: система виджетов (screen, button, edit box, slider, list, scroll), фокус, tab-навигация, макеты
- [ ] TODO-251 [P0] Шрифт и текст: растеризация bitmap/TTF по font json, ширина символов, кернинг ванильный, тени, жирный/курсив/подчёркнутый/зачёркнутый/obfuscated
- [ ] TODO-252 [P0] Масштаб GUI (auto/1–N), координаты, 9-slice sprites, текстуры виджетов
- [~] TODO-253 [P0] HUD: хотбар, выбор слота, здоровье/голод/воздух/броня/XP-бар/уровень, прицел, джамп-бар лошади, босс-бары — сердца/еда/броня/XP сделаны 2026-11-09; локатор-бар сделан 2026-11-09; осталось: воздух, эффекты, джамп-бар, босс-бары
- [~] TODO-254 [P0] Прицел: crosshair ✔ (2026-11-08), attack indicator (cooldown) — спрайты и расчёт есть (hud.rs, attack_strength), вживую не сверен; F3-прицел 3D-оси не сделан
- [x] TODO-255 [P0] Title screen: логотип, панорама, splash, кнопки, версия, copyright (ванильный вид)
- [~] TODO-256 [P0] Экран мультиплеера: список серверов (`servers.dat`), ping, иконки, MOTD, direct connect, add/edit/delete
- [ ] TODO-257 [P0] Экран подключения/загрузки: «Connecting...», «Loading terrain», disconnected screen (причина), «Encrypting…»
- [~] TODO-258 [P0] Pause menu (Esc), Options (Controls, Video, Sound, Language, Skin Customization, Resource Packs, Accessibility, Chat Settings, Online…) — все опции с ванильными значениями по умолчанию
- [~] TODO-259 [P0] Чат: рендер (history, scroll, fade, цвета, opacity, width/height, narrator нет), ввод (history ↑↓), автодополнение Tab, clickable links, hover
- [ ] TODO-260 [P0] Tab-лист: сетка игроков, пинг-иконки, header/footer, команды, gamemode цвета
- [~] TODO-261 [P0] F3 экран отладки: координаты, чанк, биом, свет, FPS, память, GPU, версия, направление, цель-блок/сущность, сервер — **ванильная раскладка и формат строк**
- [ ] TODO-262 [P0] Экран смерти, «Вы умерли!», счёт, кнопки Respawn/Title
- [ ] TODO-263 [P1] Actionbar, титры (title/subtitle/times), toast’ы (advancements, рецепты, system), subtitles (субтитры звуков)
- [ ] TODO-264 [P1] Боссбар: цвет/оверлей/прогресс, несколько, darken/fog flags
- [ ] TODO-265 [P1] Scoreboard: sidebar, belowName, list; Teams (префикс/суффикс/цвет/видимость); Update Objectives/Score, Reset Score, Display Objective
- [ ] TODO-266 [P1] Advancements: экран (дерево, иконки, рамки, прогресс), toast, пакеты Update Advancements/Select Tab
- [ ] TODO-267 [P1] Экран статистик, Open Statistics/Award Statistics
- [x] TODO-268 [P1] Экран ресурспаков: список, перетаскивание, совместимость pack_format, предупреждения
- [ ] TODO-269 [P1] Экран языка, скинов, звуков (категории громкости), графики (все ванильные слайдеры/переключатели)
- [ ] TODO-270 [P1] Экран «Ошибка подключения»: версия клиента/сервера несовместима с ванильными сообщениями (в нашем случае нужна политика для multi-version)
- [ ] TODO-271 [P1] Диалоги (Dialog API, если в версии): Show Dialog, кнопки, ввод
- [ ] TODO-272 [P2] Edit boxes: выделение, курсор, копи-паст, undo, ввод юникода/эмодзи, IME
- [ ] TODO-273 [P2] Анимированный загрузочный оверлей (Mojang studios/красный фон), прогресс-бар
- [ ] TODO-274 [P2] Realms: **не делаем** (нужна отдельная DEC), кнопка скрыта/заглушка
- [ ] TODO-275 [P2] Single-player/Create World экран — **отложено**, делается в главе 22 (встроенный сервер)
- [ ] TODO-276 [P2] Accessibility: субтитры, high contrast, narrator (не делаем без DEC), «reduce motion», darkness pulsing, glint speed, FOV effects
- [ ] TODO-277 [P2] Экран «Edit Sign», «Book edit», «Command Block», «Structure Block», «Jigsaw», «Test block», «Hanging sign», подтверждения (Chat warnings, Unsecure server warning)
- [ ] TODO-278 [P2] Меню кнопок мыши (ПКМ-контекст в творчестве/креатив поиск)
- [ ] TODO-279 [P3] Вывод «Server Links» экран, «Report Player» (по сети), Social Interactions (блокировка игроков)
- [ ] TODO-280 [P3] Горячие клавиши F3+*: B, G, H, T, A, C, D, N, P, Q (список), F1 скрыть HUD, F2, F4 (gamemode switcher), F5, F11

## Глава 13. Звук
- [ ] TODO-281 [P0] Аудио-бэкенд (cpal/OpenAL-soft-подобный): устройство, микшер, потоки, 3D-позиционирование
- [ ] TODO-282 [P0] Декодирование OGG Vorbis (lewton/symphonia), стриминг музыки, предзагрузка коротких звуков
- [ ] TODO-283 [P0] Sound events: Sound Effect (по id/inline), Entity Sound, Stop Sound, Custom Sound Effect
- [ ] TODO-284 [P0] Позиционный звук: затухание по расстоянию (attenuation как в ваниле), стерео-панорама, допплера нет (ваниль без)
- [ ] TODO-285 [P1] Категории громкости: master/music/records/weather/blocks/hostile/neutral/players/ambient/voice + слайдеры
- [ ] TODO-286 [P1] Шаги/блоки/удары/атаки/урон/еда/плавание/жидкости/фоновые/музыка по `SoundType` и событиям — 1:1
- [ ] TODO-287 [P1] Фоновая музыка (menu, game, creative, dimensions, биомы), задержки (`MusicManager` логика), пластинки/jukebox, нотный блок инструменты
- [ ] TODO-288 [P1] Ambient-звуки пещер (mood), дождь, биомные ambient (loop, additions, mood)
- [ ] TODO-289 [P2] Субтитры: сопоставление sound event → ключ lang, показ с затуханием, направление стрелками
- [ ] TODO-290 [P2] Звуки GUI: клик кнопок, переключение слотов, инвентарь (item pickup, level up, XP)
- [ ] TODO-291 [P2] Звук сердцебиения Warden, sculk, trial, vault, breeze и др. спец. события
- [ ] TODO-292 [P3] Эхо/реверб — нет в ваниле; не делаем

## Глава 14. Чат и команды (unsigned)
- [ ] TODO-293 [P0] Пакеты: Player Chat, Disguised Chat, System Chat, Chat Message (client), Chat Command (client), Signed Chat Command, Chat Session Update
- [ ] TODO-294 [P0] Chat types registry: форматирование (chat.type.text, say, emote…), narration
- [-] TODO-295 ОТМЕНЕНО (§0Б): signed chat не нужен; корректно отправлять unsigned (см. TODO-296)
- [ ] TODO-296 [P0] Unsigned-чат и команды: ванильное поведение клиента без ключей профиля, `enforce-secure-chat=false` на сервере
- [ ] TODO-297 [P0] Команды: клиент шлёт Chat Command, Command Suggestions (Request/Response), автоподсказка по дереву Brigadier
- [ ] TODO-298 [P1] Фильтры: Chat Mode (shown/commands only/hidden), Message Deleted, Clear Titles, Set Action Bar/Title
- [ ] TODO-299 [P1] Индикаторы чата (unsigned-вид), Social Interactions hide — только локально
- [ ] TODO-300 [P1] Форматирование текст-компонентов: click_event (open_url/run_command/suggest_command/copy/ change_page), hover_event (text/item/entity)
- [ ] TODO-301 [P2] Клиентские команды: **нет** в ванили (кроме нескольких); не придумывать, свои — только по DEC
- [ ] TODO-302 [P2] Whisper/Team message/emote отображение, system messages, death messages, advancement messages
- [ ] TODO-303 [P3] Репорт сообщений (Report Chat) — API Mojang, опционально

## Глава 15. Измерения и порталы
- [ ] TODO-304 [P0] Обычный мир: небо/цвета/туман/облака/звёзды как в Overworld
- [ ] TODO-305 [P0] Смена измерения: Respawn/Login (dimension, dataKept), очистка мира, загрузочный экран, Spawn pos, новый dimension_type
- [ ] TODO-306 [P0] Нижний мир: dimension effects (нет неба, потолок, туман красный), освещение, лава-свет, частицы (ash, spore), музыка/звуки
- [ ] TODO-307 [P0] Эффект портала Нижнего мира: оверлей-искажение, звук, прогресс (Nausea-like), задержки как в ванили
- [ ] TODO-308 [P0] Энд: пустое небо (endsky texture), туман Энда, свет, острова
- [ ] TODO-309 [P1] Порталы: рамка портала (блок), анимированная текстура, частицы, звук triggers, End Portal frame/сейчас с глазами (Eye of Ender), End Gateway, End Portal (визуал «звёздное небо» в блоке — шейдер)
- [ ] TODO-310 [P1] Эффекты неба Нижнего/Энда (`DimensionSpecialEffects`: sky type, fog color, cloud height, constant ambient light, forceBrightLightmap)
- [ ] TODO-311 [P1] Тьма Энда: Ender Eye/ Ender Pearl в Энде телепортация, рендер End Gateway beam, end rod частицы
- [ ] TODO-312 [P2] Мир с custom dimension: чтение dimension_type из реестра; высота/min_y ≠ −64/384; coordinate_scale
- [ ] TODO-313 [P2] Граница мира: Initialize/Set Border Center/Lerp Size/Size/Warning Delay/Distance — рендер полупрозрачной стены, цвет по состоянию

## Глава 16. Игровая механика на клиенте (всё, что видит игрок)
- [ ] TODO-314 [P0] Режимы игры: survival/creative/adventure/spectator, Game Event (change gamemode, begin raining, win game…), Player Abilities, Gamemode switcher F3+F4
- [ ] TODO-315 [P0] Здоровье/голод/насыщение: Set Health/Food пакеты, HUD, эффект дрожания, мигание сердец (урон/отрегенерация/яд/иссушение/заморозка/поглощение), absorption
- [~] TODO-316 [P0] Опыт: Set Experience, оболочка уровня, XP-бар, звуки, орбы — XP-бар/уровень сделаны 2026-11-09 (звуки/орбы нет)
- [ ] TODO-317 [P0] Эффекты зелий: Entity Effect, иконки в HUD (+таймеры), инвентарь эффектов, рендер эффектов на экране (nausea, blindness, darkness, nightvision)
- [ ] TODO-318 [P0] Броня: HUD, damage reduction визуализация нет (только HUD), прочность, звуки
- [ ] TODO-319 [P0] Смерть и респавн: Combat Death, Death Screen, hardcore режим (экран, замена кнопок), spectate
- [ ] TODO-320 [P1] Огонь, лава, кактус, утопление: оверлеи на экране (fire, water, suffocation, portal, pumpkin, powder snow freezing, spyglass)
- [ ] TODO-321 [P1] Спальня: кровать (пакет Sleep, ночь затемнение), спаун точка, Set Default Spawn Position, Player Rotation, Respawn Position
- [ ] TODO-322 [P1] Зачарование предмета (рендер glint), ремонт/объединение наковальня (клиентская логика цены — вывод)
- [ ] TODO-323 [P1] Крафт: Place Recipe, Recipe Book, shaped/shapeless подсветка, результат (preview) — результат считает сервер, UI показывает
- [ ] TODO-324 [P1] Приготовление/плавка: прогресс-стрелки, огонь печи, xp
- [ ] TODO-325 [P1] Варка зелий: стойка, прогресс, fuel
- [ ] TODO-326 [P1] Торговля: цены, скидки (Hero of the Village), блокировки, обновления
- [~] TODO-327 [P1] (поршни: клиентская симуляция сделана, не проверено вживую; толкание игрока и дым раздатчика сделаны 2026-11-08; TNT-мигание сделано (без дыма); осталось: колокол (нужен BE-рендер), проверка мёда вживую; слизь и дым раздатчика проверены) Редстоун-визуал: провод (цвет по силе, частицы, соединения), повторитель/компаратор состояния, поршни (анимация Block Event: рука поршня — block entity piston moving), observer, target, дроппер-визуал, TNT мигание, рельсы активные
- [ ] TODO-328 [P1] Огонь/лава/вода течение: рендер анимаций, пузыри, дым, испарение в Nether
- [ ] TODO-329 [P1] Звуки/частицы блоков-«событий»: Level Event (дверь, диск, эффекты зелья, вспышка, разрушение блока), Level Particles
- [ ] TODO-330 [P1] Фейерверки: Firework Rocket entity, взрыв (Entity Event 17), форма/цвета/трейл/мерцание — частицы 1:1
- [ ] TODO-331 [P2] Рыбалка: поплавок (entity), рендер лески, bite анимация, звук
- [ ] TODO-332 [P2] Лук/арбалет/трезубец/спиртовые: анимации натяжения (item properties/`item model` range_dispatch use_duration/pull)
- [ ] TODO-333 [P2] Щит: блок-анимация, баннер на щите, рендер от 1-го/3-го
- [ ] TODO-334 [P2] Эндер-сундук, ender chest sound/частицы, enderman teleport частицы
- [ ] TODO-335 [P2] Коралловые/морские: пузыри (bubble column), kelp, морские огурцы/сердце моря/conduit, вода-эффекты, дыхание
- [ ] TODO-336 [P2] Лес/пещеры: sculk-датчик частицы, sculk shrieker, warden, glow lichen/berries, azalea, moss, dripstone, амет, deep dark — всё визуально
- [ ] TODO-337 [P2] Стелс/сенсоры: пакеты Game Event sculk/«vibration» (Entity/Block positions), рендер вибраций
- [ ] TODO-338 [P2] Trial chambers: trial spawner, vault, crafter, ominous события — визуал и частицы, ключи
- [ ] TODO-339 [P2] Деревенская жизнь: колокол, рейд (боссбар), «Bad Omen/Raid Omen» эффекты, патруль
- [ ] TODO-340 [P2] Поводки, седла, приручение (частицы сердец/дыма), разведение (Entity Event 18), стадии роста, baby модели (размеры, голова)
- [ ] TODO-341 [P2] Погода-интенсивность: Game Event (rain level change, thunder level change), эффекты на небо/свет
- [ ] TODO-342 [P2] Время: Update Time (world age, time of day, tick day time flag), день/ночь циклы, `doDaylightCycle` визуально
- [ ] TODO-343 [P3] Spyglass, Goat horn, Brush (археология), Decorated pot, Chiseled bookshelf, Calibrated sculk, Armadillo scute — визуальные детали и звуки
- [ ] TODO-344 [P3] Easter eggs: jeb_ овцы, Dinnerbone/Grumm (вверх ногами), toast, Herobrine нет, Killer Bunny, Steve/Alex дефолт скины по UUID, панорамы с версией

## Глава 17. Энд и Эндер-дракон (финал)
- [ ] TODO-345 [P0] Путь в Энд: Eye of Ender (бросок, полёт к крепости, ломание, частицы), Stronghold, End Portal frame с глазами → активация
- [ ] TODO-346 [P0] Рендер блока End Portal (звёздный шейдер), эффект при входе, смена измерения, спаун на платформе обсидиана
- [ ] TODO-347 [P0] Энд: острова, обсидиановые башни, эндер-кристаллы (модель, вращение, луч к дракону), клетки на башнях
- [ ] TODO-348 [P0] Модель Эндер-дракона: части (голова/челюсть/шея/тело/крылья/ноги/хвост), анимация крыльев, шея/хвост по истории позиций (`DragonFlightHistory`), красные глаза
- [ ] TODO-349 [P0] Фазы дракона (визуально): hover, circling, strafing, landing, sitting, charging, dying — Entity Event/метаданные (Phase), дыхание (AreaEffectCloud частицы), огненные шары
- [ ] TODO-350 [P0] Босс-бар дракона (Boss Event пакеты: add/remove/health/title/style/flags), фиолетовый, darken sky effect
- [ ] TODO-351 [P0] Смерть дракона: анимация (развоплощение, лучи света, частицы), XP-орбы большой взрыв, Entity Event 3 / death ticks, звук
- [ ] TODO-352 [P0] Выходной портал (Exit Portal) генерация видна: bedrock фонтан, драконье яйцо, портал, луч в небо
- [ ] TODO-353 [P1] Эндер-кристаллы взрываются (пакет Explosion, particles, звук), урон, лучи кристалл→дракон отрисовать
- [ ] TODO-354 [P1] Эндермены в Энде, шалкеры, Ender city / End ship / элитры / dragon head, врата (End Gateway), хорус (chorus plant/flower визуал, рост)
- [ ] TODO-355 [P1] Ender pearl бросок/ телепорт, звуки, частицы
- [ ] TODO-356 [P1] Взрыв (Explosion пакет): частицы, звук, выбитые блоки анимации, knockback (Set Entity Motion)
- [ ] TODO-357 [P1] Game Event «win game» (код 4) → экран концовки: Credits/Poem (текст End Poem `end.txt`, титры `credits.json`, скорость прокрутки как в ванили, пропуск), «Respawn»/ Perform Respawn
- [ ] TODO-358 [P2] Скролл поэмы Энда: текст с цветами (gradient/random), фон `end_portal` шейдер, музыка `credits`, звук
- [ ] TODO-359 [P2] Достижения/advancement «Free the End» отображение (toast)
- [ ] TODO-360 [P2] Эндерная драконья голова/яйцо: рендер блока яйца (текстура), перемещение при касании (визуал), частицы телепорта
- [ ] TODO-361 [P3] Редкие сценарии: Dragon Phase «prevent perching» стрела/дыхание, второе убийство дракона (респавн кристаллами), возрождение дракона (4 кристалла на портале) — визуально кристаллы/лучи/руны

## Глава 17А. Решённые вопросы (см. CLAUDE.md и STATUS.md → DEC-003)
- [x] TODO-362 [P0] DEC: нужен ли одиночный режим (интегрированный сервер)? Сейчас **не цель** — только клиент. Если да → отдельная гигантская глава «Сервер» (генерация мира, симуляция мобов, редстоун, сохранение region/anvil).
- [x] TODO-363 [P0] DEC: async-рантайм сети (tokio vs свой), ECS vs плоская структура сущностей, wgpu vs прямой Vulkan (ash)
- [x] TODO-364 [P0] DEC: политика совместимости версий при несовпадении ванильного UI-текста (клиент 26.3 → сервер 1.8: как показывать «устаревшую версию»)
- [x] TODO-365 [P1] DEC: формат собственного кэша ресурсов/моделей (бинарный, версионированный)
- [x] TODO-366 [P1] DEC: допустимая «ванильность» для оптимизаций, меняющих тайминги кадра, но не поведение (форсированный triple-buffering и т.п.)
- [x] TODO-367 [P2] DEC: Realms, Realms-подобные функции — не делаем / делаем
- [x] TODO-368 [P2] DEC: моды/Fabric/Forge/OptiFine — не делаем (зафиксировано GOAL.md); решить вопрос совместимости с серверами, требующими Forge handshake


Итог: одиночный режим — **планируется позже, глава 22** (после M1, главы 18 и вехи дракона); рантайм — std-потоки+crossbeam; ECS — нет; wgpu+winit; несовпадение версий — молчаливый автодетект; Realms/моды — нет.

## Глава 18. Мульти-версии протокола (1.5.2 → latest)
- [ ] TODO-369 [P0] Архитектура: трейт `Codec` на версию, `Translator` в каноническую модель, цепочка трансляторов, выбор версии по ping/ручной
- [ ] TODO-370 [P0] Полная таблица версий в `data/versions.toml` + проверка всех protocol numbers по первоисточнику
- [ ] TODO-371 [P0] Автодетект версии сервера через Status; fallback-ручной выбор; корректное handshake с protocol версии сервера
- [ ] TODO-372 [P0] Транслятор: каноническая → предыдущая (26.x → 1.21.x) по каждому реальному изменению протокола между соседними версиями
- [ ] TODO-373 [P0] Эра 1.21.x: Configuration, data components, known packs — основной слой
- [ ] TODO-374 [P0] Эра 1.20.5/1.20.3/1.20.2: Configuration state, chunk batch, NBT без имени root, text component как NBT
- [ ] TODO-375 [P1] Эра 1.19.x: signed chat v1/v2/v3, chat session, player info, registry codec в Login Join Game, features
- [ ] TODO-376 [P1] Эра 1.18.x: высота мира, новые чанки (biomes 3D), light в Chunk Data
- [ ] TODO-377 [P1] Эра 1.17: высота, секции bitset в Chunk Data, Update Light, Ping, Pong
- [ ] TODO-378 [P1] Эра 1.16.x: dimension codec, UUID в Spawn, flat NBT в Chunk, Pack/unpack long bitstorage (no spanning)
- [ ] TODO-379 [P1] Эра 1.15/1.14: свет, биомы массивом, heightmaps, POI, Open Book, sound events id
- [ ] TODO-380 [P1] Эра 1.13 «Flattening»: новая нумерация блоков/предметов, Brigadier Commands пакет, tags, recipes, Player Position
- [ ] TODO-381 [P1] Эра 1.12.x: Advancement пакеты, recipe book, unlock recipes, NBT в Slot
- [ ] TODO-382 [P1] Эра 1.9–1.11: wall boundary «second hand», Packed position, переработанные Metadata (VarInt types), Entity Teleport, Resource Pack Status
- [ ] TODO-383 [P1] Эра 1.8.x: сжатие, Position в long, `Chunk Data Bulk`, NBT в Slot, player list, Title, custom payload, spectate, pre-sneak, лёгкий в пинге — **приоритетная**
- [ ] TODO-384 [P1] Эра 1.7.x: Netty, VarInt, UUID профили Mojang, Chunk Data (bit-mask, zlib), метаданные тип/индекс byte, Slot NBT gzip, Entity Equipment
- [ ] TODO-385 [P1] Эра 1.6.x (Pre-Netty): пакет 0x02 Handshake, 0xCD Client Statuses, 0xFA Plugin Message, 0xFC/0xFD шифрование, `0x33` Chunk Data zlib, `0x38` map chunk bulk, строки UTF-16BE (short length)
- [ ] TODO-386 [P1] Эра 1.5.2 (protocol 61): 0xFE Server List Ping формат (старый/расширенный), 0xFA, 0x01 Login Request, 0x03 Chat, 0x14 spawn named entity, metadata как Watchable-object, ChunkData без секций (Anvil/MCRegion формат 8 bit-IDs), нет компрессии пакетов, `0xFD` Encryption Key Request
- [ ] TODO-387 [P1] Блоки/предметы старых версий: числовые ID + meta → канонические BlockState/Item (таблицы миграции flattening, обратные), «отсутствующие» блоки (замена на ближайший)
- [ ] TODO-388 [P1] Мобы/сущности старых версий: объекты (object id) vs entity id vs type id → EntityType; метаданные и их индексы → канонические
- [ ] TODO-389 [P1] Чанки старых форматов: пре-1.2.1 (ID), 1.2.1–1.8 (16-bit id/meta), 1.9–1.12 (paletted), 1.13+ (global palette) — единый загрузчик → каноническая секция
- [ ] TODO-390 [P1] Освещение старых версий (светдатa в пакете чанка, отдельные пакеты, пересчёт на клиенте) → единый путь
- [ ] TODO-391 [P1] Звуки: старые строковые имена → современные id; события, Level Event коды, Particles — таблицы
- [ ] TODO-392 [P1] Команды старых версий: нет Brigadier — tab-complete пакетом, нет дерева; отображение
- [ ] TODO-393 [P1] Чат старых версий: JSON чат с 1.7.2, plain-строки 1.5.2–1.6, цвета `§`, нет подписи
- [ ] TODO-394 [P1] Инвентарь старых версий: id окон, типы окон (строки), слоты (рецепт 2×2 vs 3×3), энч. стол, крафт, клики Mode 0–6, `ClickWindow` confirm transaction (0x6A) — трансляция
- [ ] TODO-395 [P1] Игровая механика, зависящая от версии: ванильная боевая система 1.8 (без cooldown) vs 1.9+ — клиент ведёт себя **по версии сервера**, не по канону (ванильность версии!)
- [ ] TODO-396 [P1] Физика и коллизии версий: различия в движении (например, 1.8 vs 1.9+, swimming 1.13, hitbox позы 1.14, sneak height 1.14/1.15) — эмулировать **физику клиента соответствующей версии**, т.к. сервер/античит её ждёт
- [ ] TODO-397 [P1] Рендер версии: визуальные отличия (модели мобов 1.5 vs 1.8 vs 1.14, текстуры/блоки), «рендерить с ресурсами версии» либо канонические — DEC
- [ ] TODO-398 [P2] Транслятор ViaVersion-like: исходящие пакеты (клиент → сервер) каноническая → версия сервера
- [ ] TODO-399 [P2] Транслятор: записи тегов/реестров для старых серверов (их нет) — подстановка значений по умолчанию из канона
- [ ] TODO-400 [P2] Негативные пути: сервер версии вне диапазона/снапшот/пре-релиз/старые бета (b1.7.3, alpha) — аккуратная ошибка (вне цели)
- [ ] TODO-401 [P2] Snapshot-поддержка: протоколы пре-релизов (0x40000000-флаг) — опционально
- [ ] TODO-402 [P2] Таблица поддержки: каждая версия `ping → login → play-min → play-full → verified`, интеграционные тесты против реального сервера этой версии
- [ ] TODO-403 [P2] `xtask verify-matrix`: прогон интеграционных тестов по всем версиям, отчёт в STATUS.md
- [ ] TODO-404 [P3] Бета/альфа (Classic, Alpha, Beta 1.x) — **не цель** (диапазон от 1.5.2)
- [ ] TODO-405 [P3] Bedrock — не цель (другая игра/протокол)

## Глава 19. Оптимизация (непрерывно, с главы 2)
- [~] TODO-406 [P0] (2026-11-09: samply + llvm-symbolizer работают, см. docs/ref/perf-fps-2026-11-09.md; GPU timestamp queries и бенчмарк с неподвижной камерой — не сделаны) Профайлинг-инфраструктура: tracy/puffin + perf, GPU timestamp queries, кадровые бюджеты
- [ ] TODO-407 [P0] Бенчмарки: парсинг чанка, декомпрессия, меш секции, приём пакетов, рендер кадр (синтетический мир)
- [ ] TODO-408 [P0] Аллокации: арены/пулы для пакетов и мешей, `no-alloc in frame` проверка (dhat/счётчик аллокатора)
- [ ] TODO-409 [P1] Zero-copy парсинг пакетов (`bytes`), переиспользование буферов декомпрессии
- [ ] TODO-410 [P1] SIMD: bitstorage unpack, свет, culling тесты (std::simd или `wide`) с рантайм-детектом
- [ ] TODO-411 [P1] Чанк-хранилище: плоский массив вместо HashMap, быстрый lookup соседей
- [ ] TODO-412 [P1] Greedy meshing или эквивалент, не меняющий вид (согласовано с AO/освещением ванили — только если кадр-в-кадр совпадает)
- [ ] TODO-413 [P1] GPU culling (compute): frustum + occlusion, indirect draw, bindless textures/texture arrays
- [ ] TODO-414 [P1] Асинхронная загрузка ресурсов, параллельное декодирование PNG/OGG, предкэш
- [ ] TODO-415 [P1] Старт < 100 мс до окна: ленивая загрузка, кэш атласов, mmap
- [ ] TODO-416 [P2] Приоритизация меша (центр экрана → края), отмена устаревших задач, бюджет мс на кадр
- [ ] TODO-417 [P2] Память: компактные палитры, выгрузка дальних секций, отчёт использования по подсистемам
- [ ] TODO-418 [P2] PGO/BOLT пайплайн, `target-cpu=native` профили сборки (опциональные)
- [ ] TODO-419 [P2] Ввод→кадр латентность: mailbox/immediate, раннее чтение ввода, late-latching камеры
- [ ] TODO-420 [P2] Сеть: batching, меньше копий, быстрая таблица id→handler, сжатие в отдельном потоке
- [ ] TODO-421 [P2] Энергопотребление/idle: низкий CPU в меню и при паузе
- [ ] TODO-422 [P3] Многоадаптерные системы: iGPU/dGPU выбор, Vulkan 1.3 фичи, mesh shaders (опц., по возможностям)
- [ ] TODO-423 [P3] Автоподбор настроек под железо (не меняя ванильные значения по умолчанию без согласия)

## Глава 19А. Рендер-оптимизации (GOAL §3.3, после M1.4; не менять вид — скриншот-тест с эталонным путём)
- [x] TODO-490 [P1] Compute frustum culling секций на GPU + `draw_indexed_indirect`/`multi_draw_indirect_count` (fallback на CPU-culling TODO-143) — `render/gpu_cull.rs`; непрозрачные слои; прозрачный остаётся на CPU (порядок дальние→ближние). Не подключено в `worldview` (шаг M1.4/M1.5: вызвать `set_visible` + `cull` + `draw`)
- [ ] TODO-491 [P1] Hi-Z: пирамида глубины прошлого кадра, occlusion-тест секций в compute; сочетать с ванильным cave-culling, без «просветов»
- [ ] TODO-492 [P3] Task+Mesh shaders (мешлеты), опционально по feature GPU/wgpu, с fallback на MDI
- [ ] TODO-493 [P2] Occlusion culling сущностей (AABB против Hi-Z/видимости секций)
- [ ] TODO-494 [P2] Instancing сущностей одного типа (мобы, предметы, опыт) одним draw
- [ ] TODO-495 [P2] Замер: Texture2DArray vs атлас (mipmap-bleeding, анимированные текстуры); переходить только при идентичном виде и выигрыше
- [ ] TODO-496 [P1] Суб-аллокатор/кольцо GPU-памяти чанков: переиспользование слотов изменённых секций, дефрагментация, без аллокаций в кадре
- [ ] TODO-497 [P1] UI dynamic batching: текст и 2D-GUI одним буфером, один draw на слой
- [ ] TODO-412/413 (глава 19) — greedy meshing и GPU culling; делать вместе с этой главой

## Глава 20. Качество, релиз, обновление версий
- [ ] TODO-424 [P0] Интеграционные тесты: логин/чанки/движение/блок на реальном сервере последней версии
- [ ] TODO-425 [P0] Golden-тесты: физика (шаги движения), мешинг (хэш меша секции), свет, рецепты, расчёт скорости ломания — с ванили
- [ ] TODO-426 [P0] Тест паритета рендера: скриншот-сравнение (допуск) vs ваниль на одинаковых сценах (seed-мир, позиция)
- [ ] TODO-427 [P1] Fuzz парсеров, тесты на злые пакеты (переполнения, огромные длины)
- [ ] TODO-428 [P1] Стабильность: 24-часовой soak-тест на сервере, утечки памяти, переподключения
- [ ] TODO-429 [P3] Локальная сборка и запуск на машине владельца (релизы/подписи/AppImage/Windows/macOS — не цель, §0Б)
- [-] TODO-430 ОТМЕНЕНО (§0Б): автообновление/распространение не нужно
- [ ] TODO-431 [P1] `xtask update-version <ver>` end-to-end: скачать, сгенерировать данные, diff, новые таблицы, шаблон транслятора, прогон матрицы
- [ ] TODO-432 [P1] Отчёт о расхождениях с ванилью `[vanilla-diff]` (автоматический из golden-тестов)
- [ ] TODO-433 [P2] Документация для разработчиков: `docs/` (архитектура, добавление версии, добавление пакета, добавление блока/сущности)
- [ ] TODO-434 [P2] Диагностика: экспорт дампа, `--debug-net`, репро-запись для баг-репортов
- [ ] TODO-435 [P3] Краш-репорты как в ванили (crash-reports/), «Minecraft crashed» экран

## Глава 21. Приёмка: «От запуска окна до убийства дракона»
Сценарий проходит **живой человек** (или бот на тестовом ванильном сервере) на последней версии; каждый пункт — галочка.

- [ ] TODO-436 [P0] 1. Запуск: окно открывается < 100 мс, title screen как в ванили, панорама, splash, версия
- [ ] TODO-437 [P0] 2. Добавить локальный offline-сервер, ping показывает MOTD/игроков/версию; вход по нику (offline-UUID)
- [ ] TODO-438 [P0] 3. Загрузка мира: видна земля, небо, облака, солнце, звук; FPS по бюджету
- [ ] TODO-439 [P0] 4. Движение: ходьба, бег, прыжок, подкрадывание, плавание, лестницы — физика совпадает с ванилью (golden)
- [ ] TODO-440 [P0] 5. Добыча дерева, крафт верстака/инструментов/печи, плавка железа, сундук
- [ ] TODO-441 [P0] 6. Ночь: мобы (зомби/скелет/крипер/паук) — модели, звуки, урон, смерть, респавн
- [ ] TODO-442 [P0] 7. Голод, еда, здоровье, XP, броня, щит, лук, меч — HUD и анимации
- [ ] TODO-443 [P0] 8. Пещеры и руды: освещение, лава, вода, обвалы, падение, факелы, редстоун-основа
- [ ] TODO-444 [P0] 9. Алмазы, обсидиан, портал в Нижний мир, вход, эффект портала, смена измерения
- [ ] TODO-445 [P0] 10. Нижний мир: свет, туман, визер/блейз/пиглины (торговля)/гаст, крепость, добыча стержней, жемчуг эндера
- [ ] TODO-446 [P0] 11. Око Эндера, поиск крепости, активация портала в Энд
- [ ] TODO-447 [P0] 12. Энд: башни, кристаллы, дракон с боссбаром, бой (лук/меч/кровать-взрыв), смерть дракона, поэма Энда, титры
- [ ] TODO-448 [P0] 13. Выход через выходной портал, возврат в Overworld, респавн-чекпоинты работают
- [ ] TODO-449 [P0] 14. На всём пути: ни одной паники, ни одного `[vanilla-diff]` уровня high/critical, FPS в бюджете
- [ ] TODO-450 [P1] 15. Повторить сценарий на 1.21.x, 1.20.x, 1.8.9, 1.12.2, 1.7.10 (где применимо к версии), затем 1.5.2 (лобби/базовый геймплей)

## Глава 22. Встроенный сервер / одиночный мир (ПОЗЖЕ — после M1, главы 18 и вехи «убить дракона»)
Решение владельца: одиночная игра **планируется**, но отложена. Не брать, пока не закрыты M1, глава 18 и P0/P1 глав 0–17, если владелец не скажет иначе. Сервер **ванильный** (§0): эталон — исходник `server.jar` (`net.minecraft.server.*`, `net.minecraft.world.level.*`). Клиент и сервер общаются через **внутренний канал пакетов** (те же структуры, что по сети, без TCP), версия — последняя каноническая.
- [ ] TODO-459 [P2] Архитектура: крейт `server` (+ `worldgen`, `storage`), внутренний транспорт пакетов клиент↔сервер, серверный тик 20 TPS в отдельном потоке
- [ ] TODO-460 [P2] Серверный `Level`/`ServerLevel`: измерения (overworld/nether/end), game rules, difficulty, время, погода, spawn
- [ ] TODO-461 [P2] Формат хранения: Anvil/region (`.mca`), `level.dat`, `playerdata`, чтение/запись, совместимость с ванильными мирами 1:1
- [ ] TODO-462 [P2] Чанк-менеджер сервера: ChunkMap, статусы генерации (empty→…→full), тикеты, загрузка/выгрузка, view/simulation distance
- [ ] TODO-463 [P2] Шумы и рандом: Perlin/Simplex/Improved noise, Xoroshiro/legacy random, позиционные seeds — бит-в-бит с ванилью
- [ ] TODO-464 [P2] (справочник: chunker-worldgen, GOAL §2А) Генерация мира: noise settings, density functions, router, aquifers, surface rules, carvers (пещеры/каньоны), биомный источник (multi-noise)
- [ ] TODO-465 [P2] Биомы и климат: biome source, параметры, climate sampler, 1:1 с датапаком ванили
- [ ] TODO-466 [P2] Features: деревья, руды, озёра, цветы/трава, грибы, кораллы, ледники, лианы, и т.д. (placed/configured features, порядок шагов генерации)
- [ ] TODO-467 [P2] Структуры: деревни, шахты, крепости, храмы, форты, бастионы, руины, корабли, города Энда, траил-чемберы, древние города — structure sets, placement, pieces
- [ ] TODO-468 [P2] Нижний мир и Энд: генерация, острова Энда, башни, дракон-файт (спаун и состояние), выходной портал
- [ ] TODO-469 [P2] Освещение сервера: sky/block light engine, обновление при изменениях, пересылка клиенту
- [ ] TODO-470 [P2] Тики мира: random ticks, scheduled ticks (блоки/жидкости), рост растений, снег/лёд/дождь, огонь
- [ ] TODO-471 [P2] Жидкости: распространение воды/лавы, взаимодействия, источники, falling
- [ ] TODO-472 [P2] Редстоун: провод, питание, повторитель/компаратор, поршни, наблюдатель, раздатчики, воронки, рельсы — ванильные тайминги и квазисвязность
- [ ] TODO-473 [P2] Блоки-логика: двери, кровати, печи, контейнеры, наковальни, зачарователь, варка, спавнер, портал и др. (block entities на сервере)
- [ ] TODO-474 [P2] Игрок на сервере: ServerPlayer, инвентарь, режимы, голод/здоровье/урон, эффекты, XP, смерть/респавн, достижения, статистика
- [ ] TODO-475 [P2] Валидация движения/действий игрока (как у ванильного сервера), блок-ломание/постановка, предметы на земле, подбор
- [ ] TODO-476 [P2] Датапак: загрузка встроенного data pack ванили (теги, рецепты, лут-таблицы, advancements, loot, damage types, enchantments, worldgen)
- [ ] TODO-477 [P2] Рецепты, лут-таблицы, предикаты, модификаторы лута, энчанты — серверная логика
- [ ] TODO-478 [P2] Сущности на сервере: движение, физика, ИИ (goals/brains), пути (pathfinding), спавн (natural spawning, spawn cap, light/biome rules)
- [ ] TODO-479 [P2] Мобы пассивные/враждебные/боссы, их атрибуты, лут, звуки-события, размножение, приручение
- [ ] TODO-480 [P2] Жители: профессии, POI, расписание, торговля, рейды, големы, деревни
- [ ] TODO-481 [P2] Эндер-дракон на сервере: фазы, кристаллы, боссбар, смерть, портал, яйцо, респавн дракона
- [ ] TODO-482 [P2] Команды (Brigadier) на сервере: основные (`/gamemode`, `/give`, `/tp`, `/time`, `/weather`, `/setblock`, `/fill`, `/summon`, `/kill`, `/gamerule`, `/difficulty`, `/seed`, `/locate` …)
- [ ] TODO-483 [P2] Сетевой слой сервера для внутреннего канала: те же пакеты (Login/Config/Play), chunk batch, tracking сущностей (EntityTracker), keep-alive
- [ ] TODO-484 [P2] Экран «Singleplayer / Create New World / Edit World / Delete / Recreate»: список миров, seed, game mode, difficulty, cheats, bonus chest, game rules, datapacks
- [ ] TODO-485 [P2] Сохранение/автосохранение мира, пауза при открытом меню (тик замораживается как в ванили), корректный выход
- [ ] TODO-486 [P2] Открыть для LAN (`Open to LAN`) — TCP-слушатель для ванильных клиентов, опционально
- [ ] TODO-487 [P3] Загрузка существующих ванильных миров (1.18+ формат), конвертация старых версий (DataFixer) — по необходимости
- [ ] TODO-488 [P3] Golden-тесты сервера: world seed → хэши чанков/структур/биомов с настоящей ванили; тик-тесты редстоуна/жидкостей
- [ ] TODO-489 [P3] Производительность сервера: параллельная генерация чанков, SIMD-шумы, кэши, без изменения результата
