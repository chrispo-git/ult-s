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
	smashline::install_agent_frame_callbacks!(mk);
}
#[fighter_frame_callback]
pub fn mk(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
			let fighter_kind = smash::app::utility::get_kind(boma);
			if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
				if [hash40("attack_air_lw")].contains(&MotionModule::motion_kind(boma)) {
					if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) && MotionModule::frame(boma) < 37.0{
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
						MotionModule::set_frame_sync_anim_cmd(boma, 38.0, true, true, false);
						if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
							MotionModule::set_rate(boma, 0.5);
						};
					};
				};
			};
		}
	};
}