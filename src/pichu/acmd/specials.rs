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
    agent = "pichu",
    scripts =  ["game_speciallw", "game_specialairlw", "effect_speciallw", "sound_speciallw", "effect_specialairlw", "sound_specialairlw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn pichu_downb_default(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "pichu",
    scripts =  ["game_speciallwhit", "game_specialairlwhit"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn pichu_downb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
		macros::FT_ADD_DAMAGE(agent, 0.4);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 70, 80, 0, 60, 7.0, 0.0, 6.0, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}	
#[acmd_script(
    agent = "pichu",
    scripts =  ["effect_speciallwhit", "effect_specialairlwhit"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn pichu_downb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit2"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 0.73, true);
        macros::FLASH(agent, 0, 0, 0, 0);
        macros::BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 2, 0, 0, 0, 0);
        macros::BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 2, 0, 0, 0, 0);
        macros::BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_NORMAL(agent);
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_kaminari_hit2"), false, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek"), false, true);
    }
}	
#[acmd_script(
    agent = "pichu",
    scripts =  ["sound_speciallwhit", "sound_specialairlwhit"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn pichu_downb_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
	if macros::is_excute(agent) {
		macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_attack"));
		macros::PLAY_SE(agent, Hash40::new("se_pichu_attackair_b01"));
	};
}	
#[acmd_script(
    agent = "pichu",
    scripts =  ["expression_speciallwhit", "expression_specialairlwhit"],
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn pichu_downb_expr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
}	

pub fn install() {
    smashline::install_acmd_scripts!(
		pichu_downb_default,
        pichu_downb, pichu_downb_eff, pichu_downb_snd, pichu_downb_expr
    );
}