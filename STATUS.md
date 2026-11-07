# STATUS.md — живой журнал ezzmc

> Читай `GOAL.md` первым. Правила ведения этого файла — `GOAL.md §7`.
> Новые записи в журнале — **сверху**. Даты `YYYY-MM-DD`. Номера BUG/TODO/DEC сквозные.
> Правишь точечно, чужие записи не удаляй.

## Текущий фокус
- Фаза: **P0 Фундамент**. Workspace и профили готовы (TODO-009/010/011). `data/versions.toml` готов. CI и бенчи готовы. `fetch-reference` готов. Дальше: TODO-017 (`gen-reports`), 016 (версии <1.14.4), 018+ (кодоген).

## Следующие шаги
1. TODO-017 `xtask gen-reports <ver>` (data generator из server.jar), затем 018 `gen-protocol`.
2. TODO-016: reference для версий до 1.14.4 (нет официальных маппингов).

## Журнал изменений
- 2026-11-07 — [M] `cargo xtask fetch-reference <ver>`: manifest → client.jar (sha1) → Mojmap(ProGuard)→Tiny → tiny-remapper → Vineflower → `reference/<ver>/client/`. Проверено на 1.21.11 (6622 .java, ~1 мин) и 26.3 (jar без обфускации, 7301 .java). Только клиент; сервер/reports — TODO-017; до 1.14.4 — TODO-016. Закрыто TODO-007/015. Зависимости: ureq 2, serde(+json), sha1; внешние: Java 21+, vineflower 1.12.0, tiny-remapper 0.15.0 (скачиваются в `reference/_tools`).
- 2026-11-07 — [S] Бенчмарки: крейт `benches/` (criterion 0.5, `smoke`), `cargo xtask bench [аргументы criterion]` (алиас в `.cargo/config.toml`). Работает. Закрыто TODO-004/014. Новая зависимость: criterion (dev).
- 2026-11-07 — [S] CI `.github/workflows/ci.yml` (fmt на linux, clippy -D warnings + test на linux/win/mac). Локально test зелёный; сам workflow не запускался (репозиторий не публикуется — GOAL §0Б, файл на будущее/локальный линт). Закрыто TODO-003/013.
- 2026-11-07 — [S] `data/versions.toml`: 87 релизов 1.5.2→26.3 (protocol/data_version из PrismarineJS minecraft-data, даты из Mojang manifest). 26.3 = protocol 777. Не проверено на живых серверах. Закрыто TODO-002/012; новая TODO-452 (1.7.3).
- 2026-11-07 — [M] Cargo workspace (7 крейтов + xtask), `rust-toolchain.toml` (stable), профили release/dev/bench/profiling. `cargo check`/`clippy -D warnings` зелёные. Закрыто TODO-001/009/010/011.
- 2026-11-07 — Решение владельца: проект приватный, оффлайн, без аккаунтов; лицензии/публикация не касаются (GOAL.md §0Б). Отменены TODO-071/072/073/077/079/295/430, пересмотрены 051/074/075/296/299/429/437, снят штраф за хранение исходников Mojang.
- 2026-11-07 — GOAL.md дополнен: §0 «всё ванильно», §2А эталонные исходники (читать оригинал → порт на Rust, `reference/` вне git, `docs/ref/` выжимки), §7А лимиты и штрафы.
- 2026-11-07 — Созданы `GOAL.md` и `STATUS.md`. Кода нет.

## Журнал нарушений (ШБ, см. GOAL.md §7А.4)
(формат: `дата | чат/агент | пункт | ШБ | статус`)
(пусто)

## Эталоны (reference)
| Версия | Исходник получен (`xtask fetch-reference`) | Выжимки в docs/ref | Заметки |
|---|---|---|---|
| 1.21.11 | ✅ client (2026-11-07) | — | `reference/1.21.11/client/`, Mojmap |
| 26.3 | ✅ client (2026-11-07) | — | jar без обфускации |

## Матрица версий
Статусы: `—` · `ping` · `login` · `play-min` · `play-full` · `verified`

| Версия | Protocol | Эра | Статус | Тест | Заметки |
|---|---|---|---|---|---|
| 26.3 (latest, канон) | 777 | Config | — | — | из minecraft-data, не проверено на сервере |
| 1.21.x | 767+ | Config | — | — | |
| 1.20.2–1.20.4 | 764–765 | Config | — | — | |
| 1.19.x | 759–763 | Play | — | — | |
| 1.16.5 | 754 | Play | — | — | |
| 1.12.2 | 340 | Play | — | — | |
| 1.8.9 | 47 | Play | — | — | приоритет |
| 1.7.10 | 5 | Netty-early | — | — | |
| 1.6.4 | 78 | Pre-Netty | — | — | |
| 1.5.2 | 61 | Pre-Netty | — | — | нижняя граница |

(Полную таблицу вести в `data/versions.toml`; здесь — сводка. Номера protocol сверять с первоисточником.)

## Бенчмарки
| Дата | Что мерили | Метод | До | После | Коммит |
|---|---|---|---|---|---|
| — | — | — | — | — | — |

## Баги
(пусто)

## TODO
> Полный бэклог по главам (TODO-009…TODO-450, от окна до убийства дракона) — в `TASKS.md`. Здесь держи только срочные/текущие пункты; номера общие, сквозные.

- [x] TODO-001 [P0] Cargo workspace + структура крейтов — 2026-11-07
- [x] TODO-002 [P0] `data/versions.toml` со всеми версиями 1.5.2 → latest — 2026-11-07
- [x] TODO-003 [P0] CI: fmt + clippy + test — 2026-11-07
- [x] TODO-004 [P0] Каркас бенчмарков (criterion/divan) — 2026-11-07
- [ ] TODO-005 [P1] Кодогенерация протокола из официальных `--reports` (`xtask gen-protocol`) — 2026-11-07
- [x] TODO-007 [P0] `xtask fetch-reference <ver>` (jar → маппинги → декомпиляция в `reference/`, в .gitignore) — 2026-11-07
- [ ] TODO-008 [P0] Каркас golden-тестов (`tests-integration/golden/`) с данными из настоящей ванили — 2026-11-07
- [ ] TODO-006 [P7] `xtask update-version <ver>` — 2026-11-07

## Решения
(формат: `DEC-NNN дата — решение — почему — отвергнутые альтернативы`)
- DEC-004 2026-11-07 — MSRV 1.85, edition 2021, toolchain `stable`; имена пакетов `ezzmc-*`; dev: opt-level=1, горячие крейты (protocol/net/world/mesh) opt-level=3; release: lto fat, cgu=1, panic=abort, strip — чтобы dev-клиент был играбелен без полной release-сборки. Edition 2024 не берём (нет выгоды сейчас).
- DEC-003 2026-11-07 — Стек и развилки зафиксированы в `CLAUDE.md` (winit+wgpu+glam, std-потоки без tokio, rayon, без ECS, serde, symphonia+cpal, один процесс, молчаливый автодетект версии) — чтобы агенты не тратили токены на выбор; меняется только решением человека.
- DEC-002 2026-11-07 — Мульти-версии 1.5.2 → latest остаются полной целью: вход на любую версию, у сервера которой `online-mode=false`; стенд на каждую версию — ванильный сервер с этим флагом.
- DEC-001 2026-11-07 — Проект приватный, оффлайн, без аккаунтов; только offline-логин, unsigned-чат, локальные скины; серверы online-mode вне цели — решение владельца (GOAL.md §0Б).

## Заметки по протоколам
(особенности и подводные камни конкретных версий)
(пусто)

## Блокеры
(пусто)

## Архив
(закрытые баги/TODO)
