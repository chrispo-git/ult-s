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

#[fighter_frame( agent = FIGHTER_KIND_WARIO )]
fn wario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //println!("It'sa me, Mario, wahoooooooo!");
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
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
		if situation_kind != *SITUATION_KIND_AIR && MotionModule::motion_kind(boma) != hash40("special_s_search"){
			BAN_SIDEB[ENTRY_ID] = false;
			HAS_BOUNCE[ENTRY_ID] = false;
		};
		if BAN_SIDEB[ENTRY_ID] == true {
			CAN_SIDEB[ENTRY_ID] = 1;
		} else {
			CAN_SIDEB[ENTRY_ID] = 0;
		};
		if MotionModule::motion_kind(boma) == hash40("special_s_search") && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && status_kind != *FIGHTER_STATUS_KIND_ATTACK_DASH{
			BAN_SIDEB[ENTRY_ID] = true;
			if MotionModule::frame(boma) >= 20.0 {
				if is_near_ground == 0 {
					if MotionModule::frame(boma) >= 70.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
				} else if MotionModule::frame(boma) > 31.0 {
						if MotionModule::frame(boma) < 37.0 {
							if !HAS_BOUNCE[ENTRY_ID] {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
							};
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
						};
				};
				if is_near_ground == 1 {
					MotionModule::set_rate(boma, 0.5);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if !HAS_BOUNCE[ENTRY_ID] && MotionModule::frame(boma) < 24.0 {
				//macros::SET_SPEED_EX(fighter, -0.3, -0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				let dummy = 0;
			} else {
				macros::COL_NORMAL(fighter);
			};
			if is_near_ground == 1 && !HAS_BOUNCE[ENTRY_ID]  && !IS_JUMP[ENTRY_ID]  && MotionModule::frame(boma) < 20.0 {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) { //|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
					WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
					macros::SET_SPEED_EX(fighter, 3.0, 2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
					KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
					macros::PLAY_SE(fighter, Hash40::new("se_wario_jump01"));
					IS_JUMP[ENTRY_ID] = true;
				};
			};
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL)  || GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32){
				if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !HAS_BOUNCE[ENTRY_ID] && !IS_JUMP[ENTRY_ID]{
					macros::PLAY_SE(fighter, Hash40::new("se_wario_landing01"));
					WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
					macros::SET_SPEED_EX(fighter, -0.5, 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
					KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
					macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
				};
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !HAS_BOUNCE[ENTRY_ID] {
					macros::PLAY_SE(fighter, Hash40::new("se_wario_landing01"));
					macros::SET_SPEED_EX(fighter, -0.5, 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
				};
				HAS_BOUNCE[ENTRY_ID] = true;
			}
			if HAS_BOUNCE[ENTRY_ID] &&  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 {
				if MotionModule::frame(boma) < 25.0 {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s_search"), 25.0, 1.0, false, 0.0, false, false);
				} else {
					MotionModule::set_rate(boma, 0.1);
					let stop_rise  = smash::phx::Vector3f { x: 0.0, y: 1.0, z: 1.0 };
					KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
				};
				if MotionModule::frame(boma) >= 26.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
				if MotionModule::frame(boma) >= 28.5 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
		} else {
			IS_JUMP[ENTRY_ID] = false;
		};
		if MotionModule::motion_kind(boma) == hash40("special_s_search") && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			if HAS_BOUNCE[ENTRY_ID] &&  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 {
				if MotionModule::frame(boma) < 25.0 {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s_search"), 25.0, 1.0, false, 0.0, false, false);
				} else {
					MotionModule::set_rate(boma, 0.2);
				};
			};
		};
		if [*FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WHEELIE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
			if MotionModule::motion_kind(boma) != hash40("special_s_search") {
				MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s_search"), 0.0, 1.0, 0.0, false, false);
			};
			if MotionModule::frame(boma) < 48.0 {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
				StatusModule::set_keep_situation_air(boma, true);
			} else if is_near_ground == 1{
				StatusModule::set_keep_situation_air(boma, false);
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			};
			/*if StatusModule::is_situation_changed(boma) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
			};*/	
			if [5.0, 20.0].contains(&MotionModule::frame(boma)){
				macros::COL_NORMAL(fighter);
				macros::FLASH(fighter, 1.0, 0.0, 0.0, 0.5);
			} else if [10.0, 25.0].contains(&MotionModule::frame(boma)){
				macros::COL_NORMAL(fighter);
				macros::FLASH(fighter, 0.0, 0.0, 1.0, 0.5);
			} else if [15.0].contains(&MotionModule::frame(boma)){
				macros::COL_NORMAL(fighter);
				macros::FLASH(fighter, 1.0, 1.0, 0.0, 0.5);
			} else if MotionModule::frame(boma) > 25.0{
				macros::COL_NORMAL(fighter);
			};
			if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					if !HAS_BOUNCE[ENTRY_ID] && !IS_JUMP[ENTRY_ID] {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
						};
					} else {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
						};
					};
					if !HAS_BOUNCE[ENTRY_ID] && !IS_JUMP[ENTRY_ID] && frame >= 42.0 {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
						};
					};
			} else {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
					};
			};
		};
		/*ArticleModule::remove_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_APPEAL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WHEELIE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_THROWN_OUT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START].contains(&status_kind) {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, true);
		};
		if status_kind == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH {
			if frame >= 69.0 {
				if situation_kind == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
			if frame >= 57.0 {
				if situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
				};
			};
			/*if MotionModule::frame(boma) < 38.0 {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
				StatusModule::set_keep_situation_air(boma, true);
			} else if is_near_ground == 1{
				StatusModule::set_keep_situation_air(boma, false);
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			};*/
			if situation_kind == *SITUATION_KIND_GROUND && motion_kind != hash40("special_s_search"){
				MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s_search"), 0.0, 1.0, 0.0, false, false);
			};
			if situation_kind == *SITUATION_KIND_AIR && motion_kind != hash40("special_s_search") && frame < 2.0{
				MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s_search"), 0.0, 1.0, 0.0, false, false);
			};
			/*if situation_kind == *SITUATION_KIND_AIR {
				StatusModule::set_keep_situation_air(boma, true);
			};*/
			if situation_kind == *SITUATION_KIND_AIR && motion_kind != hash40("special_s_search") && frame >= 2.0{
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};
			/*if StatusModule::is_situation_changed(boma) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};*/
			if frame < 7.0 && frame > 5.0 {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && ADD_DAMAGE[ENTRY_ID] <= 40{
					ADD_DAMAGE[ENTRY_ID] += 1;
					MotionModule::set_rate(boma, 0.0225);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			} else {
				MotionModule::set_rate(boma, 1.0);
				if frame > 20.0 {
					ADD_DAMAGE[ENTRY_ID] = 0;
				};
			};
			if frame < 2.0 {
					macros::SET_SPEED_EX(fighter, 0.0, -0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			};
			if frame < 38.0{ 
					if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) { 
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
						};
					} else {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
						};
					};
					if frame >= 9.0 && frame < 37.0{
						macros::SET_SPEED_EX(fighter, 1.5 + (ADD_DAMAGE[ENTRY_ID] as f32/40.0)*1.5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
					if frame == 37.0{
						macros::SET_SPEED_EX(fighter, 0.1, -0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
			} else {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
					};
			};
		} else {
			ADD_DAMAGE[ENTRY_ID] = 0;
		};*/
    }
}
#[weapon_frame( agent = WEAPON_KIND_WARIO_WARIOBIKE )]
fn bike_frame(weapon: &mut L2CFighterBase) {
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

pub fn install() {
    smashline::install_agent_frames!(
        wario_frame,
		bike_frame
    );
}