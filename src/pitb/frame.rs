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
    Agent::new("pitb")
    .on_line(Main, pitoo)
    .install();
}

//Pit Downb nerf
unsafe extern "C" fn pitoo(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let situation_kind = StatusModule::situation_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma);
		if is_default(boma) {
			if  MotionModule::motion_kind(boma) == hash40("special_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DOWN, true);
			};
			if  MotionModule::motion_kind(boma) == hash40("special_air_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_air_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, true);
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END {
				if situation_kind == *SITUATION_KIND_AIR {
					if frame < 10.0 {
						StatusModule::set_keep_situation_air(boma, true);
					} else {
						StatusModule::set_keep_situation_air(boma, false);
						if situation_kind == *SITUATION_KIND_GROUND {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
						}
					}
				}
				if end_frame - frame < 3.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
				};
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
				StatusModule::set_keep_situation_air(boma, true);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, false);
			};
			if situation_kind != *SITUATION_KIND_AIR {
				CAN_SIDEB[ENTRY_ID] = 0;
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH {
					CAN_SIDEB[ENTRY_ID] = 1;
					if MotionModule::frame(boma) > 10.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, false);
					};
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END {
					CAN_SIDEB[ENTRY_ID] = 1;
					reimpl_cancel_frame(fighter);
					if end_frame - frame < 3.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
			};
		};
	}
}