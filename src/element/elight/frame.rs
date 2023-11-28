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

use crate::element::*;

pub fn install() {
    smashline::install_agent_frames!(mythra_frame);
}
#[fighter_frame( agent = FIGHTER_KIND_ELIGHT )]
fn mythra_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let situation_kind = StatusModule::situation_kind(boma);
			if ![*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_STANDBY, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_STANDBY].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
				FAST_SWITCH[ENTRY_ID] = false;
			};
			if status_kind == *FIGHTER_STATUS_KIND_THROW {
				let mut can_do = false;
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					if motion_kind == hash40("throw_lw") && frame > 26.0{
						can_do = true;
					};
					if motion_kind == hash40("throw_f") && frame > 13.0{
						can_do = true;
					};
					if motion_kind == hash40("throw_hi") && frame > 10.0{
						can_do = true;
					};
					if motion_kind == hash40("throw_b") && frame > 18.0{
						can_do = true;
					};
				}
				if can_do == true {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
					FAST_SWITCH[ENTRY_ID] = true;
				};
			};
			if FAST_SWITCH[ENTRY_ID] == true && [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_STANDBY, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_STANDBY].contains(&status_kind) {
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
					MotionModule::set_rate(boma, 2.75);
				} else {
					MotionModule::set_rate(boma, 2.0);
				};
			};
			if status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END {
				if situation_kind == *SITUATION_KIND_AIR {
					if (MotionModule::end_frame(boma) - frame < 35.0) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
					};
					if (frame > 12.0) && ControlModule::get_stick_y(boma) >= 0.5 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					};
				};
			};
		}
    }
}