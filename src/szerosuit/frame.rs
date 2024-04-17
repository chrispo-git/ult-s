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

#[fighter_frame_callback]
pub fn zss(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		if is_default(boma) {
			let fighter_kind = smash::app::utility::get_kind(boma);
			let lr = PostureModule::lr(boma);
			let stick_x = ControlModule::get_stick_x(boma)* lr;		
			let stick_y = ControlModule::get_stick_y(boma);		
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let situation_kind = StatusModule::situation_kind(boma);
			let frame = MotionModule::frame(boma);
			if fighter_kind == *FIGHTER_KIND_SZEROSUIT {
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
					if StatusModule::is_situation_changed(boma) {
						if situation_kind == *SITUATION_KIND_GROUND {
							if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
							} else {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
							};
						} else {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
						};
					};
					if situation_kind == *SITUATION_KIND_AIR {
						if frame >= 24.3 {
							if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
							} else {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
							}
						};
						IS_AIR_SIDEB[ENTRY_ID] = true;
					};
				} else {
					IS_AIR_SIDEB[ENTRY_ID] = false;
				};
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) == false {
					CAN_CANCEL[ENTRY_ID] = false;
				};
				if CAN_CANCEL_TIMER[ENTRY_ID] > 0 {
					if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 {
						CAN_CANCEL_TIMER[ENTRY_ID] -= 1;
					};
				} else {
					CAN_CANCEL[ENTRY_ID] = false;
				};
				if NO_WAVEDASH_TIMER[ENTRY_ID] > 0{
					CAN_AIRDODGE[ENTRY_ID] = 1;
					NO_WAVEDASH_TIMER[ENTRY_ID] -= 1;
				} else {
					CAN_AIRDODGE[ENTRY_ID] = 0;
				};
				if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT){
					CAN_CANCEL[ENTRY_ID] = true;
					CAN_CANCEL_TIMER[ENTRY_ID] = WINDOW;
				};
				if CAN_CANCEL[ENTRY_ID] && StopModule::is_stop(boma) == false &&  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1{
					if [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
						if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
							if stick_x < 0.0{
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_START, true);
								WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_REVERSE);
							}
							else stick_x > 0.0{
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_START, true);
							};
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
						}/* else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
						}*/;
					};
					if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) {
						if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
							if stick_x < 0.0{
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_START, true);
								WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_REVERSE);
							}
							else stick_x > 0.0{
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_START, true);
							};
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
						}/* else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
						}*/;
					};
					if status_kind == *FIGHTER_STATUS_KIND_ATTACK && frame > 7.0 && frame < 9.0{
						if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
							MotionModule::change_motion(boma, Hash40::new("attack_11"), 0.0, 1.0, false, 0.0, false, false);
						};
					};
					if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
						if check_jump(boma) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP){
							ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP);
							ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP_BUTTON);
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
							NO_WAVEDASH_TIMER[ENTRY_ID] = NO_WAVEDASH_MAX;
						};
					};
					if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR].contains(&status_kind) {
						if motion_kind == hash40("attack_air_n") {
							if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 || (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
							};
							if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
							};
							if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 || (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
							};
						};
						if motion_kind == hash40("attack_air_hi") {
							if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 || (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
							};
							if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 || (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
							};
							/*if check_jump(boma) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP){
								ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP);
								ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP_BUTTON);
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
							};*/
						};
						if [hash40("attack_air_f"), hash40("attack_air_b")].contains(&motion_kind) {
							if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 || (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
							};
						};
						if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
							if stick_x < 0.0{
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_START, true);
								WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_REVERSE);
							}
							else{
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_START, true);
							};
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
						};
						if hash40("landing_air_lw") == motion_kind {
							if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
							};
						};
					};
				};
			};
		}
    };
}		

pub fn install() {
    smashline::install_agent_frame_callbacks!(zss);
}