use smash::app::sv_animcmd::*;
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use crate::util::*;
static mut STATIC_MUT : [i32; 8] = [6; 8];

// A Once-Per-Fighter-Frame that only applies to Mario. Neat!
#[fighter_frame( agent = FIGHTER_KIND_MARIO )]
fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        println!("It'sa me, Mario, wahoooooooo!");
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
}
#[acmd_script(
    agent = "simon",
    script =  "effect_landingheavy",
    category = ACMD_EFFECT)]
unsafe fn simon_landing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
		}
}	

#[acmd_script( 
	agent = "simon", 
	scripts = ["game_speciallw", "game_specialairlw"], 
	category = ACMD_GAME,
	low_priority)]
unsafe fn simon_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_HOLYWATER) {
			ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_SIMONHOLYWATER), 0, 0, false, false);
		};
	}
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		let mut stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
		let mut stick_y = ControlModule::get_stick_y(fighter.module_accessor);
		if stick_x < 0.0 {
			stick_x = 0.2;
		} else if stick_x <= 0.2 {
			stick_x = 0.21
		};
		if stick_y < 0.2 && stick_y >= 0.0 {
			stick_y = 0.2;
		};
		if stick_y > -0.2 && stick_y <= 0.0 {
			stick_y = -0.2;
		};
		let mut angle = stick_y.atan()/stick_x.atan();
		let hypotenuse = ((stick_x*stick_x)+(stick_y*stick_y)).sqrt();
		if stick_x == 0.2 {
			MotionModule::set_rate(fighter.module_accessor, 1.9);
		};
		let power = hypotenuse * 2.5;
		ItemModule::throw_item(fighter.module_accessor, angle, power, 1.0, 0, true, 0.0);
		println!("Power {}, Angle {}", power, angle);
	}
}


pub fn install() {
	smashline::install_acmd_scripts!(simon_downb, simon_landing);
}
