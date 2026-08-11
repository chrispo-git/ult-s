use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use std::os::raw::c_int;
use std::os::raw::c_ulong;
use crate::common::*;
use crate::util::*;
use crate::config;


pub unsafe fn cancel_to_taunt(fighter: &mut L2CFighterCommon,config : &config::Config) {
    if config.special.cancel_to_taunt == 0 {
        return;
    }
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_APPEAL) || StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND  {
        return;
    }
    if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) ||
    is_hitlag(boma(fighter)) {
        return;
    }
    let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
    if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 
		|| (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 
		|| (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 
		|| (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0  {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_APPEAL, false);
    }
}

pub unsafe fn taunt_cancel(fighter: &mut L2CFighterCommon,config : &config::Config) {
    if config.special.taunt_cancel == 0 {
        return;
    }
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_APPEAL {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

pub unsafe fn random_trip(fighter: &mut L2CFighterCommon,config : &config::Config) {
    if config.special.random_trip == 0 {
        return;
    }
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH) {
        if MotionModule::frame(fighter.module_accessor) as i32 == 5 {
            if smash::app::sv_math::rand(hash40("fighter"), 100) < 6 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SLIP, false);
            };
        }
    }
}

pub unsafe fn no_special_fall(fighter: &mut L2CFighterCommon,config : &config::Config) {
    if config.special.no_special_fall == 0 {
        return;
    }
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
        crate::transition_set!(ENTRY_ID, can_upb);
    }
    let state = crate::get_state!(ENTRY_ID, TransitionEnableState);
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR && state.can_upb == 1 {
        crate::transition_reset!(ENTRY_ID, can_upb);
    }
}

pub unsafe fn opff(fighter: &mut L2CFighterCommon,config : &config::Config) {
    no_special_fall(fighter, config);
    random_trip(fighter, config);
    taunt_cancel(fighter, config);
    cancel_to_taunt(fighter, config);
}