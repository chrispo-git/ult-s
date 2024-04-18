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
    Agent::new("reflet")
    .on_line(Main, robin)
    .install();
}

unsafe extern "C" fn robin(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let fighter_kind = smash::app::utility::get_kind(boma);
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let situation_kind = StatusModule::situation_kind(boma);
			let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
			let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
			let stick_y = ControlModule::get_stick_y(boma);
			let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			let speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			let frame = MotionModule::frame(boma);
			let end_frame = MotionModule::end_frame(boma);
			let lua_state = fighter.lua_state_agent;
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let motion_kind = MotionModule::motion_kind(boma);

			WorkModule::set_int(boma, 10, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
			if !IS_GRIMA[ENTRY_ID] {
				SPECIAL_HI_CURR[ENTRY_ID] = WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
				SPECIAL_N_CURR[ENTRY_ID] = WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
				SPECIAL_S_CURR[ENTRY_ID] = WorkModule::get_float(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
			} else {
				if SPECIAL_HI_CURR[ENTRY_ID] > 2 {
					WorkModule::set_int(boma, SPECIAL_HI_CURR[ENTRY_ID], *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
				} else {
					WorkModule::set_int(boma, 2, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
				};
				WorkModule::set_int(boma, SPECIAL_N_CURR[ENTRY_ID], *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
				WorkModule::set_float(boma, SPECIAL_S_CURR[ENTRY_ID], *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
			};
			if [hash40("special_hi"), hash40("special_air_hi"), hash40("special_hi_fail"), hash40("special_air_hi_fail")].contains(&motion_kind) {
				if IS_GRIMA[ENTRY_ID] == false {
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) && MotionModule::frame(boma) >= 8.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
					};
				} else{
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_hi3"), 0.0, 1.0, false, 0.0, false, false);
				};
			};
			if IS_GRIMA[ENTRY_ID]{
				WorkModule::set_int(boma, 0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
				if status_kind == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT, true);
				};
			};
			/*if [hash40("special_n_start")].contains(&motion_kind) {
				if IS_GRIMA[ENTRY_ID]{
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n"), -1.0, 1.0, false, 0.0, false, false);
				};
			};
			if [hash40("special_air_n_start")].contains(&motion_kind) {
				if IS_GRIMA[ENTRY_ID]  {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n"), -1.0, 1.0, false, 0.0, false, false);
				};
			};*/
			if [hash40("special_hi3")].contains(&motion_kind) {
				if frame as i32 % 5 == 0 {
					let a1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footr"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footl"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a5: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("hip"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, a1, 3.3, 0.4, 1.0);
					EffectModule::set_rgb(boma, a2, 3.3, 0.4, 1.0);
					EffectModule::set_rgb(boma, a3, 3.3, 0.4, 1.0);
					EffectModule::set_rgb(boma, a4, 3.3, 0.4, 1.0);
					EffectModule::set_rgb(boma, a5, 3.3, 0.4, 1.0);
				};
			};
			if [hash40("special_lw_start"), hash40("special_air_lw_start"), hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) {
				if MotionModule::frame(boma) == 8.0 {
					if IS_GRIMA[ENTRY_ID] == true {
						IS_GRIMA[ENTRY_ID] = false;
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.725, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.15, 1.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.15, 1.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.15, 1.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
					} else {
						IS_GRIMA[ENTRY_ID] = true;
						macros::FT_ADD_DAMAGE(fighter, DMG_ADD*5.0);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.725, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 0.25, 0.0, 3.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 0.25, 0.0, 3.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 0.25, 0.0, 3.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
					};
					macros::PLAY_FLY_VOICE(fighter, Hash40::new("seq_reflet_rnd_futtobi01"),  Hash40::new("seq_reflet_rnd_futtobi02"));
					macros::PLAY_SE(fighter, Hash40::new("se_reflet_special_l01"));
					if StatusModule::is_situation_changed(boma) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					};
				};
				if end_frame-frame < 5.0 && situation_kind != *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
				};
				if  end_frame-frame < 3.0 && situation_kind == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
			};
			if status_kind == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
					if situation_kind == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
					};
			};
			if StatusModule::situation_kind(boma) != SITUATION_KIND_AIR {
				FLOAT[ENTRY_ID] = 0;
				START_FLOAT[ENTRY_ID] = false;
				CHECK_FLOAT[ENTRY_ID] = 0;
			};
			if smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
				IS_GRIMA[ENTRY_ID] = false;
				DMG_COUNTER[ENTRY_ID] = DMG_COUNTER_MAX;
			};
			if IS_GRIMA[ENTRY_ID] == true {
				if DMG_COUNTER[ENTRY_ID] > 0 {
					DMG_COUNTER[ENTRY_ID] -= 1;
				} else {
					DMG_COUNTER[ENTRY_ID] = DMG_COUNTER_MAX;
					if DamageModule::damage(boma, 0) < 120.0 {
						macros::FT_ADD_DAMAGE(fighter, DMG_ADD);
					};
				};
				//Dash Speed
				if [*FIGHTER_STATUS_KIND_DASH].contains(&status_kind) {
					if MotionModule::frame(boma) == 3.0 {
							let speed = smash::phx::Vector3f { x: 0.2, y: 0.0, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
					};
				};
				//Effect Code
				if DMG_COUNTER[ENTRY_ID] % 40 == 0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.25, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
					let a1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("waist"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, a1, 3.3, 0.4, 1.0);
					if FLOAT[ENTRY_ID] > 1 {
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.2, true);
					};
				};
				//Float Code
				if FLOAT[ENTRY_ID] == 1{
					if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_MOTION_AIR && [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) == false {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
					};
				};
				if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW)  && ControlModule::get_stick_y(boma) < -0.5 {
					CAN_DOUBLE_JUMP[ENTRY_ID] = 1;
				} else {
					CAN_DOUBLE_JUMP[ENTRY_ID] = 0;
				}
				if situation_kind == *SITUATION_KIND_AIR && (!(*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) && status_kind != *FIGHTER_STATUS_KIND_FALL_SPECIAL){
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
						CHECK_FLOAT[ENTRY_ID] += 1;
					} else {
						CHECK_FLOAT[ENTRY_ID] = 0;
					};
					if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)  && stick_y < -0.5 {
						CHECK_FLOAT[ENTRY_ID] = CHECK_FLOAT_MAX;
					};
					if (CHECK_FLOAT[ENTRY_ID] >= CHECK_FLOAT_MAX || JUMPSQUAT_FLOAT[ENTRY_ID]) && FLOAT[ENTRY_ID] == 0 {
						START_FLOAT[ENTRY_ID] = true;
					};
				};
				if status_kind == *FIGHTER_STATUS_KIND_JUMP && JUMPSQUAT_FLOAT[ENTRY_ID] {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON){
					JUMPSQUAT_FLOAT[ENTRY_ID] = false;
				}
				if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
					if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && ControlModule::get_stick_y(boma) < -0.5 {
						JUMPSQUAT_FLOAT[ENTRY_ID] = true;
						WorkModule::set_flag(boma, false, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
					} else {
						JUMPSQUAT_FLOAT[ENTRY_ID] = false;
						CHECK_FLOAT[ENTRY_ID] = 0;
					};
				} else {
					JUMPSQUAT_FLOAT[ENTRY_ID] = false;
				};
				if [
					*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_SPECIAL_N, 
					*FIGHTER_STATUS_KIND_SPECIAL_S,*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW,
					*FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL,
					*FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_JUMP_CANCEL
				].contains(&status_kind) && FLOAT[ENTRY_ID] > 1{
					FLOAT[ENTRY_ID] = 1;
				};
				if FLOAT[ENTRY_ID] > 1 {
					FLOAT[ENTRY_ID] -= 1;
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
					if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) {
						FLOAT[ENTRY_ID] = 1;
					};
					if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						FLOAT[ENTRY_ID] = 1;
					};
					let mut y_add = 0.0;
					let mut x_add = 0.0;
					if stick_x > 0.2 {
						x_add = ((stick_x-0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
						if speed_x > X_MAX || speed_x < -X_MAX{{}
							x_add = 0.0;
						};
					};
					if stick_x < -0.2 {
						x_add = ((stick_x+0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
						if speed_x > X_MAX || speed_x < -X_MAX{
							x_add = 0.0;
						};
					};
					if stick_y > 0.2 {
						y_add = ((stick_y-0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
						if speed_y > Y_MAX || speed_y < -Y_MAX{
							y_add = 0.0;
						};
					};
					if stick_y < -0.2 {
						y_add = ((stick_y+0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
						if speed_y > Y_MAX || speed_y < -Y_MAX{
							y_add = 0.0;
						};
					};
					if stick_x > -0.2 && stick_x < 0.2 && stick_y > -0.2 && stick_y < 0.2 {
						if speed_y > 0.0 {
							y_add = -Y_ACCEL_MUL - Y_ACCEL_ADD;
						} else if speed_y < 0.0{
							y_add = Y_ACCEL_MUL + Y_ACCEL_ADD;
						};
						let mut x_add = 0.0;
						if speed_x > 0.0 {
							x_add = -X_ACCEL_MUL - X_ACCEL_ADD;
						} else if speed_x < 0.0{
							x_add = X_ACCEL_MUL + X_ACCEL_ADD;
						};
					};
					x_add = (stick_x)*X_ACCEL_MUL;
					y_add = (stick_y)*X_ACCEL_MUL;
					if x_add > 0.0 && X[ENTRY_ID] > X_MAX {
						x_add = 0.0;
					};
					if x_add < 0.0 && X[ENTRY_ID] < X_MAX*-1.0 {
						x_add = 0.0;
					};
					if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
						y_add = 0.0;
					};
					if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*-1.0 {
						y_add = 0.0;
					};
					println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
					println!("x_add{}, y_add{}", x_add, y_add);
					X[ENTRY_ID] += x_add;
					Y[ENTRY_ID] += y_add;
					macros::SET_SPEED_EX(fighter, X[ENTRY_ID], Y[ENTRY_ID], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				} else {
					X[ENTRY_ID] = 0.0;
					Y[ENTRY_ID] = 0.0;
				};
				if status_kind == *FIGHTER_STATUS_KIND_THROW {
					if FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 > 6.0 {
						if MotionModule::frame(boma) >= (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 - 6.0) {
							CancelModule::enable_cancel(boma);
						};
					};
				};
				if START_FLOAT[ENTRY_ID] == true {
					FLOAT[ENTRY_ID] = FLOAT_MAX;
					START_FLOAT[ENTRY_ID] = false;
					ControlModule::clear_command(boma, false);
					WorkModule::set_flag(boma, false, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					/*if status_kind == *FIGHTER_STATUS_KIND_JUMP {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
					if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};*/
				};
				/*if [*FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_JUMP_CANCEL, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_FAIL, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
					MotionModule::set_rate(boma, 0.75);
				};*/
			};
		};
	}
}