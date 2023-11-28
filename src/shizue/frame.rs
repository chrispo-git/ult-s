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

#[fighter_frame( agent = FIGHTER_KIND_SHIZUE )]
fn shizue_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let fighter_kind = smash::app::utility::get_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let stick_y = ControlModule::get_stick_y(boma);
			if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
				ISA_RESHOOT_TIME[ENTRY_ID] = 0;
				ISA_SHOT_KIND[ENTRY_ID] = 1;
			};
			if ISA_RESHOOT_TIME[ENTRY_ID] > 0 {
				ISA_RESHOOT_TIME[ENTRY_ID] -= 1;
			};
			if status_kind == *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE {
					if motion_duration(boma) == 5 {
						if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
							PostureModule::reverse_lr(boma);
							PostureModule::update_rot_y_lr(boma);
							let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
							KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						};
					};
			};
			if ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_CLIFF_ATTACK].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POMPON,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if [hash40("special_air_hi"), hash40("special_hi"), hash40("special_air_hi_wait1"), hash40("special_air_hi_wait2"), hash40("special_air_hi_flap1"), hash40("special_air_hi_flap2"), hash40("special_air_hi_flap1")].contains(&MotionModule::motion_kind(boma)) {
				if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
					ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
					ArticleModule::remove(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
				};
				if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR){
					ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
					ArticleModule::remove(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
				};
			};
			if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
				if MotionModule::frame(boma) >= 28.0 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
					};
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					let cat2 = ControlModule::get_command_flag_cat(boma, 1);
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
						WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					}
				};
			};
			if ISA_RESHOOT_TIME[ENTRY_ID] < 1{
				if [
					*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH, 
					*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_S4_START, 
					*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, 
					*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END,
					*FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HOOK, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_REEL, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_START, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_THROW, 
					*FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_PICKUP, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CATCH_WAIT, *FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_LANDING,
					*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_CATCH_ATTACK
				].contains(&status_kind) {
					if ControlModule::get_stick_y(boma) <= -0.5 && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						ArticleModule::shoot_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
						ISA_RESHOOT_TIME[ENTRY_ID] = 130;
					};
				};
			};
		}
	};
}		

pub fn install() {
    smashline::install_agent_frames!(
        shizue_frame
    );
}