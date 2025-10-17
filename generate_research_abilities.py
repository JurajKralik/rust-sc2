# Script to generate src/dicts/research_abilities.rs from the python-sc2 dict
import re
import requests
from pathlib import Path


ABILITY_RS_PATH = Path("src/ids/ability_id.rs")
UNIT_RS_PATH    = Path("src/ids/unit_typeid.rs")
UPGRADE_RS_PATH = Path("src/ids/upgrade_id.rs")
OUT_PATH        = Path("src/dicts/research_abilities.rs")
URL = "https://raw.githubusercontent.com/BurnySc2/python-sc2/develop/sc2/dicts/unit_research_abilities.py"

code = requests.get(URL).text
ns = {}
exec(code, ns)
RESEARCH_INFO = ns["RESEARCH_INFO"]

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
upgrade_id_to_name = parse_rust_enum_variants(UPGRADE_RS_PATH, "UpgradeId")

def render_unit(unit):
    if unit.value in unit_id_to_name:
        return f"UnitTypeId::{unit_id_to_name[unit.value]}"
    else:
        print(f"Unit {unit.name}({unit.value}) not in Rust enums")
        return None

def render_ability(ability):
    if ability.value in ability_id_to_name:
        return f"AbilityId::{ability_id_to_name[ability.value]}"
    else:
        print(f"Ability {ability.name}({ability.value}) not in Rust enums")
        return None

def render_upgrade(upgrade):
    if upgrade.value in upgrade_id_to_name:
        return f"UpgradeId::{upgrade_id_to_name[upgrade.value]}"
    else:
        print(f"Upgrade {upgrade.name}({upgrade.value}) not in Rust enums")
        return None

lines = []
lines.append("// THIS FILE WAS AUTOMATICALLY GENERATED.")
lines.append("// Source: python-sc2 sc2/dicts/unit_research_abilities.py")
lines.append("")
lines.append("use crate::ids::{AbilityId, UpgradeId};")
lines.append("use std::collections::HashMap;")
lines.append("use lazy_static::lazy_static;")
lines.append("")
lines.append("lazy_static! {")
lines.append("\t/// Maps research abilities to their corresponding upgrade IDs")
lines.append("\tpub static ref ABILITY_TO_UPGRADE: HashMap<AbilityId, UpgradeId> = {")
lines.append("\t\tlet mut m = HashMap::new();")

# Build the ability -> upgrade mapping
ability_to_upgrade = {}

for unit, upgrades in sorted(RESEARCH_INFO.items(), key=lambda x: x[0].value):
    u_str = render_unit(unit)
    if u_str is None:
        continue

    for upgrade, info in sorted(upgrades.items(), key=lambda x: x[0].value):
        upgrade_str = render_upgrade(upgrade)
        if upgrade_str is None:
            continue

        if "ability" in info:
            ability = info["ability"]
            ability_str = render_ability(ability)
            if ability_str is None:
                continue
            
            # Store the mapping
            ability_to_upgrade[ability] = upgrade

# Generate the HashMap entries
for ability, upgrade in sorted(ability_to_upgrade.items(), key=lambda x: x[0].value):
    ability_str = render_ability(ability)
    upgrade_str = render_upgrade(upgrade)
    if ability_str and upgrade_str:
        lines.append(f"\t\tm.insert({ability_str}, {upgrade_str});")

lines.append("\t\tm")
lines.append("\t};")
lines.append("}")
lines.append("")
lines.append("/// Get the upgrade ID associated with a research ability")
lines.append("pub fn get_upgrade_for_ability(ability: AbilityId) -> Option<UpgradeId> {")
lines.append("\tABILITY_TO_UPGRADE.get(&ability).copied()")
lines.append("}")

OUT_PATH.parent.mkdir(parents=True, exist_ok=True)
OUT_PATH.write_text("\n".join(lines), encoding="utf8")
print(f"Generated {OUT_PATH}")
print(f"Found {len(ability_to_upgrade)} ability->upgrade mappings")