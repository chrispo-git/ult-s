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

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
fn richter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		if [hash40("attack_air_hi")].contains(&motion_kind) && frame >= 33.0 {
			MotionModule::change_motion(boma, smash::phx::Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
		};
		if ![hash40("haved"), hash40("fly")].contains(&ArticleModule::motion_kind(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))) {
			ArticleModule::remove_exist(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
    }
}
#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn richter_kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		if ![hash40("haved"), hash40("fly")].contains(&ArticleModule::motion_kind(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))) {
			ArticleModule::remove_exist(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
    }
}

pub fn install() {
    smashline::install_agent_frames!(richter_frame, richter_kirby_frame);
}