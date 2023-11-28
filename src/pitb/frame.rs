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

//Pit Downb nerf
#[fighter_frame_callback]
pub fn pitoo(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let situation_kind = StatusModule::situation_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if fighter_kind == *FIGHTER_KIND_PITB && is_default(boma) {
			if [hash40("attack_s3_s"), hash40("attack_dash")].contains(&motion_kind) && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)){
				if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0{ 
					StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, true);
				};
			};
			if  MotionModule::motion_kind(boma) == hash40("special_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DOWN, true);
			};
			if  MotionModule::motion_kind(boma) == hash40("special_air_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_air_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
			};
			if MotionModule::motion_kind(boma) == hash40("special_air_s_end") && MotionModule::frame(boma) >= 58.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, true);
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END {
				if situation_kind == *SITUATION_KIND_AIR {
					StatusModule::set_keep_situation_air(boma, true);
				};
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
				StatusModule::set_keep_situation_air(boma, true);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, false);
			};
			if situation_kind != *SITUATION_KIND_AIR {
				CAN_SIDEB[ENTRY_ID] = 0;
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH {
					CAN_SIDEB[ENTRY_ID] = 1;
					if MotionModule::frame(boma) > 6.0 {
						if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
						};
					};
					/*if MotionModule::frame(boma) > 8.0 {
						if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						};
					};*/
					if MotionModule::frame(boma) > 46.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, true);
					};
			};
		};
		/*if fighter_kind == *FIGHTER_KIND_KIRBY && WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_PITB{
			if (status_kind == *FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_SHOOT || status_kind == *FIGHTER_KIRBY_STATUS_KIND_PITB_SPECIAL_N_SHOOT) && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
				if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
				};
			};
			if situation_kind == *SITUATION_KIND_AIR && [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::get_stick_y(boma) >= 0.5{
					CHECK_FLOAT[ENTRY_ID] += 1;
				} else {
					CHECK_FLOAT[ENTRY_ID] = 0;
				};
				if CHECK_FLOAT[ENTRY_ID] >= CHECK_FLOAT_MAX && CHECK_FLOAT[ENTRY_ID] % 5 == 0 {
					if !is_hitlag(boma) {
						let speed = smash::phx::Vector3f { x: 0.0, y: (FLOAT_FALLSPEED)/2.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					} else {
						let speed = smash::phx::Vector3f { x: 0.0, y: (FLOAT_FALLSPEED*HITLAG_MULT)/2.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					};
					macros::EFFECT(fighter, Hash40::new("pitb_feather"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				};
			} else {
				CHECK_FLOAT[ENTRY_ID] = 0;
			};
		};
		if fighter_kind == *FIGHTER_KIND_PITB {
			if  MotionModule::motion_kind(boma) == hash40("special_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DOWN, true);
			};
			if  MotionModule::motion_kind(boma) == hash40("special_air_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_air_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
				if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
				};
			};
			if situation_kind == *SITUATION_KIND_AIR && [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::get_stick_y(boma) >= 0.5{
					CHECK_FLOAT[ENTRY_ID] += 1;
				} else {
					CHECK_FLOAT[ENTRY_ID] = 0;
				};
				if CHECK_FLOAT[ENTRY_ID] >= CHECK_FLOAT_MAX && CHECK_FLOAT[ENTRY_ID] % 5 == 0 {
					if !is_hitlag(boma) {
						let speed = smash::phx::Vector3f { x: 0.0, y: FLOAT_FALLSPEED, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					} else {
						let speed = smash::phx::Vector3f { x: 0.0, y: FLOAT_FALLSPEED*HITLAG_MULT, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					};
					macros::EFFECT(fighter, Hash40::new("pitb_feather"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				};
			} else {
				CHECK_FLOAT[ENTRY_ID] = 0;
			};
		};*/
	}
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(pitoo);
}