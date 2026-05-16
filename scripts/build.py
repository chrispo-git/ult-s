import os
import shutil
import sys
from zipfile import ZipFile
import subprocess

from merge_config import merge_configs

def log(msg):
    print(msg, flush=True)

try:
    inputs = ("".join(sys.argv)).lower()
    inputs = inputs.replace('build.py', "")
    inputs = inputs.replace('\n', "")
    log(f"[build] Inputs: {inputs}")
except IndexError:
    raise Exception("No version inputted!")

inputs = inputs.replace('build.py ', "")

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

log("[build] Starting cargo skyline build...")
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
log(f"[build] cargo skyline build exited with code {result_code}")
if result_code != 0:
    raise Exception(f"cargo skyline build failed with return code {result_code}")

log(f"[build] Current directory: {os.getcwd()}")
old = r"target\aarch64-skyline-switch\release\libplugin.nro"
new = r"releases/ultimate/mods/Ultimate S Arcropolis"
old_rename = r"libplugin.nro"
rename = r"plugin.nro"

def empty_folder():
    folder = r'releases'
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

def get_all_file_paths(directory):
    file_paths = []
    for root, directories, files in os.walk(directory):
        for filename in files:
            filepath = os.path.join(root, filename)
            file_paths.append(filepath)
    return file_paths

log("[build] Finished building... now compiling Romfs")
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
            os.chdir('../')
            os.chdir('../')
            os.chdir('../')
            log(f"[build] Back in: {os.getcwd()}")
            if os.path.exists(r'releases'):
                log("[build] Emptying existing releases/ folder")
                empty_folder()
            if not os.path.exists(new):
                log(f"[build] Creating output directory: {new}")
                os.makedirs(new)
            log(f"[build] Moving {old} -> {new}")
            shutil.move(old, new)
            if not os.path.exists(r'releases/ultimate/mods/Ultimate S Arcropolis'):
                os.makedirs(r'releases/ultimate/mods/Ultimate S Arcropolis')
            log("[build] Renaming libplugin.nro -> plugin.nro")
            shutil.move(os.path.join(new, r'libplugin.nro'), os.path.join(new, r'plugin.nro'))
            if os.path.exists(r'romfs'):
                log("[build] Starting romfs copy")
                copytree(r'romfs', r'releases/ultimate/mods/Ultimate S Arcropolis')
                merge_configs(r'releases/ultimate/mods/Ultimate S Arcropolis')
                log("[build] Copying from romfs finished, now zipping")
            else:
                log("[build] ERROR: No romfs folder! Please check your install")

            log("[build] Writing version.txt")
            f = open(r'releases/ultimate/mods/Ultimate S Arcropolis/version.txt', "w")
            f.write(f"v.{inputs}")
            f.close()
            log("[build] Copying readme and credits")
            shutil.copy(r'readme.txt', r'releases/readme.txt')
            shutil.copy(r'credits.txt', r'releases/credits.txt')

            zip_path = r'releases/Ultimate S Arcropolis.zip'
            if os.path.exists(zip_path):
                log(f"[build] Removing old zip: {zip_path}")
                os.remove(zip_path)
            log("[build] Gathering files to zip...")
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