use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
#[acmd_script(
    agent = "buddy",
    scripts =  ["game_specialnfire", "game_specialairnfire"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn banjo_egg(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		wait(fighter.lua_state_agent, 6.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_GENERATE_BULLET);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_ENABLE_SHOOT);
		}
}		
#[acmd_script(
    agent = "buddy",
    script =  "game_attack13",
    category = ACMD_GAME,
	low_priority)]
unsafe fn banjo_jab3(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.8, /*Angle*/ 74, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 72, /*Size*/ 2.6, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 4.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.8, /*Angle*/ 74, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 72, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 9.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.8, /*Angle*/ 74, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 72, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 14.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 3.8, /*Angle*/ 74, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 72, /*Size*/ 3.6, /*X*/ 4.2, /*Y*/ 0.0, /*Z*/ -0.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 10.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 10.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 10.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.7826086956521739);
		}
}
#[acmd_script(
    agent = "buddy",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn banjo_utilt(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::HIT_NO(fighter, 12, *HIT_STATUS_NORMAL);
			macros::HIT_NO(fighter, 13, *HIT_STATUS_NORMAL);
			macros::HIT_NO(fighter, 14, *HIT_STATUS_NORMAL);
			macros::HIT_NO(fighter, 15, *HIT_STATUS_NORMAL);
			macros::HIT_NO(fighter, 16, *HIT_STATUS_NORMAL);
			macros::HIT_NO(fighter, 17, *HIT_STATUS_NORMAL);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legr"), /*Damage*/ 10.0, /*Angle*/ 86, /*KBG*/ 123, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 4.4, /*X*/ 1.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 10.0, /*Angle*/ 86, /*KBG*/ 123, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 4.6, /*X*/ 2.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 10.0, /*Angle*/ 86, /*KBG*/ 123, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 4.4, /*X*/ 2.6, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 50, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 4.4, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(7.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
			macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
			macros::HIT_NO(fighter, 14, *HIT_STATUS_OFF);
			macros::HIT_NO(fighter, 15, *HIT_STATUS_OFF);
			macros::HIT_NO(fighter, 16, *HIT_STATUS_OFF);
			macros::HIT_NO(fighter, 17, *HIT_STATUS_OFF);
		}
}	
#[acmd_script(
    agent = "buddy",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn banjo_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.25);
		wait(fighter.lua_state_agent, 8.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 11.0);
		for _ in 0..7 {
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 48, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 4.2, /*Z*/ -4.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 335, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 14.6, /*Z*/ -4.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 220, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 14.6, /*Z*/ 5.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
				macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 140, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 14.6, /*Z*/ 5.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
				macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 130, /*KBG*/ 100, /*FKB*/ 75, /*BKB*/ 0, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 4.2, /*Z*/ 5.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
			}
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}
			wait(fighter.lua_state_agent, 1.0);
		}
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.2, /*Angle*/ 48, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 84, /*Size*/ 9.8, /*X*/ 0.0, /*Y*/ 8.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 39.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}	
#[acmd_script(
    agent = "buddy",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn banjo_uair(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 100, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.4, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 130, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 11.8, /*Z*/ 9.4, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 50, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 11.8, /*Z*/ -9.4, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 100, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.4, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 178, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 18.6, /*Z*/ 8.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 2, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 18.6, /*Z*/ -8.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 76, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 13.8, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_BUDDY_WING, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 76, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_BUDDY_WING, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::set_size(fighter.module_accessor, /*ID*/ 0, /*Size*/ 5.2);
			AttackModule::set_size(fighter.module_accessor, /*ID*/ 1, /*Size*/ 5.8);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}
#[acmd_script(
    agent = "buddy",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn banjo_da(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::SET_SPEED_EX(fighter, 0.6, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 5.2, /*X*/ 0.0, /*Y*/ 5.8, /*Z*/ 3.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 42, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 5.8, /*Z*/ 3.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		wait(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
#[acmd_script(
    agent = "buddy",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn banjo_bair(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.6, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 4.4, /*X*/ 0.0, /*Y*/ 4.4, /*Z*/ -5.6, /*X2*/ Some(0.0), /*Y2*/ Some(11.4), /*Z2*/ Some(-5.6), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("k_bust"), /*Damage*/ 1.6, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 1.6, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(8.6), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.6, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 4.4, /*X*/ 0.0, /*Y*/ 4.4, /*Z*/ -5.6, /*X2*/ Some(0.0), /*Y2*/ Some(11.4), /*Z2*/ Some(-5.6), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("k_bust"), /*Damage*/ 1.6, /*Angle*/ 110, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ -0.2, /*Y*/ 2.0, /*Z*/ 0.0, /*X2*/ Some(9.4), /*Y2*/ Some(2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("k_bust"), /*Damage*/ 1.6, /*Angle*/ 180, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ -0.2, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(9.4), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.6, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 4.4, /*X*/ 0.0, /*Y*/ 4.4, /*Z*/ -5.6, /*X2*/ Some(0.0), /*Y2*/ Some(11.4), /*Z2*/ Some(-5.6), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("k_bust"), /*Damage*/ 1.6, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 1.6, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(9.4), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.6, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 4.4, /*X*/ 0.0, /*Y*/ 4.4, /*Z*/ -5.6, /*X2*/ Some(0.0), /*Y2*/ Some(11.4), /*Z2*/ Some(-5.6), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("k_bust"), /*Damage*/ 1.6, /*Angle*/ 190, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 1.6, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(9.4), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.8, /*Angle*/ 37, /*KBG*/ 136, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 4.4, /*Z*/ -5.6, /*X2*/ Some(0.0), /*Y2*/ Some(11.4), /*Z2*/ Some(-5.6), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("k_bust"), /*Damage*/ 4.8, /*Angle*/ 37, /*KBG*/ 136, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 5.8, /*X*/ 1.6, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(8.6), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 37.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}		
#[acmd_script(
    agent = "buddy",
    script =  "game_throwf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn banjo_fthrow(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 5.4, /*Angle*/ 85, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 68, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 4.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("k_footr"), /*Damage*/ 3.0, /*Angle*/ 64, /*KBG*/ 34, /*FKB*/ 0, /*BKB*/ 82, /*Size*/ 4.6, /*X*/ 2.8, /*Y*/ 2.8, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "buddy",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn banjo_dair(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			macros::SET_SPEED_EX(fighter, 0, 0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			JostleModule::set_status(fighter.module_accessor, false);
			KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			macros::SET_SPEED_EX(fighter, 0, -3.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, true);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 68, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.2, /*X*/ 0.0, /*Y*/ -0.8, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(1.8), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 270, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ -0.8, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(1.8), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_HEAD);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 68, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.2, /*X*/ 0.0, /*Y*/ -0.8, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(1.8), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_HEAD);
		}
		frame(fighter.lua_state_agent, 46.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 50.0);
		if macros::is_excute(fighter) {
			KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 69.0);
		if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
		}
}		
#[fighter_frame( agent = FIGHTER_KIND_BUDDY )]
fn buddy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		if motion_kind == hash40("attack_air_lw") {
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && frame < 50.0{
				KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
				MotionModule::set_frame_sync_anim_cmd(boma, 51.0, true, true, false);
			};
		};
		if status_kind == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL {
			if frame > 20.0 {
				CancelModule::enable_cancel(boma);
			};
			if frame == 5.0 {
				if ControlModule::get_stick_x(boma)*PostureModule::lr(boma) < -0.6 {
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
					KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
				};
			};
		};
    }
}	
pub fn install() {
    smashline::install_acmd_scripts!(
		banjo_jab3,
		banjo_utilt,
		banjo_uair,
		banjo_nair,
		banjo_egg,
		banjo_bair,
		banjo_da,
		banjo_fthrow,
		banjo_dair
    );
    smashline::install_agent_frames!(
        buddy_frame
    );
}
