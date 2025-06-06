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
    Agent::new("miiswordsman")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_HOLD, sword_specials2_pre)
	.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_HIT, sword_aa_pre)
	.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_HIT, sword_aa_main)
    .install();
}

unsafe extern "C" fn sword_specials2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH);
    1.into()
}
unsafe extern "C" fn sword_aa_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
	let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
	if situation_kind == *SITUATION_KIND_GROUND {
		StatusModule::init_settings(
			fighter.module_accessor,
			smash::app::SituationKind(*SITUATION_KIND_GROUND),
			*FIGHTER_KINETIC_TYPE_NONE,
			*GROUND_CORRECT_KIND_KEEP as u32,
			smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
			true,
			*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
			*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
			*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
			0
		);
	
		FighterStatusModuleImpl::set_fighter_status_data(
			fighter.module_accessor,
			false,
			*FIGHTER_TREADED_KIND_NO_REAC,
			false,
			false,
			false,
			(*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
			0,
			*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
			0
		);
		0.into()
	} else {
		return smashline::original_status(Pre, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_HIT)(fighter);
    }
}
unsafe extern "C" fn sword_aa_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32; //Cancel frame
	let frame = MotionModule::frame(fighter.module_accessor);
	if situation_kind == *SITUATION_KIND_GROUND {
		if MotionModule::motion_kind(fighter.module_accessor) != hash40("special_ground_s1_hit") {
			KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
			MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_ground_s1_hit"), 0.0, 1.0, false, 0.0, false, false);
		} else {
			if frame >= cancel_frame && cancel_frame > 0.0 {
				if (
					(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
					(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
					(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 ||
					(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
					(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 ||
					(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 ||
					(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 ||
					(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 ||
					(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 ||
					(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 ||
					ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
					(ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))||
					(ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) && ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_stick_y(fighter.module_accessor) >= 0.7 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))
				) {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
				}
			}
		}
		if MotionModule::is_end(fighter.module_accessor) {
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
		}
		0.into()
	} else {
		return smashline::original_status(Main, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_HIT)(fighter);
    }
}