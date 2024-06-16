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
use super::super::*;

pub fn install() {
    Agent::new("lucina")
    .on_line(Main, lucina)
    .install();
}

unsafe extern "C" fn lucina(fighter : &mut L2CFighterCommon) {
    unsafe {
		let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		if is_default(boma) {
			let fighter_kind = smash::app::utility::get_kind(boma);
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let lr = PostureModule::lr(boma);
			let stick_x = ControlModule::get_stick_x(boma) * lr;
			let stick_y = ControlModule::get_stick_y(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32;
			let cat1 = ControlModule::get_command_flag_cat(boma, 0);
			let mask_is_exist = ArticleModule::is_exist(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK);
			if fighter_kind == *FIGHTER_KIND_LUCINA {
				if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE].contains(&status_kind) || (smash::app::sv_information::is_ready_go() == false && !smash::app::smashball::is_training_mode()) {
					LUCINA_STANCE[ENTRY_ID] = 0;
					TIMER[ENTRY_ID] = 5;
				};
				TIMER[ENTRY_ID] += 1;
				if [
					*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_ATTACK_S3, 
					*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3,
					*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_HI4,
					*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
					*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
					*FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4,
					*FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_APPEAL
				].contains(&status_kind) || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 && LUCINA_STANCE[ENTRY_ID] == 0) || (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && LUCINA_STANCE[ENTRY_ID] == 0){
						TIMER[ENTRY_ID] = 5;
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					if LUCINA_STANCE[ENTRY_ID] == 0 {
						if [hash40("attack_air_n"), hash40("attack_air_b"), hash40("attack_air_f"), hash40("attack_air_hi"), hash40("attack_air_lw")].contains(&motion_kind) {
							TIMER[ENTRY_ID] = 5;
						}
					} else {
						if [hash40("attack_air_n"), hash40("attack_air_lw")].contains(&motion_kind) {
							TIMER[ENTRY_ID] = 5;
						}
					}
				}
				//Early endlag cancel 
				if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
					if cancel_frame - frame <= (10.0/MotionModule::rate(boma)) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
					};
				};
				if [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE].contains(&status_kind) {
					macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_fire"), false, true);
					macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
				};
				if TIMER[ENTRY_ID] >= 6 && smash::app::sv_information::is_ready_go() && ![*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE].contains(&status_kind){
						TIMER[ENTRY_ID] = 0;
						let elec1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("sword1"), &S1, &S1, 0.075, true, 0, 0, 0, 0, 0, true, true) as u32;
						let elec2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("sword1"), &S2, &S2, 0.15, true, 0, 0, 0, 0, 0, true, true) as u32;
						let elec3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("sword1"), &S3, &S3, 0.15, true, 0, 0, 0, 0, 0, true, true) as u32;
						let fire1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("sword1"), &S1, &S1, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
						let fire2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("sword1"), &S2, &S2, 0.275, true, 0, 0, 0, 0, 0, true, true) as u32;
						let fire3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("sword1"), &S3, &S3, 0.275, true, 0, 0, 0, 0, 0, true, true) as u32;
						EffectModule::set_rgb(boma, elec1, 5.0, 5.0, 4.0);
						EffectModule::set_rgb(boma, elec2, 5.0, 5.0, 4.0);
						EffectModule::set_rgb(boma, elec3, 5.0, 5.0, 4.0);
						EffectModule::set_rgb(boma, fire1, 0.0, 0.5, 6.0);
						EffectModule::set_rgb(boma, fire2, 0.0, 0.5, 6.0);
						EffectModule::set_rgb(boma, fire3, 0.0, 0.5, 6.0);
						if LUCINA_STANCE[ENTRY_ID] == 1 {
							EffectModule::set_visible(boma, elec1, true);
							EffectModule::set_visible(boma, elec2, true);
							EffectModule::set_visible(boma, elec3, true);
						} else {
							EffectModule::set_visible(boma, elec1, false);
							EffectModule::set_visible(boma, elec2, false);
							EffectModule::set_visible(boma, elec3, false);
						};
						if LUCINA_STANCE[ENTRY_ID] == 0 {
							EffectModule::set_visible(boma, fire1, true);
							EffectModule::set_visible(boma, fire2, true);
							EffectModule::set_visible(boma, fire3, true);
						} else {
							EffectModule::set_visible(boma, fire1, false);
							EffectModule::set_visible(boma, fire2, false);
							EffectModule::set_visible(boma, fire3, false);
						};
						if [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
							EffectModule::set_visible(boma, fire1, false);
							EffectModule::set_visible(boma, fire2, false);
							EffectModule::set_visible(boma, fire3, false);
							EffectModule::set_visible(boma, elec1, false);
							EffectModule::set_visible(boma, elec2, false);
							EffectModule::set_visible(boma, elec3, false);
						};
				};
				if [hash40("special_air_lw"), hash40("special_lw")].contains(&motion_kind) {
					if situation_kind == *SITUATION_KIND_AIR {
						CAN_DOWNB[ENTRY_ID] = 1;
					}
					if MotionModule::frame(boma) >= 10.0 && MotionModule::frame(boma) < 11.0 {
						if LUCINA_STANCE[ENTRY_ID] == 0 {
							LUCINA_STANCE[ENTRY_ID] = 1;
						} else {
							LUCINA_STANCE[ENTRY_ID] = 0;
						};
					};
					if MotionModule::frame(boma) >= 20.0 {
						CancelModule::enable_cancel(boma);
					};
					if MotionModule::frame(boma) > 19.0 && MotionModule::frame(boma) < 44.0 {
						MotionModule::set_rate(boma, 5.4);
					} else {
						MotionModule::set_rate(boma, 1.2);
					};
					WorkModule::set_flag(boma, false, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};

				if ![*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) {
					UPB_FALL[ENTRY_ID] = false;
				};

				//Stance Specific Shit
				if LUCINA_STANCE[ENTRY_ID] == 0 {
					
					if BAN_SIDEB[ENTRY_ID] == true && StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR{
						CAN_SIDEB[ENTRY_ID] = 0;
						BAN_SIDEB[ENTRY_ID] = false;
					};
					if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) && MotionModule::frame(boma) <= 5.0 {
						HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					};
					if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) && MotionModule::frame(boma) > 9.0 {
						HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					};
					if MotionModule::motion_kind(boma) == hash40("throw_lw") {
						if MotionModule::frame(boma) >= 16.0 {
							MotionModule::set_rate(boma, 2.272);
						};
					};
					//Landing Lag
					if MotionModule::motion_kind(boma) == hash40("landing_air_n") {
						let landing = ((1.0/(HERO_NAIR_LANDING/ cancel_frame))as f32);
						MotionModule::set_rate(boma, landing);
					};
					if MotionModule::motion_kind(boma) == hash40("landing_air_f") {
						let landing = ((1.0/(HERO_FAIR_LANDING/ cancel_frame))as f32);
						MotionModule::set_rate(boma, landing);
					};
					if MotionModule::motion_kind(boma) == hash40("landing_air_b") {
						let landing = ((1.0/(HERO_BAIR_LANDING/ cancel_frame))as f32);
						MotionModule::set_rate(boma, landing);
					};
					if MotionModule::motion_kind(boma) == hash40("landing_air_hi") {
						let landing = ((1.0/(HERO_UAIR_LANDING/ cancel_frame))as f32);
						MotionModule::set_rate(boma, landing);
					};
					if MotionModule::motion_kind(boma) == hash40("landing_air_lw") {
						let landing = ((1.0/(HERO_DAIR_LANDING/ cancel_frame))as f32);
						MotionModule::set_rate(boma, landing);
					};
					//Nair Anim Shift
					if [hash40("attack_air_n")].contains(&motion_kind) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_n2"), 1.0, 1.0, false, 0.0, false, false);
					};
					//Fair Anim Shift
					if [hash40("attack_air_f")].contains(&motion_kind) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_f2"), 1.0, 1.0, false, 0.0, false, false);
					};
					let max_rot = 60.0;
					let mut wrist_rot = ((frame-9.0)/(16.0-9.0))*max_rot;
					if wrist_rot > max_rot {
						wrist_rot = max_rot;
					} else if wrist_rot < 0.0 {
						wrist_rot = 0.0;
					};
					if  StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR || (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind){
						CAN_DOWNB[ENTRY_ID] = 0;
					};
					/*bone_const(boma, *FIGHTER_KIND_LUCINA, hash40("attack_air_f2"), hash40("handr"), 0.0, 40.0, 0.0, 0.0, wrist_rot, wrist_rot, 0.0, 0.0);
					bone_const(boma, *FIGHTER_KIND_LUCINA, hash40("attack_air_f2"), hash40("handr"), 40.0, 69.0, 0.0, 0.0, wrist_rot, 0.0, 0.0, 0.0);
					bone_const(boma, *FIGHTER_KIND_LUCINA, hash40("attack_air_f2"), hash40("rot"), 0.0, 40.0, wrist_rot*0.8, wrist_rot*0.8, 0.0, 0.0, 0.0, 0.0);
					bone_const(boma, *FIGHTER_KIND_LUCINA, hash40("attack_air_f2"), hash40("rot"), 40.0, 69.0, wrist_rot*0.8, 0.0, 0.0, 0.0, 0.0, 0.0);*/
					let joint_scale = smash::phx::Vector3f { x: HERO_SWORD_MULT, y: HERO_SWORD_MULT, z: HERO_SWORD_MULT };
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &joint_scale);	
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword1"), &joint_scale);	
					AttackModule::set_attack_scale(fighter.module_accessor, 1.0, true);
					if mask_is_exist {
						if ![hash40("entry_l"), hash40("entry_r")].contains(&motion_kind) { // ignore during entry animation
							ArticleModule::remove_exist(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
						}
					} 
				} else {
					if [*FIGHTER_STATUS_KIND_DASH].contains(&status_kind) {
						if MotionModule::frame(boma) == 3.0 {
								let speed = smash::phx::Vector3f { x: SWORDMASTER_DASH - WorkModule::get_param_float(fighter.module_accessor, hash40("dash_speed"), 0), y: 0.0, z: 0.0 };
								KineticModule::add_speed(boma, &speed);
						};
					};
					let joint_scale = smash::phx::Vector3f { x: SWORDMASTER_SWORD_MULT, y: SWORDMASTER_SWORD_MULT, z: SWORDMASTER_SWORD_MULT };
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &joint_scale);	
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword1"), &joint_scale);	
					AttackModule::set_attack_scale(fighter.module_accessor, SWORDMASTER_SWORD_MULT, true);
					if !mask_is_exist {
						ArticleModule::generate_article(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, false, -1);
						let article = ArticleModule::get_article(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK);
						let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
						let article_boma = sv_battle_object::module_accessor(article_id);
						MotionModule::change_motion(article_boma, Hash40::new("appeal_lw"), 0.0, 1.0, false, 0.0, false, false);
						MotionModule::set_frame(article_boma, 50.0, true);
						MotionModule::set_rate(article_boma, 0.0);
					} 
					//Landing Lag
					if MotionModule::motion_kind(boma) == hash40("landing_air_n") {
						let landing = ((1.0/(SWORD_NAIR_LANDING/ cancel_frame))as f32);
						MotionModule::set_rate(boma, landing);
					};
					if MotionModule::motion_kind(boma) == hash40("landing_air_f") {
						let landing = ((1.0/(SWORD_FAIR_LANDING/ cancel_frame))as f32);
						MotionModule::set_rate(boma, landing);
					};
					if MotionModule::motion_kind(boma) == hash40("landing_air_b") {
						let landing = ((1.0/(SWORD_BAIR_LANDING/ cancel_frame))as f32);
						MotionModule::set_rate(boma, landing);
					};
					if MotionModule::motion_kind(boma) == hash40("landing_air_hi") {
						let landing = ((1.0/(SWORD_UAIR_LANDING/ cancel_frame))as f32);
						MotionModule::set_rate(boma, landing);
					};
					if MotionModule::motion_kind(boma) == hash40("landing_air_lw") {
						let landing = ((1.0/(SWORD_DAIR_LANDING/ cancel_frame))as f32);
						MotionModule::set_rate(boma, landing);
					};
					//Anim Shift
					if [hash40("attack_air_hi")].contains(&motion_kind) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_hi_2"), 1.0, 1.0, false, 0.0, false, false);
					};
					if [hash40("attack_air_hi_2"), hash40("attack_air_hi2")].contains(&motion_kind) && frame >= 32.0 {
						CancelModule::enable_cancel(boma);
						reimpl_cancel_frame(fighter);
					};
					if [hash40("attack_air_b")].contains(&motion_kind) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_b2"), 1.0, 1.0, false, 0.0, false, false);
					};
					if [hash40("attack_air_b2")].contains(&motion_kind) && frame >= 46.0 {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
					};
					if [hash40("attack_air_lw")].contains(&motion_kind) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_lw2"), 1.0, 1.0, false, 0.0, false, false);
					};
					if [hash40("attack_11")].contains(&motion_kind) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_112"), 1.0, 1.0, false, 0.0, false, false);
					};
					if [hash40("attack_12")].contains(&motion_kind) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_122"), 1.0, 1.0, false, 0.0, false, false);
					};
					if [hash40("attack_dash")].contains(&motion_kind) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_dash2"), 1.0, 1.0, false, 0.0, false, false);
					};
					if [hash40("attack_s3_s")].contains(&motion_kind) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s3_s2"), 1.0, 1.0, false, 0.0, false, false);
					};
					if [hash40("attack_hi3")].contains(&motion_kind) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_hi32"), 1.0, 1.0, false, 0.0, false, false);
					};
					if [hash40("attack_lw3")].contains(&motion_kind) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_lw32"), 1.0, 1.0, false, 0.0, false, false);
					};
					//Grab Nerf
					if [hash40("catch"), hash40("catch_dash"), hash40("catch_turn")].contains(&MotionModule::motion_kind(boma)) {
						if MotionModule::frame(boma) < 2.0 {
							MotionModule::set_rate(boma, 0.5);
						} else {
							MotionModule::set_rate(boma, 1.0);
						};
					};
					/*if[hash40("special_s4_hi"), hash40("special_air_s4_hi")].contains(&motion_kind) && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && (frame >= 16.0 && frame <= 36.0){
						if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
						};
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
						};
					};*/
					/*if [hash40("landing_heavy")].contains(&motion_kind) {
						let decrease_drift  = smash::phx::Vector3f { x: 0.925, y: 1.0, z: 1.0 };
						println!("icky wavedash!");
						KineticModule::mul_speed(boma, &decrease_drift, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
					};*/
					if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&status_kind){
						WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
					};
					if [*FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD].contains(&status_kind) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
					};
					if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD].contains(&status_kind) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
					};
					if [*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
					};
					if [*FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, true);
					};
					if [*FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
							if !([hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) && frame > 13.0) {
								macros::SET_SPEED_EX(fighter, 0.25, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
							} else {
								KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
							};
						};
						if StatusModule::is_situation_changed(boma) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
						};
						if [hash40("special_n_end"), hash40("special_air_n_end")].contains(&motion_kind) {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
								MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n_end_hi"), 0.0, 1.0, false, 0.0, false, false);
							} else {
								MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n_end_hi"), 0.0, 1.0, false, 0.0, false, false);
							};
						};
						if [hash40("special_n_end_hi"), hash40("special_air_n_end_hi")].contains(&motion_kind) && frame > 12.0 {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
								MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n_end_lw"), 4.0, 1.0, false, 0.0, false, false);
							} else {
								MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n_end_lw"), 4.0, 1.0, false, 0.0, false, false);
							};
						};
						if [hash40("special_n_end_lw"), hash40("special_air_n_end_lw")].contains(&motion_kind) && frame > 12.0 {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
								MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n"), 4.0, 1.0, false, 0.0, false, false);
							} else {
								MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n"), 4.0, 1.0, false, 0.0, false, false);
							};
						};
					};
					if  StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR || (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind){
						BAN_SIDEB[ENTRY_ID] = false;
						CAN_DOWNB[ENTRY_ID] = 0;
					};
					if BAN_SIDEB[ENTRY_ID] == true {
							CAN_SIDEB[ENTRY_ID] = 1;
					} else {
							CAN_SIDEB[ENTRY_ID] = 0;
					};
					if [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
						BAN_SIDEB[ENTRY_ID] = true;
						//StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, true);
						if StatusModule::is_situation_changed(boma) || (StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && ray_check_pos(boma, 0.0, -3.0, true) == 1 && KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_FALL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
						};
						if [hash40("special_s1"), hash40("special_air_s1")].contains(&motion_kind) {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
								MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
							} else {
								MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
							};
						};
						if [hash40("special_s"), hash40("special_air_s")].contains(&motion_kind) {
							if frame > 8.0 {
								MotionModule::set_rate(boma, 0.8);
							};
							if frame > 56.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
							};
							if frame > 55.0 {
								CancelModule::enable_cancel(boma);
							};
							if frame < 41.0 || StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
								if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION{
									KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
								};
							} else {
								if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL{
									KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
								};
								MotionModule::set_rate(boma, 0.4);
							};
						};
					};
					if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
						HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
						if ![hash40("special_hi_1"), hash40("special_hi_2"), hash40("special_hi_3")].contains(&MotionModule::motion_kind(boma)) {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_hi_1"), 0.0, 1.0, false, 0.0, false, false);
						};
					};
					if [hash40("special_hi_1")].contains(&MotionModule::motion_kind(boma)) {
						if MotionModule::end_frame(boma) as f32 -MotionModule::frame(boma) < 3.0 {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_hi_2"), 0.0, 1.0, false, 0.0, false, false);
						};
						/*if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
						};*/
						if MotionModule::frame(boma) < 27.0 {
							KineticModule::clear_speed_all(boma);
						};
					};
					if [hash40("special_hi_2")].contains(&MotionModule::motion_kind(boma)) {
						if MotionModule::frame(boma) >= 0.0 {
							macros::SET_SPEED_EX(fighter,0.0, -6.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						};
						UPB_FALL[ENTRY_ID] = true;
						GroundModule::clear_pass_floor(boma);
					};
					if UPB_FALL[ENTRY_ID] == true && (status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL || [hash40("special_hi"),hash40("special_air_hi")].contains(&MotionModule::motion_kind(boma))) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("special_hi_3"), 0.0, 1.0, false, 0.0, false, false);
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
						UPB_FALL[ENTRY_ID] = false;
					};
					if [hash40("special_hi_3")].contains(&MotionModule::motion_kind(boma)) {
						if MotionModule::frame(boma) >= 52.0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
						};
					};
					if [*FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) {
						MotionModule::set_rate(boma, 0.75);
					};
				};
			};
		}
	};
}