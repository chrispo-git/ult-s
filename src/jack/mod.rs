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

//Joker Gun Cancel Constants 
const NONE : i32 = 100;
const ATTACK : i32 = 101;
const ATTACK_S3 : i32 = 104;
const ATTACK_HI3 : i32 = 105;
const ATTACK_LW3 : i32 = 106;
const ATTACK_S4 : i32 = 107;
const ATTACK_HI4 : i32 = 108;
const ATTACK_LW4 : i32 = 110;
const ATTACK_AIR_N : i32 = 111;
const ATTACK_AIR_F : i32 = 112;
const ATTACK_AIR_B : i32 = 113;
const ATTACK_AIR_HI : i32 = 114;
const ATTACK_AIR_LW : i32 = 115;

//Joker Gun Cancel 
static mut GUN_C: [i32; 8] = [100; 8];
static mut IS_ARSENE: [bool; 8] = [false; 8];
static mut X: [f32; 8] = [0.0; 8];
static mut Y: [f32; 8] = [0.0; 8];
static mut BATON_TYPE: [i32; 8] = [0; 8];
static BATON_MAX : i32 = 2;

#[acmd_script(
    agent = "jack",
    scripts =  ["game_attackairhi"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 0.75, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.0, /*X*/ 4.7, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.2, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 0.75, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.0, /*X*/ 4.7, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.2, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 3.0, /*Angle*/ 80, /*KBG*/ 155, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.5, /*X*/ 4.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(2.5), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 3.0, /*Angle*/ 80, /*KBG*/ 155, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ 4.3, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(1.5), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 80, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ 1.5, /*X2*/ Some(0.0), /*Y2*/ Some(24.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
			}
		}
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_attackairf"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legr"), /*Damage*/ 2.0, /*Angle*/ 367, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 3.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(3.2), /*Y2*/ Some(-0.5), /*Z2*/ Some(1.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("legr"), /*Damage*/ 2.0, /*Angle*/ 76, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 73, /*Size*/ 4.0, /*X*/ 3.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(3.2), /*Y2*/ Some(0.0), /*Z2*/ Some(1.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 2.0, /*Angle*/ 367, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 4.2, /*Y*/ -0.7, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 2.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 73, /*Size*/ 4.0, /*X*/ 4.2, /*Y*/ -0.7, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 367, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 76, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 73, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			AttackModule::clear(fighter.module_accessor, /*ID*/ 4, false);
			AttackModule::clear(fighter.module_accessor, /*ID*/ 5, false);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 3.5, /*X*/ 4.4, /*Y*/ -0.7, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 3.5, /*X*/ 3.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 34, /*KBG*/ 148, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.5), /*Z2*/ Some(7.5), /*Hitlag*/ 1.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 34, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.5), /*Z2*/ Some(7.5), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 34, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(17.0), /*Z2*/ Some(6.5), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
			}
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
	}		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_attackairb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 43, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 54, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 3.3, /*Z*/ -6.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.7), /*Z2*/ Some(-8.4), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 43, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 54, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 4.7, /*Z*/ -11.1, /*X2*/ Some(0.0), /*Y2*/ Some(11.1), /*Z2*/ Some(-14.2), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 43, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 54, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(-5.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 35, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 51, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ -12.5, /*X2*/ Some(0.0), /*Y2*/ Some(15.0), /*Z2*/ Some(-12.5), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 35, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 51, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.5), /*Z2*/ Some(-8.5), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 35, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.5), /*Z2*/ Some(-8.5), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
				macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 35, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ -12.5, /*X2*/ Some(0.0), /*Y2*/ Some(15.0), /*Z2*/ Some(-12.5), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
				macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 35, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.5, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(20.5), /*Z2*/ Some(-9.0), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
			}
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}		
		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_attacks4"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		wait(fighter.lua_state_agent, 8.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(12.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(12.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 15.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(15.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_NONE);
				macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.5), /*Z2*/ Some(10.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_NONE);
			}
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(12.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(12.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 15.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(15.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_NONE);
				macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.5), /*Z2*/ Some(10.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_NONE);
			}
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialn1_ex", "game_specialn1"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_gun(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			if GUN_C[ENTRY_ID] == ATTACK {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(30.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			} else if GUN_C[ENTRY_ID] == ATTACK_S4 {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 120, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(44.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(50.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			};
			GUN_C[ENTRY_ID] = NONE;
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 6.0, /*Unk*/ false);
			};
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
		}
		frame(fighter.lua_state_agent, 31.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
		}
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialairn1_ex", "game_specialairn1"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_gun_air(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
    	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			if GUN_C[ENTRY_ID] == ATTACK_AIR_HI {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 14.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			} else if GUN_C[ENTRY_ID] == ATTACK_AIR_F {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 50, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(30.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(50.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			};
			GUN_C[ENTRY_ID] = NONE;
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 6.0, /*Unk*/ false);
			};
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
		}
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
		}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
		}
		frame(fighter.lua_state_agent, 31.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
		}
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
}	
#[acmd_script(
    agent = "jack",
    scripts =  ["sound_specialn1_ex", "sound_specialairn1_ex"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn s_joker_gun(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_jack_special_n01"));
		}
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["effect_specialn1_ex", "effect_specialairn1_ex"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn e_joker_gun(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new_raw(0x16b6b8d02d), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["effect_specialnescapeb_ex", "effect_specialairnescapeb_ex"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn e_joker_gun_b(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
    	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new_raw(0x16b6b8d02d), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
}	
#[acmd_script(
    agent = "jack",
    scripts =  ["sound_specialnescapeb", "sound_specialairnescapeb", "sound_specialnescapeb_ex", "sound_specialairnescapeb_ex"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn s_joker_gun_b(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
   		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_jack_escape"));
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_jack_special_n01"));
		}
}	
		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialnescapeb_ex", "game_specialairnescapeb_ex","game_specialnescapeb", "game_specialairnescapeb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_gun_b(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, true);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			if GUN_C[ENTRY_ID] == ATTACK_S3 {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 68, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(30.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 50, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(50.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			};
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 6.0, /*Unk*/ false);
			};
			GUN_C[ENTRY_ID] = NONE;
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 39.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
		}
		frame(fighter.lua_state_agent, 42.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
}			
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialairnescapeb_ex", "game_specialairnescapeb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_gun_b_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, true);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			if GUN_C[ENTRY_ID] == ATTACK_AIR_B {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 50, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(50.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			};
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 6.0, /*Unk*/ false);
			};
			GUN_C[ENTRY_ID] = NONE;
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
		}
		frame(fighter.lua_state_agent, 36.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
		}
		frame(fighter.lua_state_agent, 37.0);
		frame(fighter.lua_state_agent, 39.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
		}
		frame(fighter.lua_state_agent, 42.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specials2"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_eiagon(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.4375);
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
}	
			
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialairs2"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_eiagon_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.4375);
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL);
		}
}
#[acmd_script(
    agent = "jack",
    scripts =  ["game_attackhi4"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			MotionModule::set_rate(fighter.module_accessor, 1.5);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			MotionModule::set_rate(fighter.module_accessor, 1.0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			MotionModule::set_rate(fighter.module_accessor, 1.5);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			MotionModule::set_rate(fighter.module_accessor, 1.0);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 12.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 12.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 2.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(6.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 12.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 12.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 2.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(6.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("knife"), /*Damage*/ 5.0, /*Angle*/ 83, /*KBG*/ 181, /*FKB*/ 0, /*BKB*/ 59, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
						macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 1, /*Bone*/ Hash40::new("armr"), /*Damage*/ 5.0, /*Angle*/ 83, /*KBG*/ 181, /*FKB*/ 0, /*BKB*/ 59, /*Size*/ 2.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
						macros::ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 83, /*KBG*/ 181, /*FKB*/ 0, /*BKB*/ 59, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(6.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
						macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.25);
						macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.25);
						macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.25);
						macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 3, /*ShieldstunMul*/ 1.25);
						macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 4, /*ShieldstunMul*/ 1.25);
						macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 5, /*ShieldstunMul*/ 1.25);
						macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
						macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
			}
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
			AttackModule::clear(fighter.module_accessor, /*ID*/ 5, false);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
				if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 28.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(9.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
						macros::ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 83, /*KBG*/ 181, /*FKB*/ 0, /*BKB*/ 59, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 28.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(9.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
				}
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
				if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 28.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
						macros::ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 83, /*KBG*/ 181, /*FKB*/ 0, /*BKB*/ 59, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 28.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
				}
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specials1", "game_specialairs1"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_sideb(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			BATON_TYPE[ENTRY_ID] += 1;
			if BATON_TYPE[ENTRY_ID] > BATON_MAX {
				BATON_TYPE[ENTRY_ID] = 0;
			}
		};
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			if BATON_TYPE[ENTRY_ID] == 0 {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA, false, 0);
				ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::phx::Hash40::new("special_s1"),false,0.0);
			} else if BATON_TYPE[ENTRY_ID] == 1 {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA, false, 0);
				ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::phx::Hash40::new("special_s2"),false,0.0);
			} else {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA, false, 0);
				ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::phx::Hash40::new("special_s3"),false,0.0);
			};
		}
		//ACMD for each one of the baton passes
		if BATON_TYPE[ENTRY_ID] == 0 { //Skull (Ryuji)
			frame(fighter.lua_state_agent, 15.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 270, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 55.0, /*Z*/ 28.0, /*X2*/ Some(0.0), /*Y2*/ Some(40.0), /*Z2*/ Some(28.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 40.0, /*Z*/ 28.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(28.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
				macros::ATTACK(fighter, /*ID*/ 	0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 28.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
			};
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			};
		} else if BATON_TYPE[ENTRY_ID] == 1 { // Panther (Ann)
			frame(fighter.lua_state_agent, 20.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
			};
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			};
		} else { //Mona (Morgana)
			frame(fighter.lua_state_agent, 15.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 11, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 0.6, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
			};
			frame(fighter.lua_state_agent, 30.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 110, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 0.6, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
			};
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			};
		}
		frame(fighter.lua_state_agent, 48.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
#[acmd_script(
    agent = "jack",
    scripts =  ["effect_specials1", "effect_specialairs1"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn joker_sideb_eff(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 2.0);
		if BATON_TYPE[ENTRY_ID] == 0 { //Skull (Ryuji)
			frame(fighter.lua_state_agent, 14.0);
			if macros::is_excute(fighter) {
				macros::EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 0, 28, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			};
			println!("");
		} else if BATON_TYPE[ENTRY_ID] == 1 { // Panther (Ann)
			frame(fighter.lua_state_agent, 19.0);
			if macros::is_excute(fighter) {
				macros::EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 8.0, 30.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
			};
		} else { //Mona (Morgana)
			frame(fighter.lua_state_agent, 14.0);
			for _ in 0..5 {
				if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 12.0, 30.0, 0, 0, 0, 1.2*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 7.5, 30.0, 0, 10, 0, 1.0*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 5.5, 30.0, 0, -18, 0, 0.9*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				};	
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 9.0, 30.0, 0, 25, 0, 1.1*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 10.2, 30.0, 0, -16, 0, 0.8*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 5.5, 30.0, 0, 3, 0, 1.3*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				};	
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 7.0, 30.0, 0, 11, 0, 1.0*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 10.5, 30.0, 0, 13, 0, 1.325*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 3.5, 30.0, 0, -5, 0, 1.15*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				};	
				wait(fighter.lua_state_agent, 1.0);
			};
			frame(fighter.lua_state_agent, 30.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 12.0, 30.0, 0, 0, 0, 1.2*1.5*0.7, true);
				macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 5.5, 30.0, 0, 10, 0, 1.0*1.5*0.7, true);
				macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 7.5, 30.0, 0, -18, 0, 0.9*1.5*0.7, true);
				macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
			};
		}
		frame(fighter.lua_state_agent, 47.0);
		if macros::is_excute(fighter) {
			if BATON_TYPE[ENTRY_ID] != 2 { //Not Mona
				macros::EFFECT(fighter, Hash40::new("jack_mona_smoke"), Hash40::new("top"), 0.0, 6.0, 15.0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, true);
			} else {
				macros::EFFECT(fighter, Hash40::new("jack_mona_smoke"), Hash40::new("top"), 0.0, 2.0, 15.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
			};
		}
}	
#[acmd_script(
    agent = "jack",
    scripts =  ["sound_specials1", "sound_specialairs1"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn joker_sideb_snd(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 3.0);
		let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
		if BATON_TYPE[ENTRY_ID] == 0 { //Skull (Ryuji)
			frame(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				if rand_val == 0 {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_s02")); // vc 2
				} else {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n03")); // vc 1
				};
			};
			frame(fighter.lua_state_agent, 14.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_smashswing_02"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_down_m_02"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_electric_hit_s"));
			};
			println!("");
		} else if BATON_TYPE[ENTRY_ID] == 1 { // Panther (Ann)
			frame(fighter.lua_state_agent, 8.0);
			if macros::is_excute(fighter) {
				if rand_val == 0 {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n02")); // vc 1
				} else {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_s01")); // vc 2
				};
			};
			frame(fighter.lua_state_agent, 19.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
			};
		} else { //Mona (Morgana)
			frame(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				if rand_val == 0 {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n01")); // vc 1
				} else {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n04")); // vc 2
				};
			};
			frame(fighter.lua_state_agent, 14.0);
			for _ in 0..5 {
				if macros::is_excute(fighter) {
					macros::PLAY_SE(fighter, Hash40::new("se_common_swing_01"));
				}
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
				}
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
				}
				wait(fighter.lua_state_agent, 1.0);
			};
			frame(fighter.lua_state_agent, 30.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_swing_08"));
			}
		}
}	

//KIRBY - MAKE SURE ANY CHANGES TO NEUTRLAB ARE ON BOTH
#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_joker_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		let motion_kind = MotionModule::motion_kind(boma);
		if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_JACK {
			if smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_END, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START, *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_JUMP, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON, *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_ITEM, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_BEETLE, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_DRIVER, *FIGHTER_STATUS_KIND_CAPTURE_NABBIT, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP, *FIGHTER_STATUS_KIND_CAPTURE_KAWASAKI, *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU, *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE, *FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE, *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA, *FIGHTER_STATUS_KIND_CAPTURE_MASTERCORE, *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD, *FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_ICE, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_SLIP, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_FINAL, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_REBOUND, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_PASSIVE, *FIGHTER_STATUS_KIND_KILLER, *FIGHTER_STATUS_KIND_ICE_JUMP, *FIGHTER_STATUS_KIND_LAY_DOWN, *FIGHTER_STATUS_KIND_PIT_FALL, *FIGHTER_STATUS_KIND_ROULETTE, *FIGHTER_STATUS_KIND_WARPSTAR, *FIGHTER_STATUS_KIND_BURY_JUMP, *FIGHTER_STATUS_KIND_BURY_WAIT, *FIGHTER_STATUS_KIND_SLIP_WAIT, *FIGHTER_STATUS_KIND_SLEEP_END, *FIGHTER_STATUS_KIND_STOP_CEIL, *FIGHTER_STATUS_KIND_STOP_WALL, *FIGHTER_STATUS_KIND_SWALLOWED, *FIGHTER_STATUS_KIND_YOSHI_EGG, *FIGHTER_STATUS_KIND_KASEY_WARP, *FIGHTER_STATUS_KIND_SLIP_STAND, *FIGHTER_STATUS_KIND_TREAD_FALL, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_DEMON_DIVED, *FIGHTER_STATUS_KIND_DETACH_WALL, *FIGHTER_STATUS_KIND_BITTEN_WARIO, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, *FIGHTER_STATUS_KIND_DRAGOON_RIDE, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_KAMUI_PIERCE, *FIGHTER_STATUS_KIND_PASSIVE_CEIL, *FIGHTER_STATUS_KIND_PASSIVE_WALL, *FIGHTER_STATUS_KIND_REBOUND_JUMP, *FIGHTER_STATUS_KIND_REBOUND_STOP, *FIGHTER_STATUS_KIND_SLIP_STAND_B, *FIGHTER_STATUS_KIND_SLIP_STAND_F, *FIGHTER_STATUS_KIND_SMASH_APPEAL, *FIGHTER_STATUS_KIND_SUICIDE_BOMB, *FIGHTER_STATUS_KIND_TREAD_DAMAGE, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_STAND_FB, *FIGHTER_STATUS_KIND_GENESIS_SHOOT, *FIGHTER_STATUS_KIND_GIMMICK_EATEN, *FIGHTER_STATUS_KIND_GLIDE_LANDING, *FIGHTER_STATUS_KIND_ITEM_STARRING, *FIGHTER_STATUS_KIND_MEWTWO_THROWN].contains(&status_kind) {
				GUN_C[ENTRY_ID] = NONE;
			};
			if GUN_C[ENTRY_ID] != NONE && status_kind == *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE {
				if MotionModule::frame(boma) < 20.0 {
					MotionModule::set_rate(boma, 2.0);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [*FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_BARRAGE, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_JUMP, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE].contains(&status_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
					};
				};
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
					if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					};
				};
				if ControlModule::get_stick_y(boma) <= -0.625 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					};
				};
			};
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 && (WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == false || AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) == false){
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
					GUN_C[ENTRY_ID] = ATTACK;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
					GUN_C[ENTRY_ID] = ATTACK_S3;
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
					GUN_C[ENTRY_ID] = ATTACK_S4;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					if motion_kind == hash40("attack_air_f") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_F;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
					};
					if motion_kind == hash40("attack_air_b") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_B;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE, true);
					};
					if motion_kind == hash40("attack_air_hi") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_HI;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
					};
				};
				println!("{}", GUN_C[ENTRY_ID]);
			};
		};
    }
}
#[fighter_frame( agent = FIGHTER_KIND_JACK )]
fn joker_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		let motion_kind = MotionModule::motion_kind(boma);
		if smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_END, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START, *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_JUMP, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON, *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_ITEM, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_BEETLE, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_DRIVER, *FIGHTER_STATUS_KIND_CAPTURE_NABBIT, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP, *FIGHTER_STATUS_KIND_CAPTURE_KAWASAKI, *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU, *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE, *FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE, *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA, *FIGHTER_STATUS_KIND_CAPTURE_MASTERCORE, *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD, *FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_ICE, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_SLIP, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_FINAL, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_REBOUND, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_PASSIVE, *FIGHTER_STATUS_KIND_KILLER, *FIGHTER_STATUS_KIND_ICE_JUMP, *FIGHTER_STATUS_KIND_LAY_DOWN, *FIGHTER_STATUS_KIND_PIT_FALL, *FIGHTER_STATUS_KIND_ROULETTE, *FIGHTER_STATUS_KIND_WARPSTAR, *FIGHTER_STATUS_KIND_BURY_JUMP, *FIGHTER_STATUS_KIND_BURY_WAIT, *FIGHTER_STATUS_KIND_SLIP_WAIT, *FIGHTER_STATUS_KIND_SLEEP_END, *FIGHTER_STATUS_KIND_STOP_CEIL, *FIGHTER_STATUS_KIND_STOP_WALL, *FIGHTER_STATUS_KIND_SWALLOWED, *FIGHTER_STATUS_KIND_YOSHI_EGG, *FIGHTER_STATUS_KIND_KASEY_WARP, *FIGHTER_STATUS_KIND_SLIP_STAND, *FIGHTER_STATUS_KIND_TREAD_FALL, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_DEMON_DIVED, *FIGHTER_STATUS_KIND_DETACH_WALL, *FIGHTER_STATUS_KIND_BITTEN_WARIO, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, *FIGHTER_STATUS_KIND_DRAGOON_RIDE, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_KAMUI_PIERCE, *FIGHTER_STATUS_KIND_PASSIVE_CEIL, *FIGHTER_STATUS_KIND_PASSIVE_WALL, *FIGHTER_STATUS_KIND_REBOUND_JUMP, *FIGHTER_STATUS_KIND_REBOUND_STOP, *FIGHTER_STATUS_KIND_SLIP_STAND_B, *FIGHTER_STATUS_KIND_SLIP_STAND_F, *FIGHTER_STATUS_KIND_SMASH_APPEAL, *FIGHTER_STATUS_KIND_SUICIDE_BOMB, *FIGHTER_STATUS_KIND_TREAD_DAMAGE, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_STAND_FB, *FIGHTER_STATUS_KIND_GENESIS_SHOOT, *FIGHTER_STATUS_KIND_GIMMICK_EATEN, *FIGHTER_STATUS_KIND_GLIDE_LANDING, *FIGHTER_STATUS_KIND_ITEM_STARRING, *FIGHTER_STATUS_KIND_MEWTWO_THROWN].contains(&status_kind) {
			GUN_C[ENTRY_ID] = NONE;
		};
		if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
			BATON_TYPE[ENTRY_ID] = 2;
		};
		if ![*FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) && smash::app::sv_information::is_ready_go(){
			ArticleModule::remove_exist(boma, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
		if GUN_C[ENTRY_ID] != NONE && status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE {
			if MotionModule::frame(boma) < 20.0 {
				MotionModule::set_rate(boma, 2.0);
			} else {
				MotionModule::set_rate(boma, 1.0);
			};
		};
		if [*FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE].contains(&status_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
				};
			};
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
				if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				};
			};
			if ControlModule::get_stick_y(boma) <= -0.625 {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				};
			};
		};
		if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 && (WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == false || AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) == false){
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
				GUN_C[ENTRY_ID] = ATTACK;
				macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x15e858a896), true, true);
				macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0a38380378), true, true);
				macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0f6ac1c8de), true, true);
				macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x17a5cc8181), true, true);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
				GUN_C[ENTRY_ID] = ATTACK_S3;
				PostureModule::reverse_lr(boma);
				PostureModule::update_rot_y_lr(boma);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
				GUN_C[ENTRY_ID] = ATTACK_S4;
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
				if motion_kind == hash40("attack_air_f") {
					GUN_C[ENTRY_ID] = ATTACK_AIR_F;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
				};
				if motion_kind == hash40("attack_air_b") {
					GUN_C[ENTRY_ID] = ATTACK_AIR_B;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, true);
				};
				if motion_kind == hash40("attack_air_hi") {
					GUN_C[ENTRY_ID] = ATTACK_AIR_HI;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
				};
			};
			println!("{}", GUN_C[ENTRY_ID]);
		};
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
		joker_eiagon,
		joker_eiagon_air,
		joker_gun,
		joker_gun_air,
		joker_gun_b,
		joker_gun_b_air,
		s_joker_gun,
		s_joker_gun_b,
		e_joker_gun,
		e_joker_gun_b,
		joker_fsmash,
		joker_bair,
		joker_fair,
		joker_uair,
		joker_usmash,
		
		//Baton Pass
		joker_sideb,
		joker_sideb_eff,
		joker_sideb_snd
    );
	smashline::install_agent_frames!(joker_frame, kirby_joker_frame);
}
