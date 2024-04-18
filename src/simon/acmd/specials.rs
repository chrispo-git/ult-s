use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
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
use smash::phx::Vector3f;
use crate::util::*;
use super::*;

pub fn install() {
    Agent::new("simon")
    .acmd("game_speciallw", simon_downb)    
    .acmd("game_specialairlw", simon_downb)    
    .install();
}

unsafe extern "C" fn simon_downb(fighter: &mut L2CAgentBase) {
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