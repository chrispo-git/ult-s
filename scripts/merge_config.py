import json
import os

def log(msg):
    print(msg, flush=True)

def merge_configs(mod_dir):
    """Merge all config.json files found under mod_dir into a single
    config.json at the root of mod_dir, then delete the originals."""

    LIST_KEYS = {"new-dir-infos"}
    DICT_KEYS = {"new-dir-infos-base", "share-to-vanilla", "new-dir-files", "share-to-added"}

    mod_dir = os.path.normpath(mod_dir)
    root_config = os.path.join(mod_dir, "config.json")

    found = []
    for dirpath, _dirs, filenames in os.walk(mod_dir):
        for name in filenames:
            if name == "config.json":
                found.append(os.path.join(dirpath, name))

    if not found:
        log(f"[merge_configs] No config.json files found under {mod_dir}, skipping.")
        return

    log(f"[merge_configs] Merging {len(found)} config.json file(s)...")

    merged = {"new-dir-infos": [], "new-dir-infos-base": {}, "share-to-vanilla": {}, "new-dir-files": {}, "share-to-added": {}}

    for path in sorted(found):
        with open(path, "r", encoding="utf-8") as f:
            data = json.load(f)
        for key in LIST_KEYS:
            if key in data:
                merged[key].extend(data[key])
        for key in DICT_KEYS:
            if key in data:
                for k, v in data[key].items():
                    if isinstance(v, list) and k in merged[key] and isinstance(merged[key][k], list):
                        merged[key][k] = merged[key][k] + v
                    else:
                        merged[key][k] = v

    with open(root_config, "w", encoding="utf-8") as f:
        json.dump(merged, f, indent=2, ensure_ascii=False)
    log(f"[merge_configs] Written -> {root_config}")

    for path in found:
        if os.path.normpath(path) == os.path.normpath(root_config):
            continue
        os.remove(path)
        parent = os.path.dirname(path)
        while os.path.normpath(parent) != os.path.normpath(mod_dir):
            if not os.listdir(parent):
                os.rmdir(parent)
                parent = os.path.dirname(parent)
            else:
                break

    log("[merge_configs] Done.")