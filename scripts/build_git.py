import os
import shutil
import sys
import json
from zipfile import ZipFile

def log(msg):
    print(msg, flush=True)

try:
    inputs = ("".join(sys.argv)).lower()
    inputs = inputs.replace('build.py', "").replace('\n', "").strip()
    log(f"[build] Version input: {inputs}")
except IndexError:
    raise Exception("No version inputted!")

# Load changed files from the JSON written by the workflow
changed_files = []
changed_files_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'changed_files.json')
if os.path.exists(changed_files_path):
    with open(changed_files_path, 'r') as f:
        changed_files = json.load(f)
    log(f"[build] Loaded {len(changed_files)} changed files from PR")
    for cf in changed_files:
        log(f"[build]   {cf}")
else:
    log("[build] WARNING: changed_files.json not found, will copy all romfs files")

def is_changed(src_path):
    """Return True if this file was changed in the PR, or if we have no changed files list."""
    if not changed_files:
        return True
    # Normalize to forward slashes for comparison
    norm = src_path.replace('\\', '/')
    for cf in changed_files:
        cf_norm = cf.replace('\\', '/')
        if cf_norm.endswith(norm) or norm.endswith(cf_norm) or cf_norm == norm:
            return True
    return False

def copytree_changed(src, dst, symlinks=False, ignore=None):
    log(f"[copytree] Scanning {src} -> {dst}")
    copied = 0
    for item in os.listdir(src):
        s = os.path.join(src, item)
        d = os.path.join(dst, item)
        if os.path.isdir(s):
            copytree_changed(s, d, symlinks, ignore)
        else:
            if is_changed(s):
                os.makedirs(os.path.dirname(d), exist_ok=True)
                shutil.copy2(s, d)
                log(f"[copytree]   Copied: {s}")
                copied += 1
            else:
                log(f"[copytree]   Skipped (unchanged): {s}")
    log(f"[copytree] Done — {copied} files copied from {src}")

def get_all_file_paths(directory):
    file_paths = []
    for root, directories, files in os.walk(directory):
        for filename in files:
            file_paths.append(os.path.join(root, filename))
    return file_paths

def empty_folder(folder):
    log(f"[build] Emptying folder: {folder}")
    for filename in os.listdir(folder):
        file_path = os.path.join(folder, filename)
        try:
            if os.path.isfile(file_path) or os.path.islink(file_path):
                os.unlink(file_path)
            elif os.path.isdir(file_path):
                shutil.rmtree(file_path)
        except Exception as e:
            log(f'[build] Failed to delete {file_path}. Reason: {e}')

new = r"releases/ultimate/mods/Ultimate S Arcropolis"

log("[build] Finished building... now packaging Romfs")
log(f"[build] Changing directory up one level from: {os.getcwd()}")
os.chdir("..")
log(f"[build] Now in: {os.getcwd()}")

if os.path.exists(r'target'):
    log("[build] Found target/ directory")
    os.chdir(r'target')
    log(f"[build] Contents of target/: {os.listdir()}")
    if os.path.exists(r'aarch64-skyline-switch'):
        os.chdir(r'aarch64-skyline-switch')
        log(f"[build] Contents of aarch64-skyline-switch/: {os.listdir()}")
        if os.path.exists(r'release'):
            os.chdir(r'release')
            old = os.path.join(os.path.abspath(os.getcwd()), r'libplugin.nro')
            log(f"[build] Found libplugin.nro at: {old}")
            os.chdir('../../../')
            log(f"[build] Back in: {os.getcwd()}")

            if os.path.exists(r'releases'):
                empty_folder(r'releases')
            if not os.path.exists(new):
                log(f"[build] Creating output directory: {new}")
                os.makedirs(new)

            log(f"[build] Moving {old} -> {new}")
            shutil.move(old, new)
            log("[build] Renaming libplugin.nro -> plugin.nro")
            shutil.move(os.path.join(new, r'libplugin.nro'), os.path.join(new, r'plugin.nro'))

            if os.path.exists(r'romfs'):
                log("[build] Starting romfs copy (changed files only)")
                copytree_changed(r'romfs', r'releases/ultimate/mods/Ultimate S Arcropolis')
                log("[build] Romfs copy finished")
            else:
                log("[build] ERROR: No romfs folder! Please check your install")

            log("[build] Writing version.txt")
            with open(r'releases/ultimate/mods/Ultimate S Arcropolis/version.txt', "w") as f:
                f.write(f"v.{inputs}")

            log("[build] Copying readme and credits")
            shutil.copy(r'readme.txt', r'releases/readme.txt')
            shutil.copy(r'credits.txt', r'releases/credits.txt')
            copytree_changed(r'ultimate-s-setup', r'releases')

            zip_path = r'releases/Ultimate S Arcropolis.zip'
            if os.path.exists(zip_path):
                os.remove(zip_path)
            file_paths = get_all_file_paths(new)
            log(f"[build] Zipping {len(file_paths)} files into {zip_path}")
            with ZipFile(zip_path, 'w') as zip:
                for file in file_paths:
                    log(f"[build]   Adding: {file}")
                    zip.write(file)
            log("[build] Done!")
        else:
            log('[build] ERROR: release/ does not exist inside aarch64-skyline-switch/')
    else:
        log('[build] ERROR: aarch64-skyline-switch does not exist inside target/')
else:
    log('[build] ERROR: target/ does not exist')