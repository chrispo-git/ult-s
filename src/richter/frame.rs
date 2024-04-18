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
    Agent::new("richter")
    .on_line(Main, richter_frame)
    .install();

	Agent::new("kirby")
    .on_line(Main, richter_kirby_frame)
    .install();
}

unsafe extern "C" fn richter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let end_frame = MotionModule::end_frame(boma);
			let situation_kind = StatusModule::situation_kind(boma);
			if [hash40("attack_air_hi")].contains(&motion_kind) && frame >= 33.0 {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
			};
			if ![hash40("haved"), hash40("fly")].contains(&ArticleModule::motion_kind(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))) {
				ArticleModule::remove_exist(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S  {
				if situation_kind == *SITUATION_KIND_AIR && end_frame - frame < 3.0 {
					if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
					}
				}
				if frame < 40.0 {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR{
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
				} else {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_FALL{
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
					};
				}
			}
			if [hash40("special_n"), hash40("special_n_blank"), hash40("special_air_n"), hash40("special_air_n_blank")].contains(&motion_kind) {
				if [hash40("special_air_n"), hash40("special_air_n_blank")].contains(&motion_kind) {
					WAS_AIR[ENTRY_ID] = true;
				} else if WAS_AIR[ENTRY_ID] {
					WorkModule::set_float(boma, 12.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				}
			} else {
				WAS_AIR[ENTRY_ID] = false;
			}
		}
    }
}

unsafe extern "C" fn richter_kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		if ![hash40("haved"), hash40("fly")].contains(&ArticleModule::motion_kind(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))) {
			ArticleModule::remove_exist(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
    }
}