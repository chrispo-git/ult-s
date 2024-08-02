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
	Agent::new("brave")
	.on_line(Main, hero)
	.install();
}

unsafe extern "C" fn hero(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let frame = MotionModule::frame(boma);
			let situation_kind = smash::app::lua_bind::StatusModule::situation_kind;
			let fighter_kind = smash::app::utility::get_kind(boma);
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let stick_y = ControlModule::get_stick_y(boma);
			if fighter_kind == *FIGHTER_KIND_BRAVE{
				if [*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_HOLD].contains(&status_kind){
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						let cat2 = ControlModule::get_command_flag_cat(boma, 1);
						if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
							WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
						}
					};
				};
				if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
					CAN_UPB[ENTRY_ID] = 0;
				};
				if [hash40("special_hi1"), hash40("special_air_hi1"), hash40("special_hi_empty"), hash40("special_air_hi_empty")].contains(&MotionModule::motion_kind(boma)) {
					CAN_UPB[ENTRY_ID] = 1;
					if MotionModule::frame(boma) >= 41.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
					};
					WorkModule::set_int(boma, WorkModule::get_int(boma,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX), *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
				};
				if [hash40("special_n1"), hash40("special_air_n1"), hash40("special_n2"),  hash40("special_air_n2")].contains(&motion_kind){
					if StatusModule::is_situation_changed(boma) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
						WorkModule::set_float(boma, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
					};
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						let cat2 = ControlModule::get_command_flag_cat(boma, 1);
						if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
							WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
						}
					};
				};
			};
		}
	};
}