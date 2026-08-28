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
    .set_costume(get_marked_costumes("master","master"))
    .on_line(Main, master_frame)
    .install();

	Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_line(Main, master_kirby_frame)
    .install();
}

unsafe extern "C" fn master_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		{
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
			let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
			let frame = MotionModule::frame(fighter.module_accessor) as i32;
			if smash::app::sv_information::is_ready_go() == false {
				VariableModule::set_flag(fighter.module_accessor, false, FIGHTER_MASTER_INSTANCE_WORK_ID_FLAG_IS_THUNDER);
			};
			if crate::is_in!(status_kind, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_STATUS_KIND_SPECIAL_N) {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
				if frame < 3 {
					if (ControlModule::get_stick_x(fighter.module_accessor)*PostureModule::lr(fighter.module_accessor)) < -0.2 {
						PostureModule::reverse_lr(fighter.module_accessor);
						PostureModule::update_rot_y_lr(fighter.module_accessor);
					};
				};
			};
		}
    }
}			
unsafe extern "C" fn master_kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		if smash::app::sv_information::is_ready_go() == false {
			VariableModule::set_flag(fighter.module_accessor false, FIGHTER_MASTER_INSTANCE_WORK_ID_FLAG_IS_THUNDER);
		};
	}
}