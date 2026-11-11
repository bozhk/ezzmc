#!/usr/bin/env python3
"""Генерирует PROGRESS.md: прогресс задач из TASKS.md и счётчик .md-файлов проекта.

Запуск из корня репозитория:  python3 tools/progress.py
Задачи: `- [x]` сделано, `- [~]` в работе, `- [ ]` не начато, `- [-]` отменено (не считается).
"""

import re
import sys
from collections import OrderedDict, Counter
from datetime import date
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "PROGRESS.md"
SKIP_DIRS = {"target", "reference", "dist", ".git", ".idea", "screenshots", "__pycache__"}
WIDTH = 30
TASK_RE = re.compile(r"^- \[([ x~\-])\] ")
HEAD_RE = re.compile(r"^## (.+?)\s*$")


def bar(done: int, total: int, width: int = WIDTH) -> str:
    """Полоса-слайдер: заполнение и ползунок ● на позиции прогресса."""
    if total <= 0:
        return "░" * width
    pos = min(width - 1, round(done / total * (width - 1)))
    cells = ["█" if i < round(done / total * width) else "░" for i in range(width)]
    cells[pos] = "◉"
    return "".join(cells)


def pct(done: int, total: int) -> str:
    return f"{(done / total * 100):.1f}%" if total else "—"


def parse_tasks(path: Path):
    chapters = OrderedDict()
    current = "Без главы"
    for line in path.read_text(encoding="utf-8").splitlines():
        h = HEAD_RE.match(line)
        if h:
            current = h.group(1).replace("⭐ ", "")
            chapters.setdefault(current, Counter())
            continue
        m = TASK_RE.match(line)
        if m:
            chapters.setdefault(current, Counter())[m.group(1)] += 1
    return chapters


def count_md():
    by_dir = Counter()
    for p in ROOT.rglob("*.md"):
        rel = p.relative_to(ROOT)
        if any(part in SKIP_DIRS for part in rel.parts):
            continue
        folder = "корень" if len(rel.parts) == 1 else "/".join(rel.parts[:-1])
        by_dir[folder] += 1
    return by_dir


def render(chapters, md_by_dir) -> str:
    total_all = Counter()
    for c in chapters.values():
        total_all.update(c)
    done = total_all["x"]
    doing = total_all["~"]
    todo = total_all[" "]
    cancelled = total_all["-"]
    active = done + doing + todo
    md_total = sum(md_by_dir.values())

    lines = [
        "# PROGRESS.md — прогресс проекта",
        "",
        f"> Генерируется `python3 tools/progress.py` из `TASKS.md`. Обновлено: **{date.today().isoformat()}**.",
        "> Не править руками: перезаписывается.",
        "",
        "## Общий прогресс",
        "",
        f"`{bar(done, active)}` **{pct(done, active)}**",
        "",
        f"| Сделано | В работе | Не начато | Отменено | Всего (без отменённых) |",
        f"|---:|---:|---:|---:|---:|",
        f"| ✅ {done} | 🔧 {doing} | ⬜ {todo} | ✖ {cancelled} | **{active}** |",
        "",
        "## По главам",
        "",
        "| Глава | Прогресс | % | Готово / всего |",
        "|---|---|---:|---:|",
    ]
    for name, c in chapters.items():
        d, w, t = c["x"], c["~"], c[" "]
        a = d + w + t
        if a == 0:
            continue
        lines.append(f"| {name} | `{bar(d, a, 20)}` | {pct(d, a)} | {d} / {a} |")

    lines += [
        "",
        "## Markdown-документы в проекте",
        "",
        f"**Всего `.md`: {md_total}** (без `target`, `reference`, `dist`, `screenshots`, `.idea`)",
        "",
        "| Папка | Файлов |",
        "|---|---:|",
    ]
    for folder, n in sorted(md_by_dir.items(), key=lambda kv: (-kv[1], kv[0])):
        lines.append(f"| `{folder}` | {n} |")
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    tasks = ROOT / "TASKS.md"
    if not tasks.exists():
        print("TASKS.md не найден", file=sys.stderr)
        return 1
    OUT.write_text(render(parse_tasks(tasks), count_md()), encoding="utf-8")
    print(f"Записано: {OUT}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
