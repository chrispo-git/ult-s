mod status;
mod frame;
mod acmd;
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


//Joker Gun Cancel Constants 
pub const NONE : i32 = 100;
pub const ATTACK_N : i32 = 101;
pub const ATTACK_S3 : i32 = 104;
pub const ATTACK_HI3 : i32 = 105;
pub const ATTACK_LW3 : i32 = 106;
pub const ATTACK_S4 : i32 = 107;
pub const ATTACK_HI4 : i32 = 108;
pub const ATTACK_LW4 : i32 = 110;
pub const ATTACK_AIR_N : i32 = 111;
pub const ATTACK_AIR_F : i32 = 112;
pub const ATTACK_AIR_B : i32 = 113;
pub const ATTACK_AIR_HI : i32 = 114;
pub const ATTACK_AIR_LW : i32 = 115;

//Joker Gun Cancel 
static mut GUN_C: [i32; 8] = [100; 8];
static mut IS_ARSENE: [bool; 8] = [false; 8];
static mut X: [f32; 8] = [0.0; 8];
static mut Y: [f32; 8] = [0.0; 8];
static mut BATON_TYPE: [i32; 8] = [0; 8];
static BATON_MAX : i32 = 2;


pub(crate) fn check_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
			return true;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
			if ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_stick_y(boma) >= 0.7 {
				return true;
			};
		};
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
			return true;
		};
		return false;
	}
}

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}