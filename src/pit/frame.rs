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
pub fn pit_arrow_land_cancel(fighter : &mut L2CFighterCommon) {
    unsafe {	
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let lua_state = fighter.lua_state_agent;
		if fighter_kind == *FIGHTER_KIND_PIT {
			if  MotionModule::motion_kind(boma) == hash40("special_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DOWN, true);
			};
			if  MotionModule::motion_kind(boma) == hash40("special_air_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_air_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT && StatusModule::is_situation_changed(boma) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && MotionModule::frame(boma) > 11.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, true);
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END && MotionModule::frame(boma) > 45.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) == false{
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END && MotionModule::frame(boma) < 3.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) == false{
				let speed = smash::phx::Vector3f { x: 0.9, y: 0.0, z: 0.0 };
				KineticModule::add_speed(boma, &speed);
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END && MotionModule::frame(boma) > 8.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
				KineticModule::clear_speed_all(boma);
			};
		};
		if fighter_kind == *FIGHTER_KIND_KIRBY && WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_PIT{
			if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_SHOOT && StatusModule::is_situation_changed(boma) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
			};
		};
    }
}	

pub fn install() {
    smashline::install_agent_frame_callbacks!(pit_arrow_land_cancel);
}