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
    smashline::install_agent_frames!(
        bayo_frame
    );
}


#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
fn bayo_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let can_sideb = !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
			let can_upb = !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
			let stick_y = ControlModule::get_stick_y(boma);
			let cat1 = ControlModule::get_command_flag_cat(boma, 0);
			if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F {
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1{
					if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 && can_sideb {
						if stick_y <= -0.5 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, true);
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, true);
						};
					};
				};
			};
		}
    }
}	