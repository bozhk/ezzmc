#!/usr/bin/env python3
"""Генерирует crates/client/data/tooltip_data.txt для тултипа предмета.
Источники: reference/26.3/reports/minecraft/components/item/*.json (дефолтные компоненты предметов),
Attributes.java (сентимент атрибута), MobEffects.java (категория эффекта и его атрибут-модификаторы),
Potions.java + PotionIds.java (эффекты зелий).
Формат строк (поля через '|'):
  I|имя|редкость|max_damage или -|компонент,компонент,...
  M|предмет|атрибут|сумма|операция|слот|id модификатора
  J|предмет|песня (дефолтный jukebox_playable, ключ реестра без minecraft:)
  D|предмет|компонент,компонент (дефолтный tooltip_display: скрытые компоненты)
  A|атрибут|сентимент (POSITIVE/NEUTRAL/NEGATIVE)
  E|эффект|категория (BENEFICIAL/HARMFUL/NEUTRAL)
  T|эффект|атрибут|сумма|операция|id модификатора
  P|зелье|имя_в_lang|эффект:длительность:амплификатор,...
VANILLA: ItemStack#getTooltipLines, ItemAttributeModifiers, PotionContents#addPotionTooltip."""
import glob, json, os, re

ROOT = "/home/bozhk/PETS/ezzmc"
REF = os.path.join(ROOT, "reference/26.3")
MC = os.path.join(REF, "client/net/minecraft")
OUT = os.path.join(ROOT, "crates/client/data/tooltip_data.txt")


def read(p):
    with open(p, encoding="utf-8") as f:
        return f.read()


def short(name):
    return name.split(":", 1)[-1]


def main():
    lines = []

    # --- Атрибуты: константа -> имя реестра, сентимент (по умолчанию POSITIVE) ---
    attr_src = read(os.path.join(MC, "world/entity/ai/attributes/Attributes.java"))
    attr_const = {}
    attr_sent = {}
    for m in re.finditer(r'public static final Holder<Attribute> (\w+) = register\(\s*"(\w+)"', attr_src):
        const, name = m.group(1), m.group(2)
        start = m.end()
        nxt = attr_src.find("public static final Holder<Attribute>", start)
        body = attr_src[start: nxt if nxt != -1 else len(attr_src)]
        s = re.search(r"Sentiment\.(\w+)", body)
        attr_const[const] = name
        attr_sent[name] = s.group(1) if s else "POSITIVE"
    for name, sent in sorted(attr_sent.items()):
        lines.append(f"A|{name}|{sent}")

    # --- Эффекты: категория и атрибут-модификаторы ---
    eff_src = read(os.path.join(MC, "world/effect/MobEffects.java"))
    eff_const = {}
    for m in re.finditer(r'public static final Holder<MobEffect> (\w+) = register\(\s*"(\w+)",\s*new MobEffect\(MobEffectCategory\.(\w+)', eff_src):
        const, name, cat = m.group(1), m.group(2), m.group(3)
        eff_const[const] = name
        lines.append(f"E|{name}|{cat}")
        start = m.end()
        nxt = eff_src.find("public static final Holder<MobEffect>", start)
        body = eff_src[start: nxt if nxt != -1 else len(eff_src)]
        for t in re.finditer(
            r'\.addAttributeModifier\(\s*Attributes\.(\w+),\s*Identifier\.\w+\("([^"]+)"\),\s*([-\d.]+)F?,\s*AttributeModifier\.Operation\.(\w+)',
            body,
        ):
            attr = attr_const.get(t.group(1), t.group(1).lower())
            op = {"ADD_VALUE": "add_value", "ADD_MULTIPLIED_BASE": "add_multiplied_base", "ADD_MULTIPLIED_TOTAL": "add_multiplied_total"}[t.group(4)]
            lines.append(f"T|{name}|{attr}|{t.group(3)}|{op}|{t.group(2)}")

    # --- Зелья: PotionIds (константа -> id) и Potions (эффекты) ---
    pid_src = read(os.path.join(MC, "world/item/alchemy/PotionIds.java"))
    pid = dict(re.findall(r'ResourceKey<Potion> (\w+) = register\("(\w+)"\)', pid_src))
    pot_src = read(os.path.join(MC, "world/item/alchemy/Potions.java"))
    for m in re.finditer(r'register\(\s*PotionIds\.(\w+),\s*new Potion\("(\w+)"((?:[^;])*?)\)\s*\)\s*;', pot_src):
        pid_name = pid.get(m.group(1), m.group(1).lower())
        lang = m.group(2)
        effs = []
        for e in re.finditer(r"new MobEffectInstance\(\s*MobEffects\.(\w+),\s*(\d+)(?:,\s*(\d+))?\s*\)", m.group(3)):
            effs.append(f"{eff_const.get(e.group(1), e.group(1).lower())}:{e.group(2)}:{e.group(3) or 0}")
        lines.append(f"P|{pid_name}|{lang}|{','.join(effs)}")

    # --- Предметы: дефолтные компоненты, редкость, прочность, модификаторы ---
    for path in sorted(glob.glob(os.path.join(REF, "reports/minecraft/components/item/*.json"))):
        name = os.path.splitext(os.path.basename(path))[0]
        with open(path, encoding="utf-8") as f:
            comps = json.load(f).get("components", {})
        rarity = comps.get("minecraft:rarity", "common")
        maxd = comps.get("minecraft:max_damage", "-")
        names = ",".join(short(k) for k in sorted(comps))
        lines.append(f"I|{name}|{rarity}|{maxd}|{names}")
        jp = comps.get("minecraft:jukebox_playable")
        if isinstance(jp, str):
            lines.append(f"J|{name}|{short(jp)}")
        td = comps.get("minecraft:tooltip_display")
        if isinstance(td, dict) and td.get("hidden_components"):
            hidden = ",".join(short(h) for h in td["hidden_components"])
            lines.append(f"D|{name}|{hidden}")
        for mod in comps.get("minecraft:attribute_modifiers", []):
            lines.append(
                f"M|{name}|{short(mod['type'])}|{mod['amount']!r}|{mod['operation']}|{mod['slot']}|{short(mod['id'])}"
            )

    with open(OUT, "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print(f"{OUT}: {len(lines)} строк")


if __name__ == "__main__":
    main()
