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
	Agent::new("daisy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
	.on_line(Main, daisy_frame)
	.install();

	Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
	.on_line(Main, kirby_daisy_frame)
	.install();
}

unsafe extern "C" fn daisy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		neutral_special(fighter, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT);
    }
}
unsafe extern "C" fn kirby_daisy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		neutral_special(fighter, *FIGHTER_KIRBY_STATUS_KIND_DAISY_SPECIAL_N_HIT);
    }
}
#[inline(always)]
pub unsafe fn neutral_special(fighter: &mut L2CFighterCommon, target_status : i32) {
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
	if status_kind == target_status && KineticModule::get_kinetic_type(fighter.module_accessor)  {
		KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
	};
}