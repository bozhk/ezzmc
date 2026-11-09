# HUD: сердца/еда/броня/XP (сделано) и локатор-бар (НЕ сделан) — выжимка, 26.3

## Сделано (проверено вживую в выживании)
- `crates/client/src/hudbars.rs`: `draw_health` (броня, сердца с миганием и тряской при ≤4, еда), `draw_xp_bar`, `draw_xp_level`; спрайты в `hudbars::SPRITES` (регистрируются в main.rs рядом с `hud::SPRITES`).
- Пакеты в `netthread.rs`: `set_health` (health, food, saturation), `set_experience` (progress, level), `update_attributes` (только свой id: armor, max_health, max_absorption → `NetEvt::Attribute`; формула base+add, *(1+Σmul_base), *Π(1+mul_total)), hardcore из Login. Поля в `WorldView`: food, saturation, xp_progress, xp_level, armor, max_health, hardcore.
- Рисуется только при game_mode 0 или 2 (VANILLA: canHurtPlayer/hasExperience); в креативе сердец/еды/XP нет — это ванильно.
- Не сделано: воздух (пузыри, нужна entity data air supply), поглощение (absorption), эффекты (регенерация — подпрыгивание, яд/иссушение — цвет сердец, голод — спрайты food_*_hunger), ездовое здоровье, `damageCooldownTime` (мигание считаем при любом падении здоровья — VANILLA-UNSURE), `experienceDisplayStartTick`.

## Локатор-бар (то, что хотел владелец: «радар в шкале опыта», игроки видны, без шифта)
Ванильный `Hud#nextContextualInfoState`: если есть waypoints → LOCATOR, но если выживание и XP менялся последние 100 тиков (`experienceDisplayStartTick`) → EXPERIENCE; без waypoints: XP (выживание) или пусто (креатив). Позиция бара: left=(gw-182)/2, top=gh-24-5. Фон `hud/locator_bar_background` 182×5. Уровень XP рисуется поверх при level>0 в выживании.
Шифт: игроки без видимого waypoint — это делает сервер (приседающий не передаётся), клиенту ничего.

### Пакет `waypoint` (ClientboundTrackedWaypointPacket)
`varint operation` (0 TRACK, 1 UNTRACK, 2 UPDATE) + TrackedWaypoint:
- identifier: `bool left` (true → UUID 16 байт, false → UTF string);
- icon: `string style` (Identifier, по умолчанию `minecraft:default`), `optional color`: bool + 3 байта RGB;
- `varint type`: 0 EMPTY (нет данных), 1 VEC3I (3 varint x,y,z), 2 CHUNK (2 varint chunkX, chunkZ), 3 AZIMUTH (f32 радианы).
UNTRACK удаляет по id, UPDATE заменяет данные. `hasWaypoints` = карта непуста. Свой waypoint (id == свой UUID) не рисуется; offline-UUID = nameUUIDFromBytes("OfflinePlayer:"+имя) (в проекте пока нигде не считается; в `world/entities.rs` у сущностей есть `uuid`).

### Рисование (`LocatorBar#extractRenderState`)
- `angle = degreesDifference(camYaw, waypointAngle)` = wrapDegrees(waypointAngle − camYaw); VEC3I/CHUNK: `d = cam − wp` (CHUNK: центр чанка на Y камеры; VEC3I: центр блока, а если id=UUID сущности известной и manhattan<=3 — её глаза), `dir = rotateClockwise90(d) = (−d.z, d.x)`, `waypointAngle = atan2(dir.z, dir.x)` в градусах; AZIMUTH: `angle·180/π`. EMPTY → NaN (не рисовать).
- Видимы только `-60 < angle <= 60`. `dotPos = floor(angle·173/2/60)`; `screenMiddle = ceil((gw−9)/2)`; точка 9×9 в `(screenMiddle+dotPos, top−2)`.
- Цвет: icon.color, иначе `ARGB.setBrightness(opaque(hash), 0.9)`, hash: для UUID `(int)(msb^lsb >>32) ^ (int)(msb^lsb)`, для строки — Java String.hashCode; rgb = hash & 0xFFFFFF. setBrightness: HSB, заменить B на 0.9.
- Спрайт: `WaypointStyle.sprite(distance)`: стиль default = near 128, far 332, спрайты `hud/locator_bar_dot/default_0..3` (есть png; также `bowtie`). dist<near → первый; ≥far → последний; 1 спрайт → он; 3 спрайта → средний; иначе `lerpInt((d−near)/(far−near), 1, n−1)`. distanceSquared: CHUNK/VEC3I — до центра, AZIMUTH/EMPTY — ∞ (последний спрайт). Проверить `assets/minecraft/waypoint_style/*.json` в jar (в распакованной копии каталога нет — взять из jar клиента).
- Стрелка вверх/вниз (`locator_bar_arrow_up/down` 7×5) в `(screenMiddle+dotPos+1, top−6 | top+6)`: для AZIMUTH/CHUNK по горизонту: `ndc = tan(pitch)/tan(fovY/2)`; >1 → UP, <−1 → DOWN (смотрим вниз → горизонт выше → UP). Для VEC3I: `ndcY = tan(elev + pitch)/tan(fovY/2)`, elev = atan2(dy, horizDist) (при |angle|≤60 точка всегда спереди), >1 UP, <−1 DOWN. FOV: `WorldView::fov_deg()` (вертикальный).

### План
1. `net`/`netthread`: разобрать `waypoint` → `NetEvt::Waypoint{op, id, style, color, kind}`; на смену Login/Respawn очищать.
2. `WorldView.waypoints: HashMap<id, Wp>` + `xp_display_start_tick`.
3. `hudbars::draw_locator(p, view_state…)` и выбор бара по `nextContextualInfoState` (в main.rs рядом с `draw_xp_bar`); спрайты `locator_bar_background/arrow_up/arrow_down/dot_*` добавить в `hudbars::SPRITES`.
4. Проверка вживую: стенд с Vanilla (вторым игроком) — точка Vanilla должна быть видна в креативе; с шифтом Vanilla точка пропадает.
