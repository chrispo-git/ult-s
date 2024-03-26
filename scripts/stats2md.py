import os


def run_stats2md():
    if not os.path.isfile("output.csv"):
        raise Exception("output.csv not found! please run statistics.py [character] first")


    in_lists = [
    "Jab 1",
    "Jab 2",
    "Jab 3",
    "Rapid Jab",
    "Rapid Jab Finisher",
    "Jab 4",
    "Jab 5",
    "Jab 6",
    "Jab 7",
    "Jab 8",
    "Jab 9",
    "FTilt",
    "FTilt (Up)",
    "FTilt (Down)",
    "FTilt 2",
    "FTilt 3",
    "UTilt",
    "DTilt",
    "Dash Attack",
    "Forward Smash",
    "Forward Smash (Up)",
    "Forward Smash (Down)",
    "Forward Smash (Charge)",
    "Forward Smash 2",
    "Forward Smash 3",
    "Up Smash",
    "Up Smash (Charge)",
    "Down Smash",
    "Down Smash (Charge)",
    "Slide Up Attack",
    "Slide Down Attack",
    "Slide Neutral Attack",
    "Zair",
    "Nair",
    "Nair 2",
    "Nair 3",
    "Nair (Landing)",
    "Fair",
    "Fair 2",
    "Fair 3",
    "Fair (Landing)",
    "Bair",
    "Bair (Landing)",
    "Uair",
    "Uair (Landing)",
    "Dair",
    "Dair (Landing)",
    "Grab",
    "Dash Grab",
    "Pivot Grab",
    "Pummel",
    "FThrow",
    "BThrow",
    "Up Throw",
    "Down Throw",
    "Cargo FThrow",
    "Cargo BThrow",
    "Cargo Up Throw",
    "Cargo Down Throw",
    "Command Input"
    ]
    specials = [
    "Neutral Special",
    "Side Special",
    "Up Special",
    "Down Special"
    ]
    f = open("output.csv")
    stats = f.readlines()
    f.close()

    md = []

    stats.pop(0)
    md.append(f"## Attributes\n")
    md.append(f"| Stat | Value |\n")
    md.append(f"| ------------- |:-------------:|\n")

    for i in range(0,19):
        x = stats[0]
        stats.pop(0)
        x = x.replace(",", "|")
        x = x.replace("\n", "")
        md.append(f"{x}|\n")

    md.append(f"\n\n\n\n## Attacks\n")
    while len(stats) > 0:
        x = stats[0]
        stats.pop(0)
        x = x.replace("\n", "")
        x = x.replace(", Notes: ", "")

        if "ID" in x:
            x = x.replace(",", "|")
            md.append(f"|Frame|ID|Damage|Angle|BKB|KBG|Notes| | | |\n")
            md.append(f"|-|-|-|-|-|-|-|-|-|-|\n")
            continue
        
        if "Frame" in x:
            x = x.replace(",", "|")
            md.append(f"|{x}|\n")
            continue
        
        if len(x) > 0:
            for i in specials:
                if i in x:
                    md.append(f"\n\n\n\n### {x}\n")
                    continue
            if x in in_lists:
                md.append(f"\n\n\n\n### {x}\n")
                continue
            elif "FaF" in x or "Autocancel" in x:
                md.append(f"\n\n{x}\n\n")
            else:
                md.append(f"|{x}|\n")
            
        else:
            md.append(f"\n")
        

    f = open("output.md", "w")
    for i in md:
        is_repeat = False
        for s in specials:
            if s in i and "#" not in i:
                is_repeat = True
        if not is_repeat:
            f.write(i)
    f.close()
    print("stats2md is done!")
