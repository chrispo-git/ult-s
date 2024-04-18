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
use crate::master::*;
use super::*;

pub fn install() {
    Agent::new("master")
    .on_line(Main, master_frame)
    .install();

	Agent::new("kirby")
    .on_line(Main, master_kirby_frame)
    .install();
}

unsafe extern "C" fn master_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if MotionModule::motion_kind(boma) == hash40("special_air_s_front") || MotionModule::motion_kind(boma) == hash40("special_air_s_front_dash") || MotionModule::motion_kind(boma) == hash40("special_s_front") || MotionModule::motion_kind(boma) == hash40("special_s_front_dash") || MotionModule::motion_kind(boma) == hash40("special_s_start") || MotionModule::motion_kind(boma) == hash40("special_air_s_start") {
				ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if smash::app::sv_information::is_ready_go() == false {
				IS_THUNDER[ENTRY_ID] = false;
			};
			if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT, true);
			};
			if MotionModule::motion_kind(boma) == hash40("special_s_front") || MotionModule::motion_kind(boma) == hash40("special_air_s_front")|| MotionModule::motion_kind(boma) == hash40("special_air_s_front_dash")|| MotionModule::motion_kind(boma) == hash40("special_s_front_dash"){
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
					CancelModule::enable_cancel(boma);
				};
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
				if MotionModule::frame(boma) < 3.0 {
					if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
						PostureModule::reverse_lr(boma);
						PostureModule::update_rot_y_lr(boma);
					};
				};
			};
			if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT].contains(&status_kind) {
				if motion_duration(boma) == 2 {
					if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
						PostureModule::reverse_lr(boma);
						PostureModule::update_rot_y_lr(boma);
						let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
						KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
				};
			};
			if [hash40("special_lw_landing_1"), hash40("special_lw_landing_2")].contains(&MotionModule::motion_kind(boma)) && motion_duration(boma) > 14 {
				CancelModule::enable_cancel(boma);
			};
		}
    }
}			
unsafe extern "C" fn master_kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if smash::app::sv_information::is_ready_go() == false {
			IS_THUNDER[ENTRY_ID] = false;
		};
	}
}