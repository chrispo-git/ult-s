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
unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_whole(fighter.module_accessor, true);
    JostleModule::set_status(fighter.module_accessor, true);	
	EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40::new("samusd_entry"), false, false);
    0.into()
}	
unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_ATTACK) as i32
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        (*FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
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
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre)
        .install();

    Agent::new("samusd_missile")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
        .status(Exec, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, missile_exec)
        .install();
}