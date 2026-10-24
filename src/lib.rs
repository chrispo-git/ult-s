
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(warnings, unused)]

use std::{fs, path::Path};

#[cfg(feature = "main_nro")]
use skyline_web::dialog_ok::DialogOk;

#[macro_use]
extern crate modular_bitfield;

#[macro_use]
extern crate lazy_static;

pub static mut FIGHTER_MANAGER: usize = 0;

use skyline::libc::c_char;
use skyline::nro::{self, NroInfo};
use smash::params::add_hook;
use std::sync::atomic::{AtomicBool, Ordering};
use skyline::hooks::InlineCtx;


pub fn is_on_ryujinx() -> bool {
    unsafe {
        // Ryujinx skip based on text addr
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8504000 || text_addr == 0x80004000 {
            crate::debug!("[Ultimate S] we are on Emulator");
            return true;
        } else {
            crate::debug!("[Ultimate S] we are not on Emulator");
            return false;
        }
    }
}
mod state_manager;
mod variable_module;
mod param_cache;
mod s_macros;
mod config;
mod config_apply;


pub fn quick_validate_install() -> bool {
    let has_param_config = Path::new(
        "rom:/skyline/plugins/libparam_config.nro",
    ).is_file();

    if has_param_config {
        crate::debug!("[Ultimate S] libparam_config.nro is present");
    } else {
        if is_on_ryujinx() {
            crate::debug!("[Ultimate S] libparam_config.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        } else {
            DialogOk::ok("libparam_config.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        }
        return false;
    }
    let has_css_redirector = Path::new(
        "rom:/skyline/plugins/libthe_csk_collection.nro",
    )
    .is_file();
    if has_css_redirector {
        crate::debug!("[Ultimate S] libthe_csk_collection.nro is present");
    } else {
        if is_on_ryujinx() {
            crate::debug!("[Ultimate S] libthe_csk_collection.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        } else {
            DialogOk::ok("libthe_csk_collection.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        }
        return false;
    }
    let has_arcropolis = Path::new(
        "rom:/skyline/plugins/libarcropolis.nro",
    )
    .is_file();
    if has_arcropolis {
        crate::debug!("[Ultimate S] libarcropolis.nro is present");
    } else {
        if is_on_ryujinx() {
            crate::debug!("[Ultimate S] libarcropolis.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        } else {
            DialogOk::ok("libarcropolis.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        }
        return false;
    }
    let has_nro_hook = Path::new(
        "rom:/skyline/plugins/libnro_hook.nro"
    )
    .is_file();
    if has_nro_hook {
        crate::debug!("[Ultimate S] libnro_hook.nro is present");
    } else {
        if is_on_ryujinx() {
            crate::debug!("[Ultimate S] libnro_hook.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        } else {
            DialogOk::ok("libnro_hook.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        }
        return false;
    }
    let has_smashline = Path::new(
        "rom:/skyline/plugins/libsmashline_plugin.nro",
    )
    .is_file();
    if has_smashline {
        crate::debug!("[Ultimate S] libsmashline_plugin.nro is present");
    } else {
        if is_on_ryujinx() {
            crate::debug!("[Ultimate S] libsmashline_plugin.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        } else {
            DialogOk::ok("libsmashline_plugin.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        }
        return false;
    }

    return true;
}

extern "C" {
	fn change_version_string(arg: u64, string: *const c_char);
}
pub fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.module.isLoaded {
        return;
    }

    if info.name == "common" {
        skyline::install_hooks!(
            cpu::dmg_fly_main,
            cpu::dmg_fly_roll_main,
            cpu::dmg_main,
            cpu::dmg_air_main
        );
    }
}


unsafe fn calc_nnsdk_offset() -> u64 {
    let mut symbol = 0usize;
    skyline::nn::ro::LookupSymbol(&mut symbol, b"_ZN7android7IBinderD1Ev\0".as_ptr());
    (symbol - 0x240) as u64
}

static mut OFFSET1: u64 = 0;
static mut OFFSET2: u64 = 0;

#[skyline::hook(replace = OFFSET1)]
unsafe fn set_interval_1(window: u64, _: i32) {
    call_original!(window, 0);
}

#[skyline::hook(replace = OFFSET2, inline)]
unsafe fn set_interval_2(ctx: &mut InlineCtx) {
    ctx.registers[8].set_x(0);
    
}


static mut RUN: AtomicBool = AtomicBool::new(false);

#[skyline::hook(offset = 0x3811014, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

static mut DUMMY_BLOCK: [u8; 0x100] = [0; 0x100];

#[skyline::hook(offset = 0x374812c, inline)]
unsafe fn run_scene_update(_: &skyline::hooks::InlineCtx) {
    while !RUN.swap(false, Ordering::SeqCst) {
        skyline::nn::hid::GetNpadFullKeyState(DUMMY_BLOCK.as_mut_ptr() as _, &0);
    }
}
  
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
	let original_str = unsafe { skyline::from_c_str(string) };
	if original_str.contains("Ver. 13") {
        if Path::new("sd:/ultimate/mods/Ultimate S Arcropolis/").is_dir() {
            let mut s_ver = match std::fs::read_to_string("sd:/ultimate/mods/Ultimate S Arcropolis/version.txt") {
                Ok(version_value) => version_value.trim().to_string(),
                Err(_) => {
                    String::from("UNKNOWN")
                }
            };
            let version_str = format!("{} / Ultimate S {}\0", original_str, s_ver);
            call_original!(arg, skyline::c_str(&version_str))
        } else {
            let mut s_ver = match std::fs::read_to_string("sd:/ultimate/mods/Ultimate S Lite/version.txt") {
                Ok(version_value) => version_value.trim().to_string(),
                Err(_) => {
                    String::from("UNKNOWN")
                }
            };
            let version_str = format!("{} / Ultimate S {}\0", original_str, s_ver);
            call_original!(arg, skyline::c_str(&version_str))
        }
	} else {
		call_original!(arg, string)
	}
}












mod util;
mod controls;
mod common;
mod cpu;

mod bayonetta;
mod bomberman;
mod brave;
mod buddy;
mod captain;
mod chrom;
mod cloud;
mod daisy;
mod dedede;
mod demon;
mod diddy;
mod dolly;
mod donkey;
mod duckhunt;
mod edge;
mod element;
mod falco;
mod fox;
mod gamewatch;
mod ganon;
mod gaogaen;
mod gekkouga;
mod ike;
mod inkling; 
mod jack;
mod kamui;
mod ken;
mod kirby;
mod koopa;
mod koopajr;
mod krool;
mod link;
mod littlemac;
mod lucario;
mod lucas;
mod lucina;
mod luigi;
mod mario;
mod mariod;
mod marth;
mod master;
mod metaknight;
mod mewtwo;
mod miifighter;
mod miigunner;
mod miiswordsman;
mod murabito;
mod ness;
mod packun;
mod pacman;
mod palutena;
mod peach;
mod peppy;
mod pichu;
mod pikachu;
mod pickel;
mod pikmin;
mod pit;
mod pitb;
mod popo;
mod ptrainer;
mod purin;
mod rayman;
mod reflet;
mod richter;
mod ridley;
mod robot;
mod rockman;
mod rosetta;
mod roy;
mod ryu;
mod samus;
mod samusd;
mod sheik;
mod shizue;
mod shulk;
mod simon;
mod snake;
mod sonic;
mod szerosuit;
mod tantan;
mod toad;
mod toonlink;
mod trail;
mod wario;
mod wiifit;
mod wolf;
mod younglink;
mod yoshi;
mod zelda;

std::arch::global_asm!(
    r#"
    .section .nro_header
    .global __nro_header_start
    .word 0
    .word _mod_header
    .word 0
    .word 0
    
    .section .rodata.module_name
        .word 0
        .word 5
        .ascii "ultimate-s"
    .section .rodata.mod0
    .global _mod_header
    _mod_header:
        .ascii "MOD0"
        .word __dynamic_start - _mod_header
        .word __bss_start - _mod_header
        .word __bss_end - _mod_header
        .word __eh_frame_hdr_start - _mod_header
        .word __eh_frame_hdr_end - _mod_header
        .word __nx_module_runtime - _mod_header // runtime-generated module object offset
    .global IS_NRO
    IS_NRO:
        .word 1
    
    .section .bss.module_runtime
    __nx_module_runtime:
    .space 0xD0
    "#
);
#[no_mangle]
pub extern "C" fn is_ultimate_s() {}

#[no_mangle]
pub extern "C" fn main() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        crate::s_macros::write_log(&format!("PANIC: {}", info));
        prev(info);
    }));
    crate::debug!("[Ultimate S] Running config setup");
    match config::init() {
        Ok(_) => {
            crate::debug!("[Ultimate S] Config loaded successfully");
        }
        Err(e) => {
            if is_on_ryujinx() {
                crate::debug!("[Ultimate S] Your config.toml is incorrect! Please provide a correct one.");
            } else {
                DialogOk::ok("Your config.toml is incorrect! Please provide a correct one.");
            }
            return;
        }
    }
    config::init();
    crate::debug!("[Ultimate S] Complete config setup.");

    if !quick_validate_install() {
        return; // don't do anything else since they don't have all dependencies
    }


    crate::debug!("[Ultimate S] Doing online play stuff");
    //allows online play with added chars
    unsafe { 
        if Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libthe_csk_collection.nro").is_file() {
            extern "C" { fn allow_ui_chara_hash_online(ui_chara_hash: u64); }
            allow_ui_chara_hash_online(0xf1062d2e5); //rayman
            allow_ui_chara_hash_online(0xda4cbcb12); //toad
            allow_ui_chara_hash_online(0x12e2fb36c6); //bomberman
            allow_ui_chara_hash_online(smash::hash40("ui_chara_peppy")); //peppy
        }
    }
	
    crate::debug!("[Ultimate S] Installing some hooks");
	//Common
    if !is_on_ryujinx(){
        crate::debug!("[Ultimate S] We're on switch! Yay!");
        unsafe {
                OFFSET1 = calc_nnsdk_offset() + 0x429d60;
                OFFSET2 = calc_nnsdk_offset() + 0x26e94;
        }
        
        skyline::install_hooks!(
                set_interval_1,
                set_interval_2,
                run_scene_update,
                vsync_count_thread,
        );
    }
    skyline::install_hooks!(change_version_string_hook);
	nro::add_hook(nro_hook).unwrap();














	
	
    crate::debug!("[Ultimate S] about to install scripts");
	util::install();
    crate::debug!("[Ultimate S] util installed");
    config_apply::install();
    crate::debug!("[Ultimate S] config_apply installed");
	common::install();
	controls::install();
	cpu::install();
	

    brave::install();
    crate::debug!("[Ultimate S] brave installed");



    bayonetta::install();
    crate::debug!("[Ultimate S] bayonetta installed");



    buddy::install();
    crate::debug!("[Ultimate S] buddy installed");



    captain::install();
    crate::debug!("[Ultimate S] captain installed");



    chrom::install();
    crate::debug!("[Ultimate S] chrom installed");



    cloud::install();
    crate::debug!("[Ultimate S] cloud installed");



    daisy::install();
    crate::debug!("[Ultimate S] daisy installed");



    dedede::install();
    crate::debug!("[Ultimate S] dedede installed");



    demon::install();
    crate::debug!("[Ultimate S] demon installed");



    diddy::install();
    crate::debug!("[Ultimate S] diddy installed");



    dolly::install();
    crate::debug!("[Ultimate S] dolly installed");



    donkey::install();
    crate::debug!("[Ultimate S] donkey installed");



    duckhunt::install();
    crate::debug!("[Ultimate S] duckhunt installed");



    edge::install();
    crate::debug!("[Ultimate S] edge installed");



    element::install();
    crate::debug!("[Ultimate S] element installed");



    falco::install();
    crate::debug!("[Ultimate S] falco installed");



    fox::install();
    crate::debug!("[Ultimate S] fox installed");



    gamewatch::install();
    crate::debug!("[Ultimate S] gamewatch installed");



    ganon::install();
    crate::debug!("[Ultimate S] ganon installed");



    gaogaen::install();
    crate::debug!("[Ultimate S] gaogaen installed");



    gekkouga::install();
    crate::debug!("[Ultimate S] gekkouga installed");



    ike::install();
    crate::debug!("[Ultimate S] ike installed");



    inkling::install();
    crate::debug!("[Ultimate S] inkling installed");



    jack::install();
    crate::debug!("[Ultimate S] jack installed");



    kamui::install();
    crate::debug!("[Ultimate S] kamui installed");



    ken::install();
    crate::debug!("[Ultimate S] ken installed");



    kirby::install();
    crate::debug!("[Ultimate S] kirby installed");



    koopa::install();
    crate::debug!("[Ultimate S] koopa installed");



    koopajr::install();
    crate::debug!("[Ultimate S] koopajr installed");



    krool::install();
    crate::debug!("[Ultimate S] krool installed");



    link::install();
    crate::debug!("[Ultimate S] link installed");



    littlemac::install();
    crate::debug!("[Ultimate S] littlemac installed");



    lucario::install();
    crate::debug!("[Ultimate S] lucario installed");



    lucas::install();
    crate::debug!("[Ultimate S] lucas installed");



    lucina::install();
    crate::debug!("[Ultimate S] lucina installed");



    luigi::install();
    crate::debug!("[Ultimate S] luigi installed");



    mario::install();
    crate::debug!("[Ultimate S] mario installed");



    mariod::install();
    crate::debug!("[Ultimate S] mariod installed");



    marth::install();
    crate::debug!("[Ultimate S] marth installed");



    master::install();
    crate::debug!("[Ultimate S] master installed");



    metaknight::install();
    crate::debug!("[Ultimate S] metaknight installed");



    mewtwo::install();
    crate::debug!("[Ultimate S] mewtwo installed");



    miifighter::install();
    crate::debug!("[Ultimate S] miifighter installed");



    miigunner::install();
    crate::debug!("[Ultimate S] miigunner installed");



    miiswordsman::install();
    crate::debug!("[Ultimate S] miiswordsman installed");



    murabito::install();
    crate::debug!("[Ultimate S] murabito installed");



    ness::install();
    crate::debug!("[Ultimate S] ness installed");



    packun::install();
    crate::debug!("[Ultimate S] packun installed");



    pacman::install();
    crate::debug!("[Ultimate S] pacman installed");



    palutena::install();
    crate::debug!("[Ultimate S] palutena installed");



    peach::install();
    crate::debug!("[Ultimate S] peach installed");



    pichu::install();
    crate::debug!("[Ultimate S] pichu installed");



    pickel::install();
    crate::debug!("[Ultimate S] pickel installed");



    pikachu::install();
    crate::debug!("[Ultimate S] pikachu installed");



    pit::install();
    crate::debug!("[Ultimate S] pit installed");



    pitb::install();
    crate::debug!("[Ultimate S] pitb installed");



    popo::install();
    crate::debug!("[Ultimate S] popo installed");



    ptrainer::install();
    crate::debug!("[Ultimate S] ptrainer installed");



    purin::install();
    crate::debug!("[Ultimate S] purin installed");



    reflet::install();
    crate::debug!("[Ultimate S] reflet installed");



    richter::install();
    crate::debug!("[Ultimate S] richter installed");



    ridley::install();
    crate::debug!("[Ultimate S] ridley installed");



    robot::install();
    crate::debug!("[Ultimate S] robot installed");



    rockman::install();
    crate::debug!("[Ultimate S] rockman installed");



    rosetta::install();
    crate::debug!("[Ultimate S] rosetta installed");



    roy::install();
    crate::debug!("[Ultimate S] roy installed");



    ryu::install();
    crate::debug!("[Ultimate S] ryu installed");



    samus::install();
    crate::debug!("[Ultimate S] samus installed");



    samusd::install();
    crate::debug!("[Ultimate S] samusd installed");



    sheik::install();
    crate::debug!("[Ultimate S] sheik installed");



    shizue::install();
    crate::debug!("[Ultimate S] shizue installed");



    shulk::install();
    crate::debug!("[Ultimate S] shulk installed");



    simon::install();
    crate::debug!("[Ultimate S] simon installed");



    snake::install();
    crate::debug!("[Ultimate S] snake installed");



    sonic::install();
    crate::debug!("[Ultimate S] sonic installed");



    szerosuit::install();
    crate::debug!("[Ultimate S] szerosuit installed");



    tantan::install();
    crate::debug!("[Ultimate S] tantan installed");



    toonlink::install();
    crate::debug!("[Ultimate S] toonlink installed");



    trail::install();
    crate::debug!("[Ultimate S] trail installed");



    wario::install();
    crate::debug!("[Ultimate S] wario installed");



    wiifit::install();
    crate::debug!("[Ultimate S] wiifit installed");



    wolf::install();
    crate::debug!("[Ultimate S] wolf installed");



    younglink::install();
    crate::debug!("[Ultimate S] younglink installed");



    yoshi::install();
    crate::debug!("[Ultimate S] yoshi installed");



    zelda::install();
    crate::debug!("[Ultimate S] zelda installed");

	
    rayman::install();
    bomberman::install();
    toad::install();
    peppy::install();
    
    crate::debug!("[Ultimate S] added chars installed");

    param_cache::install();
    crate::debug!("[Ultimate S] param_cache installed");
    unsafe {
        util::reload_config_values();
    }
















	//Stage Patching

	//Arena Ferox Screenshake
	skyline::patching::Patch::in_text(0x28444cc + 0xc80 + 0x20 + 0x490).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x28440f4 + 0xc80 + 0x20 + 0x490).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x2844500+ 0xc80 + 0x20 + 0x490).nop();
    skyline::patching::Patch::in_text(0x2844128+ 0xc80 + 0x20 + 0x490).nop(); 

    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_peppy");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_rayman");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_bomberman");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_toad");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_toadette");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_toadsworth");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_captaintoad");
}



