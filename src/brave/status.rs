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
    Agent::new("brave")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    	.status(Exec, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_exec)
        .install();
}

unsafe extern "C" fn special_n_shoot_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) != hash40("special_air_n3") {
        crate::fastfall_land_cancel!(fighter, 10.0);
    }
    true.into()
}