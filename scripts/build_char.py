import os
import shutil
from zipfile import ZipFile
import sys

try:
    inputs = (" ".join(sys.argv)).lower()
except IndexError:
    inputs = "invalid"

inputs = inputs.replace('build_char.py', "")
print(inputs)
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

has_replace = False

for i in replace:
    for f in i:
        if f in output_folder:
            output_folder = i[0]
            has_replace = True
            print(output_folder)
            break
    if has_replace == True:
        break

print(output_folder)

def copytree(src, dst, symlinks=False, ignore=None):
    for item in os.listdir(src):
        s = os.path.join(src, item)
        d = os.path.join(dst, item)
        if os.path.isdir(s):
            shutil.copytree(s, d, symlinks, ignore)
        else:
            shutil.copy2(s, d)

stream = os.popen('cargo skyline build --release --features="main_nro"')
output = stream.read()
output
os.chdir('../')
print(os.getcwd())
old = r"target\aarch64-skyline-switch\release\libplugin.nro"
new = r"releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)"
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
            if os.path.exists(r'releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)') == False:
                os.makedirs(r'releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)')
            shutil.move(os.path.join(new, r'libplugin.nro'), os.path.join(new, r'plugin.nro'))
            if os.path.exists(r'romfs'):
                print("Starting copy")
                copytree(r'romfs/fighter/common', r'releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)/fighter/common')
                shutil.copy(r'romfs/config.json', r'releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)/config.json')
                if os.path.exists(os.path.join(r'romfs/fighter', output_folder)):
                    copytree(os.path.join(r'romfs/fighter', output_folder), os.path.join(r'releases/ultimate/mods/Ultimate S Arcropolis (plugin, singular character)/fighter', output_folder) )
                else:
                    print("Invalid! They may not have a romfs folder, or you may have not used their name")
                print("Copying from romfs finished, now zipping")
            else:
                print("Error! No romfs folder! Please check your install")
            if os.path.exists(r'releases/Ultimate S Arcropolis (plugin, singular character).zip'):
                os.remove(r'releases/Ultimate S Arcropolis (plugin, singular character).zip')
            file_paths = get_all_file_paths(new)
            with ZipFile(r'releases/Ultimate S Arcropolis (plugin, singular character).zip','w') as zip:
                for file in file_paths:
                    zip.write(file)
            print("Done!")
        else:
            print('aarch64-skyline-switch does not exist')
    else:
        print('aarch64-skyline-switch does not exist')
else:
    print('target does not exist')
