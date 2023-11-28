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

#[acmd_script(
    agent = "packun_poisonbreath",
    script =  "game_explode",
    category = ACMD_GAME,
	low_priority)]
unsafe fn poison_explosion(fighter: &mut L2CAgentBase) {
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

#[acmd_script(
    agent = "packun_poisonbreath",
    script =  "effect_explode",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn poison_explosion_eff(fighter: &mut L2CAgentBase) {
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
#[acmd_script(
    agent = "packun_poisonbreath",
    script =  "sound_explode",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn poison_explosion_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
		poison_explosion, poison_explosion_eff, poison_explosion_snd
    );
}