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
use crate::koopa::*;
use super::*;
pub fn install() {
    smashline::install_agent_frames!(
        bowser_frame,
		fireball_frame,
		kirby_bowser_frame
    );
}

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_bowser_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma);
		if [hash40("koopa_special_n")].contains(&MotionModule::motion_kind(boma)){
			if end_frame-frame < 5.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
			if frame >= 19.0 {
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(boma, 0.775);
		};
		if [hash40("koopa_special_air_n")].contains(&MotionModule::motion_kind(boma)){
			if end_frame-frame < 5.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};
			if frame >= 19.0 {
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(boma, 0.775);
		};
		if [hash40("koopa_special_n_end")].contains(&MotionModule::motion_kind(boma)){
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
		};
		if [hash40("koopa_special_air_n_end")].contains(&MotionModule::motion_kind(boma)){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
		};
		if ArticleModule::is_exist(boma, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH) {
			FIREBALL[ENTRY_ID] += 1;
		} else {
			FIREBALL[ENTRY_ID] = 0;
		};
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_breath_m_fire"), false, true);
	}
}		
#[fighter_frame( agent = FIGHTER_KIND_KOOPA )]
fn bowser_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		if is_default(boma) { 
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
			let frame = MotionModule::frame(boma);
			let end_frame = MotionModule::end_frame(boma);
			if [hash40("special_s_catch"), hash40("special_s_air_catch")].contains(&MotionModule::motion_kind(boma)){
				if MotionModule::frame(boma) < 6.0 {
					MotionModule::set_rate(boma, 0.6);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("attack_air_lw")].contains(&MotionModule::motion_kind(boma)){
				if PostureModule::lr(boma) == -1.0 {
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
				};
			};
			if [hash40("special_n")].contains(&MotionModule::motion_kind(boma)){
				if end_frame-frame < 5.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
				};
				if frame >= 19.0 {
					CancelModule::enable_cancel(boma);
				};
				MotionModule::set_rate(boma, 0.775);
			};
			if [hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)){
				if end_frame-frame < 5.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
				if frame >= 19.0 {
					CancelModule::enable_cancel(boma);
				};
				MotionModule::set_rate(boma, 0.775);
			};
			if [hash40("special_n_end")].contains(&MotionModule::motion_kind(boma)){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
			if [hash40("special_air_n_end")].contains(&MotionModule::motion_kind(boma)){
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};
			if ArticleModule::is_exist(boma, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH) {
				FIREBALL[ENTRY_ID] += 1;
			} else {
				FIREBALL[ENTRY_ID] = 0;
			};
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_breath_m_fire"), false, true);

			//Fsmash shit
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD {
				if frame >= 50.0 && frame < 60.0 {
					KOOPA_EXCELLENT_SMASH[ENTRY_ID] = true;
				}
				else {
					KOOPA_EXCELLENT_SMASH[ENTRY_ID] = false;
				}
			}
			if [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
				if KOOPA_EXCELLENT_SMASH[ENTRY_ID] == true {
					if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
						SPECIAL_ZOOM_GFX[ENTRY_ID] += 1;
						if SPECIAL_ZOOM_GFX[ENTRY_ID] < 2 {
							SlowModule::set_whole(boma, 8, 80);
							macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
							EffectModule::req_follow(boma, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
							macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
							macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
						}
						if SPECIAL_ZOOM_GFX[ENTRY_ID] >= 6 {
							SlowModule::clear_whole(boma);
							CameraModule::reset_all(boma);
							EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
							macros::CAM_ZOOM_OUT(fighter);
						}
					}
				}
			}
			if ![*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
				KOOPA_EXCELLENT_SMASH[ENTRY_ID] = false;
				SPECIAL_ZOOM_GFX[ENTRY_ID] = 0;
			}
		}
	}
}		
#[weapon_frame( agent = WEAPON_KIND_KOOPA_BREATH )]
pub fn fireball_frame(weapon : &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_KIRBY {
			if FIREBALL[ENTRY_ID] % 4 == 0 {
					EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_fireflower_shot"), false, true);
					let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE, &NONE, 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, f1, 1.5, 0.5, 0.5);
			};
		} else {
			if FIREBALL[ENTRY_ID] % 14 == 0 {
					EffectModule::kill_kind(weapon.module_accessor, Hash40::new("koopa_breath_m_fire"), false, true);
					let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE, &NONE, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, f1, 1.5, 0.5, 0.5);
					EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("koopa_breath_m_fire"), smash::phx::Hash40::new("top"), &NONE, &NONE, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
			};
		};
    }
}