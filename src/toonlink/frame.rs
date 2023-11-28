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

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn tink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let end_frame = MotionModule::end_frame(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if [hash40("special_air_hi")].contains(&motion_kind) {
				if end_frame-frame < 3.0 {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_hi"), 12.0, 1.0, false, 0.0, false, false);
				};
				if frame == 3.0 {
					macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_toonlink_rnd_attack"));
					macros::PLAY_SE(fighter, Hash40::new("se_toonlink_005"));
				};
				if frame > 2.0 && frame < 12.0 && (frame as i32 % 3 == 0) {
					macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"),  Hash40::new("top"), 0, 3, 0, 90, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
				};
				if SPEED_Y[ENTRY_ID] <= 0.0 {
					if (ControlModule::get_command_flag_cat(boma, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
					};
					AttackModule::clear_all(boma);
				};
			};
		}
	}
}

pub fn install() {
    smashline::install_agent_frames!(tink_frame);
}