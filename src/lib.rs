
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
            println!("we are on Emulator");
            return true;
        } else {
            println!("we are not on Emulator");
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
        println!("libparam_config.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libparam_config.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
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
        println!("libthe_csk_collection.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libthe_csk_collection.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
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
        println!("libarcropolis.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libarcropolis.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
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
        println!("libnro_hook.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libnro_hook.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
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
        println!("libsmashline_plugin.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libsmashline_plugin.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
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

#[skyline::hook(offset = 0x3810a64, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

static mut DUMMY_BLOCK: [u8; 0x100] = [0; 0x100];

#[skyline::hook(offset = 0x3747b7c, inline)]
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
    println!("Running config setup");
    match config::init() {
        Ok(_) => {
            println!("Config loaded successfully");
        }
        Err(e) => {
            if is_on_ryujinx() {
                println!("Your config.toml is incorrect! Please provide a correct one.");
            } else {
                DialogOk::ok("Your config.toml is incorrect! Please provide a correct one.");
            }
            return;
        }
    }
    config::init();
    println!("Complete config setup.");

    if !quick_validate_install() {
        return; // don't do anything else since they don't have all dependencies
    }


    println!("Doing online play stuff");
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
	
    println!("Installing some hooks");
	//Common
    if !is_on_ryujinx(){
        println!("We're on switch! Yay!");
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














	
	
    println!("about to install scripts");
	util::install();
    println!("util installed");
    config_apply::install();
    println!("config_apply installed");
	common::install();
	controls::install();
	cpu::install();
	

    brave::install();
    println!("brave installed");



    bayonetta::install();
    println!("bayonetta installed");



    buddy::install();
    println!("buddy installed");



    captain::install();
    println!("captain installed");



    chrom::install();
    println!("chrom installed");



    cloud::install();
    println!("cloud installed");



    daisy::install();
    println!("daisy installed");



    dedede::install();
    println!("dedede installed");



    demon::install();
    println!("demon installed");



    diddy::install();
    println!("diddy installed");



    dolly::install();
    println!("dolly installed");



    donkey::install();
    println!("donkey installed");



    duckhunt::install();
    println!("duckhunt installed");



    edge::install();
    println!("edge installed");



    element::install();
    println!("element installed");



    falco::install();
    println!("falco installed");



    fox::install();
    println!("fox installed");



    gamewatch::install();
    println!("gamewatch installed");



    ganon::install();
    println!("ganon installed");



    gaogaen::install();
    println!("gaogaen installed");



    gekkouga::install();
    println!("gekkouga installed");



    ike::install();
    println!("ike installed");



    inkling::install();
    println!("inkling installed");



    jack::install();
    println!("jack installed");



    kamui::install();
    println!("kamui installed");



    ken::install();
    println!("ken installed");



    kirby::install();
    println!("kirby installed");



    koopa::install();
    println!("koopa installed");



    koopajr::install();
    println!("koopajr installed");



    krool::install();
    println!("krool installed");



    link::install();
    println!("link installed");



    littlemac::install();
    println!("littlemac installed");



    lucario::install();
    println!("lucario installed");



    lucas::install();
    println!("lucas installed");



    lucina::install();
    println!("lucina installed");



    luigi::install();
    println!("luigi installed");



    mario::install();
    println!("mario installed");



    mariod::install();
    println!("mariod installed");



    marth::install();
    println!("marth installed");



    master::install();
    println!("master installed");



    metaknight::install();
    println!("metaknight installed");



    mewtwo::install();
    println!("mewtwo installed");



    miifighter::install();
    println!("miifighter installed");



    miigunner::install();
    println!("miigunner installed");



    miiswordsman::install();
    println!("miiswordsman installed");



    murabito::install();
    println!("murabito installed");



    ness::install();
    println!("ness installed");



    packun::install();
    println!("packun installed");



    pacman::install();
    println!("pacman installed");



    palutena::install();
    println!("palutena installed");



    peach::install();
    println!("peach installed");



    pichu::install();
    println!("pichu installed");



    pickel::install();
    println!("pickel installed");



    pikachu::install();
    println!("pikachu installed");



    pit::install();
    println!("pit installed");



    pitb::install();
    println!("pitb installed");



    popo::install();
    println!("popo installed");



    ptrainer::install();
    println!("ptrainer installed");



    purin::install();
    println!("purin installed");



    reflet::install();
    println!("reflet installed");



    richter::install();
    println!("richter installed");



    ridley::install();
    println!("ridley installed");



    robot::install();
    println!("robot installed");



    rockman::install();
    println!("rockman installed");



    rosetta::install();
    println!("rosetta installed");



    roy::install();
    println!("roy installed");



    ryu::install();
    println!("ryu installed");



    samus::install();
    println!("samus installed");



    samusd::install();
    println!("samusd installed");



    sheik::install();
    println!("sheik installed");



    shizue::install();
    println!("shizue installed");



    shulk::install();
    println!("shulk installed");



    simon::install();
    println!("simon installed");



    snake::install();
    println!("snake installed");



    sonic::install();
    println!("sonic installed");



    szerosuit::install();
    println!("szerosuit installed");



    tantan::install();
    println!("tantan installed");



    toonlink::install();
    println!("toonlink installed");



    trail::install();
    println!("trail installed");



    wario::install();
    println!("wario installed");



    wiifit::install();
    println!("wiifit installed");



    wolf::install();
    println!("wolf installed");



    younglink::install();
    println!("younglink installed");



    yoshi::install();
    println!("yoshi installed");



    zelda::install();
    println!("zelda installed");

	
    rayman::install();
    bomberman::install();
    toad::install();
    peppy::install();
    
    println!("added chars installed");

    param_cache::install();
    println!("param_cache installed");
    unsafe {
        util::reload_config_values();
    }
















	//Stage Patching

	//Arena Ferox Screenshake
	skyline::patching::Patch::in_text(0x28444cc + 0xc80 + 0x20).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x28440f4 + 0xc80 + 0x20).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x2844500+ 0xc80 + 0x20).nop();
    skyline::patching::Patch::in_text(0x2844128+ 0xc80 + 0x20).nop(); 

    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_peppy");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_rayman");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_bomberman");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_toad");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_toadette");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_toadsworth");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_captaintoad");
}



