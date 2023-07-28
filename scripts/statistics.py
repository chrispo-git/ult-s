import math
import os

rs = []
default_param = []
new_param = []
output = []

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


character = input("Character? ")
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

#["param", "Label", "value"]
attribute_list = [
  ["dash_speed", "Initial Dash", 0.0],
  ["run_speed_max", "Run Speed", 0.0],
  ["walk_accel_max", "Walk Speed", 0.0],
  ["ground_brake", "Traction", 0.0],
  ["", "Wavedash Traction Category", ""],
  ["jump_y", "Full Hop Height", 0.0],
  ["mini_jump_y", "Short Hop Height", 0.0],
  ["air_accel_x_stable", "Air Speed", 0.0],
  ["air_accel_x_add", "Air Accel Add", 0.0],
  ["air_accel_x_mul", "Air Accel Mul", 0.0],
  ["air_accel_y_stable", "Fall Speed", 0.0],
  ["air_accel_y", "Gravity", 0.0],
  ["weight", "Weight", 0.0],
  ["wall_jump_type", "Wall Jump?", False]
]
f = open("data/default.xml")
default_param = f.readlines()
f.close()
os.chdir('../')
f = open("romfs/fighter/common/param/fighter_param.prcxml")
new_param = f.readlines()
f.close()
os.chdir('scripts')

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
        x = x.replace(");", "")
        x = x.replace(" ", "")
        x = x.replace("\n", "")
        x = x.replace("\t", "")
        x = x.replace(",", "")
        if script_name == "mario_nair":
          print(f"new_frame:{frame + ((float(x) - last_frame_check) * motion_rate)}")
          
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
        print("Autocancel Off!")
      if "WorkModule::off_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
        autocancel_end = frame
        print("Autocancel On!")
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




f = open("output.csv",'w')
f.close()

f = open("output.csv", "a")
for i in output:
  f.write(i)
f.close()