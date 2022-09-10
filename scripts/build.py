import os
import shutil

stream = os.popen('cargo skyline build')
output = stream.read()
output
os.chdir('../')
print(os.getcwd())
old = r"target\aarch64-skyline-switch\debug\libplugin.nro"
new = r"releases"
old_rename = r"libplugin.nro"
rename = r"plugin.nro"

print(os.listdir())

if os.path.exists(old) == False:
    print("issue")

if os.path.exists(new) == False:
    os.mkdir(new)
print(os.listdir())

shutil.move(old, new)
print(os.listdir())

os.chdir(r'releases')
if os.path.exists('plugin.nro'):
    stream = os.popen('del plugin.nro')
    output = stream.read()
    output
stream = os.popen('rename libplugin.nro plugin.nro')
output = stream.read()
output