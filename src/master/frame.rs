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
}

unsafe extern "C" fn master_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		{
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
				if MotionModule::frame(boma) < 3.0 {
					if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
						PostureModule::reverse_lr(boma);
						PostureModule::update_rot_y_lr(boma);
					};
				};
			};
			if [hash40("special_lw_landing_1"), hash40("special_lw_landing_2")].contains(&MotionModule::motion_kind(boma)) && motion_duration(boma) > 14 {
				CancelModule::enable_cancel(boma);
			};
		}
    }
}			