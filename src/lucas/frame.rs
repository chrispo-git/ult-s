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
    Agent::new("lucas")
    .on_line(Main, lucas_frame)
    .install();
}

unsafe extern "C" fn lucas_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let frame = MotionModule::frame(boma);
			let end_frame = MotionModule::end_frame(boma) as f32;
			let motion_kind = MotionModule::motion_kind(boma);
			let situation_kind = StatusModule::situation_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if [hash40("special_air_n_start"), hash40("special_n_start")].contains(&motion_kind) {
				CAN_NEUTRALB[ENTRY_ID] = 1;
				if end_frame - frame < 3.0 {
					if situation_kind == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
					};
				};
			};
			if situation_kind != *SITUATION_KIND_AIR {
				CAN_NEUTRALB[ENTRY_ID] = 0;
			};
		}
    }
}