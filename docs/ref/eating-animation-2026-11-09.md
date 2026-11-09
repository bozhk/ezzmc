# Анимация поедания / use-item (выжимка для порта, 26.3) — РЕАЛИЗОВАНО (кроме частиц item), см. STATUS 2026-11-09О

Статус: код не написан, только разведка. Связано с TODO-193. Ниже всё, что нужно, чтобы не перечитывать исходники.

## Что есть / чего нет в ezzmc
- Нет состояния «использует предмет»: ни `using`, ни remaining ticks. `WorldView::use_item` (worldview.rs ~2971) шлёт `UseItem`/`UseItemOn` и всё.
- Нет food level: `netthread.rs` ~485 для `set_health` читает только `f32 health` (дальше идут `varint food`, `f32 saturation`). `NetEvt::Health(f32)` → нужно `Health(f32, i32)`.
- Нет данных consumable: сервер шлёт только patch компонентов, дефолты предметов (consumable/food) надо брать таблицей. В 26.3 нет `reports/items.json`.
- `net/src/play.rs` ~365: `entity_event` для себя разбирается только на 24..=28 (op level); нужен ещё `9` (USE_ITEM_COMPLETE) для локального игрока.
- Рука от 1-го лица: `worldview.rs::prepare_hand` (~1741). Ветка предмета строит `root = rotX*rotY*T(swing)*T(i*0.56,-0.52+inv*-0.6,-0.72)*swingArm`. `tick_hand` (~2782) тикает раз в тик.
- Третье лицо: в ванили для EAT/DRINK позы руки НЕТ (`AvatarRenderer` меняет позу только для BLOCK/BOW/TRIDENT/CROSSBOW/SPYGLASS/TOOT_HORN/BRUSH/SPEAR). Менять `humanoid.rs` не нужно.

## Логика ванили (источники в reference/26.3/client/net/minecraft/)
- `Minecraft#handleKeybinds` (client/Minecraft.java ~1939): если `player.isUsingItem()`: при отпущенном `key.use` → `gameMode.releaseUsingItem` (пакет `player_action` RELEASE_USE_ITEM = 5, pos 0, face down, + локальный stop); клики attack/use/pickItem проглатываются. Иначе клики запускают `startAttack/startUseItem/pickBlock`. Удержание use: `keyUse.isDown && rightClickDelay==0 && !isUsingItem` → `startUseItem`.
- Старт: `ItemStack.use` → `Consumable#startConsuming`: `canConsume` (для еды `player.canEat(canAlwaysEat)` = `abilities.invulnerable || alwaysEdible || foodLevel<20`), затем `LivingEntity#startUsingItem(hand)`: `useItemRemaining = consumeTicks = (int)(consumeSeconds*20)`. Вызывается только если `useItemOn` не «consumed» (наша ветка `!consumed && held_name.is_some()`).
- Тик (`LivingEntity#updatingUsingItem` → `updateUsingItem`): если предмет в руке уже другой (`!isSameItem`) → `stopUsingItem`; `useItem.onUseTick(remaining)` (ItemStack.java:1118): `Consumable#shouldEmitParticlesAndSounds(remaining)`: `used = consumeTicks - remaining; used > (int)(consumeTicks*0.21875) && remaining % 4 == 0` → `emitParticlesAndSounds(random, user, stack, 5)`; потом `--remaining`. На клиенте при 0 ничего не завершается — ждёт сервер.
- `emitParticlesAndSounds`: громкость еды `random.nextBoolean()?0.5:1.0`, pitch `triangle(1.0, 0.2)`; напиток: vol 0.5, pitch `randomBetween(0.9,1.0)`; звук `entity.generic.eat`/`entity.generic.drink` (honey_bottle: `honey_drink`); частицы `item` (count 5 в тике, 16 при завершении) только если `hasConsumeParticles` (еда — да, напитки — нет). Позиция частиц `LivingEntity#spawnItemParticles` (LivingEntity.java:3550).
- Завершение: сервер шлёт `entity_event` 9 → клиент `completeUsingItem`: звук+16 частиц, `stopUsingItem`; слоты придут от сервера. Смена слота хотбара/предмета → `stopUsingItem`.
- Замедление: `LocalPlayer#modifyInput` умножает ввод на `0.2` (UseEffects.DEFAULT speedMultiplier) при `isUsingItem && !passenger` — ДО множителя подкрадывания и «квадратного» выравнивания. `isSlowDueToUsingItem` (= using && !canSprint; у еды canSprint=false): `sprintTriggerTime = 0`, `canStartSprinting` требует `!isSlowDueToUsingItem`. Остановку бега при use ваниль делает через `canStartSprinting`/сброс триггера (в `shouldStopRunSprinting` этого условия нет — сверять по LocalPlayer.java:576-580, 816-820, 1170-1177).
- First-person (`FirstPersonHandsAndItemsRenderer`): условие `isUsingItem && useItemRemainingTicks > 0 && useItemHand == hand` → ветка use вместо swing. Для EAT/DRINK (`customArmTransform=true`, поэтому `applyItemArmTransform` вызывается ПОСЛЕ eat-трансформа):
  ```
  curr = remaining - partialTicks + 1;  scaled = curr / useDuration
  if scaled < 0.8: translate(0, |cos(curr/4*PI)*0.1|, 0)
  j = 1 - pow(scaled, 27);  i = (arm==RIGHT ? 1 : -1)
  translate(j*0.6*i, j*-0.5, 0); rotY(i*j*90); rotX(j*10); rotZ(i*j*30)
  затем applyItemArmTransform: translate(i*0.56, -0.52 + inverseArmHeight*-0.6, -0.72)
  ```
  Swing-трансформ (`swingArm`) в этой ветке не применяется. В нашем `root`: `rotX(pitch-xbob)*rotY(yaw-ybob)` остаются, дальше вместо `T(swing)…swingArm` ставить eat-цепочку + `T(i*0.56,…)`.

## Данные consumable (из Items.java/Consumables.java/Foods.java 26.3)
Дефолт еда: 1.6 с (32 тика), EAT, звук generic_eat, частицы есть. Дефолт питьё: 1.6 с, DRINK, generic_drink, частиц нет.
- Еда по умолчанию: apple, mushroom_stew, bread, porkchop, cooked_porkchop, cod, salmon, tropical_fish, cooked_cod, cooked_salmon, cookie, melon_slice, beef, cooked_beef, cooked_chicken, baked_potato, golden_carrot, pumpkin_pie, rabbit, cooked_rabbit, rabbit_stew, mutton, cooked_mutton, beetroot, beetroot_soup, carrot, potato, sweet_berries, glow_berries; со своими Consumable, но 1.6 с EAT: golden_apple, enchanted_golden_apple, pufferfish, chicken, rotten_flesh, spider_eye, poisonous_potato, chorus_fruit.
- `dried_kelp`: 0.8 с (16 тиков). `honey_bottle`: DRINK 2.0 с (40), звук honey_drink. `milk_bucket`, `potion`, `ominous_bottle`: DRINK 1.6 с (ominous — др. звук после).
- `alwaysEdible` (едят при сытости): golden_apple, enchanted_golden_apple, chorus_fruit, suspicious_stew, honey_bottle. (Питьё без `FOOD` — `canConsume` = true всегда: milk_bucket, potion, ominous_bottle.)

## План реализации (мелкими шагами)
1. `itemdata.rs`: `consumable(name) -> Option<(ticks, drink, always_edible, sound)>` по таблице выше.
2. `set_health`: читать food; `view.food`.
3. `WorldView`: `using: Option<Using{item, slot, remaining, duration, drink}>`; старт в `use_item` (`!consumed`, `can_eat`); `tick_using()` из `tick_hand` (смена предмета/слота → стоп; звук по `shouldEmit…`; частицы позже); `release_using()` (NetCmd `PlayerAction{action:5,…}`).
4. `main.rs` (блок клавиш use ~1795): при `using` — проглатывать клики, при отпущенном use → release; повтор use только если `!using`.
5. `entity_event` 9 для себя → `PlayEvent::UseComplete` → стоп + звук + 16 частиц.
6. `prepare_hand`: eat-цепочка из секции выше.
7. `physics.rs`: `Player.using_item`; `modify_input` ×0.2; `sprint_trigger_time = 0` и запрет старта спринта при use.
8. Частицы `item` (BreakingItemParticle, спрайт предмета) — в `particles.rs` пока только terrain; отдельная подзадача.
9. MCP: поле `using`/`use_remaining` в `state` (`control.rs`) + проверка вживую на стенде (сравнить с ванилью 26.3).

## Сделано в этой же сессии (для журнала)
- Курсор в чате: клик в чате/инвентаре/смерти больше не захватывает мышь (`main.rs` MouseInput); `control_tick` не ставит `grabbed` при открытом чате.
- Ползание: `anim.swimming = swimming || crawling` (`physics.rs`), `SwimPose.visually = swimming || crawling` (`worldview.rs`).
- СКМ: `key.pickItem` → пакет `pick_item_from_block` (`NetCmd::PickBlock`, `net/play.rs`), Ctrl+креатив → includeData.
- F3: строка `Speed` (debugscreen.rs) и `speed`/`speed_xz`/`crouching` в MCP-state (control.rs). Ваниль-стенд: пропатчен `DebugEntryPosition` (jar-копия без подписи `patch/mc-patched.jar`, `vlaunch_p2.sh` в scratchpad сессии 3eab4a0f).
- Ввод: `Input::move_vector` нормализуется как `KeyboardInput#tick` (диагональ с шифтом 1.83 b/s как в ванили), порог спринта `fwd > 1e-5` (hasForwardImpulse).
- Все изменения собраны в release; вживую проверено только запуск MCP и поле скорости.
