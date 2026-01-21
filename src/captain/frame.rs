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
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_line(Main, captain_frame)
    .install();
}
unsafe fn cancel_to_falcon_punch(fighter: &mut L2CFighterCommon, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
        return;
    }
    if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) ||
    is_hitlag(boma(fighter)) {
        return;
    }
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
    }
}
unsafe extern "C" fn captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        cancel_to_falcon_punch(fighter, StatusModule::status_kind(fighter.module_accessor));
    }
}