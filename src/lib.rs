#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(warnings, unused)]

#[macro_use]
extern crate modular_bitfield;

#[macro_use]
extern crate lazy_static;

pub static mut FIGHTER_MANAGER: usize = 0;

use skyline::libc::c_char;
use skyline::nro::{self, NroInfo};
use smash::params::add_hook;

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

  
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
	let original_str = unsafe { skyline::from_c_str(string) };
	if original_str.contains("Ver.") {
		let version_str = format!("{} / Ultimate S enabled\0", original_str);
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
mod simon;
mod snake;
mod sonic;
mod szerosuit;
mod tantan;
mod toonlink;
mod trail;
mod wario;
mod wiifit;
mod wolf;
mod younglink;
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
pub extern "C" fn main() {
	//Common
	skyline::install_hooks!(change_version_string_hook);
	nro::add_hook(nro_hook).unwrap();
	util::install();
	common::install();
	controls::install();
	cpu::install();
	
	//Fighters
	bayonetta::install();
	brave::install();
	buddy::install();
	
	captain::install();
	chrom::install();
	cloud::install();
	
	daisy::install();
	dedede::install();
	demon::install();
	diddy::install();
	dolly::install();
	donkey::install();
	duckhunt::install();
	
	edge::install();
	element::install();
	
	falco::install();
	fox::install();
	
	gamewatch::install();
	ganon::install();
	gaogaen::install();
	gekkouga::install();
	
	ike::install();
	inkling::install();
	
	jack::install();
	
	kamui::install();
	ken::install();
	kirby::install();
	koopa::install();
	koopajr::install();
	krool::install();
	
	link::install();
	littlemac::install();
	lucario::install();
	lucas::install();
	lucina::install();
	luigi::install();
	
	mario::install();
	mariod::install();
	marth::install();
	master::install();
	metaknight::install();
	mewtwo::install();
	miifighter::install();
	miigunner::install();
	miiswordsman::install();
	murabito::install();
	
	packun::install();
	pacman::install();
	palutena::install();
	peach::install();
	pichu::install();
	pikachu::install();
	pikmin::install();
	pit::install();
	pitb::install();
	popo::install();
	ptrainer::install();
	purin::install();
	
	reflet::install();
	richter::install();
	ridley::install();
	robot::install();
	rockman::install();
	rosetta::install();
	roy::install();
	ryu::install();
	
	samus::install();
	samusd::install();
	sheik::install();
	shizue::install();
	simon::install();
	snake::install();
	sonic::install();
	szerosuit::install();
	
	tantan::install();
	toonlink::install();
	trail::install();
	
	wario::install();
	wiifit::install();
	wolf::install();
	
	younglink::install();
	
	zelda::install();

	//Stage Patching

	//Arena Ferox Screenshake
	skyline::patching::Patch::in_text(0x28444cc).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x28440f4).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x2844500).nop();
    skyline::patching::Patch::in_text(0x2844128).nop();
}
