import os
import shutil
from zipfile import ZipFile

def copytree(src, dst, symlinks=False, ignore=None):
    for item in os.listdir(src):
        s = os.path.join(src, item)
        d = os.path.join(dst, item)
        if os.path.isdir(s):
            shutil.copytree(s, d, symlinks, ignore)
        else:
            shutil.copy2(s, d)

stream = os.popen('cargo skyline build --release')
output = stream.read()
output
os.chdir('../')
print(os.getcwd())
old = r"target\aarch64-skyline-switch\release\libplugin.nro"
new = r"releases/ultimate/mods/Ultimate S Arcropolis (plugin and common files only)"
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
            if os.path.exists(r'releases/ultimate/mods/Ultimate S Arcropolis (plugin and common files only)') == False:
                os.makedirs(r'releases/ultimate/mods/Ultimate S Arcropolis (plugin and common files only)')
            shutil.move(os.path.join(new, r'libplugin.nro'), os.path.join(new, r'plugin.nro'))
            if os.path.exists(r'romfs'):
                print("Starting copy")
                copytree(r'romfs/fighter/common', r'releases/ultimate/mods/Ultimate S Arcropolis (plugin and common files only)/fighter/common')
                print("Copying from romfs finished, now zipping")
            else:
                print("Error! No romfs folder! Please check your install")
            if os.path.exists(r'releases/Ultimate S Arcropolis (plugin and common files only).zip'):
                os.remove(r'releases/Ultimate S Arcropolis (plugin and common files only).zip')
            file_paths = get_all_file_paths(new)
            with ZipFile(r'releases/Ultimate S Arcropolis (plugin and common files only).zip','w') as zip:
                for file in file_paths:
                    zip.write(file)
            print("Done!")
        else:
            print('aarch64-skyline-switch does not exist')
    else:
        print('aarch64-skyline-switch does not exist')
else:
    print('target does not exist')
