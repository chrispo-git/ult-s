import os
import sys
import shutil
from zipfile import ZipFile
import subprocess

needed_folders = ["util", "common", "controls", "cpu"]
all_folders = []
in_ = []
out_ = []

try:
    os.chdir('../')
    basepath  = r"src"
    for i in os.listdir(basepath):
        path = os.path.join(basepath, i)
        if os.path.isdir(path) and i not in ["template"]:
            all_folders.append(i)
    os.chdir("scripts")

    params = sys.argv

    if len(params) > 1:
        for i in params:
            if i == "all":
                needed_folders= all_folders
                print(f"all included")
                break
            if i in all_folders:
                needed_folders.append(i)
                print(f"{i} included")

    os.chdir('../')
    shutil.copyfile(r"src/lib.rs", r"src/og_lib.rs")

    with open(r"src/lib.rs") as f:
        in_ = f.readlines()
    f.close()

    with open(r"src/lib.rs", 'w') as f:
        for i in range(0,216):
            f.write(in_[i])
        for x in needed_folders:
            f.write(f"mod {x};\n")
        for i in range(330,416):
            f.write(in_[i])
        for x in needed_folders:
            f.write(f"	{x}::install();\n")
        for i in range(864,878):
            f.write(in_[i])
    f.close()
    os.chdir("scripts")







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
    new = r"releases/ultimate/mods/Ultimate S Arcropolis (dev)"
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

    print("Finished Building!")

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
                if os.path.exists(r'releases/ultimate/mods/Ultimate S Arcropolis (dev)') == False:
                    os.makedirs(r'releases/ultimate/mods/Ultimate S Arcropolis (dev)')
                shutil.move(os.path.join(new, r'libplugin.nro'), os.path.join(new, r'plugin.nro'))
                print("Done!")
            else:
                print('aarch64-skyline-switch does not exist')
        else:
            print('aarch64-skyline-switch does not exist')
    else:
        print('target does not exist')

    os.remove(r"src/lib.rs")
    shutil.move(r"src/og_lib.rs", r"src/lib.rs")
except Exception as e:
    print("Stopped building! Reason:" + str(e))