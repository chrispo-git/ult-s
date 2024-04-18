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
    Agent::new("sonic")
    .on_line(Main, sonic)
    .install();
}

unsafe extern "C" fn sonic(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let lua_state = fighter.lua_state_agent;
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let fighter_kind = smash::app::utility::get_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let situation_kind = StatusModule::situation_kind(boma);
			let mut stick_x = ControlModule::get_stick_x(boma) ;
			stick_x *= PostureModule::lr(boma);
			if fighter_kind == *FIGHTER_KIND_SONIC {
				if motion_kind == hash40("attack_100") {
					if MotionModule::frame(fighter.module_accessor) >= 16.0 {
						if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
							MotionModule::change_motion(boma, Hash40::new("attack_100"), 0.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, Hash40::new("attack_100_end"), 2.0, 1.0, false, 0.0, false, false);
						};
					};
				};
				if motion_kind == hash40("attack_air_f") {
					if MotionModule::frame(fighter.module_accessor) >= 18.0 {
						if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
							MotionModule::change_motion(boma, Hash40::new("attack_air_f_hit"), 0.0, 1.0, false, 0.0, false, false);
						};
					};
				};
				if situation_kind != *SITUATION_KIND_AIR || (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind){
					BAN_SIDEB[ENTRY_ID] = false;
				};
				if BAN_SIDEB[ENTRY_ID] == true {
						CAN_SIDEB[ENTRY_ID] = 1;
				} else {
						CAN_SIDEB[ENTRY_ID] = 0;
				};
				if [hash40("special_s_start"), hash40("special_air_s_start")].contains(&MotionModule::motion_kind(boma)) {
					reimpl_cancel_frame(fighter);
					/*println!("cliff check: {}", GroundModule::cliff_check(boma));
					println!("is near cliff: {}", GroundModule::is_near_cliff(boma, PostureModule::pos_x(boma), PostureModule::pos_y(boma)));
					println!("can entry cliff: {}", );*/
					if GroundModule::can_entry_cliff(boma) == 1 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) < 7 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME) < 1 {
						fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(),false.into());
						macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), false, true);
					};
					/*WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
					WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
					WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);*/
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						/*if MotionModule::frame(boma) >= 24.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
							if MotionModule::frame(boma) >= 43.0 {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
							};
						};*/
						if MotionModule::frame(boma) >= 54.0 && MotionModule::frame(boma) < 75.0{
							MotionModule::set_rate(boma, 2.0);
						} else {
							MotionModule::set_rate(boma, 1.0);
						};
						if MotionModule::frame(boma) >= 85.0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
						};
					} else {
						/*if MotionModule::frame(boma) >= 24.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
							MotionModule::set_rate(boma, 2.0);
						};*/
						if MotionModule::frame(boma) >= 54.0 {
							MotionModule::set_rate(boma, 2.0);
						};
						if MotionModule::frame(boma) >= 60.0 {
							CancelModule::enable_cancel(boma);
						};
						if MotionModule::frame(boma) >= 70.0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
						};
					};
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD){
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
					
					if StatusModule::situation_kind(boma) != *SITUATION_KIND_GROUND && MotionModule::motion_kind(boma) == hash40("special_s_start") {
						if MotionModule::frame(boma) < 34.0 && MotionModule::frame(boma) != 14.0{
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_s_start"), MotionModule::frame(boma)+1.0, 1.0, false, 0.0, false, false);
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
						};
					};
					if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR && MotionModule::motion_kind(boma) == hash40("special_air_s_start") {
						if MotionModule::frame(boma) < 42.0 && MotionModule::frame(boma) != 14.0{
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s_start"), MotionModule::frame(boma)+1.0, 1.0, false, 0.0, false, false);
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
						};
					};
					if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && MotionModule::frame(boma) > 16.0{
							
						if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
							if MotionModule::frame(boma) < 27.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND{ 
								MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s_start"), 27.0, 1.0, false, 0.0, false, false);
							}
						} else {
							macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
						} 
					}
					if MotionModule::frame(boma) == 14.0 && !HAS_MADE_SIDEB_EFF[ENTRY_ID]{
						let lightspeed: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("top"), &LIGHTSPEED, &LIGHTSPEED_ROT, 2.2, true, 0, 0, 0, 0, 0, true, true) as u32;
						EffectModule::set_rgb(boma, lightspeed, 0.2, 0.4, 10.0);
						EffectModule::set_rate(boma, lightspeed, 0.2);
						macros::STOP_SE(fighter, Hash40::new("se_sonic_smash_h01"));
						macros::PLAY_SE(fighter, Hash40::new("vc_sonic_attack05"));
						macros::PLAY_SE(fighter, Hash40::new("se_sonic_swing_m"));
						macros::PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
						macros::PLAY_SE(fighter, Hash40::new("se_sonic_attackair_l01"));
						HAS_MADE_SIDEB_EFF[ENTRY_ID] = true;
					};
					if MotionModule::frame(boma) == 1.0 {
						macros::PLAY_SE(fighter, Hash40::new("se_sonic_smash_h01"));
					};
					notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
					notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07 as u64), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
					notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07u64), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
					notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
					notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
					BAN_SIDEB[ENTRY_ID] = true;
				} else {
					HAS_MADE_SIDEB_EFF[ENTRY_ID] = false;
				}
			};
		}
	};
}