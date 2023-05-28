use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::lua_bind::*;
use crate::util::*;
use smash::lib::L2CValue;
use crate::util::CAN_ATTACK_AIR;
use smash::app::sv_animcmd::*;

static mut ISA_RESHOOT_TIME: [i32; 8] = [0; 8];
static mut ISA_SHOT_KIND: [i32; 8] = [1; 8];



#[acmd_script(
    agent = "shizue",
    script =  "game_attack11",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_jab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER, false, 0);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 75, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 75, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 75, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}		

#[acmd_script(
    agent = "shizue",
    script =  "game_throwhi",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_uthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 7.5, /*Angle*/ 90, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 50, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "shizue",
    script =  "game_throwf",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_fthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 9.0, /*Angle*/ 30, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 107, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
			AttackModule::clear_all(fighter.module_accessor);
		}
}
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
	};
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
    script =  "game_attackdash",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SHIZUE_INSTANCE_WORK_ID_FLAG_POMPON_LEFT);
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POMPON, false, 0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SHIZUE_INSTANCE_WORK_ID_FLAG_POMPON_RIGHT);
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POMPON, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POMPON,smash::phx::Hash40::new("attack"),false,0.0);
		}
		frame(fighter.lua_state_agent, /*Frames*/ 9.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 10.0, /*Angle*/ 70, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
			AttackModule::enable_safe_pos(fighter.module_accessor);
		}
		wait(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 6.0, /*Angle*/ 70, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 5.7, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		frame(fighter.lua_state_agent, /*Frames*/ 22.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, /*Frames*/ 37.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POMPON,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}
#[acmd_script(
    agent = "shizue",
    script =  "game_attackairb",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM,smash::phx::Hash40::new("attack_hi3"),false,0.0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2);
		wait(fighter.lua_state_agent, 2.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		frame(fighter.lua_state_agent, 8.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 10.5, /*Angle*/ 361, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 10.5, /*Angle*/ 361, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 38.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}
#[acmd_script(
    agent = "shizue",
    script =  "game_attackairlw",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.769);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER, false, 0);
		}
		frame(fighter.lua_state_agent, 14.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 10.0, /*Angle*/ 270, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 5.0, /*Angle*/ 45, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_SHIZUE_HAMMER, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 39.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 46.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}
#[acmd_script(
    agent = "shizue",
    script =  "game_landingairlw",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_dair_land(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}
#[acmd_script(
    agent = "shizue",
    script =  "effect_attackairb",
    category = ACMD_EFFECT, low_priority )]
unsafe fn isa_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new_raw(0x1288a621ad), Hash40::new_raw(0x1288a621ad), Hash40::new("top"), -3, 0, 0, 13, 180, 180, 1.6, false, *EF_FLIP_YZ);
		}
}
#[acmd_script(
    agent = "shizue",
    script =  "sound_attackairb",
    category = ACMD_SOUND, low_priority )]
unsafe fn isa_bair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_shizue_attackhard_h01"));
		}
}
#[acmd_script(
    agent = "shizue",
    script =  "game_landingairb",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_bairland(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}
#[acmd_script(
    agent = "shizue",
    script =  "game_attackairf",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER,smash::phx::Hash40::new("fire"),false,0.0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::SET_SPEED_EX(fighter, -1.2, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.57, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 46.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}
#[acmd_script(
    agent = "shizue",
    script =  "sound_attackairf",
    category = ACMD_SOUND, low_priority )]
unsafe fn isa_fair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
			macros::STOP_SE(fighter, Hash40::new("se_shizue_smash_s02"));
		}
		wait(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_shizue_smash_s01"));
		}
}
#[acmd_script(
    agent = "shizue",
    script =  "effect_attackairf",
    category = ACMD_EFFECT, low_priority )]
unsafe fn isa_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
			macros::STOP_SE(fighter, Hash40::new("se_shizue_smash_s02"));
		}
		wait(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_shizue_smash_s01"));
		}
}
#[acmd_script(
    agent = "shizue",
    script =  "game_landingairf",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_fairland(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
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
#[status_script(agent = "shizue", status = FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

#[fighter_frame( agent = FIGHTER_KIND_SHIZUE )]
fn shizue_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
			ISA_RESHOOT_TIME[ENTRY_ID] = 0;
			ISA_SHOT_KIND[ENTRY_ID] = 1;
		};
		if ISA_RESHOOT_TIME[ENTRY_ID] > 0 {
			ISA_RESHOOT_TIME[ENTRY_ID] -= 1;
		};
		if status_kind == *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE {
				if motion_duration(boma) == 5 {
					if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
						PostureModule::reverse_lr(boma);
						PostureModule::update_rot_y_lr(boma);
						let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
						KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
				};
		};
		if ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR].contains(&status_kind) {
			ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
		if ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_CLIFF_ATTACK].contains(&status_kind) {
			ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POMPON,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
		if [hash40("special_air_hi"), hash40("special_hi"), hash40("special_air_hi_wait1"), hash40("special_air_hi_wait2"), hash40("special_air_hi_flap1"), hash40("special_air_hi_flap2"), hash40("special_air_hi_flap1")].contains(&MotionModule::motion_kind(boma)) {
			if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
				ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				ArticleModule::remove(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
			};
			if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR){
				ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				ArticleModule::remove(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
			};
		};
		if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
			if MotionModule::frame(boma) >= 28.0 {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
				};
			};
			if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
				let cat2 = ControlModule::get_command_flag_cat(boma, 1);
				if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
					WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
				}
			};
		};
		if ISA_RESHOOT_TIME[ENTRY_ID] < 1{
			if [
				*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH, 
				*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_S4_START, 
				*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, 
				*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END,
				*FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HOOK, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_REEL, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_START, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_THROW, 
				*FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_PICKUP, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CATCH_WAIT, *FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_LANDING,
				*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_CATCH_ATTACK
			].contains(&status_kind) {
				if ControlModule::get_stick_y(boma) <= -0.5 && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					ArticleModule::shoot_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
					ISA_RESHOOT_TIME[ENTRY_ID] = 130;
				};
			};
		};
	};
}		

pub fn install() {
    smashline::install_acmd_scripts!(
		isa_uthrow,
		isa_fthrow,
		isa_jab,
		isa_lloid_end,
		isa_lloid_set,
		isa_fair,
		isa_fairland,
		isa_fair_eff,
		isa_fair_snd,
		isa_bair,
		isa_bairland,
		isa_bair_eff,
		isa_bair_snd,
		isa_da,
		isa_neutralb,
		isa_neutralb_eff,
		isa_neutralb_snd,
		isa_neutralb_hit,
		isa_item_burst_eff,
		isa_item_burst_snd,
		isa_dair,
		isa_dair_land
    );
    smashline::install_agent_frames!(
        shizue_frame
    );
	//smashline::install_status_scripts!(special_n_pre);
}
