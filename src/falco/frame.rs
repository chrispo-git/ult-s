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
use crate::falco::*;

pub fn install() {
	smashline::install_agent_frames!(falco_frame);
}
	
#[fighter_frame( agent = FIGHTER_KIND_FALCO )]
fn falco_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let stick_y = ControlModule::get_stick_y(boma);
			let fallspeed = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);

			if [hash40("special_lw"), hash40("special_lw_r"), hash40("special_lw_l"), hash40("special_air_lw"), hash40("special_air_lw_r"), hash40("special_air_lw_l")].contains(&motion_kind) {
				if !HAS_DOWNB[ENTRY_ID] && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					HAS_DOWNB[ENTRY_ID] = true;
					DO_STALL[ENTRY_ID] = true;
				};
				if frame > 32.0 {
					DO_STALL[ENTRY_ID] = false; 
					CancelModule::enable_cancel(boma);
				} else {
					if DO_STALL[ENTRY_ID] {
						macros::SET_SPEED_EX(fighter, 0.0, fallspeed*-0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
				};
			} else {
				DO_STALL[ENTRY_ID] = false;
			};
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
				HAS_DOWNB[ENTRY_ID] = false;
				DO_STALL[ENTRY_ID] = false;
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
				if StatusModule::is_situation_changed(boma) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					let cat2 = ControlModule::get_command_flag_cat(boma, 1);
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
						WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					}
				};
			};
		}
    }
}