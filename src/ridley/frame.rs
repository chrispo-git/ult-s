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
    Agent::new("ridley")
    .on_line(Main, ridley)
    .install();
}

unsafe extern "C" fn ridley(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let mut stick_x = ControlModule::get_stick_x(boma);
			let stick_y = ControlModule::get_stick_y(boma);
			stick_x *= PostureModule::lr(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			if fighter_kind == *FIGHTER_KIND_RIDLEY {
				if MotionModule::motion_kind(boma) == hash40("attack_air_lw") {
					if (33..35).contains(&(MotionModule::frame(boma) as i32)) {
						let mut is_bounce = false;
						for i in 0..12 {
							for f in 0..16 {
								if ray_check_pos(boma, (f as f32)-8.0, (i as f32)*-1.0 - 10.0, true) == 1 {
									is_bounce = true;
								};
							};
						};
						if  is_bounce || AttackModule::is_infliction_status(boma, *COLLISION_CATEGORY_MASK_ALL) {
							macros::SET_SPEED_EX(fighter, 0.9, 2.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						};
					};
				};
				if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR && ![*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI].contains(&status_kind) {
					HAS_DOUBLE_UPB[ENTRY_ID] = false;
				};
				/*if [hash40("special_s_start")].contains(&MotionModule::motion_kind(boma)) {
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_DASH, true);
					} else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK){
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
					};
				};*/
				if [*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_CHARGE].contains(&status_kind) {
					if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT {
						if StatusModule::is_situation_changed(boma) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
						};
					};
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						let cat2 = ControlModule::get_command_flag_cat(boma, 1);
						if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
							WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
						}
					};
				};
				if [hash40("special_s_start"), hash40("special_air_s_start")].contains(&MotionModule::motion_kind(boma)) {
					if MotionModule::frame(boma) < 24.0 {
						MotionModule::set_rate(boma, 1.5);
					} else {
						MotionModule::set_rate(boma, 1.0);
					};
				};
				if [hash40("special_air_s_start")].contains(&MotionModule::motion_kind(boma)) {
					if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) && MotionModule::frame(boma) > 27.0 && MotionModule::frame(boma) < 35.0 && stick_x < -0.7 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
					};
				};
				if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CATCH {
					macros::SET_SPEED_EX(fighter, 1.5, 0.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					MotionModule::set_rate(boma, 2.0);
					StatusModule::set_keep_situation_air(boma, true);
				};
				if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL {
					macros::SET_SPEED_EX(fighter, 1.25, 0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					StatusModule::set_keep_situation_air(boma, true);
				};
				if [*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI].contains(&status_kind) &&  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK){
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && HAS_DOUBLE_UPB[ENTRY_ID] == false {
						HAS_DOUBLE_UPB[ENTRY_ID] = true;
						SPEED_DOUBLE_UPB[ENTRY_ID] = true;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
					};
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER].contains(&status_kind) && SPEED_DOUBLE_UPB[ENTRY_ID] == true {
					if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && MotionModule::frame(boma) > 3.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER, false);
					};
					if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER {
						MotionModule::set_rate(boma, 8.0);
					};
				} else {
					SPEED_DOUBLE_UPB[ENTRY_ID] = false;
				};
			};
		}
	};
}