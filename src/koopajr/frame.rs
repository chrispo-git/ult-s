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
use crate::koopajr::*;
use super::*;

pub fn install() {
    Agent::new("koopajr")
	.on_line(Main, jr)
	.install();
}

unsafe extern "C" fn jr(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
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
		if fighter_kind == *FIGHTER_KIND_KOOPAJR && is_default(boma) {
				if StatusModule::situation_kind(boma) != SITUATION_KIND_AIR {
					FLOAT[ENTRY_ID] = 0;
					START_FLOAT[ENTRY_ID] = false;
					CHECK_FLOAT[ENTRY_ID] = 0;
				};
				//Effect Code
				if FLOAT[ENTRY_ID] % 5 == 0 && FLOAT[ENTRY_ID] > 1{
						macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("clownshaft3"), 0, 0, 0, 0, 0, 0, 0.7, true);
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
					*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH,
					*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT,
					*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING,
					*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_HIT_WALL, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_SPIN_TURN, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END
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
					macros::SET_SPEED_EX(fighter, X[ENTRY_ID], Y[ENTRY_ID]+0.075, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				} else {
					X[ENTRY_ID] = 0.0;
					Y[ENTRY_ID] = 0.0;
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
		};
	};
}