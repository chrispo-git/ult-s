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
    'metaknight', 'mewtwo', 'miifighter', 'miigunner', 'miiswordsman', 'murabito', 
    'nana', 'ness', 'packun', 'pacman', 'palutena', 'peach', 'pfushigisou', 
    'pichu', 'pickel', 'pikachu', 'pikmin', 'pitb', 'pit', 'plizardon', 'popo', 
    'purin', 'pzenigame', 'reflet', 'richter', 'ridley', 'robot', 'rockman', 
    'rosetta', 'roy', 'ryu', 'samusd', 'samus', 'sheik', 'shizue', 'shulk', 
    'simon', 'snake', 'sonic', 'szerosuit', 'tantan', 'toonlink', 'trail', 
    'wario', 'wiifit', 'wolf', 'yoshi', 'younglink', 'zelda'
]


os.chdir('../')
os.chdir('romfs')

for char in folder_list:
    for slot in range(8):
        directory = f'fighter/{char}/model/body/c0{slot}'
        path = f'{directory}/{char}.marker'
        os.makedirs(directory, exist_ok=True)
        if not os.path.isfile(path):
            with open(path, 'w') as fp:
                pass
print('done :)')