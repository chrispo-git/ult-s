import os
import shutil
from zipfile import ZipFile
import sys
import subprocess

def log(msg):
    print(msg, flush=True)

try:
    inputs = (" ".join(sys.argv)).lower()
except IndexError:
    inputs = "invalid"

inputs = inputs.replace('build_char.py', "")
log(f"[build_char] Raw inputs: {inputs}")
inputs = inputs.replace("'", "")
inputs = inputs.replace('"', "")
inputs = inputs.replace('.', "")
inputs = inputs.replace('-', "")
inputs = inputs.replace('_', "")
inputs = inputs.replace(',', "")
inputs = inputs.replace('!', "")
inputs = inputs.replace('?', "")
inputs = inputs.replace(':', "")

inputs = inputs.replace("mii gunner", "miigunner")
inputs = inputs.replace("mii brawler", "miifighter")
inputs = inputs.replace("mii sword", "miiswordsman")
inputs = inputs.replace("bowser jr", "koopajr")
inputs = inputs.replace("dark pit", "pitb")
inputs = inputs.replace("dr mario", "mariod")
inputs = inputs.replace("dark samus", "samusd")

output_folder = inputs.split(" ")[1]
if output_folder == " ":
    output_folder = inputs.split(" ")[2]

log(f"[build_char] Parsed output_folder (pre-replace): {output_folder}")

replace = [
    ['bayonetta', 'bayo'],
    ['brave', 'hero'],
    ['buddy', 'banjo'],
    ['captain', 'falcon', 'cap'],
    ['chrom'],
    ['cloud'],
    ['daisy'],
    ['dedede', 'd3'],
    ['diddy'],
    ['dolly', 'terry'],
    ['donkey', 'dk'],
    ['duckhunt', 'duck'],
    ['edge', 'seph'],
    ['eflame', 'pyra'],
    ['elight', 'mythra'],
    ['gaogaen', 'incin'],
    ['falco'],
    ['fox'],
    ['ganon', 'ganondorf', 'dorf'],
    ['gamewatch', 'gnw', 'g&w'],
    ['gekkouga', 'greninja', 'gren'],
    ['ike'],
    ['inkling'],
    ['jack', 'joker'],
    ['kamui', 'corrin'],
    ['ken'],
    ['kirby'],
    ['koopa', 'bowser'],
    ['koopajr', 'jr'],
    ['krool'],
    ['link'],
    ['littlemac', 'mac'],
    ['lucario'],
    ['lucas'],
    ['lucina'],
    ['luigi'],
    ['mariod', 'dr', 'doc'],
    ['mario'],
    ['marth'],
    ['master', 'byleth'],
    ['metaknight', 'meta', 'mk'],
    ['mewtwo', 'mew2'],
    ['miifighter', 'brawler'],
    ['miigunner', 'gunner'],
    ['miiswordsman', 'sword'],
    ['murabito', 'villager', 'villy', 'toad'],
    ['nana'],
    ['ness'],
    ['packun', 'plant', 'piranha'],
    ['pacman', 'pac'],
    ['palutena', 'palu'],
    ['peach'],
    ['pfushigisou', 'ivy'],
    ['pichu'],
    ['pickel', 'steve'],
    ['pikachu', 'pika'],
    ['pikmin', 'olimar', 'alph', 'rayman'],
    ['pitb', 'dpit'],
    ['pit'],
    ['plizardon', 'charizard', 'zard'],
    ['popo'],
    ['purin', 'puff', 'jigglypuff'],
    ['pzenigame', 'squirtle'],
    ['reflet', 'robin'],
    ['richter'],
    ['ridley'],
    ['robot', 'rob'],
    ['rockman', 'megaman', 'mega'],
    ['rosetta', 'rosa', 'rosalina'],
    ['roy'],
    ['ryu'],
    ['samusd', 'dsamus'],
    ['samus'],
    ['sheik', 'shiek'],
    ['shizue', 'isa', 'isabelle'],
    ['shulk'],
    ['simon'],
    ['snake'],
    ['sonic'],
    ['szerosuit', 'zero', 'zss'],
    ['tantan', 'minmin', 'min'],
    ['toonlink', 'tink', 'toon'],
    ['trail', 'sora'],
    ['wario'],
    ['wiifit', 'wii', 'wft'],
    ['wolf'],
    ['yoshi'],
    ['younglink', 'yink', 'young'],
    ['zelda']
]

has_replace = False
for i in replace:
    for f in i:
        if f in output_folder:
            output_folder = i[0]
            has_replace = True
            log(f"[build_char] Matched alias '{f}' -> '{output_folder}'")
            break
    if has_replace:
        break

log(f"[build_char] Final output_folder: {output_folder}")

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

log("[build_char] Starting cargo skyline build...")
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
log(f"[build_char] cargo skyline build exited with code {result_code}")
if result_code != 0:
    raise Exception(f"cargo skyline build failed with return code {result_code}")

os.chdir('../')
log(f"[build_char] Now in: {os.getcwd()}")
new = r"releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)"

def empty_folder():
    folder = r'releases'
    log(f"[build_char] Emptying folder: {folder}")
    for filename in os.listdir(folder):
        file_path = os.path.join(folder, filename)
        try:
            if os.path.isfile(file_path) or os.path.islink(file_path):
                os.unlink(file_path)
            elif os.path.isdir(file_path):
                shutil.rmtree(file_path)
        except Exception as e:
            log(f'[build_char] Failed to delete {file_path}. Reason: {e}')

def get_all_file_paths(directory):
    file_paths = []
    for root, directories, files in os.walk(directory):
        for filename in files:
            filepath = os.path.join(root, filename)
            file_paths.append(filepath)
    return file_paths

log("[build_char] Finished building... now compiling Romfs")

if os.path.exists(r'target'):
    log("[build_char] Found target/ directory")
    os.chdir(r'target')
    log(f"[build_char] Contents of target/: {os.listdir()}")
    if os.path.exists(r'aarch64-skyline-switch'):
        os.chdir(r'aarch64-skyline-switch')
        log(f"[build_char] Contents of aarch64-skyline-switch/: {os.listdir()}")
        if os.path.exists(r'release'):
            os.chdir(r'release')
            old = os.path.join(os.path.abspath(os.getcwd()), r'libplugin.nro')
            log(f"[build_char] Found libplugin.nro at: {old}")
            os.chdir('../')
            os.chdir('../')
            os.chdir('../')
            log(f"[build_char] Back in: {os.getcwd()}")
            if os.path.exists(r'releases'):
                log("[build_char] Emptying existing releases/ folder")
                empty_folder()
            if not os.path.exists(new):
                log(f"[build_char] Creating output directory: {new}")
                os.makedirs(new)
            log(f"[build_char] Moving {old} -> {new}")
            shutil.move(old, new)
            if not os.path.exists(r'releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)'):
                os.makedirs(r'releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)')
            log("[build_char] Renaming libplugin.nro -> plugin.nro")
            shutil.move(os.path.join(new, r'libplugin.nro'), os.path.join(new, r'plugin.nro'))
            if os.path.exists(r'romfs'):
                log("[build_char] Starting romfs copy")
                copytree(r'romfs/fighter/common', r'releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)/fighter/common')
                copytree(r'romfs/prebuilt', r'releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)/prebuilt')
                shutil.copy(r'romfs/config.json', r'releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)/config.json')
                char_romfs = os.path.join(r'romfs/fighter', output_folder)
                char_out = os.path.join(r'releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)/fighter', output_folder)
                if os.path.exists(char_romfs):
                    log(f"[build_char] Copying character romfs: {char_romfs} -> {char_out}")
                    copytree(char_romfs, char_out)
                else:
                    log(f"[build_char] WARNING: No romfs folder found for '{output_folder}' at {char_romfs}")
                log("[build_char] Copying from romfs finished, now zipping")
            else:
                log("[build_char] ERROR: No romfs folder! Please check your install")

            zip_path = r'releases/Ultimate S Arcropolis (plugin, singular character).zip'
            if os.path.exists(zip_path):
                log(f"[build_char] Removing old zip: {zip_path}")
                os.remove(zip_path)
            log("[build_char] Gathering files to zip...")
            file_paths = get_all_file_paths(new)
            log(f"[build_char] Zipping {len(file_paths)} files into {zip_path}")
            with ZipFile(zip_path, 'w') as zip:
                for file in file_paths:
                    log(f"[build_char]   Adding: {file}")
                    zip.write(file)
            log("[build_char] Done!")
        else:
            log('[build_char] ERROR: release/ does not exist inside aarch64-skyline-switch/')
    else:
        log('[build_char] ERROR: aarch64-skyline-switch does not exist inside target/')
else:
    log('[build_char] ERROR: target/ does not exist')