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

mod status;
mod frame;
mod acmd;

static mut LAND_SIDEB_BOUNCE: [i32; 8] = [0; 8];
static mut BEFORE_SIDEB_BOUNCE: [i32; 8] = [0; 8];
static mut HAS_DOWNB: [bool; 8] = [false; 8];
static mut HAS_DEADED: [bool; 8] = [false; 8];
static mut BOUNCE_DA: [bool; 8] = [false; 8];

pub(crate) unsafe fn attack_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 7);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_b01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_b02")),
		2 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_f01")),
		3 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_f02")),
		4 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h01")),
		_ => println!("toad is silent"),
	}
}
pub(crate) unsafe fn dmg_vc(fighter: &mut L2CAgentBase) -> () {
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_h02"));
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_h03"));
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_h01"));
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h02")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h03")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h01")),
	}
}
pub(crate) unsafe fn dmg_fly_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_l02"));
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_l03"));
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_l01"));
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_l02")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_l03")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_l01")),
	}
}

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}