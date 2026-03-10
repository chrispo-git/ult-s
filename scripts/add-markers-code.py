import os
import shutil
from zipfile import ZipFile
import sys

folder_list = [
    'bayonetta', 'brave', 'buddy', 'captain', 'chrom', 'cloud', 'daisy', 'dedede', 
    'diddy', 'dolly', 'donkey', 'duckhunt', 'edge', 'eflame', 'elight', 'gaogaen', 
    'falco', 'fox', 'ganon', 'gamewatch', 'gekkouga', 'ike', 'inkling', 'jack', 
    'kamui', 'ken', 'kirby', 'koopa', 'koopajr', 'krool', 'link', 'littlemac', 
    'lucario', 'lucas', 'lucina', 'luigi', 'mariod', 'mario', 'marth', 'master', 
    'metaknight', 'mewtwo', 'murabito', 
    'nana', 'ness', 'packun', 'pacman', 'palutena', 'peach', 'pfushigisou', 
    'pichu', 'pickel', 'pikachu', 'pikmin', 'pitb', 'pit', 'plizardon', 'popo', 
    'purin', 'pzenigame', 'reflet', 'richter', 'ridley', 'robot', 'rockman', 
    'rosetta', 'roy', 'ryu', 'samusd', 'samus', 'sheik', 'shizue', 'shulk', 
    'simon', 'snake', 'sonic', 'szerosuit', 'tantan', 'toonlink', 'trail', 
    'wario', 'wiifit', 'wolf', 'yoshi', 'younglink', 'zelda'
]


os.chdir('../')
os.chdir('src')

for char in folder_list:
    directory = f'{char}'
    old_str = '.set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())'
    new_str = f'.set_costume(get_marked_costumes("{char}","{char}"))'
    # TODO - Write code that crawls through EVERY *.rs file and replaces 

    if not os.path.exists(directory):
        continue
    for root, dirs, files in os.walk(directory):
        for filename in files:
            if filename.endswith('.rs'):
                file_path = os.path.join(root, filename)

                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                if old_str in content:
                    new_content = content.replace(old_str, new_str)
                    
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    print(f"Updated: {file_path}")

added_char_markers = [
    ["murabito", "toad"],
    ["pikmin", "rayman"],
    ["pacman","bomberman"],
    ["falco", "peppy"]
]
for char_pair in added_char_markers:
    for slot in range(120,128):
        directory = f'fighter/{char_pair[0]}/model/body/c{slot}'
        path = f'{directory}/{char_pair[1]}.marker'
        os.makedirs(directory, exist_ok=True)
        if not os.path.isfile(path):
            with open(path, 'w') as fp:
                pass
print('done :)')