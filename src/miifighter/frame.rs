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
use crate::miifighter::*;
use super::*;

pub fn install() {
    Agent::new("miifighter")
    .on_line(Main, brawler_frame)
    .install();
}

pub(crate) fn check_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
			return true;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
			if ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_stick_y(boma) >= 0.7 {
				return true;
			};
		};
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
			return true;
		};
		return false;
	}
}
unsafe extern "C" fn brawler_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
		let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);	
		let frame = MotionModule::frame(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		let kinetic_type = KineticModule::get_kinetic_type(boma);
		let end_frame = MotionModule::end_frame(boma);
		let last_frame_cancel = (end_frame-frame)-2.0;
		let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true) == 1;
		if fighter_kind == *FIGHTER_KIND_MIIFIGHTER && is_default(boma) {
			
			//HOA replacement
			if [hash40("special_lw1_loop")].contains(&MotionModule::motion_kind(boma)) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				KineticModule::clear_speed_all(boma);
			};
			if [hash40("special_lw1")].contains(&MotionModule::motion_kind(boma)) && frame >= 79.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
			if [hash40("special_lw1")].contains(&MotionModule::motion_kind(boma)) && frame >= 55.0 {
				if ray_check_pos(boma, 0.0, -1.0, false) == 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
			if [hash40("special_air_lw1")].contains(&MotionModule::motion_kind(boma))  {
				if MotionModule::frame(boma) >= 60.0 {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
					};
					CancelModule::enable_cancel(boma);
				} else {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
				};
				if frame >= 61.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					KineticModule::clear_speed_all(boma);
				};
				BAN_DOWNB[ENTRY_ID] = true;
			};
			if situation_kind != *SITUATION_KIND_AIR {
				BAN_DOWNB[ENTRY_ID] = false;
			};
			if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_LW_NO) == 0 {
				if BAN_DOWNB[ENTRY_ID] == true {
					CAN_DOWNB[ENTRY_ID] = 1;
				} else {
					CAN_DOWNB[ENTRY_ID] = 0;
				};
			};
			//Dair bounce
			if [hash40("attack_air_lw")].contains(&MotionModule::motion_kind(boma))  {
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
					MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_lw_bounce"), 0.0, 1.0, false, 0.0, false, false);
				}
			}
			//HOA Special Kick 
			if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING{
				if MotionModule::frame(boma) < 2.0 {
					let speed = smash::phx::Vector3f { x: 0.7, y: 0.0, z: 0.0 };
					KineticModule::add_speed(boma, &speed);
				}
				if MotionModule::frame(boma) > 9.0 &&  WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_LW_NO) == 0 {
					CancelModule::enable_cancel(boma);
				};
			};
			//CT Special Kick Bounce
			if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_LW_NO) == 2 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
			};
			//CT Reflect
			if [hash40("special_lw3"), hash40("special_air_lw3")].contains(&MotionModule::motion_kind(boma)) && SearchModule::is_inflict(boma) {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("throw_f"), 12.0, 1.0, false, 0.0, false, false);
				shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 5.0, 0.0, 6.5, 3.5, 0.0, 6.5, 5.5, 1.4, 1.5, 50, false, 0.5, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				println!("reflection!");
				COUNTER_IS[ENTRY_ID] = true;
			};
			if MotionModule::motion_kind(boma) == hash40("throw_f") && MotionModule::frame(boma) > 20.0 {		
				search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
				macros::COL_NORMAL(fighter);
			};
			if MotionModule::motion_kind(boma) == hash40("special_lw3") && COUNTER_IS[ENTRY_ID] == true {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("throw_f"), 12.0, 1.0, false, 0.0, false, false);
				COUNTER_IS[ENTRY_ID] = false;
				search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
				macros::COL_NORMAL(fighter);
			};
			if status_kind != *FIGHTER_STATUS_KIND_THROW && MotionModule::motion_kind(boma) == hash40("throw_f"){
				if situation_kind == *SITUATION_KIND_AIR {
					StatusModule::set_keep_situation_air(boma, true);
					if is_near_ground {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					}
				}
				if MotionModule::frame(boma) > 20.0 {
					CancelModule::enable_cancel(boma);
				}
			};
			if [hash40("special_lw3"), hash40("special_air_lw3"), hash40("throw_f")].contains(&MotionModule::motion_kind(boma)) == false {
				COUNTER_IS[ENTRY_ID] = false;
			};
			
			//Onslaught
			if [hash40("special_air_s1"), hash40("special_air_s1_end")].contains(&MotionModule::motion_kind(boma)) {
					if MotionModule::frame(boma) > 48.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
					}
					if MotionModule::frame(boma) > 18.0 && is_near_ground{
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					}
			};
			if [hash40("special_s1"), hash40("special_s1_end")].contains(&MotionModule::motion_kind(boma))  && MotionModule::frame(boma) > 53.0 {
				CancelModule::enable_cancel(boma);
			};
			
			//ESK
			if ESK_CHARGE[ENTRY_ID] > 0 && total_hitstun > 0.0 {
				if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_N {
					ESK_CHARGE[ENTRY_ID] = 0;
				};
			};
			if smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
				ESK_CHARGE[ENTRY_ID] = 0;
			};
			if [hash40("special_n3"), hash40("special_air_n3")].contains(&MotionModule::motion_kind(boma)) {
				if motion_duration(boma) == 5 {
					if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
						PostureModule::reverse_lr(boma);
						PostureModule::update_rot_y_lr(boma);
						let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
						KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
				};
				if MotionModule::frame(boma) > 12.0 && MotionModule::frame(boma) < 20.0 {
					if ESK_CHARGE[ENTRY_ID] % 18 == 0 || MotionModule::frame(boma) == 13.0 {
						EffectModule::req_follow(boma, smash::phx::Hash40::new_raw(0x198abfaca9), smash::phx::Hash40::new("toel"), &ESK, &ESK, 1.0, true, 0, 0, 0, 0, 0, true, true);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
							macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
						} else if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) == false{
							EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
							macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
						};
					};
					if check_jump(boma) {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
							EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
						} else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
							EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
							macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
						};
					};
					if motion_duration(boma) % 60 == 0 || MotionModule::frame(boma) == 13.0 && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD){
						macros::PLAY_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
					};
					let cat1 = ControlModule::get_command_flag_cat(boma, 0);
					if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 || ESK_CHARGE[ENTRY_ID] >= 186 {
						EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
						macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							macros::SET_SPEED_EX(fighter, 1.0, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n3"), 40.0, 1.0, false, 0.0, false, false);
						} else {
							macros::SET_SPEED_EX(fighter, 1.4, 0.45, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n3"), 40.0, 1.0, false, 0.0, false, false);
						};
					};
				};
				if MotionModule::frame(boma) > 19.0 && MotionModule::frame(boma) < 39.0 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n3"), 16.0, 1.0, false, 0.0, false, false);
					} else {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n3"), 16.0, 1.0, false, 0.0, false, false);
					};
				};
				if ESK_CHARGE[ENTRY_ID] == 185 {
					ESK_CHARGE[ENTRY_ID] += 1;
					smash::app::FighterUtil::flash_eye_info(boma);
					EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("footl"), &ESK, &ESK, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
						EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
					} else {
						EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
						macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
				};
				if ESK_CHARGE[ENTRY_ID] < 187 {
					ESK_CHARGE[ENTRY_ID] += 1;
				};
			};
		};
    }
}