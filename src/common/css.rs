
use crate::util::*;
use std::path::Path;
use std::fs::*;
use std::io::*;
#[cfg(feature = "main_nro")]
use skyline_web::dialog_ok::DialogOk;
static mut IS_UNPRESSED : bool = false;
#[skyline::hook(offset = 0x1792dc0, inline)]
unsafe fn on_rule_selection(_: &skyline::hooks::InlineCtx) {
    println!("We Are On Rule Select");
    if ninput::any::is_down(ninput::Buttons::R) {
        println!("R Pressed!");
        if !is_on_ryujinx() {
            show_gamemodes();
        } else {
            println!("Emu Mode!");
            show_gamemodes_emu();
        }
    } else {
        reset_gamemodes();
    }
}

#[skyline::hook(offset = 0x1a2b570)]
unsafe fn css_main_loop(arg: *const CharaSelect) {
    {
        if ninput::any::is_down(ninput::Buttons::MINUS) {
            if !IS_UNPRESSED {
                println!("Minus Pressed!");
                if !is_on_ryujinx() {
                    show_mod_settings();
                } else {
                    println!("Emu Mode!");
                    show_mod_settings_emu();
                }
            }
            IS_UNPRESSED = true;
        } else {
            IS_UNPRESSED = false;
        }
        original!()(arg)
    }
}

lazy_static! {
    static ref MENU_HTML: Vec<u8> = std::fs::read("mods:/ui/docs/menu.html").unwrap();
    static ref GAMEMODES_HTML: Vec<u8> = std::fs::read("mods:/ui/docs/gamemodes.html").unwrap();
    static ref MENU_JS: Vec<u8> = std::fs::read("mods:/ui/docs/menu.js").unwrap();
    static ref GRID_MENU_JS: Vec<u8> = std::fs::read("mods:/ui/docs/gridmenu.js").unwrap();
    static ref COMMON_JS: Vec<u8> = std::fs::read("mods:/ui/docs/common.js").unwrap();
    static ref COMMON_CSS: Vec<u8> = std::fs::read("mods:/ui/docs/common.css").unwrap();
    static ref MENU_CSS: Vec<u8> = std::fs::read("mods:/ui/docs/menu.css").unwrap();
    static ref TOGGLE_JS: Vec<u8> = std::fs::read("mods:/ui/docs/toggles.js").unwrap();
}
pub fn show_gamemodes_emu() {
    unsafe {
        reset_gamemodes();
        let path = "sd:/ultimate/ult-s/gamemode-default.txt";
        if !Path::new(path).exists() {
            let mut file = File::create(path);
            file.expect("Can't Write To Gamemodes!").write_all("airdash\nparry\nhitfall\nfgmode".as_bytes());
            println!("Default File Made");
        }
        let mut gamemodes = match std::fs::read_to_string(path) {
            Ok(gamemodes) => gamemodes.trim().replace("\n", "-").to_string(),
            Err(_) => {
                String::from("airdash-parry-hitfall-fgmode")
            }
        };
        if gamemodes.is_empty() {
            return;
        }
        let options = gamemodes.split("-");

        for i in options {
            println!("{}", i);
            unsafe {
                add_gamemode(i.to_string());
            }
        }
    }
}
pub fn show_gamemodes() {
    unsafe {
        reset_gamemodes();
    }
    let response = skyline_web::Webpage::new()
        .htdocs_dir("contents")
        .file("index.html", GAMEMODES_HTML.as_slice())
        .file("menu.css", MENU_CSS.as_slice())
        .file("gridmenu.js", GRID_MENU_JS.as_slice())
        .file("common.js", COMMON_JS.as_slice())
        .file("toggles.js", TOGGLE_JS.as_slice())
        .background(skyline_web::Background::Default)
        .boot_display(skyline_web::BootDisplay::Default)
        .open()
        .unwrap();
    match response.get_last_url() {
        Ok(url) => {
            let options_str = url.trim_start_matches("http://localhost/");
            println!("Options chosen: {}", options_str);
            if options_str.is_empty() {
                return;
            }

            let options = options_str.split("-");

            for i in options {
                println!("{}", i);
                unsafe {
                    add_gamemode(i.to_string());
                }
            }
        }
        Err(_) => {
            println!("Uh oh! Error getting options!");
        }
    }
	unsafe {
		update_enabled_checks();
	}
}
pub fn show_mod_settings_emu() {
    let path = "sd:/ultimate/ult-s/sys-flags/";
    let path1 = "sd:/ultimate/ult-s/";
    println!("Emulator Mod Settings.");
    let is_toggle_off = Path::new("sd:/ultimate/ult-s/sys-flags/mechanics.flag").is_file();
    println!("Is Toggle Off? {}", is_toggle_off);
    match std::fs::create_dir(path1) {
        Ok(_) => println!("ult-s Folder Created!"),
        Err(_) => {
            println!("ult-s folder already exists!");
        }
    }
    match std::fs::create_dir(path) {
        Ok(_) => println!("Settings Folder Created!"),
        Err(_) => {
            match std::fs::remove_dir_all(path) {
                Ok(_) => println!("Settings reset successfully!"),
                Err(_) => println!("Error resetting settings!")
            }
            std::fs::create_dir(path);
        }
    }
    if !is_toggle_off {
        std::fs::File::create(format!("{}mechanics.flag", path)).unwrap();
        std::fs::File::create(format!("{}sh.flag", path)).unwrap();
        println!("Toggling On!");
    } else {
        println!("Toggling Off!");
        match std::fs::remove_dir_all(path) {
            Ok(_) => println!("Settings reset successfully!"),
            Err(_) => println!("Error resetting settings!")
        }
    }
	unsafe {
		update_enabled_checks();
	}
}
pub fn show_mod_settings() {
    let path = "sd:/ultimate/ult-s/sys-flags/";
    let path1 = "sd:/ultimate/ult-s/";
    match std::fs::create_dir(path1) {
        Ok(_) => println!("ult-s Folder Created!"),
        Err(_) => {
            println!("ult-s folder already exists!");
        }
    }
    match std::fs::create_dir(path) {
        Ok(_) => println!("Settings Folder Created!"),
        Err(_) => {
            match std::fs::remove_dir_all(path) {
                Ok(_) => println!("Settings reset successfully!"),
                Err(_) => println!("Error resetting settings!")
            }
            std::fs::create_dir(path);
        }
    }

    let response = skyline_web::Webpage::new()
        .htdocs_dir("contents")
        .file("index.html", MENU_HTML.as_slice())
        .file("menu.css", MENU_CSS.as_slice())
        .file("menu.js", MENU_JS.as_slice())
        .file("common.js", COMMON_JS.as_slice())
        .file("toggles.js", TOGGLE_JS.as_slice())
        .background(skyline_web::Background::Default)
        .boot_display(skyline_web::BootDisplay::Default)
        .open()
        .unwrap();
    
    match response.get_last_url() {
        Ok(url) => {
            let options_str = url.trim_start_matches("http://localhost/");
            println!("Options chosen: {}", options_str);
            if options_str.is_empty() {
                return;
            }

            let options = options_str.split("-");

            for i in options {
                println!("{}", i);
                let mut file = std::fs::File::create(format!("{}{}.flag", path, i)).unwrap();
            }
        }
        Err(_) => {
            println!("Uh oh! Error getting options!");
        }
    }
	unsafe {
		update_enabled_checks();
	}
}

// this structure is gross and largely undefined but it holds some useful information about the CSS instance
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct CharaSelect {
    addr: *const u64,
    some_node: *const u64,
    unk1: [u64; 3],
    unk_structs: [[u64; 2]; 12],
    _0xe8: u32,
    _0xec: u32,
    _0xf0: u32,
    _0xf4: u32,
    _0xf8: u64,
    _ptr_100: *const u64, // layout related
    _pad1: [u64; 6],
    _0x138: u32,
    frames_elapsed: i32,
    loading_state: u32,
    _0x144: u32,
    _ptr_148: *const u64,
    _ptr_150: *const u64,
    unk_bytes: [u8; 8],
    current_player_count: u32, //union
    css_mode: u32, //union
    _0x168: u8,
    is_team_battle: bool,
    _0x16a: u8,
    _0x16b: u8,
    game_mode: u32,
    local_wireless: u32, // 1 in local wireless, otherwise may be unrelated
    ready_state: u32,
    _0x178: u32,
    min_players_allowed: u32, // aka min # of ui panes
    max_players_allowed: u32,
    _0x184: u32,
    _0x188: u64,
    player_buffer: *const u64,
    player_root: *const u64,
    _ptr_1a0: *const u64,
    players: [[u64; 2]; 8], // not researched enough
    _pad2: [u64; 2],
    first_player: *const PlayerInfo,
    max_allowed_player: *const PlayerInfo,
    _ptr_248: *const u64,
    player_base: *const PlayerInfo,
    player_max: *const PlayerInfo,
    _ptr_260: *const u64,
    card_array_start: *const u64,
    card_array_end: *const u64,
    // theres way more here :)
}
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct PlayerInfo {
    root: *const u64,
    card: *const PlayerCard,
    next: *const PlayerInfo,
}
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct PlayerCard {
    _0x0: u64,
    parts: *const u64,
    layout: *const u64,
    _0x18: u64,
    css_instance: *const CharaSelect,
    active_slot_id: u32,
    current_state: u8,
    target_state: u8,
    bool_2e: bool,
    _0x2f: u8,
    index: u32,
    _0x34: u8,
    is_visible: bool,
    is_active: bool,
    root_pane: *const u64,
    _unk_range_40: [u64; 34],
    max_card_count: u32,
    _0x154: u32,
    _unk_range_158: [u64; 13],
    bool_1c0: bool,
    bool_1c1: bool,
    bool_1c2: bool,
    bool_1c3: bool,
    bool_1c4: bool,
    bool_1c5: bool,
    bool_1c6: bool,
    bool_1c7: bool,
    _0x1c8: u64,
    current_id: u16,
    id_1: u16,
    id_2: u16,
    id_3: u16,
    id_4: u16,
    id_5: u16,
    id_6: u16,
    id_7: u16,
    id_8: u16,
    id_9: u16,
    id_10: u16,
    _0x1e8: u64,
    player_num: u32,
    _0x1f4: u32,
    player_kind: i32, // 0 = player, 1 = cpu, 2 = amiibo, 3 = none
    _0x1fc: u32,
    _0x200: u64,
    _0x208: u64,
    _0x210: u8,
    bool_211: bool,
    _0x212: u8,
    _0x213: u8,
    _0x214: u16,
    _0x216: u16,
    _0x218: u32,
    card_type: u32,
    team_id: u32,
    _0x224: u32,
    _0x228: u64,
    _0x230: u64,
    _0x238: u64,
    max_card_count2: u32,
    layout_variant: u32,
    _0x248: u64,
    _unk_range_250: [u64; 40],
    controller_id: u32,
    some_state: u32,
    // theres more here
}

pub fn install() {
    skyline::install_hooks!(css_main_loop, on_rule_selection);
}
