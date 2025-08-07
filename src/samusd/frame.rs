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
use crate::samusd::*;
use std::f32::consts::PI;
use super::*;

pub(crate) unsafe fn bone_const(boma: &mut smash::app::BattleObjectModuleAccessor, new_fighter_kind : i32, new_motion_kind : u64, bone : u64,
	start_frame : f32, end_frame : f32, 
	x_rotate : f32, x_rotate_end : f32, y_rotate : f32, y_rotate_end : f32,  z_rotate : f32, z_rotate_end : f32
) -> () {
	let fighter_kind = smash::app::utility::get_kind(boma);
	let motion_kind = MotionModule::motion_kind(boma);
	let frame = MotionModule::frame(boma);
	let duration = end_frame-start_frame;
	if fighter_kind == new_fighter_kind && motion_kind == new_motion_kind && frame >= start_frame && frame <= end_frame {
		let mut rotation = Vector3f{x: x_rotate + (((x_rotate_end-x_rotate)/duration)*(frame-start_frame)), y: y_rotate + (((y_rotate_end-y_rotate)/duration)*(frame-start_frame)) , z: z_rotate + (((z_rotate_end-z_rotate)/duration)*(frame-start_frame))};
	    ModelModule::set_joint_rotate(boma, Hash40::new_raw(bone), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
	};
}

unsafe extern "C" fn samusd_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let situation_kind = StatusModule::situation_kind(boma);
			//Dark Samus Fair angle down
			bone_const(boma, *FIGHTER_KIND_SAMUSD, hash40("attack_air_f"), hash40("rot"), 0.0, 47.0, 22.5, 22.5, 0.0, 0.0, 0.0, 0.0);
			bone_const(boma, *FIGHTER_KIND_SAMUSD, hash40("attack_air_f"), hash40("rot"), 47.0, 59.0, 22.5, 0.0, 0.0, 0.0, 0.0, 0.0);
			if smash::app::sv_information::is_ready_go() == false {
				HOLD[ENTRY_ID] = 0;
				IS_HOLD[ENTRY_ID] = false;
				COOLDOWN[ENTRY_ID] = 0;
				IS_ALLOWED[ENTRY_ID] = true;
				CAN_DOWNB[ENTRY_ID] = 0;
			};
			if IS_HOLD[ENTRY_ID] == true && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
				IS_HOLD[ENTRY_ID] = false;
			};
			if IS_HOLD[ENTRY_ID] == true && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
				DamageModule::add_damage(boma, 0.075, 0);
				HOLD[ENTRY_ID] += 1;
			};
			if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
				HOLD[ENTRY_ID] = 0;
			}
			if HOLD[ENTRY_ID] >= HOLD_MAX {
				IS_HOLD[ENTRY_ID] = false;
				HOLD[ENTRY_ID] = 0;
			};
			if ArticleModule::is_exist(boma, *FIGHTER_SAMUSD_GENERATE_ARTICLE_MISSILE) {
				COOLDOWN[ENTRY_ID] = HOLD_COOLDOWN;
				IS_ALLOWED[ENTRY_ID] = false;
			}
			if END[ENTRY_ID] == true {
				COOLDOWN[ENTRY_ID] = HOLD_COOLDOWN;
				ArticleModule::remove_exist(boma, *FIGHTER_SAMUSD_GENERATE_ARTICLE_MISSILE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				END[ENTRY_ID] = false;
			};
			if COOLDOWN[ENTRY_ID] > 0 {
				COOLDOWN[ENTRY_ID] -= 1;
			};
			if ArticleModule::is_exist(boma, *FIGHTER_SAMUSD_GENERATE_ARTICLE_MISSILE) == false && IS_ALLOWED[ENTRY_ID] == false && COOLDOWN[ENTRY_ID] == 0 && END[ENTRY_ID] == false {
				END[ENTRY_ID] = true;
			};
			if COOLDOWN[ENTRY_ID] > 0 &&  COOLDOWN[ENTRY_ID] < 5{
					let m1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
					let m2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
					let m3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("footr"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
					let m4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("footl"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, m1, 0.0, 1.0, 3.8);
					EffectModule::set_rgb(boma, m2, 0.0, 1.0, 3.8);
					EffectModule::set_rgb(boma, m3, 0.0, 1.0, 3.8);
					EffectModule::set_rgb(boma, m4, 0.0,1.0, 3.8);
					COOLDOWN[ENTRY_ID] = 0;
					IS_ALLOWED[ENTRY_ID] = true;
			};
			if  IS_ALLOWED[ENTRY_ID] == false {
				CAN_DOWNB[ENTRY_ID] = 1;
			} else {
				CAN_DOWNB[ENTRY_ID] = 0;
			};
			if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW {
				if MotionModule::frame(boma) >= 18.0 && MotionModule::frame(boma) <= 20.0 {
					IS_HOLD[ENTRY_ID] = true;
				};
				if MotionModule::frame(boma) > 25.0 {
					IS_ALLOWED[ENTRY_ID] = false;
					CancelModule::enable_cancel(boma);
				};
				MotionModule::set_rate(fighter.module_accessor, 1.5);
			};
			if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW {
                GroundModule::attach_ground(fighter.module_accessor, true);
                GroundModule::set_attach_ground(fighter.module_accessor, true);
                StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			}
			if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A].contains(&status_kind) {
				if situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, true);
				} else {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, true);
				}
			}
			if IS_HOLD[ENTRY_ID] == false {
				macros::STOP_SE(fighter, Hash40::new("se_samusd_win03_02"));
			} else if HOLD[ENTRY_ID] % 70 == 0{
				macros::PLAY_SE(fighter, Hash40::new("se_samusd_win03_02"));
			};
			if motion_kind == hash40("attack_12") {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
			};
			if motion_kind == hash40("attack_lw4") {
				if frame >= 70.0 {
						MotionModule::change_motion(boma, Hash40::new("wait_4"), 0.0, 1.0, false, 0.0, false, false);
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
				};
			};
			if motion_kind == hash40("wait_4") {
				CancelModule::enable_cancel(boma);
			};
			//Teleport!
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
					WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
					let lr = PostureModule::lr(boma);
					if MotionModule::frame(boma) < 55.0 {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
							StatusModule::set_keep_situation_air(boma, true);
						}
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
						};
					} else {
						let is_near_ground = GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true);
        				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_FALL {
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
						};
						if is_near_ground == 1 {
							StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
							StatusModule::set_keep_situation_air(boma, false);
							StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
						}
					}
					if MotionModule::frame(boma) < 15.0{
						let stick_angle = get_stick_angle(boma);
						if stick_angle != -1.0 {
							UPB_ANGLE[ENTRY_ID] = stick_angle;
							if (lr > 0.0) {
								UPB_ANGLE[ENTRY_ID] = 360.0 - stick_angle;
							}
						} else {
							UPB_ANGLE[ENTRY_ID] = 0.0;
						}
					}else if MotionModule::frame(boma) < 55.0 {
						let stick_angle = UPB_ANGLE[ENTRY_ID];
						let angle_radians = (stick_angle - 90.0) * (PI / 180.0);
						let init_speed = 3.0;
						let deccel = 0.01;
						let speed = (init_speed - (deccel * (frame-1.0)))*MotionModule::rate(boma);
						let x_speed = angle_radians.cos() * speed * lr * -1.0;
						let y_speed = angle_radians.sin() * speed * -1.0;

						macros::SET_SPEED_EX(fighter, x_speed, y_speed, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						println!("Speed : ({},{}), Dir : {}", x_speed, y_speed, UPB_ANGLE[ENTRY_ID]);
					}
			};
		}
    }
}
unsafe extern "C" fn missile_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_SAMUSD {
			if MotionModule::frame(weapon.module_accessor) >= 5.0 && MotionModule::motion_kind(weapon.module_accessor) == hash40("homing") {
				if IS_HOLD[ENTRY_ID] == true && MotionModule::frame(weapon.module_accessor) < 9.0 {
					MotionModule::set_rate(weapon.module_accessor, 0.001);
					println!("hold");
				} else {
					MotionModule::set_rate(weapon.module_accessor, 1.0);
				};
				if MotionModule::frame(weapon.module_accessor) >= 39.0 && (ModelModule::scale(weapon.module_accessor) > 0.001 || PostureModule::scale(weapon.module_accessor) > 0.001){
					END[ENTRY_ID] = true;
					ModelModule::set_scale(weapon.module_accessor, 0.0001);
					PostureModule::set_scale(weapon.module_accessor, 0.0001, false);
					WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
				};
			};
		};
    }
}

pub fn install() {
    Agent::new("samusd")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
		.on_line(Main, samusd_frame)
		.install();

	Agent::new("samusd_missile")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
		.on_line(Main, missile_frame)
		.install();
}