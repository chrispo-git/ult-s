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
	Agent::new("mario")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, downb_pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, downb_main)
    .install();
}
unsafe extern "C" fn downb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
        StatusModule::init_settings(
            fighter.module_accessor,
            smash::app::SituationKind(*SITUATION_KIND_GROUND),
            *FIGHTER_KINETIC_TYPE_MOTION_AIR,
            *GROUND_CORRECT_KIND_GROUND as u32,
            smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
            0
        );

        0.into()
}
pub unsafe extern "C" fn downb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, smash::phx::Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion(fighter.module_accessor, smash::phx::Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    set_kinetic(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(downb_main_loop as *const () as _))
}
unsafe extern "C" fn downb_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        set_kinetic(fighter);
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
        if situation_kind == *SITUATION_KIND_AIR
        && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    if MotionModule::trans_move_speed(fighter.module_accessor).value[1] < 0.0 
    && fighter.sub_transition_group_check_air_landing().get_bool() {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
}
