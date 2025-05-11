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
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
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
				let mut stick_y = ControlModule::get_stick_y(boma);
				let frame = MotionModule::frame(boma);
				stick_x *= PostureModule::lr(boma);
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
				if [
					*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW,
					*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI
				].contains(&status_kind) {
				
					if [*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW].contains(&status_kind) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, true);
					}
					if [*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER].contains(&status_kind) {
						let stick_angle = get_stick_angle(boma);
						if stick_angle != -1.0 {
							UPB_ANGLE[ENTRY_ID] = stick_angle;
						} else {
							UPB_ANGLE[ENTRY_ID] = 0.0;
						}
					}
					if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI {
						let stick_angle = UPB_ANGLE[ENTRY_ID];
						let init_speed = 5.0;
						let deccel = 0.139;
						let speed = init_speed - (deccel * (frame-1.0));
						let x_speed = stick_angle.to_radians().cos()*speed;
						let y_speed = stick_angle.to_radians().sin()*speed;
						macros::SET_SPEED_EX(fighter, x_speed, y_speed, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						let mut rotation = Vector3f{x: stick_angle, y: 0.0 , z: 0.0};
						ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
					}
				} else {
					UPB_ANGLE[ENTRY_ID] = 0.0;
				}
		}
	};
}