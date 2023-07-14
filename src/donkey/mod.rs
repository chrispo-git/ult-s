use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::lib::L2CValue;
use smash::app::sv_animcmd::*;
use crate::util::*;
use smash::phx::Vector3f;

static mut IS_DK_START_ITEM_CHUCK: [bool; 8] = [false; 8];
static mut IS_DK_UPB_BARREL: [bool; 8] = [false; 8];
static mut UPB_TIMER: [i32; 8] = [0; 8];
static mut UPB_ANGLE_X: [f32; 8] = [0.0; 8];
static mut UPB_ANGLE_Y: [f32; 8] = [0.0; 8];

const UPB_SPEED: f32 = 2.5;
static mut UPB_30_X: f32 = 0.0;
static mut UPB_30_Y: f32 = 0.0;

#[acmd_script(
    agent = "donkey",
    scripts =  ["game_attacks3", "game_attacks3hi", "game_attacks3lw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dk_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.91);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.5, /*X*/ 10.0, /*Y*/ 0.0, /*Z*/ -0.4, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ 1.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ -0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 10.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
		}
}	

#[acmd_script( agent = "donkey", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn dk_nair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 361, 80, 0, 20, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 361, 80, 0, 20, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 12.0, 361, 80, 0, 20, 5.7, 1.6, 3.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.0, 361, 100, 0, 10, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 9.0, 361, 100, 0, 10, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 9.0, 361, 100, 0, 10, 5.7, 1.6, 3.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
#[acmd_script( agent = "donkey", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn dk_uair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 13.0, 90, 90, 0, 45, 7.5, 2.5, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(
    agent = "donkey",
    scripts =  ["game_attackairf"],
    category = ACMD_GAME,
    low_priority)]
unsafe fn dk_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 8.0, /*Angle*/ 90, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 7.5, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 9.0, /*Angle*/ 90, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 6.5, /*X*/ 2.2, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        frame(fighter.lua_state_agent, 27.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script(
    agent = "donkey",
    scripts =  ["effect_attackairf"],
    category = ACMD_EFFECT, 
	low_priority )]
unsafe fn dk_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_attack_arc"), Hash40::new("donkey_attack_arc"), Hash40::new("top"), 1, 16, 4, -2, 30, 90, 1.7, true, *EF_FLIP_YZ);
		}
}

#[acmd_script(
    agent = "donkey",
    scripts =  ["game_attack11"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dk_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 35, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 4.0, /*Angle*/ 180, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
		}
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["game_attack12"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dk_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
		}
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.0, /*X*/ 1.0, /*Y*/ 1.5, /*Z*/ -2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script( agent = "donkey", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn dk_dtilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
		macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
		macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 6.0, 361, 80, 0, 20, 4.4, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 6.0, 361, 80, 0, 20, 4.4, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 8.0, 80, 100, 0, 20, 5.5, 8.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.5, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.5, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "donkey", script = "game_throwhi", category = ACMD_GAME, low_priority )]
unsafe fn dk_uthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 71, 52, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 21, 60);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 6.0, 75, 55, 0, 80, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 6.0, 75, 55, 0, 80, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("handr"), 6.0, 75, 55, 0, 80, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
	frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
	frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
    }
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, false);
    }
}

#[acmd_script( agent = "donkey", script = "effect_throwhi", category = ACMD_EFFECT, low_priority )]
unsafe fn dk_uthrow_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["game_specialairlw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dk_air_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 36, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 28.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 6.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.0, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ -2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["game_specialairs", "game_specials"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dk_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
        IS_DK_START_ITEM_CHUCK[ENTRY_ID] = true;
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        ItemModule::throw_item(fighter.module_accessor, 22.5, 4.0, 1.0, 0, true, 0.0);
    } 
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        IS_DK_START_ITEM_CHUCK[ENTRY_ID] = false;
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["game_specialairhi", "game_specialhi"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dk_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["effect_specialairhi", "effect_specialhi"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dk_upb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 3.7, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["game_specialhishoot"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dk_upb_shoot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ItemModule::remove_item(boma, 0);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 11.0, /*Angle*/ 74, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 10.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["effect_specialhishoot"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dk_upb_shoot_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_box02_break"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["effect_specialairs", "effect_specials"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dk_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1356a16dd8), Hash40::new("top"), 0, 12, -3, 0, -75, -85, 0.8, true);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
		}
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["sound_specialairs", "sound_specials"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dk_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
		frame(fighter.lua_state_agent, 29.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
			macros::PLAY_SE(fighter, Hash40::new("se_donkey_attackhard_s01"));
		}		
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["expression_specialairs", "expression_specials"],
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dk_sideb_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
}
#[fighter_frame( agent = FIGHTER_KIND_DONKEY )]
fn dk_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        UPB_30_X = 30.0_f32.sin() * UPB_SPEED;
        UPB_30_Y = 0.89 * UPB_SPEED;
		if IS_DK_START_ITEM_CHUCK[ENTRY_ID] == true {
			if ![*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
				ItemModule::throw_item(fighter.module_accessor, 0.0, 0.0, 1.0, 0, true, 0.0);
				IS_DK_START_ITEM_CHUCK[ENTRY_ID] = false;
			};
		};
        if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                if frame >= 134.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind), 33.0, 1.0, false, 0.0, false, false);
                }
            }
            if frame > 33.0 {
                MotionModule::set_rate(boma, 1.5);
            }
        }
        if [hash40("special_air_hi"), hash40("special_hi")].contains(&motion_kind) {
            StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
            StatusModule::set_keep_situation_air(boma, true);
            macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            IS_DK_UPB_BARREL[ENTRY_ID] = true;
            UPB_TIMER[ENTRY_ID] += 1;
            //This sucks! Don't do it :)
            if frame < 2.0 {
                UPB_ANGLE_X[ENTRY_ID] = 0.0;
                UPB_ANGLE_Y[ENTRY_ID] = UPB_SPEED;
            }
            if frame < 30.0 {
                UPB_ANGLE_Y[ENTRY_ID] += (UPB_30_Y - UPB_SPEED)/30.0;
                UPB_ANGLE_X[ENTRY_ID] += (1.0/30.0 * (UPB_30_X));
            } else if frame < 90.0 {
                if frame < 60.0 {
                    UPB_ANGLE_Y[ENTRY_ID] += (UPB_SPEED - UPB_30_Y)/30.0;
                } else {
                    UPB_ANGLE_Y[ENTRY_ID] += (UPB_30_Y - UPB_SPEED)/30.0;
                }
                UPB_ANGLE_X[ENTRY_ID] -= (1.0/30.0 * (UPB_30_X));
            } else {
                UPB_ANGLE_Y[ENTRY_ID] += (UPB_SPEED - UPB_30_Y)/30.0;
                UPB_ANGLE_X[ENTRY_ID] += (1.0/30.0 * (UPB_30_X));
            }
            println!("X:{}, Y:{}", UPB_ANGLE_X[ENTRY_ID], UPB_ANGLE_Y[ENTRY_ID]);
            if UPB_TIMER[ENTRY_ID] > 240 || (UPB_TIMER[ENTRY_ID] > 9 && ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)){
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_shoot"), 0.0, 1.0, false, 0.0, false, false);
            }
        } else {
            UPB_TIMER[ENTRY_ID] = 0;
            if IS_DK_UPB_BARREL[ENTRY_ID] {
                IS_DK_UPB_BARREL[ENTRY_ID] = false;
                ItemModule::remove_item(boma, 0);
            }
        }
        if [hash40("special_hi_shoot")].contains(&motion_kind) {
            MotionModule::set_rate(boma, 2.0);
            macros::SET_SPEED_EX(fighter, UPB_ANGLE_X[ENTRY_ID]*-0.45, UPB_ANGLE_Y[ENTRY_ID], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        };
	}
}	

pub fn install() {
	smashline::install_acmd_scripts!(
		dk_ftilt,
		dk_dtilt, 
		dk_fair,
		dk_nair,
		dk_uthrow,
        dk_uthrow_eff,
		dk_uair,
		dk_jab1, 
		dk_jab2, 
		dk_air_downb, 
		dk_fair_eff, 
		dk_sideb, 
		dk_sideb_eff,
		dk_sideb_snd, 
		dk_sideb_expr,
        dk_upb, dk_upb_eff, dk_upb_shoot, dk_upb_shoot_eff
	);
	smashline::install_agent_frames!(dk_frame);
}