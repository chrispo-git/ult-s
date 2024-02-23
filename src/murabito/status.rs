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
    smashline::install_status_scripts!(special_n_pre, special_n_p_pre, special_n_t_pre);
}
#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if !is_added(boma) {
        if SPEED_Y[ENTRY_ID] > 2.0 {
            let speed = smash::phx::Vector3f { x: 0.0, y: (SPEED_Y[ENTRY_ID]-2.0)*-1.0, z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speed);
        }
        fighter.change_status(FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH.into(), false.into());
        0.into()
    } else {
        original!(fighter)
    }
}
#[status_script(agent = "murabito", status = FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_n_p_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if !is_added(boma) {
        if SPEED_Y[ENTRY_ID] > 2.0 {
            let speed = smash::phx::Vector3f { x: 0.0, y: (SPEED_Y[ENTRY_ID]-2.0)*-1.0, z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speed);
        }
        fighter.change_status(FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH.into(), false.into());
        0.into()
    } else {
        original!(fighter)
    }
}
#[status_script(agent = "murabito", status = FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_TAKE_OUT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_n_t_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if !is_added(boma) {
        if SPEED_Y[ENTRY_ID] > 2.0 {
            let speed = smash::phx::Vector3f { x: 0.0, y: (SPEED_Y[ENTRY_ID]-2.0)*-1.0, z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speed);
        }
        fighter.change_status(FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH.into(), false.into());
        0.into()
    } else {
        original!(fighter)
    }
}