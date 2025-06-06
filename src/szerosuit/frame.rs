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
use crate::szerosuit::*;
use super::*;

pub fn install() {
    Agent::new("szerosuit")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_line(Main, zss)
    .install();
}

unsafe extern "C" fn zss(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		if is_default(boma) {
			let fighter_kind = smash::app::utility::get_kind(boma);
			let lr = PostureModule::lr(boma);
			let stick_x = ControlModule::get_stick_x(boma)* lr;		
			let stick_y = ControlModule::get_stick_y(boma);		
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let situation_kind = StatusModule::situation_kind(boma);
			let frame = MotionModule::frame(boma);
			if fighter_kind == *FIGHTER_KIND_SZEROSUIT {
				
			};
		}
    };
}		