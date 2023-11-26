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

#[fighter_frame( agent = FIGHTER_KIND_ROSETTA )]
fn rosa_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let frame = MotionModule::frame(boma);
			if IS_TICO_DEAD[ENTRY_ID] == true || COOLDOWN[ENTRY_ID] > 0{
				CAN_DOWNB[ENTRY_ID] = 1;
			} else {
				CAN_DOWNB[ENTRY_ID] = 0;
			};
			if smash::app::sv_information::is_ready_go() == false {
				COOLDOWN[ENTRY_ID] = 0;
				IS_TICO_DEAD[ENTRY_ID] = false;
			};
			//Teleport!
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && IS_TICO_DEAD[ENTRY_ID] == false && COOLDOWN[ENTRY_ID] == 0{
				if frame > 11.0 && frame < 13.0 {
					macros::EFFECT(fighter, Hash40::new("rosetta_escape"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
					IS_TELEPORT[ENTRY_ID] = true;
					INVIS_FRAMES[ENTRY_ID] = 1;
				};
				if frame > 16.0 && frame < 19.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					VisibilityModule::set_whole(boma, false);
					JostleModule::set_status(boma, false);	
					let pos = smash::phx::Vector3f { x: TICO_X[ENTRY_ID], y: TICO_Y[ENTRY_ID], z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
					INVIS_FRAMES[ENTRY_ID] = 2;
				};
				if frame > 24.0 && frame < 26.0 {
					macros::EFFECT(fighter, Hash40::new("rosetta_escape_end"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
					INVIS_FRAMES[ENTRY_ID] = 3;
				};
				if frame > 25.0{
					VisibilityModule::set_whole(boma, true);
					JostleModule::set_status(boma, true);	
					INVIS_FRAMES[ENTRY_ID] = 4;
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					CancelModule::enable_cancel(boma);
				};
			} else {
				ROSA_Y[ENTRY_ID] = PostureModule::pos_y(boma);
				ROSA_X[ENTRY_ID] = PostureModule::pos_x(boma);
				INVIS_FRAMES[ENTRY_ID] = 0;
			};
			if COOLDOWN[ENTRY_ID] > 0 {
				COOLDOWN[ENTRY_ID] -= 1;
			};
			if COOLDOWN[ENTRY_ID] == 1 {
				smash::app::FighterUtil::flash_eye_info(boma);
				EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
			};
			if status_kind == *FIGHTER_STATUS_KIND_DEAD {
				IS_TICO_DEAD[ENTRY_ID] = false;
			};
	}
}
#[weapon_frame( agent = WEAPON_KIND_ROSETTA_TICO )]
fn tico_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_ROSETTA {
			if IS_TELEPORT[ENTRY_ID] == false {
				TICO_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
				TICO_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
			};
			if [*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, *WEAPON_ROSETTA_TICO_STATUS_KIND_NONE].contains(&status_kind) {
				IS_TICO_DEAD[ENTRY_ID] = true;
			} /*else {
				if MotionModule::motion_kind(&mut *boma) == hash40("explode") && MotionModule::motion_kind(weapon.module_accessor) != hash40("explode") && !IS_TICO_DEAD[ENTRY_ID] {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("explode"), 1.0, 1.0, false, 0.0, false, false);
					IS_TICO_DEAD[ENTRY_ID] = true;
				}
			}*/
			if [*WEAPON_ROSETTA_TICO_STATUS_KIND_REBIRTH].contains(&status_kind) {
				IS_TICO_DEAD[ENTRY_ID] = false;
			};
			println!("TICO_X {}, TICO_Y {}",TICO_X[ENTRY_ID], TICO_Y[ENTRY_ID] );
			if IS_TELEPORT[ENTRY_ID] == true {
				if INVIS_FRAMES[ENTRY_ID] == 1 {
					macros::EFFECT(weapon, Hash40::new("rosetta_escape"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
				};
				if INVIS_FRAMES[ENTRY_ID] == 2 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					VisibilityModule::set_whole(weapon.module_accessor, false);
					JostleModule::set_status(weapon.module_accessor, false);	
					let pos = smash::phx::Vector3f { x: ROSA_X[ENTRY_ID], y: ROSA_Y[ENTRY_ID], z: 0.0 };
					PostureModule::set_pos(weapon.module_accessor, &pos);
					PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
				};
				if INVIS_FRAMES[ENTRY_ID] == 3 {
					macros::EFFECT(weapon, Hash40::new("rosetta_escape_end"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				};
				if INVIS_FRAMES[ENTRY_ID] == 4 {
					JostleModule::set_status(weapon.module_accessor, true);	
					VisibilityModule::set_whole(weapon.module_accessor, true);
					IS_TELEPORT[ENTRY_ID] = false;
					COOLDOWN[ENTRY_ID] = TELEPORT_COOLDOWN;
					INVIS_FRAMES[ENTRY_ID] = 0;
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
			};
		};
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        tico_frame,
		rosa_frame
    );
}