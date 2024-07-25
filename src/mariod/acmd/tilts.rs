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
	Agent::new("mariod")
    .acmd("game_attackhi3", doc_utilt, Priority::Low)    
    .acmd("effect_attackhi3", doc_utilt_eff, Priority::Low)    
    .acmd("game_attacks3", doc_ftilt, Priority::Low)    
    .acmd("game_attacks3hi", doc_ftilt, Priority::Low)    
    .acmd("game_attacks3lw", doc_ftilt, Priority::Low)    
    .acmd("game_attacklw3", doc_dtilt, Priority::Low)    
    .acmd("effect_attacklw3", doc_dtilt_eff, Priority::Low)    
    .acmd("effect_attacks3", doc_ftilt_s, Priority::Low)    
    .acmd("effect_attacks3hi", doc_ftilt_hi, Priority::Low)    
    .acmd("sound_attacks3", doc_ftilt_snd, Priority::Low)    
    .acmd("sound_attacks3hi", doc_ftilt_snd, Priority::Low)    
    .acmd("sound_attacks3lw", doc_ftilt_snd, Priority::Low)    
    .acmd("sound_attacklw3", doc_dtilt_snd, Priority::Low)    
    .acmd("effect_attacks3lw", doc_ftilt_lw, Priority::Low)    
    .install();
}

unsafe extern "C" fn doc_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			//macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 6.3, /*Angle*/ 96, /*KBG*/ 130, /*FKB*/ 0, /*BKB*/ 28, /*Size*/ 3.5, /*X*/ -0.5, /*Y*/ -0.8, /*Z*/ 0.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 5.5, /*Angle*/ 96, /*KBG*/ 103, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 5.5, /*Angle*/ 96, /*KBG*/ 103, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 5.5, /*Angle*/ 96, /*KBG*/ 103, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 5.5, /*Angle*/ 96, /*KBG*/ 103, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("bust"), /*Damage*/ 5.5, /*Angle*/ 96, /*KBG*/ 103, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 3.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 3.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 3.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 3.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 4, /*Frames*/ 3.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			AttackModule::clear(fighter.module_accessor, 0, false);
			AttackModule::clear(fighter.module_accessor, 2, false);
			AttackModule::clear(fighter.module_accessor, 4, false);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
unsafe extern "C" fn doc_utilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, -2, -5.5, -90, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
			macros::EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, -2, 5.5, -90, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 12, 0, 4, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
			macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 29.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
}
unsafe extern "C" fn doc_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.8, /*X*/ 4.6, /*Y*/ 0.0, /*Z*/ 0.3, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legr"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.0, /*X*/ 1.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 2.0, /*X*/ 1.1, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		}
}
unsafe extern "C" fn doc_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 10.0, /*Angle*/ 90, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 3.5, /*X*/ 4.6, /*Y*/ 0.0, /*Z*/ 0.3, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.4, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 6.0, /*Angle*/ 70, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.2, /*X*/ 1.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.4, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 6.0, /*Angle*/ 70, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.2, /*X*/ 1.1, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.4, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 3.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 3.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 5.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
unsafe extern "C" fn doc_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), -2, 3, 4, -1, 12, -172, 0.95, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("kneel"), 4.6, 0, 0.3, 0, 0, 0, 0.461, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
		}
}
unsafe extern "C" fn doc_ftilt_s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 5, 2, 10, -39, 154, 0.95, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("kneer"), 4.6, 0, 0.3, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
		}
}
unsafe extern "C" fn doc_ftilt_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 7, 0.5, 30, -60, 135, 0.95, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("kneer"), 4.6, 0, 0.3, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
		}
}
unsafe extern "C" fn doc_ftilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"));
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_mariod_rnd_attack"));
			macros::PLAY_SE(fighter, Hash40::new("se_mariod_smash_s01"));
		}
}
unsafe extern "C" fn doc_dtilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"));
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_mariod_rnd_attack"));
			macros::PLAY_SE(fighter, Hash40::new("se_mariod_smash_s01"));
		}
}
unsafe extern "C" fn doc_ftilt_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), -2, 3, 2, 5, -90, 170, 0.95, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("kneer"), 4.6, 0, 0.3, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
		}
}	