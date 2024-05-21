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

pub const MAX_HITS : i32 = 6;
pub const DISABLE_SPECIAL_N: i32 = 0x0100;
pub const USE_SPECIAL_N_CALLBACK: i32 = 0x38;
pub const USE_SPECIAL_S_CALLBACK: i32 = 0x39;
pub const USE_SPECIAL_HI_CALLBACK: i32 = 0x3A;
pub const USE_SPECIAL_LW_CALLBACK: i32 = 0x3B;
pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C;
pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D;
pub const STATUS_CHANGE_CALLBACK: i32 = 0x3E;
pub const DAMAGE_MOTION_KIND_CALLBACK: i32 = 0x42;
pub const DASH_POST_TRANSITION_CALLBACK: i32 = 0x57;
pub static mut HAS_NEUTRALB : [bool; 8] = [true; 8];
pub static mut NEUTRALB_CHARGE : [i32; 8] = [0; 8];
pub static mut TIMER_TO_DOWNB : [i32; 8] = [0; 8];
pub static mut DOWNB_COOLDOWN : [i32; 8] = [0; 8];


pub(crate) unsafe fn trail(agent: &mut L2CAgentBase) -> () {
	let color_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) - 120;
	println!("color id {}", color_id);
	if [1].contains(&color_id) {
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_maskedman_sword3"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
	} else if [2].contains(&color_id) {
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_maskedman_sword4"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
	}  else if [4].contains(&color_id) {
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_maskedman_sword5"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
	}  else if [6,7].contains(&color_id) {
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_maskedman_sword6"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
	} else {
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_maskedman_sword1"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
	}
}


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}