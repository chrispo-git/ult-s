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
    Agent::new("buddy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
	.on_line(Main, buddy_frame)
	.install();
}
unsafe fn dair_bounce(fighter : &mut L2CFighterCommon, motion_kind:u64, frame:f32) -> () {
	if motion_kind != hash40("attack_air_lw") || frame >= 50.0 {
		return;
	}
	if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
	&& !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
		KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
		MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 51.0, true, true, false);
	}
}
unsafe fn sideb_fail_movement(fighter : &mut L2CFighterCommon, status_kind:i32, frame:f32) -> () {
	if status_kind != *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL {
		return;
	}
	crate::fake_b_reverse!(fighter);
	if frame > 20.0 {
		CancelModule::enable_cancel(fighter.module_accessor);
	};
}
unsafe extern "C" fn buddy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
			let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
			let frame = MotionModule::frame(fighter.module_accessor);
			dair_bounce(fighter, motion_kind, frame);
			sideb_fail_movement(fighter, status_kind, frame);
    }
}	