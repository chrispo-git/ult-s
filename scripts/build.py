import os
import shutil
from zipfile import ZipFile

stream = os.popen('cargo skyline build')
output = stream.read()
output
os.chdir('../')
print(os.getcwd())
old = r"target\aarch64-skyline-switch\debug\libplugin.nro"
new = r"releases"
old_rename = r"libplugin.nro"
rename = r"plugin.nro"
file_name = "plugin.zip"

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

print(os.listdir())

if os.path.exists(r'target'):
    os.chdir(r'target')
    print(os.listdir())
    if os.path.exists(r'aarch64-skyline-switch'):
        os.chdir(r'aarch64-skyline-switch')
        print(os.listdir())
        if os.path.exists(r'debug'):
            os.chdir(r'debug')
            print(os.listdir())
            old = os.path.join(os.path.abspath(os.getcwd()), r'libplugin.nro')
            os.chdir('../')
            os.chdir('../')
            os.chdir('../')
            print(os.getcwd())
            if os.path.exists(new) == False:
                os.mkdir(new)
            print(old)
            shutil.move(old, new)
            print(os.listdir())
            shutil.move(os.path.join(new, r'libplugin.nro'), os.path.join(new, r'plugin.nro'))
            if os.path.exists(r'releases/plugin.zip'):
                os.remove(r'releases/plugin.zip')
            file_paths = get_all_file_paths(new)
            with ZipFile(r'releases/plugin.zip','w') as zip:
                for file in file_paths:
                    zip.write(file)
            print("done")
        else:
            print('aarch64-skyline-switch does not exist')
    else:
        print('aarch64-skyline-switch does not exist')
else:
    print('target does not exist')

"""
if os.path.exists(old) == False:
    print("issue")

if os.path.exists(new) == False:
    os.mkdir(new)

shutil.move(old, new)


os.chdir(r'releases')
if os.path.exists('plugin.nro'):
    stream = os.popen('del plugin.nro')
    output = stream.read()
    output
stream = os.popen('rename libplugin.nro plugin.nro')
output = stream.read()
output
"""