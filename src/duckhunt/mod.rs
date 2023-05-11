use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;

#[acmd_script(
    agent = "duckhunt",
    script =  "game_attacks3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dh_ftilts(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("duckhead"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 3.5, /*X*/ 2.5, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ Some(-4.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
		}
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "duckhunt",
    script =  "game_attacks3hi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dh_ftilthi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("duckhead"), /*Damage*/ 14.0, /*Angle*/ 80, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 3.5, /*X*/ 2.5, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ Some(-4.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
		}
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "duckhunt",
    script =  "game_attacks3lw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dh_ftiltlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("duckhead"), /*Damage*/ 8.0, /*Angle*/ 25, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 2.5, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ Some(-4.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
		}
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "duckhunt",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dh_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 93, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 93, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		wait(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
		}
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "duckhunt",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dh_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.74);
}	
#[acmd_script(
    agent = "duckhunt",
    script =  "game_throwhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dh_uthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 4.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		}
}
#[acmd_script(
    agent = "duckhunt",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dh_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 38.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 44.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}		
#[acmd_script(
    agent = "duckhunt",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dh_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(-2.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(-2.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_punch"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_HEAD);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 21.5, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.5), /*Z2*/ Some(-5.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(14.5), /*Z2*/ Some(-2.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_punch"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_HEAD);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 91, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 22.5, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(22.5), /*Z2*/ Some(3.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_HEAD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 91, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(14.5), /*Z2*/ Some(-2.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_punch"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_HEAD);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}		
pub fn install() {
    smashline::install_acmd_scripts!(
		dh_ftilthi,
		dh_ftilts,
		dh_ftiltlw,
		dh_dtilt,
		dh_utilt,
		dh_uthrow,
		dh_nair,
		dh_uair
    );
}
