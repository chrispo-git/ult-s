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
use super::*;

unsafe extern "C" fn iceball_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = StatusModule::status_kind(weapon.module_accessor);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let is_toad_weapon = (WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
        
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_BOUND);
		if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) {
			WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
		};
    }
}

//bowling ball
unsafe extern "C" fn bob_omb_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = StatusModule::status_kind(weapon.module_accessor);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let is_toad_weapon = (WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
        
        if fighter_kind == *WEAPON_KIND_MURABITO_BOWLING_BALL && is_toad_weapon {
			if status_kind == *WEAPON_MURABITO_BOWLING_BALL_STATUS_KIND_FALL {
				if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) {
				};
			};
		}
    }
}
unsafe extern "C" fn toad(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let end_frame = MotionModule::end_frame(boma);
		//Kirby neutralb logic
		/*if fighter_kind == *FIGHTER_KIND_KIRBY {
			if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_MURABITO {
				if status_kind == *FIGHTER_KIRBY_STATUS_KIND_MURABITO_SPECIAL_N_SEARCH {
					if frame > 38.0 {
						if situation_kind == *SITUATION_KIND_GROUND {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
						};
					};
				};
				if ItemModule::is_have_item(boma, 0) {
					CAN_NEUTRALB[ENTRY_ID] = 1;
				} else {
					CAN_NEUTRALB[ENTRY_ID] = 0;
				};
			} else {
				CAN_NEUTRALB[ENTRY_ID] = 0;
			};
		};*/
		if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) && fighter_kind == *FIGHTER_KIND_MURABITO {
			if ModelModule::scale(boma) == WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0) {
				PostureModule::set_scale(fighter.module_accessor, 0.85, false);
                GrabModule::set_size_mul(boma, 0.85);
            }
			WorkModule::set_int(boma, 1, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_TIME_LIMIT);
			if status_kind == *FIGHTER_STATUS_KIND_DEAD {
				if MotionModule::motion_kind(boma) == hash40("fall_damage") && !HAS_DEADED[ENTRY_ID] {
					macros::PLAY_SE(fighter, Hash40::new("se_murabito_catch_net"));
					HAS_DEADED[ENTRY_ID] = true;
				};
			} else {
				HAS_DEADED[ENTRY_ID] = false;
			};
			if ![*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if ![*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_FIREWORK,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
					if frame < 32.0 {
						MotionModule::set_frame(boma, 32.0, false);
					};
				};
			};
			ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_HELMET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			if ![*FIGHTER_STATUS_KIND_APPEAL].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH].contains(&status_kind) {
				if GroundModule::can_entry_cliff(boma) == 1 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) < 7 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME) < 1 && ControlModule::get_stick_y(boma) > -0.5{
					fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(),false.into());
				};
				if MotionModule::frame(boma) > 10.0 && MotionModule::end_frame(boma) - MotionModule::frame(boma) < 3.0  && MotionModule::motion_kind(boma) == hash40("special_air_hi_detach") {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_hi_loop"), 0.0, 1.0, false, 0.0, false, false);
				}
				if MotionModule::motion_kind(boma) == hash40("special_air_hi_loop") {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
					};
					let gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
                    let speed = smash::phx::Vector3f { x: 0.0, y: gravity*0.9, z: 0.0 };
                    KineticModule::add_speed(boma, &speed);

					if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_hi_screw"), 0.0, 1.0, false, 0.0, false, false);
					}
				}
				if MotionModule::motion_kind(boma) == hash40("special_air_hi_screw") {
                	KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    macros::SET_SPEED_EX(fighter, 0.0, -4.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				}
				
				if [hash40("special_air_hi_screw"), hash40("special_air_hi_loop")].contains(&MotionModule::motion_kind(boma)){
					let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true) == 1;
					if is_near_ground {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
					}
				}
			}
			if ![*FIGHTER_STATUS_KIND_THROW].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_WEEDS,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT].contains(&status_kind) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP, true);
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT_FAIL].contains(&status_kind) && situation_kind == *SITUATION_KIND_GROUND {
				if frame < 2.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP, true);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				};
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP].contains(&status_kind) {
				if frame >= 66.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
				if frame > 30.0 && situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				}
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && frame < 50.0{
					macros::SET_SPEED_EX(fighter, SPEED_X[ENTRY_ID]*PostureModule::lr(boma), 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_lw_water"), 51.0, 1.0, false, 0.0, false, false);
				};
				if frame > 5.0 && frame < 7.0{
					macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackdash01"));
				}
			};
			if [hash40("special_air_lw_plant_failure")].contains(&MotionModule::motion_kind(boma)) {
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) && MotionModule::frame(boma) < 37.0{
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
					MotionModule::set_frame_sync_anim_cmd(boma, 38.0, true, true, false);
                    let speed = smash::phx::Vector3f { x: 0.0, y: 0.6, z: 0.0 };
                    KineticModule::add_speed(boma, &speed);
					if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
						MotionModule::set_rate(boma, 0.5);
					};
				};
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_LANDING, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT].contains(&status_kind) {
				if situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				}; 
			};
			if situation_kind != *SITUATION_KIND_AIR {
				HAS_DOWNB[ENTRY_ID] = false;
			};
			if [hash40("special_air_hi"), hash40("special_hi"), hash40("special_air_hi_wait1"), hash40("special_air_hi_wait2"), hash40("special_air_hi_flap1"), hash40("special_air_hi_flap2"), hash40("special_air_hi_flap1")].contains(&MotionModule::motion_kind(boma)) {
				fighter.clear_lua_stack();
				lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
				smash::app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, true);
			};
			if [hash40("throw_f")].contains(&MotionModule::motion_kind(boma)) {
				if status_kind != *FIGHTER_STATUS_KIND_THROW_KIRBY {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_THROW_KIRBY, true);
				};
				if frame > 60.0 {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
					};
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					};
				};
			};
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
				if frame > 40.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
				};
			};
			if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
				if frame > 38.0 {
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
					};
				};
			};
			if [hash40("appeal_lw_l"), hash40("appeal_lw_r")].contains(&motion_kind) {
				if MotionModule::frame(boma) as i32 == 70 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
					MotionModule::set_frame_sync_anim_cmd(boma, 25.0, true, true, false);
				}
				if MotionModule::frame(boma) > 25.0 && MotionModule::frame(boma) < 70.0 {
					let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
					let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(boma);
					if stick_x < -0.2 {
						if speed_x > -0.9 {
							let speed = smash::phx::Vector3f { x:-0.05, y: 0.0, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
						}
					} else if stick_x > 0.2 {
						if speed_x < 0.9 {
							let speed = smash::phx::Vector3f { x:0.05, y: 0.0, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
						}
					} else {
						let speed = smash::phx::Vector3f { x:0.1*speed_x, y: 0.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					}
				}
			}
			if ItemModule::is_have_item(boma, 0) {
				CAN_NEUTRALB[ENTRY_ID] = 1;
			} else {
				CAN_NEUTRALB[ENTRY_ID] = 0;
			};
			//ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_CLAYROCKET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_BALLOON,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			WorkModule::off_flag(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_CATCHING);
			if [hash40("special_s_jump"), hash40("special_s_loop"), hash40("special_air_s_loop")].contains(&motion_kind) {
				let lr = PostureModule::lr(boma);
				let dist = 4.0*lr;
				if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) 
				&& ray_check_pos(boma, dist, 0.0, false) == 1 {
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
					KineticModule::mul_speed(fighter.module_accessor, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
				SIDEB_LENGTH[ENTRY_ID] += 1;
			} else {
				SIDEB_LENGTH[ENTRY_ID] = 0;
			}
			if [hash40("special_s_end"), hash40("special_air_s_end")].contains(&motion_kind) {
				SIDEB_END[ENTRY_ID] = true;
			}
			if [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
				CAN_SIDEB[ENTRY_ID] = 1;
				if [hash40("special_s"), hash40("special_air_s")].contains(&motion_kind) {
					if SIDEB_END[ENTRY_ID] {
						if situation_kind == *SITUATION_KIND_GROUND {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DOWN, true);
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DAMAGE_FALL, true);
						}
					}
					if MotionModule::end_frame(boma) - frame < 3.0 || SIDEB_RESET[ENTRY_ID] {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
						if !SIDEB_RESET[ENTRY_ID] {
                    		macros::SET_SPEED_EX(fighter, 1.5, -2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
							SIDEB_DIR[ENTRY_ID] = PostureModule::lr(boma);
						} else {
                    		macros::SET_SPEED_EX(fighter, 1.5, PREV_SPEED_Y[ENTRY_ID], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
							if PostureModule::lr(boma) != SIDEB_DIR[ENTRY_ID] {
								PostureModule::reverse_lr(boma);
								PostureModule::update_rot_y_lr(boma);
								let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
								KineticModule::mul_speed(fighter.module_accessor, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
							}
						}
						SIDEB_RESET[ENTRY_ID] = false;
						if situation_kind == *SITUATION_KIND_GROUND {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s_loop"), 0.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_s_loop"), 0.0, 1.0, false, 0.0, false, false);
						}
					}
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
					};
				} else if [hash40("special_s_loop"), hash40("special_air_s_loop")].contains(&motion_kind) {
					if SIDEB_LENGTH[ENTRY_ID] >= SIDEB_LENGTH_MAX {
						if situation_kind == *SITUATION_KIND_GROUND {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false);
						}
					}
					if check_jump(boma) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
						
						if situation_kind == *SITUATION_KIND_GROUND {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_JUMP, true);
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
							let speed = smash::phx::Vector3f { x:1.5, y: -0.5, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
						}
					}
					if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					}
					if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
						SIDEB_LENGTH[ENTRY_ID] = SIDEB_LENGTH_MAX+1;
					}
				} else if motion_kind == hash40("special_s_end") && MotionModule::end_frame(boma) - frame < 3.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DOWN, true);
				}
			} else {
				SIDEB_END[ENTRY_ID] = false;
			}
			if [hash40("special_s_jump"), hash40("special_air_s_loop"), hash40("special_s_loop")].contains(&motion_kind) {
				SIDEB_RESET[ENTRY_ID] = true;
			} else {
				SIDEB_RESET[ENTRY_ID] = false;
			}
			if situation_kind != *SITUATION_KIND_AIR || (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind){ 
				CAN_SIDEB[ENTRY_ID] = 0;
			}
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_JUMP].contains(&status_kind) {
				KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_JUMP_AERIAL {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL);
                    let speed = smash::phx::Vector3f { x:1.5, y: -0.5, z: 0.0 };
                    KineticModule::add_speed(boma, &speed);
				};
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
					SIDEB_LENGTH[ENTRY_ID] = SIDEB_LENGTH_MAX+1;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
				} else {
					if MotionModule::end_frame(boma) - frame < 3.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
					}
					if frame > 4.0 && check_jump(boma) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
					}
					if frame > 4.0 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					}
				}
			}
		};
	};
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

unsafe extern "C" fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_MURABITO {
            return;
        }
		/*if !(WorkModule::get_int(&mut *fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(&mut *fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) {
			return;
		}*/
        fighter.global_table[0x45].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
    }
}

pub fn install() {
    Agent::new("murabito")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
		.on_start(agent_reset)
        .on_line(Main, toad)
        .install();
	/*Agent::new("murabito_flowerpot")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
		.on_line(Main, iceball_frame)
		.install();*/
}