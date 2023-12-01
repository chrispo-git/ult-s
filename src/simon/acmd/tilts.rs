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
    smashline::install_acmd_scripts!(
		whip_dtilt, simon_dtilt, simon_dtilt_eff, simon_dtilt_snd
    );
}


#[acmd_script(
    agent = "simon_whip",
    script =  "game_attacklw3",
    category = ACMD_GAME)]
unsafe fn whip_dtilt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_NONE);
    }
	let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
	WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(object as *mut smash::app::Weapon);
	frame(agent.lua_state_agent, 11.0);
	if macros::is_excute(agent) {
		PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_COLLIDE);
	}
	frame(agent.lua_state_agent, 23.0);
	if macros::is_excute(agent) {
		PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_MOVE);
	}
}	
#[acmd_script(
    agent = "simon",
    script =  "game_attacklw3",
    category = ACMD_GAME)]
unsafe fn simon_dtilt(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	if macros::is_excute(agent) {
		ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_WHIP,smash::phx::Hash40::new("attack_lw3"),false,0.0);
	}
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 2.5, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(34.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(agent.lua_state_agent, 14.0);
	if macros::is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 92, 90, 0, 50, 2.5, 0.0, 5.0, 34.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
		macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 82, 90, 0, 50, 2.5, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(34.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
		macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 72, 90, 0, 50, 4.5, 0.0, 5.0, 7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		search!(agent, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
	}
	wait(agent.lua_state_agent, 2.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
}	
#[acmd_script(
    agent = "simon",
    script =  "effect_attacklw3",
    category = ACMD_EFFECT)]
unsafe fn simon_dtilt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("simon_whip_straight"), Hash40::new("haver"), 0, 0, 0, 4, 30, 4, 0.98, true);
    }
}	
#[acmd_script(
    agent = "simon",
    script =  "sound_attacklw3",
    category = ACMD_SOUND)]
unsafe fn simon_dtilt_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_simon_whip_holding"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_simon_attackhard_s01"));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_simon_rnd_attack"));
    }
}	