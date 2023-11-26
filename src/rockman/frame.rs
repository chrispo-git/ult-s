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

#[fighter_frame_callback]
pub fn megaman(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let mut stick_x = ControlModule::get_stick_x(boma) ;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let nair_landing = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_n"), 0);
		if fighter_kind == *FIGHTER_KIND_ROCKMAN {
			let is_shooting = [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_AIR, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_LANDING, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE].contains(&status_kind);
			if status_kind == 54 {
				MEGA_AERIAL[ENTRY_ID] = true;
			};
			if status_kind != 54 && !is_shooting{
				MEGA_AERIAL[ENTRY_ID] = false;
			};
			if is_shooting && MEGA_AERIAL[ENTRY_ID] {
				WorkModule::set_float(boma, nair_landing-1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
			};
			//println!("Is Shooting: {}, Is Landing: {}, Status: {}", is_shooting, is_land_cancel, status_kind);
		};
	};
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(megaman);
}