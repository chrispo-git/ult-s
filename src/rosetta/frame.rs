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
    Agent::new("rosetta")
    .set_costume(get_marked_costumes("rosetta","rosetta"))
    .on_line(Main, rosa_frame)
    .install();

	Agent::new("rosetta_tico")
    .set_costume(get_marked_costumes("rosetta","rosetta"))
    .on_line(Main, tico_frame)
    .install();
}

unsafe extern "C" fn rosa_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
			{
				let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
				let fighter_kind = smash::app::utility::get_kind(boma);
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				let frame = MotionModule::frame(boma);
				if VariableModule::is_flag((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TICO_DEAD) == true || VariableModule::get_int((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_COOLDOWN) > 0{
					crate::transition_set!(ENTRY_ID, can_downb);
				} else {
					crate::transition_reset!(ENTRY_ID, can_downb);
				};
				if smash::app::sv_information::is_ready_go() == false {
					VariableModule::set_int((boma) as *mut _, 0, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_COOLDOWN);
					VariableModule::set_flag((boma) as *mut _, false, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TICO_DEAD);
				};
				//Teleport!
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && VariableModule::is_flag((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TICO_DEAD) == false && VariableModule::get_int((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_COOLDOWN) == 0{
					if frame > 11.0 && frame < 13.0 {
						macros::EFFECT(fighter, Hash40::new("rosetta_escape"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
						VariableModule::set_flag((boma) as *mut _, true, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TELEPORT);
						VariableModule::set_int((boma) as *mut _, 1, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_INVIS_FRAMES);
					};
					if frame > 16.0 && frame < 19.0 {
						HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
						VisibilityModule::set_whole(boma, false);
						JostleModule::set_status(boma, false);	
						let pos = smash::phx::Vector3f { x: TICO_X[ENTRY_ID], y: TICO_Y[ENTRY_ID], z: 0.0 };
						PostureModule::set_pos(boma, &pos);
						PostureModule::init_pos(boma, &pos, true, true);
						VariableModule::set_int((boma) as *mut _, 2, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_INVIS_FRAMES);
					};
					if frame > 24.0 && frame < 26.0 {
						macros::EFFECT(fighter, Hash40::new("rosetta_escape_end"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
						VariableModule::set_int((boma) as *mut _, 3, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_INVIS_FRAMES);
					};
					if frame > 25.0{
						VisibilityModule::set_whole(boma, true);
						JostleModule::set_status(boma, true);	
						VariableModule::set_int((boma) as *mut _, 4, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_INVIS_FRAMES);
						HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
						CancelModule::enable_cancel(boma);
					};
				} else {
					VariableModule::set_float((boma) as *mut _, PostureModule::pos_y(boma), FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLOAT_ROSA_Y);
					VariableModule::set_float((boma) as *mut _, PostureModule::pos_x(boma), FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLOAT_ROSA_X);
					VariableModule::set_int((boma) as *mut _, 0, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_INVIS_FRAMES);
				};
				if VariableModule::get_int((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_COOLDOWN) > 0 {
					VariableModule::dec_int((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_COOLDOWN);
				};
				if VariableModule::get_int((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_COOLDOWN) == 1 {
					smash::app::FighterUtil::flash_eye_info(boma);
					EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
				};
				if status_kind == *FIGHTER_STATUS_KIND_DEAD {
					VariableModule::set_flag((boma) as *mut _, false, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TICO_DEAD);
				};
			}
	}
}
unsafe extern "C" fn tico_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_ROSETTA {
			if VariableModule::is_flag((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TELEPORT) == false {
				TICO_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
				TICO_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
			};
			if [*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, *WEAPON_ROSETTA_TICO_STATUS_KIND_NONE].contains(&status_kind) {
				VariableModule::set_flag((boma) as *mut _, true, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TICO_DEAD);
			} /*else {
				if MotionModule::motion_kind(&mut *boma) == hash40("explode") && MotionModule::motion_kind(weapon.module_accessor) != hash40("explode") && !VariableModule::is_flag((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TICO_DEAD) {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("explode"), 1.0, 1.0, false, 0.0, false, false);
					VariableModule::set_flag((boma) as *mut _, true, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TICO_DEAD);
				}
			}*/
			if [*WEAPON_ROSETTA_TICO_STATUS_KIND_REBIRTH].contains(&status_kind) {
				VariableModule::set_flag((boma) as *mut _, false, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TICO_DEAD);
			};
			println!("TICO_X {}, TICO_Y {}",TICO_X[ENTRY_ID], TICO_Y[ENTRY_ID] );
			if VariableModule::is_flag((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TELEPORT) == true {
				if VariableModule::get_int((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_INVIS_FRAMES) == 1 {
					macros::EFFECT(weapon, Hash40::new("rosetta_escape"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
				};
				if VariableModule::get_int((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_INVIS_FRAMES) == 2 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					VisibilityModule::set_whole(weapon.module_accessor, false);
					JostleModule::set_status(weapon.module_accessor, false);	
					let pos = smash::phx::Vector3f { x: VariableModule::get_float((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLOAT_ROSA_X), y: VariableModule::get_float((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLOAT_ROSA_Y), z: 0.0 };
					PostureModule::set_pos(weapon.module_accessor, &pos);
					PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
				};
				if VariableModule::get_int((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_INVIS_FRAMES) == 3 {
					macros::EFFECT(weapon, Hash40::new("rosetta_escape_end"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				};
				if VariableModule::get_int((boma) as *mut _, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_INVIS_FRAMES) == 4 {
					JostleModule::set_status(weapon.module_accessor, true);	
					VisibilityModule::set_whole(weapon.module_accessor, true);
					VariableModule::set_flag((boma) as *mut _, false, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLAG_IS_TELEPORT);
					VariableModule::set_int((boma) as *mut _, TELEPORT_COOLDOWN, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_COOLDOWN);
					VariableModule::set_int((boma) as *mut _, 0, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_INVIS_FRAMES);
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
			};
		};
    }
}