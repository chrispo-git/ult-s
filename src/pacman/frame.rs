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

#[fighter_frame( agent = FIGHTER_KIND_PACMAN )]
fn pacman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let situation_kind = StatusModule::situation_kind(boma);
		let end_frame = MotionModule::end_frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		if smash::app::sv_information::is_ready_go() == false {
			HYDRANT_POS_X[ENTRY_ID] = 0.0;
			HYDRANT_POS_Y[ENTRY_ID] = 0.0;
			TRAMPOLINE_POS_X[ENTRY_ID] = 0.0;
			TRAMPOLINE_POS_Y[ENTRY_ID] = 0.0;
			TRAMPOLINE_DELETE_TIMER[ENTRY_ID] = 0;
			HAS_UPB_ENDS[ENTRY_ID] = false;
		};
		if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) {
			HYDRANT_POS_X[ENTRY_ID] = 0.0;
			HYDRANT_POS_Y[ENTRY_ID] = 0.0;
		}
		if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_TRAMPOLINE) {
			TRAMPOLINE_POS_X[ENTRY_ID] = 0.0;
			TRAMPOLINE_POS_Y[ENTRY_ID] = 0.0;
			TRAMPOLINE_DELETE_TIMER[ENTRY_ID] = 0;
		}
		if TRAMPOLINE_DELETE_TIMER[ENTRY_ID] > 0 {
			TRAMPOLINE_DELETE_TIMER[ENTRY_ID] -= 1; 
		}
		if status_kind == *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP && frame > 5.0{
			StatusModule::change_status_request_from_script(boma, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN, false);
			HAS_UPB_ENDS[ENTRY_ID] = true;
		}
		if situation_kind != *SITUATION_KIND_AIR {
			HAS_UPB_ENDS[ENTRY_ID] = false;
			WorkModule::off_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);
		}
		if status_kind != *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN && situation_kind == *SITUATION_KIND_AIR && HAS_UPB_ENDS[ENTRY_ID]{
			if cancel_frame != 0.0 && cancel_frame - frame < 4.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
			}
		}
		if status_kind == *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN && situation_kind == *SITUATION_KIND_AIR && HAS_UPB_ENDS[ENTRY_ID]{
			let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
			let stable_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
			KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
			if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > stable_accel_y*-1.0 {
				let speed = smash::phx::Vector3f { x: 0.0, y: -0.18+accel_y, z: 0.0 };
				KineticModule::add_speed(boma, &speed);
			}
			if frame > 41.0{
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
			}
		}
		//println!("Jump num: {}", WorkModule::get_int(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_HI_JUMP_NUM));
		//println!("Hydrant [{}, {}] Trampoline [{}, {}]", HYDRANT_POS_X[ENTRY_ID], HYDRANT_POS_Y[ENTRY_ID], TRAMPOLINE_POS_X[ENTRY_ID], TRAMPOLINE_POS_Y[ENTRY_ID]);
	}
}
#[weapon_frame( agent = WEAPON_KIND_PACMAN_FIREHYDRANT )]
fn hydrant_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_PACMAN {
			let mut offset = 6.0; //Allows Trampoline to bounce standing hydrant while keeping the falling hydrant accurate
			if KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
				offset = 0.0;
			}
			HYDRANT_POS_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
			HYDRANT_POS_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor) + offset;
			if ((HYDRANT_POS_X[ENTRY_ID]  - TRAMPOLINE_POS_X[ENTRY_ID]).abs() < 11.0) &&
				((HYDRANT_POS_Y[ENTRY_ID]  - TRAMPOLINE_POS_Y[ENTRY_ID]).abs() < 3.0) &&
				(ArticleModule::is_exist(&mut *boma, *FIGHTER_PACMAN_GENERATE_ARTICLE_TRAMPOLINE) && TRAMPOLINE_POS_Y[ENTRY_ID] != 0.0)  {
					if status_kind != *WEAPON_PACMAN_FIREHYDRANT_STATUS_KIND_FLY {
						StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PACMAN_FIREHYDRANT_STATUS_KIND_FLY, false);
						macros::SET_SPEED_EX(weapon, 0.0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						WorkModule::on_flag(weapon.module_accessor, *WEAPON_PACMAN_FIREHYDRANT_INSTANCE_WORK_ID_FLAG_HIT);
					} else if KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
						StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PACMAN_FIREHYDRANT_STATUS_KIND_FLY, false);
						macros::SET_SPEED_EX(weapon, 0.0, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						WorkModule::on_flag(weapon.module_accessor, *WEAPON_PACMAN_FIREHYDRANT_INSTANCE_WORK_ID_FLAG_HIT);
					}
			}
		};
    }
}
#[weapon_frame( agent = WEAPON_KIND_PACMAN_TRAMPOLINE )]
fn trampoline_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_PACMAN {
			TRAMPOLINE_POS_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
			TRAMPOLINE_POS_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
			if ((HYDRANT_POS_X[ENTRY_ID]  - TRAMPOLINE_POS_X[ENTRY_ID]).abs() < 11.0) &&
				((HYDRANT_POS_Y[ENTRY_ID]  - TRAMPOLINE_POS_Y[ENTRY_ID]).abs() < 3.0) &&
				(ArticleModule::is_exist(&mut *boma, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) && HYDRANT_POS_Y[ENTRY_ID] != 0.0) &&
				(![*WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_SHAKE, *WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_REMOVE].contains(&status_kind) || TRAMPOLINE_DELETE_TIMER[ENTRY_ID] == 0) {
					StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_SHAKE, false);
					TRAMPOLINE_DELETE_TIMER[ENTRY_ID] = 30;
			}
			if TRAMPOLINE_DELETE_TIMER[ENTRY_ID] == 1 {
				StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_REMOVE, false);
			}
		};
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        pacman_frame,
		hydrant_frame,
		trampoline_frame  
    );
}