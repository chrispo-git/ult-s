
import json
import os
import re
from collections import defaultdict


SLOT_PATTERN = re.compile(r'/c(\d+)(?:/|$)')
ADDED_THRESHOLD = 8  # slots c00–c07 are vanilla; c08+ go to added/

# Patterns tried in order to extract a character name from a path.
_CHAR_PATTERNS = [
    # fighter/donkey/...
    re.compile(r'^fighter/([^/]+)/'),
    # sound/bank/fighter/se_donkey_c00.nus3audio
    # sound/bank/fighter_voice/vc_donkey_c00.nus3bank
    re.compile(r'sound/bank/fighter(?:_voice)?/(?:se|vc)_([a-z0-9]+)_'),
    # camera/fighter/donkey/c00/...
    re.compile(r'camera/fighter/([^/]+)/'),
    # prebuilt:/movie/fighter/edge/c00/final_00.h264
    re.compile(r'prebuilt:/[^/]+/fighter/([^/]+)/'),
]


def extract_char(path: str) -> str | None:
    for pattern in _CHAR_PATTERNS:
        m = pattern.search(path)
        if m:
            return m.group(1)
    return None

def is_added_slot(path: str) -> bool:
    slots = [int(m.group(1)) for m in SLOT_PATTERN.finditer(path)]
    if not slots:
        return False
    return max(slots) >= ADDED_THRESHOLD


def split_config(input_path: str, output_dir: str) -> None:
    with open(input_path, "r", encoding="utf-8") as f:
        config = json.load(f)

    def empty_config():
        return {
            "new-dir-infos": [],
            "new-dir-infos-base": {},
            "share-to-vanilla": {},
            "new-dir-files": {},
            "share-to-added": {},
        }

    vanilla: dict[str, dict] = defaultdict(empty_config)
    added:   dict[str, dict] = defaultdict(empty_config)

    def bucket(char: str, path: str) -> dict:
        return added[char] if is_added_slot(path) else vanilla[char]

    for entry in config.get("new-dir-infos", []):
        char = extract_char(entry)
        if char:
            bucket(char, entry)["new-dir-infos"].append(entry)

    for key, val in config.get("new-dir-infos-base", {}).items():
        char = extract_char(key)
        if char:
            bucket(char, key)["new-dir-infos-base"][key] = val

    for key, val in config.get("share-to-vanilla", {}).items():
        char = extract_char(key)
        if char:
            bucket(char, key)["share-to-vanilla"][key] = val

    for key, val in config.get("new-dir-files", {}).items():
        char = extract_char(key)
        if char:
            bucket(char, key)["new-dir-files"][key] = val

    for key, val in config.get("share-to-added", {}).items():
        char = extract_char(key)
        if char:
            bucket(char, key)["share-to-added"][key] = val

    chars_written = set()

    def write(cfg: dict, path: str) -> None:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as f:
            json.dump(cfg, f, indent=2, ensure_ascii=False)

    for char, cfg in vanilla.items():
        out = os.path.join(output_dir, "fighter", char, "config.json")
        write(cfg, out)
        chars_written.add(char)

    for char, cfg in added.items():
        out = os.path.join(output_dir, "fighter", char, "added", "config.json")
        write(cfg, out)
        chars_written.add(char)

    print(f"Done. Split config for {len(chars_written)} character(s):")
    for char in sorted(chars_written):
        has_vanilla = char in vanilla
        has_added   = char in added
        parts = []
        if has_vanilla:
            parts.append("vanilla (c00–c07)")
        if has_added:
            parts.append("added (c08+)")
        print(f"  fighter/{char}/  [{', '.join(parts)}]")


if __name__ == "__main__":
    # scripts/ sits next to romfs/, so go one level up from this file's directory.
    repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    input_path = os.path.join(repo_root, "romfs", "config.json")
    output_dir = os.path.join(repo_root, "romfs")

    split_config(input_path, output_dir)