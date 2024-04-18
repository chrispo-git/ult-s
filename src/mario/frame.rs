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
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();
}

unsafe extern "C" fn mario_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if fighter_kind == *FIGHTER_KIND_MARIO &&  is_default(boma) {
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR || (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) {
				CAN_SIDEB[ENTRY_ID] = 0;
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S  && MotionModule::frame(boma) > 9.0 && MotionModule::frame(boma) < 22.0 && StopModule::is_stop(boma) == false {
				CAN_SIDEB[ENTRY_ID] = 1;
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