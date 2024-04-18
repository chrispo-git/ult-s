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

use crate::ganon::*;

pub fn install() {
    Agent::new("ganon")
        .on_line(Main, ganon_float)
        .install();
}
		
pub(crate) fn check_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
			return true;
		};
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
			return true;
		};
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
			return true;
		};
		return false;
	}
}
unsafe extern "C" fn ganon_float(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		let speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *FIGHTER_KIND_GANON && is_default(boma){
			if StatusModule::situation_kind(boma) != SITUATION_KIND_AIR || smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
				FLOAT[ENTRY_ID] = 0;
			};
			if [hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)) {
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
					StatusModule::set_keep_situation_air(boma, true);
			};
			if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) {
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
				if MotionModule::frame(boma) >= 24.0 && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, true);
				};
			};
			if FLOAT[ENTRY_ID] == 1{
				CAN_NEUTRALB[ENTRY_ID] = 1;
			} else {
				CAN_NEUTRALB[ENTRY_ID] = 0;
			};
			if FLOAT[ENTRY_ID] == 1{
				if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_MOTION_AIR && [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) == false {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
				};
				if MotionModule::motion_kind(boma) == hash40("special_air_n") {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
			if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) && FLOAT[ENTRY_ID] > 1{
				FLOAT[ENTRY_ID] = 1;
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind){
				FLOAT[ENTRY_ID] = 0;
			};
			if FLOAT[ENTRY_ID] > 1{
				if FLOAT[ENTRY_ID] % 30 == 0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_rekkikyaku"), Hash40::new("kneer"), 12, -1.5, 0, 0, 0, 0, 0.5, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_rekkikyaku"), Hash40::new("kneel"), 12, -1.5, 0, 0, 0, 0, 0.5, true);
				};
				if [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL].contains(&status_kind) && MotionModule::motion_kind(boma) != hash40("special_air_n") {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n"), 1.0, 1.0, false, 0.0, false, false);
				};
				FLOAT[ENTRY_ID] -= 1;
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
				};
				if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
					FLOAT[ENTRY_ID] = 1;
				};
				if check_jump(boma) {
					FLOAT[ENTRY_ID] = 1;
				};
				let mut y_add = 0.0;
				let mut x_add = 0.0;
				if stick_x > 0.2 {
					x_add = ((stick_x-0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
					if speed_x > X_MAX || speed_x < -X_MAX{
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
				let speed = smash::phx::Vector3f { x: x_add, y: y_add, z: 0.0 };
				KineticModule::add_speed(boma, &speed);
				X[ENTRY_ID] += x_add;
				Y[ENTRY_ID] += y_add;
			} else {
				X[ENTRY_ID] = 0.0;
				Y[ENTRY_ID] = 0.0;
			};
			if [hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) == 2.0 && FLOAT[ENTRY_ID] == 0 {
					FLOAT[ENTRY_ID] = FLOAT_MAX;
					macros::PLAY_SE(fighter, Hash40::new("se_ganon_special_l01"));
				};
				if FLOAT[ENTRY_ID] == 1 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				};
				CancelModule::enable_cancel(boma);
			};
		};
		if fighter_kind == *FIGHTER_KIND_KIRBY {
			if StatusModule::situation_kind(boma) != SITUATION_KIND_AIR || smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
				FLOAT[ENTRY_ID] = 0;
			};
			if [hash40("ganon_special_air_n")].contains(&MotionModule::motion_kind(boma)) {
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
					StatusModule::set_keep_situation_air(boma, true);
			};
			if FLOAT[ENTRY_ID] == 1 && WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_GANON {
				CAN_NEUTRALB[ENTRY_ID] = 1;
			} else if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_NONE{
				println!("Empty");
				CAN_NEUTRALB[ENTRY_ID] = 0;
			};
			if FLOAT[ENTRY_ID] == 1{
				if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_MOTION_AIR && [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) == false {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
				};
				if MotionModule::motion_kind(boma) == hash40("ganon_special_air_n") {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
			if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) && FLOAT[ENTRY_ID] > 1{
				FLOAT[ENTRY_ID] = 1;
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind){
				FLOAT[ENTRY_ID] = 1;
			};
			if FLOAT[ENTRY_ID] > 1{
				if FLOAT[ENTRY_ID] % 30 == 0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_rekkikyaku"), Hash40::new("haver"), 0, 3, 0, 0, 0, 0, 0.22, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_rekkikyaku"), Hash40::new("havel"), 0, 3, 0, 0, 0, 0, 0.22, true);
				};
				if [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL].contains(&status_kind) && MotionModule::motion_kind(boma) != hash40("ganon_special_air_n") {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("ganon_special_air_n"), 1.0, 1.0, false, 0.0, false, false);
				};
				FLOAT[ENTRY_ID] -= 1;
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
				};
				if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
					FLOAT[ENTRY_ID] = 1;
				};
				if check_jump(boma) {
					FLOAT[ENTRY_ID] = 1;
				};
				let mut y_add = 0.0;
				let mut x_add = 0.0;
				if stick_x > 0.2 {
					x_add = ((stick_x-0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
					if speed_x > X_MAX || speed_x < -X_MAX{
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
				let speed = smash::phx::Vector3f { x: x_add, y: y_add, z: 0.0 };
				KineticModule::add_speed(boma, &speed);
				X[ENTRY_ID] += x_add;
				Y[ENTRY_ID] += y_add;
			} else {
				X[ENTRY_ID] = 0.0;
				Y[ENTRY_ID] = 0.0;
			};
			if [hash40("ganon_special_air_n")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) == 0.3 && FLOAT[ENTRY_ID] == 0 {
					FLOAT[ENTRY_ID] = FLOAT_MAX;
					macros::PLAY_SE(fighter, Hash40::new("se_ganon_special_l01"));
				};
				if FLOAT[ENTRY_ID] == 1 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				};
				CancelModule::enable_cancel(boma);
				MotionModule::set_rate(boma, 0.05);
			};
		};
	};
}