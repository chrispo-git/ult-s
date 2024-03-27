import subprocess
import shutil
import os
import sys
import time

path = ""
try:
    inputs = ("".join(sys.argv)).lower()
    inputs = inputs.replace('build.py', "")
    inputs = inputs.replace('\n', "")
    print(inputs)
    path = inputs
    if "c00" in path:
        path = path.replace("/c00", "")
        path = path.replace("\c00", "")
    os.chdir(path)
except IndexError:
    print("no filepath added")


#clones a c00 into c00-c007
if os.path.exists(r'c00'):
    original = r'c00'
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