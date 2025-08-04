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

unsafe extern "C" fn missile_exec(fighter: &mut L2CFighterBase) -> L2CValue {
	0.into()
}	
unsafe extern "C" fn special_air_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Pre, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A)(fighter);
}	
unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Pre, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G)(fighter);
}	
unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(End, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G)(fighter);
}	
unsafe extern "C" fn special_air_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(End, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A)(fighter);
}	
unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Main, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G)(fighter);
}	
unsafe extern "C" fn special_air_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Main, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A)(fighter);
}	
unsafe extern "C" fn special_lw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Init, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G)(fighter);
}	
unsafe extern "C" fn special_air_lw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Init, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A)(fighter);
}	

pub fn install() {
    Agent::new("samusd")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
        .status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, special_air_lw_pre)
        .status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, special_lw_pre)
        .status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, special_air_lw_main)
        .status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, special_lw_main)
        .status(Init, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, special_air_lw_init)
        .status(Init, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, special_lw_init)
        .status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, special_lw_end)
        .status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, special_air_lw_end)
        .install();

    Agent::new("samusd_missile")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
        .status(Exec, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, missile_exec)
        .install();
}