#! /usr/bin/bash
fighter=$1
alt="0"
case $1 in
    bomberman)
        fighter="pacman"
        alt="12"
    ;;
    toad)
        fighter="murabito"
        alt="12"
    ;;
    rayman)
        fighter="pikmin"
        alt="12"
    ;;
    peppy)
        fighter="falco"
        alt="12"
    ;;
esac
cd ..
echo "romfs/fighter/$fighter/motion/body/c${alt}0/motion_list.bin"
wine yamlist.exe disasm "romfs/fighter/$fighter/motion/body/c${alt}0/motion_list.bin" -l 'Labels.txt' -o "romfs/fighter/$fighter/motion/body/c${alt}0/motion_list.yml"
xdg-open "romfs/fighter/$fighter/motion/body/c${alt}0/motion_list.yml"
read "Press enter to continue"
echo "romfs/fighter/$fighter/motion/body/c${alt}0/motion_list.bin"
wine yamlist.exe asm "romfs/fighter/$fighter/motion/body/c${alt}0/motion_list.yml" -o "romfs/fighter/$fighter/motion/body/c${alt}0/motion_list.bin"
rm "romfs/fighter/$fighter/motion/body/c${alt}0/motion_list.yml"
cd scripts
python3 c00clone.py $fighter
chmod u+x export_yamlist.sh