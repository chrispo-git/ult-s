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
use crate::ike::*;

pub fn install() {
	Agent::new("ike")
	.on_line(Main, ike)
	.install();
}

pub(crate) fn check_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
			return true;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
			if ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_stick_y(boma) >= 0.7 {
				return true;
			};
		};
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
			return true;
		};
		return false;
	}
}

//Ike
unsafe extern "C" fn ike(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		if is_default(boma) {
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
				MotionModule::set_rate(boma, 10.0);
			};
			if status_kind == *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, true);
			};
			if status_kind == *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
				if MotionModule::frame(boma) >= 25.0 {
					CancelModule::enable_cancel(boma);
					IKE_INSTALL[ENTRY_ID] = IKE_INSTALL_TIME;
					EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x10ae069777), false, false);
				};
			};
			if IKE_INSTALL[ENTRY_ID] > 0 && [*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK].contains(&status_kind) && check_jump(boma) && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD){
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
				} else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
				};
			};
			if IKE_INSTALL[ENTRY_ID] > 0 {
				IKE_INSTALL[ENTRY_ID] -= 1;
			} else {
				EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x10ae069777), false, false);
			};
			if [*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_ITEM, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, *FIGHTER_STATUS_KIND_CAPTURE_WAIT].contains(&status_kind) {
				IKE_INSTALL[ENTRY_ID] = 0;
			};
			if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
				IKE_INSTALL[ENTRY_ID] = 0;
			};
			if IKE_INSTALL[ENTRY_ID] > 0{
				TIMER[ENTRY_ID] += 1;
				if [*FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind) {
					TIMER[ENTRY_ID] = 3;
				};
				if TIMER[ENTRY_ID] >= 5 {
					TIMER[ENTRY_ID] = 0;
					let fire1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("sword"), &S1, &S1, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let fire2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("sword"), &S2, &S2, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let fire3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("sword"), &S3, &S3, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, fire1, 0.0, 0.8, 15.0);
					EffectModule::set_rgb(boma, fire2, 0.0, 0.8, 15.0);
					EffectModule::set_rgb(boma, fire3, 0.0, 0.8, 15.0);
				};
			};
			if IKE_INSTALL[ENTRY_ID] > 0 {
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
					if MotionModule::frame(boma) >= 22.0 {
						CancelModule::enable_cancel(boma);
					};
				}else if motion_kind == hash40("attack_11"){
					if MotionModule::frame(boma) >= 12.0 {
						CancelModule::enable_cancel(boma);
					};
				}else if motion_kind == hash40("throw_lw"){
					if MotionModule::frame(boma) >= 53.0 {
						CancelModule::enable_cancel(boma);
					};
				}else if motion_kind == hash40("throw_hi"){
					if MotionModule::frame(boma) >= 33.0 {
						CancelModule::enable_cancel(boma);
					};
				}else if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
					if motion_kind == hash40("landing_air_n") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_n"), 0)-3.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					} else if motion_kind == hash40("landing_air_f") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_f"), 0)-3.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					} else if motion_kind == hash40("landing_air_b") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_b"), 0)-3.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					} else if motion_kind == hash40("landing_air_hi") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_hi"), 0)-3.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					} else {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_lw"), 0)-3.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					};
				} else if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF {
						let landing = 1.0/(((FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 )-3.0)/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
				} else if status_kind == *FIGHTER_STATUS_KIND_DASH {
						if frame >= 8.0 {
							WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
						};
				}else if status_kind != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
					if FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 > 3.0 {
						if frame >= (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 - 3.0) {
							CancelModule::enable_cancel(boma);
						};
					};
				};
			};
		};
		if fighter_kind == *FIGHTER_KIND_KIRBY {
			if status_kind == *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_LOOP {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END, true);
			};
			if status_kind == *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N {
				MotionModule::set_rate(boma, 3.0);
			};
			if status_kind == *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
				if MotionModule::frame(boma) >= 25.0 {
					CancelModule::enable_cancel(boma);
					IKE_INSTALL[ENTRY_ID] = IKE_INSTALL_TIME;
					EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x10ae069777), false, false);
				};
			};
			if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
				IKE_INSTALL[ENTRY_ID] = 0;
			};
			if IKE_INSTALL[ENTRY_ID] > 0 {
				IKE_INSTALL[ENTRY_ID] -= 1;
			} else {
				EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x10ae069777), false, false);
			};
			if [*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_ITEM, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, *FIGHTER_STATUS_KIND_CAPTURE_WAIT].contains(&status_kind) {
				IKE_INSTALL[ENTRY_ID] = 0;
			};
			if IKE_INSTALL[ENTRY_ID] > 0{
				TIMER[ENTRY_ID] += 1;
				if TIMER[ENTRY_ID] >= 5 {
					TIMER[ENTRY_ID] = 0;
					let fire1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("havel"), &S3, &S3, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let fire2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("haver"), &S3, &S3, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, fire1, 0.0, 0.8, 15.0);
					EffectModule::set_rgb(boma, fire2, 0.0, 0.8, 15.0);
				};
			};
			if IKE_INSTALL[ENTRY_ID] > 0 {
				if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
					if motion_kind == hash40("landing_air_f") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_f"), 0)-2.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					} else if motion_kind == hash40("landing_air_b") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_b"), 0)-3.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					} else if motion_kind == hash40("landing_air_hi") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_hi"), 0)-1.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					} else if motion_kind == hash40("landing_air_lw") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_lw"), 0)-3.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					};
				} else if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF {
						let landing = 1.0/(((FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 )-3.0)/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
				} else if status_kind == *FIGHTER_STATUS_KIND_DASH {
						if frame >= 8.0 {
							WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
						};
				}else if status_kind != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
					if FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 > 3.0 {
						if frame >= (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 - 3.0) {
							CancelModule::enable_cancel(boma);
						};
					};
				};
			};
		};
    };
}	