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

static mut LUCINA_STANCE : [i32; 8] = [0; 8];
static mut DOWNB_TIMER : [i32; 8] = [1; 8];
static mut LUCINA_SIDEB : [bool; 8] = [false; 8];
static mut TIMER : [i32; 8] = [0; 8];
static HERO_DASH : f32 = 1.76;
static HERO_RUN : f32 = 1.825;
static SWORDMASTER_AIR_SPEED : f32 = 2.0;
static SWORDMASTER_MAX_GRAVITY: f32 = 0.1;
static HERO_SWORD_MULT : f32 = 1.15;

static HERO_NAIR_LANDING : f32 = 9.0;
static HERO_FAIR_LANDING : f32 = 13.0;
static HERO_BAIR_LANDING : f32 = 9.0;
static HERO_UAIR_LANDING : f32 = 9.0;
static HERO_DAIR_LANDING : f32 = 16.0;

static SWORD_NAIR_LANDING : f32 = 9.0;
static SWORD_FAIR_LANDING : f32 = 10.0;
static SWORD_BAIR_LANDING : f32 = 6.0;
static SWORD_UAIR_LANDING : f32 = 8.0;
static SWORD_DAIR_LANDING : f32 = 14.0;
/*
Base Lucina Landing Lag Stats for reference:
<float hash="landing_attack_air_frame_n">9</float>
<float hash="landing_attack_air_frame_f">12</float>
<float hash="landing_attack_air_frame_b">10</float>
<float hash="landing_attack_air_frame_hi">8</float>
<float hash="landing_attack_air_frame_lw">14</float>
*/
static mut S1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 2.0 };
static mut S2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 4.0 };
static mut S3 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 7.5 };
//Stance 0 - Hero Stance
//Stance 1 - Swordmaster Stance

#[acmd_script(
    agent = "lucina",
    script =  "game_attacks3",
    category = ACMD_GAME)]
unsafe fn lucina_ftilt(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 0 {
						macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.25);
				} else {
						macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
				};
		};
		frame(fighter.lua_state_agent, 8.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 72, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 72, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 72, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 6.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 0.1);
				} else {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 11.0, /*Angle*/ 35, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 11.0, /*Angle*/ 35, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 11.0, /*Angle*/ 35, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 1.5);
				};
		};
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
				};
		};
}		
		
#[acmd_script(
    agent = "lucina",
    script =  "game_attacklw3",
    category = ACMD_GAME)]
unsafe fn lucina_dtilt(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 0 {
						macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.43);
				};
		};
		frame(fighter.lua_state_agent, 7.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 74, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 49, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 2.7, /*Z*/ 14.7, /*X2*/ Some(0.0), /*Y2*/ Some(4.3), /*Z2*/ Some(9.2), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 8.5, /*Angle*/ 74, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 49, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 7.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_SWORD);
				} else {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 30, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 2.7, /*Z*/ 16.7, /*X2*/ Some(0.0), /*Y2*/ Some(4.3), /*Z2*/ Some(9.2), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 10.0, /*Angle*/ 30, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.7, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 9.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				};
		};
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}	
#[acmd_script(
    agent = "lucina",
    script =  "game_specials4hi",
    category = ACMD_GAME)]
unsafe fn lucina_sideb4(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.7, /*Angle*/ 79, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 10.5, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(10.5), /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.7, /*Angle*/ 87, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(15.0), /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.7, /*Angle*/ 79, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 11.0, /*X2*/ Some(0.0), /*Y2*/ Some(24.0), /*Z2*/ Some(4.0), /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.7, /*Angle*/ 87, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 23.0, /*Z*/ 14.0, /*X2*/ Some(0.0), /*Y2*/ Some(27.0), /*Z2*/ Some(4.0), /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		};
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}	
#[acmd_script(
    agent = "lucina",
    script =  "game_specialairs4s",
    category = ACMD_GAME)]
unsafe fn lucina_air_sideb4s(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 11.7,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.5,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.0,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 11.7,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.5,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.0,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			};
		};
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}	
#[acmd_script(
    agent = "lucina",
    script =  "game_specials4s",
    category = ACMD_GAME)]
unsafe fn lucina_sideb4s(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 11.7,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.5,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.0,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ShieldstunMul*/ 4.5);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 11.7,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.5,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.75, /*Angle*/ 361, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.0,  /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			};
		};
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}	
#[acmd_script(
    agent = "lucina",
    script =  "game_specialairs4hi",
    category = ACMD_GAME)]
unsafe fn lucina_airsideb4(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.7, /*Angle*/ 79, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 10.5, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(10.5), /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.7, /*Angle*/ 87, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(15.0), /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.7, /*Angle*/ 79, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 11.0, /*X2*/ Some(0.0), /*Y2*/ Some(24.0), /*Z2*/ Some(4.0), /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.7, /*Angle*/ 87, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 23.0, /*Z*/ 14.0, /*X2*/ Some(0.0), /*Y2*/ Some(27.0), /*Z2*/ Some(4.0), /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		};
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}	
#[acmd_script(
    agent = "lucina",
    script =  "game_attackairn",
    category = ACMD_GAME)]
unsafe fn lucina_nair(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 4.275, /*Angle*/ 75, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.8, /*X*/ 1.0, /*Y*/ -1.3, /*Z*/ 1.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 4.275, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.0, /*X*/ -1.5, /*Y*/ 1.0, /*Z*/ -1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 4.275, /*Angle*/ 90, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 3.3, /*X*/ 1.0, /*Y*/ -1.3, /*Z*/ 7.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 0.27);*/
				} else {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 10.275, /*Angle*/ 361, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.8, /*X*/ 1.0, /*Y*/ -1.3, /*Z*/ 1.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.275, /*Angle*/ 361, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.5, /*X*/ -1.5, /*Y*/ 1.0, /*Z*/ -1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 10.275, /*Angle*/ 361, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 4.3, /*X*/ 1.0, /*Y*/ -1.3, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 1.4);*/
				};
		};
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						AttackModule::clear_all(fighter.module_accessor);
				};
		};
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 0 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 5.75, /*Angle*/ 361, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.8, /*X*/ 1.0, /*Y*/ -1.3, /*Z*/ 1.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 5.75, /*Angle*/ 361, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.0, /*X*/ -1.5, /*Y*/ 1.0, /*Z*/ -1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 5.75, /*Angle*/ 361, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 3.3, /*X*/ 1.0, /*Y*/ -1.3, /*Z*/ 8.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 1.4);*/
				};
		};
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 8.55, /*Angle*/ 50, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.8, /*X*/ 1.2, /*Y*/ -1.1, /*Z*/ 1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 8.55, /*Angle*/ 50, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.0, /*X*/ -2.0, /*Y*/ 1.0, /*Z*/ -1.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 8.55, /*Angle*/ 50, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.4, /*X*/ 0.8, /*Y*/ -1.1, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 0.27);*/
				};
		};
		wait(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
				};
		};
		frame(fighter.lua_state_agent, 51.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
}
#[acmd_script(
    agent = "lucina",
    scripts =  ["game_specialnendhi", "game_specialairnendhi"],
    category = ACMD_GAME)]
unsafe fn lucina_neutralb_hi(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
				if LUCINA_STANCE[ENTRY_ID] == 0 {
					let speed = smash::phx::Vector3f { x: 0.5, y: 0.0, z: 0.0 };
					KineticModule::add_speed(fighter.module_accessor, &speed);
				};
			};
		};
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 60, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(20.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 180, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 25.0, /*X2*/ Some(0.0), /*Y2*/ Some(14.6), /*Z2*/ Some(27.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(20.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 25.0, /*X2*/ Some(0.0), /*Y2*/ Some(14.6), /*Z2*/ Some(27.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						let hit1 = smash::phx::Vector2f { x: 14.0, y: 5.25 };
						AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40::new("top"), &hit1, 8, false);
						let hit2 = smash::phx::Vector2f { x: 23.5, y: 2.85 };
						AttackModule::set_vec_target_pos(fighter.module_accessor, 3, Hash40::new("top"), &hit2, 8, false);
				} else {
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(20.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 25.0, /*X2*/ Some(0.0), /*Y2*/ Some(14.6), /*Z2*/ Some(27.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 0.9, /*X*/ 0.0, /*Y*/ 9.8, /*Z*/ 17.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.3), /*Z2*/ Some(22.0), /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_HEAD, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_marth_shield_breaker"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 0.9, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 23.5, /*X2*/ Some(0.0), /*Y2*/ Some(14.7), /*Z2*/ Some(28.9), /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_HEAD, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_marth_shield_breaker"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				};
		};
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}
#[acmd_script(
    agent = "lucina",
    scripts =  ["game_specialnendlw", "game_specialairnendlw"],
    category = ACMD_GAME)]
unsafe fn lucina_neutralb_lw(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
				if LUCINA_STANCE[ENTRY_ID] == 0 {
					let speed = smash::phx::Vector3f { x: 0.5, y: 0.0, z: 0.0 };
					KineticModule::add_speed(fighter.module_accessor, &speed);
				};
			};
		};
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 60, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(20.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 180, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 25.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.7), /*Z2*/ Some(27.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(20.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 25.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.7), /*Z2*/ Some(27.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						let hit1 = smash::phx::Vector2f { x: 14.0, y: 8.5 };
						AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40::new("top"), &hit1, 8, false);
						let hit2 = smash::phx::Vector2f { x: 23.5, y: 8.5 };
						AttackModule::set_vec_target_pos(fighter.module_accessor, 3, Hash40::new("top"), &hit2, 8, false);
				} else {
					macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(20.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
					macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 25.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.7), /*Z2*/ Some(27.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
					macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 0.9, /*X*/ 0.0, /*Y*/ 5.6, /*Z*/ 17.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(22.0), /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_HEAD, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_marth_shield_breaker"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
					macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 0.9, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 23.5, /*X2*/ Some(0.0), /*Y2*/ Some(1.7), /*Z2*/ Some(28.9), /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_HEAD, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_marth_shield_breaker"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				};
		};
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		
}
#[acmd_script(
    agent = "lucina",
    scripts =  ["game_specialn"],
    category = ACMD_GAME)]
unsafe fn lucina_astra_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.5, Angle=42, KBG=84, FKB=0, BKB=75, Size=6.5, X=0.0, Y=8.5, Z=8.0, X2=0.0, Y2=8.5, Z2=20.0, Hitlag=1.15, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.5, Angle=42, KBG=84, FKB=0, BKB=75, Size=6.5, X=0.0, Y=8.5, Z=25.0, X2=0.0, Y2=8.5, Z2=27.0, Hitlag=1.15, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=10)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	

#[acmd_script(
    agent = "lucina",
    scripts =  ["game_specialairn"],
    category = ACMD_GAME)]
unsafe fn lucina_astra_air_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.5, Angle=42, KBG=84, FKB=0, BKB=75, Size=6.5, X=0.0, Y=8.5, Z=8.0, X2=0.0, Y2=8.5, Z2=20.0, Hitlag=1.15, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.5, Angle=42, KBG=84, FKB=0, BKB=75, Size=6.5, X=0.0, Y=8.5, Z=25.0, X2=0.0, Y2=8.5, Z2=27.0, Hitlag=1.15, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=10)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=18)
		FT_MOTION_RATE(FSM=1.2)
    });
}	
#[acmd_script(
    agent = "lucina",
    scripts =  ["expression_specialnendlw", "expression_specialairnendlw", "expression_specialnendhi", "expression_specialairnendhi"],
    category = ACMD_EXPRESSION)]
unsafe fn lucina_neutralb_expr(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("sword1"), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Z), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_X));
		};
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
		};
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 0 {
					macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
				};
				macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_pierces"), 0);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "game_attackairlw",
    category = ACMD_GAME)]
unsafe fn lucina_dair(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 0 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 12.35, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 12.35, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 12.35, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 1.4);*/
				} else {
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 8.35, /*Angle*/ 80, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 8.35, /*Angle*/ 80, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 8.35, /*Angle*/ 80, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 5.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 0.27);*/
				};
		};
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 0 {
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 12.35, /*Angle*/ 280, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			};
		};
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.630435);
			};
		};
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
				};
		};
		frame(fighter.lua_state_agent, 55.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
	}			
#[acmd_script(
    agent = "lucina",
    script =  "game_attackairb",
    category = ACMD_GAME)]
unsafe fn lucina_bair(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if LUCINA_STANCE[ENTRY_ID] == 0{
					macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.25);
		};
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			macros::REVERSE_LR(fighter);
		};
		frame(fighter.lua_state_agent, 7.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 0 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 11.875, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 3.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 11.875, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 11.875, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 1.4);*/
				} else {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 9.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 3.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 9.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 3.5, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 9.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 0.27);*/
				};
		};
		wait(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
}		
#[acmd_script(
    agent = "lucina",
    script =  "game_attackairb2",
    category = ACMD_GAME)]
unsafe fn lucina_bair_sword(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 10.5, /*Angle*/ 361, /*KBG*/ 92, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 5.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 10.5, /*Angle*/ 361, /*KBG*/ 92, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 5.5, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
			/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 0.27);*/
		};
		frame(fighter.lua_state_agent, 10.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.2);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 92, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 4.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 92, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 4.5, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
			/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 0.27);*/
		};
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
}		
#[acmd_script(
    agent = "lucina",
    script =  "sound_attackairb2",
    category = ACMD_SOUND)]
unsafe fn lucina_bair_sword_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_lucina_rnd_attack"))
			PLAY_SE(hash40("se_lucina_swing_s"))
		}
    });
}	
#[acmd_script(
    agent = "lucina",
    script =  "effect_attackairb2",
    category = ACMD_EFFECT)]
unsafe fn lucina_bair_sword_eff(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc"), hash40("top"), -1, 8, -5, -167.5, 18.2, 9.3, 0.8, true)
			LAST_EFFECT_SET_COLOR(1.0, 0.18, 2.0)
		}
		frame(Frame=8)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("toer"), 0, 0, 0, 0, 0, 0, 1.35, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=12)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_attack_arc"), false, true)
		}
    });
}		
		
#[acmd_script(
    agent = "lucina",
    script =  "game_attackairf",
    category = ACMD_GAME)]
unsafe fn lucina_fair(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if LUCINA_STANCE[ENTRY_ID] == 1 {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		} else {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
		};
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
		frame(fighter.lua_state_agent, 6.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 0 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.8, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 1.4);*/
				} else {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 7.5, /*Angle*/ 57, /*KBG*/ 53, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 7.5, /*Angle*/ 57, /*KBG*/ 53, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 3.8, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 7.5, /*Angle*/ 57, /*KBG*/ 53, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 8.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ShieldstunMul*/ 0.27);*/
				};
		};
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		if LUCINA_STANCE[ENTRY_ID] == 1 {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.72);
		};
		frame(fighter.lua_state_agent, 36.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
	}	
#[acmd_script(
    agent = "lucina",
    script =  "game_attackairf2",
    category = ACMD_GAME)]
unsafe fn lucina_fair_hero(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 13.5, /*Angle*/ 361, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0*HERO_SWORD_MULT, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 4.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_marth_shield_breaker"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 13.5, /*Angle*/ 361, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.8, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_marth_shield_breaker"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 13.5, /*Angle*/ 361, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0*HERO_SWORD_MULT, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ ((9.5-4.5)*HERO_SWORD_MULT)+4.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_marth_shield_breaker"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 1.4);*/
		};
		frame(fighter.lua_state_agent, 17.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.68);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		frame(fighter.lua_state_agent, 36.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "sound_attackairf2",
    category = ACMD_SOUND)]
unsafe fn lucina_fair_hero_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_lucina_rnd_attack"))
			PLAY_SE(hash40("se_lucina_smash_s01"))
		}
    });
}	
#[acmd_script(
    agent = "lucina",
    script =  "effect_attackairf2",
    category = ACMD_EFFECT)]
unsafe fn lucina_fair_hero_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
		};
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 2);
		};
}		
#[acmd_script(
    agent = "lucina",
    script =  "game_attackairhi",
    category = ACMD_GAME)]
unsafe fn lucina_uair(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if LUCINA_STANCE[ENTRY_ID] == 1 {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		} else {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
		};
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
		frame(fighter.lua_state_agent, 5.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 8.4, /*Angle*/ 85, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 8.4, /*Angle*/ 85, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 8.4, /*Angle*/ 85, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 8.4, /*Angle*/ 85, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 5.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 0.27);*/
				} else {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 11.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 11.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 11.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 11.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 1.4);*/
				};
		};
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 4.4, /*Angle*/ 361, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 4.4, /*Angle*/ 461, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 4.4, /*Angle*/ 85, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 4.4, /*Angle*/ 361, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 5.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						/*macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 0.27);*/
						macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
				};
		};
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		frame(fighter.lua_state_agent, 38.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
		wait(fighter.lua_state_agent, 16.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
}		
#[acmd_script(
    agent = "lucina",
    script =  "game_attackairhi2",
    category = ACMD_GAME)]
unsafe fn lucina_uair_sword(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legr"), /*Damage*/ 7.6, /*Angle*/ 75, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 8.4, /*Angle*/ 75, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 8.4, /*Angle*/ 75, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
		};
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			CancelModule::enable_cancel(fighter.module_accessor);
		};
}	
#[acmd_script(
    agent = "lucina",
    script =  "effect_attackairhi2",
    category = ACMD_EFFECT)]
unsafe fn lucina_uair_sword_eff(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			//macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -0, -0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 11, 0, -5.8, 61.8, 92.4, 1, true);
			macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.18, 2.0);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
				macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc"), false, true);
		}
		/*frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
		};
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		};*/
}	
#[acmd_script(
    agent = "lucina",
    script =  "sound_attackairhi2",
    category = ACMD_SOUND)]
unsafe fn lucina_uair_sword_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			PLAY_SE(hash40("se_lucina_swing_s"))
			PLAY_SEQUENCE(hash40("seq_lucina_rnd_attack"))
		}
    });
}	
#[acmd_script(
    agent = "lucina",
    script =  "game_attack11",
    category = ACMD_GAME)]
unsafe fn lucina_jab1(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 3.325, /*Angle*/ 361, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 3.325, /*Angle*/ 361, /*KBG*/ 12, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 0.5, /*Y*/ 0.0, /*Z*/ 1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 3.325, /*Angle*/ 180, /*KBG*/ 12, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 3.325, /*Angle*/ 361, /*KBG*/ 12, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 3.325, /*Angle*/ 361, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 3.325, /*Angle*/ 361, /*KBG*/ 12, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0*HERO_SWORD_MULT, /*X*/ 0.5, /*Y*/ 0.0, /*Z*/ 1.5*HERO_SWORD_MULT, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 3.325, /*Angle*/ 180, /*KBG*/ 12, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0*HERO_SWORD_MULT, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ ((7.0-1.5)*HERO_SWORD_MULT)+1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 3.325, /*Angle*/ 361, /*KBG*/ 12, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0*HERO_SWORD_MULT, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ ((7.0-1.5)*HERO_SWORD_MULT)+1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			};
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 3.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 3.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor,  /*ID*/ 2, /*Frames*/ 3.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor,  /*ID*/ 3, /*Frames*/ 3.0, /*Unk*/ false);
		};
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
		};
}		
#[acmd_script(
    agent = "lucina",
    script =  "game_attacks4",
    category = ACMD_GAME)]
unsafe fn lucina_fsmash(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		};
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0*HERO_SWORD_MULT, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("bust"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ ((8.3-2.0)*HERO_SWORD_MULT)+2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		};
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}	
		
#[acmd_script(
    agent = "lucina",
    script =  "game_attackdash",
    category = ACMD_GAME)]
unsafe fn lucina_da(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if LUCINA_STANCE[ENTRY_ID] == 1 {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.46);
		} else {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.1538461538461538461538461538462);
		};
		frame(fighter.lua_state_agent, 13.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 67, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 67, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 7.0, /*Angle*/ 461, /*KBG*/ 67, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 67, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 5.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 0.2);
				} else {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 13.0, /*Angle*/ 35, /*KBG*/ 67, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0*HERO_SWORD_MULT, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 13.0, /*Angle*/ 35, /*KBG*/ 67, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 13.0, /*Angle*/ 35, /*KBG*/ 67, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 2.0*HERO_SWORD_MULT, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 13.0, /*Angle*/ 35, /*KBG*/ 67, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ ((8.5-2.0)*HERO_SWORD_MULT)+2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ID4*/ 3, /*ShieldstunMul*/ 1.5);
				};
		};
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}		
#[acmd_script(
    agent = "lucina",
    script =  "game_attack12",
    category = ACMD_GAME)]
unsafe fn lucina_jab2(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.5, /*X*/ 2.5, /*Y*/ -1.5, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 6.0, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 6.0, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0*HERO_SWORD_MULT, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 6.0, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 2.5, /*Y*/ -1.5, /*Z*/ ((8.0-2.0)*HERO_SWORD_MULT)+2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			};
		};
		wait(fighter.lua_state_agent, 2.0);
		if LUCINA_STANCE[ENTRY_ID] == 0 {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.1);
		}
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}		
#[acmd_script(
    agent = "lucina",
    script =  "game_throwhi",
    category = ACMD_GAME)]
unsafe fn lucina_uthrow(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 0 {
				macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 93, 135, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
				macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
			} else {
				macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 93, 102, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
				macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
			};
		};
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::CHECK_FINISH_CAMERA(fighter, 1, 21);
		};
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),  WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "game_attackhi3",
    category = ACMD_GAME)]
unsafe fn lucina_utilt(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 7.6, /*Angle*/ 100, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 7.6, /*Angle*/ 100, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 7.6, /*Angle*/ 100, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 7.6, /*Angle*/ 100, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 5.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 7.6, /*Angle*/ 100, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ Some(4.5), /*Y2*/ Some(0.0), /*Z2*/ Some(2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 7.6, /*Angle*/ 100, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 5.7, /*X2*/ Some(6.0), /*Y2*/ Some(0.0), /*Z2*/ Some(6.7), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				} else {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 10.0, /*Angle*/ 100, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 2.0*HERO_SWORD_MULT, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 100, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 10.0, /*Angle*/ 100, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 10.0, /*Angle*/ 100, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ ((8.7-2.0)*HERO_SWORD_MULT)+2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 10.0, /*Angle*/ 100, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 2.0*HERO_SWORD_MULT, /*X2*/ Some(4.5), /*Y2*/ Some(0.0), /*Z2*/ Some(2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 10.0, /*Angle*/ 100, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ ((8.7-2.0)*HERO_SWORD_MULT)+2.0, /*X2*/ Some(6.0), /*Y2*/ Some(0.0), /*Z2*/ Some(6.7), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				};
		};
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
				if LUCINA_STANCE[ENTRY_ID] == 1 {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.775);
				} else {
						macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 8.075, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 2.0*HERO_SWORD_MULT, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 8.075, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 8.075, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
						macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 8.075, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.5*HERO_SWORD_MULT, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ ((7.7-2.0)*HERO_SWORD_MULT)+2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				};
		};
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}	
#[acmd_script(
    agent = "lucina",
    scripts =  ["game_speciallw", "game_specialairlw"],
    category = ACMD_GAME)]
unsafe fn lucina_downb(fighter: &mut L2CAgentBase) {

}
#[acmd_script(
    agent = "lucina",
    scripts =  ["sound_speciallw", "sound_specialairlw"],
    category = ACMD_SOUND)]
unsafe fn lucina_downb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_lucina_special_l01"))
			PLAY_SE(hash40("se_lucina_special_l01"))
			PLAY_SE(hash40("se_lucina_special_l01"))
			PLAY_SE(hash40("se_lucina_special_l01"))
			PLAY_SE(hash40("se_lucina_special_l01"))
			PLAY_SE(hash40("se_lucina_special_l01"))
			PLAY_SE(hash40("se_lucina_special_l01"))
			PLAY_SE(hash40("se_lucina_special_l01"))
			PLAY_SE(hash40("se_lucina_special_l02"))
			//Dont ask
		}
    });
}
//Effects
#[acmd_script(
    agent = "lucina",
    scripts =  ["effect_speciallw", "effect_specialairlw"],
    category = ACMD_EFFECT)]
unsafe fn lucina_downb_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			//Intentionally swapped flashes due to it changing class on frame 10
			macros::EFFECT(fighter, Hash40::new("lucina_counter_flash"), Hash40::new("top"), 0, 11, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::FLASH(fighter, 0.18, 0.25, 1.0, 1.0);
			} else {
				macros::FLASH(fighter, 0.52, 0.18, 1.0, 1.0);
			};
		};
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::COL_NORMAL(fighter);
		};
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::FLASH(fighter, 0.18, 0.25, 1.0, 1.0);
			} else {
				macros::FLASH(fighter, 0.52, 0.18, 1.0, 1.0);
			};
		};
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::COL_NORMAL(fighter);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_attack11",
    category = ACMD_EFFECT)]
unsafe fn lucina_jab_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_attack12",
    category = ACMD_EFFECT)]
unsafe fn lucina_jab2_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_attacks3",
    category = ACMD_EFFECT)]
unsafe fn lucina_ftilt_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 4);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_attackhi3",
    category = ACMD_EFFECT)]
unsafe fn lucina_utilt_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 1);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_attacklw3",
    category = ACMD_EFFECT)]
unsafe fn lucina_dtilt_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 2);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_attackdash",
    category = ACMD_EFFECT)]
unsafe fn lucina_da_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		};
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_attackairb",
    category = ACMD_EFFECT)]
unsafe fn lucina_bair_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_attackairf",
    category = ACMD_EFFECT)]
unsafe fn lucina_fair_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_attackairhi",
    category = ACMD_EFFECT)]
unsafe fn lucina_uair_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_cliffattack",
    category = ACMD_EFFECT)]
unsafe fn lucina_cliff_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("swordl"), -0, -0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
		};
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_downattackd",
    category = ACMD_EFFECT)]
unsafe fn lucina_downd_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
		};
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
		};
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 2);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_downattacku",
    category = ACMD_EFFECT)]
unsafe fn lucina_downu_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6.5, -6, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
			macros::LAST_PARTICLE_SET_COLOR(fighter, 0.6, 0.65, 1);
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		};
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 4);
		};
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6.5, -6, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
			macros::LAST_PARTICLE_SET_COLOR(fighter, 0.6, 0.65, 1);
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		};
		frame(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 4);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_attackairlw",
    category = ACMD_EFFECT)]
unsafe fn lucina_dair_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 5);
		};
}
#[acmd_script(
    agent = "lucina",
    script =  "effect_attackairn",
    category = ACMD_EFFECT)]
unsafe fn lucina_nair_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE_OFF(fighter, 4);
			};
		};
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		};
}
#[acmd_script(
    agent = "lucina",
    scripts =  ["effect_specialhi", "effect_specialairhi"],
    category = ACMD_EFFECT)]
unsafe fn lucina_upb_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] != 1 {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_swing"), Hash40::new("top"), 0, 12, -1, 14, -30, 37, 1, true);
			};
		}
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] == 1 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("killsword_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			} else {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_jump"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 1, true);
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
			};
		};
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] != 1 {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_sword_blue"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
				macros::EFFECT_DETACH_KIND(fighter, Hash40::new("lucina_dolphin_jump"), -1);
				macros::EFFECT_DETACH_KIND(fighter, Hash40::new("lucina_dolphin_swing"), -1);
				macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_shadow"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
				EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
			};
		};
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, -10, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 6);
		};
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			if LUCINA_STANCE[ENTRY_ID] != 1 {
				macros::EFFECT_OFF_KIND(fighter, Hash40::new("lucina_sword_blue"), false, true);
			};
		};
}
#[acmd_script(
    agent = "lucina",
    scripts =  ["effect_specials", "effect_specialairs"],
    category = ACMD_EFFECT)]
unsafe fn lucina_sword_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), 0, 6.5, 0, 0, 180, 0, 2.0, true)
			LAST_EFFECT_SET_COLOR(0.8*(0.6), 0.26*(0.7), 0.96*(1.0))
		}
		frame(Frame=33)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_attack_speedline"), false, true)
		}
    });
}	
#[acmd_script(
    agent = "lucina",
    scripts =  ["game_specials", "game_specialairs"],
    category = ACMD_GAME)]
unsafe fn lucina_sword_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			JostleModule::set_status(false)
		}
		frame(Frame=30)
		if(is_excute){
			JostleModule::set_status(true)
		}
    });
}	
#[acmd_script(
    agent = "lucina",
    scripts =  ["sound_specials", "sound_specialairs"],
    category = ACMD_SOUND)]
unsafe fn lucina_sword_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			PLAY_STEP(hash40("se_lucina_dash_start"))
			SET_PLAY_INHIVIT(hash40("se_lucina_dash_start"), 20)
			PLAY_SE(hash40("se_lucina_attackl_s01"))
		}
		wait(Frames=8)
		if(is_excute){
			PLAY_STEP(hash40("se_lucina_step_right_l"))
		}
		wait(Frames=5)
		if(is_excute){
			PLAY_SE(hash40("se_lucina_step_left_l"))
		}
    });
}	
pub(crate) unsafe fn bone_const(boma: &mut smash::app::BattleObjectModuleAccessor, new_fighter_kind : i32, new_motion_kind : u64, bone : u64,
	start_frame : f32, end_frame : f32, 
	x_rotate : f32, x_rotate_end : f32, y_rotate : f32, y_rotate_end : f32,  z_rotate : f32, z_rotate_end : f32
) -> () {
	let fighter_kind = smash::app::utility::get_kind(boma);
	let motion_kind = MotionModule::motion_kind(boma);
	let frame = MotionModule::frame(boma);
	let duration = end_frame-start_frame;
	if fighter_kind == new_fighter_kind && motion_kind == new_motion_kind && frame >= start_frame && frame <= end_frame {
		let mut rotation = Vector3f{x: x_rotate + (((x_rotate_end-x_rotate)/duration)*(frame-start_frame)), y: y_rotate + (((y_rotate_end-y_rotate)/duration)*(frame-start_frame)) , z: z_rotate + (((z_rotate_end-z_rotate)/duration)*(frame-start_frame))};
	    ModelModule::set_joint_rotate(boma, Hash40::new_raw(bone), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
	};
}
				
#[fighter_frame_callback]
pub fn lucina(fighter : &mut L2CFighterCommon) {
    unsafe {
		let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let lr = PostureModule::lr(boma);
		let stick_x = ControlModule::get_stick_x(boma) * lr;
		let stick_y = ControlModule::get_stick_y(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32;
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		if fighter_kind == *FIGHTER_KIND_LUCINA {
			if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE].contains(&status_kind) || (smash::app::sv_information::is_ready_go() == false && !smash::app::smashball::is_training_mode()) {
				LUCINA_STANCE[ENTRY_ID] = 0;
				TIMER[ENTRY_ID] = 5;
			};
			TIMER[ENTRY_ID] += 1;
			if [
				*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_ATTACK_S3, 
				*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3,
				*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_HI4,
				*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
				*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
				*FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4,
				*FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_APPEAL
			].contains(&status_kind) || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 && LUCINA_STANCE[ENTRY_ID] == 0) || (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && LUCINA_STANCE[ENTRY_ID] == 0){
					TIMER[ENTRY_ID] = 5;
			};
			//Early endlag cancel 
			if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
				if cancel_frame - frame <= (10.0/MotionModule::rate(boma)) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
				};
			};
			if TIMER[ENTRY_ID] >= 6 {
					TIMER[ENTRY_ID] = 0;
					let elec1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("sword1"), &S1, &S1, 0.075, true, 0, 0, 0, 0, 0, true, true) as u32;
					let elec2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("sword1"), &S2, &S2, 0.15, true, 0, 0, 0, 0, 0, true, true) as u32;
					let elec3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("sword1"), &S3, &S3, 0.15, true, 0, 0, 0, 0, 0, true, true) as u32;
					let fire1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("sword1"), &S1, &S1, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
					let fire2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("sword1"), &S2, &S2, 0.275, true, 0, 0, 0, 0, 0, true, true) as u32;
					let fire3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("sword1"), &S3, &S3, 0.275, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, elec1, 5.0, 5.0, 4.0);
					EffectModule::set_rgb(boma, elec2, 5.0, 5.0, 4.0);
					EffectModule::set_rgb(boma, elec3, 5.0, 5.0, 4.0);
					EffectModule::set_rgb(boma, fire1, 0.0, 0.5, 6.0);
					EffectModule::set_rgb(boma, fire2, 0.0, 0.5, 6.0);
					EffectModule::set_rgb(boma, fire3, 0.0, 0.5, 6.0);
					if LUCINA_STANCE[ENTRY_ID] == 1 {
						EffectModule::set_visible(boma, elec1, true);
						EffectModule::set_visible(boma, elec2, true);
						EffectModule::set_visible(boma, elec3, true);
					} else {
						EffectModule::set_visible(boma, elec1, false);
						EffectModule::set_visible(boma, elec2, false);
						EffectModule::set_visible(boma, elec3, false);
					};
					if LUCINA_STANCE[ENTRY_ID] == 0 {
						EffectModule::set_visible(boma, fire1, true);
						EffectModule::set_visible(boma, fire2, true);
						EffectModule::set_visible(boma, fire3, true);
					} else {
						EffectModule::set_visible(boma, fire1, false);
						EffectModule::set_visible(boma, fire2, false);
						EffectModule::set_visible(boma, fire3, false);
					};
			};
			if [hash40("special_air_lw"), hash40("special_lw")].contains(&motion_kind) {
				if MotionModule::frame(boma) >= 10.0 && MotionModule::frame(boma) < 11.0 {
					if LUCINA_STANCE[ENTRY_ID] == 0 {
						LUCINA_STANCE[ENTRY_ID] = 1;
					} else {
						LUCINA_STANCE[ENTRY_ID] = 0;
					};
				};
				if MotionModule::frame(boma) >= 20.0 {
					CancelModule::enable_cancel(boma);
				};
				if MotionModule::frame(boma) > 19.0 && MotionModule::frame(boma) < 44.0 {
					MotionModule::set_rate(boma, 5.4);
				} else {
					MotionModule::set_rate(boma, 1.2);
				};
				WorkModule::set_flag(boma, false, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
				HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
			};
			if LUCINA_STANCE[ENTRY_ID] == 0 {
				if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) && MotionModule::frame(boma) <= 5.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) && MotionModule::frame(boma) > 9.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
				if MotionModule::motion_kind(boma) == hash40("throw_lw") {
					if MotionModule::frame(boma) >= 16.0 {
						MotionModule::set_rate(boma, 2.272);
					};
				};
				//Landing Lag
				if MotionModule::motion_kind(boma) == hash40("landing_air_n") {
					let landing = ((1.0/(HERO_NAIR_LANDING/ cancel_frame))as f32);
					MotionModule::set_rate(boma, landing);
				};
				if MotionModule::motion_kind(boma) == hash40("landing_air_f") {
					let landing = ((1.0/(HERO_FAIR_LANDING/ cancel_frame))as f32);
					MotionModule::set_rate(boma, landing);
				};
				if MotionModule::motion_kind(boma) == hash40("landing_air_b") {
					let landing = ((1.0/(HERO_BAIR_LANDING/ cancel_frame))as f32);
					MotionModule::set_rate(boma, landing);
				};
				if MotionModule::motion_kind(boma) == hash40("landing_air_hi") {
					let landing = ((1.0/(HERO_UAIR_LANDING/ cancel_frame))as f32);
					MotionModule::set_rate(boma, landing);
				};
				if MotionModule::motion_kind(boma) == hash40("landing_air_lw") {
					let landing = ((1.0/(HERO_DAIR_LANDING/ cancel_frame))as f32);
					MotionModule::set_rate(boma, landing);
				};
				//Fair Anim Shift
				if [hash40("attack_air_f")].contains(&motion_kind) {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_f2"), 1.0, 1.0, false, 0.0, false, false);
				};
				let max_rot = 60.0;
				let mut wrist_rot = ((frame-9.0)/(16.0-9.0))*max_rot;
				if wrist_rot > max_rot {
					wrist_rot = max_rot;
				} else if wrist_rot < 0.0 {
					wrist_rot = 0.0;
				};
				bone_const(boma, *FIGHTER_KIND_LUCINA, hash40("attack_air_f2"), hash40("handr"), 0.0, 40.0, 0.0, 0.0, wrist_rot, wrist_rot, 0.0, 0.0);
				bone_const(boma, *FIGHTER_KIND_LUCINA, hash40("attack_air_f2"), hash40("handr"), 40.0, 69.0, 0.0, 0.0, wrist_rot, 0.0, 0.0, 0.0);
				bone_const(boma, *FIGHTER_KIND_LUCINA, hash40("attack_air_f2"), hash40("rot"), 0.0, 40.0, wrist_rot*0.8, wrist_rot*0.8, 0.0, 0.0, 0.0, 0.0);
				bone_const(boma, *FIGHTER_KIND_LUCINA, hash40("attack_air_f2"), hash40("rot"), 40.0, 69.0, wrist_rot*0.8, 0.0, 0.0, 0.0, 0.0, 0.0);
				let joint_scale = smash::phx::Vector3f { x: HERO_SWORD_MULT, y: HERO_SWORD_MULT, z: HERO_SWORD_MULT };
				ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &joint_scale);	
				ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword1"), &joint_scale);	
			} else {
				if [*FIGHTER_STATUS_KIND_DASH].contains(&status_kind) {
					if MotionModule::frame(boma) == 3.0 {
							let speed = smash::phx::Vector3f { x: 0.62, y: 0.0, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
					};
				};
				//Landing Lag
				if MotionModule::motion_kind(boma) == hash40("landing_air_n") {
					let landing = ((1.0/(SWORD_NAIR_LANDING/ cancel_frame))as f32);
					MotionModule::set_rate(boma, landing);
				};
				if MotionModule::motion_kind(boma) == hash40("landing_air_f") {
					let landing = ((1.0/(SWORD_FAIR_LANDING/ cancel_frame))as f32);
					MotionModule::set_rate(boma, landing);
				};
				if MotionModule::motion_kind(boma) == hash40("landing_air_b") {
					let landing = ((1.0/(SWORD_BAIR_LANDING/ cancel_frame))as f32);
					MotionModule::set_rate(boma, landing);
				};
				if MotionModule::motion_kind(boma) == hash40("landing_air_hi") {
					let landing = ((1.0/(SWORD_UAIR_LANDING/ cancel_frame))as f32);
					MotionModule::set_rate(boma, landing);
				};
				if MotionModule::motion_kind(boma) == hash40("landing_air_lw") {
					let landing = ((1.0/(SWORD_DAIR_LANDING/ cancel_frame))as f32);
					MotionModule::set_rate(boma, landing);
				};
				//Uair Anim Shift
				if [hash40("attack_air_hi")].contains(&motion_kind) {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_hi_2"), 1.0, 1.0, false, 0.0, false, false);
				};
				//Bair Anim Shift
				if [hash40("attack_air_b")].contains(&motion_kind) {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_b2"), 1.0, 1.0, false, 0.0, false, false);
				};
				if [hash40("attack_air_b2")].contains(&motion_kind) && frame >= 46.0 {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
				};
				//Grab Nerf
				if [hash40("catch"), hash40("catch_dash"), hash40("catch_turn")].contains(&MotionModule::motion_kind(boma)) {
					if MotionModule::frame(boma) < 2.0 {
						MotionModule::set_rate(boma, 0.5);
					} else {
						MotionModule::set_rate(boma, 1.0);
					};
				};
				/*if[hash40("special_s4_hi"), hash40("special_air_s4_hi")].contains(&motion_kind) && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && (frame >= 16.0 && frame <= 36.0){
					if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
					};
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
					};
				};*/
				if [hash40("landing_heavy")].contains(&motion_kind) {
					let decrease_drift  = smash::phx::Vector3f { x: 0.925, y: 1.0, z: 1.0 };
					println!("icky wavedash!");
					KineticModule::mul_speed(boma, &decrease_drift, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
				};
				if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&status_kind){
					WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
				};
				if [*FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD].contains(&status_kind) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
				};
				if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD].contains(&status_kind) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
				};
				if [*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					if MotionModule::frame(boma) < 9.0 {
						MotionModule::set_rate(boma, 0.3846);
					} else {
						MotionModule::set_rate(boma, 1.0);
					};
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, true);
				};
				if [*FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						if !([hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) && frame > 13.0) {
							macros::SET_SPEED_EX(fighter, 0.25, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						} else {
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
						};
					};
					if StatusModule::is_situation_changed(boma) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					};
					if [hash40("special_n_end"), hash40("special_air_n_end")].contains(&motion_kind) {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n_end_hi"), 0.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n_end_hi"), 0.0, 1.0, false, 0.0, false, false);
						};
					};
					if [hash40("special_n_end_hi"), hash40("special_air_n_end_hi")].contains(&motion_kind) && frame > 12.0 {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n_end_lw"), 4.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n_end_lw"), 4.0, 1.0, false, 0.0, false, false);
						};
					};
					if [hash40("special_n_end_lw"), hash40("special_air_n_end_lw")].contains(&motion_kind) && frame > 12.0 {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n"), 4.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n"), 4.0, 1.0, false, 0.0, false, false);
						};
					};
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
					if LUCINA_SIDEB[ENTRY_ID] == true {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s4_s"), 0.0, 1.5, false, 0.0, false, false);
							LUCINA_SIDEB[ENTRY_ID] = false;
						} else {
							WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_s4_hi"), 0.0, 1.0, false, 0.0, false, false);
							let speed = smash::phx::Vector3f { x: 0.0, y: 0.5, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
							LUCINA_SIDEB[ENTRY_ID] = false;
						};
					};
				} else {
					LUCINA_SIDEB[ENTRY_ID] = false;
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
					//StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, true);
					if [hash40("special_s1"), hash40("special_air_s1")].contains(&motion_kind) {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
						};
					};
					if [hash40("special_s"), hash40("special_air_s")].contains(&motion_kind) {
						if frame > 8.0 {
							MotionModule::set_rate(boma, 0.8);
						};
						if frame > 2.0 && frame < 15.0 {
							let cat1 = ControlModule::get_command_flag_cat(boma, 0);
							if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 && frame > 6.0{
								if stick_x > -0.2 {
									if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
										StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
										LUCINA_SIDEB[ENTRY_ID] = true;
										MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s4_hi"), 0.0, 1.25, false, 0.0, false, false);
										//KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_NONE);
										//macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_NONE);
									} else {
										StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
										LUCINA_SIDEB[ENTRY_ID] = true;
										MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_s4_hi"), 0.0, 1.25, false, 0.0, false, false);
									};
								}  else {
									if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
										StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
										LUCINA_SIDEB[ENTRY_ID] = true;
										MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s4_hi"), 0.0, 1.25, false, 0.0, false, false);
										PostureModule::reverse_lr(boma);
										PostureModule::update_rot_y_lr(boma);
										//KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_NONE);
										//macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_NONE);
									} else {
										StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
										LUCINA_SIDEB[ENTRY_ID] = true;
										MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_s4_hi"), 0.0, 1.25, false, 0.0, false, false);
										PostureModule::reverse_lr(boma);
										PostureModule::update_rot_y_lr(boma);
										//macros::SET_SPEED_EX(fighter, 0.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_NONE);
									};
								};
							};
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
								if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
									if stick_x < -0.2 {
										StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_TURN, true);
									} else {
										StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_DASH, true);
									};
								} else if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
										StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
								};
							};
						};
						if frame > 17.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
						};
						if frame > 39.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							CancelModule::enable_cancel(boma);
						};
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_DASH{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_DASH);
						};
						if MotionModule::frame(boma) > 6.0 && MotionModule::frame(boma) < 8.0 {
							macros::SET_SPEED_EX(fighter, 1.0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_NONE);
						};
					};
				};
				if [*FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) {
					MotionModule::set_rate(boma, 0.75);
				};
			};
		};
	};
}

	
pub fn install() {
    smashline::install_acmd_scripts!(
		lucina_dtilt,
		lucina_sideb4,
		lucina_airsideb4,
		lucina_da,
		lucina_uair,
		lucina_jab2,
		lucina_jab1,
		lucina_utilt,
		lucina_nair,
		lucina_ftilt,
		lucina_fsmash,
		lucina_fair,
		lucina_bair,
		lucina_dair,
		lucina_uthrow,
		lucina_downb,
		//Effects
		lucina_jab_eff,
		lucina_jab2_eff,
		lucina_da_eff,
		lucina_ftilt_eff,
		lucina_utilt_eff,
		lucina_dtilt_eff,
		lucina_nair_eff,
		lucina_fair_eff,
		lucina_bair_eff,
		lucina_uair_eff,
		lucina_dair_eff,
		lucina_upb_eff,
		lucina_downd_eff,
		lucina_downu_eff,
		lucina_cliff_eff,
		lucina_downb_eff,
		lucina_air_sideb4s,
		lucina_sword_sideb,
		lucina_sword_sideb_eff,
		lucina_sword_sideb_snd,
		lucina_neutralb_hi,
		lucina_neutralb_lw,
		lucina_astra_end,
		lucina_astra_air_end,
		lucina_neutralb_expr,
		lucina_sideb4s,
		lucina_uair_sword,
		lucina_uair_sword_eff,
		lucina_uair_sword_snd,
		lucina_fair_hero,
		lucina_fair_hero_eff,
		lucina_fair_hero_snd,
		lucina_bair_sword,
		lucina_bair_sword_eff,
		lucina_bair_sword_snd,
		lucina_downb_snd
    );
	smashline::install_agent_frame_callbacks!(
		lucina
	);
}