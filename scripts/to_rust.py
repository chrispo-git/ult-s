"""
Converts TOML param config entries into param_config Rust function calls.

Input format (one entry):
    [[param_int]]
    param = "param_special_hi"
    subparam = "landing_frame"
    value = 15
    kinds = ["bayonetta"]
    slots = [0,1,2,3,4,5,6,7]

Output format:
    param_config::update_int_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta"), (smash::hash40("param_special_hi"), smash::hash40("landing_frame"), 15));

Rules:
  - param_float  -> update_float_2
  - param_int    -> update_int_2
  - empty subparam -> 0 instead of smash::hash40("subparam")
  - slots [120..127] -> get_marked_costumes("kind", _)   [left for manual editing]
  - kinds like ["fighter_somethingElse"] -> -*FIGHTER_KIND_SOMETHINGELSE (negated)
  - multi-kind entries (e.g. ["daisy", "daisy_daisydaikon"]) -> one line per kind
"""

import re
import sys
from pathlib import Path


SLOTS_CUSTOM = [120, 121, 122, 123, 124, 125, 126, 127]


def kind_to_fighter_const(kind: str) -> tuple[str, bool]:
    """
    Convert a kind string to a FIGHTER_KIND_XXX constant and a negation flag.

    "fighter_somethingElse" -> ("*FIGHTER_KIND_SOMETHINGELSE", True)  [negated]
    "bayonetta"             -> ("*FIGHTER_KIND_BAYONETTA",     False)
    """
    negated = kind.startswith("fighter_")
    if negated:
        # Strip the "fighter_" prefix, use whatever remains
        name = kind[len("fighter_"):]
    else:
        name = kind

    const = f"*FIGHTER_KIND_{name.upper()}"
    return const, negated


def format_value(raw_value: str, entry_type: str) -> str:
    """Format the value for output. Floats get a trailing .0 if needed."""
    if entry_type == "param_float":
        try:
            f = float(raw_value)
            # Preserve the original string if it already has a decimal point,
            # otherwise append .0 so Rust sees it as a float literal.
            if "." in raw_value:
                return raw_value
            return f"{f:.1f}"
        except ValueError:
            return raw_value
    return raw_value


def build_line(entry_type: str, fighter_const: str, negated: bool,
               kind: str, slots_custom: bool,
               param: str, subparam: str, value: str) -> str:
    """Produce a single output line for one (entry, kind) pair."""

    fn = "update_float_2" if entry_type == "param_float" else "update_int_2"

    first_arg = f"-{fighter_const}" if negated else fighter_const

    if slots_custom:
        costumes = f'get_marked_costumes("{kind}", _)'
    else:
        costumes = f'get_marked_costumes("{kind}","{kind}")'

    param_hash = f'smash::hash40("{param}")'
    subparam_hash = '0' if subparam == "" else f'smash::hash40("{subparam}")'

    return (
        f'param_config::{fn}({first_arg}, {costumes}, '
        f'({param_hash}, {subparam_hash}, {value}));'
    )


def parse_toml_entries(text: str) -> list[dict]:
    """
    Parse a TOML file that consists entirely of [[param_int]] / [[param_float]]
    array-of-table entries. Returns a list of dicts with keys:
        type, param, subparam, value, kinds, slots
    """
    # Split on [[param_int]] or [[param_float]] boundaries
    entry_pattern = re.compile(r'^\[\[(param_int|param_float)\]\]', re.MULTILINE)
    positions = [(m.start(), m.group(1)) for m in entry_pattern.finditer(text)]

    entries = []
    for i, (start, etype) in enumerate(positions):
        end = positions[i + 1][0] if i + 1 < len(positions) else len(text)
        block = text[start:end]

        def extract(key):
            m = re.search(rf'^{key}\s*=\s*(.+)$', block, re.MULTILINE)
            return m.group(1).strip() if m else None

        param = extract("param").strip('"')
        subparam_raw = extract("subparam")
        subparam = subparam_raw.strip('"') if subparam_raw is not None else ""
        value_raw = extract("value")
        kinds_raw = extract("kinds")
        slots_raw = extract("slots")

        # Parse kinds array: ["a", "b", ...]
        kinds = re.findall(r'"([^"]+)"', kinds_raw) if kinds_raw else []

        # Parse slots array
        slots = list(map(int, re.findall(r'\d+', slots_raw))) if slots_raw else []

        entries.append({
            "type": etype,
            "param": param,
            "subparam": subparam,
            "value": format_value(value_raw, etype),
            "kinds": kinds,
            "slots": slots,
        })

    return entries


def convert(input_path: Path) -> list[str]:
    text = input_path.read_text(encoding="utf-8")
    entries = parse_toml_entries(text)

    # Collect (kind, line) pairs, preserving original order
    kind_lines: list[tuple[str, str]] = []
    for entry in entries:
        slots_custom = entry["slots"] == SLOTS_CUSTOM
        kinds = entry["kinds"]

        if entry["param"] in ["tread_jump_speed_y_mul", "tread_mini_jump_speed_y_mul"]:
            continue

        # If there are multiple kinds, the extra ones are sub-object kinds
        # (e.g. ["miiswordsman", "miiswordsman_chakram"]).
        # This means: use the PRIMARY kind only, but negate the FIGHTER_KIND.
        primary_kind = kinds[0]
        has_secondary = len(kinds) > 1

        fighter_const, negated = kind_to_fighter_const(primary_kind)
        if has_secondary:
            negated = True

        line = build_line(
            entry_type=entry["type"],
            fighter_const=fighter_const,
            negated=negated,
            kind=primary_kind,
            slots_custom=slots_custom,
            param=entry["param"],
            subparam=entry["subparam"],
            value=entry["value"],
        )
        kind_lines.append((primary_kind, line))

    # Group by kind, preserving first-seen order of each kind
    seen_order: list[str] = []
    groups: dict[str, list[str]] = {}
    for kind, line in kind_lines:
        if kind not in groups:
            groups[kind] = []
            seen_order.append(kind)
        groups[kind].append(line)

    # Flatten groups with a blank line between each
    output: list[str] = []
    for i, kind in enumerate(seen_order):
        if i > 0:
            output.append("")
        output.extend(groups[kind])

    return output


def main():
    if len(sys.argv) < 2:
        print("Usage: python convert_params.py <input.toml> [output.txt]")
        sys.exit(1)

    input_path = Path(sys.argv[1])
    if not input_path.exists():
        print(f"Error: file not found: {input_path}")
        sys.exit(1)

    output_lines = convert(input_path)

    if len(sys.argv) >= 3:
        output_path = Path(sys.argv[2])
        output_path.write_text("\n".join(output_lines) + "\n", encoding="utf-8")
        print(f"Written {len(output_lines)} lines to {output_path}")
    else:
        for line in output_lines:
            print(line)


if __name__ == "__main__":
    main()