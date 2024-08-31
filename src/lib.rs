#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(warnings, unused)]

#[cfg(feature = "main_nro")]
use std::{fs, path::Path};
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


pub fn quick_validate_install() -> bool {
    let mut passed = true;
    //plugin checks
    let has_param_config = Path::new(
        "rom:/skyline/plugins/libparam_config.nro",
    )
    .is_file();
    let has_css_redirector = Path::new(
        "rom:/skyline/plugins/libthe_csk_collection.nro",
    )
    .is_file();
    let has_arcropolis = Path::new(
        "rom:/skyline/plugins/libarcropolis.nro",
    )
    .is_file();
    let has_nro_hook = Path::new(
        "rom:/skyline/plugins/libnro_hook.nro"
    )
    .is_file();
    let has_smashline = Path::new(
        "rom:/skyline/plugins/libsmashline_plugin.nro",
    )
    .is_file();

    if has_param_config {
        println!("libparam_config.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libparam_config.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        } else {
            DialogOk::ok("libparam_config.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        }
        passed = false;
    }
    if has_css_redirector {
        println!("libthe_csk_collection.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libthe_csk_collection.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        } else {
            DialogOk::ok("libthe_csk_collection.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        }
        passed = false;
    }
    if has_arcropolis {
        println!("libarcropolis.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libarcropolis.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        } else {
            DialogOk::ok("libarcropolis.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        }
        passed = false;
    }
    if has_nro_hook {
        println!("libnro_hook.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libnro_hook.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        } else {
            DialogOk::ok("libnro_hook.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        }
        passed = false;
    }
    if has_smashline {
        println!("libsmashline_plugin.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libsmashline_plugin.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        } else {
            DialogOk::ok("libsmashline_plugin.nro not found! This installation is incomplete. Please run Ultimate S Setup Tool.");
        }
        passed = false;
    }

    passed
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
    *ctx.registers[8].x.as_mut() = 0;
}


static mut RUN: AtomicBool = AtomicBool::new(false);

#[skyline::hook(offset = 0x3810664, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

static mut DUMMY_BLOCK: [u8; 0x100] = [0; 0x100];

#[skyline::hook(offset = 0x374777C, inline)]
unsafe fn run_scene_update(_: &skyline::hooks::InlineCtx) {
    while !RUN.swap(false, Ordering::SeqCst) {
        skyline::nn::hid::GetNpadFullKeyState(DUMMY_BLOCK.as_mut_ptr() as _, &0);
    }
}
  
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
	let original_str = unsafe { skyline::from_c_str(string) };
	if original_str.contains("Ver.") {
        let s_ver = match std::fs::read_to_string("sd:/ultimate/mods/Ultimate S Arcropolis/version.txt") {
            Ok(version_value) => version_value.trim().to_string(),
            Err(_) => {
                #[cfg(feature = "main_nro")]
                if !is_on_ryujinx() {
                    skyline_web::dialog_ok::DialogOk::ok(
                        "Ultimate S Version unknown!",
                    );
                }

                String::from("UNKNOWN")
            }
        };
		let version_str = format!("{} / Ultimate S {}\0", original_str, s_ver);
		call_original!(arg, skyline::c_str(&version_str))
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
mod masked;
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
mod pichu;
mod pikachu;
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
mod sandbag;
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

    if !quick_validate_install() {
        return; // don't do anything else since they don't have all dependencies
    }

    //allows online play with added chars
    unsafe { 
        if Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libthe_csk_collection.nro").is_file() {
            extern "C" { fn allow_ui_chara_hash_online(ui_chara_hash: u64); }
            allow_ui_chara_hash_online(0xf1062d2e5); //rayman
            allow_ui_chara_hash_online(0xda4cbcb12); //toad
            allow_ui_chara_hash_online(0x12e2fb36c6); //bomberman
            allow_ui_chara_hash_online(0x189bd7b932); //sandbag
            allow_ui_chara_hash_online(0x124d54553d); //masked man
        }
    }
	
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






	
	
	util::install();
	common::install();
	controls::install();
	cpu::install();
	

	if Path::new("sd:/ultimate/ult-s/bayonetta.flag").is_file() {
        bayonetta::install();
        println!("bayonetta installed");
    }

    if Path::new("sd:/ultimate/ult-s/brave.flag").is_file() {
        brave::install();
        println!("brave installed");
    }

    if Path::new("sd:/ultimate/ult-s/buddy.flag").is_file() {
        buddy::install();
        println!("buddy installed");
    }

    if Path::new("sd:/ultimate/ult-s/captain.flag").is_file() {
        captain::install();
        println!("captain installed");
    }

    if Path::new("sd:/ultimate/ult-s/chrom.flag").is_file() {
        chrom::install();
        println!("chrom installed");
    }

    if Path::new("sd:/ultimate/ult-s/cloud.flag").is_file() {
        cloud::install();
        println!("cloud installed");
    }

    if Path::new("sd:/ultimate/ult-s/daisy.flag").is_file() {
        daisy::install();
        println!("daisy installed");
    }

    if Path::new("sd:/ultimate/ult-s/dedede.flag").is_file() {
        dedede::install();
        println!("dedede installed");
    }

    if Path::new("sd:/ultimate/ult-s/demon.flag").is_file() {
        demon::install();
        println!("demon installed");
    }

    if Path::new("sd:/ultimate/ult-s/diddy.flag").is_file() {
        diddy::install();
        println!("diddy installed");
    }

    if Path::new("sd:/ultimate/ult-s/dolly.flag").is_file() {
        dolly::install();
        println!("dolly installed");
    }

    if Path::new("sd:/ultimate/ult-s/donkey.flag").is_file() {
        donkey::install();
        println!("donkey installed");
    }

    if Path::new("sd:/ultimate/ult-s/duckhunt.flag").is_file() {
        duckhunt::install();
        println!("duckhunt installed");
    }

    if Path::new("sd:/ultimate/ult-s/edge.flag").is_file() {
        edge::install();
        println!("edge installed");
    }

    if Path::new("sd:/ultimate/ult-s/elight.flag").is_file() || Path::new("sd:/ultimate/ult-s/eflame.flag").is_file(){
        element::install();
        println!("element installed");
    }

    if Path::new("sd:/ultimate/ult-s/falco.flag").is_file() {
        falco::install();
        println!("falco installed");
    }

    if Path::new("sd:/ultimate/ult-s/fox.flag").is_file() {
        fox::install();
        println!("fox installed");
    }

    if Path::new("sd:/ultimate/ult-s/gamewatch.flag").is_file() {
        gamewatch::install();
        println!("gamewatch installed");
    }

    if Path::new("sd:/ultimate/ult-s/ganon.flag").is_file() {
        ganon::install();
        println!("ganon installed");
    }

    if Path::new("sd:/ultimate/ult-s/gaogaen.flag").is_file() {
        gaogaen::install();
        println!("gaogaen installed");
    }

    if Path::new("sd:/ultimate/ult-s/gekkouga.flag").is_file() {
        gekkouga::install();
        println!("gekkouga installed");
    }

    if Path::new("sd:/ultimate/ult-s/ike.flag").is_file() {
        ike::install();
        println!("ike installed");
    }

    if Path::new("sd:/ultimate/ult-s/inkling.flag").is_file() {
        inkling::install();
        println!("inkling installed");
    }

    if Path::new("sd:/ultimate/ult-s/jack.flag").is_file() {
        jack::install();
        println!("jack installed");
    }

    if Path::new("sd:/ultimate/ult-s/kamui.flag").is_file() {
        kamui::install();
        println!("kamui installed");
    }

    if Path::new("sd:/ultimate/ult-s/ken.flag").is_file() {
        ken::install();
        println!("ken installed");
    }

    if Path::new("sd:/ultimate/ult-s/kirby.flag").is_file() {
        kirby::install();
        println!("kirby installed");
    }

    if Path::new("sd:/ultimate/ult-s/koopa.flag").is_file() {
        koopa::install();
        println!("koopa installed");
    }

    if Path::new("sd:/ultimate/ult-s/koopajr.flag").is_file() {
        koopajr::install();
        println!("koopajr installed");
    }

    if Path::new("sd:/ultimate/ult-s/krool.flag").is_file() {
        krool::install();
        println!("krool installed");
    }

    if Path::new("sd:/ultimate/ult-s/link.flag").is_file() {
        link::install();
        println!("link installed");
    }

    if Path::new("sd:/ultimate/ult-s/littlemac.flag").is_file() {
        littlemac::install();
        println!("littlemac installed");
    }

    if Path::new("sd:/ultimate/ult-s/lucario.flag").is_file() {
        lucario::install();
        println!("lucario installed");
    }

    if Path::new("sd:/ultimate/ult-s/lucas.flag").is_file() {
        lucas::install();
        println!("lucas installed");
    }

    if Path::new("sd:/ultimate/ult-s/lucina.flag").is_file() {
        lucina::install();
        println!("lucina installed");
    }

    if Path::new("sd:/ultimate/ult-s/luigi.flag").is_file() {
        luigi::install();
        println!("luigi installed");
    }

    if Path::new("sd:/ultimate/ult-s/mario.flag").is_file() {
        mario::install();
        println!("mario installed");
    }

    if Path::new("sd:/ultimate/ult-s/mariod.flag").is_file() {
        mariod::install();
        println!("mariod installed");
    }

    if Path::new("sd:/ultimate/ult-s/marth.flag").is_file() {
        marth::install();
        println!("marth installed");
    }

    if Path::new("sd:/ultimate/ult-s/master.flag").is_file() {
        master::install();
        println!("master installed");
    }

    if Path::new("sd:/ultimate/ult-s/metaknight.flag").is_file() {
        metaknight::install();
        println!("metaknight installed");
    }

    if Path::new("sd:/ultimate/ult-s/mewtwo.flag").is_file() {
        mewtwo::install();
        println!("mewtwo installed");
    }

    if Path::new("sd:/ultimate/ult-s/miifighter.flag").is_file() {
        miifighter::install();
        println!("miifighter installed");
    }

    if Path::new("sd:/ultimate/ult-s/miigunner.flag").is_file() {
        miigunner::install();
        println!("miigunner installed");
    }

    if Path::new("sd:/ultimate/ult-s/miiswordsman.flag").is_file() {
        miiswordsman::install();
        println!("miiswordsman installed");
    }

    if Path::new("sd:/ultimate/ult-s/murabito.flag").is_file() {
        murabito::install();
        println!("murabito installed");
    }

    if Path::new("sd:/ultimate/ult-s/ness.flag").is_file() {
        ness::install();
        println!("ness installed");
    }

    if Path::new("sd:/ultimate/ult-s/packun.flag").is_file() {
        packun::install();
        println!("packun installed");
    }

    if Path::new("sd:/ultimate/ult-s/pacman.flag").is_file() {
        pacman::install();
        println!("pacman installed");
    }

    if Path::new("sd:/ultimate/ult-s/palutena.flag").is_file() {
        palutena::install();
        println!("palutena installed");
    }

    if Path::new("sd:/ultimate/ult-s/peach.flag").is_file() {
        peach::install();
        println!("peach installed");
    }

    if Path::new("sd:/ultimate/ult-s/pichu.flag").is_file() {
        pichu::install();
        println!("pichu installed");
    }

    if Path::new("sd:/ultimate/ult-s/pikachu.flag").is_file() {
        pikachu::install();
        println!("pikachu installed");
    }

    if Path::new("sd:/ultimate/ult-s/pit.flag").is_file() {
        pit::install();
        println!("pit installed");
    }

    if Path::new("sd:/ultimate/ult-s/pitb.flag").is_file() {
        pitb::install();
        println!("pitb installed");
    }

    if Path::new("sd:/ultimate/ult-s/popo.flag").is_file() {
        popo::install();
        println!("popo installed");
    }

    if Path::new("sd:/ultimate/ult-s/pfushigisou.flag").is_file() || Path::new("sd:/ultimate/ult-s/pzenigame.flag").is_file()  || Path::new("sd:/ultimate/ult-s/plizardon.flag").is_file() {
        ptrainer::install();
        println!("ptrainer installed");
    }

    if Path::new("sd:/ultimate/ult-s/purin.flag").is_file() {
        purin::install();
        println!("purin installed");
    }

    if Path::new("sd:/ultimate/ult-s/reflet.flag").is_file() {
        reflet::install();
        println!("reflet installed");
    }

    if Path::new("sd:/ultimate/ult-s/richter.flag").is_file() {
        richter::install();
        println!("richter installed");
    }

    if Path::new("sd:/ultimate/ult-s/ridley.flag").is_file() {
        ridley::install();
        println!("ridley installed");
    }

    if Path::new("sd:/ultimate/ult-s/robot.flag").is_file() {
        robot::install();
        println!("robot installed");
    }

    if Path::new("sd:/ultimate/ult-s/rockman.flag").is_file() {
        rockman::install();
        println!("rockman installed");
    }

    if Path::new("sd:/ultimate/ult-s/rosetta.flag").is_file() {
        rosetta::install();
        println!("rosetta installed");
    }

    if Path::new("sd:/ultimate/ult-s/roy.flag").is_file() {
        roy::install();
        println!("roy installed");
    }

    if Path::new("sd:/ultimate/ult-s/ryu.flag").is_file() {
        ryu::install();
        println!("ryu installed");
    }

    if Path::new("sd:/ultimate/ult-s/samus.flag").is_file() {
        samus::install();
        println!("samus installed");
    }

    if Path::new("sd:/ultimate/ult-s/samusd.flag").is_file() {
        samusd::install();
        println!("samusd installed");
    }

    if Path::new("sd:/ultimate/ult-s/sheik.flag").is_file() {
        sheik::install();
        println!("sheik installed");
    }

    if Path::new("sd:/ultimate/ult-s/shizue.flag").is_file() {
        shizue::install();
        println!("shizue installed");
    }

    if Path::new("sd:/ultimate/ult-s/shulk.flag").is_file() {
        shulk::install();
        println!("shulk installed");
    }

    if Path::new("sd:/ultimate/ult-s/simon.flag").is_file() {
        simon::install();
        println!("simon installed");
    }

    if Path::new("sd:/ultimate/ult-s/snake.flag").is_file() {
        snake::install();
        println!("snake installed");
    }

    if Path::new("sd:/ultimate/ult-s/sonic.flag").is_file() {
        sonic::install();
        println!("sonic installed");
    }

    if Path::new("sd:/ultimate/ult-s/szerosuit.flag").is_file() {
        szerosuit::install();
        println!("szerosuit installed");
    }

    if Path::new("sd:/ultimate/ult-s/tantan.flag").is_file() {
        tantan::install();
        println!("tantan installed");
    }

    if Path::new("sd:/ultimate/ult-s/toonlink.flag").is_file() {
        toonlink::install();
        println!("toonlink installed");
    }

    if Path::new("sd:/ultimate/ult-s/trail.flag").is_file() {
        trail::install();
        println!("trail installed");
    }

    if Path::new("sd:/ultimate/ult-s/wario.flag").is_file() {
        wario::install();
        println!("wario installed");
    }

    if Path::new("sd:/ultimate/ult-s/wiifit.flag").is_file() {
        wiifit::install();
        println!("wiifit installed");
    }

    if Path::new("sd:/ultimate/ult-s/wolf.flag").is_file() {
        wolf::install();
        println!("wolf installed");
    }

    if Path::new("sd:/ultimate/ult-s/younglink.flag").is_file() {
        younglink::install();
        println!("younglink installed");
    }

    if Path::new("sd:/ultimate/ult-s/yoshi.flag").is_file() {
        yoshi::install();
        println!("yoshi installed");
    }

    if Path::new("sd:/ultimate/ult-s/zelda.flag").is_file() {
        zelda::install();
        println!("zelda installed");
    }
	
    rayman::install();
    bomberman::install();
    toad::install();
    sandbag::install();
    masked::install();
    
    println!("added chars installed");

	//Stage Patching

	//Arena Ferox Screenshake
	skyline::patching::Patch::in_text(0x28444cc + 0xc80).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x28440f4 + 0xc80).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x2844500+ 0xc80).nop();
    skyline::patching::Patch::in_text(0x2844128+ 0xc80).nop();
}
