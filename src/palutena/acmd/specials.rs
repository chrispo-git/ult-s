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

#[acmd_script(
    agent = "palutena",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn palu_reticle(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 18.0); 
		for _ in 0..3 {
			if macros::is_excute(fighter) {
				WorkModule::set_float(fighter.module_accessor, PostureModule::pos_x(fighter.module_accessor)+ (90.0 *PostureModule::lr(fighter.module_accessor)), *FIGHTER_PALUTENA_STATUS_SPECIAL_N_WORK_FLOAT_TARGET_POS_X);
				WorkModule::set_float(fighter.module_accessor, PostureModule::pos_y(fighter.module_accessor)+10.0, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_WORK_FLOAT_TARGET_POS_Y);
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTOAIMBULLET, true, 0);
			}
			wait(fighter.lua_state_agent, 7.0);
		}
		wait(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, 1.1);
}		
#[acmd_script(
    agent = "palutena",
    scripts =  ["game_speciallw", "game_specialairlw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn palu_black_hole(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, 4.0);
		frame(fighter.lua_state_agent, 5.0); 
		macros::FT_MOTION_RATE(fighter, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.4, 367, 100, 60, 0, 9.0, 0.0, 14.5, 19.0, None, None, None, 0.1, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
			AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
		}
		frame(fighter.lua_state_agent, 15.0); 
		if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
				macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 40, 50, 0, 80, 9.0, 0.0, 14.5, 19.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
				AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 30.0);
		macros::FT_MOTION_RATE(fighter, 0.5);
}	
#[acmd_script(
    agent = "palutena",
    scripts =  ["effect_speciallw", "effect_specialairlw"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn palu_black_hole_eff(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		let mut eff = 0 as u32;
		let mut scale = 0.2;
		frame(fighter.lua_state_agent, 4.0); 
		if macros::is_excute(fighter) {
			let pos = smash::phx::Vector3f { x: 0.0, y: 14.5, z: 19.0 };
			let rot = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
			scale = 0.2;
			eff = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40::new("palutena_final_blackhole"), smash::phx::Hash40::new("top"), &pos, &rot, scale, true, 0, 0, 0, 0, 0, true, true) as u32;
			EffectModule::set_rate(fighter.module_accessor, eff, 9.75);
		}
		frame(fighter.lua_state_agent, 9.0); 
		if macros::is_excute(fighter) {
			EffectModule::set_rate(fighter.module_accessor, eff, 6.75);
		}
		frame(fighter.lua_state_agent, 19.0); 
		if macros::is_excute(fighter) {
			EffectModule::set_rate(fighter.module_accessor, eff, 3.75);
		}
		frame(fighter.lua_state_agent, 29.0); 
		if macros::is_excute(fighter) {
			EffectModule::set_rate(fighter.module_accessor, eff, 1.75);
		}
		/*frame(fighter.lua_state_agent, 25.0); 
		for _ in 0..40 {
			if macros::is_excute(fighter) {
				let scales = smash::phx::Vector3f { x: scale, y: scale, z: scale };
				EffectModule::set_scale(fighter.module_accessor, eff, &scales);
				scale *= 0.8;
			}
			wait(fighter.lua_state_agent, 1.0);
		}*/
}		
#[acmd_script(
    agent = "palutena",
    scripts =  ["sound_speciallw", "sound_specialairlw"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn palu_black_hole_snd(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0); 
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_palutena_final02"));
			macros::PLAY_SE(fighter, Hash40::new("se_palutena_final02"));
		}
		frame(fighter.lua_state_agent, 18.0); 
		if macros::is_excute(fighter) {
			macros::STOP_SE(fighter, Hash40::new("se_palutena_final02"));
		}
}	

pub fn install() {
    smashline::install_acmd_scripts!(
		palu_reticle,
        palu_black_hole, palu_black_hole_eff, palu_black_hole_snd
    );
}