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

static mut STATIC_MUT : [i32; 8] = [6; 8];

#[acmd_script( agent = "ness_pkfire", scripts = ["game_pillar", "game_pillarair"], category = ACMD_GAME, low_priority )]
unsafe fn ness_pk_fire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 100, 50, 0, 6.5, 0.0, 3.1, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 367, 100, 50, 0, 4.5, 0.0, 9.6, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 115, 70, 0, 70, 6.5, 0.0, 3.1, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 115, 70, 0, 70, 4.5, 0.0, 9.6, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
    }
}
#[acmd_script( agent = "ness", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn ness_dtilt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 6.5, 65, 25, 0, 75, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("toel"), 9.5, 80, 25, 0, 75, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script( agent = "ness", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn ness_dtilt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 6, 1, -2.5, 0, -25, 0, 0.8, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("ness_psi_atk"), Hash40::new("ness_psi_atk"), Hash40::new("toel"), 0, 0, 0, 0, 180, 0, 1.25, true, *EF_FLIP_YZ);
    }
}
#[acmd_script( agent = "ness", script = "sound_attacklw3", category = ACMD_SOUND, low_priority )]
unsafe fn ness_dtilt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ness_rnd_attack"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ness_pk_m"));
    }
}


pub fn install() {
	smashline::install_acmd_scripts!(
        ness_pk_fire,
        ness_dtilt, ness_dtilt_eff, ness_dtilt_snd
    );
}
