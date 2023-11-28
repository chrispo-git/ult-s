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

#[fighter_frame_callback]
pub fn sora(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
			let end_frame = MotionModule::end_frame(boma);
			if fighter_kind == *FIGHTER_KIND_TRAIL {
				if motion_kind == hash40("attack_air_f") {
					WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLOAT_LANDING_ATTACK_AIR_FRAME_F);
				};
				if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false || StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
					HAS_WALLJUMP[ENTRY_ID] = false;
				};
				/*if motion_kind == hash40("attack_air_lw") {
					if frame > 14.0 && frame < 20.0 {
						MotionModule::change_motion(boma, Hash40::new("attack_air_lw"), 41.0, 1.0, false, 0.0, false, false);
					};
				};*/
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					if frame > 9.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_LW_ATTACK, true);
					};
					if frame as i32 % 3 == 0 {
						let a1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
						let a2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
						let a3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footr"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
						let a4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footl"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
						let a5: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("hip"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
						EffectModule::set_rgb(boma, a1, 2.2, 0.4, 1.0);
						EffectModule::set_rgb(boma, a2, 2.2, 0.4, 1.0);
						EffectModule::set_rgb(boma, a3, 2.2, 0.4, 1.0);
						EffectModule::set_rgb(boma, a4, 2.2, 0.4, 1.0);
						EffectModule::set_rgb(boma, a5, 2.2, 0.4, 1.0);
					};
					if motion_duration(boma) == 1 {
						if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
							PostureModule::reverse_lr(boma);
							PostureModule::update_rot_y_lr(boma);
						};
					};
					if motion_duration(boma) == 5 {
						if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
							PostureModule::reverse_lr(boma);
							PostureModule::update_rot_y_lr(boma);
							let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
							KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						};
					};
				};
				if status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F {
					let dummy = 0;
					//if MotionModule::frame(boma) > 2.0 && MotionModule::frame(boma) < 4.0 {
						//TO_FAIR[ENTRY_ID] = true;
						//StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
					//};
				} else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					if TO_FAIR[ENTRY_ID] == true && MotionModule::motion_kind(boma) != hash40("attack_air_f"){
						MotionModule::change_motion(boma, Hash40::new("attack_air_f"), 2.0, 1.0, false, 0.0, false, false);
					};
					if !WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) && MotionModule::frame(boma) > 5.0 {
						TO_FAIR[ENTRY_ID] = false;
					};
				} else if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
					if TO_FAIR[ENTRY_ID] == true && MotionModule::motion_kind(boma) != hash40("landing_air_f"){
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_f"), 0)))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::change_motion(boma, Hash40::new("landing_air_f"), 2.0, landing, false, 0.0, false, false);
					};
				} else {
					TO_FAIR[ENTRY_ID] = false;
				};
				if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_LW_ATTACK {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					if end_frame-frame < 4.0 || (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && frame > 12.0) {
						if GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -2.0}, false) == 1{
							StatusModule::set_keep_situation_air(boma, false);
							StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END, true);
						};
					} else {
						StatusModule::set_keep_situation_air(boma, true);
						if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
							StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
						};
					};
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
					};
					if frame < 9.0 {
						macros::SET_SPEED_EX(fighter, frame * 0.125, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					} else if frame > 20.0 {
						macros::SET_SPEED_EX(fighter, (26.0 - frame) * 0.125, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					} else {
						macros::SET_SPEED_EX(fighter, 1.25, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
					MotionModule::set_rate(boma, 0.66667);
					if frame as i32 % 5 == 0 {
						let a1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
						let a2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
						let a3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footr"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
						let a4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footl"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
						let a5: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("hip"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
						EffectModule::set_rgb(boma, a1, 2.2, 0.4, 1.0);
						EffectModule::set_rgb(boma, a2, 2.2, 0.4, 1.0);
						EffectModule::set_rgb(boma, a3, 2.2, 0.4, 1.0);
						EffectModule::set_rgb(boma, a4, 2.2, 0.4, 1.0);
						EffectModule::set_rgb(boma, a5, 2.2, 0.4, 1.0);
					};
					if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) && (ControlModule::get_stick_x(boma)*PostureModule::lr(boma))< -0.7 && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)  && !HAS_WALLJUMP[ENTRY_ID]{
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
						HAS_WALLJUMP[ENTRY_ID] = true;
					};
					if !is_hitlag(boma) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
						if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						} else {
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
							StatusModule::change_status_request_from_script(boma, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, true);
						};
					};
				};
			};
		}
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(sora);
}