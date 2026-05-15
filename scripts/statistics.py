import math
import os
import sys
import subprocess
import tempfile
from string import ascii_lowercase
import tkinter as tk
from tkinter import filedialog
import stats2md


char_directory = []
current_script = ""
rs = []
wavedash_rs = []
default_param = []
output = []
yaml_output = []
yaml_faf = []

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
    ['murabito', 'villager', 'villy'],
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
      ['"0x1034e9490a"', "Nair (Hero)"],
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


def parse_install_list(rs):
    """Return list of [script_name, fn_name] pairs from Agent::new acmd builder."""
    install_list = []
    for line in rs:
        stripped = line.replace(" ", "").replace("\t", "").replace("\n", "")
        if '.acmd("' in stripped or '.game_acmd("' in stripped or \
           '.effect_acmd("' in stripped or '.sound_acmd("' in stripped or \
           '.expression_acmd("' in stripped:
            for prefix in ['.acmd("', '.game_acmd("', '.effect_acmd("',
                           '.sound_acmd("', '.expression_acmd("']:
                stripped = stripped.replace(prefix, '\x00')
            if '\x00' not in stripped:
                continue
            parts = stripped.split('\x00', 1)[1]
            if '"' not in parts:
                continue
            script_name = parts.split('"')[0]
            rest = parts.split('"', 1)[1]
            rest = rest.lstrip(',').rstrip(')')
            fn_name = rest.split(',')[0].strip()
            script_name = _strip_char_prefixes(script_name)
            install_list.append([script_name, fn_name])
    return install_list


def _strip_char_prefixes(s):
    for p in ['toad', 'rayman', 'sandbag', 'maskedman']:
        s = s.replace(p, '', 1)
    if 'boom' in s:
        s = s.replace('boom', '', 1)
    else:
        s = s.replace('bomb', '', 1)
    return s


GITHUB_API = "https://api.github.com/repos/WuBoytH/SSBU-Dumped-Scripts/contents/smashline"

_vanilla_cache = {}

VANILLA_REPO_URL = "https://github.com/WuBoytH/SSBU-Dumped-Scripts"
VANILLA_REPO_NAME = "SSBU-Dumped-Scripts"

def _get_vanilla_dir():
    """Return the path to the local smashline folder inside SSBU-Dumped-Scripts.
    Clones the repo automatically if not already present, placing it next to the
    scripts/ folder. No user interaction required."""
    scripts_dir = os.path.dirname(os.path.abspath(__file__))
    repo_path = os.path.join(os.path.dirname(scripts_dir), VANILLA_REPO_NAME)
    smashline_path = os.path.join(repo_path, "smashline")

    if not os.path.isdir(repo_path):
        if os.path.exists(repo_path):
            import shutil
            shutil.rmtree(repo_path, ignore_errors=True)
        print(f"[vanilla] Cloning {VANILLA_REPO_URL} ...")
        result = subprocess.run(
            ["git", "clone", VANILLA_REPO_URL, repo_path],
            text=True
        )
        if result.returncode != 0 or not os.path.isdir(smashline_path):
            print(f"[vanilla] Clone failed (return code {result.returncode}).")
            print("[vanilla] Skipping vanilla pass.")
            if os.path.exists(repo_path):
                import shutil
                shutil.rmtree(repo_path, ignore_errors=True)
            return None
        print("[vanilla] Clone complete.")

    return smashline_path


def _read_dir_txt(dirpath, include_list, exclude_list):
    """Read matching .txt files from a local directory. Returns None if dir missing."""
    if not os.path.isdir(dirpath):
        return None
    result = {}
    for name in os.listdir(dirpath):
        if not name.endswith(".txt"):
            continue
        if include_list and not any(b in name.lower() for b in include_list):
            continue
        if any(b in name.lower() for b in exclude_list):
            continue
        with open(os.path.join(dirpath, name), encoding="utf-8", errors="replace") as fh:
            result[name] = fh.readlines()
    return result


def fetch_vanilla_files(character):
    """Read vanilla game scripts for a character from the local smashline folder."""
    global _vanilla_cache

    cache_key = f"vanilla_{character}"
    if cache_key in _vanilla_cache:
        return _vanilla_cache[cache_key]

    vanilla_dir = _get_vanilla_dir()
    if not vanilla_dir:
        print("[vanilla] No vanilla directory set, skipping vanilla pass.")
        _vanilla_cache[cache_key] = {}
        return {}

    include_list = ["attack", "special", "catch", "throw", "appeal"]
    exclude_list = ["slip", "item"]

    candidate_paths = [
        os.path.join(vanilla_dir, f"lua2cpp_{character}", character),
        os.path.join(vanilla_dir, character),
    ]

    result = {}
    for path in candidate_paths:
        r = _read_dir_txt(path, include_list, exclude_list)
        if r is not None:
            result = r
            print(f"[vanilla] Found {len(result)} files for {character} at {path}")
            break
        else:
            print(f"[vanilla] Could not find scripts for {character} under {vanilla_dir}")

        _vanilla_cache[cache_key] = result
    return result


def fetch_vanilla_projectile_files(character, projectile):
    """Read vanilla game scripts for a projectile from the local smashline folder."""
    vanilla_dir = _get_vanilla_dir()
    if not vanilla_dir:
        return {}

    candidate_paths = [
        os.path.join(vanilla_dir, f"lua2cpp_{character}", projectile),
        os.path.join(vanilla_dir, projectile),
    ]

    for path in candidate_paths:
        r = _read_dir_txt(path, [], [])
        if r is not None:
            print(f"[vanilla] Found {len(r)} files for projectile {projectile}")
            return r
    print(f"[vanilla] Could not find scripts for projectile {projectile}")
    return {}


def parse_motion_rate(line):
    x = line
    for tag in ["macros::FT_MOTION_RATE(fighter,", "macros::FT_MOTION_RATE(agent,"]:
        x = x.replace(tag, "")
    x = x.replace("/*FSM*/", "").replace(");", "").replace(" ", "").replace("\n", "").replace("\t", "")
    if "/" in x:
        parts = x.split("/")
        try:
            return float(parts[0]) / float(parts[1])
        except Exception:
            return 1.0
    try:
        return float(x)
    except Exception:
        return 1.0


def _offset_proj_lines(proj_lines, spawn_frame):
    """Take parsed projectile hitbox lines (e.g. "\nFrame 1, 0, 7.0%, ...")
    and add spawn_frame to each frame number, since the projectile detaches
    from the fighter at that point and its internal frames are relative."""
    import re
    result = []
    for line in proj_lines:
        def add_offset(m):
            return f"Frame {int(m.group(1)) + spawn_frame}"
        result.append(re.sub(r"Frame\s+(\d+)", add_offset, line))
    return result


def process_rs_lines(rs, character, yaml_faf, already_edited,
                     install_list=None, is_vanilla=False, projectile_data=None):
    """
    Parse lines of a Rust acmd file and return output list entries.
    install_list: list of [script_name, fn_name] from parse_install_list().
                  Pass None for vanilla files (they use the old fn-name-as-script convention).
    is_vanilla: if True, end-of-function is detected by a bare "}" line.
    """
    output = []

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
    pending_articles = {}

    not_game = ["eff", "sound", "snd", "expr", "_s ", "_s(", "_e ", "_e("]

    def flush_script():
        nonlocal frame, wait_frames, enable_cancel_frame, hit_times
        nonlocal last_frame_check, game_script_name, motion_rate
        nonlocal autocancel_start, autocancel_end, has_hitstun
        nonlocal additional_info, throw_stats, script_name, pending_articles

        if len(additional_info) == 0:
            additional_info.append("\n")
        if yaml_faf:
            for x in yaml_faf:
                checked_val = x[0].replace("\n", "")
                if game_script_name == checked_val:
                    if enable_cancel_frame == 0.0:
                        additional_info.append(f"FaF: {x[1]}\n")
                    else:
                        additional_info.append(f"FaF: {enable_cancel_frame}\n")
        if autocancel_start != autocancel_end:
            a_start = int(math.ceil(autocancel_start)) - 1
            if a_start > 1:
                additional_info.append(f"Autocancel - 1-{int(math.ceil(autocancel_start))-1}/{int(math.ceil(autocancel_end))}+")
            elif a_start > 0:
                additional_info.append(f"Autocancel - 1/{int(math.ceil(autocancel_end))}+")
            else:
                additional_info.append(f"Autocancel - {int(math.ceil(autocancel_end))}+")

        for art_key, art_frame in pending_articles.items():
            if projectile_data and art_key in projectile_data:
                for hline in _offset_proj_lines(projectile_data[art_key], art_frame - 1):
                    output.append(hline)
        pending_articles = {}

        for item in additional_info:
            output.append(item)
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
        pending_articles = {}

        return (frame, wait_frames, enable_cancel_frame, hit_times,
                last_frame_check, game_script_name, motion_rate,
                autocancel_start, autocancel_end, has_hitstun,
                additional_info, throw_stats)

    for line_ in rs:
        line = line_.replace("(agent", "(fighter")

        is_func_boundary = False
        if is_vanilla:
            if line == "}\n" or line == "}":
                is_func_boundary = True
        else:
            if 'unsafe extern "C" ' in line or "pub fn install() {" in line:
                is_func_boundary = True

        if is_func_boundary:
            (frame, wait_frames, enable_cancel_frame, hit_times,
             last_frame_check, game_script_name, motion_rate,
             autocancel_start, autocancel_end, has_hitstun,
             additional_info, throw_stats) = flush_script()
            pending_articles = {}
            script_name = ""

        if 'unsafe extern "C" fn' in line:
            fn_name = line.replace('unsafe extern "C" fn', "")
            for suffix in ["(fighter: &mut L2CAgentBase) {",
                           "(fighter: &mut L2CFighterCommon) {",
                           "(agent: &mut L2CAgentBase) {"]:
                fn_name = fn_name.replace(suffix, "")
            fn_name = fn_name.replace(" ", "").replace("{", "").replace("\t", "").replace("\n", "")

            if install_list is not None:
                scriptname = ""
                for entry in install_list:
                    if entry[1] == fn_name:
                        scriptname = entry[0]
                        break
                game_script_name = scriptname
            else:
                game_script_name = fn_name

        game = True
        for ng in not_game:
            if ng in line:
                game = False

        if "(fighter: &mut L2CAgentBase)" in line and game and "crate" not in line:
            x = line.replace("(fighter: &mut L2CAgentBase) {", "")
            x = x.replace("unsafe fn ", "").replace("unsafe extern \"C\" fn ", "")
            x = x.replace(" ", "").replace("\n", "")
            script_name = x
            output.append(make_printable(game_script_name))
            output.append("\nFrame,ID,Damage,Angle,BKB,KBG")
            already_edited.append(game_script_name)

        if "for _ in 0.." in line:
            x = line.replace("for _ in 0..", "").replace(" ", "").replace("{", "")
            x = x.replace("\n", "").replace("\t", "").replace(",", "")
            if x.strip() == "i32::MAX":
                x = "999999"
            try:
                hit_times = int(x.strip())
            except ValueError:
                pass

        for frame_fn in ["frame(fighter.lua_state_agent", "frame(agent.lua_state_agent"]:
            if frame_fn in line and "//" not in line:
                x = line.replace(frame_fn, "")
                x = x.replace("/*Frames*/", "").replace("/*", "").replace("*/", "")
                x = x.replace(");", "").replace(")", "").replace(" ", "")
                x = x.replace("\n", "").replace("\t", "").replace(",", "")
                if "/" in x:
                    parts = x.split("/")
                    try:
                        x = float(parts[0]) / float(parts[1])
                    except Exception:
                        x = 0.0
                try:
                    x = float(x)
                    frame += (x - last_frame_check) * motion_rate
                    last_frame_check = x
                except ValueError:
                    pass

        for wait_fn in ["wait(fighter.lua_state_agent,", "wait(agent.lua_state_agent,"]:
            if wait_fn in line and "//" not in line:
                x = line.replace(wait_fn, "")
                x = x.replace(");", "").replace(")", "").replace(" ", "")
                x = x.replace("\n", "").replace("\t", "").replace(",", "")
                try:
                    x = float(x)
                    next_check = last_frame_check + x
                    frame += (next_check - last_frame_check) * motion_rate
                    last_frame_check = next_check
                except ValueError:
                    pass

        if "WorkModule::on_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
            autocancel_start = frame
        if "WorkModule::off_flag(fighter.module_accessor" in line and "//" not in line and "*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING" in line:
            autocancel_end = frame
        if "CancelModule::enable_cancel(fighter.module_accessor);" in line and "//" not in line:
            enable_cancel_frame = math.ceil(frame)

        if "FT_MOTION_RATE" in line and "//" not in line:
            motion_rate = parse_motion_rate(line)

        for atk_prefix in ["macros::ATTACK(fighter, ", "macros::ATTACK(agent, "]:
            if atk_prefix in line:
                x = line.replace(atk_prefix, "")
                for tag in ["/*Damage*/", "/*Angle*/", "/*BKB*/", "/*KBG*/", "/*FKB*/"]:
                    x = x.replace(tag, "")
                x = x.split(", ")
                try:
                    z = x[7]
                    if int(x[6]) > 0:
                        z = f"{x[6]} (Set)"
                except (IndexError, ValueError):
                    z = "?"

                atk_frame = int(math.ceil(frame))
                if atk_frame == 0:
                    atk_frame = 1
                notes = [""]
                if hit_times > 0:
                    notes.append(f"Hits {hit_times} times")

                try:
                    ground_air = x[28].replace("/*Ground_or_Air*/", "").replace(" ", "")
                    if ground_air != "*COLLISION_SITUATION_MASK_GA":
                        if ground_air == "*COLLISION_SITUATION_MASK_G":
                            notes.append("Hits grounded only")
                        elif ground_air == "*COLLISION_SITUATION_MASK_A":
                            notes.append("Hits air only")
                        elif ground_air == "*COLLISION_SITUATION_MASK_G_d":
                            notes.append("Hits grounded only (not downed)")
                        elif ground_air == "*COLLISION_SITUATION_MASK_GA_d":
                            notes.append("Does not hit downed foes")
                except IndexError:
                    pass

                try:
                    effect = x[32].replace("/*Effect*/", "").replace(" ", "")
                    effect = effect.replace("Hash40::new(", "").replace("Hash40::new_raw(", "")
                    effect = effect.replace(")", "").replace('"', "")
                    effect = effect.replace("collision_attr_", "").replace(" ", "")
                    banned_effects = ["normal", "rush", "magic", "cutup", "sting", "purple"]
                    if effect not in banned_effects:
                        notes.append(f"hit effect is {effect}")
                except IndexError:
                    pass

                try:
                    if "true" in x[23]:
                        notes.append("Reflectable")
                    if "true" in x[24]:
                        notes.append("Absorbable")
                    if "true" in x[25]:
                        notes.append("Flinchless")
                except IndexError:
                    pass

                try:
                    shield = x[20].replace("/*ShieldDamage*/", "").replace(" ", "")
                    if shield not in ("0", "0.0"):
                        notes.append(f"deals {shield} bonus shield damage")
                except IndexError:
                    pass

                try:
                    rehit = x[22].replace("/*Rehit*/", "").replace(" ", "")
                    if rehit != "0":
                        notes.append(f"rehits every {rehit} frames")
                except IndexError:
                    pass

                try:
                    trip = x[21].replace("/*Trip*/", "").replace(" ", "")
                    if trip not in ("0", "0.0"):
                        notes.append(f"{trip} bonus trip chance")
                except IndexError:
                    pass

                try:
                    atk_id = x[0].replace(' ', '').replace('/*ID*/', '').replace('\t', '')
                    output.append(f"\nFrame {atk_frame}, {atk_id}, {x[3]}%, {x[4]}, {z}, {x[5]}, Notes: {', '.join(notes)}")
                except IndexError:
                    pass

        for abs_prefix in ["macros::ATTACK_ABS(fighter, ", "macros::ATTACK_ABS(agent, "]:
            if abs_prefix in line:
                x = line.replace(abs_prefix, "")
                for tag in ["/*Damage*/", "/*Angle*/", "/*BKB*/", "/*KBG*/", "/*FKB*/", "/*Kind*/"]:
                    x = x.replace(tag, "")
                x = x.split(", ")
                if "THROW" in x[0]:
                    throw_stats = x

        for hit_prefix in ["macros::ATK_HIT_ABS(fighter", "macros::ATK_HIT_ABS(agent"]:
            if hit_prefix in line and len(throw_stats) >= 7:
                x = throw_stats
                try:
                    z = x[6]
                    if int(x[5]) > 0:
                        z = f"{x[5]} (Set)"
                    atk_frame = int(math.ceil(frame)) + 1
                    if atk_frame == 0:
                        atk_frame = 1
                    output.append(f"\nThrows on Frame {atk_frame}, , {x[2]}%, {x[3]}, {z}, {x[4]}")
                except (IndexError, ValueError):
                    pass
                throw_stats = []

        for catch_pat in ["macros::CATCH(fighter", "macros::CATCH(agent"]:
            if catch_pat in line:
                atk_frame = int(math.ceil(frame)) + 1
                if atk_frame == 0:
                    atk_frame = 1
                output.append(f"\nGrabs on Frame {atk_frame}")

        if "ArticleModule::generate_article(" in line:
            atk_frame = int(math.ceil(frame)) + 1
            if atk_frame == 0:
                atk_frame = 1
            raw = line.replace("ArticleModule::generate_article(fighter.module_accessor,", "")
            raw = raw.replace(" ", "").replace("\t", "").replace("\n", "")
            raw = raw.replace(f"*FIGHTER_{character.upper()}_GENERATE_ARTICLE_", "")
            import re as _re
            art_key = _re.sub(r'[^A-Z0-9_]', '', raw.split(",")[0].upper())
            art_label = art_key.capitalize()
            if projectile_data and art_key in projectile_data:
                pending_articles[art_key] = atk_frame
            else:
                output.append(f"\n{art_label} created on Frame {atk_frame}")

        if "ArticleModule::shoot(" in line:
            raw = line.replace("ArticleModule::shoot(fighter.module_accessor,", "")
            raw = raw.replace(" ", "").replace("\t", "").replace("\n", "")
            raw = raw.replace(f"*FIGHTER_{character.upper()}_GENERATE_ARTICLE_", "")
            import re as _re
            art_key = _re.sub(r'[^A-Z0-9_]', '', raw.split(",")[0].upper())
            shoot_frame = int(math.ceil(frame)) + 1
            if shoot_frame == 0:
                shoot_frame = 1
            if art_key in pending_articles:
                gen_frame = pending_articles[art_key]
                if projectile_data and art_key in projectile_data:
                    for hline in _offset_proj_lines(projectile_data[art_key], gen_frame - 1):
                        output.append(hline)
                del pending_articles[art_key]
            else:
                output.append(f"\n{art_key.capitalize()} fired on Frame {shoot_frame}")

        for grab_pat in ["grab!(fighter", "grab!(agent"]:
            if grab_pat in line:
                atk_frame = int(math.ceil(frame)) + 1
                if atk_frame == 0:
                    atk_frame = 1
                output.append(f"\nGrabbox ceases on Frame {atk_frame}")

        if "AttackModule::clear_all(fighter.module_accessor);" in line or \
           "AttackModule::clear_all(agent.module_accessor);" in line:
            output.append(f"\nHitboxes terminated on Frame {int(math.ceil(frame))}")
            hit_times = 0

        if ("WorkModule::on_flag(fighter.module_accessor" in line or
                "WorkModule::on_flag(agent.module_accessor" in line) and \
                "//" not in line and "*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD" in line:
            output.append(f"\nCharge hold Frame: {int(math.ceil(frame))}")

        if ("AttackModule::set_add_reaction_frame(fighter.module_accessor," in line or
                "AttackModule::set_add_reaction_frame(agent.module_accessor," in line) \
                and not has_hitstun:
            x = line
            for tag in ["AttackModule::set_add_reaction_frame(fighter.module_accessor,",
                        "AttackModule::set_add_reaction_frame(agent.module_accessor,",
                        "/*ID*/", "/*Unk*/", "/*Frames*/", "false);"]:
                x = x.replace(tag, "")
            x = x.replace("\t", "").replace("[", "").replace("\n", "")
            x = x.split(", ")
            has_hitstun = True
            try:
                additional_info.append(f"\nNote: Deals {int(math.ceil(float(x[1])))}f bonus hitstun\n")
            except (ValueError, IndexError):
                additional_info.append(f"\nNote: Deals _f bonus hitstun\n")

    flush_script()

    return output


try:
    char = (" ".join(sys.argv)).lower()
    char = char.replace('statistics.py', "").replace(' ', "")
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
    if has_replace:
        break

replace_add = [
  ["bomberman", "pacman"],
  ["sandbag", "mariod"],
  ["toad", "murabito"],
  ["rayman", "pikmin"],
  ["masked", "lucas"]
]
is_add = False
new_char = ""
for i in replace_add:
    if i[0] in character:
        character = i[1]
        has_replace = True
        is_add = True
        new_char = i[0]
        print(new_char)
        break

if not has_replace:
    raise Exception("Character not found! Did you misspell their name?")

my_projectiles = []
for i in projectile_list:
    if i[0] == character:
        my_projectiles = i[1:]
        print(my_projectiles)
        break

attribute_list = [
  ["dash_speed", "Initial Dash", 0.0, "float", 40],
  ["run_speed_max", "Run Speed", 0.0, "float", 43],
  ["walk_accel_max", "Walk Speed", 0.0, "float", 35],
  ["ground_brake", "Traction", 0.0, "float", 39],
  ["dummy_yap", "Wavedash Traction Category", "Average", "", 0],
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

f = open("edited.csv", 'w')
f.close()

f = open("data/default.xml")
default_param = f.readlines()
f.close()

os.chdir('../')
os.chdir('src/common')
f = open("wavedash.rs")
wavedash_rs = f.readlines()
f.close()
os.chdir('../')
os.chdir('../')
os.chdir('scripts')

for f_idx in range(0, len(default_param)):
    if f'<hash40 hash="fighter_kind">fighter_kind_{character.lower()}<' in default_param[f_idx]:
        for x in range(0, 4):
            if attribute_list[x][2] == 0.0:
                new_str = default_param[f_idx + attribute_list[x][4]]
                for ch in ["float","bool","int","hash","/","<",">"," ","\n","=",'"',"_"]:
                    new_str = new_str.replace(ch, "")
                for w in ascii_lowercase:
                    new_str = new_str.replace(w, "")
                attribute_list[x][2] = new_str
        for x in range(5, 18):
            if attribute_list[x][2] == 0.0:
                print(f"{attribute_list[x][1]} being replaced")
                new_str = default_param[f_idx + attribute_list[x][4]]
                for ch in ["float","bool","int","hash","/","<",">"," ","\n","=",'"',"_"]:
                    new_str = new_str.replace(ch, "")
                for w in ascii_lowercase:
                    new_str = new_str.replace(w, "")
                attribute_list[x][2] = new_str
            if "True" in default_param[f_idx + attribute_list[18][4]]:
                attribute_list[18][2] = "True"

begin = -1
values = ["Highest", "High", "Low", "Lowest"]
adders = ["let max = [", "let high = [", "let low = [", "let min = ["]
name_wd = f"*FIGHTER_KIND_{character.upper()},"
name2_wd = f"*FIGHTER_KIND_{character.upper()}\n"
for f_idx in range(0, len(wavedash_rs)):
    for u in adders:
        if u in wavedash_rs[f_idx]:
            begin += 1
    if begin > -1:
        if name_wd in wavedash_rs[f_idx] or name2_wd in wavedash_rs[f_idx]:
            attribute_list[4][2] = values[begin]
            break

src_char = new_char if is_add else character
mod_rs_path = f'../src/{src_char}/mod.rs'
if os.path.isfile(mod_rs_path):
    with open(mod_rs_path) as fh:
        mod_rs_lines = fh.readlines()
    for line in mod_rs_lines:
        if "param_config::update_float_2" not in line and "param_config::update_int_2" not in line:
            continue
        is_int = "update_int_2" in line
        for x in range(0, len(attribute_list)):
            param_key = attribute_list[x][0]
            if f'hash40("{param_key}")' not in line:
                continue
            try:
                inner = line[line.rfind('(') + 1 : line.rfind(')')]
                new_val = inner.split(',')[-1].strip().rstrip(')')
                if is_int or attribute_list[x][3] == "int" or "landing" in param_key:
                    attribute_list[x][2] = int(float(new_val))
                else:
                    attribute_list[x][2] = float(new_val)
            except (ValueError, IndexError):
                pass

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
try:
    alt = "12" if is_add else "0"
    fighter_dir = new_char if is_add else character
    bin_path = f"romfs/fighter/{fighter_dir}/motion/body/c{alt}0/motion_list.bin"
    yml_path  = f"romfs/fighter/{fighter_dir}/motion/body/c{alt}0/motion_list.yml"

    wine_result = os.system(f'wine yamlist.exe disasm "{bin_path}" -l "Labels.txt" -o "{yml_path}"')

    if os.path.isfile(yml_path):
        with open(yml_path) as f_yml:
            yaml_output = f_yml.readlines()
        os.remove(yml_path)
        os.chdir('scripts')

        current_script = ""
        for i in yaml_output:
            if "game_script:" in i:
                script = i.replace("game_script:", "").replace(" ", "").replace("\t", "").replace("\n", "")
                if not is_add:
                    for pfx in ['toad', 'rayman', 'sandbag', 'maskedman']:
                        script = script.replace(pfx, "", 1)
                    if 'boom' in script:
                        script = script.replace('boom', "", 1)
                    else:
                        script = script.replace('bomb', "", 1)
                    if "game_attacks3s" in script:
                        block3s = False
                    if "game_attacks4s" in script:
                        block4s = False
                current_script = script
            if "cancel_frame:" in i and current_script:
                faf = int(i.replace("cancel_frame:", "").replace(" ", "").replace("\t", "").strip())
                if faf > 0:
                    yaml_faf.append([current_script, faf])
                current_script = ""
    else:
        print("No motion_list.yml produced, skipping FAF from yamlist")
        os.chdir('scripts')

except Exception as e:
    print(f"No motion_list to extract: {e}")
    yaml_faf = []
    if os.getcwd().endswith('scripts') is False:
        os.chdir('scripts')



print(os.getcwd())

projectile_data = {}
for projectile in my_projectiles:
    proj_files = fetch_vanilla_projectile_files(character, projectile)
    art_key = projectile.replace(f"{character}_", "").upper()
    proj_lines = []
    for fname, rs_proj in proj_files.items():
        proj_out = process_rs_lines(rs_proj, character, yaml_faf, [],
                                    install_list=None, is_vanilla=True)
        for line in proj_out:
            stripped = line.strip()
            if not stripped:
                continue
            if stripped.startswith("Frame,"):
                continue
            if stripped.startswith("##"):
                continue
            if not any(stripped.startswith(p) for p in (
                    "Frame ", "Throws", "Grabs", "Hitboxes", "FaF", "Note",
                    "Autocancel", "Charge")):
                continue
            proj_lines.append(line)
    if proj_lines:
        projectile_data[art_key] = proj_lines
        print(f"[projectile] Built data for {art_key}: {len(proj_lines)} lines")

if not os.path.isdir('src'):
    os.chdir('../')
reset_dir = os.getcwd()

to_open_list = [
    "acmd/aerials.rs", "acmd/ground.rs", "acmd/other.rs",
    "acmd/specials.rs", "acmd/throws.rs", "acmd/tilts.rs"
]

for rs_file in to_open_list:
    os.chdir(reset_dir)
    src_char = new_char if is_add else character
    if os.path.isdir(f'src/{src_char}'):
        os.chdir(f'src/{src_char}')
        try:
            with open(rs_file) as fh:
                rs = fh.readlines()
        except FileNotFoundError:
            rs = []
        os.chdir('../')
    else:
        os.chdir('src/common')
        rs = []
    os.chdir('../')
    os.chdir('scripts')

    install_list = parse_install_list(rs)
    print(f"install_list for {rs_file}: {install_list}")

    out = process_rs_lines(rs, character, yaml_faf, already_edited,
                           install_list=install_list, is_vanilla=False,
                           projectile_data=projectile_data)
    f = open("edited.csv", "a")
    for item in out:
        f.write(item)
    f.close()

f = open("edited.csv", "a")
f.write("\n\nVanilla\n")
f.close()
output = []

os.chdir(reset_dir)
os.chdir('scripts')

if not is_add:
    print(f"[vanilla] Fetching scripts for {character} from GitHub...")
    vanilla_files = fetch_vanilla_files(character)

    for fname, rs in vanilla_files.items():
        output.append("\n")

        gsn = ""
        if len(rs) > 1:
            gsn = rs[1].replace("unsafe fn ", "").replace("(agent: &mut L2CAgentBase) {", "")
            gsn = gsn.replace("\n", "").replace(" ", "")

        out = process_rs_lines(rs, character, yaml_faf, already_edited,
                               install_list=None, is_vanilla=True,
                               projectile_data=projectile_data)
        output.extend(out)

    os.chdir(reset_dir)
    os.chdir('scripts')
    f = open("edited.csv", "a")
    for item in output:
        f.write(item)
    f.write("\n\n\n\n\n")
    f.close()

move_order = [
   "Jab 1", "Jab 2", "Jab 3", "Rapid Jab", "Rapid Jab Finisher",
   "Jab 4","Jab 5","Jab 6","Jab 7","Jab 8","Jab 9",
   "FTilt","FTilt (Up)","FTilt (Down)","FTilt 2","FTilt 3",
   "UTilt","DTilt","Dash Attack",
   "Forward Smash","Forward Smash (Up)","Forward Smash (Down)",
   "Forward Smash (Charge)","Forward Smash 2","Forward Smash 3",
   "Up Smash","Up Smash (Charge)","Down Smash","Down Smash (Charge)",
   "Slide Up Attack","Slide Down Attack","Slide Neutral Attack","Zair",
   "Nair","Nair 2","Nair 3","Nair (Landing)",
   "Fair","Fair 2","Fair 3","Fair (Landing)",
   "Bair","Bair (Landing)","Uair","Uair (Landing)","Dair","Dair (Landing)",
   "Grab","Dash Grab","Pivot Grab","Pummel",
   "FThrow","BThrow","Up Throw","Down Throw",
   "Cargo FThrow","Cargo BThrow","Cargo Up Throw","Cargo Down Throw",
   "Command Input"
]
specials_order = ["Neutral Special","Side Special","Up Special","Down Special"]

banned_order = []
lists = []

has_been_done = [False] * len(move_order)
done_order = 0

f = open("edited.csv")
edit_csv = f.readlines()
f.close()


for i in range(0, 20):
    lists.append(edit_csv[i])
lists.append("\n")

for i in move_order:
    add_to = False
    for w in range(0, len(edit_csv)):
        if edit_csv[w].strip() == i:
            next2 = edit_csv[w + 2].strip() if w + 2 < len(edit_csv) else ""
            has_data = "Frame" in next2 or next2.startswith("Grabs") or next2.startswith("Throws")
            if w + 2 < len(edit_csv) and has_data and not add_to:
                add_to = True
                has_been_done[done_order] = True
                lists.append("\n\n")
        if add_to:
            if len(edit_csv[w]) < 3:
                if w + 1 < len(edit_csv) and len(edit_csv[w + 1]) < 3:
                    add_to = False
                    break
            else:
                lists.append(edit_csv[w])
    if done_order < len(has_been_done) - 1:
        done_order += 1

for i in specials_order:
    add_to = False
    now_in_vanilla = False
    next_ban = ""
    for w in range(0, len(edit_csv)):
        if "Vanilla" in edit_csv[w] and not now_in_vanilla:
            now_in_vanilla = True
        if i in edit_csv[w].strip():
            if w + 2 < len(edit_csv) and "Frame" in edit_csv[w + 2] and not add_to and edit_csv[w] not in banned_order:
                if not now_in_vanilla:
                    add_to = True
                    banned_order.append(edit_csv[w])
                    next_ban = edit_csv[w]
                    lists.append("\n\n")
                else:
                    name_e = edit_csv[w]
                    if name_e not in banned_order:
                        add_to = True
                        next_ban = name_e
                        banned_order.append(next_ban)
                        next_ban = ""
                        lists.append("\n\n")
        if add_to:
            if len(edit_csv[w]) < 3:
                add_to = False
                if next_ban:
                    banned_order.append(next_ban)
                    next_ban = ""
            else:
                lists.append(edit_csv[w])

output = []
os.chdir(reset_dir)

os.chdir('scripts')

os.chdir(reset_dir)
os.chdir('scripts')
f = open("output.csv", "w")
f.write("")
f.close()

for i in range(0, 19):
    lists[i] = original_attr[i]

f = open("output.csv", "a")
for i in lists:
    f.write(i)
f.write("\n\n\n\n")
f.close()

stats2md.run_stats2md()