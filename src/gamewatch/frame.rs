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
	Agent::new("gamewatch")
	.on_line(Main, gnw)
	.install();
}		
		
unsafe extern "C" fn gnw(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let mut stick_x = ControlModule::get_stick_x(boma) ;
			stick_x *= PostureModule::lr(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			if fighter_kind == *FIGHTER_KIND_GAMEWATCH {
				if [hash40("landing_air_f")].contains(&MotionModule::motion_kind(boma)) {
					if MotionModule::frame(boma) > 15.0 {
						ArticleModule::remove(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
					};
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 && MotionModule::frame(boma) >= 26.0 {
					MotionModule::set_rate(boma, 0.54545);
				};
			};
		}
	};
}