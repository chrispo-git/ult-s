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

pub unsafe fn hitstun_change(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize, hitstun_mul: f32) {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let remaining_hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    let total_hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
    if remaining_hitstun > 0.0 {
        if remaining_hitstun == total_hitstun {
            WorkModule::set_float(fighter.module_accessor, remaining_hitstun*hitstun_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            WorkModule::set_float(fighter.module_accessor, (remaining_hitstun*hitstun_mul)+1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
        }
    }
}
pub unsafe fn hitstun(fighter: &mut L2CFighterCommon,config : &config::Config) {
    if config.general.hitstun == 0 {
        return;
    }
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	match config.general.hitstun {
        0 => {},
        1 => {hitstun_change(fighter, status_kind, ENTRY_ID, 1.5)},
        _ => {hitstun_change(fighter, status_kind, ENTRY_ID, 0.75)},
    }
}

pub unsafe fn ledges(fighter: &mut L2CFighterCommon,config : &config::Config) {
    if config.general.ledges == 0 {
        return;
    }
	GroundModule::set_cliff_check(fighter.module_accessor, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE));
}

pub unsafe fn opff(fighter: &mut L2CFighterCommon,config : &config::Config) {
    hitstun(fighter, config);
    ledges(fighter, config);
}