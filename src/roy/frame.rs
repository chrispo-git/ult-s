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
    Agent::new("roy")
    .set_costume(get_marked_costumes("roy","roy"))
    .on_line(Main, roy)
    .install();
}

unsafe extern "C" fn roy(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let speed = 5.8;
		if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
			if MotionModule::frame(boma) >= 36.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_RV, true);
			};
			if MotionModule::frame(boma) >= 22.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) == true {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_dash_2"), 0.0, 1.0, false, 0.0, false, false);
			};
		};
	};
}