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
use smash::phx::Vector2f;
use crate::util::*;
use super::*;

mod status;
mod frame;
mod acmd;

static mut LUCINA_STANCE : [i32; 8] = [0; 8];
static mut DOWNB_TIMER : [i32; 8] = [1; 8];
static mut BAN_SIDEB : [bool; 8] = [false; 8];
static mut UPB_FALL : [bool; 8] = [false; 8];
static mut TIMER : [i32; 8] = [0; 8];
static SWORDMASTER_DASH : f32 = 2.38;
static SWORDMASTER_AIR_SPEED : f32 = 2.0;
static SWORDMASTER_MAX_GRAVITY: f32 = 0.1;
static HERO_SWORD_MULT : f32 = 1.15;
static SWORDMASTER_SWORD_MULT : f32 = 0.85;

static HERO_NAIR_LANDING : f32 = 9.0;
static HERO_FAIR_LANDING : f32 = 13.0;
static HERO_BAIR_LANDING : f32 = 9.0;
static HERO_UAIR_LANDING : f32 = 9.0;
static HERO_DAIR_LANDING : f32 = 16.0;

static SWORD_NAIR_LANDING : f32 = 9.0;
static SWORD_FAIR_LANDING : f32 = 10.0;
static SWORD_BAIR_LANDING : f32 = 6.0;
static SWORD_UAIR_LANDING : f32 = 8.0;
static SWORD_DAIR_LANDING : f32 = 14.0;
/*
Base Lucina Landing Lag Stats for reference:
<float hash="landing_attack_air_frame_n">9</float>
<float hash="landing_attack_air_frame_f">12</float>
<float hash="landing_attack_air_frame_b">10</float>
<float hash="landing_attack_air_frame_hi">8</float>
<float hash="landing_attack_air_frame_lw">14</float>
*/
static mut S1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 2.0 };
static mut S2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 4.0 };
static mut S3 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 7.5 };
//Stance 0 - Hero Stance
//Stance 1 - Swordmaster Stance

pub(crate) unsafe fn bone_const(boma: &mut smash::app::BattleObjectModuleAccessor, new_fighter_kind : i32, new_motion_kind : u64, bone : u64,
	start_frame : f32, end_frame : f32, 
	x_rotate : f32, x_rotate_end : f32, y_rotate : f32, y_rotate_end : f32,  z_rotate : f32, z_rotate_end : f32
) -> () {
	let fighter_kind = smash::app::utility::get_kind(boma);
	let motion_kind = MotionModule::motion_kind(boma);
	let frame = MotionModule::frame(boma);
	let duration = end_frame-start_frame;
	if fighter_kind == new_fighter_kind && motion_kind == new_motion_kind && frame >= start_frame && frame <= end_frame {
		let mut rotation = Vector3f{x: x_rotate + (((x_rotate_end-x_rotate)/duration)*(frame-start_frame)), y: y_rotate + (((y_rotate_end-y_rotate)/duration)*(frame-start_frame)) , z: z_rotate + (((z_rotate_end-z_rotate)/duration)*(frame-start_frame))};
	    ModelModule::set_joint_rotate(boma, Hash40::new_raw(bone), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
	};
}


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
