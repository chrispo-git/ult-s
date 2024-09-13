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
    Agent::new("samus")
    .on_line(Main, samus_frame)
    .install();
}

unsafe extern "C" fn samus_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			if [hash40("attack_lw3")].contains(&motion_kind) {
				if check_jump(boma) && frame > 6.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD){
					CancelModule::enable_cancel(boma);
					NO_WAVEDASH_TIMER[ENTRY_ID] = NO_WAVEDASH_MAX;
				};
			};
			if NO_WAVEDASH_TIMER[ENTRY_ID] > 0{
				CAN_AIRDODGE[ENTRY_ID] = 1;
				NO_WAVEDASH_TIMER[ENTRY_ID] -= 1;
			} else {
				CAN_AIRDODGE[ENTRY_ID] = 0;
			};
		}
    }
}