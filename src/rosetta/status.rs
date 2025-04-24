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


unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_whole(fighter.module_accessor, true);
    JostleModule::set_status(fighter.module_accessor, true);	
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_POINTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into() 
}	

pub fn install() {
    Agent::new("rosetta")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end)
        .install();
}