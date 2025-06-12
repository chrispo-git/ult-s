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
    Agent::new("ridley")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_pre)
    .status(Pre, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, special_hi_charge_other_pre)
    .status(Pre, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B, special_hi_charge_other_pre)
    .status(Pre, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW, special_hi_charge_other_pre)
    .install();
}
unsafe extern "C" fn special_hi_charge_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_lr(fighter.module_accessor, -1.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_KEEP as u32,
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}	
unsafe extern "C" fn special_hi_charge_other_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI.into(),false.into());
    0.into()
}	