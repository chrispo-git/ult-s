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

static mut SIDEB : [bool; 8] = [false; 8];
static mut SPIN : [bool; 8] = [false; 8];
static mut SPIN_EFF : [i32; 8] = [0; 8];
static mut SPIN1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 0.0 };
static mut SPIN2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.2, z: 0.0 };
static mut SPIN3 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.15, z: 0.0 };
static mut SPIN4 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.3, z: 0.0 };
static mut SPIN5 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.1, z: 0.0 };
static mut STAR1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 0.0 };
static mut STAR2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 9.0 };
static mut STAR3 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: -9.0 };
static mut STAR4 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 4.5 };
static mut STAR5 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: -4.5 };
static mut NOSPIN :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

#[acmd_script( agent = "mario", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn mario_bair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 30.5, 361, 106, 0, 12, 4.5, 4.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 30.5, 361, 106, 0, 12, 5.9, 1.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 7.0, 361, 90, 0, 7, 4.5, 4.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 7.0, 361, 90, 0, 7, 5.3, 1.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
#[acmd_script(
    agent = "mario",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mario_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 5.0);
		for _ in 0..5 {
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 367, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 367, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 5.0, /*Unk*/ false);
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}
			wait(fighter.lua_state_agent, 1.0);
		}
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 6.8, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 33.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}
#[acmd_script(
    agent = "mario",
    script =  "game_landingairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mario_dair_land(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 4.3, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 4.3, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ -4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 55, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 4.3, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 55, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 4.3, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ -4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}

#[acmd_script(
    agent = "mario",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mario_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 7.0, /*Angle*/ 90, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.2, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.4, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("toel"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.4, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 5.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "mario",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mario_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legr"), /*Damage*/ 7.0, /*Angle*/ 75, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 12, /*Size*/ 4.4, /*X*/ 1.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 7.0, /*Angle*/ 75, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 12, /*Size*/ 5.5, /*X*/ 3.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}
#[acmd_script(
    agent = "mario",
    script =  "game_specials",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mario_ground_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 2.0, /*Angle*/ 53, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KAMEHIT, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 53.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, true);
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 70.0);
		if macros::is_excute(fighter) {
			CancelModule::enable_cancel(fighter.module_accessor);
		}
}			
#[acmd_script(
    agent = "mario",
    script =  "effect_specialairs",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mario_air_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 12.0);
		for _ in 0..5 
 {
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("hat"), 0, 0.0, 0, 0, 0, 0, 0.75, true, *EF_FLIP_YZ);
				macros::LAST_EFFECT_SET_COLOR(fighter, 1.25, 1.0875, 0.3375);
				macros::LAST_EFFECT_SET_RATE(fighter, 2.2);
				EffectModule::set_alpha_last(fighter.module_accessor, 0.5);
				macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("hat"), 0, 0.0, 0, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
				macros::LAST_EFFECT_SET_COLOR(fighter, 1.25, 1.0875, 0.3375);
				macros::LAST_EFFECT_SET_RATE(fighter, 2.2);
			}
			wait(fighter.lua_state_agent, 4.0);
		}
	}
#[acmd_script(
    agent = "mario",
    script =  "effect_specials",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mario_ground_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		for _ in 0..7 
 {
			if macros::is_excute(fighter) {
				macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
				macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 0, 0, 0, 0, 270, 0.6, true, 0.6);
				macros::LAST_EFFECT_SET_RATE(fighter, 2);
			}
			wait(fighter.lua_state_agent, 6.0);
		}
	}		
#[acmd_script(
    agent = "mario",
    script =  "sound_specials",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn mario_ground_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_mario_rnd_attack"));
		}
		frame(fighter.lua_state_agent, 10.0);
		for _ in 0..4 
 {
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_mario_dash_start"));
			}
			wait(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_mario_dash_start"));
				macros::PLAY_SE(fighter, Hash40::new("se_mario_attackdash"));
			}
			wait(fighter.lua_state_agent, 6.0);
		}
	}		
#[acmd_script(
    agent = "mario",
    script =  "sound_specialairs",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn mario_air_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_mario_appeal_s01"));
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_mario_rnd_attack"));
		}
		wait(fighter.lua_state_agent, 3.0);
		for _ in 0..4 
 {
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_mario_appeal_s02"));
			}
			wait(fighter.lua_state_agent, 3.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_mario_appeal_s03"));
			}
			wait(fighter.lua_state_agent, 3.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_mario_appeal_s04"));
			}
			wait(fighter.lua_state_agent, 3.0);
		}
		frame(fighter.lua_state_agent, 51.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_mario_appeal_s06"));
		}
	}	
#[acmd_script(
    agent = "mario",
    script =  "game_specialairs",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mario_air_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.5);
		frame(fighter.lua_state_agent, 4.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
		frame(fighter.lua_state_agent, 9.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hat"), /*Damage*/ 8.0, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 150, /*BKB*/ 0, /*Size*/ 13.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ -7, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KAMEHIT, /*Type*/ *ATTACK_REGION_BODY);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 9.0, /*Unk*/ false);
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, false, 0);
		}
		frame(fighter.lua_state_agent, 10.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hat"), /*Damage*/ 8.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 150, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ -7, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KAMEHIT, /*Type*/ *ATTACK_REGION_BODY);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, false, 0);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hat"), /*Damage*/ 8.0, /*Angle*/ 75, /*KBG*/ 100, /*FKB*/ 150, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ -7, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KAMEHIT, /*Type*/ *ATTACK_REGION_BODY);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 20.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.575);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
		frame(fighter.lua_state_agent, 29.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hat"), /*Damage*/ 8.0, /*Angle*/ 75, /*KBG*/ 100, /*FKB*/ 150, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ -7, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KAMEHIT, /*Type*/ *ATTACK_REGION_BODY);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ -1.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 35.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "mario",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mario_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		frame(fighter.lua_state_agent, 11.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 12.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.025);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.6, /*X*/ 3.4, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderr"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.6, /*X*/ 3.4, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_PUNCH);
			if true{
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				if SPIN[ENTRY_ID] == false{
											macros::SET_SPEED_EX(fighter, 0.45, 1.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
									};
				SPIN[ENTRY_ID] = true;
			}
		}
		frame(fighter.lua_state_agent, 22.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 26.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.675);
		frame(fighter.lua_state_agent, 41.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}
#[acmd_script(
    agent = "mario",
    script =  "effect_attackairn",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mario_nair_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "mario",
    script =  "sound_attackairn",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn mario_nair_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"));
			macros::PLAY_SE(fighter, Hash40::new("vc_mario_attack05"));
			macros::PLAY_SE(fighter, Hash40::new("se_mario_special_s01"));
			macros::PLAY_SE(fighter, Hash40::new("se_mario_attackair_l01"));
		}
}		
#[acmd_script(
    agent = "mario",
    scripts =  ["sound_appealhir", "sound_appealhil"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn mario_utaunt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_item_mushroom"));
		}
		frame(fighter.lua_state_agent, 118.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_item_mushd"));
		}
}	
#[acmd_script(
    agent = "mario",
    scripts =  ["effect_appealhir", "effect_appealhil"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mario_utaunt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 11.0);
		for _ in 0..6 
 {
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 1.0, 1.0, 0.502, 0.667);
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 0.313, 0.313, 0.313, 0.35);
			}
			wait(fighter.lua_state_agent, 3.0);
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 0.313, 0.313, 0.313, 0.35);
			}
			wait(fighter.lua_state_agent, 3.0);
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 1.0, 1.0, 0.502, 0.667);
			}
			wait(fighter.lua_state_agent, 3.0);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::COL_NORMAL(fighter, );
		}
}	
#[acmd_script(
    agent = "mario",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn mario_fireball(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
		}
}	
pub(crate) fn check_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
			return true;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
			if ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_stick_y(boma) >= 0.7 {
				return true;
			};
		};
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
			return true;
		};
		return false;
	}
}

#[acmd_script( agent = "mario", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn mario_fair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.0, 361, 80, 0, 30, 3.0, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 14.0, 280, 78, 0, 32, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 14.0, 280, 78, 0, 32, 4.6, 3.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 10.0, 361, 80, 0, 20, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 361, 80, 0, 20, 4.6, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
#[acmd_script( agent = "mario", script = "effect_attackairf", category = ACMD_EFFECT, low_priority )]
unsafe fn mario_fair_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
	frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("handl"), 0.75, -1, 0, 0, -45, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("handl"), 0.75, -1, 0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 7, -1, -3, -11, -113, 1.1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
	frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_bullet_r"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
}

#[fighter_frame_callback]
pub fn mario_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if fighter_kind == *FIGHTER_KIND_MARIO {
			if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) == false {
				ArticleModule::remove_exist(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					if MotionModule::frame(boma) > 9.0 && MotionModule::frame(boma) < 73.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) == false {
						if check_jump(boma) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
						};
					};
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
					};
				} else {
					SIDEB[ENTRY_ID] = true;
					if MotionModule::frame(boma) < 23.0 {
						if MotionModule::frame(boma) < 2.0 {
							KineticModule::clear_speed_all(boma);
							let speed = smash::phx::Vector3f { x: 1.5, y: 1.5, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
						} else {
							KineticModule::clear_speed_all(boma);
							let speed = smash::phx::Vector3f { x: 0.5, y: 0.5, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
						};
					};
					if MotionModule::frame(boma) < 46.0 {
						StatusModule::set_keep_situation_air(boma, true);
					} else {
						StatusModule::set_keep_situation_air(boma, false);
					};
					if MotionModule::frame(boma) > 74.0 {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
						};
						CancelModule::enable_cancel(boma);
					} else {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
						};
					};
					if MotionModule::frame(boma) > 56.0 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0{
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					};
				};
			};
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
				SPIN[ENTRY_ID] = false;
				SIDEB[ENTRY_ID] = false;
			};
			if [hash40("attack_air_n")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) > 9.0 && MotionModule::frame(boma) < 22.0 && StopModule::is_stop(boma) == false {
				if SPIN_EFF[ENTRY_ID] == 0{
					let handbg1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN2, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let handbg2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN3, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let handbg3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN4, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let handbg4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN5, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let hand1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN1, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR1, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR2, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR3, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("handl"), &NOSPIN, &NOSPIN, 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star5: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("handr"), &NOSPIN, &NOSPIN, 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star6: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR4, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star7: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR5, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, hand1, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, hand1, 0.2);
					EffectModule::set_rgb(boma, handbg1, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, handbg1, 0.15);
					EffectModule::set_rgb(boma, handbg2, 0.045, 0.045, 2.05);
					EffectModule::set_alpha(boma, handbg2, 0.15);
					EffectModule::set_rgb(boma, handbg3, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, handbg3, 0.15);
					EffectModule::set_rgb(boma, handbg4, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, handbg4, 0.15);
					EffectModule::set_rgb(boma, star1, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star2, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star3, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star4, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star5, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star6, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star7, 1.65, 1.95, 1.85);
					EffectModule::set_alpha(boma, star1, 0.6);
					EffectModule::set_alpha(boma, star2, 0.6);
					EffectModule::set_alpha(boma, star3, 0.6);
					EffectModule::set_alpha(boma, star4, 0.6);
					EffectModule::set_alpha(boma, star5, 0.6);
					EffectModule::set_alpha(boma, star6, 0.6);
					EffectModule::set_alpha(boma, star7, 0.6);
				};
				SPIN_EFF[ENTRY_ID] += 1;
				if SPIN_EFF[ENTRY_ID] > 4 {
					SPIN_EFF[ENTRY_ID] = 0;
				};
			} else {
				SPIN_EFF[ENTRY_ID] = 0;
			};
		};
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
		mario_dtilt,
		mario_uair,
		mario_nair,
		mario_nair_effect,
		mario_nair_sound,
		mario_air_sideb,
		mario_air_sideb_eff,
		mario_air_sideb_snd,
		mario_ground_sideb,
		mario_ground_sideb_eff,
		mario_ground_sideb_snd,
		mario_fireball,
		mario_utaunt_eff,
		mario_utaunt_snd,
		mario_dair,
		mario_dair_land,
		mario_fair,
		mario_fair_eff,
		mario_bair
    );
    smashline::install_agent_frame_callbacks!(
        mario_frame
    );
}
