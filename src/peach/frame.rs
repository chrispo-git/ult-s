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
use smash::phx::Vector2f;
use super::*;

pub fn install() {
    Agent::new("peach")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_line(Main, peach_frame)
    .install();

	Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
	.on_line(Main, kirby_frame)
    .install();
}

unsafe extern "C" fn peach_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //println!("It'sa me, Mario, wahoooooooo!");
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let is_near_ground = GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true);
        if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let frame = MotionModule::frame(boma);
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
				if StatusModule::is_situation_changed(boma) {
					ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				};
				if frame >= 40.0 {
					CancelModule::enable_cancel(boma);
				};
				if frame >= 36.0 {
					StatusModule::set_keep_situation_air(boma, false);
				} else {
					StatusModule::set_keep_situation_air(boma, true);
				};
				if is_near_ground == 1 {
                    KineticModule::clear_speed_all(boma);
				}
			};
		}
		
    }
}
unsafe extern "C" fn kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //println!("It'sa me, Mario, wahoooooooo!");
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let frame = MotionModule::frame(boma);
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PEACH_SPECIAL_N {
			if StatusModule::is_situation_changed(boma) {
				ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if frame >= 40.0 {
				CancelModule::enable_cancel(boma);
			};
			if frame >= 36.0 {
				StatusModule::set_keep_situation_air(boma, false);
			} else {
				StatusModule::set_keep_situation_air(boma, true);
			};
		};
		
    }
}