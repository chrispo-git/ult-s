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
    ganon_fthrow, ganon_fthrow_eff, ganon_fthrow_sound,
    
    ganon_bthrow, ganon_bthrow_eff, ganon_bthrow_sound,
    
    ganon_dthrow, ganon_dthrow_eff, ganon_dthrow_sound
);    
}

#[acmd_script( agent = "ganon", script = "game_throwf", category = ACMD_GAME, low_priority )]
unsafe fn ganon_fthrow(agent: &mut L2CAgentBase) {
if macros::is_excute(agent) {
    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 35, 70, 0, 35, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
}
frame(agent.lua_state_agent, 25.0);
if macros::is_excute(agent) {
    macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
}
}

#[acmd_script( agent = "ganon", script = "effect_throwf", category = ACMD_EFFECT, low_priority )]
unsafe fn ganon_fthrow_eff(agent: &mut L2CAgentBase) {
/*frame(agent.lua_state_agent, 9.0);
if macros::is_excute(agent) {
    macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), -3, 17, 2.5, 0, -45, 50, 0.85, true);
}*/
frame(agent.lua_state_agent, 7.0);
if macros::is_excute(agent) {
    macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("throw"), 0, 0, 0, 65, 0, 12, 1.6, true);
}
frame(agent.lua_state_agent, 24.0);
if macros::is_excute(agent) {
    macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
}
}

#[acmd_script( agent = "ganon", script = "sound_throwf", category = ACMD_SOUND, low_priority )]
unsafe fn ganon_fthrow_sound(agent: &mut L2CAgentBase) {
frame(agent.lua_state_agent, 3.0);
if macros::is_excute(agent) {
    macros::PLAY_SE(agent, Hash40::new("vc_ganon_appeal_s01"));
}
frame(agent.lua_state_agent, 25.0);
if macros::is_excute(agent) {
    macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    /*macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));*/
}
}

#[acmd_script( agent = "ganon", script = "game_throwb", category = ACMD_GAME, low_priority )]
unsafe fn ganon_bthrow(agent: &mut L2CAgentBase) {
if macros::is_excute(agent) {
    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 43, 120, 0, 25, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
}
frame(agent.lua_state_agent, 11.0);
if macros::is_excute(agent) {
    macros::REVERSE_LR(agent);
}
frame(agent.lua_state_agent, 44.0);
if macros::is_excute(agent) {
    macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
}
}

#[acmd_script( agent = "ganon", script = "effect_throwb", category = ACMD_EFFECT, low_priority )]
unsafe fn ganon_bthrow_eff(agent: &mut L2CAgentBase) {
frame(agent.lua_state_agent, 16.0);
if macros::is_excute(agent) {
    macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
}
frame(agent.lua_state_agent, 17.0);
if macros::is_excute(agent) {
    macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_catch"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
    macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("throw"), 0, 0, 0, 65, 0, 12, 1.6, true);
    macros::LAST_EFFECT_SET_RATE(agent, 0.5);
}
frame(agent.lua_state_agent, 40.0);
if macros::is_excute(agent) {
    macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -6, 6, 8, 215, -15, 12, 1.2, true);
    macros::LAST_PARTICLE_SET_COLOR(agent, 0.8, 0.6, 3);
    macros::LAST_EFFECT_SET_RATE(agent, 0.5);
}
}

#[acmd_script( agent = "ganon", script = "sound_throwb", category = ACMD_SOUND, low_priority )]
unsafe fn ganon_bthrow_sound(agent: &mut L2CAgentBase) {
frame(agent.lua_state_agent, 7.0);
if macros::is_excute(agent) {
    macros::PLAY_SE(agent, Hash40::new("vc_ganon_appeal_s01"));
}
/*frame(agent.lua_state_agent, 16.0);
if macros::is_excute(agent) {
    macros::PLAY_STATUS(agent, Hash40::new("se_mewtwo_throw_b"));
}*/
frame(agent.lua_state_agent, 40.0);
if macros::is_excute(agent) {
    macros::PLAY_SE(agent, Hash40::new("vc_ganon_attack01"));
    macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    /*macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));*/
}
}

#[acmd_script( agent = "ganon", script = "game_throwlw", category = ACMD_GAME, low_priority )]
unsafe fn ganon_dthrow(agent: &mut L2CAgentBase) {
if macros::is_excute(agent) {
    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 70, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
}
frame(agent.lua_state_agent, 22.0);
if macros::is_excute(agent) {
    macros::CHECK_FINISH_CAMERA(agent, 11, 0);
}
frame(agent.lua_state_agent, 35.0);
if macros::is_excute(agent) {
    macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
}
}

#[acmd_script( agent = "ganon", script = "effect_throwlw", category = ACMD_EFFECT, low_priority )]
unsafe fn ganon_dthrow_eff(agent: &mut L2CAgentBase) {
frame(agent.lua_state_agent, 3.0);
if macros::is_excute(agent) {
    macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("throw"), 0, 0, 0, 65, 0, 12, 1.6, true);
    macros::LAST_PARTICLE_SET_COLOR(agent, 0.8, 0.6, 3.0);
    macros::LAST_EFFECT_SET_RATE(agent, 0.5);
}
frame(agent.lua_state_agent, 31.0);
if macros::is_excute(agent) {
    macros::LANDING_EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    macros::LANDING_EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), 12, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("throw"), 12, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("throw"), 12, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
}
}

#[acmd_script( agent = "ganon", script = "sound_throwlw", category = ACMD_SOUND, low_priority )]
unsafe fn ganon_dthrow_sound(agent: &mut L2CAgentBase) {
frame(agent.lua_state_agent, 14.0);
if macros::is_excute(agent) {
    macros::PLAY_SE(agent, Hash40::new("se_common_throw_03"));
    macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
}
/*frame(agent.lua_state_agent, 22.0);
if macros::is_excute(agent) {
    macros::PLAY_DOWN_SE(agent, Hash40::new("se_common_down_m_01"));
    macros::PLAY_SE(agent, Hash40::new("se_common_kick_hit_m"));
}*/
frame(agent.lua_state_agent, 35.0);
if macros::is_excute(agent) {
    macros::PLAY_SE(agent, Hash40::new("se_ganon_attackhard_h03"));
}
}