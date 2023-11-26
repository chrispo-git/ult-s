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

#[fighter_frame( agent = FIGHTER_KIND_SAMUSD )]
fn samusd_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let situation_kind = StatusModule::situation_kind(boma);
		if smash::app::sv_information::is_ready_go() == false {
			HOLD[ENTRY_ID] = 0;
			IS_HOLD[ENTRY_ID] = false;
			COOLDOWN[ENTRY_ID] = 0;
			IS_ALLOWED[ENTRY_ID] = true;
			CAN_SIDEB[ENTRY_ID] = 0;
		};
		if IS_HOLD[ENTRY_ID] == true && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
			IS_HOLD[ENTRY_ID] = false;
		};
		if IS_HOLD[ENTRY_ID] == true && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
			DamageModule::add_damage(boma, 0.075, 0);
			HOLD[ENTRY_ID] += 1;
		};
		if HOLD[ENTRY_ID] >= HOLD_MAX {
			IS_HOLD[ENTRY_ID] = false;
			HOLD[ENTRY_ID] = 0;
		};
		if END[ENTRY_ID] == true {
			COOLDOWN[ENTRY_ID] = HOLD_COOLDOWN;
			ArticleModule::remove_exist(boma, *FIGHTER_SAMUSD_GENERATE_ARTICLE_MISSILE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			END[ENTRY_ID] = false;
		};
		if COOLDOWN[ENTRY_ID] > 0 {
			COOLDOWN[ENTRY_ID] -= 1;
		};
		if ArticleModule::is_exist(boma, *WEAPON_KIND_SAMUSD_MISSILE) == false && IS_ALLOWED[ENTRY_ID] == false && COOLDOWN[ENTRY_ID] == 0 && END[ENTRY_ID] == false {
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
			CAN_SIDEB[ENTRY_ID] = 1;
		} else {
			CAN_SIDEB[ENTRY_ID] = 0;
		};
		if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G {
			if MotionModule::frame(boma) >= 18.0 && MotionModule::frame(boma) <= 20.0 {
				IS_HOLD[ENTRY_ID] = true;
			};
			if MotionModule::frame(boma) > 25.0 {
				IS_ALLOWED[ENTRY_ID] = false;
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(fighter.module_accessor, 1.5);
		};
		/*
		if IS_ALLOWED[ENTRY_ID] == false {
			if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
			};
			if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			};
		};*/
		if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A{
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, true);
		};
		if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G{
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, true);
		};
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
		if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) {
				if frame > 12.0 && frame < 14.0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.0, true);
					macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_critical_l_tail"));
				};
				if StatusModule::is_situation_changed(boma) && situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				};
				if frame > 5.0 && frame < 7.0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, 0.5, 0, 0, 0, 2, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 2, true);
				};
				if frame > 24.0 && frame < 47.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					VisibilityModule::set_whole(boma, false);
					JostleModule::set_status(boma, false);	
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
					macros::SET_SPEED_EX(fighter, 0.27, 0.05, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_NONE);
				};
				if frame > 47.0 || (frame > 30.0 && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL)){
					KineticModule::clear_speed_all(boma);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.0, true);
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					VisibilityModule::set_whole(boma, true);
					JostleModule::set_status(boma, true);	
					MotionModule::change_motion(boma, Hash40::new("special_air_lw_end"), 0.0, 1.0, false, 0.0, false, false);
				};
				if situation_kind == *SITUATION_KIND_AIR {
					StatusModule::set_keep_situation_air(boma, true);
				} else {
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
				};
		};
		if [hash40("special_lw_end"), hash40("special_air_lw_end")].contains(&motion_kind) {
			StatusModule::set_keep_situation_air(boma, true);
			if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_MOTION_AIR {
				KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
			};
			if frame < 2.0 {
				macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_critical_m_tail"));
			};
			if StatusModule::is_situation_changed(boma){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
			};
			if frame > 29.0 {
				macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
				if ray_check_pos(boma, 0.0, -3.0, true) == 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
				};
			};
		};
		if [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) {
			VisibilityModule::set_whole(boma, true);
		};
    }
}
#[weapon_frame( agent = WEAPON_KIND_SAMUSD_MISSILE )]
fn missile_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_SAMUSD {
			if MotionModule::frame(weapon.module_accessor) >= 5.0 && MotionModule::motion_kind(weapon.module_accessor) == hash40("homing") {
				if IS_HOLD[ENTRY_ID] == true {
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
    smashline::install_agent_frames!(
        samusd_frame,
		missile_frame  
    );
}