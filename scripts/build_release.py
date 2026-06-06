"""
build_release.py
----------------
Headless packaging script for CI. Called by the release workflow after
cargo skyline build has already run as its own step.

Usage:
    python build_release.py <version> <mode>

    mode = full | lite | diff

    For diff mode two extra args are required:
    python build_release.py <version> diff <old_arcropolis_dir> <old_stages_dir>
"""

import os
import sys
import shutil
import hashlib
import json
import zipfile
from zipfile import ZipFile

from merge_config import merge_configs

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def log(msg):
    print(msg, flush=True)

def get_all_file_paths(directory):
    file_paths = []
    for root, dirs, files in os.walk(directory):
        for filename in files:
            file_paths.append(os.path.join(root, filename))
    return file_paths

def copytree(src, dst):
    log(f"  copytree: {src} -> {dst}")
    if not os.path.exists(src):
        log(f"  WARNING: source does not exist, skipping: {src}")
        return
    os.makedirs(dst, exist_ok=True)
    for item in os.listdir(src):
        s = os.path.join(src, item)
        d = os.path.join(dst, item)
        if os.path.isdir(s):
            copytree(s, d)
        else:
            shutil.copy2(s, d)

def empty_or_create(folder):
    if os.path.exists(folder):
        log(f"  Emptying {folder}")
        shutil.rmtree(folder)
    os.makedirs(folder)

def make_zip(src_dir, zip_path):
    if os.path.exists(zip_path):
        os.remove(zip_path)
    file_paths = get_all_file_paths(src_dir)
    log(f"  Zipping {len(file_paths)} files -> {zip_path}")
    with ZipFile(zip_path, 'w', compression=zipfile.ZIP_DEFLATED, compresslevel=6) as z:
        for fp in file_paths:
            z.write(fp)
    log(f"  Zip complete: {zip_path}")

def locate_nro():
    """Find libplugin.nro in target/aarch64-skyline-switch/release/"""
    nro = os.path.join('target', 'aarch64-skyline-switch', 'release', 'libplugin.nro')
    if not os.path.exists(nro):
        raise FileNotFoundError(f"libplugin.nro not found at expected path: {nro}")
    return os.path.abspath(nro)

# ---------------------------------------------------------------------------
# Full build
# ---------------------------------------------------------------------------

def build_full(version):
    log("=== Full build ===")
    out_dir = os.path.join('releases', 'ultimate', 'mods', 'Ultimate S Arcropolis')
    empty_or_create('releases')
    os.makedirs(out_dir, exist_ok=True)

    nro = locate_nro()
    shutil.copy2(nro, os.path.join(out_dir, 'plugin.nro'))
    log("  Copied plugin.nro")

    if os.path.exists('romfs'):
        copytree('romfs', out_dir)
    else:
        raise FileNotFoundError("romfs/ not found")

    merge_configs(out_dir)
    
    with open(os.path.join(out_dir, 'version.txt'), 'w') as f:
        f.write(f"v.{version}")

    shutil.copy('readme.txt',  os.path.join('releases', 'readme.txt'))
    shutil.copy('credits.txt', os.path.join('releases', 'credits.txt'))
    
    flag_list = ["bayonetta","brave","buddy","captain","chrom","cloud","daisy","dedede","demon","diddy","dolly","donkey","duckhunt","edge","element","falco","fox","gamewatch","ganon","gaogaen","gekkouga","ike","inkling" ,"jack","kamui","ken","kirby","koopa","koopajr","krool","link","littlemac","lucario","lucas","lucina","luigi","mario","mariod","marth","master","metaknight","mewtwo","miifighter","miigunner","miiswordsman","murabito","ness","packun","pacman","palutena","peach","pichu","pikachu","pikmin","pit","pitb","popo","ptrainer","purin","reflet","richter","ridley","robot","rockman","rosetta","roy","ryu","samus","samusd","sheik","shizue","shulk","simon","snake","sonic","szerosuit","tantan","toonlink","trail","wario","wiifit","wolf","younglink","yoshi","zelda"]

    for flag in flag_list:
        path = os.path.join('releases', 'ultimate', 'ult-s', f"{flag}.flag")
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, 'w') as f:
            f.close()
    make_zip(out_dir, os.path.join('releases', 'Ultimate S Arcropolis.zip'))
    log("=== Full build done ===")

# ---------------------------------------------------------------------------
# Lite build
# ---------------------------------------------------------------------------

def build_lite(version):
    log("=== Lite build ===")
    out_dir = os.path.join('releases-lite', 'ultimate', 'mods', 'Ultimate S Lite')
    empty_or_create('releases-lite')
    os.makedirs(out_dir, exist_ok=True)

    nro = locate_nro()
    shutil.copy2(nro, os.path.join(out_dir, 'plugin.nro'))
    log("  Copied plugin.nro")

    if not os.path.exists('romfs'):
        raise FileNotFoundError("romfs/ not found")

    copytree(os.path.join('romfs', 'fighter', 'common'), os.path.join(out_dir, 'fighter', 'common'))
    copytree(os.path.join('romfs', 'item'),    os.path.join(out_dir, 'item'))
    for excl in ['barrel', 'daisydaikon']:
        excl_path = os.path.join(out_dir, 'item', excl)
        if os.path.exists(excl_path):
            shutil.rmtree(excl_path)
            log(f"  Removed excluded item folder: {excl}")

    copytree(os.path.join('romfs', 'effect'),   os.path.join(out_dir, 'effect'))
    copytree(os.path.join('romfs', 'ui'),        os.path.join(out_dir, 'ui'))
    copytree(os.path.join('romfs', 'camera'),    os.path.join(out_dir, 'camera'))
    copytree(os.path.join('romfs', 'sound'),     os.path.join(out_dir, 'sound'))
    copytree(os.path.join('romfs', 'prebuilt'),  os.path.join(out_dir, 'prebuilt'))
    copytree(os.path.join('romfs', 'stream;'),   os.path.join(out_dir, 'stream;'))

    # Remove c0x sound files
    for root, dirs, files in os.walk(os.path.join(out_dir, 'sound'), topdown=False):
        for name in files:
            if 'c0' in name:
                os.remove(os.path.join(root, name))

    # Copy fighter files excluding costume dirs and config.json
    # (configs handled separately below)
    COSTUME_DIRS = {'c00', 'c01', 'c02', 'c03', 'c04', 'c05', 'c06', 'c07'}

    for fighter in os.listdir(os.path.join('romfs', 'fighter')):
        fighter_src = os.path.join('romfs', 'fighter', fighter)
        if not os.path.isdir(fighter_src) or fighter == 'common':
            continue
        for root, dirs, files in os.walk(fighter_src, topdown=True):
            dirs[:] = [d for d in dirs if d not in COSTUME_DIRS]
            for name in files:
                if name == 'config.json':
                    continue
                src_path = os.path.join(root, name)
                dst_path = src_path.replace(
                    'romfs',
                    os.path.join('releases-lite', 'ultimate', 'mods', 'Ultimate S Lite'),
                    1
                )
                os.makedirs(os.path.dirname(dst_path), exist_ok=True)
                shutil.copy2(src_path, dst_path)

    # Copy fighter config.json files only where the fighter has real content
    # in lite — check the fighter root dst exists, then copy all configs found
    # anywhere in that fighter's tree (creating subdirs like added/ as needed)
    log("  Copying fighter config.json files...")
    configs_copied = 0
    for fighter in os.listdir(os.path.join('romfs', 'fighter')):
        fighter_src = os.path.join('romfs', 'fighter', fighter)
        if not os.path.isdir(fighter_src) or fighter == 'common':
            continue
        fighter_dst = os.path.join(out_dir, 'fighter', fighter)
        if not os.path.exists(fighter_dst):
            log(f"    Skipped (no content): fighter/{fighter}")
            continue
        for root, dirs, files in os.walk(fighter_src, topdown=True):
            dirs[:] = [d for d in dirs if d not in COSTUME_DIRS]
            if 'config.json' in files:
                src_path = os.path.join(root, 'config.json')
                dst_path = src_path.replace(
                    'romfs',
                    os.path.join('releases-lite', 'ultimate', 'mods', 'Ultimate S Lite'),
                    1
                )
                os.makedirs(os.path.dirname(dst_path), exist_ok=True)
                shutil.copy2(src_path, dst_path)
                log(f"    Copied: {src_path}")
                configs_copied += 1
    log(f"  {configs_copied} fighter config.json file(s) copied")

    merge_configs(out_dir)

    with open(os.path.join(out_dir, 'version.txt'), 'w') as f:
        f.write(f"v.{version}-LITE")

    shutil.copy('readme-lite.txt', os.path.join('releases-lite', 'readme.txt'))
    shutil.copy('credits.txt',     os.path.join('releases-lite', 'credits.txt'))

    make_zip(out_dir, os.path.join('releases-lite', 'Ultimate S Lite.zip'))
    log("=== Lite build done ===")

# ---------------------------------------------------------------------------
# Diff build
# ---------------------------------------------------------------------------

BUFFER = 65536

def md5_file(path):
    h = hashlib.md5()
    with open(path, 'rb') as f:
        while True:
            data = f.read(BUFFER)
            if not data:
                break
            h.update(data)
    return h.hexdigest()

def diff(new_dir, old_dir, diff_dir_name, removed_files_list):
    log(f"  Diffing {diff_dir_name}: {old_dir} -> {new_dir}")
    old_files = []
    new_files = []

    for root, dirs, files in os.walk(old_dir):
        for name in files:
            old_files.append(os.path.join(root, name))

    for root, dirs, files in os.walk(new_dir):
        for name in files:
            new_files.append(os.path.join(root, name))

    changed = []
    for new_path in new_files:
        old_path = new_path.replace(new_dir, old_dir)
        if not os.path.isfile(old_path):
            log(f"    New file: {new_path}")
            changed.append(new_path)
            continue
        if md5_file(new_path) != md5_file(old_path):
            log(f"    Changed: {new_path}")
            changed.append(new_path)

    for old_path in old_files:
        new_path = old_path.replace(old_dir, new_dir)
        if not os.path.isfile(new_path):
            log(f"    Removed: {old_path}")
            removed_files_list.append(old_path)

    out_base = os.path.join('releases-diff', 'ultimate', 'mods', diff_dir_name)
    os.makedirs(out_base, exist_ok=True)

    for src in changed:
        relative = os.path.relpath(src, new_dir)
        dst = os.path.join(out_base, relative)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy(src, dst)

def build_diff(version, old_arcropolis_dir, old_stages_dir):
    log("=== Diff build ===")
    log(f"  Old Arcropolis: {old_arcropolis_dir}")
    log(f"  Old Stages:     {old_stages_dir}")

    empty_or_create('releases-diff')

    new_arcropolis = os.path.join('releases', 'ultimate', 'mods', 'Ultimate S Arcropolis')
    new_stages     = os.path.join('releases', 'ultimate', 'mods', 'Ultimate S Stages')

    removed_files = []
    diff(new_arcropolis, old_arcropolis_dir, 'Ultimate S Arcropolis', removed_files)

    if os.path.exists(new_stages) and os.path.exists(old_stages_dir):
        diff(new_stages, old_stages_dir, 'Ultimate S Stages', removed_files)
    else:
        log("  Skipping stages diff (one or both dirs missing)")

    # Write removed_files.txt with truncated paths
    removed_txt = os.path.join('releases-diff', 'removed_files.txt')
    with open(removed_txt, 'w') as f:
        for path in removed_files:
            norm = path.replace('\\', '/')
            marker = 'ultimate/mods/'
            idx = norm.lower().find(marker)
            line = norm[idx:] if idx != -1 else norm
            f.write(line + '\n')
    log(f"  Wrote {len(removed_files)} removed file(s) to removed_files.txt")

    # Bundle apply_diff.py alongside the diff
    apply_script = os.path.join('scripts', 'apply_diff.py')
    if os.path.exists(apply_script):
        shutil.copy(apply_script, os.path.join('releases-diff', 'click me to apply the diff.py'))
    else:
        log("  WARNING: apply_diff.py not found in scripts/, skipping")

    make_zip('releases-diff', os.path.join('releases-diff', 'ult_s_diff.zip'))
    log("=== Diff build done ===")

# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

if __name__ == '__main__':
    if len(sys.argv) < 3:
        print(__doc__)
        sys.exit(1)

    version = sys.argv[1]
    mode    = sys.argv[2].lower()

    log(f"build_release.py: version={version} mode={mode}")
    log(f"Working directory: {os.getcwd()}")

    if mode == 'full':
        build_full(version)
    elif mode == 'lite':
        build_lite(version)
    elif mode == 'diff':
        if len(sys.argv) < 5:
            print("diff mode requires <old_arcropolis_dir> <old_stages_dir>")
            sys.exit(1)
        build_diff(version, sys.argv[3], sys.argv[4])
    else:
        print(f"Unknown mode: {mode}")
        sys.exit(1)