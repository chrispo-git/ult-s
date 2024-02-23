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
use super::super::*;

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if LUCINA_STANCE[ENTRY_ID] == 0 {
		StatusModule::init_settings(
			fighter.module_accessor,
			smash::app::SituationKind(*SITUATION_KIND_NONE),
			*FIGHTER_KINETIC_TYPE_NONE,
			*GROUND_CORRECT_KIND_KEEP as u32,
			smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
			true,
			*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
			*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
			*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
			0
		);
	} else {
		StatusModule::init_settings(
			fighter.module_accessor,
			smash::app::SituationKind(*SITUATION_KIND_NONE),
			*FIGHTER_KINETIC_TYPE_NONE,
			*GROUND_CORRECT_KIND_KEEP as u32,
			smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
			true,
			*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
			*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
			*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
			0
		);
	}
	FighterStatusModuleImpl::set_fighter_status_data(
		fighter.module_accessor,
		false,
		*FIGHTER_TREADED_KIND_NO_REAC,
		false,
		false,
		false,
		(*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
		*FIGHTER_STATUS_ATTR_START_TURN as u32,
		*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
		0
	);
    0.into()
}
#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_FALL,
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
		(*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
		*FIGHTER_STATUS_ATTR_START_TURN as u32,
		*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
		0
	);
    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(special_hi_pre, special_lw_pre);
}