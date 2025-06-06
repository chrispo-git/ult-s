import os
import shutil
from zipfile import ZipFile
import sys
import subprocess

try:
    inputs = ("".join(sys.argv)).lower()
    inputs = inputs.replace('build_lite.py', "")
    inputs = inputs.replace('\n', "")
    print(inputs)
except IndexError:
    raise Exception("No version inputted!")

inputs = inputs.replace('build_lite.py ', "")

def copytree(src, dst, symlinks=False, ignore=None):
    for item in os.listdir(src):
        s = os.path.join(src, item)
        d = os.path.join(dst, item)
        if os.path.isdir(s):
            shutil.copytree(s, d, symlinks, ignore)
        else:
            shutil.copy2(s, d)

subprocess.run('cargo skyline build --release --features="main_nro"', shell=True)
os.chdir('../')
print(os.getcwd())
old = r"target\aarch64-skyline-switch\release\libplugin.nro"
new = r"releases/ultimate/mods/Ultimate S Lite"
old_rename = r"libplugin.nro"
rename = r"plugin.nro"

def empty_folder():
    folder = r'releases'
    for filename in os.listdir(folder):
        file_path = os.path.join(folder, filename)
        try:
            if os.path.isfile(file_path) or os.path.islink(file_path):
                os.unlink(file_path)
            elif os.path.isdir(file_path):
                shutil.rmtree(file_path)
        except Exception as e:
            print('Failed to delete %s. Reason: %s' % (file_path, e))

def get_all_file_paths(directory):
  
    # initializing empty file paths list
    file_paths = []
  
    # crawling through directory and subdirectories
    for root, directories, files in os.walk(directory):
        for filename in files:
            # join the two strings in order to form the full filepath.
            filepath = os.path.join(root, filename)
            file_paths.append(filepath)
  
    # returning all file paths
    return file_paths       

print("Finished Building... now compiling Romfs")

if os.path.exists(r'target'):
    os.chdir(r'target')
    print(os.listdir())
    if os.path.exists(r'aarch64-skyline-switch'):
        os.chdir(r'aarch64-skyline-switch')
        print(os.listdir())
        if os.path.exists(r'release'):
            os.chdir(r'release')
            #print(os.listdir())
            old = os.path.join(os.path.abspath(os.getcwd()), r'libplugin.nro')
            os.chdir('../')
            os.chdir('../')
            os.chdir('../')
            print(os.getcwd())
            if os.path.exists(r'releases'):
                empty_folder()
            if os.path.exists(new) == False:
                os.makedirs(new)
            #print(old)
            shutil.move(old, new)
            #print(os.listdir())
            if os.path.exists(r'releases/ultimate/mods/Ultimate S Lite') == False:
                os.makedirs(r'releases/ultimate/mods/Ultimate S Lite')
            shutil.move(os.path.join(new, r'libplugin.nro'), os.path.join(new, r'plugin.nro'))
            if os.path.exists(r'romfs'):
                print("Starting copy")
                copytree(r'romfs/fighter/common', r'releases/ultimate/mods/Ultimate S Lite/fighter/common')
                os.remove(r'releases/ultimate/mods/Ultimate S Lite/fighter/common/param/fighter_param.prcxml')
                copytree(r'romfs/effect', r'releases/ultimate/mods/Ultimate S Lite/effect')
                copytree(r'romfs/ui', r'releases/ultimate/mods/Ultimate S Lite/ui')
                copytree(r'romfs/camera', r'releases/ultimate/mods/Ultimate S Lite/camera')
                copytree(r'romfs/sound', r'releases/ultimate/mods/Ultimate S Lite/sound')
                copytree(r'romfs/prebuilt', r'releases/ultimate/mods/Ultimate S Lite/prebuilt')
                copytree(r'romfs/stream;', r'releases/ultimate/mods/Ultimate S Lite/stream;')
                shutil.copy(r'romfs/config_param.toml', r'releases/ultimate/mods/Ultimate S Lite/config_param.toml')
                f = open(r'releases/ultimate/mods/Ultimate S Lite/config_param.toml')
                config_param = f.readlines()
                f.close()
                cut = config_param.index("# CUSTOM CHARACTERS\n")
                cut_config_param = config_param[cut:]
                f = open(r'releases/ultimate/mods/Ultimate S Lite/config_param.toml', "w")
                for i in cut_config_param:
                    f.write(i)
                f.close()
                shutil.copy(r'romfs/config.json', r'releases/ultimate/mods/Ultimate S Lite/config.json')
                shutil.copy(r'romfs/victory.toml', r'releases/ultimate/mods/Ultimate S Lite/victory.toml')
                for i in os.listdir(r'romfs/fighter'):
                    for root, dirs, files in os.walk(f"romfs/fighter/{i}/", topdown=False):
                        for name in dirs:
                            if "c1" in name:
                                path = os.path.join(root, name)
                                new_path = path.replace("romfs", "releases/ultimate/mods/Ultimate S Lite")
                                if os.path.exists(new_path) == False:
                                    os.makedirs(new_path)
                                copytree(path, new_path)

                #Version Text
                
                print("Copying from romfs finished, now zipping")
            else:
                print("Error! No romfs folder! Please check your install")
            

            f = open(r'releases/ultimate/mods/Ultimate S Lite/version.txt',"w")
            f.write(f"v.{inputs}-LITE")
            f.close()
            shutil.copy(r'readme-lite.txt', r'releases/readme.txt')
            shutil.copy(r'credits.txt', r'releases/credits.txt')
            if os.path.exists(r'releases/Ultimate S Lite.zip'):
                os.remove(r'releases/Ultimate S Lite.zip')
            file_paths = get_all_file_paths(new)
            with ZipFile(r'releases/Ultimate S Lite.zip','w') as zip:
                for file in file_paths:
                    zip.write(file)
            print("Done!")
        else:
            print('aarch64-skyline-switch does not exist')
    else:
        print('aarch64-skyline-switch does not exist')
else:
    print('target does not exist')
