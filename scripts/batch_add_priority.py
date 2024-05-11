
all_files = [""]

try:
    os.chdir('../')
    for root, dirs, files in os.walk(r"src", topdown=False):
        for name in files:
            if name != "modules" and name not in all_files:
                all_files.append(name)

for i in all_files:
    print(i)
    file = []
    f = open(i, "r")
    file = f.readlines()
    f.close()
    for r in range(0, len(file)):
        if ".acmd" not in file[r]:
            continue
        
        new = file[r]
        new.replace(")","Priority::Low")
        file[r] = new
    
    f = open(i, "w")
    for r in file:
        f.write(r)
    f.close()
    print("done")