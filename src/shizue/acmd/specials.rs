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
use super::super::*;

#[acmd_script(
    agent = "shizue_pot",
    script =  "effect_burst",
    category = ACMD_EFFECT, low_priority )]
unsafe fn isa_item_burst_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script(
    agent = "shizue_pot",
    script =  "sound_burst",
    category = ACMD_SOUND, low_priority )]
unsafe fn isa_item_burst_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "shizue_pot",
    scripts =  ["game_throwed"],
    category = ACMD_GAME, low_priority )]
unsafe fn isa_neutralb_hit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let otarget_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
	let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_SHIZUE {
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture02"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture03"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture06"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture08"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture10"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture12"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture13"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture15"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture16"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture17"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture18"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture19"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture21"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture22"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture23"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture24"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture25"),false);
			if ISA_SHOT_KIND[ENTRY_ID] == 1 {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("item_1"), 0.0, 1.0, false, 0.0, false, false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture01"),true); //Photo

				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture05"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture04"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture07"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture09"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture11"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture20"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture14"),false);
				if macros::is_excute(fighter) {
					macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
					AttackModule::enable_safe_pos(fighter.module_accessor);
				}
			
			};
			if ISA_SHOT_KIND[ENTRY_ID] == 2 {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("item_2"), 0.0, 1.0, false, 0.0, false, false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture05"),true); //Rug

				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture01"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture04"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture07"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture09"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture11"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture20"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture14"),false);
				if macros::is_excute(fighter) {
					macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 9.0, /*Angle*/ 30, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
					AttackModule::enable_safe_pos(fighter.module_accessor);
				}
			
			};
			if ISA_SHOT_KIND[ENTRY_ID] == 3 {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("item_3"), 0.0, 1.0, false, 0.0, false, false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture04"),true); //Triforce

				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture01"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture05"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture07"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture09"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture11"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture20"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture14"),false);
				if macros::is_excute(fighter) {
					macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 1.4, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -8, /*Trip*/ 0.0, /*Rehit*/ 4, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *COLLISION_SOUND_ATTR_MAGIC);
					AttackModule::enable_safe_pos(fighter.module_accessor);
				}
				wait(fighter.lua_state_agent, 24.0);
				if macros::is_excute(fighter) {
					AttackModule::clear_all(fighter.module_accessor);
				}
			};
			if ISA_SHOT_KIND[ENTRY_ID] == 4 {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("item_4"), 0.0, 1.0, false, 0.0, false, false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture07"),true); //Table

				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture01"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture05"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture04"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture09"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture11"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture20"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture14"),false);
				if macros::is_excute(fighter) {
					macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 11.0, /*Angle*/ 60, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 71, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
					AttackModule::enable_safe_pos(fighter.module_accessor);
				}
			};
			if ISA_SHOT_KIND[ENTRY_ID] == 5 {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("item_5"), 0.0, 1.0, false, 0.0, false, false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture09"),true); //Trophy

				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture01"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture05"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture04"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture07"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture11"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture20"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture14"),false);
				if macros::is_excute(fighter) {
					macros::SET_SPEED_EX(fighter, 0.0, 10.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 5.0, /*Angle*/ 270, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 41, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 18, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bury"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_OBJECT);
					AttackModule::enable_safe_pos(fighter.module_accessor);
				}
			};
			if ISA_SHOT_KIND[ENTRY_ID] == 6 {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("item_6"), 0.0, 1.0, false, 0.0, false, false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture11"),true); //Froggy

				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture01"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture05"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture04"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture07"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture09"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture20"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture14"),false);
				if macros::is_excute(fighter) {
					macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 18.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 1.4, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 18, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_BAT, /*Type*/ *ATTACK_REGION_OBJECT);
					AttackModule::enable_safe_pos(fighter.module_accessor);
				}
			};
			if ISA_SHOT_KIND[ENTRY_ID] == 7 {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("item_7"), 0.0, 1.0, false, 0.0, false, false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture20"),true); //Moyai

				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture01"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture05"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture04"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture07"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture09"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture11"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture14"),false);
				if macros::is_excute(fighter) {
					macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 15.0, /*Angle*/ 85, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 23, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_OBJECT);
					AttackModule::enable_safe_pos(fighter.module_accessor);
				}
			
			};
			if ISA_SHOT_KIND[ENTRY_ID] == 8 {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("item_8"), 0.0, 1.0, false, 0.0, false, false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture14"),true); //Couch

				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture01"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture05"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture04"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture07"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture09"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture11"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture20"),false);
				if macros::is_excute(fighter) {
					macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 13.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_OBJECT);
					AttackModule::enable_safe_pos(fighter.module_accessor);
				}
			};
	} else {
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture02"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture03"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture06"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture08"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture10"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture12"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture13"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture15"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture16"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture17"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture18"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture19"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture21"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture22"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture23"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture24"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture25"),false);
		MotionModule::change_motion(fighter.module_accessor, Hash40::new("item_1"), 0.0, 1.0, false, 0.0, false, false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture01"),true); //Photo

		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture05"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture04"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture07"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture09"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture11"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture20"),false);
		ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("furniture14"),false);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
			AttackModule::enable_safe_pos(fighter.module_accessor);
		}
		
	}
}
#[acmd_script(
    agent = "shizue",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME, low_priority )]
unsafe fn isa_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_N_FLAG_SEARCH);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			WorkModule::inc_int(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_N_INT_TAKEOUT_REQUEST);
		}
	} else {
		if macros::is_excute(fighter) {
			let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let rand_val = smash::app::sv_math::rand(hash40("fighter"), 8);
			ISA_SHOT_KIND[ENTRY_ID] = rand_val + 1;
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.5);
		frame(fighter.lua_state_agent, 8.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.5);
		if macros::is_excute(fighter) {
			if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POT) {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POT, false, 0);
			}
			ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
		}
	}
}
#[acmd_script(
    agent = "shizue",
    scripts =  ["effect_specialn", "effect_specialairn"],
    category = ACMD_EFFECT, low_priority )]
unsafe fn isa_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new_raw(0x1408ec2771), Hash40::new("top"), 0, 9, 7.3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
}
#[acmd_script(
    agent = "shizue",
    scripts =  ["sound_specialn", "sound_specialairn"],
    category = ACMD_SOUND, low_priority )]
unsafe fn isa_neutralb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_shizue_attackdash_01"));
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_shizue_special_n02"));
		}
}

#[acmd_script(
    agent = "shizue",
    scripts =  ["game_speciallwfire", "game_specialairlwfire"],
    category = ACMD_GAME, low_priority )]
unsafe fn isa_lloid_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if ISA_RESHOOT_TIME[ENTRY_ID] < 1 {
		ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
		ISA_RESHOOT_TIME[ENTRY_ID] = 130;
	};
}
#[acmd_script(
    agent = "shizue",
    scripts =  ["game_speciallwset"],
    category = ACMD_GAME, low_priority )]
unsafe fn isa_lloid_set(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_LW_FLAG_SET);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
}

pub fn install() {
    smashline::install_acmd_scripts!(
		isa_item_burst_eff, isa_item_burst_snd,
        isa_neutralb_hit,
        isa_neutralb, isa_neutralb_eff, isa_neutralb_snd,
        isa_lloid_end,
        isa_lloid_set
    );
}