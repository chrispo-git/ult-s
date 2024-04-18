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
    Agent::new("kamui")
	.on_line(Main, kamui_frame)
	.install();
}

unsafe extern "C" fn kamui_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if status_kind == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP){
				if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
				};
		};
		if [*FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) {
			EffectModule::kill_kind(boma, smash::phx::Hash40::new("sys_smash_flash"), false, false);
			EffectModule::kill_kind(boma, smash::phx::Hash40::new("sys_smash_flash_s"), false, false);
		};
		if [*FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) {
			if MotionModule::frame(boma) > 8.0 && MotionModule::frame(boma) < 33.0 {
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_rspearhand"),true);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_weapon"),false);
			} else {
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_rspearhand"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_weapon"),true);
			};
		};
		if (StopModule::is_hit(boma) || StopModule::is_damage(boma) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) == false) || smash::app::sv_information::is_ready_go() == false {
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_rspearhand"),false);
			if [*FIGHTER_STATUS_KIND_LOSE].contains(&status_kind) {
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_weapon"),false);
			} else {
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_weapon"),true);
			};
		};
    }
}