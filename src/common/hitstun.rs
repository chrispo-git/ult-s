use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lib::L2CAgent;
use smash::lib::L2CValue;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::phx::Hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::*;
use crate::util::*;

pub unsafe fn opff(fighter: &mut L2CFighterCommon, status_kind : i32) {
    kd_throw(fighter, status_kind);
    non_tumble_di(fighter, status_kind);
}

pub unsafe fn kd_throw(fighter : &mut L2CFighterCommon, status_kind : i32) {
        if ![*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
            return;
        }
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if IS_KD_THROW[ENTRY_ID] {
            IS_KD_THROW[ENTRY_ID] = false;
            KineticModule::clear_speed_all(fighter.module_accessor);
            fighter.clear_lua_stack();
			lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
			smash::app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
            let y_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_y_stable"));
			macros::SET_SPEED_EX(fighter, 0, -y_speed, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
}
pub unsafe fn non_tumble_di(fighter : &mut L2CFighterCommon, status_kind : i32) {
		if !is_mechanics_enabled() {
            return;
        }
        let status_kind_prev = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR].contains(&status_kind) {
            if StopModule::get_hit_stop_real_frame(fighter.module_accessor) as i32 == 1{
                smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
            }
            if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR].contains(&status_kind)
                && [*FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_THROWN].contains(&status_kind_prev) {
                let remaining_hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
                let total_hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
                let hitstun_passed = total_hitstun - remaining_hitstun;
                if total_hitstun > 0.0 && hitstun_passed == 1.0 {
                    smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
                }
            }
        }
}
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon::ftStatusUniqProcessDamage_exec_common)]
pub unsafe fn ftStatusUniqProcessDamage_exec_common_hook(fighter: &mut L2CFighterCommon){
    let ret = original!()(fighter);
	if !is_mechanics_enabled() {
        ret;
    }
    smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
    println!("hook for hitstun being triggered");

    ret
}
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon::ftStatusUniqProcessDamage_exec)]
pub unsafe fn ftStatusUniqProcessDamage_exec_hook(fighter: &mut L2CFighterCommon){
    let ret = original!()(fighter);
	if !is_mechanics_enabled() {
        ret;
    }
    smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
    println!("hook2 for hitstun being triggered");

    ret
}


pub fn install() {
    skyline::install_hooks!(ftStatusUniqProcessDamage_exec_common_hook, ftStatusUniqProcessDamage_exec_hook);
}