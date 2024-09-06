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

pub fn install() {
    Agent::new("wario")
    .on_line(Main, wario_frame)
    .install();

	Agent::new("wario_wariobike")
    .on_line(Main, bike_frame)
    .install();
}

unsafe extern "C" fn wario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //println!("It'sa me, Mario, wahoooooooo!");
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let end_frame = MotionModule::end_frame(boma);
			let situation_kind = StatusModule::situation_kind(boma);
			let stick_y = ControlModule::get_stick_y(boma);

			//Why are wario's proportions so wrong in ultimate?
			let armscale = smash::phx::Vector3f { x: 1.2, y: 1.2, z: 1.2 };
			ModelModule::set_joint_scale(boma, Hash40::new("shoulderl"), &armscale);
			ModelModule::set_joint_scale(boma, Hash40::new("shoulderr"), &armscale);
			let legscale = smash::phx::Vector3f { x: 0.95, y: 0.95, z:0.95 };
			ModelModule::set_joint_scale(boma, Hash40::new("footl"), &legscale);
			ModelModule::set_joint_scale(boma, Hash40::new("footr"), &legscale);

			if ![*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) && smash::app::sv_information::is_ready_go() {
				ArticleModule::remove_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			
			
			if [*FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START].contains(&status_kind) {
				if StatusModule::is_situation_changed(boma) && situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					let cat2 = ControlModule::get_command_flag_cat(boma, 1);
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
						WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					}
				};
				if end_frame-frame < 2.0 {
					if situation_kind == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
					}
				}
			};
			if [*FIGHTER_STATUS_KIND_CATCH_ATTACK].contains(&status_kind) && (frame as i32) == 5 {
				COIN_COUNT[ENTRY_ID] += 1;
			};
			if COIN_COUNT[ENTRY_ID] > 30 {
				COIN_COUNT[ENTRY_ID] = 30;
			}
			if smash::app::smashball::is_training_mode() {
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
					COIN_COUNT[ENTRY_ID] = 30;
				}
			}
			let tens = (COIN_COUNT[ENTRY_ID] / 10) as i32;
			if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
				if tens == 3 {
					if (frame as i32) == 9 {
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
						let y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("speed_y_fly"));
						macros::SET_SPEED_EX(fighter, 0.0, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					}
				} else if tens == 2 {
					if (frame as i32) == 5 {
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
						let y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("speed_y_large_max"));
						macros::SET_SPEED_EX(fighter, 0.0, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					}
				} else if tens == 1 {
					if (frame as i32) == 10 {
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
						let y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("speed_y_large_min"));
						macros::SET_SPEED_EX(fighter, 0.0, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					}
				} else {
					if (frame as i32) == 10 {
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
						macros::SET_SPEED_EX(fighter, 0.0, 1.65, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					}
				}
			}

			if [*FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_WALK_BRAKE, *FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) {
				SHOW_COUNT[ENTRY_ID] = true;
			} else {
				SHOW_COUNT[ENTRY_ID] = false;
			}
			if is_reset() {
				COIN_COUNT[ENTRY_ID] = 0;
				SHOW_COUNT[ENTRY_ID] = false;
                ALPHA_COUNTER[ENTRY_ID] = 0.0;
				ArticleModule::remove_exist(boma, FIGHTER_WARIO_GENERATE_ARTICLE_COUNTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			} else {
				if !ArticleModule::is_exist(fighter.module_accessor, FIGHTER_WARIO_GENERATE_ARTICLE_COUNTER) {
					ArticleModule::generate_article(fighter.module_accessor, FIGHTER_WARIO_GENERATE_ARTICLE_COUNTER, false, -1);
				}
			}
		}
    }
}
unsafe extern "C" fn bike_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = StatusModule::status_kind(weapon.module_accessor);
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_WARIO {
			if !smash::app::sv_information::is_ready_go() {
				ModelModule::set_scale(weapon.module_accessor, 1.0);
			} else{
				ModelModule::set_scale(weapon.module_accessor, 0.0000001);
			};
		};
    }
}