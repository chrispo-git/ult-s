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

unsafe extern "C" fn attack_air_f_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
	if !attack_air_f_check(fighter) {
        return smashline::original_status(Exec, fighter, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F)(fighter);
    }
    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
    true.into()
}
unsafe fn attack_air_f_check(fighter: &mut L2CFighterCommon) -> bool {
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) {
        return false;
    }
    if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) || 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) >= 1 {
        return false;
    }
	if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) == 0{
        return false;
    }
    return true;
}
pub fn install() {
    Agent::new("bayonetta")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    	.status(Exec, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F, attack_air_f_exec)
        .install();
}