# Креатив-инвентарь 26.3: сверка с исходником (2026-11-08)

Источник: `reference/26.3/client/net/minecraft/client/gui/screens/inventory/CreativeModeInventoryScreen.java`,
`HotbarManager.java`, `player/inventory/Hotbar.java`, `Minecraft.java#handleKeybinds`, `LocalPlayer.java`.
Заметки владельца (из памяти) частично неверны — ниже исправления.

## Исправления к заметкам
- **C+цифра = СОХРАНИТЬ, X+цифра = ЗАГРУЗИТЬ** (keySaveHotbarActivator=C, keyLoadHotbarActivator=X; если зажаты оба — загрузка). Только в мире без экрана, только креатив (`hasInfiniteMaterials`), не спектатор. Файл `hotbar.nbt`.
- Антиспам бросков: `TickThrottler(20, 1280)`: +20 за бросок (`handleCreativeModeItemDrop`), −1 за тик, блок при ≥1280 → около 64 бросков подряд, не 5. Блокирует и THROW, и клик вне окна.
- Курсор при закрытии экрана **не пропадает**: он живёт в `inventoryMenu` (ItemPickerMenu.getCarried делегирует), клиентский `removed()` его не трогает. Предмет остаётся «в руке» до следующего открытия инвентаря.
- Подсказка пустого набора: «Save hotbar with C + N» (`inventory.hotbarInfo`), бумага с CREATIVE_SLOT_LOCK — `mayPickup=false`, любой клик по ней игнорируется.
- Тултипы: синие строки с названиями вкладок добавляются у всех слотов, **кроме ячеек каталога во вкладках-категориях**. В Search с `#` — фиолетовые `#minecraft:тег` на позиции 1.
- Вкладка Saved Hotbars: верх, колонка 5 (0-based), справа (x=142), иконка bookshelf, фон `tab_items`, 81 ячейка (9 наборов×9), скролл есть.
- Операторская вкладка: нужны права + опция; не сделана.
- Цифры в Search: работают как хотбар только если наведён непустой слот каталога или любой не-каталожный слот (`doQuickSwap`); иначе идут в текст. Верно как в заметках.
- Остальное из заметок (сетка/хотбар/Survival Inventory/корзина) совпало с исходником и уже было в `creativescreen.rs`.

## Что сделано в коде (не проверено вживую)
- `client/src/hotbars.rs` — `Hotbars` (hotbar.nbt, DataVersion 5023, `{}`/`{id,count}`; компоненты — свой ключ `ezz_components`, VANILLA-UNSURE), тест roundtrip зелёный.
- `creativescreen.rs` — `Kind::Hotbar`, `HOTBAR_TAB=12`, `Cell`, `fill_hotbar_tab`, поиск `#тег` (`data/item_tags.txt`, генератор `tools/gen_item_tags.py`), `tab_lines`/теги в `hover_text`, антиспам в `click()`.
- `inventory.rs`/`worldview.rs` — `drop_spam`, `can_drop`, тик −1.
- `main.rs`/`control.rs` — `hotbar_key(n)` (C/X+цифра, оверлей-сообщение 3 с над хотбаром), курсор не сбрасывается при закрытии.
- `hud.rs` — спрайты вкладки №6.
- `cargo check -p ezzmc-client` чист, тест hotbars зелёный. Release-сборка прервана, живой тест не делался.

## Что осталось
1. `cargo build --release -p ezzmc-client`, запустить на 25611 (сервер уже слушает), проверить через `game_command` (`creative_click` и др.): вкладка Saved Hotbars, C/X+цифра (game_key C down → game_tap цифра), теги `#`, тултипы, антиспам, курсор после закрытия.
2. Добавить в `control.rs`/`ezzmc_mcp.py` поле `state.creative.overlay`/содержимое хотбар-вкладки для проверки.
3. Метки клавиш в подсказке пустого набора сейчас жёстко «C» (не из биндингов).
4. fmt + clippy, обновить STATUS/CODEMAP/TASKS (не сделано).
