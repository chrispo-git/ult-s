import math
import os
import sys
from string import ascii_lowercase

char_directory = []
current_script = ""
rs = []
wavedash_rs = []
default_param = []
new_param = []
output = []
yaml_output = []

replace = [
    ['bayonetta', 'bayo'],
    ['brave', 'hero'],
    ['buddy', 'banjo'],
    ['captain', 'falcon', 'cap'],
    ['chrom'],
    ['cloud'],
    ['daisy'],
    ['dedede', 'd3'],
    ['diddy'],
    ['dolly', 'terry'],
    ['donkey', 'dk'],
    ['duckhunt', 'duck'],
    ['edge', 'seph'],
    ['eflame', 'pyra'],
    ['elight', 'mythra'],
    ['gaogaen', 'incin'],
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
    ['nana'],
    ['ness'],
    ['packun', 'plant', 'piranha'],
    ['pacman', 'pac'],
    ['palutena', 'palu'],
    ['peach'],
    ['pfushigisou', 'ivy'],
    ['pichu'],
    ['pickel', 'steve'],
    ['pikachu', 'pika'],
    ['pikmin', 'olimar', 'alph'],
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

#["param", "Label", "value", "type", offset]
attribute_list = [
  ["dash_speed", "Initial Dash", 0.0, "float", 40],
  ["run_speed_max", "Run Speed", 0.0, "float", 43],
  ["walk_accel_max", "Walk Speed", 0.0, "float", 35],
  ["ground_brake", "Traction", 0.0, "float", 39],
  ["", "Wavedash Traction Category", "Average", "", 0],
  ["jump_y", "Full Hop Height", 0.0, "float", 49],
  ["mini_jump_y", "Short Hop Height", 0.0, "float", 50],
  ["air_accel_x_stable", "Air Speed", 0.0, "float", 54],
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
f = open("output.csv",'w')
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
          print("Running!")
      for a in range(5,19):
        if f'"{attribute_list[a][0]}' in i:
          x = i.replace(f'<{attribute_list[a][3]} hash="{attribute_list[a][0]}">', "")
          x = x.replace(f'</{attribute_list[a][3]}>', "")
          x = x.replace(f' ', "")
          x = x.replace(f'\n', "")
          attribute_list[a][2] = x
    if char.upper() in i:
      is_right_char = True 
      print("Starting with character")
    elif "<!--" in i:
      is_right_char = False

#Checks default.xml
is_right_char = False
for f in range(0, len(default_param)):
    if f'<hash40 hash="fighter_kind">fighter_kind_{character.lower()}<' in default_param[f]:
      print("OG Attribute check")
      for x in range(0,4):
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
      

f = open("output.csv", "a")
f.write("Attributes:\n")
for i in attribute_list:
  f.write(f"{i[1]} | {i[2]}\n")
f.write("\nEdited Moves:\n")
f.close()     

os.chdir('../')
if os.path.isdir(f'src/{character}'):
    os.chdir(f'src/{character}')
    f = open("mod.rs")
    rs = f.readlines()
    f.close()
    os.chdir('../')
    os.chdir('../')
    os.chdir('scripts')


    #Game Scripts
    frame = 0.0
    last_frame_check = 0.0
    script_name = ""
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
        if autocancel_start != autocancel_end:
          a_start = int(math.ceil(autocancel_start))-1
          if a_start > 1:
            additional_info.append(f"Autocancel - 1-{int(math.ceil(autocancel_start))-1}/{int(math.ceil(autocancel_end))}+")
          elif a_start > 0:
            additional_info.append(f"Autocancel - 1/{int(math.ceil(autocancel_end))}+")
          else:
            additional_info.append(f"Autocancel - {autocancel_end}+")
    
        for i in additional_info:
          output.append(i)
        if script_name != "":
          output.append("\n\n")
        frame = 0.0
        last_frame_check = 0.0
        motion_rate = 1.0
        autocancel_start = 0.0
        autocancel_end = 0.0
        has_hitstun = False
        additional_info = []
        throw_stats = []
    
      game = True
      for i in not_game:
        if i in line:
          game = False
      if "(fighter: &mut L2CAgentBase)" in line and game == True:
        x = line
        x = line.replace("(fighter: &mut L2CAgentBase) {", "")
        x = x.replace("unsafe fn ", "")
        x = x.replace(" ", "")
        x = x.replace("\n", "")
        script_name = x
        output.append(x)
      if "frame(fighter.lua_state_agent" in line:
        x = line.replace("frame(fighter.lua_state_agent", "")
        x = x.replace("/*Frames*/", "")
        x = x.replace(");", "")
        x = x.replace(" ", "")
        x = x.replace("\n", "")
        x = x.replace("\t", "")
        x = x.replace(",", "")
        #if script_name == "mario_nair":
          #print(f"new_frame:{frame + ((float(x) - last_frame_check) * motion_rate)}")
          
        frame += (float(x) - last_frame_check) * motion_rate
        last_frame_check = float(x)
      if "wait(fighter.lua_state_agent" in line:
        x = line.replace("wait(fighter.lua_state_agent,", "")
        x = x.replace(");", "")
        x = x.replace(" ", "")
        x = x.replace("\n", "")
        x = x.replace("\t", "")
        x = x.replace(",", "")
        frame += float(x) * motion_rate
      if "WorkModule::on_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
        autocancel_start = frame
        #print("Autocancel Off!")
      if "WorkModule::off_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
        autocancel_end = frame
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
        output.append(f"\nFrame {atk_frame}|Damage: {x[3]}%| Angle: {x[4]}| BKB: {z}| KBG: {x[5]}")
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
    
        atk_frame = int(math.ceil(frame))
        if atk_frame == 0:
          atk_frame = 1
        output.append(f"\nThrows on Frame {atk_frame}|Damage: {x[2]}%| Angle: {x[3]}| BKB: {z}| KBG: {x[4]}")
        throw_stats = []
      if "macros::CATCH(" in line:
        atk_frame = int(math.ceil(frame))
        if atk_frame == 0:
          atk_frame = 1
        output.append(f"\nGrabs on Frame {atk_frame}")
      if "grab!(" in line:
        atk_frame = int(math.ceil(frame))
        if atk_frame == 0:
          atk_frame = 1
        output.append(f"\nGrabbox ceases on Frame {atk_frame}")
      if "AttackModule::clear_all(fighter.module_accessor);" in line:
        output.append(f"\nHitboxes terminated on frame {int(math.ceil(frame))}")
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





f = open("output.csv", "a")
for i in output:
  f.write(i)  
f.write("\nVanilla Moves:\n")
f.close()

include_list = [
  "Attack1", "AttackDash", "AttackS3", "AttackHi3", "AttackLw3",
  "AttackS4", "AttackHi4", "AttackLw4", 
  "AttackAirN", "AttackAirF", "AttackAirB", "AttackAirHi", "AttackAirLw", "AirCatch",
  "SpecialN", "SpecialS", "SpecialHi", "SpecialLw",
  "Catch.", "CatchDash", "CatchTurn",
  "ThrowF", "ThrowB", "ThrowHi", "ThrowLw"
]

output = []
os.chdir('../')
os.chdir('../')
os.chdir('../')
os.chdir(f'Dumped_Scripts/smashline/lua2cpp_{character}/{character}/game')

char_directory = os.listdir()
for i in char_directory:
    is_included = False
    current_script = i
    for x in include_list:
      if x in current_script:
        is_included = True
    if is_included == False:
      pass
    f = open(current_script)
    rs = f.readlines()
    f.close()
    output.append(f"\n{current_script}\n")


    #Game Scripts
    frame = 0.0
    last_frame_check = 0.0
    script_name = ""
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
    
      game = True
      for i in not_game:
        if i in line:
          game = False
      if "(fighter: &mut L2CAgentBase)" in line and game == True:
        x = line
        x = line.replace("(fighter: &mut L2CAgentBase) {", "")
        x = x.replace("unsafe fn ", "")
        x = x.replace(" ", "")
        x = x.replace("\n", "")
        script_name = x
        #output.append(x)
      if "frame(fighter.lua_state_agent" in line:
        x = line.replace("frame(fighter.lua_state_agent", "")
        x = x.replace("/*Frames*/", "")
        x = x.replace(");", "")
        x = x.replace(" ", "")
        x = x.replace("\n", "")
        x = x.replace("\t", "")
        x = x.replace(",", "")
        #if script_name == "mario_nair":
          #print(f"new_frame:{frame + ((float(x) - last_frame_check) * motion_rate)}")
          
        frame += (float(x) - last_frame_check) * motion_rate
        last_frame_check = float(x)
      if "wait(fighter.lua_state_agent" in line:
        x = line.replace("wait(fighter.lua_state_agent,", "")
        x = x.replace(");", "")
        x = x.replace(" ", "")
        x = x.replace("\n", "")
        x = x.replace("\t", "")
        x = x.replace(",", "")
        frame += float(x) * motion_rate
      if "WorkModule::on_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
        autocancel_start = frame
        #print("Autocancel Off!")
      if "WorkModule::off_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
        autocancel_end = frame
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
        output.append(f"\nFrame {atk_frame}|Damage: {x[3]}%| Angle: {x[4]}| BKB: {z}| KBG: {x[5]}")
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
    
        atk_frame = int(math.ceil(frame))
        if atk_frame == 0:
          atk_frame = 1
        output.append(f"\nThrows on Frame {atk_frame}|Damage: {x[2]}%| Angle: {x[3]}| BKB: {z}| KBG: {x[4]}")
        throw_stats = []
      if "macros::CATCH(" in line:
        atk_frame = int(math.ceil(frame))
        if atk_frame == 0:
          atk_frame = 1
        output.append(f"\nGrabs on Frame {atk_frame}")
      if "grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL" in line:
        atk_frame = int(math.ceil(frame))
        if atk_frame == 0:
          atk_frame = 1
        output.append(f"\nGrabbox ceases on Frame {atk_frame}")
      if "AttackModule::clear_all(fighter.module_accessor);" in line:
        output.append(f"\nHitboxes terminated on frame {int(math.ceil(frame))}")
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
        additional_info.append(f"\nNote: Deals {int(math.ceil(float(x[1])))}f bonus hitstun\n")
    
    if True == True:
        if len(additional_info) == 0:
          additional_info.append("\n")
        if autocancel_start != autocancel_end:
          a_start = int(math.ceil(autocancel_start))-1
          if a_start > 1:
            additional_info.append(f"Autocancel - 1-{int(math.ceil(autocancel_start))-1}/{int(math.ceil(autocancel_end))}+")
          elif a_start > 0:
            additional_info.append(f"Autocancel - 1/{int(math.ceil(autocancel_end))}+")
          else:
            additional_info.append(f"Autocancel - {autocancel_end}+")
    
        for i in additional_info:
          output.append(i)
        if script_name != "":
          output.append("\n\n")
        frame = 0.0
        last_frame_check = 0.0
        motion_rate = 1.0
        autocancel_start = 0.0
        autocancel_end = 0.0
        has_hitstun = False
        throw_stats = []
        additional_info = []




os.chdir('../')
os.chdir('../')
os.chdir('../')
os.chdir('../')
os.chdir('../')
os.chdir(f'Ultimate S Smashline/ultimate-s/scripts')

f = open("output.csv", "a")
for i in output:
  f.write(i)
f.close()
os.chdir('../')
os.chdir('../')
os.chdir('../')
try:
    os.system(f'py yamlist.py "Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c00/motion_list.bin"')
    f = open(f"Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c00/motion_list.yml")
    yaml_output = f.readlines()
    f.close()
    os.remove(f"Ultimate S Smashline/ultimate-s/romfs/fighter/{character}/motion/body/c00/motion_list.yml")
    os.chdir(f'Ultimate S Smashline/ultimate-s/scripts')
    f = open("output.csv", "a")
    f.write("\nMotion List:\n")
    for i in yaml_output:
        f.write(i)
    f.close()
except Exception:
    print("No motion_list to extract")