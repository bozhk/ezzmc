# Стрелы, крит-частицы, чужие игроки (разведка 2026-11-09)

## Сделано в коде (cargo check чистый; релиз после этих правок НЕ пересобран, вживую НЕ проверено)
- `world/entities.rs`: поля `owner, arrow_flags, shake, left_owner, dead`; `tick_projectile(solid, others)` — стрела тикает всегда (раньше пропускала тики при `steps>0` → «зависание 200 мс» после выстрела), попадание в чужого игрока (`others`) → `dead` (VANILLA: `RemotePlayer#hurtClient`=true → `AbstractArrow#onHitEntity` → discard), при ударе в блок: отступ 0.05, `shake=7`, крит сброшен.
- `world/level.rs`: `add_entity` читает `data` (= id владельца), метаданные стрелы `#8` byte = флаги (1 крит), `#10` bool = inGround; мёртвые снаряды удаляются.
- `client/particles.rs`: `Kind::Crit`, `add_crit` (CritParticle: friction .7, gravity .5, цвет .6–.9, g*=.96 b*=.9 за тик); набор `crit` добавлен в список спрайтов (`worldview.rs`). Конструкторский `tick()` не воспроизведён (VANILLA-UNSURE).
- `worldview::tick_particles`: 4 CRIT вдоль хода крит-стрелы (`-m, -my+0.2, -mz`).
- Тряска: `bake_arrow(..., shake, ..)` → `root.zRot += -sin(shake*3)*shake` (градусы).
- Мерцание стрел: ваниль = `entityCutoutCull` (отсечение задних граней). Сдвиг на 0.01 убран; `ParticleDraw::new_culled` (cull Back) для `arrow_draw`. Порядок вершин quad'ов CCW — **проверить вживую**, если стрела невидима/вывернута, cull надо инвертировать/убрать. Скриншот пользователя после сдвига 0.01 показывал «расщеплённые» полосы — вероятно артефакт offset-хака.

## Выводы по ваниле
- Стрела в игрока: на клиенте-наблюдателе (RemotePlayer) стрела исчезает; у локального игрока `hurtClient`=false → клиентский `deflect REVERSE` (отскок 0.2). Мобы: `Entity#hurtClient` false → клиентский deflect. Не реализовано (только удаление при попадании в remote-игроков).
- `ProjectileUtil#computeMargin` = clamp((tickCount−2)/20, 0, 0.3).
- Звук попадания стрелы клиент сам не играет (playSound(null,…) на клиенте молчит).

## НЕ сделано (следующий шаг)
1. **Свинг и предметы в руках у ЧУЖИХ игроков** — полностью отсутствуют. Нужно:
   - пакет `animate` (varint id, u8 action: 0 main swing, 3 off swing, 2 wake, 4 crit, 5 magic crit) и `set_equipment` (varint id; цикл: byte slot|0x80=ещё, `read_stack`; slot ordinal: 0 mainhand, 1 offhand, 2 feet, 3 legs, 4 chest, 5 head, 6 body, 7 saddle — ordinal в enum `EquipmentSlot`, проверь по `reference/26.3/.../EquipmentSlot.java`). Разбор в `world/level.rs::handle` (рядом с `set_entity_data`), в `Entity`: `held: Option<ItemStack>`, `held_off`, `swing_seq: u32`.
   - рендер: `worldview::bake_remote_players` (≈2424): `anim.swing()` при смене `swing_seq` (добавить поле `swing_seen` в `avatar::AvatarAnim`); предмет — как у локального игрока в `worldview.rs` ≈1744–1815: `handitem::stack_ctx` → `hand_items.get` → `mesh.bake(part_matrices(root)[parts.right_arm] * rotX(-90) * rotY(180) * translate(1/16, 0.125, -0.625), View::ThirdPerson, light, item_data.0/1)`; `state.item_right/left = true`; `entity_item_visible = true`; в `bake_remote_players` передать `item_data`.
   - частицы CRIT/magic crit на action 4/5 (`add_crit`).
2. Пользовательский список ещё: проверить вживую (пересобрать `cargo build --release -p ezzmc-client`, `game_stop`+`game_launch`) лук, кулдаун жемчуга, стрелы, крит-частицы; сверка с ванилью бок о бок.
3. Прочее по стрелам: вода (сопротивление 0.6/inertia, пузыри), pierce, tipped-цвет, звук/клиентский deflect от мобов и локального игрока.

## Обновление (после /clear): свинг и предметы в руках чужих игроков — КОД ГОТОВ, вживую не проверено
- `world/level.rs`: пакеты `animate` (0/3 → `swing_seq`, 4/5 → `crit_seq`) и `set_equipment` (слоты 0 mainhand → `held`, 1 offhand → `held_off`); поля в `Entity`.
- `client/avatar.rs`: `swing_seen`; `worldview::bake_remote_players` вызывает `anim.swing()` при смене `swing_seq`, рисует предметы в руках (ItemInHandLayer, как у локального игрока), `state.item_right/left`.
- Релиз пересобран (cargo build --release). Осталось: частицы крита при animate 4/5 (`crit_seq` уже копится, не потребляется), проверка вживую (второй игрок с предметом/свингом), клиентский deflect стрел от локального игрока/мобов.

## Поправка (по `Projectile#deflect`, 26.3)
- Клиентский `deflect` — **no-op** (`if (!isClientSide())`), так что отскок стрелы от игрока/моба на клиенте НЕ моделируется: разворот скорости (REVERSE, power 0.2) приходит только от сервера (пакеты motion/teleport). Прежняя запись «клиентский deflect REVERSE 0.2» неверна — реализовывать его на клиенте не нужно.
- Наблюдение пользователя в ванили: вплотную к игроку стрела отлетает, издалека — стрелы не видно, но звук попадания в блок слышен (звук — серверный). Причина невидимости не выяснена; проверить, как клиент ведёт стрелу после серверного разворота (motion-пакет vs своя физика, `lastDeflectedBy`, `computeMargin`).
