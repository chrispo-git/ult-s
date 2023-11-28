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
use crate::lucario::*;
pub fn install() {
	smashline::install_agent_frame_callbacks!(lucario);
}
#[fighter_frame_callback]
pub fn lucario(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let frame = MotionModule::frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32; 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION));  
		if fighter_kind == *FIGHTER_KIND_LUCARIO && is_default(boma) {
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
				if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION{
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
					};
				} else {
					CAN_DOWNB[ENTRY_ID] = 1;
					if frame < 29.0 {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
						};
					} else {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_FALL{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
						};
					}
				}
				smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 0.5);
			};
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
				CAN_DOWNB[ENTRY_ID] = 0;
			}
			if (cancel_frame - frame < 1.0 && cancel_frame != 0.0) || ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
				HAS_DOWNB[ENTRY_ID] = false;
			}
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && HAS_DOWNB[ENTRY_ID]{
				if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR && frame > 22.0 {
					if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_DASH, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
					}
				} else if frame > 16.0 {
					if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
						macros::SET_SPEED_EX(fighter, 1.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
					};
				}
			};
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) &&  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1{
				if [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) {
					if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && HAS_DOWNB[ENTRY_ID] == false {
						HAS_DOWNB[ENTRY_ID] = true;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
					};
				};
			};
		};
    };
}	