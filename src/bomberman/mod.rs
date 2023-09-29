use smash::app::sv_animcmd::*;
use smash::phx::*;
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
use crate::util::*;


static mut MAKE_NEW_BOMB : [bool; 8] = [false; 8];
static mut NEW_BOMB_X : [f32; 8] = [0.0; 8];
static mut NEW_BOMB_Y : [f32; 8] = [0.0; 8];
static mut EXPLODE_END_TIMER : [i32; 8] = [0; 8];
static mut EXPLODE : [bool; 8] = [false; 8];

#[acmd_script(
    agent = "pacman",
    script =  "game_attack11bomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_jab1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 2.0, 361, 25, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "effect_attack11bomb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_jab1_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.7);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attack12bomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_jab2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 2.5, 361, 25, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "effect_attack12bomb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_jab2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.7);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attack13bomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_jab3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 3.0, 40, 115, 0, 70, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "effect_attack13bomb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_jab3_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.7);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attacks3bomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_ftilts(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 7.0, 361, 100, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 7.0, 361, 100, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("footl"), 8.0, 361, 100, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("hip"), 7.0, 0, 80, 45, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 4, 0, Hash40::new("kneel"), 7.0, 0, 80, 45, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 5, 0, Hash40::new("footl"), 8.0, 0, 80, 45, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attacks3hibomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_ftilthi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 7.0, 45, 80, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 7.0, 45, 80, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("footl"), 8.0, 45, 80, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("hip"), 7.0, 45, 80, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 4, 0, Hash40::new("kneel"), 7.0, 45, 80, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 5, 0, Hash40::new("footl"), 8.0, 45, 80, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attacks3lwbomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_ftiltlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 7.0, 28, 100, 0, 35, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 7.0, 28, 100, 0, 35, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("footl"), 8.0, 28, 100, 0, 35, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("hip"), 7.0, 350, 80, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 4, 0, Hash40::new("kneel"), 7.0, 350, 80, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 5, 0, Hash40::new("footl"), 8.0, 350, 80, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script(
    agent = "pacman",
    scripts =  ["effect_attacks3bomb","effect_attacks3hibomb","effect_attacks3lwbomb"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_ftilt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 1.0);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attacklw3bomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_dtilt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 7.0, 80, 70, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 7.0, 80, 70, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("footl"), 7.0, 80, 70, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "effect_attacklw3bomb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_dtilt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attackhi3bomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_utilt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 7.0, 110, 100, 0, 60, 5.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 7.0, 110, 100, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "effect_attackhi3bomb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_utilt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
			macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1.5, 7.5, 0, 0, -80, -105, 1.0, true, *EF_FLIP_YZ);
	}
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attackdashbomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_da(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 11.0, 60, 70, 0, 100, 6.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 11.0, 60, 70, 0, 100, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 8.0, 90, 70, 0, 100, 6.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 8.0, 90, 70, 0, 100, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "effect_attackdashbomb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_da_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attackairfbomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_fair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 45, 65, 0, 62, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footr"), 7.0, 45, 65, 0, 62, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "effect_attackairfbomb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_fair_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 7.5, 1, 0, -20, 90, 0.95, false);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attackairhibomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_uair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 85, 108, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.05, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footr"), 10.0, 85, 108, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.05, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "effect_attackairhibomb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_uair_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 1.0);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attackairbbomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_bair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 2.0, 367, 100, 50, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footr"), 2.0, 367, 100, 50, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneel"), 2.0, 367, 100, 50, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("footl"), 2.0, 367, 100, 50, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 9.0, 45, 99, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footr"), 9.0, 45, 99, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneel"), 9.0, 45, 99, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("footl"), 9.0, 45, 99, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "effect_attackairbbomb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_bair_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 1.0);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attackairnbomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_nair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 361, 80, 0, 50, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "effect_attackairnbomb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_nair_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "sound_attackairnbomb",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn bomb_nair_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "game_attackairlwbomb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_dair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 1.0, 367, 100, 40, 0, 2.3, 0.0, 0.0, 0.0, None, None, None, 0.7, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 1.0, 367, 100, 40, 0, 4.2, 4.0, 0.0, 0.0, None, None, None, 0.7, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 5.0, 60, 105, 0, 60, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 5.0, 60, 105, 0, 60, 5.2, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
#[acmd_script(
    agent = "pacman",
    script =  "effect_attackairlwbomb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_dair_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2, 4, 49.602, -59.68, 150.37, 0.35, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, 2, 39.6, -59.68, 150.37, 0.43, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, -1, 59.23, -52.05, 138.77, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 1, 49.23, -52.05, 138.77, 0.56, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2, 4, 49.602, -59.68, 150.37, 0.35, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, 2, 39.6, -59.68, 150.37, 0.43, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, -1, 59.23, -52.05, 138.77, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 1, 49.23, -52.05, 138.77, 0.56, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
}
#[acmd_script(
    agent = "pacman",
    scripts =  ["game_speciallwfailurebomb", "game_specialairlwfailurebomb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_detonate(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let is_left = PostureModule::lr(fighter.module_accessor) < 0.0;
    if macros::is_excute(fighter) {
        if is_left {
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhavel"),false);
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhaver"),true);
        } else {
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhavel"),true);
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhaver"),false);
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        EXPLODE[ENTRY_ID] = true;
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhavel"),false);
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhaver"),false);
    }
}
#[acmd_script(
    agent = "pacman",
    scripts =  ["effect_speciallwfailurebomb", "effect_specialairlwfailurebomb"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bomb_detonate_eff(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        }
    }
}
#[acmd_script(
    agent = "pacman",
    scripts =  ["sound_speciallwfailurebomb", "sound_specialairlwfailurebomb"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn bomb_detonate_snd(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) {
            macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
        }
    }
}
#[acmd_script(
    agent = "pacman_firehydrant",
    script =  "game_fly",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_bomb_fly(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let is_bomb = (WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_bomb {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("rot2"), 1.0, 361, 100, 5, 0, 5.0, 0.0, -1.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 3.0, /*Unk*/ false);
        }
    } else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("rot2"), 13.0, 45, 90, 0, 30, 5.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}
#[acmd_script(
    agent = "pacman_firehydrant",
    script =  "game_fall",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_bomb_fall(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let is_bomb = (WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_bomb {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 100, 5, 0, 5.0, 0.0, 6.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 3.0, /*Unk*/ false);
        }
    } else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 60, 80, 0, 60, 5.0, 0.0, 6.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(agent.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
}
#[acmd_script(
    agent = "pacman_firehydrant",
    script =  "game_down",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_bomb_down(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let is_bomb = (WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_bomb {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("rot2"), 1.0, 361, 100, 5, 0, 5.0, 0.0, -1.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 3.0, /*Unk*/ false);
        }
    } else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("rot2"), 10.0, 45, 90, 0, 30, 5.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}
#[acmd_script(
    agent = "pacman_firehydrant",
    script =  "game_wait",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bomb_bomb_wait(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let is_bomb = (WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if !is_bomb {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 5.0, 0.0, 6.0, 0.0, Some(0.0), Some(4.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        }
    }
}
#[fighter_frame( agent = FIGHTER_KIND_PACMAN)]
fn bomberman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let situation_kind = StatusModule::situation_kind(boma);
		let end_frame = MotionModule::end_frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
        let is_bomb = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
        if is_bomb {
            if !ArticleModule::is_exist(boma, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) {
                EXPLODE[ENTRY_ID] = false;
            }
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_switchhavel"),false);
                ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_switchhaver"),false);
            }
        }
    }
}
#[weapon_frame( agent = WEAPON_KIND_PACMAN_FIREHYDRANT, main)]
fn bomb_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
        let woma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
        let situation = StatusModule::situation_kind(weapon.module_accessor);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let is_bomb = (WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
		let is_near_ground = GroundModule::ray_check(weapon.module_accessor, &Vector2f{ x: PostureModule::pos_x(weapon.module_accessor), y: PostureModule::pos_y(weapon.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true);
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_PACMAN  && is_bomb{
			ModelModule::set_scale(weapon.module_accessor, 0.769);
            if MAKE_NEW_BOMB[ENTRY_ID] {
                MAKE_NEW_BOMB[ENTRY_ID] = false;
                let pos = smash::phx::Vector3f { x: NEW_BOMB_X[ENTRY_ID], y: NEW_BOMB_Y[ENTRY_ID]+4.0, z: 0.0 };
                PostureModule::set_pos(weapon.module_accessor, &pos);
                PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
                println!("New Bombed");
            }
            if EXPLODE_END_TIMER[ENTRY_ID] == 0 {
                if status_kind == *WEAPON_PACMAN_FIREHYDRANT_STATUS_KIND_FLY && (situation == *SITUATION_KIND_GROUND){
                    MAKE_NEW_BOMB[ENTRY_ID] = true;
                    NEW_BOMB_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
                    NEW_BOMB_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
                    ArticleModule::generate_article(&mut *boma, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT, false, -1);
                    println!("End Bombed");
                }
            }
            if EXPLODE_END_TIMER[ENTRY_ID] > 0{
                EXPLODE_END_TIMER[ENTRY_ID] -= 1;
            }
            if EXPLODE_END_TIMER[ENTRY_ID] == 1{
                EXPLODE[ENTRY_ID] = false;
                StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PACMAN_FIREHYDRANT_STATUS_KIND_REMOVE, false);
            }
            if EXPLODE_END_TIMER[ENTRY_ID] == 15{
                AttackModule::clear_all(weapon.module_accessor);
            }
            if EXPLODE_END_TIMER[ENTRY_ID] == 0 && (AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) || EXPLODE[ENTRY_ID] == true) {
                KineticModule::clear_speed_all(weapon.module_accessor);
                EXPLODE_END_TIMER[ENTRY_ID] = 19;
                AttackModule::clear_all(weapon.module_accessor);
                VisibilityModule::set_model_visible(weapon.module_accessor, false);
                macros::EFFECT(weapon, Hash40::new("sys_bomb_a"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
                macros::ATTACK(weapon, 0, 0, Hash40::new("rot"), 13.0, 45, 90, 0, 30, 13.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            }
		};
    }
}
pub fn install() {
	smashline::install_acmd_scripts!(
        //Jabs
        bomb_jab1,
        bomb_jab1_eff,

        bomb_jab2,
        bomb_jab2_eff,
        
        bomb_jab3,
        bomb_jab3_eff,

        
        //Dash Attack
        bomb_da,
        bomb_da_eff,


        //Tilts
        bomb_ftilts, bomb_ftilthi, bomb_ftiltlw,
        bomb_ftilt_eff,

        bomb_dtilt,
        bomb_dtilt_eff,

        bomb_utilt,
        bomb_utilt_eff,


        //Aerials
        bomb_fair,
        bomb_fair_eff,

        bomb_uair,
        bomb_uair_eff,

        bomb_bair,
        bomb_bair_eff,

        bomb_nair,
        bomb_nair_eff,
        bomb_nair_snd,

        bomb_dair,
        bomb_dair_eff,


        //Specials
        bomb_detonate,
        bomb_detonate_eff,
        bomb_detonate_snd,

        //Bomb
        bomb_bomb_fly,
        bomb_bomb_down,
        bomb_bomb_fall,
        bomb_bomb_wait
    );
    smashline::install_agent_frames!(
        bomberman_frame,
        bomb_frame
    );
}
