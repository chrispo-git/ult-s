import math
import os
import sys
import shutil
from string import ascii_lowercase

#REQUIREMENTS TO USE THIS SCRIPT:
#1. have yamlist.exe on the outside of your Ultimate S Smashline folder
#2. have yamlist.py in the same spot

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
    ['pikmin', 'olimar', 'alph'],
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

try:
    char = (" ".join(sys.argv)).lower()
    char = char.replace('statistics.py', "")
    char = char.replace(' ', "")
except IndexError:
    char = input("Character?")

character = char.lower()

character = character.replace("mii gunner", "miigunner")
character = character.replace("mii brawler", "miifighter")
character = character.replace("mii sword", "miiswordsman")
character = character.replace("bowser jr", "koopajr")
character = character.replace("dark pit", "pitb")
character = character.replace("dr mario", "mariod")
character = character.replace("dark samus", "samusd")

has_replace = False

for i in replace:
    for f in i:
        if f in character:
            character = i[0]
            has_replace = True
            print(character)
            break
    if has_replace == True:
        break

if has_replace != True:
  raise Exception("Character not found! Did you misspell their name?")

os.chdir('../')
os.chdir('../')
os.chdir('../')
try:
    os.system(f'py yamlist.py "Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c00/motion_list.bin"')
    f = open(f"Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c00/motion_list.yml")
    yaml_output = f.readlines()
    f.close()
    shutil.copyfile(f'Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c00/motion_list.yml', f'Ultimate S Smashline/ultimate-s/scripts/motion_list.yml')
    os.remove(f"Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c00/motion_list.yml")
    os.chdir(f'Ultimate S Smashline/ultimate-s/scripts')
except Exception:
    print("No motion_list to extract") 

input("Press enter to convert it back")
os.chdir('../')
os.chdir('../')
os.chdir('../')
os.system(f'py yamlist.py "Ultimate S Smashline/ultimate-s/scripts/motion_list.yml"')
shutil.copyfile(f'Ultimate S Smashline/ultimate-s/scripts/motion_list.bin', f'Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c00/motion_list.bin')
if os.path.isdir(f'Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c01'):
    for i in range(1,8):
        if os.path.isdir(f'Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c0{i}') == False:
            os.mkdir(f'Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c0{i}')
        shutil.copyfile(f'Ultimate S Smashline/ultimate-s/scripts/motion_list.bin', f'Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c0{i}/motion_list.bin')
os.remove(f"Ultimate S Smashline/ultimate-s/scripts/motion_list.yml")
os.chdir(f'Ultimate S Smashline/ultimate-s/scripts')
