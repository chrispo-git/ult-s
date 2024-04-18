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
    Agent::new("littlemac")
    .on_line(Main, mac_frame)
    .install();

	Agent::new("kirby")
    .on_line(Main, kirby_mac_frame)
    .install();
}

unsafe extern "C" fn mac_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if [*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH_TURN].contains(&status_kind) {
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && frame < 19.0 && frame > 13.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						let cat1 = ControlModule::get_command_flag_cat(boma, 0);
						if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT, true);
						};
				};
				if StatusModule::is_situation_changed(boma) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				};
			};
			if [*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START].contains(&status_kind) {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					if status_duration(boma) > 12 && status_duration(boma) < 32 {
						let cat1 = ControlModule::get_command_flag_cat(boma, 0);
						if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH, true);
						};
					};
				};
			};
		}
    }
}
unsafe extern "C" fn kirby_mac_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_LITTLEMAC {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if [*FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_DASH, *FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_DASH_TURN].contains(&status_kind) {
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && frame < 19.0 && frame > 13.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						let cat1 = ControlModule::get_command_flag_cat(boma, 0);
						if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
						};
				};
				if StatusModule::is_situation_changed(boma) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				};
			};
			if [*FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_START].contains(&status_kind) {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					if status_duration(boma) > 12 && status_duration(boma) < 32 {
						let cat1 = ControlModule::get_command_flag_cat(boma, 0);
						if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_DASH, true);
						};
					};
				};
			};
		};
    }
}