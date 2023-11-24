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
use crate::mario::*;
use super::*;
pub fn install() {
    smashline::install_agent_frame_callbacks!(
        mario_frame
    );
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


#[fighter_frame_callback]
pub fn mario_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if fighter_kind == *FIGHTER_KIND_MARIO {
			if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) == false {
				ArticleModule::remove_exist(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					if MotionModule::frame(boma) > 9.0 && MotionModule::frame(boma) < 73.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) == false {
						if check_jump(boma) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
						};
					};
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
					};
				} else {
					SIDEB[ENTRY_ID] = true;
					if MotionModule::frame(boma) < 23.0 {
						if MotionModule::frame(boma) < 2.0 {
							KineticModule::clear_speed_all(boma);
							let speed = smash::phx::Vector3f { x: 1.5, y: 1.5, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
						} else {
							KineticModule::clear_speed_all(boma);
							let speed = smash::phx::Vector3f { x: 0.5, y: 0.5, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
						};
					};
					if MotionModule::frame(boma) < 46.0 {
						StatusModule::set_keep_situation_air(boma, true);
					} else {
						StatusModule::set_keep_situation_air(boma, false);
					};
					if MotionModule::frame(boma) > 74.0 {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
						};
						CancelModule::enable_cancel(boma);
					} else {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
						};
					};
					if MotionModule::frame(boma) > 56.0 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0{
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					};
				};
			};
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
				SPIN[ENTRY_ID] = false;
				SIDEB[ENTRY_ID] = false;
			};
			if [hash40("attack_air_n")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) > 9.0 && MotionModule::frame(boma) < 22.0 && StopModule::is_stop(boma) == false {
				if SPIN_EFF[ENTRY_ID] == 0{
					let handbg1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN2, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let handbg2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN3, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let handbg3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN4, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let handbg4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN5, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let hand1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN1, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR1, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR2, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR3, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("handl"), &NOSPIN, &NOSPIN, 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star5: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("handr"), &NOSPIN, &NOSPIN, 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star6: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR4, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star7: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR5, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, hand1, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, hand1, 0.2);
					EffectModule::set_rgb(boma, handbg1, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, handbg1, 0.15);
					EffectModule::set_rgb(boma, handbg2, 0.045, 0.045, 2.05);
					EffectModule::set_alpha(boma, handbg2, 0.15);
					EffectModule::set_rgb(boma, handbg3, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, handbg3, 0.15);
					EffectModule::set_rgb(boma, handbg4, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, handbg4, 0.15);
					EffectModule::set_rgb(boma, star1, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star2, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star3, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star4, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star5, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star6, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star7, 1.65, 1.95, 1.85);
					EffectModule::set_alpha(boma, star1, 0.6);
					EffectModule::set_alpha(boma, star2, 0.6);
					EffectModule::set_alpha(boma, star3, 0.6);
					EffectModule::set_alpha(boma, star4, 0.6);
					EffectModule::set_alpha(boma, star5, 0.6);
					EffectModule::set_alpha(boma, star6, 0.6);
					EffectModule::set_alpha(boma, star7, 0.6);
				};
				SPIN_EFF[ENTRY_ID] += 1;
				if SPIN_EFF[ENTRY_ID] > 4 {
					SPIN_EFF[ENTRY_ID] = 0;
				};
			} else {
				SPIN_EFF[ENTRY_ID] = 0;
			};
		};
    }
}