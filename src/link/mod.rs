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
use crate::util::*;

static mut NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut SHOOT :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 10.0, z: 20.0 };

#[acmd_script(
    agent = "link",
    script =  "effect_landingheavy",
    category = ACMD_EFFECT)]
unsafe fn link_landing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
		}
}		

#[acmd_script(
    agent = "link",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_jab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		wait(fighter.lua_state_agent, 6.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 1.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 1.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 14.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 180, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 1.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 18.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 1.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 18.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
		}
}
#[acmd_script(
    agent = "link",
    script =  "game_attack12",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(8.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 14.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(8.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 180, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 18.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(8.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 18.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(8.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
		}
}
#[acmd_script(
    agent = "link",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
		frame(fighter.lua_state_agent, 13.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 80, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 1.3, /*Z*/ 17.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 4.0, /*Unk*/ false);
			macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.05);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "link",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 11.0, /*Angle*/ 95, /*KBG*/ 103, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.4, /*X*/ 8.4, /*Y*/ 0.0, /*Z*/ -2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 11.0, /*Angle*/ 85, /*KBG*/ 111, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.3, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ -3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 11.0, /*Angle*/ 85, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 11.0, /*Angle*/ 85, /*KBG*/ 104, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 2.16, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "link",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 6.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 5.0, /*Angle*/ 73, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 5.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 36, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 5.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ -0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 5.0, /*Angle*/ 66, /*KBG*/ 100, /*FKB*/ 36, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ -0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 5.0, /*Angle*/ 66, /*KBG*/ 100, /*FKB*/ 36, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.5, /*X*/ 5.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ -0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 29.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
} 

#[acmd_script(
    agent = "link",
    scripts =  ["game_specialairnstart"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_arrow_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,smash::phx::Hash40::new("n_air_start"),false,0.0);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.722222);
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
		}
} 
#[acmd_script(
    agent = "link",
    scripts =  ["game_specialnstart"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_arrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,smash::phx::Hash40::new("n_start"),false,0.0);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.722222);
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
		}
} 

#[acmd_script( 
	agent = "link", 
	scripts = ["game_attacklw4"], 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn link_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW,smash::phx::Hash40::new("d_smash"),false,0.0);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 15.2, /*Angle*/ 32, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 5.5, /*X2*/ Some(0.0), /*Y2*/ Some(1.6), /*Z2*/ Some(5.5), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 15.2, /*Angle*/ 32, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -0.5, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(12.5), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 14.2, /*Angle*/ 32, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -7.5, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(19.5), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 38.0);
        if macros::is_excute(fighter) {
            //Before the FaF, do this
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK);
        }
}
#[acmd_script( 
	agent = "link", 
	scripts = ["effect_attacklw4"], 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn link_dsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("bow"), 2, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -0.9, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
		}
        frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 6, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
			macros::LAST_EFFECT_SET_ALPHA(fighter, 0.5);
		}
		frame(fighter.lua_state_agent, 56.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -0.9, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script( 
	agent = "link", 
	scripts = ["sound_attacklw4"], 
	category = ACMD_SOUND, 
	low_priority )]
unsafe fn link_dsmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_link_attack05"));
		}
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			macros::PLAY_LANDING_SE(fighter, Hash40::new("se_link_attackair_l01"));
		}
}
#[acmd_script(
    agent = "link",
    script =  "expression_attacklw4",
    category = ACMD_EXPRESSION,
    low_priority)]
unsafe fn link_dsmash_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
}    
#[acmd_script(
    agent = "link",
    script =  "expression_attacklw4charge",
    category = ACMD_EXPRESSION,
    low_priority)]
unsafe fn link_dsmash_charge_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
}  

#[acmd_script( 
	agent = "link", 
	scripts = ["game_attackairf"], 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn link_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW,smash::phx::Hash40::new("f_air"),false,0.0);
        }
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(8.5), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 13.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 2.3, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 22.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(28.5), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 10.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(20.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(8.5), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 1.8, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 22.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(28.5), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 6.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(20.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
        frame(fighter.lua_state_agent, 25.0);
        if macros::is_excute(fighter) {
            //Before the FaF, do this
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK);
        }
		frame(fighter.lua_state_agent, 39.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}
#[acmd_script( 
	agent = "link", 
	scripts = ["effect_attackairf"], 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn link_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 11.5, 11.0, 0, 0, 0, 0.65, true);
			macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
			macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, 12.5, 30.0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true);
		}
}
#[acmd_script( 
	agent = "link", 
	scripts = ["sound_attackairf"], 
	category = ACMD_SOUND, 
	low_priority )]
unsafe fn link_fair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_link_swing_l"));
		}
}
#[acmd_script(
    agent = "link",
    script =  "expression_attackairf",
    category = ACMD_EXPRESSION,
    low_priority)]
unsafe fn link_fair_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
}

#[acmd_script( 
	agent = "link", 
	scripts = ["game_attacks3"], 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn link_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW,smash::phx::Hash40::new("f_air"),false,0.0);
        }
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 39, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 6.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(8.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 39, /*Size*/ 2.3, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 19.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(25.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 39, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(17.5), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter) {
            //Before the FaF, do this
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK);
        }
}
#[acmd_script( 
	agent = "link", 
	scripts = ["effect_attacks3"], 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn link_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 8.0, 9.5, 0, 0, 0, 0.65, true);
			macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
			macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, 8.0, 26.0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true);
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6.5, 0.0, -1.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script( 
	agent = "link", 
	scripts = ["sound_attacks3"], 
	category = ACMD_SOUND, 
	low_priority )]
unsafe fn link_ftilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_link_swing_l"));
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_link_rnd_attack"));
		}
}
#[acmd_script(
    agent = "link",
    script =  "expression_attacks3",
    category = ACMD_EXPRESSION,
    low_priority)]
unsafe fn link_ftilt_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
}

#[acmd_script( 
	agent = "link", 
	scripts = ["game_attackairhi"], 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn link_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW,smash::phx::Hash40::new("u_air"),false,0.0);
        }
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 7.6, /*Angle*/ 80, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 7.6, /*Angle*/ 80, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 7.6, /*Angle*/ 80, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ 6.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            //Before the FaF, do this
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK);
        }
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}
#[acmd_script( 
	agent = "link", 
	scripts = ["effect_attackairhi"], 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn link_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), -1.5, 11.0, 0.5, 0, 90, 60, 1.5, true, *EF_FLIP_YZ, 0.7);
			macros::LAST_PARTICLE_SET_COLOR(fighter, 1.07/2.55, 0.69/2.55, 0.43/2.55);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
		}
}
#[acmd_script( 
	agent = "link", 
	scripts = ["sound_attackairhi"], 
	category = ACMD_SOUND, 
	low_priority )]
unsafe fn link_uair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_link_rnd_attack"));
			macros::PLAY_SE(fighter, Hash40::new("se_link_swing_l"));
		}
}
#[acmd_script(
    agent = "link",
    script =  "expression_attackairhi",
    category = ACMD_EXPRESSION,
    low_priority)]
unsafe fn link_uair_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
}

#[acmd_script(
    agent = "link",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.66);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 22, /*Size*/ 4.0, /*X*/ 3.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 22, /*Size*/ 4.0, /*X*/ -2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 22, /*Size*/ 2.5, /*X*/ 1.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 3.5, /*X*/ 3.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 3.5, /*X*/ -2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 2.0, /*X*/ 1.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
} 	
#[acmd_script(
    agent = "link",
    scripts =  ["game_speciallwblast", "game_specialairlwblast"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST {
					if macros::is_excute(fighter) {
				WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_BLAST);
			}
			} else {
					frame(fighter.lua_state_agent, 11.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 97, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 20.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 20.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
				macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.1);
				macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 0.1);
			}
			frame(fighter.lua_state_agent, 16.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}
			};
}		
#[acmd_script(
    agent = "link",
    scripts =  ["effect_speciallwblast", "effect_specialairlwblast"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn link_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST {
			if macros::is_excute(fighter) {
				macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_smash_flash"), false, true);
			}
			frame(fighter.lua_state_agent, 10.0);
			if macros::is_excute(fighter) {
				let eff: u32 = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40::new("link_swordbeam_hit"), smash::phx::Hash40::new("top"), &SHOOT, &NONE, 0.85, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(fighter.module_accessor, eff, 6.95, 1.5, 0.06);
				EffectModule::set_rate(fighter.module_accessor, eff, 0.65);
			};
	};
}	
#[acmd_script(
    agent = "link",
    scripts =  ["sound_speciallwblast", "sound_specialairlwblast"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn link_downb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST {
					frame(fighter.lua_state_agent, 10.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_link_rnd_attack"));
				macros::PLAY_STATUS(fighter, Hash40::new("se_link_smash_s01"));
				macros::PLAY_STATUS(fighter, Hash40::new("se_link_smash_s01"));
				macros::PLAY_STATUS(fighter, Hash40::new("se_link_smash_s01"));
			}
			};
}	
#[acmd_script(
    agent = "link_boomerang",
    script =  "game_fly",
    category = ACMD_GAME,
	low_priority)]
unsafe fn linkerang(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 70, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
			AttackModule::enable_safe_pos(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 65, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
		}
}

#[fighter_frame( agent = FIGHTER_KIND_LINK )]
fn link_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        //Make sure to put all statuses that use the weapon in here
        if ![*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) || [hash40("special_lw"), hash40("special_air_lw")].contains(&MotionModule::motion_kind(boma)) {
            ArticleModule::change_status_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK);
			let motion_kind = MotionModule::motion_kind(boma);
        };
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
		link_dtilt,
		link_nair,
		link_bair,
		link_fair,
		link_fair_expr,
		link_fair_snd,
		link_fair_eff,
		link_ftilt,
		link_ftilt_snd,
		link_ftilt_eff,
		link_ftilt_expr,
		link_uair,
		link_uair_snd,
		link_uair_eff,
		link_uair_expr,
		link_arrow_air,
		link_arrow,
		link_dsmash,
		link_dsmash_eff,
		link_dsmash_snd,
		link_dsmash_expr,
		link_dsmash_charge_expr,
		link_utilt,
		link_jab,
		link_jab2,
		linkerang,
		link_downb,
		link_downb_eff,
		link_downb_snd,
		link_landing
    );
	smashline::install_agent_frames!(link_frame);
}