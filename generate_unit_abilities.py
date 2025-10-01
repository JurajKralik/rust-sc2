# Script to generate src/dicts/unit_abilities.rs from the python-sc2 dict
import re
import requests
from pathlib import Path


ABILITY_RS_PATH = Path("src/ids/ability_id.rs")
UNIT_RS_PATH    = Path("src/ids/unit_typeid.rs")
OUT_PATH        = Path("src/dicts/unit_abilities.rs")
URL = "https://raw.githubusercontent.com/BurnySc2/python-sc2/develop/sc2/dicts/unit_abilities.py"

code = requests.get(URL).text
ns = {}
exec(code, ns)
UNIT_ABILITIES = ns["UNIT_ABILITIES"]

def parse_rust_enum_variants(path: Path, enum_name: str) -> dict[int, str]:
    text = path.read_text(encoding="utf8")

    m = re.search(rf'pub\s+enum\s+{enum_name}\s*\{{', text)
    if not m:
        raise RuntimeError(f"Could not find enum {enum_name} in {path}")

    i = m.end()
    depth = 1
    j = i
    while j < len(text) and depth > 0:
        if text[j] == "{":
            depth += 1
        elif text[j] == "}":
            depth -= 1
        j += 1
    body = text[i:j-1]

    id_to_name: dict[int, str] = {}
    for line in body.splitlines():
        line = line.strip()
        if not line or line.startswith("//") or line.startswith("#["):
            continue
        m2 = re.match(r"([A-Za-z0-9_]+)\s*=\s*(\d+)\s*,", line)
        if m2:
            name, num = m2.groups()
            id_to_name[int(num)] = name
    return id_to_name

unit_id_to_name    = parse_rust_enum_variants(UNIT_RS_PATH,   "UnitTypeId")
ability_id_to_name = parse_rust_enum_variants(ABILITY_RS_PATH, "AbilityId")

def render_unit(unit):
    if unit.value in unit_id_to_name:
        return f"UnitTypeId::{unit_id_to_name[unit.value]}"
    else:
        print(f"Unit {unit.name}({unit.value}) not in Rust enums")
        return f"// Unit {unit.name}({unit.value}) not in Rust enums"

def render_ability(ability):
    if ability.value in ability_id_to_name:
        return f"AbilityId::{ability_id_to_name[ability.value]}"
    else:
        print(f"Ability {ability.name}({ability.value}) not in Rust enums")
        return f"// Ability {ability.name}({ability.value}) not in Rust enums"

lines = []
lines.append("// THIS FILE WAS AUTOMATICALLY GENERATED.")
lines.append("// Source: python-sc2 sc2/dicts/unit_abilities.py")
lines.append("")
lines.append("use crate::ids::{AbilityId, UnitTypeId};")
lines.append("use std::collections::{HashMap, HashSet};")
lines.append("use lazy_static::lazy_static;")
lines.append("")
lines.append("lazy_static! {")
lines.append("\tpub static ref UnitAbilities: HashMap<UnitTypeId, HashSet<AbilityId>> = {")
lines.append("\t\tlet mut m = HashMap::new();")

for unit, abilities in sorted(UNIT_ABILITIES.items(), key=lambda x: x[0].value):
    u_str = render_unit(unit)
    if u_str.startswith("//"):
        lines.append(f"\t{u_str}")
        continue

    rendered_abilities = []
    for ability in sorted(abilities, key=lambda a: a.value):
        a_str = render_ability(ability)
        rendered_abilities.append(a_str)

    if all(s.startswith("//") for s in rendered_abilities):
        print(f"{unit.name}({unit.value}) abilities missing")
        lines.append(f"\t// {unit.name}({unit.value}) abilities missing")
        continue

    lines.append("\t\tm.insert(")
    lines.append(f"\t\t\t{u_str},")
    lines.append("\t\t\tHashSet::from([")
    for a_str in rendered_abilities:
        lines.append(f"\t\t\t\t{a_str},")
    lines.append("\t\t\t]),")
    lines.append("\t\t);")

lines.append("\t\tm")
lines.append("\t};")
lines.append("}")

OUT_PATH.parent.mkdir(parents=True, exist_ok=True)
OUT_PATH.write_text("\n".join(lines), encoding="utf8")
print(f"Generated {OUT_PATH}")
