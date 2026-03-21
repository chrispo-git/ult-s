import os
import shutil
from zipfile import ZipFile
import sys
import subprocess

def log(msg):
    print(msg, flush=True)

def copytree(src, dst, symlinks=False, ignore=None):
    log(f"[copytree] Copying {src} -> {dst}")
    for item in os.listdir(src):
        s = os.path.join(src, item)
        d = os.path.join(dst, item)
        if os.path.isdir(s):
            shutil.copytree(s, d, symlinks, ignore)
        else:
            shutil.copy2(s, d)
    log(f"[copytree] Done copying {src}")

log("[build_small] Starting cargo skyline build...")
process = subprocess.Popen(
    'cargo skyline build --release --features="main_nro"',
    shell=True,
    stdout=subprocess.PIPE,
    stderr=subprocess.STDOUT,
    text=True
)
for line in process.stdout:
    print(line, end='', flush=True)
process.wait()
result_code = process.returncode
log(f"[build_small] cargo skyline build exited with code {result_code}")
if result_code != 0:
    raise Exception(f"cargo skyline build failed with return code {result_code}")

os.chdir('../')
log(f"[build_small] Now in: {os.getcwd()}")
new = r"releases/ultimate/mods/Ultimate S Arcropolis (plugin and common files only)"

def empty_folder():
    folder = r'releases'
    log(f"[build_small] Emptying folder: {folder}")
    for filename in os.listdir(folder):
        file_path = os.path.join(folder, filename)
        try:
            if os.path.isfile(file_path) or os.path.islink(file_path):
                os.unlink(file_path)
            elif os.path.isdir(file_path):
                shutil.rmtree(file_path)
        except Exception as e:
            log(f'[build_small] Failed to delete {file_path}. Reason: {e}')

def get_all_file_paths(directory):
    file_paths = []
    for root, directories, files in os.walk(directory):
        for filename in files:
            filepath = os.path.join(root, filename)
            file_paths.append(filepath)
    return file_paths

log("[build_small] Finished building... now compiling Romfs")

if os.path.exists(r'target'):
    log("[build_small] Found target/ directory")
    os.chdir(r'target')
    log(f"[build_small] Contents of target/: {os.listdir()}")
    if os.path.exists(r'aarch64-skyline-switch'):
        os.chdir(r'aarch64-skyline-switch')
        log(f"[build_small] Contents of aarch64-skyline-switch/: {os.listdir()}")
        if os.path.exists(r'release'):
            os.chdir(r'release')
            old = os.path.join(os.path.abspath(os.getcwd()), r'libplugin.nro')
            log(f"[build_small] Found libplugin.nro at: {old}")
            os.chdir('../')
            os.chdir('../')
            os.chdir('../')
            log(f"[build_small] Back in: {os.getcwd()}")
            if os.path.exists(r'releases'):
                log("[build_small] Emptying existing releases/ folder")
                empty_folder()
            if not os.path.exists(new):
                log(f"[build_small] Creating output directory: {new}")
                os.makedirs(new)
            log(f"[build_small] Moving {old} -> {new}")
            shutil.move(old, new)
            if not os.path.exists(r'releases/ultimate/mods/Ultimate S Arcropolis (plugin and common files only)'):
                os.makedirs(r'releases/ultimate/mods/Ultimate S Arcropolis (plugin and common files only)')
            log("[build_small] Renaming libplugin.nro -> plugin.nro")
            shutil.move(os.path.join(new, r'libplugin.nro'), os.path.join(new, r'plugin.nro'))
            if os.path.exists(r'romfs'):
                log("[build_small] Starting romfs copy")
                copytree(r'romfs/fighter/common', r'releases/ultimate/mods/Ultimate S Arcropolis (plugin and common files only)/fighter/common')
                copytree(r'romfs/prebuilt', r'releases/ultimate/mods/Ultimate S Arcropolis (plugin and common files only)/prebuilt')
                log("[build_small] Copying from romfs finished, now zipping")
            else:
                log("[build_small] ERROR: No romfs folder! Please check your install")

            zip_path = r'releases/Ultimate S Arcropolis (plugin and common files only).zip'
            if os.path.exists(zip_path):
                log(f"[build_small] Removing old zip: {zip_path}")
                os.remove(zip_path)
            log("[build_small] Gathering files to zip...")
            file_paths = get_all_file_paths(new)
            log(f"[build_small] Zipping {len(file_paths)} files into {zip_path}")
            with ZipFile(zip_path, 'w') as zip:
                for file in file_paths:
                    log(f"[build_small]   Adding: {file}")
                    zip.write(file)
            log("[build_small] Done!")
        else:
            log('[build_small] ERROR: release/ does not exist inside aarch64-skyline-switch/')
    else:
        log('[build_small] ERROR: aarch64-skyline-switch does not exist inside target/')
else:
    log('[build_small] ERROR: target/ does not exist')