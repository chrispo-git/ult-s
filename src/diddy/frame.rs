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
	Agent::new("diddy")
	.on_line(Main, diddy_frame)
	.install();

	Agent::new("diddy_gun")
	.on_line(Main, gun_frame)
	.install();
}

static mut DIDDY_PEANUT_CANCEL : [i32; 8] = [0; 8];

unsafe extern "C" fn diddy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
			let frame = MotionModule::frame(boma);
			let end_frame = MotionModule::end_frame(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			if [hash40("special_air_n_start"), hash40("special_air_n_shoot")].contains(&MotionModule::motion_kind(boma)) {
				DIDDY_PEANUT_CANCEL[ENTRY_ID] = 1;
			};
			if [hash40("special_n_blow"), hash40("special_air_n_blow"), hash40("special_air_n_shoot"), hash40("special_n_shoot"), hash40("special_air_n_start"), hash40("special_n_start")].contains(&MotionModule::motion_kind(boma)) == false{
				DIDDY_PEANUT_CANCEL[ENTRY_ID] = 0;
			};
			if [hash40("special_n_shoot"), hash40("special_n_blow"), hash40("special_n_start")].contains(&MotionModule::motion_kind(boma)) && DIDDY_PEANUT_CANCEL[ENTRY_ID] == 1 {
				DIDDY_PEANUT_CANCEL[ENTRY_ID] = 0;
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
			};
			if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind) {
				if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
					if frame >= 76.0 && frame < 80.0 {
						//MotionModule::set_frame_sync_anim_cmd(boma, 10.0, true, true, false);
						MotionModule::set_frame(boma, 10.0, false);
					}
				}
				if frame as i32 == 82 {
					macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("havel"), 0, 3, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
					macros::STOP_SE(fighter, Hash40::new("vc_diddy_001"));
				}
				if frame > 82.0 {
					ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				}
				if frame < 70.0 {
					MotionModule::set_rate(boma, 0.75);
				}
				if frame >= (50.0*MotionModule::rate(boma)) {
					CancelModule::enable_cancel(boma);
				}
			} else {
				macros::STOP_SE(fighter, Hash40::new("vc_diddy_001"));
			}
			if [hash40("appeal_hi_r"), hash40("appeal_hi_l"), hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind) {
				ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			}
		}
	}
}	

unsafe extern "C" fn gun_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		if is_default(&mut *boma) {
			let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let owner_status_kind = StatusModule::status_kind(&mut *boma);
			let own_motion_kind = MotionModule::motion_kind(&mut *boma);
			if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_DIDDY {
				if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&own_motion_kind) {
					ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("brktutum"),false);
					ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("tutu1m"),false);
					ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("tutu2m"),false);
					ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("tutu3m"),true);
				} else {
					ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("tutu3m"),false);
					if [hash40("special_air_n_start"), hash40("special_n_start")].contains(&own_motion_kind) {
						ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("tutu1m"),true);
					};
				}
			};
		}	
    }
}