import sys
import os
import shutil


def empty_folder(folder):
        for filename in os.listdir(folder):
            file_path = os.path.join(folder, filename)
            try:
                if os.path.isfile(file_path) or os.path.islink(file_path):
                    os.unlink(file_path)
                elif os.path.isdir(file_path):
                    shutil.rmtree(file_path)
            except Exception as e:
                print('Failed to delete %s. Reason: %s' % (file_path, e))
params = sys.argv
params.pop(0)
print(params)

for i in params:
    if os.path.isdir(i):
        empty_folder(i)
    else:
        os.mkdir(i)
    
    shutil.copyfile(f"model.numatb", f"{i}/model.numatb")
    shutil.copyfile(f"model.numdlb", f"{i}/model.numdlb")

print("done")