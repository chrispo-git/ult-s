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
use smash::app::*;
use crate::util::*;
use super::*;
	
pub fn install() {
	Agent::new("brave")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
	.on_line(Main, hero)
	.install();
}
unsafe fn special_hi1_no_freefall(fighter : &mut L2CFighterCommon, entry_id: usize, motion_kind: u64, frame: f32, situation_kind: i32) -> () {
	if situation_kind != *SITUATION_KIND_AIR {
		CAN_UPB[entry_id] = 0;
	};
	if ![hash40("special_hi1"), 
			hash40("special_air_hi1"), 
			hash40("special_hi_empty"), 
			hash40("special_air_hi_empty")
	].contains(&motion_kind) {
		return;
	}
	CAN_UPB[entry_id] = 1;
	if frame >= 41.0 {
		StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
		WorkModule::set_int(
			fighter.module_accessor, 
			WorkModule::get_int(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX), 
			*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT
		);
	}
}
unsafe extern "C" fn hero(fighter : &mut L2CFighterCommon) {
    unsafe {
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
		let frame = MotionModule::frame(fighter.module_accessor);
		let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(fighter.module_accessor);

		special_hi1_no_freefall(
			fighter,
			entry_id,
			motion_kind,
			frame,
			situation_kind
		);
	};
}