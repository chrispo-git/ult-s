use smash::app::sv_animcmd::*;
use smash::phx::{Hash40, Vector2f, Vector3f};
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

mod status;
mod frame;
mod acmd;
			
static mut CURRENT_PIKMIN : [i32; 8] = [0; 8];
static mut SET_UPB_FREEFALL: [bool; 8] = [false; 8];
static mut IS_SLIDE_MOVE: [bool; 8] = [false; 8];
static mut PULL_DISTANCE: [i32; 8] = [0; 8];
static mut DO_WALLJUMP_FORCE: [bool; 8] = [false; 8];
static mut HAS_DEADED: [bool; 8] = [false; 8];
static mut WAS_SLIDE: [bool; 8] = [false; 8];
static mut FINAL_DURATION : [i32; 8] = [0; 8];
static mut X : [f32; 8] = [0.0; 8];
static mut Y : [f32; 8] = [0.0; 8];
static mut X_MAX : f32 = 1.155;
static mut X_ACCEL_ADD : f32 = 0.06;
static mut X_ACCEL_MUL : f32 = 0.12;
static mut Y_MAX : f32 = 1.155;
static mut Y_ACCEL_ADD : f32 = 0.06;
static mut Y_ACCEL_MUL : f32 = 0.12;

pub(crate) unsafe fn attack_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 6);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_l01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_h01")),
		2 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_f01")),
        3 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackdash02")),
		_ => println!("rayman is silent"),
	}
}
pub(crate) unsafe fn dmg_vc(fighter: &mut L2CAgentBase) -> () {
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_h01"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_h02"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_h03"));
    macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_l01"));
    macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_s01"));
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 5);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h02")),
        2 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h03")),
        3 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_l01")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_s01")),
	}
}
pub(crate) unsafe fn dmg_fly_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackdash01"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackair_n03"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackair_n02"));
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackdash01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_n03")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_n02")),
	}
}
pub(crate) unsafe fn jump_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 2);
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackdash02"));
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackdash02")),
        _ => println!("rayman is silent"),
	}
}
pub(crate) unsafe fn jump_aerial_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 6);
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_s02"));
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_s02")),
		_ => println!("rayman is silent"),
	}
}

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}