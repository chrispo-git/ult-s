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
    Agent::new("packun")
    .acmd("game_specialsshoot", plant_sideb, Priority::Low)    
    .acmd("effect_specialsshoot", plant_sideb_eff, Priority::Low)    
    .acmd("sound_specialsshoot", plant_sideb_snd, Priority::Low)    
    .acmd("game_specialairsshoot", plant_sideb, Priority::Low)    
    .acmd("effect_specialairsshoot", plant_sideb_eff, Priority::Low)    
    .acmd("sound_specialairsshoot", plant_sideb_snd, Priority::Low)    
    .install();

    Agent::new("packun_poisonbreath")
    .acmd("game_explode", poison_explosion, Priority::Low)    
    .acmd("effect_explode", poison_explosion_eff, Priority::Low)    
    .acmd("sound_explode", poison_explosion_snd, Priority::Low)    
    .install();
}

unsafe extern "C" fn plant_sideb_eff(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 6.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("packun_poison_breath"), Hash40::new("mouth"), 5, -0.6, 0, 0, 90, -100, 1.1, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.6);
        }
	} else {
		if macros::is_excute(agent) {
			macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("packun_atk_air_b_breath"), Hash40::new("mouth"), 7, 0.2, 0, 0, 0, -90, 1, true);
		}
	}
	frame(agent.lua_state_agent, 25.0);
	if macros::is_excute(agent) {
		macros::EFFECT_DETACH_KIND(agent, Hash40::new("packun_poison_breath"), -1);
		macros::EFFECT_DETACH_KIND(agent, Hash40::new("packun_atk_air_b_breath"), -1);
	}
	frame(agent.lua_state_agent, 29.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 4, -2, 0, 0, 90, -125, 1, true);
        }
	}
}	
unsafe extern "C" fn plant_sideb_snd(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
		if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
			macros::PLAY_SE(agent, Hash40::new("se_packun_attackair_b01"));
		} else {
			macros::PLAY_SE(agent, Hash40::new("se_packun_special_s03"));
		}
    }
}	
unsafe extern "C" fn plant_sideb(agent: &mut L2CAgentBase) {
	let ENTRY_ID = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 10.0, 3.0);
    }
	frame(agent.lua_state_agent, 2.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
		if macros::is_excute(agent) {
        	macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 18, 100, 30, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		}
	}
	frame(agent.lua_state_agent, 6.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
		if macros::is_excute(agent) {
			IS_SIDEB_EAT[ENTRY_ID] = true;
        	macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 100, 70, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		}
	}
	frame(agent.lua_state_agent, 10.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_POISONBREATH, false, -1);
        }
	}
	frame(agent.lua_state_agent, 14.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
		if macros::is_excute(agent) {
			IS_SIDEB_EAT[ENTRY_ID] = false;
        	AttackModule::clear_all(agent.module_accessor);
		}
	}
	frame(agent.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(agent, 0.9);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 5.0, 5.0);
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC);
    }
    frame(agent.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(agent, 0.55);
}	
unsafe extern "C" fn poison_explosion(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 45, 150, 0, 45, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 45, 176, 0, 45, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
	wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		WorkModule::set_int(fighter.module_accessor, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	}
}	

unsafe extern "C" fn poison_explosion_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40::new("packun_poison_breath"), false, false);
		EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40::new("packun_poison_breath2"), false, false);
		EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40::new("packun_poison_gas"), false, false);
		EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40::new("packun_poison_max"), false, false);
		EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40::new("packun_poison_max_smoke"), false, false);
		EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40::new("packun_poison_mouth"), false, false);
		EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40::new("packun_poison_mouth2"), false, false);
		macros::EFFECT(fighter, Hash40::new("sys_flame"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, true);
		macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}
unsafe extern "C" fn poison_explosion_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}