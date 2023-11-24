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
		kirby_dtilt,
		kirby_ftilt,
		kirby_ftilt_eff,
		kirby_ftilt_sound
    );
}
#[acmd_script(
    agent = "kirby",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("toer"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.7, /*X*/ 4.3, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("toer"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.7, /*X*/ -2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}

#[acmd_script(
    agent = "kirby",
    scripts =  ["game_attacks3", "game_attacks3hi", "game_attacks3lw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, 1.8);
		frame(fighter.lua_state_agent, 6.0);
		macros::FT_MOTION_RATE(fighter, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.4, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 18.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(5.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
		}
		wait(fighter.lua_state_agent, 5.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.538);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_attacks3", "sound_attacks3hi", "sound_attacks3lw"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_ftilt_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kirby_rnd_attack"));
			macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_s03"));
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_attacks3", "effect_attacks3hi", "effect_attacks3lw"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::BURN_COLOR(fighter, 2, 0.15, 0.02, 0.7);
			macros::BURN_COLOR_FRAME(fighter, 8, 2, 0.15, 0.02, 0);
			macros::EFFECT(fighter, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), 12, 4.5, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), 18, 4.5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), 6, 4.5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), 5, 4.5, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::BURN_COLOR_NORMAL(fighter, );
		}
}	