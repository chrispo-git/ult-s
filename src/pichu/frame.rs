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
use smash::phx::Vector2f;
use crate::util::*;
use super::*;

#[fighter_frame( agent = FIGHTER_KIND_PICHU )]
fn pichu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let frame = MotionModule::frame(boma);
		let fallspeed = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
		let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true);
		if [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
			if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
				if GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true) == 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
					LAG_INCREASE[ENTRY_ID] = true;
				};
			};
		};
		if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && LAG_INCREASE[ENTRY_ID] == true {
			if cancel_frame-frame < 4.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
				LAG_INCREASE[ENTRY_ID] = false;
			};
		};
		if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR && LAG_INCREASE[ENTRY_ID] == true {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				LAG_INCREASE[ENTRY_ID] = false;
		};
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT, true);
		};
		if status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT {
			if !HAS_DOWNB[ENTRY_ID] && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
				HAS_DOWNB[ENTRY_ID] = true;
				DO_STALL[ENTRY_ID] = true;
			};
			if frame > 4.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
				if frame > 32.0 {
					DO_STALL[ENTRY_ID] = false; 
				}
				if DO_STALL[ENTRY_ID] {
						macros::SET_SPEED_EX(fighter, 0.0, fallspeed*-0.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				} else {
						macros::SET_SPEED_EX(fighter, 0.0, fallspeed*-1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				}
			}
			if is_near_ground == 1 && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
				StatusModule::set_keep_situation_air(boma, false);
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			}
		} else {
			DO_STALL[ENTRY_ID] = false;
		};
		if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
			HAS_DOWNB[ENTRY_ID] = false;
			DO_STALL[ENTRY_ID] = false;
		};
		if ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
			LAG_INCREASE[ENTRY_ID] = false;
		};
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        pichu_frame
    );
}