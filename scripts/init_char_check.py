import os
import shutil
from zipfile import ZipFile
import sys

try:
    inputs = (" ".join(sys.argv)).lower()
except IndexError:
    inputs = "invalid"

inputs = inputs.replace('init_char_check.py ', "")
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


output = inputs.split(" ")[0]
if output == " ":
    output = inputs.split(" ")[1]

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
        if f in output:
            output = i[0]
            has_replace = True
            print(output)
            break
    if has_replace == True:
        break

if has_replace == True:
    new_run = 'py build_char.py ' + output
    print(new_run)
    stream = os.popen(new_run)
    output = stream.read()
    output