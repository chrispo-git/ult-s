import subprocess
import shutil
import os
import sys
import time

path = ""
try:
    inputs = ("".join(sys.argv))
    inputs = inputs.replace('c00clone.py', "")
    inputs = inputs.replace('\n', "")
    print(inputs)
    path = inputs
    os.chdir('../')
    os.chdir(f'romfs/fighter/{path}/motion/body')
except IndexError:
    print("no filepath added")


#clones a c00 into c00-c007
if os.path.exists(r'c00'):
    original = r'c00'
    shutil.rmtree(r'c01')
    shutil.rmtree(r'c02')
    shutil.rmtree(r'c03')
    shutil.rmtree(r'c04')
    shutil.rmtree(r'c05')
    shutil.rmtree(r'c06')
    shutil.rmtree(r'c07')
    target =  r'c01'
    shutil.copytree(original, target)
    target =  r'c02'
    shutil.copytree(original, target)
    target =  r'c03'
    shutil.copytree(original, target)
    target =  r'c04'
    shutil.copytree(original, target)
    target =  r'c05'
    shutil.copytree(original, target)
    target =  r'c06'
    shutil.copytree(original, target)
    target =  r'c07'
    shutil.copytree(original, target)