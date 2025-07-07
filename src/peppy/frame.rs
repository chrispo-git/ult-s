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
use crate::peppy::*;

pub fn install() {
	unsafe {
		FIGHTER_FALCO_GENERATE_ARTICLE_MISSILE += smashline::clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "falco", "missile", false);
	}
	Agent::new("falco")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
	.on_line(Main, peppy_frame)
	.install();
}
	
unsafe extern "C" fn peppy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_added(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let end_frame = MotionModule::end_frame(boma);
			let stick_y = ControlModule::get_stick_y(boma);
			let fallspeed = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
			let lr = PostureModule::lr(boma);

			if [hash40("attack_air_lw"), hash40("attack_air_lw2")].contains(&motion_kind) {
				if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) {
					let cat2 = ControlModule::get_command_flag_cat(boma, 1);
                    if !((cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_SHIELD)) {
                        KineticModule::clear_speed_all(fighter.module_accessor);
                        let speed = smash::phx::Vector3f { x: 0.0, y: 1.9, z: 0.0 };
                        KineticModule::add_speed(boma, &speed);
                    }
                } else if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && motion_kind == hash40("attack_air_lw") {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_lw2"), -1.0, 1.0, false, 0.0, false, false);
                }
			} else {
				DO_STALL[ENTRY_ID] = false;
			};
			if [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind) {
				if frame > 100.0 && frame < 330.0 &&  ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
					MotionModule::set_frame_sync_anim_cmd(boma, 330.0, true, true, false);
				}
				if frame as i32 == 330 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
					MotionModule::set_frame_sync_anim_cmd(boma, 70.0, true, true, false);
				}
			}
			if [hash40("special_s_start")].contains(&motion_kind) {
				if frame < 10.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
            		KineticModule::add_speed(boma, &smash::phx::Vector3f { x: 0.0, y: -2.0, z: 0.0 });
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				}
			}
			if [hash40("special_air_s_start")].contains(&motion_kind) {
				if (frame as i32)  == 14 {
					macros::PLAY_SE(fighter, Hash40::new("se_item_beamsword_m"));
					macros::PLAY_SE(fighter, Hash40::new("se_item_beamsword_m"));
					macros::PLAY_SE(fighter, Hash40::new("se_item_beamsword_m"));
					macros::PLAY_SE(fighter, Hash40::new("se_item_magicball_warpin"));
				}
				if (frame as i32)  == 41 {
					macros::PLAY_SE(fighter, Hash40::new("vc_falco_heavyget"));
				}
				if frame > 14.0 && frame < 35.0 {
					let next_effect = EffectModule::req_on_joint(boma, smash::phx::Hash40::new("sys_raygun_bullet"), smash::phx::Hash40::new("throw"), &Vector3f{x: 0.0, y: 2.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 60.0, z: 90.0} as *const Vector3f, 0.25, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, true, 0, 0, 0) as u32;
					TETHER_EFFECTS.push(next_effect);
				} else if frame > 12.0 {
					for i in &mut TETHER_EFFECTS {
						if *i != 0 {
							if EffectModule::is_exist_effect(fighter.module_accessor, *i){
								EffectModule::kill(fighter.module_accessor, *i, false,false);
							}
							*i = 0;
							break;
						}
					}
				}
				if frame > 15.0 && frame < 41.0 {
					KineticModule::clear_speed_all(fighter.module_accessor);
					let tether_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
					ModelModule::joint_global_position(boma, Hash40::new("throw"), tether_pos, false);
					if 
						GroundModule::ray_check(boma, &Vector2f{ x: tether_pos.x, y: tether_pos.y}, &Vector2f{ x: 0.0, y: -4.0}, true) == 1 ||
						GroundModule::ray_check(boma, &Vector2f{ x: tether_pos.x, y: tether_pos.y}, &Vector2f{ x: 7.0*PostureModule::lr(boma), y: 0.0}, true) == 1 ||
						AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL)
					{
						println!("tether! x{} y{}", tether_pos.x, tether_pos.y);
						MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, false, 0.0, false, false);
					} else {
					}
					println!("tether? x{} y{}", tether_pos.x, tether_pos.y);
				}
				if end_frame-frame < 3.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				}
			}
			if [hash40("special_air_s")].contains(&motion_kind) {
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR{
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
				};
				SUPER_LAUNCH[ENTRY_ID] = true;
				if ray_check_pos(boma, 0.0, -3.0, true) == 1 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
				GroundModule::clear_pass_floor(boma);
				CancelModule::enable_cancel(boma);
				if frame > 1.0 {
					for i in &mut TETHER_EFFECTS {
						if *i != 0 {
							if EffectModule::is_exist_effect(fighter.module_accessor, *i){
								EffectModule::kill(fighter.module_accessor, *i, false,false);
							}
							*i = 0;
							break;
						}
					}
				}
				if (frame as i32) < 2{
					macros::PLAY_SE(fighter, Hash40::new("se_item_magicball_warpout"));
					macros::PLAY_SE(fighter, Hash40::new("se_common_metal_landing_s"));
        			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_falco_rnd_attack"));
				}
				if end_frame-frame < 5.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				}
				if ray_check_pos(boma, 7.0*lr, 0.0, true) == 1 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
				}
			}
			if ![*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
				for i in &mut TETHER_EFFECTS {
					if *i != 0 {
						if EffectModule::is_exist_effect(fighter.module_accessor, *i){
							EffectModule::kill(fighter.module_accessor, *i, false,false);
						}
						*i = 0;
					}
				}
				TETHER_EFFECTS.clear();
				EffectModule::kill_kind(boma, Hash40::new("sys_raygun_bullet"), false, false);
			}
			if ![*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
				SUPER_LAUNCH[ENTRY_ID] = false;
			} else if SUPER_LAUNCH[ENTRY_ID] {
				WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
			}

			if [hash40("special_lw"), hash40("special_lw_r"), hash40("special_lw_l"), hash40("special_air_lw"), hash40("special_air_lw_r"), hash40("special_air_lw_l")].contains(&motion_kind) {
				if !HAS_DOWNB[ENTRY_ID] && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					HAS_DOWNB[ENTRY_ID] = true;
					DO_STALL[ENTRY_ID] = true;
				};
				if frame > 32.0 {
					DO_STALL[ENTRY_ID] = false; 
					KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
					KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
					CancelModule::enable_cancel(boma);
				} else {
					if DO_STALL[ENTRY_ID] {
						KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
						KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                        macros::SET_SPEED_EX(fighter, 0.0, -0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
				};
				if frame as i32 == 8 {
					if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
						MotionModule::set_rate(boma, 0.0);
					} else {
						MotionModule::set_rate(boma, 1.0);
					}
					let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
					if stick_x < -0.5 {
						PostureModule::reverse_lr(boma);
						PostureModule::update_rot_y_lr(boma);
					}
				}
			} else {
				DO_STALL[ENTRY_ID] = false;
			};
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
				HAS_DOWNB[ENTRY_ID] = false;
				DO_STALL[ENTRY_ID] = false;
			}
			if [*FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
				if StatusModule::is_situation_changed(boma) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					let cat2 = ControlModule::get_command_flag_cat(boma, 1);
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
						WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					}
				};
			};
		}
    }
}