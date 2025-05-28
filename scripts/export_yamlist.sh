#! /usr/bin/bash
cd ..
echo "romfs/fighter/$1/motion/body/c00/motion_list.bin"
wine yamlist.exe disasm "romfs/fighter/$1/motion/body/c00/motion_list.bin" -l 'Labels.txt' -o "romfs/fighter/$1/motion/body/c00/motion_list.yml"
xdg-open "romfs/fighter/$1/motion/body/c00/motion_list.yml"
read -p "Press enter to continue"
echo "romfs/fighter/$1/motion/body/c00/motion_list.bin"
wine yamlist.exe asm "romfs/fighter/$1/motion/body/c00/motion_list.yml" -o "romfs/fighter/$1/motion/body/c00/motion_list.bin"
rm "romfs/fighter/$1/motion/body/c00/motion_list.yml"
cd scripts
python3 c00clone.py $1
chmod u+x export_yamlist.sh