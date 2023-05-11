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

#[acmd_script(
    agent = "littlemac",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 8.0, /*Angle*/ 90, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.5, /*X*/ 4.4, /*Y*/ 0.2, /*Z*/ 0.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderr"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, true);
		}
}
#[acmd_script(
    agent = "littlemac",
    script =  "game_specialhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::SA_SET(fighter, *SITUATION_KIND_AIR);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 367, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 26.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 80, /*KBG*/ 190, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 28.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
		
#[acmd_script(
    agent = "littlemac",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 5.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 5.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderr"), /*Damage*/ 5.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 35.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}		
#[acmd_script(
    agent = "littlemac",
    script =  "effect_attackairf",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mac_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.3, true, *EF_FLIP_YZ);
		}
}		
#[acmd_script(
    agent = "littlemac",
    script =  "sound_attackairf",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn mac_fair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_littlemac_rnd_combo03"));
			macros::PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 2.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 4.0, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 2.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 2.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 2.0, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 2.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 2.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 2.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ -1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 2.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}
		
#[acmd_script(
    agent = "littlemac",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			macros::REVERSE_LR(fighter, );
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 11.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 11.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderr"), /*Damage*/ 11.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}		
#[acmd_script(
    agent = "littlemac",
    script =  "effect_attackairb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mac_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 2, 11.5, -2.5, 0, -220, -100, 0.9, true, 0.2);
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "sound_attackairb",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn mac_bair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_littlemac_rnd_combo03"));
			macros::PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 5.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 4.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 4.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, /*Frames*/ 11.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 270, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -1.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "effect_attackairlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mac_dair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, -4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_c"), Hash40::new("sys_attack_arc_c"), Hash40::new("trans"), 0, 6, 0, 90, 0, 0, 1.2, false, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, -1.5, 0, 0, 0, 0, 1.3, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_c"), true, true);
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "sound_attackairlw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn mac_dair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_littlemac_swing_l"));
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_littlemac_rnd_attack"));
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "game_attackhi4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 21.0, /*Angle*/ 86, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 16.0, /*Angle*/ 85, /*KBG*/ 96, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 16.0, /*Angle*/ 85, /*KBG*/ 96, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.0, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderr"), /*Damage*/ 16.0, /*Angle*/ 85, /*KBG*/ 96, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "game_attacks4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			damage!(fighter, /*MSC*/ *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, /*DamageThreshold*/ 0);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 18.0, /*Angle*/ 361, /*KBG*/ 87, /*FKB*/ 0, /*BKB*/ 29, /*Size*/ 4.0, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 18.0, /*Angle*/ 361, /*KBG*/ 87, /*FKB*/ 0, /*BKB*/ 29, /*Size*/ 3.0, /*X*/ -1.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-3.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			damage!(fighter, /*MSC*/ *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_NORMAL, /*DamageThreshold*/ 0);
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "game_attacks4hi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_fsmashhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 18.0, /*Angle*/ 78, /*KBG*/ 81, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.0, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 18.0, /*Angle*/ 85, /*KBG*/ 81, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ -1.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-2.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
#[acmd_script(
    agent = "littlemac",
    script =  "game_attacks4lw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_fsmashlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 23.0, /*Angle*/ 30, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.0, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 23.0, /*Angle*/ 30, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(0.0), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}			
		
}			
#[acmd_script(
    agent = "littlemac",
    script =  "game_attacklw4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 13.0, /*Angle*/ 30, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 4.5, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 13.0, /*Angle*/ 30, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.0, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 13.0, /*Angle*/ 30, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, true);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 13.0, /*Angle*/ 30, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 4.5, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 13.0, /*Angle*/ 30, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.0, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 13.0, /*Angle*/ 30, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 2.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ -1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, true);
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "game_specialndash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_neutralbdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 12.0, /*Angle*/ 0, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 12.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			WorkModule::set_float(fighter.module_accessor, 12.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 29.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 13.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 13.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "game_specialairndash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_neutralbairdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::SET_SPEED_EX(fighter, 2.5, -2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 300, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 29.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 29.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
#[acmd_script(
    agent = "littlemac",
    script =  "game_specialairndashturn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_neutralbairdashturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_TURN_DASH_START);
		}
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::SET_SPEED_EX(fighter, 2.5, -2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 300, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 29.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 29.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "effect_specialairndash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mac_neutralbairdash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight_line"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 4.0);
	if WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE) >= 0.625{
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight3"), Hash40::new("top"), 1.6, 0.2, 13.5, 0, -45, 45, 1, true);
        }
    } else {
		if WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE) >= 0.37{
			if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight2"), Hash40::new("top"), 1.6, 0.2, 13.5, 0, -45, 45, 1, true);
			}
		} else {
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight"), Hash40::new("top"), 1.6, 0.2, 13.5, 0, -45, 45, 1, true);
				EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
			}
		}
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("littlemac_straight_line"), false, false);
		EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
	}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "effect_specialairndashturn",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mac_neutralbairdashturn_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight_line"), Hash40::new("top"), 0, 6, 0, 0, 180, 45, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 4.0);
	if WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE) >= 0.625{
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight3"), Hash40::new("top"),  -1.6, -1.2, -13.5, 0, 135, -45, 1, true);
        }
    } else {
		if WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE) >= 0.37{
			if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight2"), Hash40::new("top"), -1.6, -1.2, -13.5, 0, 135, -45, 1, true);
			}
		} else {
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight"), Hash40::new("top"), -1.6, -1.2, -13.5, 0, 135, -45, 1, true);
				EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
			}
		}
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("littlemac_straight_line"), false, false);
		EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
	}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "game_specialndashturn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_neutralbdashturn(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_TURN_DASH_START);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 0, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 13.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 13.0, /*Unk*/ false);
			WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 27.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[fighter_frame( agent = FIGHTER_KIND_LITTLEMAC )]
fn mac_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if [*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH_TURN].contains(&status_kind) {
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && frame < 19.0 && frame > 13.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					let cat1 = ControlModule::get_command_flag_cat(boma, 0);
					if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT, true);
					};
			};
			if StatusModule::is_situation_changed(boma) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
			};
		};
		if [*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START].contains(&status_kind) {
			if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
				if status_duration(boma) > 12 && status_duration(boma) < 32 {
					let cat1 = ControlModule::get_command_flag_cat(boma, 0);
					if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH, true);
					};
				};
			};
		};
    }
}
#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_mac_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_LITTLEMAC {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if [*FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_DASH, *FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_DASH_TURN].contains(&status_kind) {
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && frame < 19.0 && frame > 13.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						let cat1 = ControlModule::get_command_flag_cat(boma, 0);
						if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
						};
				};
				if StatusModule::is_situation_changed(boma) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				};
			};
			if [*FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_START].contains(&status_kind) {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					if status_duration(boma) > 12 && status_duration(boma) < 32 {
						let cat1 = ControlModule::get_command_flag_cat(boma, 0);
						if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_DASH, true);
						};
					};
				};
			};
		};
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
		mac_dtilt,
		mac_fair,
		mac_fair_eff,
		mac_fair_snd,
		mac_bair,
		mac_bair_eff,
		mac_bair_snd,
		mac_uair,
		mac_fsmash,
		mac_fsmashhi,
		mac_fsmashlw,
		mac_usmash,
		mac_nair,
		mac_dsmash,
		mac_neutralbdash,
		mac_neutralbairdash_eff,
		mac_neutralbdashturn,
		mac_neutralbairdashturn_eff,
		mac_neutralbairdash,
		mac_dair,
		mac_dair_eff,
		mac_dair_snd
    );
    smashline::install_agent_frames!(mac_frame, kirby_mac_frame);
}
