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
use crate::kirby::*;
use super::*;
pub fn install() {
    smashline::install_agent_frames!( kirby_frame, ball_frame, finalcutter_frame, hat_frame);
}

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let situation_kind = StatusModule::situation_kind(boma);
			let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true);

			//CI shit
			if [hash40("special_input")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 0.0 {
					macros::SET_SPEED_EX(fighter,0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
			};

			//Downb shit
			if DOWNB_JUMP[ENTRY_ID] && status_kind == *FIGHTER_STATUS_KIND_JUMP {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw2"), 0.0, 1.0, false, 0.0, false, false);
				DOWNB_JUMP[ENTRY_ID] = false;
			};
			if DOWNB_JUMP[ENTRY_ID] && status_kind != *FIGHTER_STATUS_KIND_JUMP {
				DOWNB_JUMP[ENTRY_ID] = false;
			};
			if [hash40("special_lw")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 4.0 && MotionModule::frame(boma) <= 24.0 {
					if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
						DOWNB_JUMP[ENTRY_ID] = true;
						macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
						macros::PLAY_SE(fighter, Hash40::new("se_kirby_jump01"));
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
					};
				};
			};
			if [hash40("special_lw")].contains(&MotionModule::motion_kind(boma)) {
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
				};
				if MotionModule::frame(boma) >= 40.0 {
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
				};
			};
			if [hash40("special_lw2")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 29.0 {
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};

			//Upb shit
			if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2 && MotionModule::frame(boma) >= 37.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
			};
			if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2 {
				if  MotionModule::frame(boma) < 2.0 {
					let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
					if stick_x <= -0.3 {
						UPB_ANGLE[ENTRY_ID] = 0;
					} else if stick_x >= 0.45 {
						UPB_ANGLE[ENTRY_ID] = 2;
					} else {
						UPB_ANGLE[ENTRY_ID] = 1;
					};
				};
				if MotionModule::frame(boma) <= 6.0 && !is_hitlag(boma) {
					if UPB_ANGLE[ENTRY_ID] == 1 {
						//let speed = smash::phx::Vector3f { x: *(((6.0/MotionModule::frame(boma))*0.003)/6.0)*0.2)-0.03, y: 0.0, z: 0.0 };
						//KineticModule::add_speed(boma, &speed);
						macros::SET_SPEED_EX(fighter, 1.0, 0.12, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
					if UPB_ANGLE[ENTRY_ID] == 2 {
						//let speed = smash::phx::Vector3f { x: ((6.0/MotionModule::frame(boma))*0.05)/6.0, y: -0.03, z: 0.0 };
						//KineticModule::add_speed(boma, &speed);
						macros::SET_SPEED_EX(fighter, 1.75, 0.08, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
					if UPB_ANGLE[ENTRY_ID] == 0 {
						//let speed = smash::phx::Vector3f { x: *(((6.0/MotionModule::frame(boma))*0.003)/6.0)*0.2)-0.03, y: 0.0, z: 0.0 };
						//KineticModule::add_speed(boma, &speed);
						macros::SET_SPEED_EX(fighter, 0.0, 0.16, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
				};
				if MotionModule::frame(boma) <= 10.0 && MotionModule::frame(boma) > 6.0 && !is_hitlag(boma) {
					if UPB_ANGLE[ENTRY_ID] != 0 {
						KineticModule::clear_speed_all(boma);
						if UPB_ANGLE[ENTRY_ID] == 1 {
							let speed = smash::phx::Vector3f { x: -0.1, y: 0.0, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
						};
						/*let speed = smash::phx::Vector3f { x: -2.0*(0.02-((MotionModule::frame(boma)-6.0/6.0)*0.02)), y: 0.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);*/
					};
				};
			} else {
				UPB_ANGLE[ENTRY_ID] = 1;
			};
			if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
			};
			if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_FINAL || status_kind == *FIGHTER_STATUS_KIND_FINAL_JUMP_END {
				IS_FINAL[ENTRY_ID] = true;
			} else {
				IS_FINAL[ENTRY_ID] = false;
			};
			if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_START].contains(&status_kind) == false {
				ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD].contains(&status_kind) == false {
				ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
				StatusModule::change_status_request_from_script(boma,*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, true);
			};
			if status_kind != *FIGHTER_STATUS_KIND_ATTACK_LW4 {
				macros::STOP_SE(fighter, Hash40::new("se_kirby_special_h02"));
			};
		}
	}
}
#[weapon_frame( agent = WEAPON_KIND_KIRBY_FINALCUTTERSHOT )]
pub fn ball_frame(weapon : &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let frame = MotionModule::frame(weapon.module_accessor) as i32;
		if frame < 15 {
			if frame >= 2 {
				if frame % 4 == 0 {
					variance[ENTRY_ID] = 8.0;
				} else if (frame+1) % 2 == 0 {
					variance[ENTRY_ID] = 4.0;
				} else {
					variance[ENTRY_ID] = -2.0;
				};
			} else {
				variance[ENTRY_ID] = 0.0;
			};
			if frame % 3 == 0 {
				let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE, &NONE, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(weapon.module_accessor, f1, 1.0, 0.5, 3.0);
				EffectModule::set_alpha(weapon.module_accessor, f1, 0.65);
				EffectModule::set_rate(weapon.module_accessor, f1, 1.5);
				if frame >= 2 {
					let f2: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &N1, &NONE, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(weapon.module_accessor, f2, 1.0, 0.5, 3.0);
					EffectModule::set_alpha(weapon.module_accessor, f2, 0.65);
					EffectModule::set_rate(weapon.module_accessor, f2, 1.5);
				};
				if frame >= 5 {
					let f3: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &N2, &NONE, 0.35, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(weapon.module_accessor, f3, 1.0, 0.5, 3.0);
					EffectModule::set_alpha(weapon.module_accessor, f3, 0.65);
					EffectModule::set_rate(weapon.module_accessor, f3, 1.5);
				};
			};
			if frame % 5 == 0 {
				let f2: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("top"), &NONE, &NONE, 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
			};
			if frame % 20 == 0 {
				EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_sscope_bullet"), false, true);
				let f2: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_sscope_bullet"), smash::phx::Hash40::new("top"), &NONE, &NONE, 2.1, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(weapon.module_accessor, f2, 2.75, 0.5, 4.5);
			};
			if frame == 2 {
				F3[ENTRY_ID] = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_sscope_bullet"), smash::phx::Hash40::new("top"), &N1, &NONE, 0.84, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(weapon.module_accessor, F3[ENTRY_ID], 2.75, 0.5, 4.5);
				EffectModule::set_alpha(weapon.module_accessor, F3[ENTRY_ID], 0.65);
			};
			if frame == 5 {
				F4[ENTRY_ID] = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_sscope_bullet"), smash::phx::Hash40::new("top"), &N2, &NONE, 0.735, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(weapon.module_accessor, F4[ENTRY_ID], 2.75, 0.5, 4.5);
				EffectModule::set_alpha(weapon.module_accessor, F4[ENTRY_ID], 0.5);
			};
			if frame >= 2 {
				let n1 =  smash::phx::Vector3f { x: 0.0, y: 2.0+variance[ENTRY_ID], z: -15.0 };
				EffectModule::set_pos(boma, F3[ENTRY_ID], &n1);
			};
			if frame >= 5 {
				let n2 =  smash::phx::Vector3f { x: 0.0, y: 8.0-variance[ENTRY_ID], z: -24.0 };
				EffectModule::set_pos(boma, F4[ENTRY_ID], &n2);
			};
		} else {
			EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_sscope_bullet"), false, true);
		};
    }
}
#[weapon_frame( agent = WEAPON_KIND_KIRBY_HAMMER )]
fn finalcutter_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
		if [hash40("attack_s4_s")].contains(&MotionModule::motion_kind(boma)) {
			ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("HammerShape"),false);
			ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("HammerShape.001"),false);
		};
		if [hash40("attack_air_b")].contains(&MotionModule::motion_kind(boma)) {
			ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("UltraswordMShape"),true);
			ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("UltraswordMShape.001"),true);
		};
    }
}
#[weapon_frame( agent = WEAPON_KIND_KIRBY_HAT )]
fn hat_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let copy_kind = WorkModule::get_int(&mut *boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
		let motion_kind = MotionModule::motion_kind(&mut *boma);
		let status_kind = StatusModule::status_kind(&mut *boma);
		let frame = MotionModule::frame(&mut *boma);
		if copy_kind == *FIGHTER_KIND_PIKMIN {
			//println!("I HAVE RAYMAN");
            EffectModule::kill_kind(weapon.module_accessor, Hash40::new("pikmin_antenna"), false, false);
            EffectModule::kill_kind(weapon.module_accessor, Hash40::new("pikmin_antenna_damage"), false, false);
            EffectModule::kill_kind(weapon.module_accessor, Hash40::new("pikmin_antenna_damage"), true, true);
            EffectModule::kill_kind(weapon.module_accessor, Hash40::new("pikmin_antenna_damage"), false, true);
            EffectModule::kill_kind(weapon.module_accessor, Hash40::new("pikmin_antenna_damage"), true, false);
			if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PIKMIN_SPECIAL_N && frame < 112.0 {
				let scale = smash::phx::Vector3f { x: 1.0, y: 1.0, z: 1.0 };
				ModelModule::set_joint_scale(weapon.module_accessor, Hash40::new("rot"), &scale);
				//println!("I should be running?");
			} else {
				let scale = smash::phx::Vector3f { x: 0.00001, y: 0.00001, z: 0.00001 };
				ModelModule::set_joint_scale(weapon.module_accessor, Hash40::new("rot"), &scale);
			}
		}
    }
}