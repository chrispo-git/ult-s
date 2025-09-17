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
use crate::peppy::*;
use super::*;

pub fn install() {
    Agent::new("falco")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
    .acmd("game_speciallwpeppy", peppy_shine, Priority::Low)    
    .acmd("game_specialairlwpeppy", peppy_shine, Priority::Low)     
    .acmd("effect_speciallwpeppy", peppy_shine_eff, Priority::Low)   
    .acmd("effect_specialairlwpeppy", peppy_shine_eff, Priority::Low)   
    .acmd("sound_speciallwpeppy", peppy_shine_snd, Priority::Low)   
    .acmd("sound_specialairlwpeppy", peppy_shine_snd, Priority::Low)   
    .acmd("effect_specialairsstartpeppy", peppy_tether_eff, Priority::Low)   
    .acmd("game_specialairsstartpeppy", peppy_tether, Priority::Low)   
    .acmd("game_specialairspeppy", peppy_tether_end, Priority::Low)   
    .acmd("game_specialairnlooppeppy", peppy_neutralb_shoot, Priority::Low)  
    .acmd("game_specialnlooppeppy", peppy_neutralb_shoot, Priority::Low)  
    .acmd("effect_specialairnlooppeppy", peppy_neutralb_shoot_eff, Priority::Low)  
    .acmd("effect_specialnlooppeppy", peppy_neutralb_shoot_eff, Priority::Low)  
    .acmd("sound_specialairnlooppeppy", peppy_neutralb_shoot_snd, Priority::Low)  
    .acmd("sound_specialnlooppeppy", peppy_neutralb_shoot_snd, Priority::Low)  
    .install();

	Agent::new("falco_missile")
    .acmd("game_regular", peppy_missile, Priority::Low)     
    .acmd("effect_regular", peppy_missile_eff, Priority::Low)     
    .acmd("game_explode", peppy_missile_boom, Priority::Low)     
    .acmd("effect_explode", peppy_missile_boom_eff, Priority::Low)   
    .acmd("sound_explode", peppy_missile_boom_snd, Priority::Low)   
    .install();
}	

unsafe extern "C" fn peppy_shine_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_falco_special_l01"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_falco_special_l03"));
    }
}	
unsafe extern "C" fn peppy_shine(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 110, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
			macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.1);
			AttackModule::enable_safe_pos(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 15.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.75);
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, /*Type*/ *COLLISION_KIND_REFLECTOR, /*ID*/ *FIGHTER_FALCO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
		}
}		
unsafe extern "C" fn peppy_shine_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FLW_POS(fighter, Hash40::new_raw(0x0ecca63d69), Hash40::new("top"), 0.0, 7.0, 0, 0, 0, 0, 1.2, true);
			EffectModule::preset_limit_num(fighter.module_accessor, 2);
			macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x0fdc7fb0a0), Hash40::new("top"), 0.0, 7.0, 0, 0, 0, 0, 1.2, true);
			macros::FLASH(fighter, 1, 1, 1, 0.627);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FLW_POS(fighter, Hash40::new_raw(0x0fecc8ba2c), Hash40::new("reflector"), 1.4, -0.6, 0, 0, 0, 0, 1, true);
			macros::FLASH(fighter, 0, 1, 1, 0.431);
			macros::FLASH_FRM(fighter, 20, 0, 0.706, 0.392, 0);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::COL_NORMAL(fighter);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0ecca63d69), false, false);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0d30ab52b6), false, false);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0fdc7fb0a0), true, false);
		}
}			
unsafe extern "C" fn peppy_tether_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
unsafe extern "C" fn peppy_tether(agent: &mut L2CAgentBase) {
	macros::FT_MOTION_RATE(agent, 0.67);
	if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    frame(agent.lua_state_agent, 16.0);
	macros::FT_MOTION_RATE(agent, 1);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 361, 100, 1, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}	
unsafe extern "C" fn peppy_neutralb_start_eff(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}	
unsafe extern "C" fn peppy_neutralb_shoot(agent: &mut L2CAgentBase) {
	macros::FT_MOTION_RATE(agent, 0.5);
	if macros::is_excute(agent) {
		WorkModule::off_flag(agent.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
		if !ArticleModule::is_exist(agent.module_accessor, FIGHTER_FALCO_GENERATE_ARTICLE_MISSILE) {
        	macros::PLAY_SE(agent, Hash40::new("se_common_bomb_m"));
			ArticleModule::generate_article(agent.module_accessor, FIGHTER_FALCO_GENERATE_ARTICLE_MISSILE, false, -1);
		}
    } 
}	
unsafe extern "C" fn peppy_neutralb_shoot_snd(agent: &mut L2CAgentBase) {
}	
unsafe extern "C" fn peppy_neutralb_shoot_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}	
unsafe extern "C" fn peppy_tether_end(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, -1);
    } 
}	
unsafe extern "C" fn peppy_missile(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 0, 100, 5, 0, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        	AttackModule::enable_safe_pos(fighter.module_accessor);
		}
}	
unsafe extern "C" fn peppy_missile_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	for _ in 0..i32::MAX {
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
		}
    	wait(fighter.lua_state_agent, 2.0);
	}
}	
unsafe extern "C" fn peppy_missile_boom(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 45, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.01);
	}
    frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
	}
}	
unsafe extern "C" fn peppy_missile_boom_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
		macros::EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn peppy_missile_boom_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}