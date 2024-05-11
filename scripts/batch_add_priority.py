import os
all_files = []

os.chdir('../')
for dirpath, subdirs, files in os.walk(r"src", topdown=False):
        for name in files:
            if name != "modules" and name not in all_files and name.endswith(".rs"):
                all_files.append(os.path.join(dirpath, name))

for i in all_files:
    print(i)
    file = []
    f = open(i, "r")
    file = f.readlines()
    f.close()
    for r in range(0, len(file)):
        if (".acmd" not in file[r] and "_acmd(" not in file[r]) or "Priority::" in file[r]:
            continue
        new = file[r]
        new = new.replace(")",", Priority::Low)")
        file[r] = new
    
    f = open(i, "w")
    for r in file:
        f.write(r)
    f.close()
    print("done")