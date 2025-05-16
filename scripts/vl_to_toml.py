import xml.etree.ElementTree as ET
import os
import shutil
from zipfile import ZipFile
import sys
import subprocess


def convert_xml_to_custom_format(xml_string, fighter_kind):
    root = ET.fromstring(xml_string)
    output = []

    for list_elem in root.findall('list'):
        param_name = list_elem.get('hash')
        for struct_elem in list_elem.findall('struct'):
            for child in struct_elem:
                tag = child.tag
                subparam = child.get('hash')
                value = child.text
                value_type = tag  # either 'int' or 'float'
                if value_type == "bool":
                    value_type = "int"
                    if value == "true":
                        value = 1
                    else:
                        value = 0
                kinds = f'"{fighter_kind}"'
                if param_name not in ["param_special_n", "param_special_s", "param_special_lw", "param_special_hi", "param_private"]:
                    kinds = kinds + ', "' + fighter_kind + param_name.replace("param","") + '"'
                block_type = f"param_{value_type}"
                block = f"""[[{block_type}]]
param = "{param_name}"
subparam = "{subparam}"
value = {value}
kinds = [{kinds}]
slots = [0,1,2,3,4,5,6,7]
"""
                output.append(block)
    return '\n'.join(output)


try:
    inputs = (" ".join(sys.argv)).lower()
except IndexError:
    inputs = "invalid"

inputs = inputs.replace('vl_to_toml.py', "")
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
    ['szerosuit', 'zero', 'zss'],
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

os.chdir('../')
try:
    f = open(f"romfs/fighter/{output_folder}/param/vl.prcxml")
    xml_in = f.read()
    f.close()
    converted = convert_xml_to_custom_format(xml_in, output_folder)
    f = open("scripts/output.toml", "w")
    f.write(converted)
    f.close()
except IOError:
    raise Exception("vl.prcxml not found")
