import os
import shutil
import sys
from pathlib import Path
import re
import zipfile
import requests
from github import Github
from ftplib import FTP
import tkinter as tk
from tkinter import filedialog
from packaging.version import Version
# If you're running this just as a python file, you're gonna need to ensure you have the dependencies imported above:
# pip install github
# pip install packaging
# pip install requests

included = [
    "bayonetta", "brave", "buddy",
    "captain", "chrom", "cloud",
    "daisy", "dedede", "demon", "diddy", "dolly", "donkey", "duckhunt",
    "edge", "element",
    "falco", "fox",
    "gamewatch", "ganon", "gaogaen", "gekkouga", 
    "ike", "inkling", 
    "jack",
    "kamui", "ken", "kirby", "koopa", "koopajr", "krool",
    "link", "littlemac", "lucario", "lucas", "lucina", "luigi",
    "mario", "mariod", "marth", "master", "metaknight", "mewtwo", "miifighter", "miigunner", "miiswordsman", "murabito",
    "ness", 
    "packun", "pacman", "palutena", "peach", "pikachu", "pichu", "pit", "pitb", 'pikmin', "popo", "ptrainer", "purin",
    "reflet", "richter", "ridley", "robot", "rockman", "rosetta", "roy", "ryu",
    "samus", "samusd", "sheik", "shizue", "shulk", "simon", "snake", "szerosuit", "sonic",
    "tantan", "toonlink", "trail", "pickel",
    "wario", "wiifit", "wolf", "younglink", "yoshi", 
    "zelda"
]

excluded = [

]

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
    ['element', 'mythra', 'pyra', 'aegis', 'pythra', 'pymy', 'pam'],
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
    ['murabito', 'villager', 'villy'],
    ['nana'],
    ['ness'],
    ['packun', 'plant', 'piranha'],
    ['pacman', 'pac'],
    ['palutena', 'palu'],
    ['peach'],
    ['ptrainer', 'ivy', 'charizard', 'squirtle', 'fushigisou', 'zenigame', 'lizardon', 'pt', "pokemon", 'pokémon'],
    ['pichu'],
    ['pickel', 'steve'],
    ['pikachu', 'pika'],
    ['pikmin', 'olimar', 'alph'],
    ['pitb', 'dpit'],
    ['pit'],
    ['popo'],
    ['purin', 'puff', 'jigglypuff'],
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
    ['szerosuit', 'zero'],
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
def run_lite():
    

    #next = ""
    #while next != "end":
        #next = input("exclude character:")
        #if next in included:
            #excluded.append(next)
            #included.remove(next)
    if len(excluded) >= 1:
        os.remove("ultimate/mods/Ultimate S Arcropolis/ui/layout/menu/title/title/layout.arc")
        shutil.copy("resources/layout.arc","ultimate/mods/Ultimate S Arcropolis/ui/layout/menu/title/title/layout.arc")
        f = open("ultimate/mods/Ultimate S Arcropolis/version.txt")
        new_word = f.readline()
        f.close()
        f = open("ultimate/mods/Ultimate S Arcropolis/version.txt", "w")
        f.write(f"{new_word}.LITE")
        f.close()
    else:
        os.remove("ultimate/mods/Ultimate S Arcropolis/ui/layout/menu/title/title/layout.arc")
        shutil.copy("resources/og_layout.arc","ultimate/mods/Ultimate S Arcropolis/ui/layout/menu/title/title/layout.arc")
        f = open("ultimate/mods/Ultimate S Arcropolis/version.txt")
        new_word = f.readline()
        f.close()
        f = open("ultimate/mods/Ultimate S Arcropolis/version.txt", "w")
        f.write(new_word.replace(".LITE",""))
        f.close()
    # Cuts out their fighter params
    f = open("resources/fighter_param.prcxml")
    fighter_param = f.readlines()
    f.close()

    deleting = False
    for i in range(0,len(fighter_param)):
        if "<!--" in fighter_param[i]:
            to_continue = False
            for f in excluded:
                if f == "element":
                    if f"EFLAME" in fighter_param[i] or f"ELIGHT" in fighter_param[i]:
                        deleting = True
                        to_continue = True
                        next_value = fighter_param[i+1]
                        next_value = next_value.replace('<struct index="', "")
                        next_value = next_value.replace(" ", "")
                        next_value = next_value.replace('">', "")
                        next_value = next_value.replace('\n', "")
                        fighter_param[i] = f'    <hash40 index="{next_value}">dummy</hash40>\n'
                        continue
                elif f == "ptrainer":
                    if f"PFUSHIGISOU" in fighter_param[i] or f"PLIZARDON" in fighter_param[i] or f"PZENIGAME" in fighter_param[i]:
                        deleting = True
                        to_continue = True
                        next_value = fighter_param[i+1]
                        next_value = next_value.replace('<struct index="', "")
                        next_value = next_value.replace(" ", "")
                        next_value = next_value.replace('">', "")
                        next_value = next_value.replace('\n', "")
                        fighter_param[i] = f'    <hash40 index="{next_value}">dummy</hash40>\n'
                        continue
                else:
                    if (f"{f.upper()}-" in fighter_param[i] or f"{f.upper()} " in fighter_param[i]) and not f"{f.upper()} /" in fighter_param[i]:
                        deleting = True
                        to_continue = True
                        next_value = fighter_param[i+1]
                        next_value = next_value.replace('<struct index="', "")
                        next_value = next_value.replace(" ", "")
                        next_value = next_value.replace('">', "")
                        next_value = next_value.replace('\n', "")
                        fighter_param[i] = f'    <hash40 index="{next_value}">dummy</hash40>\n'
                        continue
            if to_continue == False:
                deleting = False
        if deleting == True and '">dummy</hash40>' not in fighter_param[i]:
            fighter_param[i] = "\n"

    f = open("ultimate/mods/Ultimate S Arcropolis/fighter/common/param/fighter_param.prcxml", "w")
    for i in fighter_param:
        f.write(i)
    f.close()

    # Markers for the code
    if os.path.exists(f"ultimate/ult-s/"):
        shutil.rmtree(f"ultimate/ult-s/")
    os.mkdir(f"ultimate/ult-s/")

    for i in included:
        f = open(f"ultimate/ult-s/{i}.flag", "x")
        f.close()

    #Underscoring their param files and other romfs stuff
    if "ptrainer" in excluded:
        excluded.remove("ptrainer")
        excluded.append("pfushigisou")
        excluded.append("pzenigame")
        excluded.append("plizardon")
    if "element" in excluded:
        excluded.remove("element")
        excluded.append("eflame")
        excluded.append("elight")
    for i in excluded:
        if not os.path.exists(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/"):
            continue
        else:
            if os.path.exists(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/param"):
                os.rename(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/param", f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/.param")
            for root, dirs, files in os.walk(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/", topdown=False):
                for name in dirs:
                    if "c0" in name:
                        path = os.path.join(root, name)
                        new_path = path.replace("c0", ".c0")
                        os.rename(path, new_path)
        if not os.path.exists(f"ultimate/mods/Ultimate S Arcropolis/effect/fighter/{i}/"):
            continue
        else:
            path = f"ultimate/mods/Ultimate S Arcropolis/effect/fighter/{i}/"
            new_path = path.replace("fighter/", "fighter/.")
            os.rename(path, new_path)
                        
    #Un-underscoring included param files and other romfs stuff
    if "ptrainer" in included:
        included.remove("ptrainer")
        included.append("pfushigisou")
        included.append("pzenigame")
        included.append("plizardon")
    if "element" in included:
        included.remove("element")
        included.append("eflame")
        included.append("elight")
    for i in included:
        if not os.path.exists(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/"):
            continue
        else:
            if os.path.exists(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/.param"):
                os.rename(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/.param", f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/param")
            for root, dirs, files in os.walk(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/", topdown=False):
                for name in dirs:
                    if ".c0" in name:
                        path = os.path.join(root, name)
                        new_path = path.replace(".", "")
                        os.rename(path, new_path)
        if not os.path.exists(f"ultimate/mods/Ultimate S Arcropolis/effect/fighter/.{i}/"):
            continue
        else:
            path = f"ultimate/mods/Ultimate S Arcropolis/effect/fighter/.{i}/"
            new_path = path.replace("fighter/.", "fighter/")
            os.rename(path, new_path)

def grab_dependencies():

    #Setup Downloads folder
    if os.path.exists("downloads"):
        shutil.rmtree("downloads")
    os.mkdir(f"downloads")
    os.mkdir(f"downloads/romfs")
    os.mkdir(f"downloads/romfs/plugins")

    #Skyline
    download_dependency = "https://github.com/skyline-dev/skyline/releases/download/beta/skyline.zip"
    r = requests.get(download_dependency)
    f = open("downloads/skyline.zip","wb")
    f.write(r.content)
    f.close()
    with zipfile.ZipFile("downloads/skyline.zip", 'r') as zip_ref:
        zip_ref.extractall("downloads")
    os.remove("downloads/skyline.zip")
    print("       Skyline downloaded")

    #NRO Hook
    g = Github(None)
    repo = g.get_repo("ultimate-research/nro-hook-plugin")
    latest = repo.get_latest_release()
    latest_ver = latest.html_url.replace("https://github.com/ultimate-research/nro-hook-plugin/releases/tag/","")
    download_dependency = f"https://github.com/ultimate-research/nro-hook-plugin/releases/download/{latest_ver}/libnro_hook.nro"
    r = requests.get(download_dependency)
    f = open("downloads/romfs/plugins/libnro_hook.nro","wb")
    f.write(r.content)
    f.close()
    print("       NRO Hook downloaded")

    #Smashline Hook
    g = Github(None)
    repo = g.get_repo("HDR-Development/smashline")
    latest = repo.get_latest_release()
    latest_ver = latest.title
    download_dependency = f"https://github.com/HDR-Development/smashline/releases/download/{latest_ver}/libsmashline_plugin.nro"
    r = requests.get(download_dependency)
    f = open("downloads/romfs/plugins/libsmashline_plugin.nro","wb")
    f.write(r.content)
    f.close()
    print("       Smashline Hook downloaded")

    #Arcropolis
    g = Github(None)
    repo = g.get_repo("Raytwo/ARCropolis")
    latest = repo.get_latest_release()
    latest_ver = latest.html_url.replace("https://github.com/Raytwo/ARCropolis/releases/tag/","")
    download_dependency = f"https://github.com/Raytwo/ARCropolis/releases/download/{latest_ver}/release.zip"
    r = requests.get(download_dependency)
    f = open("downloads/release.zip","wb")
    f.write(r.content)
    f.close()
    with zipfile.ZipFile("downloads/release.zip", 'r') as zip_ref:
        zip_ref.extractall("downloads")
    os.remove("downloads/release.zip")
    shutil.copy("downloads/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libarcropolis.nro","downloads/romfs/plugins/libarcropolis.nro")
    shutil.rmtree("downloads/atmosphere")
    print("       ARCropolis downloaded")

    #Param Config
    g = Github(None)
    repo = g.get_repo("CSharpM7/lib_paramconfig")
    latest = repo.get_latest_release()
    latest_ver = latest.html_url.replace("https://github.com/CSharpM7/lib_paramconfig/releases/tag/","")
    download_dependency = f"https://github.com/CSharpM7/lib_paramconfig/releases/download/{latest_ver}/libparam_config.nro"
    r = requests.get(download_dependency)
    f = open("downloads/romfs/plugins/libparam_config.nro","wb")
    f.write(r.content)
    f.close()
    print("       Param Config downloaded")

    #Stage Config
    g = Github(None)
    repo = g.get_repo("ThatNintendoNerd/stage_config")
    latest = repo.get_latest_release()
    latest_ver = latest.html_url.replace("https://github.com/ThatNintendoNerd/stage_config/releases/tag/","")
    download_dependency = f"https://github.com/ThatNintendoNerd/stage_config/releases/download/{latest_ver}/release.zip"
    r = requests.get(download_dependency)
    f = open("downloads/release.zip","wb")
    f.write(r.content)
    f.close()
    with zipfile.ZipFile("downloads/release.zip", 'r') as zip_ref:
        zip_ref.extractall("downloads")
    os.remove("downloads/release.zip")
    shutil.copy("downloads/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libstage_config.nro","downloads/romfs/plugins/libstage_config.nro")
    shutil.rmtree("downloads/atmosphere")
    print("       Stage Config downloaded")

    #CSK Collection
    download_dependency = f"https://gamebanana.com/dl/1158250"
    r = requests.get(download_dependency)
    f = open("downloads/csk_collection.zip","wb")
    f.write(r.content)
    f.close()
    with zipfile.ZipFile("downloads/csk_collection.zip", 'r') as zip_ref:
        zip_ref.extractall("downloads")
    os.remove("downloads/csk_collection.zip")
    shutil.copy("downloads/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libthe_csk_collection.nro","downloads/romfs/plugins/libthe_csk_collection.nro")
    shutil.rmtree("downloads/atmosphere")
    shutil.rmtree("downloads/ultimate")
    print("       CSK Collection downloaded")

    #Arena Latency
    download_dependency = f"https://gamebanana.com/dl/1142218"
    r = requests.get(download_dependency)
    f = open("downloads/latency.zip","wb")
    f.write(r.content)
    f.close()
    with zipfile.ZipFile("downloads/latency.zip", 'r') as zip_ref:
        zip_ref.extractall("downloads/romfs/plugins/")
    os.remove("downloads/latency.zip")
    print("       Arena Latency downloaded")
    print("       Moving dependencies")
    if os.path.exists("atmosphere/contents/01006A800016E000/romfs/skyline/plugins"):
        shutil.rmtree("atmosphere/contents/01006A800016E000/romfs/skyline/plugins")
    shutil.copytree("downloads/romfs/plugins","atmosphere/contents/01006A800016E000/romfs/skyline/plugins")
    if os.path.exists("atmosphere/contents/01006A800016E000/exefs"):
        shutil.rmtree("atmosphere/contents/01006A800016E000/exefs")
    shutil.copytree("downloads/exefs","atmosphere/contents/01006A800016E000/exefs")

def remove_ftp_dir(ftp, path):
    for (name, properties) in ftp.mlsd(path=path):
        if name in ['.', '..']:
            continue
        elif properties['type'] == 'file':
            print(f"{path}/{name}")
            ftp.delete(f"{path}/{name}")
        elif properties['type'] == 'dir':
            remove_ftp_dir(ftp, f"{path}/{name}")
    ftp.rmd(path)
def contact_ftp(ip,user="anonymous",pw="anonymous"):
    host = ip
    port = 5000
    ftp = FTP()
    ftp.connect(host, port)
    ftp.login(user, pw)
    ftp.retrlines('LIST')
    print(ftp.sendcmd("TYPE I"))

    try:
        ftp.rename('atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libacmd_hook.nro', 'atmosphere/contents/01006A800016E000/romfs/skyline/disabled_plugins/libacmd_hook.nro')
        print("libacmd_hook moved")
    except Exception:
        print("libacmd_hook moved")
    try:
        ftp.rename('atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libparam_hook.nro', 'atmosphere/contents/01006A800016E000/romfs/skyline/disabled_plugins/libparam_hook.nro')
        print("libparam_hook moved")
    except Exception:
        print("libparam_hook moved")
    try:
        ftp.rename('atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libsmashline_hook.nro', 'atmosphere/contents/01006A800016E000/romfs/skyline/disabled_plugins/libsmashline_hook.nro')
        print("libsmashline_hook moved")
    except Exception:
        print("libsmashline_hook moved")
    try:
        ftp.rename('atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libtraining_modpack.nro', 'atmosphere/contents/01006A800016E000/romfs/skyline/disabled_plugins/libtraining_modpack.nro')
        print("libtraining_modpack moved")
    except Exception:
        print("libtraining_modpack moved")

    print("Setting up atmosphere folder....")
    upload_list = []
    folder_list = []
    for root, dirs, files in os.walk("atmosphere", topdown=False):
        for name in files:
            upload_list.append(os.path.join(root, name))
            folder_list.append(root)
    for i in range(0, len(folder_list)):
        with open(upload_list[i], "rb") as file:
            add_file = upload_list[i]
            add_file = add_file.replace("\\","/")
            print(add_file)
            ftp.storbinary(f"STOR {add_file}", file)
        file.close()
    print("Setting up ultimate/mods folder (This will take a moment!)")
    try:
        remove_ftp_dir(ftp, 'ultimate/mods/Ultimate S Arcropolis')
        remove_ftp_dir(ftp, 'ultimate/mods/Ultimate S Stages')
        remove_ftp_dir(ftp, 'ultimate/ult-s')
    except Exception:
        print("No previous installs")
    print("Transferring ultimate/mods folder (This will take an even longer moment! Please bare with this application)")
    upload_list = []
    folder_list = []
    for root, dirs, files in os.walk("ultimate", topdown=False):
        for name in files:
            upload_list.append(os.path.join(root, name))
            folder_list.append(root)
    print("Preparing folders")
    for i in folder_list:
        split = i.split("\\")
        for i in range(0,len(split)):
            directory = "/".join(split[0:i])
            try:
                print(ftp.sendcmd(f"MKD {directory}"))
            except Exception:
                continue
        directory = "/".join(split)
        try:
            print(ftp.sendcmd(f"MKD {directory}"))
        except Exception:
            continue
    for i in range(0, len(folder_list)):
        with open(upload_list[i], "rb") as file:
            add_file = upload_list[i]
            add_file = add_file.replace("\\","/")
            backslash = r"\ ".replace(" ","")
            add_file = add_file.replace(backslash,"/")
            print(add_file)
            ftp.storbinary(f"STOR {add_file}", file)
        file.close()

#Main
if not os.path.exists("resources/fighter_param.prcxml"):
        shutil.copy("ultimate/mods/Ultimate S Arcropolis/fighter/common/param/fighter_param.prcxml","resources/fighter_param.prcxml")
        f = open("resources/fighter_param.prcxml")
        lines = f.readlines()
        for i in range(1,363):
            lines.pop(1)
        f.close()
        f = open("resources/fighter_param.prcxml", "w")
        for i in lines:
            f.write(i)
        f.close()
if not os.path.exists("resources/og_layout.arc"):
        shutil.copy("ultimate/mods/Ultimate S Arcropolis/ui/layout/menu/title/title/layout.arc","resources/og_layout.arc")

f = open("ultimate/mods/Ultimate S Arcropolis/version.txt")
new_word = f.readline()
f.close()
f = open("ultimate/mods/Ultimate S Arcropolis/version.txt", "w")
f.write(new_word.replace(".LITE",""))
f.close()
file = open("ultimate/mods/Ultimate S Arcropolis/version.txt")
romfs_ver = file.readline()
romfs_ver = romfs_ver.replace("\n","")
romfs_ver = romfs_ver.replace("v.","")
romfs_ver = romfs_ver.replace("v","")
file.close()
latest_ver = "1.0.0"
try:
    g = Github(None)
    repo = g.get_repo("chrispo-git/ult-s")
    latest = repo.get_latest_release()
    latest_ver = latest.html_url.replace("https://github.com/chrispo-git/ult-s/releases/tag/","")
    latest_ver = latest_ver.replace("v.","")
    latest_ver = latest_ver.replace("v","")
    latest_ver = latest_ver.replace("\n","")
except Exception:
    print("no internet connection")
print(f'''
                                                                                                                                                          
        ###    ###   ###    ##########  ####  ##########  ###########  ##########-  #########        ##########       
        ###   ####  ###    ##########   ###  ##########   ###   ####   #########   ###              ###               
       ###    ###  ###       ####      ###  ### ### ##-  ##########      ####     #######          ##########         
      ##########  ########+  ####     ###   ## +## ###  ###########     ####     ####                     ###         
     ##########   ########  ####     ####  ##  ##  ##  ####    ###     ####      #########        ##########         
                                                                                       (Ultimate S v{romfs_ver})

       Welcome to the Ultimate S Setup Utility! 
''')

if Version(romfs_ver) < Version(latest_ver):
    print("       There is a new version of Ultimate S!")
    latest_dl = input("       Would you like to download the latest version from the internet? (y/n)").lower()
    if latest_dl == "y":
        folder = filedialog.askdirectory(title="Select the folder to download to")
        print("       Please Wait!")
        dl_s = f"https://github.com/chrispo-git/ult-s/releases/download/v{latest_ver}/Ultimate.S.{latest_ver}.zip"
        r = requests.get(dl_s)
        f = open(f"{folder}/Ultimate.S.{latest_ver}.zip","wb")
        f.write(r.content)
        f.close()
        input("       New version of Ultimate S downloaded! Please unzip that new version and run the setup tool there.")
        sys.exit()
print('''
       Setup full Ultimate S or Ultimate S Lite?
       Full - The full intended experience of Ultimate S, with all character changes
       Lite - allows you to enable/disable certain character changes (EXPERIMENTAL, MAY HAVE BUGS)
''')
is_lite = input("       (full/lite) ")
if is_lite.lower() != "lite":
    print("       Full chosen!")
    if "ptrainer" in included:
        included.remove("ptrainer")
        included.append("pfushigisou")
        included.append("pzenigame")
        included.append("plizardon")
    if "element" in included:
        included.remove("element")
        included.append("eflame")
        included.append("elight")
    for i in included:
        if not os.path.exists(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/"):
            continue
        else:
            if os.path.exists(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/.param"):
                os.rename(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/.param", f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/param")
            for root, dirs, files in os.walk(f"ultimate/mods/Ultimate S Arcropolis/fighter/{i}/", topdown=False):
                for name in dirs:
                    if ".c0" in name:
                        path = os.path.join(root, name)
                        new_path = path.replace(".", "")
                        os.rename(path, new_path)
    if os.path.exists(f"ultimate/ult-s/"):
        shutil.rmtree(f"ultimate/ult-s/")
    os.mkdir(f"ultimate/ult-s/")
    for i in included:
        f = open(f"ultimate/ult-s/{i}.flag", "x")
        f.close()
    os.remove("ultimate/mods/Ultimate S Arcropolis/ui/layout/menu/title/title/layout.arc")
    shutil.copy("resources/og_layout.arc","ultimate/mods/Ultimate S Arcropolis/ui/layout/menu/title/title/layout.arc")
    f = open("ultimate/mods/Ultimate S Arcropolis/version.txt")
    new_word = f.readline()
    f.close()
else:
    print("       Lite chosen!")
    print('''
       Options:
            A. Type in the characters you want to exclude from your install (e.g. you don't want Mario)
            B. Type in the characters you want to include in your install (e.g. you ONLY want Mario)
    ''')
    is_exclude = input("       A or B (a/b)").lower()
    if is_exclude == "a":
        print("       Type the characters to exclude from install (in lowercase, with a space separating each)")
        characters = input("       ")
        characters = characters.lower()

        characters = characters.replace("dark samus", "samusd")
        characters = characters.replace("mii gunner", "miigunner")
        characters = characters.replace("mii brawler", "miifighter")
        characters = characters.replace("mii sword", "miiswordsman")
        characters = characters.replace("mii swordfighter", "miiswordsman")
        characters = characters.replace("mii swordsman", "miiswordsman")
        characters = characters.replace("bowser jr", "koopajr")
        characters = characters.replace("dark pit", "pitb")
        characters = characters.replace("dr mario", "mariod")

        characters = characters.split(" ")
        for x in range(0,len(characters)):
            has_replace = False
            for i in replace:
                for f in i:
                    if f in characters[x]:
                        characters[x] = i[0]
                        has_replace = True
                        break
                if has_replace == True:
                    break
        if characters != [""]:
            
            for i in characters:
                if i not in included:
                    input("       Error! Have you misspelled a character's name? (remember, you don't need to include added Ultimate S characters in your list)")
                    sys.exit()
            excluded = characters
            for i in excluded:
                included.remove(i)
        else:
            excluded = []
        
        if "ptrainer" in included:
            included.remove("ptrainer")
            included.append("pfushigisou")
            included.append("pzenigame")
            included.append("plizardon")
        if "element" in included:
            included.remove("element")
            included.append("eflame")
            included.append("elight")
        run_lite()
    else:
        print("       Type the characters to include in install (in lowercase, with a space separating each)")
        characters = input("       ")
        characters = characters.lower()

        characters = characters.replace("dark samus", "samusd")
        characters = characters.replace("mii gunner", "miigunner")
        characters = characters.replace("mii brawler", "miifighter")
        characters = characters.replace("mii sword", "miiswordsman")
        characters = characters.replace("mii swordfighter", "miiswordsman")
        characters = characters.replace("mii swordsman", "miiswordsman")
        characters = characters.replace("bowser jr", "koopajr")
        characters = characters.replace("dark pit", "pitb")
        characters = characters.replace("dr mario", "mariod")

        characters = characters.split(" ")
        for x in range(0,len(characters)):
            has_replace = False
            for i in replace:
                for f in i:
                    if f in characters[x]:
                        characters[x] = i[0]
                        has_replace = True
                        break
                if has_replace == True:
                    break
        if characters != [""]:
            
            for i in characters:
                if i not in included:
                    input("       Error! Have you misspelled a character's name? (remember, you don't need to include added Ultimate S characters in your list)")
                    sys.exit()
            for i in characters:
                included.remove(i)
            excluded = included
            included = characters
        else:
            excluded = included
            included = []
        
        if "ptrainer" in included:
            included.remove("ptrainer")
            included.append("pfushigisou")
            included.append("pzenigame")
            included.append("plizardon")
        if "element" in included:
            included.remove("element")
            included.append("eflame")
            included.append("elight")
        print("       Please Wait! The program is making the changes.")
        run_lite()
download = input("       Would you like to download the latest dependencies? This requires an internet connection. (y/n)").lower()
if download == "y":
    grab_dependencies()
setup = input("       Would you like to automatically set this up on your SD Card? (y/n)").lower()
if setup == "y":
    print("       1. Transfer to a specific folder (sdmc folder on emulator, root of your SD card for switch)")
    print("       2. Transfer to your switch via FTP")
    setup = input("       ").lower()
    if setup == "1": # ON PC
        folder = filedialog.askdirectory(title="Select the root folder of your SD card/sdmc folder")
        
        if os.path.exists(f"{folder}/atmosphere/contents/01006A800016E000"):
            os.mkdir(f"original_folder")
            shutil.copytree(f"{folder}/atmosphere/contents/01006A800016E000", f"original_folder/atmosphere/contents/01006A800016E000")
            shutil.rmtree(f"{folder}/atmosphere/contents/01006A800016E000")
        shutil.copytree(f"atmosphere/contents/01006A800016E000", f"{folder}/atmosphere/contents/01006A800016E000")
        # Removes all the icky plugins that shouldn't be there
        disallowed_plugins = ["libacmd_hook.nro", "libparam_hook", "libsmashline_hook", "libtraining_modpack"]
        for i in disallowed_plugins:
            if os.path.isfile(f"{folder}/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/{i}"):
                shutil.move(f"{folder}/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/{i}", f"{folder}/atmosphere/contents/01006A800016E000/romfs/skyline/disabled_plugins/{i}")
        print("       Atmosphere folder copied.")
        if True:
            if os.path.exists(f"{folder}/ultimate/mods/Ultimate S Arcropolis"):
                shutil.rmtree(f"{folder}/ultimate/mods/Ultimate S Arcropolis")
            if os.path.exists(f"{folder}/ultimate/mods/Ultimate S Stages"):
                shutil.rmtree(f"{folder}/ultimate/mods/Ultimate S Stages")
            if os.path.exists(f"{folder}/ultimate/ult-s"):
                shutil.rmtree(f"{folder}/ultimate/ult-s")
            print("Copying ultimate folders.... Please Wait! This will take a little bit.")
            shutil.copytree(f"ultimate/mods/Ultimate S Arcropolis", f"{folder}/ultimate/mods/Ultimate S Arcropolis")
            shutil.copytree(f"ultimate/mods/Ultimate S Stages", f"{folder}/ultimate/mods/Ultimate S Stages")
            shutil.copytree(f"ultimate/ult-s", f"{folder}/ultimate/ult-s")
    elif setup == "2": # FTP
        ip = input("       Please enter the switch IP address:")
        usr = input("       Please enter the FTP username (leave empty if its an open FTP server):")
        pwd = "anonymous"
        if usr == "":
            usr = "anonymous"
        else:
            pwd = input("       Please enter the FTP password:")
        contact_ftp(ip, usr, pwd)

print("       Done!")
input("       (Press Enter to quit)")
