import math
import os
import sys
from string import ascii_lowercase
import tkinter as tk
from tkinter import filedialog
import hashlib
from zipfile import ZipFile
import shutil


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

BUFFER = 65536

os.chdir('../')
home_dir = os.getcwd()

new_dir = filedialog.askdirectory(title="Select the new version of Ultimate S Arcropolis folder")
old_dir = filedialog.askdirectory(title="Select the old version of Ultimate S Arcropolis folder")
new_stages = filedialog.askdirectory(title="Select the new version of Ultimate S Stages folder")
old_stages = filedialog.askdirectory(title="Select the old version of Ultimate S Stages folder")
#old_dir = r"D:\Smash Ultimate Mod Workspace\Ultimate S Packed\diff_test2"
#new_dir = r"D:\Smash Ultimate Mod Workspace\Ultimate S Packed\diff_test"

removed_files = []

def diff(home_dir, new_dir, old_dir, diff_dir):
    global removed_files
    old_dir_filelist = []
    new_dir_filelist = []
    os.chdir(old_dir)
    for (root,dirs,files) in os.walk("."):
        for name in files:
            old_dir_filelist.append(os.path.join(root, name).replace("./",f"{old_dir}/"))
    os.chdir(new_dir)
    for (root,dirs,files) in os.walk(".", topdown=False):
        for name in files:
            new_dir_filelist.append(os.path.join(root, name).replace("./",f"{new_dir}/"))
    os.chdir(home_dir)

    print("Files Read")

    changed_files = []
    for i in new_dir_filelist:
        file_in_old = i.replace(new_dir, old_dir)
        if not os.path.isfile(file_in_old) and os.path.isfile(i):
            #print(f"{i} not in old version. {file_in_old} invalid")
            changed_files.append(i)
            continue

        print(f"reading {i}")
        new_md5 = hashlib.md5()
        old_md5 = hashlib.md5()
        with open(i, 'rb') as f:
            while True:
                data = f.read(BUFFER)
                if not data:
                    break
                new_md5.update(data)
        with open(file_in_old, 'rb') as f:
            while True:
                data = f.read(BUFFER)
                if not data:
                    break
                old_md5.update(data)
        md5_1 = old_md5.hexdigest()
        md5_2 = new_md5.hexdigest()
        if md5_1 != md5_2:
            print(f"{i} has changed!")
            changed_files.append(i)

    print("Checking for removed files")
    for i in old_dir_filelist:
        file_in_new = i.replace(old_dir, new_dir)
        if not os.path.isfile(file_in_new) and os.path.isfile(i):
            #print(f"{i} not in old version. {file_in_old} invalid")
            print(f"{i} is removed!")
            removed_files.append(i)

    #print(changed_files)
    if os.path.exists(f'releases/ultimate/mods/{diff_dir}') == False:
        os.makedirs(f'releases/ultimate/mods/{diff_dir}')
    for i in changed_files:
        source = i
        destination = i.replace(new_dir, f"releases\\ultimate\\mods\\{diff_dir}")
        source = source.replace("\\","/")
        destination = destination.replace("\\","/")
        dir_only = destination.split("/")
        dir_only.pop(-1)
        dir_only = "/".join(dir_only)
        if os.path.exists(dir_only) == False:
            os.makedirs(dir_only)
        print(dir_only)
        print(source)
        print(destination)
        shutil.copy(source, destination)


empty_folder()
diff(home_dir, new_dir, old_dir, "Ultimate S Arcropolis")
diff(home_dir, new_stages, old_stages, "Ultimate S Stages")

f = open("releases/removed_files.txt", "w")
for i in removed_files:
    path = i.replace("\n","")
    f.write(f"{path}\n")
f.close()

all_files = get_all_file_paths("releases")
with ZipFile(r'releases/ult_s_diff.zip','w') as zip:
    for file in all_files:
        zip.write(file)

print("Done!")