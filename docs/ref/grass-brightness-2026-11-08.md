# Трава/яркость: расследование 2026-11-08 (не закончено)

Жалоба: трава в ezzmc не такая, как в ваниле 26.3; общая яркость не совпадает с «Нормальная» (gamma 0.5). Слева на скрине ezzmc, справа ваниль.

## Замеры (скрин `~/.cache/dms/clipboard/1794100487805896068.png`, время 12700, ясно)
- Среднее по центру: ezzmc (36.6, 47.6, 35.5), ваниль (30.7, 39.8, 29.8). Ваниль темнее в ×0.84, равномерно по каналам (оттенок свёта и тинта совпадает).
- Темнее всего хвост: p2/p50/p98 по G: ezzmc 38/47/59, ваниль 27/40/55. У ванили больше тёмных пятен (диагонали плит травы тёмные, у ezzmc светлее земли).
- Проверка формулой: tex grass_block_top mean=147.4, тинт плains (145,189,89), lightmap texel(block 0, sky 15) при 12700 = (0.434, 0.434, 0.686) → предсказание R=36.4, B=35.3. **ezzmc совпадает с формулой 26.3.** Значит, лайтмап (`render/lightmap.rs`) верен, а ваниль темнее по другой причине.
- gamma в ваниле 0.5 (options.txt стенда), у ezzmc дефолт 0.5 (опция gamma из UI в `LightmapParams` не подключена: `worldview.rs` всегда `default()` — отдельный TODO).
- Для ваниля нужен lightmap ≈ (0.366, 0.576) на этом тексле. Подходит sky-фактор ≈0.30 при том же цвете, либо множитель вершинного цвета ≈0.84. Причина НЕ найдена.

## Расхождения ezzmc ↔ 26.3, найденные в `BlockModelLighter` / `LightCoordsUtil`
1. **Исправлено** (`mesh/lighting.rs`, тесты mesh 30 зелёные):
   - `blend4` → `LightCoordsUtil#smoothBlend` (подстановка центра только если sky(center)>2 || block(center)>2; при sky(нейбор)==0 берётся sky центра).
   - «прозрачный» угол → `isLightPermeable` = `!solidRender || lightDampening==0` (было `!VIEW_BLOCKING || ...`).
2. **НЕ исправлено, главный подозреваемый:** `getDirectionalBrightness` в 26.3 = `shadeDirectionOverride != null ? cardinal.byFace(override) : cardinal.byFace(направление квада)`. Флага `shade` нет. В ezzmc `get_shade(cardinal, dir, q.shade)` для `shade=false` возвращает 1.0. Нужно: прочитать, откуда `shadeDirectionOverride` берётся при бейке (`reference/26.3/client/net/minecraft/client/resources/model/…` BakedQuad/QuadMaterialInfo/FaceBakery, `shade:false` → override=UP?) и привести `assets/bake.rs` + `lighting::get_shade`. Если у крестов/травы override нет, то грани получают 0.8/0.6 (север/юг, запад/восток), что даёт тёмные диагонали, как в ваниле.
3. Не проверено: `flat_light`/`prepareQuadFlat` (lightCoords -1 путь для `getQuads(null)`), `ambientocclusion` в моделях травы (`cross`), `solidRender` флаг у блоков, биомный блендинг (biome zoom не портирован).
4. `crates/mesh/src/lighting.rs` шапка ещё говорит «(1.21.11)» — обновить после правки п.2.

## Стенд
- Окна: ezzmc id 135, ваниль 26.3 id 131 (`niri msg windows`). `niri msg action screenshot-window --id N` ничего не сохранил (молча). `grim` снимает только текущий воркспейс.
- Ванильный F3+4 показывает lightmap — можно попросить владельца показать его на обеих сторонах для прямого сравнения.

## Дальше
1. Закрыть п.2 (shadeDirectionOverride), пересобрать release, новый скрин бок о бок.
2. Если яркость всё ещё ×0.84 — сравнить lightmap через F3+4; проверить значения sky_light_factor/lightmap в ванили.
3. Подключить опцию gamma к `LightmapParams`.
4. STATUS/CODEMAP/TASKS не обновлены (кроме этого файла).

## Обновление (п.2 закрыт)
- 26.3: ключа `shade` в моделях НЕТ (0 вхождений в jar), есть `shade_direction_override` (80 файлов; cross/tinted_cross = "up"). Портировано: `Element`/`BakedQuad.shade_direction_override: Option<Direction>`, `lighting::get_shade(cardinal, dir, override)` = `cardinal[override.unwrap_or(dir)]`. Тесты mesh/assets зелёные, workspace собирается.
- Эффект обратный подозрению: раньше кресты травы в ezzmc получали 0.8/0.6, теперь 1.0 (как в ваниле) → ezzmc станет чуть СВЕТЛЕЕ на крестах. Общий ×0.84 по плитам травы это НЕ объясняет. Причина всё ещё не найдена.
- Идеи дальше: F3+4 lightmap в обеих версиях; проверить sky-darken/`getSkyDarken` и время суток в ваниле (sky-фактор ≈0.30 при том же цвете); биомный блендинг тинта; AO для куба травы (соседние кресты в ваниле не затеняют, у ezzmc?).

## Итог
Владелец подтвердил: после правок (smoothBlend, isLightPermeable, shade_direction_override) трава совпадает с ванилью. Вывод про «override не объясняет ×0.84» выше был неверен: вероятно, вместе с исправлением света override давал вклад. Осталось: gamma из меню → `LightmapParams`.
