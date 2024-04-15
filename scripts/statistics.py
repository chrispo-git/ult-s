import math
import os
import sys
from string import ascii_lowercase
import tkinter as tk
from tkinter import filedialog
import stats2md


char_directory = []
current_script = ""
rs = []
wavedash_rs = []
default_param = []
new_param = []
output = []
yaml_output = []
yaml_faf = []
the_faf = []
faf_isolate = []
faf_output = []

already_edited = []

replace = [
    ['bayonetta', 'bayo'],
    ['brave', 'hero'],
    ['buddy', 'banjo'],
    ['captain', 'falcon', 'cap'],
    ['chrom'],
    ['cloud'],
    ['daisy'],
    ['dedede', 'd3'],
    ['demon', 'kazuya'],
    ['diddy'],
    ['dolly', 'terry'],
    ['donkey', 'dk'],
    ['duckhunt', 'duck'],
    ['edge', 'seph'],
    ['eflame', 'pyra'],
    ['elight', 'mythra'],
    ['gaogaen', 'incin'],
    ['ganon'],
    ['falco'],
    ['fox'],
    ['gamewatch', 'gnw', 'g&w'],
    ['gekkouga', 'greninja', 'gren'],
    ['ike'],
    ['inkling'],
    ['jack', 'joker'],
    ['kamui', 'corrin'],
    ['ken'],
    ['kirby'],
    ['koopa', 'bowser'],
    ['koopajr', 'jr'],
    ['krool'],
    ['link'],
    ['littlemac', 'mac'],
    ['lucario'],
    ['lucas'],
    ['lucina'],
    ['luigi'],
    ['mariod', 'dr', 'doc'],
    ['mario'],
    ['marth'],
    ['master', 'byleth'],
    ['metaknight', 'meta', 'mk'],
    ['mewtwo', 'mew2'],
    ['miifighter', 'brawler'],
    ['miigunner', 'gunner'],
    ['miiswordsman', 'sword'],
    ['murabito', 'villager', 'villy', 'toad'],
    ['ness'],
    ['packun', 'plant', 'piranha'],
    ['pacman', 'pac'],
    ['palutena', 'palu'],
    ['peach'],
    ['pfushigisou', 'ivy'],
    ['pichu'],
    ['pickel', 'steve'],
    ['pikachu', 'pika'],
    ['pikmin', 'olimar', 'alph', 'rayman'],
    ['pitb', 'dpit'],
    ['pit'],
    ['plizardon', 'charizard', 'zard'],
    ['popo'],
    ['purin', 'puff', 'jigglypuff'],
    ['pzenigame', 'squirtle'],
    ['reflet', 'robin'],
    ['richter'],
    ['ridley'],
    ['robot', 'rob'],
    ['rockman', 'megaman', 'mega'],
    ['rosetta', 'rosa', 'rosalina'],
    ['roy'],
    ['ryu'],
    ['samusd', 'dsamus'],
    ['samus'],
    ['sheik', 'shiek'],
    ['shizue', 'isa', 'isabelle'],
    ['shulk'],
    ['simon'],
    ['snake'],
    ['sonic'],
    ['szerosuit', 'zero'],
    ['tantan', 'minmin', 'min'],
    ['toonlink', 'tink', 'toon'],
    ['trail', 'sora'],
    ['wario'],
    ['wiifit', 'wii', 'wft'],
    ['wolf'],
    ['yoshi'],
    ['younglink', 'yink', 'young'],
    ['zelda']
]
projectile_list = [
   ["bayonetta","bayonetta_specialn_bullet"],
   ["brave", "brave_crash", "brave_deathball", "brave_explosion", "brave_fireball", "brave_flash", "brave_lightning", "brave_sleep", "brave_spark", "brave_tornado"],
   ["buddy", "buddy_bullet", "buddy_pad"],
   ["captain"],
   ["chrom"],
   ["cloud", "cloud_wave"],
   ["daisy"],
   ["dedede", "dedede_gordo"],
   ["demon","demon_blaster"],
   ["diddy", "diddy_barreljet", "diddy_gun"],
   ["dolly","dolly_wave"],
   ["donkey"],
   ["duckhunt","duckhunt_can","duckhunt_clay","duckhunt_gunmanbullet"],
   ["edge","edge_fire","edge_flare1", "edge_flare2"],
   ["eflame","eflame_esword"],
   ["elight","elight_exprosiveshot","elight_spreadbullet"],
   ["falco", "falco_blaster_bullet"],
   ["fox","fox_blaster_bullet"],
   ["gamewatch","gamewatch_food","gamewatch_oil"],
   ["ganon"],
   ["gaogaen"],
   ["gekkouga","gekkouga_shuriken","gekkouga_water"],
   ["ike"],
   ["inkling","inkling_inkbullet","inkling_splash"],
   ["jack","jack_fire2"],
   ["kamui","kamui_ryusensya","kamui_dragonhand", "kamui_spearhand"],
   ["ken","ken_hadoken"],
   ["kirby","kirby_finalcuttershot"],
   ["koopa","koopa_breath"],
   ["koopajr","koopajr_cannonball"],
   ["krool","krool_crown","krool_ironball","krool_backpack"],
   ["link","link_boomerang","link_bowarrow","link_swordbeam"],
   ["littlemac"],
   ["lucario","lucario_auraball","lucario_qigong"],
   ["lucas","lucas_pkfire","lucas_pkfreeze","lucas_pkthunder"],
   ["lucina"],
   ["luigi","luigi_fireball","luigi_obakyumu"],
   ["mario","mario_fireball"],
   ["mariod","mariod_drcapsule"],
   ["marth"],
   ["master","master_arrow1","master_axe"],
   ["metaknight"],
   ["mewtwo", "mewtwo_shadowball", "mewtwo_bindball"],
   ["miifighter", "miifighter_ironball"],
   ["miigunner", "miigunner_attackairf_bullet","miigunner_bottomshoot","miigunner_flamepillar","miigunner_grenadelauncher","miigunner_rapidshot_bullet","miigunner_gunnercharge","miigunner_stealthbomb_s","miigunner_supermissile","miigunner_miimissile"],
   ["miiswordsman","miiswordsman_chakram","miiswordsman_lightshuriken","miiswordsman_tornadoshot"],
   ["murabito","murabito_bullet"],
   ["nana","nana_blizzard","nana_iceshot"],
   ["popo","popo_blizzard","popo_iceshot"],
   ["ness","ness_pkfire","ness_pkflash","ness_pkthunder"],
   ["packun", "packun_poisonbreath", "packun_spikeball"],
   ["pacman","pacman_firehydrant","pacman_firehydrantwater"],
   ["palutena","palutena_autoaimbullet","palutena_explosiveflame"],
   ["peach"],
   ["pfushigisou","pfushigishou_leafcutter","pfushigisou_seed"],
   ["pichu","pichu_dengeki","pichu_dengekidama"],
   ["pickel","pickel_fire","pickel_trolley"],
   ["pikachu","pikachu_dengeki","pikachu_dengekidama", "pikachu_kaminari"],
   ["pikmin"],
   ["pit","pit_bowarrow"],
   ["pitb","pitb_bowarrow"],
   ["plizardon","plizardon_breath"],
   ["purin"],
   ["pzenigame","pzenigame_water"],
   ["reflet","reflet_elwind","reflet_gigafire","reflet_thunder"],
   ["richter","richter_axe","richter_cross"],
   ["ridley","ridley_breath"],
   ["robot","robot_beam"],
   ["rockman","rockman_airshooter","rockman_crashbomb","rockman_rockbuster","rockman_chargeshot"],
   ["rosetta","rosetta_tico","rosetta_starpiece"],
   ["roy"],
   ["ryu","ryu_hadoken","ryu_shinkuhadoken"],
   ["samus","samus_bomb","samus_cshot","samus_missile","samus_supermissile"],
   ["samusd","samusd_cshot","samusd_missile"],
   ["sheik","sheik_fusin","sheik_needle"],
   ["shizue","shizue_pot","shizue_clayrocket","shizue_fishingrod"],
   ["shulk"],
   ["simon","simon_axe","simon_cross"],
   ["snake","snake_c4","snake_cypher","snake_flaregrenades","snake_nikitamissile","snake_trenchmortarbullet"],
   ["sonic","sonic_gimmickjump"],
   ["szerosuit","szerosuit_paralyzer_bullet"],
   ["tantan","tantan_beam","tantan_punch1","tantan_punch2","tantan_punch3"],
   ["toonlink","toonlink_boomerang","toonlink_bowarrow"],
   ["trail","trail_fire","trail_thunder","trail_ice"],
   ["wario"],
   ["wiifit","wiifit_hulahoop","wiifit_sunbullet"],
   ["wolf","wolf_blaster_bullet"],
   ["yoshi","yoshi_tamago"],
   ["younglink","younglink_boomerang","younglink_bowarrow"],
   ["zelda","zelda_dein_s","zelda_phantom"]

]

def make_printable(string):
    replace_list = [
      ["attack11w", "Jab 1 (Light)"],
      ["attack11s", "Jab (Heavy)"],
      ["attack11nears", "Jab (Close)"],
      ["attack11", "Jab 1"],
      ["attack12", "Jab 2"],
      ["attack13", "Jab 3"],
      ["attack14", "Jab 4"],
      ["attack15", "Jab 5"],
      ["attack16", "Jab 6"],
      ["attack17", "Jab 7"],
      ["attack18", "Jab 8"],
      ["attack19", "Jab 9"],
      ["attack100", "Rapid Jab"],
      ["attack100end", "Rapid Jab Finisher"],
      ["attacknearw", "Ftilt (Close)"],
      ["attacks3", "FTilt"],
      ["attacks3s", "FTilt"],
      ["attacks3hi", "FTilt (Up)"],
      ["attacks3lw", "FTilt (Down)"],
      ["attacks3w", "Ftilt (Light)"],
      ["attacks3s2", "FTilt 2"],
      ["attacks3s3", "FTilt 3"],
      ["attacks32", "FTilt 2"],
      ["attacks33", "FTilt 3"],
      ["attackhi3", "UTilt"],
      ["attackhi3w", "UTilt (Light)"],
      ["attackhi3s", "UTilt (Heavy)"],
      ["attacklw3", "DTilt"],
      ["attacklw3", "DTilt"],
      ["attacklw3w", "DTilt (Light)"],
      ["attacklw3s", "DTilt (Heavy)"],
      ["attackdash", "Dash Attack"],
      ["attacks4", "Forward Smash"],
      ["attacks4hold", "Forward Smash (Charge)"],
      ["attacks4s", "Forward Smash"],
      ["attacks4hi", "Forward Smash (Up)"],
      ["attacks4lw", "Forward Smash (Down)"],
      ["attacks4s2", "Forward Smash 2"],
      ["attacks4s3", "Forward Smash 3"],
      ["attacks42", "Forward Smash 2"],
      ["attacks43", "Forward Smash 3"],
      ["attackhi4", "Up Smash"],
      ["attackhi4hold", "Up Smash (Charge)"],
      ["attacklw4", "Down Smash"],
      ["attacklw4hold", "Down Smash (Charge)"],
      ["slideattackhi", "Slide Up Attack"],
      ["slideattacklw", "Slide Down Attack"],
      ["slideattack", "Slide Neutral Attack"],
      ["aircatch", "Zair"],
      ['"0x1034e9490a"', "Nair (Hero)"] #Lucina specific bullshit!
      ["attackairn", "Nair"],
      ["attackairn2", "Nair 2"],
      ["attackairn3", "Nair 3"],
      ["landingairn", "Nair (Landing)"],
      ["attackairf", "Fair"],
      ["attackairf2", "Fair 2"],
      ["attackairf3", "Fair 3"],
      ["landingairf", "Fair (Landing)"],
      ["attackairb", "Bair"],
      ["landingairb", "Bair (Landing)"],
      ["attackairhi", "Uair"],
      ["landingairhi", "Uair (Landing)"],
      ["attackairlw", "Dair"],
      ["landingairlw", "Dair (Landing)"],
      ["cliffattack", "Ledge Attack"],
      ["downattacku", "Getup Attack (Face Up)"],
      ["downattackd", "Getup Attack (Face Down)"],
      ["throwf", "FThrow"],
      ["throwb", "BThrow"],
      ["throwhi", "Up Throw"],
      ["throwlw", "Down Throw"],
      ["throwff", "Cargo FThrow"],
      ["throwfb", "Cargo BThrow"],
      ["throwfhi", "Cargo Up Throw"],
      ["throwflw", "Cargo Down Throw"],
      ["catch", "Grab"],
      ["catchdash", "Dash Grab"],
      ["catchturn", "Pivot Grab"],
      ["catchattack", "Pummel"],
      ["specialinput", "Command Input"]
    ]
    special_list = [
      ["specialn", "Neutral Special "],
      ["specialairn", "(Air) Neutral Special "],
      ["specials", "Side Special "],
      ["specialairs", "(Air) Side Special "],
      ["specialhi", "Up Special "],
      ["specialairhi", "(Air) Up Special "],
      ["speciallw", "Down Special "],
      ["specialairlw", "(Air) Down Special "]
    ]
    string = string.lower()
    string = string.replace("game_", "")
    string = string.replace("_", "")
    string = string.replace("\n", "")
    for i in replace_list:
        if string == i[0]:
          return i[1]
    for i in special_list:
        if i[0] in string:
          string = string.replace(i[0], i[1])
          string = string.title()
          return string
    return string

try:
    char = (" ".join(sys.argv)).lower()
    char = char.replace('statistics.py', "")
    char = char.replace(' ', "")
except IndexError:
    char = input("Character?")

character = char.lower()

character = character.replace("mii gunner", "miigunner")
character = character.replace("mii brawler", "miifighter")
character = character.replace("mii sword", "miiswordsman")
character = character.replace("bowser jr", "koopajr")
character = character.replace("dark pit", "pitb")
character = character.replace("dr mario", "mariod")
character = character.replace("dark samus", "samusd")

has_replace = False

for i in replace:
    for f in i:
        if f in character:
            character = i[0]
            has_replace = True
            print(character)
            break
    if has_replace == True:
        break

if has_replace != True:
  raise Exception("Character not found! Did you misspell their name?")

my_projectiles = []
for i in projectile_list:
   if i[0] == character:
      my_projectiles = i
      my_projectiles.remove(character)
      print(my_projectiles)
      break

#["param", "Label", "value", "type", offset]
attribute_list = [
  ["dash_speed", "Initial Dash", 0.0, "float", 40],
  ["run_speed_max", "Run Speed", 0.0, "float", 43],
  ["walk_accel_max", "Walk Speed", 0.0, "float", 35],
  ["ground_brake", "Traction", 0.0, "float", 39],
  ["", "Wavedash Traction Category", "Average", "", 0],
  ["jump_y", "Full Hop Height", 0.0, "float", 49],
  ["mini_jump_y", "Short Hop Height", 0.0, "float", 50],
  ["air_speed_x_stable", "Air Speed", 0.0, "float", 54],
  ["air_accel_x_add", "Air Accel Add", 0.0, "float", 53],
  ["air_accel_x_mul", "Air Accel Mul", 0.0, "float", 52],
  ["air_speed_y_stable", "Fall Speed", 0.0, "float", 57],
  ["air_accel_y", "Gravity", 0.0, "float", 56],
  ["weight", "Weight", 0.0, "float", 62],
  ["landing_attack_air_frame_n", "Nair Landing Lag", 0.0, "float", 64],
  ["landing_attack_air_frame_f", "Fair Landing Lag", 0.0, "float", 65],
  ["landing_attack_air_frame_b", "Bair Landing Lag", 0.0, "float", 66],
  ["landing_attack_air_frame_hi", "Uair Landing Lag", 0.0, "float", 67],
  ["landing_attack_air_frame_lw", "Dair Landing Lag", 0.0, "float", 68],
  ["wall_jump_type", "Wall Jump?", "False", "bool", 352]
]
f = open("edited.csv",'w')
f.close()
f = open("data/default.xml")
default_param = f.readlines()
f.close()
os.chdir('../')
f = open("romfs/fighter/common/param/fighter_param.prcxml")
new_param = f.readlines()
f.close()
os.chdir('scripts')
os.chdir('../')
os.chdir(f'src/common')
f = open("wavedash.rs")
wavedash_rs = f.readlines()
f.close()
os.chdir('../')
os.chdir('../')
os.chdir('scripts')

#Checks fighter_param.prcxml
is_right_char = False
for i in new_param:
    if is_right_char == True:
      for a in range(0,4):
        if f'"{attribute_list[a][0]}' in i:
          x = i.replace(f'<{attribute_list[a][3]} hash="{attribute_list[a][0]}">', "")
          x = x.replace(f'</{attribute_list[a][3]}>', "")
          x = x.replace(f' ', "")
          x = x.replace(f'\n', "")
          attribute_list[a][2] = x
          #print("Running!")
      for a in range(5,19):
        if f'"{attribute_list[a][0]}' in i:
          x = i.replace(f'<{attribute_list[a][3]} hash="{attribute_list[a][0]}">', "")
          x = x.replace(f'</{attribute_list[a][3]}>', "")
          x = x.replace(f' ', "")
          x = x.replace(f'\n', "")
          attribute_list[a][2] = x
    if char.upper() in i:
      is_right_char = True 
      #print("Starting with character")
    elif "<!--" in i:
      is_right_char = False

#Checks default.xml
is_right_char = False
for f in range(0, len(default_param)):
    if f'<hash40 hash="fighter_kind">fighter_kind_{character.lower()}<' in default_param[f]:
      #print("OG Attribute check")
      for x in range(0,4):
        if attribute_list[x][2] == 0.0:
          #print(f"{attribute_list[x][1]} being replaced")
          new_str = default_param[f+attribute_list[x][4]]
          new_str = new_str.replace("float", "")
          new_str = new_str.replace("bool", "")
          new_str = new_str.replace("int", "")
          new_str = new_str.replace("hash", "")
          new_str = new_str.replace("/", "")
          new_str = new_str.replace("<", "")
          new_str = new_str.replace(">", "")
          new_str = new_str.replace(" ", "")
          new_str = new_str.replace("\n", "")
          new_str = new_str.replace("=", "")
          new_str = new_str.replace('"', "")
          new_str = new_str.replace('_', "")
          for w in ascii_lowercase:
            new_str = new_str.replace(w, "")
          attribute_list[x][2] = new_str
      for x in range(5,18):
        if attribute_list[x][2] == 0.0:
          print(f"{attribute_list[x][1]} being replaced")
          new_str = default_param[f+attribute_list[x][4]]
          new_str = new_str.replace("float", "")
          new_str = new_str.replace("bool", "")
          new_str = new_str.replace("int", "")
          new_str = new_str.replace("hash", "")
          new_str = new_str.replace("/", "")
          new_str = new_str.replace("<", "")
          new_str = new_str.replace(">", "")
          new_str = new_str.replace(" ", "")
          new_str = new_str.replace("\n", "")
          new_str = new_str.replace("=", "")
          new_str = new_str.replace('"', "")
          new_str = new_str.replace('_', "")
          for w in ascii_lowercase:
            new_str = new_str.replace(w, "")
          attribute_list[x][2] = new_str
        if "True" in default_param[f+attribute_list[18][4]]:
          attribute_list[18][2] = "True"
    elif f'<hash40 hash="fighter_kind">fighter_kind_' in default_param[f]:
      is_right_char = False


#Checks wavedash.rs
begin = -1
values = ["Highest", "High", "Low", "Lowest"]
adders = ["let max = [", "let high = [", "let low = [", "let min = ["]
name = f"*FIGHTER_KIND_{character.upper()},"
name2 = f"*FIGHTER_KIND_{character.upper()}\n"
for f in range(0, len(wavedash_rs)):
    for u in adders:
        if u in wavedash_rs[f]:
            begin += 1
    
    if begin > -1:
      if name in wavedash_rs[f] or name2 in wavedash_rs[f]:
        attribute_list[4][2] = values[begin]
        break
      

f = open("edited.csv", "a")
f.write("Attributes:\n")
for i in attribute_list:
  f.write(f"{i[1]} , {i[2]}\n")

original_attr = ["Attributes:\n"]
for i in attribute_list:
  original_attr.append(f"{i[1]} , {i[2]}\n")
print(original_attr)
f.write("\nEdited Moves:\n")
f.close()     

os.chdir('../')
f = open(f"src/common/faf_change.rs")
the_faf = f.readlines()
f.close()
os.chdir(f'scripts')

start_record = False
for y in the_faf:
    curly = "{"
    if f"*FIGHTER_KIND_{character.upper()} " in y or f"*FIGHTER_KIND_{character.upper()}{curly}" in y:
        start_record = True
    elif "*FIGHTER_KIND" in y:
        start_record = False
    if start_record == True and "*FIGHTER_KIND" not in y:
        faf_isolate.append(y)

block3s = True
block4s = True 
        

#Checks Motion List in romfs
os.chdir('../')
#print(os.getcwd())
try:
    
    os.system(f'py yamlist.py "romfs/fighter/{character}/motion/body/c00/motion_list.bin"')
    f = open(f"romfs/fighter/{character}/motion/body/c00/motion_list.yml")
    yaml_output = f.readlines()
    f.close()
    os.remove(f"romfs/fighter/{character}/motion/body/c00/motion_list.yml")
    os.chdir(f'scripts')

    faf_printer = []
    for i in yaml_output:
      if "game_script:" in i:
        script = i.replace("game_script:","")
        script = script.replace(" ","")
        script = script.replace("\t","")
        if "game_attacks3s" in script:
           block3s = False
        if "game_attacks4s" in script:
           block4s = False
        faf_printer = [script]
      if "cancel_frame:" in i:
        faf = i.replace("cancel_frame:","")
        faf = faf.replace(" ","")
        faf = faf.replace("\t","")
        faf_printer.append(int(faf))
        yaml_faf.append(faf_printer)

except Exception:
    print("No motion_list to extract") 
    yaml_faf = []
    os.chdir(f'scripts')


name = []
for i in faf_isolate:
   if "hash40" in i:
      new_list = i.replace("].contains(&motion_kind)", "")
      new_list = new_list.replace("\t", "")
      new_list = new_list.replace(" ", "")
      new_list = new_list.replace("hash40(", "")
      new_list = new_list.replace("if", "")
      new_list = new_list.replace("==", "")
      new_list = new_list.replace("motion_kind", "")
      new_list = new_list.replace("{", "")
      new_list = new_list.replace('"', "")
      new_list = new_list.replace("\n", "")
      new_list = new_list.replace("[", "")
      new_list = new_list.replace(")", "")
      if block3s:
        new_list = new_list.replace("3_s", "3")
      if block4s:
        new_list = new_list.replace("4_s", "4")
      new_list = new_list.split("&&")[0]
      new_list = new_list.split(",")
      for x in range(0,len(new_list)):
         entry = new_list[x]
         entry = entry.replace("_", "")
         new_list[x] = f"game_{entry}"
      name = new_list
      #print(f"{new_list}")
   if ("frame >=" in i or "new_cancel" in i) and not ("set_rate" in i or "else" in i):
      if not ("new_cancel" in i):
        new_list = i
        if "&&" in new_list:
          new_list = new_list.split("&&")[1]
        new_list = new_list.split("/*")[0]
        new_list = new_list.split("//")[0]
        new_list = new_list.replace("frame >", "")
        new_list = new_list.replace("=", "")
        new_list = new_list.replace(" ", "")
        new_list = new_list.replace("\t", "")
        new_list = new_list.replace("if", "")
        new_list = new_list.replace("motion_kind", "")
        new_list = new_list.replace("\n", "")
        new_list = new_list.replace("{", "")
        new_list = new_list.replace(";", "")
        new_list = float(new_list)
      else:
        new_list = i.replace(" ", "")
        new_list = new_list.split("/*")[0]
        new_list = new_list.split("//")[0]
        new_list = new_list.replace("letnew_cancel=", "")
        new_list = new_list.replace(";", "")
        new_list = new_list.replace("\t", "")
        new_list = float(new_list)
      #print(new_list)
      for x in name:
         for y in yaml_faf:
            #print(y)
            if y[0].replace("\n","") == x:
               y[1] = new_list
               #print(y)
      name = []


print(os.getcwd())


if not os.path.isdir(f'src/{character}'):
    os.chdir('../')
    reset_dir = os.getcwd()
    to_open_list = ["acmd/aerials.rs", "acmd/ground.rs", "acmd/other.rs", "acmd/specials.rs", "acmd/throws.rs", "acmd/tilts.rs"]
    for i in to_open_list:
      os.chdir(reset_dir)
      print(i)
      if os.path.isdir(f'src/{character}'):
        os.chdir(f'src/{character}')
        f = open(i)
        rs = f.readlines()
        f.close()
      else:
        os.chdir(f'src/common')
        rs = []
      os.chdir('../')
      os.chdir('../')
      os.chdir('scripts')

      #Game Scripts
      frame = 0.0
      wait_frames = 0.0
      last_frame_check = 0.0
      enable_cancel_frame = 0.0
      hit_times = 0
      script_name = ""
      game_script_name = ""
      motion_rate = 1.0
      autocancel_start = 0.0
      autocancel_end = 0.0
      additional_info = []
      has_hitstun = False
      throw_stats = []

      not_game = ["eff", "sound", "snd", "expr", "_s ", "_s(", "_e ", "_e("]

      for line_ in rs:
        line = line_.replace("(agent", "(fighter")
        #print(line)
        if "#[acmd_script(" in line or "pub fn install() {" in line:
          if len(additional_info) == 0:
            additional_info.append("\n")
          if yaml_faf != []:
            for x in yaml_faf:
              checked_val = x[0].replace("\n","")
              #print(f"{game_script_name} - {checked_val}")
              if game_script_name == checked_val:
                  faf = int(x[1])
                  faf = int(math.ceil(frame + ((float(faf) - last_frame_check) * motion_rate) ))
                  if enable_cancel_frame == 0.0:
                    if faf != 0 and faf < 200:
                      additional_info.append(f"FaF: {faf}\n")
                      #print(f"{game_script_name}: {faf}")
                    else:
                      additional_info.append(f"FaF: --\n")
                      #print(f"{game_script_name}: --")
                  else:
                      additional_info.append(f"FaF: {enable_cancel_frame}\n")
                      #print(f"{game_script_name}: {enable_cancel_frame}")
          if autocancel_start != autocancel_end:
            a_start = int(math.ceil(autocancel_start))-1
            if a_start > 1:
              additional_info.append(f"Autocancel - 1-{int(math.ceil(autocancel_start) )-1}/{int(math.ceil(autocancel_end) )}+")
            elif a_start > 0:
              additional_info.append(f"Autocancel - 1/{int(math.ceil(autocancel_end ))}+")
            else:
              additional_info.append(f"Autocancel - {int(math.ceil(autocancel_end ))}+")

          for i in additional_info:
            output.append(i)
          if script_name != "":
            output.append("\n\n")
          frame = 0.0
          wait_frames = 0.0
          enable_cancel_frame = 0.0
          hit_times = 0
          last_frame_check = 0.0
          game_script_name = ""
          motion_rate = 1.0
          autocancel_start = 0.0
          autocancel_end = 0.0
          has_hitstun = False
          additional_info = []
          throw_stats = []

        if "script" in line and not "use" in line:
          gamescript = line.replace(" ", "")
          gamescript = gamescript.replace("\t", "")
          gamescript = gamescript.replace("\n", "")
          gamescript = gamescript.split(",")
          scriptname = ""
          for i in gamescript:
              if "script" in i and not "#[acmd_script(" in i:
                scriptname = i
          scriptname = scriptname.replace(" ","")

          scriptname = scriptname.replace("scripts","")
          scriptname = scriptname.replace("script","")
          scriptname = scriptname.replace("=","")
          scriptname = scriptname.replace("[","")
          scriptname = scriptname.replace("]","")
          scriptname = scriptname.replace('"',"")
          game_script_name = scriptname

          
        game = True
        for i in not_game:
          if i in line:
            game = False
        if "(fighter: &mut L2CAgentBase)" in line and game == True and not "crate" in line:
          x = line
          x = line.replace("(fighter: &mut L2CAgentBase) {", "")
          x = x.replace("unsafe fn ", "")
          x = x.replace(" ", "")
          x = x.replace("\n", "")
          script_name = x
          output.append(make_printable(game_script_name))
          output.append("\nFrame,ID,Damage,Angle,BKB,KBG")
          already_edited.append(game_script_name)
        if "for _ in 0.." in line:
          x = line.replace("for _ in 0..", "")
          x = x.replace(" ", "")
          x = x.replace("{", "")
          x = x.replace("\n", "")
          x = x.replace("\t", "")
          x = x.replace(",", "")
          if x == "i32::MAX":
            x = 999999
          hit_times = int(x)


        if "frame(fighter.lua_state_agent" in line and not "//" in line:
          x = line.replace("frame(fighter.lua_state_agent", "")
          x = x.replace("/*Frames*/", "")
          x = x.replace("/*", "")
          x = x.replace("*/", "")
          x = x.replace(");", "")
          x = x.replace(")", "")
          x = x.replace(" ", "")
          x = x.replace("\n", "")
          x = x.replace("\t", "")
          x = x.replace(",", "")
          
          if "/" in x:
            x = x.split("/")
            x = float(x[0])/float(x[1])

          frame += (float(x) - last_frame_check) * motion_rate
          last_frame_check = float(x)
        if "wait(fighter.lua_state_agent" in line and not "//" in line:
          x = line.replace("wait(fighter.lua_state_agent,", "")
          x = x.replace(");", "")
          x = x.replace(")", "")
          x = x.replace(" ", "")
          x = x.replace("\n", "")
          x = x.replace("\t", "")
          x = x.replace(",", "")
          next_check = last_frame_check + float(x)
          frame += (next_check - last_frame_check) * motion_rate
          last_frame_check = next_check
        if "WorkModule::on_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
          autocancel_start = frame
          #print("Autocancel Off!")
        if "WorkModule::off_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
          autocancel_end = frame
          #print("Autocancel On!")
        if "CancelModule::enable_cancel(fighter.module_accessor);" in line and "//" not in line:
          enable_cancel_frame = math.ceil(frame)
          #print("Autocancel On!")
        if "FT_MOTION_RATE" in line and "//" not in line:
          x = line.replace("macros::FT_MOTION_RATE(fighter,", "")
          x = x.replace("/*FSM*/", "")
          x = x.replace(");", "")
          x = x.replace(" ", "")
          x = x.replace("\n", "")
          if "/" in x:
            x = x.split("/")
            x = float(x[0])/float(x[1])
          motion_rate = float(x)
        if "macros::ATTACK(fighter" in line:
          x = line.replace("macros::ATTACK(fighter, ", "")
          x = x.replace("/*Damage*/", "")
          x = x.replace("/*Angle*/", "")
          x = x.replace("/*BKB*/", "")
          x = x.replace("/*KBG*/", "")
          x = x.replace("/*FKB*/", "")
          x = x.split(", ")
          z = x[7]
          if int(x[6]) > 0:
            z = f"{x[6]} (Set)"

          atk_frame = int(math.ceil(frame))
          if atk_frame == 0:
            atk_frame = 1
          notes = [""]
          if hit_times > 0:
            notes.append(f"Hits {hit_times} times")

          #Notes!
          ground_air = x[28].replace("/*Ground_or_Air*/","")
          ground_air = ground_air.replace(" ","")
          if ground_air != "*COLLISION_SITUATION_MASK_GA":
            if ground_air == "*COLLISION_SITUATION_MASK_G":
              notes.append("Hits grounded only")
            elif ground_air == "*COLLISION_SITUATION_MASK_A":
              notes.append("Hits air only")
            elif ground_air == "*COLLISION_SITUATION_MASK_G_d":
              notes.append("Hits grounded only (not downed)")
            elif ground_air == "*COLLISION_SITUATION_MASK_GA_d":
              notes.append("Does not hit downed foes")
              
          effect = x[32].replace("/*Effect*/","")
          effect = effect.replace(" ","")
          effect = effect.replace("Hash40::new(","")
          effect = effect.replace("Hash40::new_raw(","")
          effect = effect.replace(")","")
          effect = effect.replace('"',"")
          effect = effect.replace('collision_attr_',"")
          effect = effect.replace(" ","")
          banned = ["normal","rush","magic","rush","cutup","sting","purple"]
          if effect not in banned:
            notes.append(f"hit effect is {effect}")

          
          if "true" in x[23]:
              notes.append("Reflectable")
          if "true" in x[24]:
              notes.append("Absorbable")
          if "true" in x[25]:
              notes.append("Flinchless")

          shield = x[20].replace("/*ShieldDamage*/","")
          shield = shield.replace(" ","")
          if shield != "0" and shield != "0.0":
            notes.append(f"deals {shield} bonus shield damage")

          rehit = x[22].replace("/*Rehit*/","")
          rehit = rehit.replace(" ","")
          if rehit != "0":
            notes.append(f"rehits every {rehit} frames")

          trip = x[21].replace("/*Trip*/","")
          trip = trip.replace(" ","")
          if trip != "0" and trip != "0.0":
            notes.append(f"{trip} bonus trip chance")
          id = x[0].replace(' ','')
          id = id.replace('/*ID*/','')
          id = id.replace('\t','')
          output.append(f"\nFrame {atk_frame}, {id}, {x[3]}%, {x[4]}, {z}, {x[5]}, Notes: {', '.join(notes)}")
        if "macros::ATTACK_ABS(fighter" in line:
          x = line.replace("macros::ATTACK_ABS(fighter, ", "")
          x = x.replace("/*Damage*/", "")
          x = x.replace("/*Angle*/", "")
          x = x.replace("/*BKB*/", "")
          x = x.replace("/*KBG*/", "")
          x = x.replace("/*FKB*/", "")
          x = x.replace("/*Kind*/", "")
          x = x.split(", ")
          if "THROW" in x[0]:
              throw_stats = x
        if "macros::ATK_HIT_ABS(fighter" in line:
          x = throw_stats
          z = x[6]
          if int(x[5]) > 0:
            z = f"{x[5]} (Set)"
      
          atk_frame = int(math.ceil(frame) )+1
          if atk_frame == 0:
            atk_frame = 1
          output.append(f"\nThrows on Frame {atk_frame}, , {x[2]}%, {x[3]}, {z}, {x[4]}")
          throw_stats = []
        if "macros::CATCH(" in line:
          atk_frame = int(math.ceil(frame) )+1
          if atk_frame == 0:
            atk_frame = 1
          output.append(f"\nGrabs on Frame {atk_frame}")
        if "ArticleModule::generate_article(" in line:
              atk_frame = int(math.ceil(frame) )+1
              article = line.replace("ArticleModule::generate_article(fighter.module_accessor,","")
              article = article.replace(" ","")
              article = article.replace(f"*FIGHTER_{character.upper()}_GENERATE_ARTICLE_","")
              article = article.split(",")[0]
              article = article.lower()
              article = article.replace("\t","")
              article = article.capitalize()
              check_article = article.lower()
              check_article = f"{character.lower()}_{check_article}"
              if check_article in my_projectiles:
                if atk_frame == 0:
                  atk_frame = 1
                output.append(f"\n{article} created on Frame {atk_frame}")
        if "grab!(" in line:
          atk_frame = int(math.ceil(frame) )+1
          if atk_frame == 0:
            atk_frame = 1
          output.append(f"\nGrabbox ceases on Frame {atk_frame}")
        if "AttackModule::clear_all(fighter.module_accessor);" in line:
          output.append(f"\nHitboxes terminated on Frame {int(math.ceil(frame) )}")
          hit_times = 0
        if "WorkModule::on_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD" in line:
          output.append(f"\nCharge hold Frame: {int(math.ceil(frame))}")
        if "AttackModule::set_add_reaction_frame(fighter.module_accessor," in line and has_hitstun == False:
          x = line.replace("AttackModule::set_add_reaction_frame(fighter.module_accessor,", "")
          x = x.replace("/*ID*/", "")
          x = x.replace("/*Unk*/", "")
          x = x.replace("/*Frames*/", "")
          x = x.replace("false);", "")
          x = x.replace("\t", "")
          x = x.replace("[", "")
          x = x.replace("\n", "")
          x = x.split(", ")
          #x = str(x).split(" ")
          has_hitstun = True
          try:
              additional_info.append(f"\nNote: Deals {int(math.ceil(float(x[1])))}f bonus hitstun\n")
          except ValueError:
              additional_info.append(f"\nNote: Deals _f bonus hitstun\n")
      #print(output)
      f = open("edited.csv", "a")
      for i in output:
        f.write(i)
      f.close()
      output = []
      #input()
f = open("edited.csv", "a")
f.write("\n\nVanilla\n")
f.close()
output = []

vanilla_directory = ""
curr_full_dir = os.getcwd()
if os.path.isfile("filepath.txt"):
   f = open("filepath.txt")
   vanilla_directory = f.readline()
   f.close()
   if not os.path.isdir(vanilla_directory):
      vanilla_directory = ""
if vanilla_directory == "":
    vanilla_directory = filedialog.askdirectory(title="Select your dumped vanilla smash scripts folder")
    f = open("filepath.txt", "w")
    f.write(vanilla_directory)
    f.close()

if os.path.isdir(f'{vanilla_directory}/smashline/lua2cpp_{character}/{character}/game'):
    os.chdir(f'{vanilla_directory}/smashline/lua2cpp_{character}/{character}/game')
    #print(os.listdir())
    include_list = [
       "attack",
       "special",
       "catch",
       "throw",
       "appeal"
    ]
    exclude_list = [
       "slip",
       "item"
    ]
    for current_file in os.listdir():
      if not current_file.endswith(".txt"):
          continue
      skip = True
      for b in include_list:
         if b in current_file.lower():
            skip = False
      for b in exclude_list:
         if b in current_file.lower():
            skip = True
      if skip:
        continue
      output.append("\n")
      f = open(current_file)
      rs = f.readlines()
      f.close()


      #Game Scripts
      frame = 0.0
      wait_frames = 0.0
      last_frame_check = 0.0
      hit_times = 0
      script_name = ""
      game_script_name = rs[1]
      game_script_name = game_script_name.replace("unsafe fn ","")
      game_script_name = game_script_name.replace("(agent: &mut L2CAgentBase) {","")
      game_script_name = game_script_name.replace("\n","")
      game_script_name = game_script_name.replace(" ","")
      motion_rate = 1.0
      autocancel_start = 0.0
      autocancel_end = 0.0
      additional_info = []
      has_hitstun = False
      throw_stats = []

      not_game = ["eff", "sound", "snd", "expr", "_s ", "_s(", "_e ", "_e("]

      for line_ in rs:
        line = line_.replace("(agent", "(fighter")
        #print(line)
        if line == "}":
          if len(additional_info) == 0:
            additional_info.append("\n")
          if yaml_faf != []:
            for x in yaml_faf:
              checked_val = x[0].replace("\n","")
              #print(f"{game_script_name} - {checked_val}")
              if game_script_name == checked_val:
                  faf = int(x[1])
                  faf = int(math.ceil(frame + ((float(faf) - last_frame_check) * motion_rate) ))
                  if enable_cancel_frame == 0.0:
                    if faf != 0 and faf < 200:
                      additional_info.append(f"FaF: {faf}\n")
                      #print(f"{game_script_name}: {faf}")
                    else:
                      additional_info.append(f"FaF: --\n")
                      #print(f"{game_script_name}: --")
                  else:
                      additional_info.append(f"FaF: {enable_cancel_frame}\n")
                      #print(f"{game_script_name}: {enable_cancel_frame}")
          if autocancel_start != autocancel_end:
            a_start = int(math.ceil(autocancel_start))-1
            if a_start > 1:
              additional_info.append(f"Autocancel - 1-{int(math.ceil(autocancel_start) )-1}/{int(math.ceil(autocancel_end) )}+")
            elif a_start > 0:
              additional_info.append(f"Autocancel - 1/{int(math.ceil(autocancel_end ))}+")
            else:
              additional_info.append(f"Autocancel - {int(math.ceil(autocancel_end ))}+")

          for i in additional_info:
            output.append(i)
          if script_name != "":
            output.append("\n\n")
          frame = 0.0
          wait_frames = 0.0
          enable_cancel_frame = 0.0
          last_frame_check = 0.0
          motion_rate = 1.0
          autocancel_start = 0.0
          autocancel_end = 0.0
          hit_times = 0
          has_hitstun = False
          additional_info = []
          throw_stats = []


          
        game = True
        for i in not_game:
          if i in line:
            game = False
        if "(fighter: &mut L2CAgentBase)" in line and game == True and not "crate" in line:
          x = line
          x = line.replace("(fighter: &mut L2CAgentBase) {", "")
          x = x.replace("unsafe fn ", "")
          x = x.replace(" ", "")
          x = x.replace("\n", "")
          script_name = x
          output.append(make_printable(game_script_name))
          output.append("\nFrame,ID,Damage,Angle,BKB,KBG")
          already_edited.append(game_script_name)
        if "for _ in 0.." in line:
          x = line.replace("for _ in 0..", "")
          x = x.replace(" ", "")
          x = x.replace("{", "")
          x = x.replace("\n", "")
          x = x.replace("\t", "")
          x = x.replace(",", "")
          if x == "i32::MAX":
            x = 999999
          hit_times = int(x)


        if "frame(fighter.lua_state_agent" in line and not "//" in line:
          x = line.replace("frame(fighter.lua_state_agent", "")
          x = x.replace("/*Frames*/", "")
          x = x.replace("/*", "")
          x = x.replace("*/", "")
          x = x.replace(");", "")
          x = x.replace(")", "")
          x = x.replace(" ", "")
          x = x.replace("\n", "")
          x = x.replace("\t", "")
          x = x.replace(",", "")
          if "/" in x:
            x = x.split("/")
            x = float(x[0])/float(x[1])
            
          frame += (float(x) - last_frame_check) * motion_rate
          last_frame_check = float(x)
        if "wait(fighter.lua_state_agent" in line and not "//" in line:
          x = line.replace("wait(fighter.lua_state_agent,", "")
          x = x.replace(");", "")
          x = x.replace(")", "")
          x = x.replace(" ", "")
          x = x.replace("\n", "")
          x = x.replace("\t", "")
          x = x.replace(",", "")
          next_check = last_frame_check + float(x)
          frame += (next_check - last_frame_check) * motion_rate
          last_frame_check = next_check
        if "WorkModule::on_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
          autocancel_start = frame
          #print("Autocancel Off!")
        if "WorkModule::off_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
          autocancel_end = frame
          #print("Autocancel On!")
        if "CancelModule::enable_cancel(fighter.module_accessor);" in line and "//" not in line:
          enable_cancel_frame = math.ceil(frame)
          #print("Autocancel On!")
        if "FT_MOTION_RATE" in line and "//" not in line:
          x = line.replace("macros::FT_MOTION_RATE(fighter,", "")
          x = x.replace("/*FSM*/", "")
          x = x.replace(");", "")
          x = x.replace(" ", "")
          motion_rate = float(x)
        if "macros::ATTACK(fighter" in line:
          x = line.replace("macros::ATTACK(fighter, ", "")
          x = x.replace("/*Damage*/", "")
          x = x.replace("/*Angle*/", "")
          x = x.replace("/*BKB*/", "")
          x = x.replace("/*KBG*/", "")
          x = x.replace("/*FKB*/", "")
          x = x.split(", ")
          z = x[7]
          if int(x[6]) > 0:
            z = f"{x[6]} (Set)"

          atk_frame = int(math.ceil(frame))
          if atk_frame == 0:
            atk_frame = 1
          notes = [""]
          if hit_times > 0:
            notes.append(f"Hits {hit_times} times")

          #Notes!
          ground_air = x[28].replace("/*Ground_or_Air*/","")
          ground_air = ground_air.replace(" ","")
          if ground_air != "*COLLISION_SITUATION_MASK_GA":
            if ground_air == "*COLLISION_SITUATION_MASK_G":
              notes.append("Hits grounded only")
            elif ground_air == "*COLLISION_SITUATION_MASK_A":
              notes.append("Hits air only")
            elif ground_air == "*COLLISION_SITUATION_MASK_G_d":
              notes.append("Hits grounded only (not downed)")
            elif ground_air == "*COLLISION_SITUATION_MASK_GA_d":
              notes.append("Does not hit downed foes")
              
          effect = x[32].replace("/*Effect*/","")
          effect = effect.replace(" ","")
          effect = effect.replace("Hash40::new(","")
          effect = effect.replace("Hash40::new_raw(","")
          effect = effect.replace(")","")
          effect = effect.replace('"',"")
          effect = effect.replace('collision_attr_',"")
          effect = effect.replace(" ","")
          banned = ["normal","rush","magic","rush","cutup","sting","purple"]
          if effect not in banned:
            notes.append(f"hit effect is {effect}")

          
          if "true" in x[23]:
              notes.append("Reflectable")
          if "true" in x[24]:
              notes.append("Absorbable")
          if "true" in x[25]:
              notes.append("Flinchless")

          shield = x[20].replace("/*ShieldDamage*/","")
          shield = shield.replace(" ","")
          if shield != "0" and shield != "0.0":
            notes.append(f"deals {shield} bonus shield damage")

          rehit = x[22].replace("/*Rehit*/","")
          rehit = rehit.replace(" ","")
          if rehit != "0":
            notes.append(f"rehits every {rehit} frames")

          trip = x[21].replace("/*Trip*/","")
          trip = trip.replace(" ","")
          if trip != "0" and trip != "0.0":
            notes.append(f"{trip} bonus trip chance")
          id = x[0].replace(' ','')
          id = id.replace('/*ID*/','')
          id = id.replace('\t','')
          output.append(f"\nFrame {atk_frame}, {id}, {x[3]}%, {x[4]}, {z}, {x[5]}, Notes: {', '.join(notes)}")
        if "macros::ATTACK_ABS(fighter" in line:
          x = line.replace("macros::ATTACK_ABS(fighter, ", "")
          x = x.replace("/*Damage*/", "")
          x = x.replace("/*Angle*/", "")
          x = x.replace("/*BKB*/", "")
          x = x.replace("/*KBG*/", "")
          x = x.replace("/*FKB*/", "")
          x = x.replace("/*Kind*/", "")
          x = x.split(", ")
          if "THROW" in x[0]:
              throw_stats = x
        if "macros::ATK_HIT_ABS(fighter" in line and len(throw_stats) >= 7:
          x = throw_stats
          z = x[6]
          if int(x[5]) > 0:
            z = f"{x[5]} (Set)"
      
          atk_frame = int(math.ceil(frame) )+1
          if atk_frame == 0:
            atk_frame = 1
          output.append(f"\nThrows on Frame {atk_frame}, , {x[2]}%, {x[3]}, {z}, {x[4]}")
          throw_stats = []
        if "macros::CATCH(" in line:
          atk_frame = int(math.ceil(frame) )+1
          if atk_frame == 0:
            atk_frame = 1
          output.append(f"\nGrabs on Frame {atk_frame}")
        if "ArticleModule::generate_article(" in line:
          atk_frame = int(math.ceil(frame) )+1
          article = line.replace("ArticleModule::generate_article(fighter.module_accessor,","")
          article = article.replace(" ","")
          article = article.replace(f"*FIGHTER_{character.upper()}_GENERATE_ARTICLE_","")
          article = article.split(",")[0]
          article = article.lower()
          article = article.replace("\t","")
          article = article.capitalize()
          if atk_frame == 0:
            atk_frame = 1
          output.append(f"\n{article} created on Frame {atk_frame}")
        if "grab!(" in line:
          atk_frame = int(math.ceil(frame) )+1
          if atk_frame == 0:
            atk_frame = 1
          output.append(f"\nGrabbox ceases on Frame {atk_frame}")
        if "AttackModule::clear_all(fighter.module_accessor);" in line:
          output.append(f"\nHitboxes terminated on Frame {int(math.ceil(frame) )}")
          hit_times = 0
        if "WorkModule::on_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD" in line:
          output.append(f"\nCharge hold Frame: {int(math.ceil(frame))}")
        if "AttackModule::set_add_reaction_frame(fighter.module_accessor," in line and has_hitstun == False:
          x = line.replace("AttackModule::set_add_reaction_frame(fighter.module_accessor,", "")
          x = x.replace("/*ID*/", "")
          x = x.replace("/*Unk*/", "")
          x = x.replace("/*Frames*/", "")
          x = x.replace("false);", "")
          x = x.replace("\t", "")
          x = x.replace("[", "")
          x = x.replace("\n", "")
          x = x.split(", ")
          #x = str(x).split(" ")
          has_hitstun = True
          try:
              additional_info.append(f"\nNote: Deals {int(math.ceil(float(x[1])))}f bonus hitstun\n")
          except ValueError:
              additional_info.append(f"\nNote: Deals _f bonus hitstun\n")
os.chdir(reset_dir)
os.chdir("scripts")
f = open("edited.csv", "a")
for i in output:
    f.write(i)
f.write("\n\n\n\n\n")
f.close()
#input(output)

move_order = [
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
banned = []
lists = []

has_been_done = []
done_order = 0
for i in range (0, len(move_order)):
   has_been_done.append(False)

f = open("edited.csv")
edit_csv = f.readlines()
f.close()
for i in range(0, 20):
   lists.append(edit_csv[i])

lists.append("\n")

for i in move_order:
  add_to = False
  now_in_vanilla = False
  for w in range(0, len(edit_csv)):
    if i in edit_csv[w]:
      if ("Frame" in edit_csv[w+2]) and not add_to:
        add_to = True
        has_been_done[done_order] = True
        lists.append("\n\n")
    if add_to:
      if len(edit_csv[w]) < 3:
        add_to = False
        break
      else:
        lists.append(edit_csv[w])
  if done_order < len(has_been_done)-1:
    done_order += 1

for i in specials:
  add_to = False
  now_in_vanilla = False
  for w in range(0, len(edit_csv)):
    next_ban = ""
    if "Vanilla" in edit_csv[w] and not now_in_vanilla:
       now_in_vanilla = True
       #print(f"{banned}")
    if i in edit_csv[w]:
      if ("Frame" in edit_csv[w+2]) and not add_to and not (edit_csv[w] in banned):
        if not now_in_vanilla:
          add_to = True
          banned.append(edit_csv[w])
          next_ban = edit_csv[w]
          lists.append("\n\n")
        else:
           name = edit_csv[w]#.replace("(Air) ","")
           if not (name in banned):
              #print(name)
              add_to = True
              next_ban = name
              banned.append(next_ban)
              next_ban = ""
              lists.append("\n\n")
    if add_to:
      if len(edit_csv[w]) < 3:
        add_to = False
        if next_ban != "":
          banned.append(next_ban)
          next_ban = ""
      else:
        lists.append(edit_csv[w])

output = []
os.chdir(curr_full_dir)
if not os.path.isdir(f'src/{character}'):
      os.chdir('../')
    #to_open_list = ["acmd/aerials.rs", "acmd/ground.rs", "acmd/other.rs", "acmd/specials.rs", "acmd/throws.rs", "acmd/tilts.rs"]
    #for i in to_open_list:
      os.chdir(reset_dir)
      if os.path.isdir(f'src/{character}'):
        os.chdir(f'src/{character}')
        f = open("mod.rs")
        rs = f.readlines()
        f.close()
      else:
        os.chdir(f'src/common')
        rs = []
      os.chdir('../')
      os.chdir('../')
      os.chdir('scripts')

      #Game Scripts
      frame = 0.0
      wait_frames = 0.0
      last_frame_check = 0.0
      enable_cancel_frame = 0.0
      hit_times = 0
      script_name = ""
      game_script_name = ""
      motion_rate = 1.0
      autocancel_start = 0.0
      autocancel_end = 0.0
      additional_info = []
      has_hitstun = False
      throw_stats = []

      not_game = ["eff", "sound", "snd", "expr", "_s ", "_s(", "_e ", "_e("]

      skip = True

      for line_ in rs:
        for projectiles in my_projectiles:
          if projectiles in line_ and "agent =" in line_ and "status" not in line_:
            output.append(f"\n\n{projectiles}\n")
            my_projectiles.remove(projectiles)
            skip = False
        
        if skip:
          continue

        line = line_.replace("(agent", "(fighter")
        #print(line)
        if "#[acmd_script(" in line or "pub fn install() {" in line:
          if len(additional_info) == 0:
            additional_info.append("\n")
          if yaml_faf != []:
            for x in yaml_faf:
              checked_val = x[0].replace("\n","")
              #print(f"{game_script_name} - {checked_val}")
              if game_script_name == checked_val:
                  faf = int(x[1])
                  faf = int(math.ceil(frame + ((float(faf) - last_frame_check) * motion_rate) ))
                  if enable_cancel_frame == 0.0:
                    if faf != 0 and faf < 200:
                      additional_info.append(f"FaF: {faf}\n")
                      #print(f"{game_script_name}: {faf}")
                    else:
                      additional_info.append(f"FaF: --\n")
                      #print(f"{game_script_name}: --")
                  else:
                      additional_info.append(f"FaF: {enable_cancel_frame}\n")
                      #print(f"{game_script_name}: {enable_cancel_frame}")
          if autocancel_start != autocancel_end:
            a_start = int(math.ceil(autocancel_start))-1
            if a_start > 1:
              additional_info.append(f"Autocancel - 1-{int(math.ceil(autocancel_start) )-1}/{int(math.ceil(autocancel_end) )}+")
            elif a_start > 0:
              additional_info.append(f"Autocancel - 1/{int(math.ceil(autocancel_end ))}+")
            else:
              additional_info.append(f"Autocancel - {int(math.ceil(autocancel_end ))}+")

          for i in additional_info:
            output.append(i)
          if script_name != "":
            output.append("\n\n")
          frame = 0.0
          wait_frames = 0.0
          enable_cancel_frame = 0.0
          hit_times = 0
          last_frame_check = 0.0
          game_script_name = ""
          motion_rate = 1.0
          autocancel_start = 0.0
          autocancel_end = 0.0
          has_hitstun = False
          additional_info = []
          skip = True
          throw_stats = []

        if "script" in line and not "use" in line:
          gamescript = line.replace(" ", "")
          gamescript = gamescript.replace("\t", "")
          gamescript = gamescript.replace("\n", "")
          gamescript = gamescript.split(",")
          scriptname = ""
          for i in gamescript:
              if "script" in i and not "#[acmd_script(" in i:
                scriptname = i
          scriptname = scriptname.replace(" ","")

          scriptname = scriptname.replace("scripts","")
          scriptname = scriptname.replace("script","")
          scriptname = scriptname.replace("=","")
          scriptname = scriptname.replace("[","")
          scriptname = scriptname.replace("]","")
          scriptname = scriptname.replace('"',"")
          game_script_name = scriptname

          
        game = True
        for i in not_game:
          if i in line:
            game = False
        if "(fighter: &mut L2CAgentBase)" in line and game == True and not "crate" in line:
          x = line
          x = line.replace("(fighter: &mut L2CAgentBase) {", "")
          x = x.replace("unsafe fn ", "")
          x = x.replace(" ", "")
          x = x.replace("\n", "")
          script_name = x
          output.append(make_printable(game_script_name))
          output.append("\nFrame,ID,Damage,Angle,BKB,KBG")
          already_edited.append(game_script_name)
        if "for _ in 0.." in line:
          x = line.replace("for _ in 0..", "")
          x = x.replace(" ", "")
          x = x.replace("{", "")
          x = x.replace("\n", "")
          x = x.replace("\t", "")
          x = x.replace(",", "")
          if x == "i32::MAX":
            x = 999999
          hit_times = int(x)


        if "frame(fighter.lua_state_agent" in line and not "//" in line:
          x = line.replace("frame(fighter.lua_state_agent", "")
          x = x.replace("/*Frames*/", "")
          x = x.replace("/*", "")
          x = x.replace("*/", "")
          x = x.replace(");", "")
          x = x.replace(")", "")
          x = x.replace(" ", "")
          x = x.replace("\n", "")
          x = x.replace("\t", "")
          x = x.replace(",", "")
          
          if "/" in x:
            x = x.split("/")
            x = float(x[0])/float(x[1])

          frame += (float(x) - last_frame_check) * motion_rate
          last_frame_check = float(x)
        if "wait(fighter.lua_state_agent" in line and not "//" in line:
          x = line.replace("wait(fighter.lua_state_agent,", "")
          x = x.replace(");", "")
          x = x.replace(")", "")
          x = x.replace(" ", "")
          x = x.replace("\n", "")
          x = x.replace("\t", "")
          x = x.replace(",", "")
          next_check = last_frame_check + float(x)
          frame += (next_check - last_frame_check) * motion_rate
          last_frame_check = next_check
        if "WorkModule::on_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
          autocancel_start = frame
          #print("Autocancel Off!")
        if "WorkModule::off_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
          autocancel_end = frame
          #print("Autocancel On!")
        if "CancelModule::enable_cancel(fighter.module_accessor);" in line and "//" not in line:
          enable_cancel_frame = math.ceil(frame)
          #print("Autocancel On!")
        if "FT_MOTION_RATE" in line and "//" not in line:
          x = line.replace("macros::FT_MOTION_RATE(fighter,", "")
          x = x.replace("/*FSM*/", "")
          x = x.replace(");", "")
          x = x.replace(" ", "")
          x = x.replace("\n", "")
          if "/" in x:
            x = x.split("/")
            x = float(x[0])/float(x[1])
          motion_rate = float(x)
        if "macros::ATTACK(fighter" in line:
          x = line.replace("macros::ATTACK(fighter, ", "")
          x = x.replace("/*Damage*/", "")
          x = x.replace("/*Angle*/", "")
          x = x.replace("/*BKB*/", "")
          x = x.replace("/*KBG*/", "")
          x = x.replace("/*FKB*/", "")
          x = x.split(", ")
          z = x[7]
          if int(x[6]) > 0:
            z = f"{x[6]} (Set)"

          atk_frame = int(math.ceil(frame))
          if atk_frame == 0:
            atk_frame = 1
          notes = [""]
          if hit_times > 0:
            notes.append(f"Hits {hit_times} times")

          #Notes!
          ground_air = x[28].replace("/*Ground_or_Air*/","")
          ground_air = ground_air.replace(" ","")
          if ground_air != "*COLLISION_SITUATION_MASK_GA":
            if ground_air == "*COLLISION_SITUATION_MASK_G":
              notes.append("Hits grounded only")
            elif ground_air == "*COLLISION_SITUATION_MASK_A":
              notes.append("Hits air only")
            elif ground_air == "*COLLISION_SITUATION_MASK_G_d":
              notes.append("Hits grounded only (not downed)")
            elif ground_air == "*COLLISION_SITUATION_MASK_GA_d":
              notes.append("Does not hit downed foes")
              
          effect = x[32].replace("/*Effect*/","")
          effect = effect.replace(" ","")
          effect = effect.replace("Hash40::new(","")
          effect = effect.replace("Hash40::new_raw(","")
          effect = effect.replace(")","")
          effect = effect.replace('"',"")
          effect = effect.replace('collision_attr_',"")
          effect = effect.replace(" ","")
          banned = ["normal","rush","magic","rush","cutup","sting","purple"]
          if effect not in banned:
            notes.append(f"hit effect is {effect}")

          
          if "true" in x[23]:
              notes.append("Reflectable")
          if "true" in x[24]:
              notes.append("Absorbable")
          if "true" in x[25]:
              notes.append("Flinchless")

          shield = x[20].replace("/*ShieldDamage*/","")
          shield = shield.replace(" ","")
          if shield != "0" and shield != "0.0":
            notes.append(f"deals {shield} bonus shield damage")

          rehit = x[22].replace("/*Rehit*/","")
          rehit = rehit.replace(" ","")
          if rehit != "0":
            notes.append(f"rehits every {rehit} frames")

          trip = x[21].replace("/*Trip*/","")
          trip = trip.replace(" ","")
          if trip != "0" and trip != "0.0":
            notes.append(f"{trip} bonus trip chance")
          id = x[0].replace(' ','')
          id = id.replace('/*ID*/','')
          id = id.replace('\t','')
          output.append(f"\nFrame {atk_frame}, {id}, {x[3]}%, {x[4]}, {z}, {x[5]}, Notes: {', '.join(notes)}")
        if "macros::ATTACK_ABS(fighter" in line:
          x = line.replace("macros::ATTACK_ABS(fighter, ", "")
          x = x.replace("/*Damage*/", "")
          x = x.replace("/*Angle*/", "")
          x = x.replace("/*BKB*/", "")
          x = x.replace("/*KBG*/", "")
          x = x.replace("/*FKB*/", "")
          x = x.replace("/*Kind*/", "")
          x = x.split(", ")
          if "THROW" in x[0]:
              throw_stats = x
        if "macros::ATK_HIT_ABS(fighter" in line:
          x = throw_stats
          z = x[6]
          if int(x[5]) > 0:
            z = f"{x[5]} (Set)"
      
          atk_frame = int(math.ceil(frame) )+1
          if atk_frame == 0:
            atk_frame = 1
          output.append(f"\nThrows on Frame {atk_frame}, , {x[2]}%, {x[3]}, {z}, {x[4]}")
          throw_stats = []
        if "macros::CATCH(" in line:
          atk_frame = int(math.ceil(frame) )+1
          if atk_frame == 0:
            atk_frame = 1
          output.append(f"\nGrabs on Frame {atk_frame}")
        if "ArticleModule::generate_article(" in line:
              atk_frame = int(math.ceil(frame) )+1
              article = line.replace("ArticleModule::generate_article(fighter.module_accessor,","")
              article = article.replace(" ","")
              article = article.replace(f"*FIGHTER_{character.upper()}_GENERATE_ARTICLE_","")
              article = article.split(",")[0]
              article = article.lower()
              article = article.replace("\t","")
              article = article.capitalize()
              check_article = article.lower()
              check_article = f"{character.lower()}_{check_article}"
              if check_article in my_projectiles:
                if atk_frame == 0:
                  atk_frame = 1
                output.append(f"\n{article} created on Frame {atk_frame}")
        if "grab!(" in line:
          atk_frame = int(math.ceil(frame) )+1
          if atk_frame == 0:
            atk_frame = 1
          output.append(f"\nGrabbox ceases on Frame {atk_frame}")
        if "AttackModule::clear_all(fighter.module_accessor);" in line:
          output.append(f"\nHitboxes terminated on Frame {int(math.ceil(frame) )}")
          hit_times = 0
        if "WorkModule::on_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD" in line:
          output.append(f"\nCharge hold Frame: {int(math.ceil(frame))}")
        if "AttackModule::set_add_reaction_frame(fighter.module_accessor," in line and has_hitstun == False:
          x = line.replace("AttackModule::set_add_reaction_frame(fighter.module_accessor,", "")
          x = x.replace("/*ID*/", "")
          x = x.replace("/*Unk*/", "")
          x = x.replace("/*Frames*/", "")
          x = x.replace("false);", "")
          x = x.replace("\t", "")
          x = x.replace("[", "")
          x = x.replace("\n", "")
          x = x.split(", ")
          #x = str(x).split(" ")
          has_hitstun = True
          try:
              additional_info.append(f"\nNote: Deals {int(math.ceil(float(x[1])))}f bonus hitstun\n")
          except ValueError:
              additional_info.append(f"\nNote: Deals _f bonus hitstun\n")
#input()

for projectiles in my_projectiles:
      os.chdir(f'{vanilla_directory}/smashline/lua2cpp_{character}/{projectiles}/game')
      output.append(f"\n\n{projectiles}\n")
      #print(os.listdir())
      for current_file in os.listdir():
        if not current_file.endswith(".txt"):
            continue
        output.append("\n")
        f = open(current_file)
        rs = f.readlines()
        f.close()


        #Game Scripts
        frame = 0.0
        wait_frames = 0.0
        last_frame_check = 0.0
        hit_times = 0
        script_name = ""
        game_script_name = rs[1]
        game_script_name = game_script_name.replace("unsafe fn ","")
        game_script_name = game_script_name.replace("(agent: &mut L2CAgentBase) {","")
        game_script_name = game_script_name.replace("\n","")
        game_script_name = game_script_name.replace(" ","")
        motion_rate = 1.0
        autocancel_start = 0.0
        autocancel_end = 0.0
        additional_info = []
        has_hitstun = False
        throw_stats = []

        not_game = ["eff", "sound", "snd", "expr", "_s ", "_s(", "_e ", "_e("]

        for line_ in rs:
          line = line_.replace("(agent", "(fighter")
          #print(line)
          if line == "}":
            if len(additional_info) == 0:
              additional_info.append("\n")
            if yaml_faf != []:
              for x in yaml_faf:
                checked_val = x[0].replace("\n","")
                #print(f"{game_script_name} - {checked_val}")
                if game_script_name == checked_val:
                    faf = int(x[1])
                    faf = int(math.ceil(frame + ((float(faf) - last_frame_check) * motion_rate) ))
                    if enable_cancel_frame == 0.0:
                      if faf != 0 and faf < 200:
                        additional_info.append(f"FaF: {faf}\n")
                        #print(f"{game_script_name}: {faf}")
                      else:
                        additional_info.append(f"FaF: --\n")
                        #print(f"{game_script_name}: --")
                    else:
                        additional_info.append(f"FaF: {enable_cancel_frame}\n")
                        #print(f"{game_script_name}: {enable_cancel_frame}")
            if autocancel_start != autocancel_end:
              a_start = int(math.ceil(autocancel_start))-1
              if a_start > 1:
                additional_info.append(f"Autocancel - 1-{int(math.ceil(autocancel_start) )-1}/{int(math.ceil(autocancel_end) )}+")
              elif a_start > 0:
                additional_info.append(f"Autocancel - 1/{int(math.ceil(autocancel_end ))}+")
              else:
                additional_info.append(f"Autocancel - {int(math.ceil(autocancel_end ))}+")

            for i in additional_info:
              output.append(i)
            if script_name != "":
              output.append("\n\n")
            frame = 0.0
            wait_frames = 0.0
            enable_cancel_frame = 0.0
            last_frame_check = 0.0
            motion_rate = 1.0
            autocancel_start = 0.0
            autocancel_end = 0.0
            hit_times = 0
            has_hitstun = False
            additional_info = []
            throw_stats = []


            
          game = True
          for i in not_game:
            if i in line:
              game = False
          if "(fighter: &mut L2CAgentBase)" in line and game == True and not "crate" in line:
            x = line
            x = line.replace("(fighter: &mut L2CAgentBase) {", "")
            x = x.replace("unsafe fn ", "")
            x = x.replace(" ", "")
            x = x.replace("\n", "")
            script_name = x
            output.append(make_printable(game_script_name))
            output.append("\nFrame,ID,Damage,Angle,BKB,KBG")
            already_edited.append(game_script_name)
          if "for _ in 0.." in line:
            x = line.replace("for _ in 0..", "")
            x = x.replace(" ", "")
            x = x.replace("{", "")
            x = x.replace("\n", "")
            x = x.replace("\t", "")
            x = x.replace(",", "")
            if x == "i32::MAX":
              x = 999999
            hit_times = int(x)


          if "frame(fighter.lua_state_agent" in line and not "//" in line:
            x = line.replace("frame(fighter.lua_state_agent", "")
            x = x.replace("/*Frames*/", "")
            x = x.replace("/*", "")
            x = x.replace("*/", "")
            x = x.replace(");", "")
            x = x.replace(")", "")
            x = x.replace(" ", "")
            x = x.replace("\n", "")
            x = x.replace("\t", "")
            x = x.replace(",", "")
            if "/" in x:
              x = x.split("/")
              x = float(x[0])/float(x[1])
              
            frame += (float(x) - last_frame_check) * motion_rate
            last_frame_check = float(x)
          if "wait(fighter.lua_state_agent" in line and not "//" in line:
            x = line.replace("wait(fighter.lua_state_agent,", "")
            x = x.replace(");", "")
            x = x.replace(")", "")
            x = x.replace(" ", "")
            x = x.replace("\n", "")
            x = x.replace("\t", "")
            x = x.replace(",", "")
            next_check = last_frame_check + float(x)
            frame += (next_check - last_frame_check) * motion_rate
            last_frame_check = next_check
          if "WorkModule::on_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
            autocancel_start = frame
            #print("Autocancel Off!")
          if "WorkModule::off_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
            autocancel_end = frame
            #print("Autocancel On!")
          if "CancelModule::enable_cancel(fighter.module_accessor);" in line and "//" not in line:
            enable_cancel_frame = math.ceil(frame)
            #print("Autocancel On!")
          if "FT_MOTION_RATE" in line and "//" not in line:
            x = line.replace("macros::FT_MOTION_RATE(fighter,", "")
            x = x.replace("/*FSM*/", "")
            x = x.replace(");", "")
            x = x.replace(" ", "")
            motion_rate = float(x)
          if "macros::ATTACK(fighter" in line:
            x = line.replace("macros::ATTACK(fighter, ", "")
            x = x.replace("/*Damage*/", "")
            x = x.replace("/*Angle*/", "")
            x = x.replace("/*BKB*/", "")
            x = x.replace("/*KBG*/", "")
            x = x.replace("/*FKB*/", "")
            x = x.split(", ")
            z = x[7]
            if int(x[6]) > 0:
              z = f"{x[6]} (Set)"

            atk_frame = int(math.ceil(frame))
            if atk_frame == 0:
              atk_frame = 1
            notes = [""]
            if hit_times > 0:
              notes.append(f"Hits {hit_times} times")

            #Notes!
            ground_air = x[28].replace("/*Ground_or_Air*/","")
            ground_air = ground_air.replace(" ","")
            if ground_air != "*COLLISION_SITUATION_MASK_GA":
              if ground_air == "*COLLISION_SITUATION_MASK_G":
                notes.append("Hits grounded only")
              elif ground_air == "*COLLISION_SITUATION_MASK_A":
                notes.append("Hits air only")
              elif ground_air == "*COLLISION_SITUATION_MASK_G_d":
                notes.append("Hits grounded only (not downed)")
              elif ground_air == "*COLLISION_SITUATION_MASK_GA_d":
                notes.append("Does not hit downed foes")
                
            effect = x[32].replace("/*Effect*/","")
            effect = effect.replace(" ","")
            effect = effect.replace("Hash40::new(","")
            effect = effect.replace("Hash40::new_raw(","")
            effect = effect.replace(")","")
            effect = effect.replace('"',"")
            effect = effect.replace('collision_attr_',"")
            effect = effect.replace(" ","")
            banned = ["normal","rush","magic","rush","cutup","sting","purple"]
            if effect not in banned:
              notes.append(f"hit effect is {effect}")

            
            if "true" in x[23]:
                notes.append("Reflectable")
            if "true" in x[24]:
                notes.append("Absorbable")
            if "true" in x[25]:
                notes.append("Flinchless")

            shield = x[20].replace("/*ShieldDamage*/","")
            shield = shield.replace(" ","")
            if shield != "0" and shield != "0.0":
              notes.append(f"deals {shield} bonus shield damage")

            rehit = x[22].replace("/*Rehit*/","")
            rehit = rehit.replace(" ","")
            if rehit != "0":
              notes.append(f"rehits every {rehit} frames")

            trip = x[21].replace("/*Trip*/","")
            trip = trip.replace(" ","")
            if trip != "0" and trip != "0.0":
              notes.append(f"{trip} bonus trip chance")
            id = x[0].replace(' ','')
            id = id.replace('/*ID*/','')
            id = id.replace('\t','')
            output.append(f"\nFrame {atk_frame}, {id}, {x[3]}%, {x[4]}, {z}, {x[5]}, Notes: {', '.join(notes)}")
          if "macros::ATTACK_ABS(fighter" in line:
            x = line.replace("macros::ATTACK_ABS(fighter, ", "")
            x = x.replace("/*Damage*/", "")
            x = x.replace("/*Angle*/", "")
            x = x.replace("/*BKB*/", "")
            x = x.replace("/*KBG*/", "")
            x = x.replace("/*FKB*/", "")
            x = x.replace("/*Kind*/", "")
            x = x.split(", ")
            if "THROW" in x[0]:
                throw_stats = x
          if "macros::ATK_HIT_ABS(fighter" in line:
            x = throw_stats
            z = x[6]
            if int(x[5]) > 0:
              z = f"{x[5]} (Set)"
        
            atk_frame = int(math.ceil(frame) )+1
            if atk_frame == 0:
              atk_frame = 1
            output.append(f"\nThrows on Frame {atk_frame}, , {x[2]}%, {x[3]}, {z}, {x[4]}")
            throw_stats = []
          if "macros::CATCH(" in line:
            atk_frame = int(math.ceil(frame) )+1
            if atk_frame == 0:
              atk_frame = 1
            output.append(f"\nGrabs on Frame {atk_frame}")
          if "ArticleModule::generate_article(" in line:
            atk_frame = int(math.ceil(frame) )+1
            article = line.replace("ArticleModule::generate_article(fighter.module_accessor,","")
            article = article.replace(" ","")
            article = article.replace(f"*FIGHTER_{character.upper()}_GENERATE_ARTICLE_","")
            article = article.split(",")[0]
            article = article.lower()
            article = article.replace("\t","")
            article = article.capitalize()
            check_article = article.lower()
            check_article = f"{character.lower()}_{check_article}"
            if check_article in my_projectiles:
              if atk_frame == 0:
                atk_frame = 1
              output.append(f"\n{article} created on Frame {atk_frame}")
          if "grab!(" in line:
            atk_frame = int(math.ceil(frame) )+1
            if atk_frame == 0:
              atk_frame = 1
            output.append(f"\nGrabbox ceases on Frame {atk_frame}")
          if "AttackModule::clear_all(fighter.module_accessor);" in line:
            output.append(f"\nHitboxes terminated on Frame {int(math.ceil(frame) )}")
            hit_times = 0
          if "WorkModule::on_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD" in line:
            output.append(f"\nCharge hold Frame: {int(math.ceil(frame))}")
          if "AttackModule::set_add_reaction_frame(fighter.module_accessor," in line and has_hitstun == False:
            x = line.replace("AttackModule::set_add_reaction_frame(fighter.module_accessor,", "")
            x = x.replace("/*ID*/", "")
            x = x.replace("/*Unk*/", "")
            x = x.replace("/*Frames*/", "")
            x = x.replace("false);", "")
            x = x.replace("\t", "")
            x = x.replace("[", "")
            x = x.replace("\n", "")
            x = x.split(", ")
            #x = str(x).split(" ")
            has_hitstun = True
            try:
                additional_info.append(f"\nNote: Deals {int(math.ceil(float(x[1])))}f bonus hitstun\n")
            except ValueError:
                additional_info.append(f"\nNote: Deals _f bonus hitstun\n")      

os.chdir(curr_full_dir)
f = open("output.csv", "w")
f.write("")
f.close()

for i in range (0,19):
  lists[i] = original_attr[i]
f = open("output.csv", "a")
for i in lists:
    f.write(i)
f.write("\n\nProjectiles:\n\n")
for i in output:
    f.write(i)
f.write("\n\n\n\n")
f.close()
#os.remove("edited.csv")
stats2md.run_stats2md()