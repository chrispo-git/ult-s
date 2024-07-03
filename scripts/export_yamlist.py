import math
import os
import sys
import shutil
from string import ascii_lowercase

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
    ['ganon', 'ganondorf'],
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
    ['murabito', 'villager', 'villy'],
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

replace_add = [
  ["bomberman","pacman"],
  ["sandbag","mariod"],
  ["toad","murabito"],
  ["rayman","pikmin"],
  ["masked","lucas"]
]

number = 0
for i in replace_add:
    if i[0] in character:
        character = i[1]
        has_replace = True
        number = 12 
        print(i[0])
        break
if has_replace != True:
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
try:
    os.system(f'py yamlist.py "romfs/fighter/{character}/motion/body/c{number}0/motion_list.bin"')
    print(os.getcwd())
    shutil.copyfile(f'romfs/fighter/{character}/motion/body/c{number}0/motion_list.yml', f'scripts/motion_list.yml')
    os.remove(f"romfs/fighter/{character}/motion/body/c{number}0/motion_list.yml")
    os.chdir(f'scripts')
except Exception:
    raise Exception("No motion_list to extract")

os.startfile("motion_list.yml")
input("Press enter to convert it back")
os.chdir('../')
os.system(f'py yamlist.py "scripts/motion_list.yml"')
shutil.copyfile(f'scripts/motion_list.bin', f'romfs/fighter/{character}/motion/body/c{number}0/motion_list.bin')
if os.path.isdir(f'romfs/fighter/{character}/motion/body/c{number}1'):
    for i in range(1,8):
        if os.path.isdir(f'romfs/fighter/{character}/motion/body/c{number}{i}') == False:
            os.mkdir(f'romfs/fighter/{character}/motion/body/c{number}{i}')
        shutil.copyfile(f'scripts/motion_list.bin', f'romfs/fighter/{character}/motion/body/c{number}{i}/motion_list.bin')
os.remove(f"scripts/motion_list.yml")
os.remove(f"scripts/motion_list.bin")
os.chdir(f'scripts')
