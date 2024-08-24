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
use smash::phx::Vector2f;
use crate::util::*;
use super::*;

pub fn install() {
    Agent::new("wario")
    .on_line(Main, wario_frame)
    .install();

	Agent::new("wario_wariobike")
    .on_line(Main, bike_frame)
    .install();
}

unsafe extern "C" fn wario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //println!("It'sa me, Mario, wahoooooooo!");
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let end_frame = MotionModule::end_frame(boma);
			let situation_kind = StatusModule::situation_kind(boma);
			let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true);
			println!("is near ground {}", is_near_ground);

			//Why are wario's proportions so wrong in ultimate?
			let armscale = smash::phx::Vector3f { x: 1.2, y: 1.2, z: 1.2 };
			ModelModule::set_joint_scale(boma, Hash40::new("shoulderl"), &armscale);
			ModelModule::set_joint_scale(boma, Hash40::new("shoulderr"), &armscale);
			let legscale = smash::phx::Vector3f { x: 0.95, y: 0.95, z:0.95 };
			ModelModule::set_joint_scale(boma, Hash40::new("footl"), &legscale);
			ModelModule::set_joint_scale(boma, Hash40::new("footr"), &legscale);

			if ![*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) && smash::app::sv_information::is_ready_go() {
				ArticleModule::remove_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if [*FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WHEELIE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
				if MotionModule::motion_kind(boma) != hash40("special_s_search") {
					MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s_search"), 0.0, 1.0, 0.0, false, false);
				} else if end_frame - frame < 2.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
				}
			};
		}
    }
}
unsafe extern "C" fn bike_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = StatusModule::status_kind(weapon.module_accessor);
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_WARIO {
			if !smash::app::sv_information::is_ready_go() {
				ModelModule::set_scale(weapon.module_accessor, 1.0);
			} else{
				ModelModule::set_scale(weapon.module_accessor, 0.0000001);
			};
		};
    }
}