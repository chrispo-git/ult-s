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
    smashline::install_agent_frames!(kaz_frame);
}

#[fighter_frame( agent = FIGHTER_KIND_DEMON )]
fn kaz_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if [hash40("special_air_hi"), hash40("special_hi")].contains(&motion_kind) {
				if MotionModule::frame(boma) > 2.0 {
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
					};
				};
				if MotionModule::frame(boma) > 14.0 {
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
					};
				};
			};
			if [hash40("special_air_hi_start"), hash40("special_hi_start")].contains(&motion_kind) {
				MotionModule::set_rate(boma, 1.5);
			};
			//Original Endlag is 27
			//Original Startup is 10
			if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F || status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2{
				MotionModule::set_rate(boma, 1.0);
				if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2 {
					HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
			};
			if [hash40("attack_hi3")].contains(&motion_kind) && frame > 11.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
				if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
				};
				if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
				};
			};
		}
    }
}