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
	Agent::new("dedede")
	.on_line(Main, dedede_frame)
	.install();
	Agent::new("dedede_gordo")
	.on_line(Main, gordo_frame)
	.install();
}

unsafe extern "C" fn dedede_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let fighter_kind = smash::app::utility::get_kind(boma);
			let pos_x = PostureModule::pos_x(boma);
			let lr = PostureModule::lr(boma);
			let mut upper_y = 10.0 + HIT_Y[ENTRY_ID];
			let mut lower_y = -10.0 + HIT_Y[ENTRY_ID];
			if ATK_HEIGHT[ENTRY_ID] == 1 {
				upper_y += HIGH_ADD;
				lower_y += HIGH_ADD;
			} else if ATK_HEIGHT[ENTRY_ID] == 2 {
				upper_y += LOW_ADD;
				lower_y += LOW_ADD;
			} else  if ATK_HEIGHT[ENTRY_ID] == 3 {
				upper_y += HIGH_ADD;
				lower_y += LOW_ADD;
			}

			if [hash40("attack_lw3")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 47.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
				};
			};
			HIT_Y[ENTRY_ID] = AttackModule::pos_y(boma);
			if AttackModule::is_attack(fighter.module_accessor, 0, false) {
				HIT0[ENTRY_ID] = AttackModule::pos_x(boma, 0, false);
				HIT0S[ENTRY_ID] = AttackModule::size(boma, 0);
				/*macros::SEARCH(fighter, 0, 0, Hash40::new("top"), HIT0S[ENTRY_ID]+1.0, 0.0, lower_y, (HIT0[ENTRY_ID]-pos_x)*lr, Some(0.0), Some(upper_y), Some((HIT0[ENTRY_ID]-pos_x)*lr), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false);
				search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);*/
				println!("Pos X{}, Scale {}, upper_y {}, lower_y {}", HIT0[ENTRY_ID], HIT0S[ENTRY_ID], upper_y, lower_y);
			} else {
				HIT0[ENTRY_ID] = 999.0;
				HIT0S[ENTRY_ID] = 0.01;
			}
			if AttackModule::is_attack(fighter.module_accessor, 1, false) {
				HIT1[ENTRY_ID] = AttackModule::pos_x(boma, 1, false);
				HIT1S[ENTRY_ID] = AttackModule::size(boma, 1);
			} else {
				HIT1[ENTRY_ID] = 999.0;
				HIT1S[ENTRY_ID] = 0.01;
			}
			if AttackModule::is_attack(fighter.module_accessor, 2, false) {
				HIT2[ENTRY_ID] = AttackModule::pos_x(boma, 2, false);
				HIT2S[ENTRY_ID] = AttackModule::size(boma, 2);
			} else {
				HIT2[ENTRY_ID] = 999.0;
				HIT2S[ENTRY_ID] = 0.02;
			}
			if AttackModule::is_attack(fighter.module_accessor, 3, false) {
				HIT3[ENTRY_ID] = AttackModule::pos_x(boma, 3, false);
				HIT3S[ENTRY_ID] = AttackModule::size(boma, 3);
			} else {
				HIT3[ENTRY_ID] = 999.0;
				HIT3S[ENTRY_ID] = 0.03;
			}
			if AttackModule::is_attack(fighter.module_accessor, 4, false) {
				HIT4[ENTRY_ID] = AttackModule::pos_x(boma, 4, false);
				HIT4S[ENTRY_ID] = AttackModule::size(boma, 4);
			} else {
				HIT4[ENTRY_ID] = 999.0;
				HIT4S[ENTRY_ID] = 0.04;
			}
			if AttackModule::is_attack(fighter.module_accessor, 5, false) {
				HIT5[ENTRY_ID] = AttackModule::pos_x(boma, 5, false);
				HIT5S[ENTRY_ID] = AttackModule::size(boma, 5);
			} else {
				HIT5[ENTRY_ID] = 999.0;
				HIT5S[ENTRY_ID] = 0.05;
			}
			if AttackModule::is_attack(fighter.module_accessor, 6, false) {
				HIT6[ENTRY_ID] = AttackModule::pos_x(boma, 6, false);
				HIT6S[ENTRY_ID] = AttackModule::size(boma, 6);
			} else {
				HIT6[ENTRY_ID] = 999.0;
				HIT6S[ENTRY_ID] = 0.06;
			}
			if [*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_FALL, 
			*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_PASS, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_TURN, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_JUMP1,
			*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_JUMP2, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_DRINK_ITEM, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_LANDING, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_TURN_AIR,
			*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SWALLOW_WAIT, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_FALL, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_JUMP, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_PASS,
			*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
				HIT0[ENTRY_ID] = 999.0;
				HIT0S[ENTRY_ID] = 0.01;
				HIT1[ENTRY_ID] = 999.0;
				HIT1S[ENTRY_ID] = 0.01;
				HIT2[ENTRY_ID] = 999.0;
				HIT2S[ENTRY_ID] = 0.01;
				HIT3[ENTRY_ID] = 999.0;
				HIT3S[ENTRY_ID] = 0.01;
			}
			if [hash40("attack_lw3"), hash40("attack_lw4"), hash40("attack_dash"), hash40("attack_air_lw")].contains(&MotionModule::motion_kind(boma)) || [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_HIT, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE].contains(&status_kind) {
				ATK_HEIGHT[ENTRY_ID] = 1;
			} else if [hash40("attack_hi3"), hash40("attack_hi4"), hash40("attack_air_hi")].contains(&MotionModule::motion_kind(boma)) {
				ATK_HEIGHT[ENTRY_ID] = 2;
			} else if [hash40("attack_s4_s"), hash40("attack_air_f")].contains(&MotionModule::motion_kind(boma)) {
				ATK_HEIGHT[ENTRY_ID] = 3;
			} else {
				ATK_HEIGHT[ENTRY_ID] = 0;
			}
		}
	};
}
unsafe extern "C" fn gordo_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let otarget_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner_boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let pos_y = PostureModule::pos_y(boma);
		let pos_x = PostureModule::pos_x(boma);
        if true {//smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_DEDEDE {
			let mut change_team = false;
			let mut upper_y = 10.0 + HIT_Y[ENTRY_ID];
			let mut lower_y = -10.0 + HIT_Y[ENTRY_ID];
			if ATK_HEIGHT[ENTRY_ID] == 1 {
				upper_y += HIGH_ADD;
				lower_y += HIGH_ADD;
			} else if ATK_HEIGHT[ENTRY_ID] == 2 {
				upper_y += LOW_ADD;
				lower_y += LOW_ADD;
			} else  if ATK_HEIGHT[ENTRY_ID] == 3 {
				upper_y += HIGH_ADD;
				lower_y += LOW_ADD;
			}
			if ((pos_x-HIT0[ENTRY_ID]).abs() < (HIT0S[ENTRY_ID]+20.0) && (pos_y <= upper_y && pos_y >= lower_y)) ||
			((pos_x-HIT1[ENTRY_ID]).abs() < (HIT1S[ENTRY_ID]+20.0) && (pos_y <= upper_y && pos_y >= lower_y)) ||
			((pos_x-HIT2[ENTRY_ID]).abs() < (HIT2S[ENTRY_ID]+20.0) && (pos_y <= upper_y && pos_y >= lower_y)) ||
			((pos_x-HIT3[ENTRY_ID]).abs() < (HIT3S[ENTRY_ID]+20.0) && (pos_y <= upper_y && pos_y >= lower_y)) ||
			((pos_x-HIT4[ENTRY_ID]).abs() < (HIT4S[ENTRY_ID]+20.0) && (pos_y <= upper_y && pos_y >= lower_y)) ||
			((pos_x-HIT5[ENTRY_ID]).abs() < (HIT5S[ENTRY_ID]+20.0) && (pos_y <= upper_y && pos_y >= lower_y)) ||
			((pos_x-HIT6[ENTRY_ID]).abs() < (HIT6S[ENTRY_ID]+20.0) && (pos_y <= upper_y && pos_y >= lower_y)) {
				change_team = true;
			}
			if [hash40("special_s_start"), hash40("special_air_s_start")].contains(&MotionModule::motion_kind(&mut *owner_boma)) {
				change_team = false;
			}
			if change_team {
				TeamModule::set_hit_team(fighter.module_accessor, -1);
				TeamModule::set_team(fighter.module_accessor, -1, true);
				TeamModule::set_team_owner_id(fighter.module_accessor, 0x50000000);
				HitModule::set_no_team(fighter.module_accessor, true);
				println!("gordo switch!");
			}
			println!("Pos X Gordo {}, Pos Y Gordo {}", pos_x, pos_y);
		}
	};
}