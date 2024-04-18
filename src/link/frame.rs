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
	Agent::new("link")
    .on_line(Main, link_frame)
    .install();
}

unsafe extern "C" fn link_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
            let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
            let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            //Make sure to put all statuses that use the weapon in here
            if ![*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) || [hash40("special_lw"), hash40("special_air_lw")].contains(&MotionModule::motion_kind(boma)) {
                ArticleModule::change_status_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK);
                let motion_kind = MotionModule::motion_kind(boma);
            };
        }
    }
}