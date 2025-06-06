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
    Agent::new("trail")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_line(Main, sora)
    .install();
}

unsafe extern "C" fn sora(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
			let end_frame = MotionModule::end_frame(boma);
			if fighter_kind == *FIGHTER_KIND_TRAIL {
				if motion_kind == hash40("attack_air_f") {
					WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLOAT_LANDING_ATTACK_AIR_FRAME_F);
				};
				if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false || StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
					HAS_WALLJUMP[ENTRY_ID] = false;
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					if TO_FAIR[ENTRY_ID] == true && MotionModule::motion_kind(boma) != hash40("attack_air_f"){
						MotionModule::change_motion(boma, Hash40::new("attack_air_f"), 2.0, 1.0, false, 0.0, false, false);
					};
					if !WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) && MotionModule::frame(boma) > 5.0 {
						TO_FAIR[ENTRY_ID] = false;
					};
				} else if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
					if TO_FAIR[ENTRY_ID] == true && MotionModule::motion_kind(boma) != hash40("landing_air_f"){
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_f"), 0)))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::change_motion(boma, Hash40::new("landing_air_f"), 2.0, landing, false, 0.0, false, false);
					};
				} else {
					TO_FAIR[ENTRY_ID] = false;
				};
			};
		}
    }
}