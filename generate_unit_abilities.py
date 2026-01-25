# Script to generate src/dicts/unit_abilities.rs from the python-sc2 dict
import re
import requests
from pathlib import Path
import logging

# Configure logging
logging.basicConfig(
	level=logging.INFO,
	format='%(levelname)s: %(message)s'
)
logger = logging.getLogger(__name__)


ABILITY_RS_PATH = Path("src/ids/ability_id.rs")
UNIT_RS_PATH    = Path("src/ids/unit_typeid.rs")
OUT_PATH        = Path("src/dicts/unit_abilities.rs")
URL = "https://raw.githubusercontent.com/BurnySc2/python-sc2/develop/sc2/dicts/unit_abilities.py"
UNIT_ID_URL = "https://raw.githubusercontent.com/BurnySc2/python-sc2/develop/sc2/ids/unit_typeid.py"
ABILITY_ID_URL = "https://raw.githubusercontent.com/BurnySc2/python-sc2/develop/sc2/ids/ability_id.py"

logger.info("Fetching unit abilities data from python-sc2")
raw_code = requests.get(URL).text

logger.info("Fetching UnitTypeId definitions")
unit_id_code = requests.get(UNIT_ID_URL).text

logger.info("Fetching AbilityId definitions")
ability_id_code = requests.get(ABILITY_ID_URL).text

# Parse ID values from python-sc2
import re
unit_id_values = {}
for match in re.finditer(r'([A-Z_0-9]+)\s*=\s*(\d+)', unit_id_code):
    name, value = match.groups()
    unit_id_values[name] = int(value)

ability_id_values = {}
for match in re.finditer(r'([A-Z_0-9]+)\s*=\s*(\d+)', ability_id_code):
    name, value = match.groups()
    ability_id_values[name] = int(value)

logger.info(f"Loaded {len(unit_id_values)} unit IDs and {len(ability_id_values)} ability IDs")

# Remove the sc2 imports to avoid protobuf issues
code_lines = raw_code.split('\n')
filtered_lines = []
for line in code_lines:
    if line.strip().startswith('from sc2') or line.strip().startswith('import sc2'):
        continue
    filtered_lines.append(line)
filtered_code = '\n'.join(filtered_lines)

# Create mock classes with actual values
ns = {}
exec("""
class MockId:
    def __init__(self, name, value):
        self.name = name
        self.value = value
    def __repr__(self):
        return f"{self.__class__.__name__}({self.name}, {self.value})"
    def __hash__(self):
        return hash(self.value)
    def __eq__(self, other):
        return self.value == other.value

class UnitTypeId(MockId):
    pass

class AbilityId(MockId):
    pass
""", ns)

# Create instances with actual values
for name, value in unit_id_values.items():
    setattr(ns['UnitTypeId'], name, ns['UnitTypeId'](name, value))

for name, value in ability_id_values.items():
    setattr(ns['AbilityId'], name, ns['AbilityId'](name, value))

exec(filtered_code, ns)
UNIT_ABILITIES = ns["UNIT_ABILITIES"]
logger.info(f"Successfully loaded UNIT_ABILITIES with {len(UNIT_ABILITIES)} units")

def parse_rust_enum_variants(path: Path, enum_name: str) -> dict[int, str]:
    logger.info(f"Parsing {enum_name} from {path}")
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
        logger.warning(f"Unit {unit.name}({unit.value}) not in Rust enums")
        return f"// Unit {unit.name}({unit.value}) not in Rust enums"

def render_ability(ability):
    if ability.value in ability_id_to_name:
        return f"AbilityId::{ability_id_to_name[ability.value]}"
    else:
        logger.warning(f"Ability {ability.name}({ability.value}) not in Rust enums")
        return f"// Ability {ability.name}({ability.value}) not in Rust enums"

lines = []
lines.append("// THIS FILE WAS AUTOMATICALLY GENERATED.")
lines.append("// Source: python-sc2 sc2/dicts/unit_abilities.py")
lines.append("")
lines.append("#![allow(non_upper_case_globals)]")
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
        logger.warning(f"{unit.name}({unit.value}) abilities missing")
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

logger.info(f"Writing output to {OUT_PATH}")
OUT_PATH.parent.mkdir(parents=True, exist_ok=True)
OUT_PATH.write_text("\n".join(lines), encoding="utf8")
logger.info(f"Successfully generated {OUT_PATH}")
