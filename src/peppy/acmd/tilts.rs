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
	Agent::new("falco")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
    .acmd("game_attacklw3peppy", peppy_dtilt, Priority::Low)    
    .acmd("effect_attacklw3peppy", peppy_dtilt_eff, Priority::Low)    
    .acmd("sound_attacklw3peppy", peppy_dtilt_snd, Priority::Low)    
    .acmd("game_attackhi3peppy", peppy_utilt, Priority::Low)    
    .acmd("effect_attackhi3peppy", peppy_utilt_eff, Priority::Low)    
    .acmd("sound_attackhi3peppy", peppy_utilt_snd, Priority::Low)    
    .acmd("game_attacks3", peppy_ftilt, Priority::Low)    
    .acmd("game_attacks3lw", peppy_ftilt, Priority::Low)    
    .acmd("game_attacks3hi", peppy_ftilt, Priority::Low)    
    .acmd("effect_attacks3", peppy_ftilt_eff, Priority::Low)    
    .acmd("effect_attacks3lw", peppy_ftilt_lw_eff, Priority::Low)    
    .acmd("effect_attacks3hi", peppy_ftilt_hi_eff, Priority::Low)    
    .install();
}
unsafe extern "C" fn peppy_dtilt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 6.0, 83, 46, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 6.0, 83, 46, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 6.0, 83, 46, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}	
unsafe extern "C" fn peppy_dtilt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_falco_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_02"));
    }
}	
unsafe extern "C" fn peppy_dtilt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 6, 1, -2.5, 0, -25, 0, 0.8, true, *EF_FLIP_YZ);
    }
}	
unsafe extern "C" fn peppy_utilt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 1.0, 108, 100, 60, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 1.0, 108, 100, 60, 0, 3.5, 4.0, 0.0, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 96, 130, 0, 28, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.05, 1.05, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 96, 130, 0, 28, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.05, 1.05, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}	
unsafe extern "C" fn peppy_utilt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 5, 6, -90, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 360, true, 0.4);
    }
}	
unsafe extern "C" fn peppy_utilt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_falco_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_s"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_m"));
    }
}	
unsafe extern "C" fn peppy_ftilt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 6.0, 6.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 7.0, 361, 100, 0, 30, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("handl"), 7.0, 361, 100, 0, 30, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 361, 100, 0, 30, 4.0, 0.0, 6.0, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 7.0, 361, 100, 0, 30, 3.0, 0.0, 6.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 6.0, 5.0);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 5.0, 3.0);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 4.0, 3.0);
    }
}	
unsafe extern "C" fn peppy_ftilt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -1, 6.5, -0.5, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 1, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}	
unsafe extern "C" fn peppy_ftilt_hi_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -2.5, 6.5, -1.75, -30, 5, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 1, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}	
unsafe extern "C" fn peppy_ftilt_lw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0.5, 7.25, 1.5, 30, -7.5, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 1, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}	