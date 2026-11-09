# Броня: что сделано и план на остаток (2026-11-09)

## Уже сделано (проверено вживую)
- `client/src/armor.rs`: `ArmorKit` (атлас 64×32-строк из `entity/equipment/humanoid[_leggings]/<материал>.png`, модели grow 1.0 внешняя / 0.5 ноги), `Look{material,dye}`, `looks_of_menu/entity`. Кожаная = слой `leather` (цвет красителя, иначе 0xA06540) + `leather_overlay`.
- `Entity.armor[4]` (голова, грудь, ноги, ступни) из `set_equipment` слоты 5/4/3/2 (`world/level.rs`).
- `worldview.rs`: отдельный `armor_draw` (ParticleDraw с атласом брони), свой игрок + чужие.
- Превью (`charpreview.rs`): броня, предметы в руках (`HandMesh::raster`), присед, плавание (`SwimPose`), снимок `WorldView::preview_snapshot`.
- Глубина в `ParticleDraw` = `LessEqual`, значит слой с той же геометрией (трим) рисуется поверх брони без z-fighting.

## Осталось (ничего из этого НЕ начато в коде)

### 1. Трим (ArmorTrim)
- Компонент `trim` в патче стека: `Struct[Holder(материал), Holder(паттерн)]` (`protocol/item.rs`, `TRIM_MATERIAL_26_3`, `TRIM_PATTERN`). Holder = VarInt: 0 → inline, иначе индекс реестра + 1.
- Имена из реестров `minecraft:trim_material` и `minecraft:trim_pattern` фазы Configuration (порядок записей = id). Сохранять в статик в `netthread.rs` рядом с `tooltip::ENCHANTS` (там же `reg(..)`). У ванильных записей NBT может быть `None` — хватит имён.
- Паттерн-текстура: `textures/trims/entity/humanoid/<p>.png` и `humanoid_leggings/<p>.png` (64×32, серые; `.mcmeta` palette base_palette=`trim_base`). 18 паттернов: bolt coast dune eye flow host raiser rib sentry shaper silence snout spire tide vex ward wayfinder wild.
- Палитра: `textures/palettes/trim_base.png` (8×1 серых) → целевая `textures/palettes/trim/<материал>.png` (8×1). Замена пикселя: цвет базы с индексом i → цвет цели i (`PaletteMapping`), альфа умножается. Палитры: amethyst copper copper_darker diamond diamond_darker emerald gold gold_darker iron iron_darker lapis netherite netherite_darker quartz redstone resin.
- Материал трима → палитра `trim/<имя>`; `trim_overrides` в `equipment/<материал>.json`: если материал брони = материал трима (gold/iron/diamond/netherite/copper) → `<имя>_darker`.
- Идея реализации: предрассчитать все ячейки (18×16×2=576, сетка 24×24 ячейки = 1536×768) в отдельный атлас + свой `trim_draw`; UV ячейки (cx+u)/cols,(cy+v)/rows (как правка v в `ArmorKit::bake`). Для HUMANOID_BABY трима нет.

### 2. Блеск зачарований (glint)
- `hasFoil` уже есть: `hud.rs` ~329 (`enchantment_glint_override`, иначе есть чары).
- Ваниль: `EquipmentLayerRenderer`: блеск только на первом слое (если нет трима), при триме — отдельный `trimmedArmorGlint` после трима.
- Текстура `textures/misc/enchanted_glint_armor.png`. Матрица текстуры (`TextureTransform.setupGlintTexturing(0.16)` для брони, 0.5 для сущностей): `millis = Util.getMillis()*glintSpeed*8`; `o0=(millis%110000)/110000`, `o1=(millis%30000)/30000`; `translate(-o0,o1,0)·rotZ(π/18)·scale(s)`. В GUI уже реализовано: `hud.rs::glint_frame` (speed 0.5, strength 0.75), `render/gui.rs` glint pipeline — переиспользовать константы.
- Нужен пайплайн с аддитивным блендом + фрагмент с UV-трансформом поверх `particle.wgsl` (depth LessEqual, без записи глубины).

### 3. Элитры
- Слой `equipment/elytra.json` / `textures/entity/equipment/wings/`. Крылья — отдельная модель (ElytraModel), позиция/поза от `isFallFlying`; нужна поза полёта в `avatar`/`HumanoidState`.
- Предмет в слоте груди: `elytra` (`look_of` сейчас принимает только `*_chestplate`).

### 4. Броня на мобах
- Гуманоидные мобы (zombie, skeleton, husk, stray, drowned, zombified_piglin, piglin…) в `client/src/mobs.rs` (`spec()`): нужен `set_equipment` для не-игроков (сейчас `Entity.armor` заполняется для всех сущностей, но рисуется только у игроков) → вызывать `ArmorKit::bake` в `Mobs::bake` с `HumanoidState` моба. У зомби/скелета другая модель позы (`zombie_arms`, `skeleton_mesh`) — armor-модель должна получать ту же позу; у детей (baby) свой слой `humanoid_baby`.
- Лошади/волк/нотилус: отдельные слои (`horse_body`, `wolf_body`…) — вне этого пункта.

### 5. Превью: лежание и элитры
- Лежание (сон): AvatarRenderer#setupRotations для `Pose.SLEEPING` (поворот по `bedOrientation`, translate) — в превью нужно добавить в `root`; полёт на элитрах — наклон по `fallFlyingScale`. Состояние брать из `physics::Player` в `preview_snapshot`.

## Правило
Каждый пункт — проверить вживую (шаги в STATUS: `cargo build --profile fast -p ezzmc-client`, `game_launch`, выдача брони через консоль сервера `item replace entity EzzBot armor.head with ...`; fifo сервера искать `ls /tmp/claude-1000/*/*/scratchpad/in`, команды слать по одной с паузой ≥1 с).
