use smash::app::sv_animcmd::*;
use smash::app::sv_module_access::*;
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
	
#[acmd_script(
    agent = "kamui",
    scripts =  ["game_attack11"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn corrin_jab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND,smash::phx::Hash40::new("attack_11"),false,0.0);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 67, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 67, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 2.2, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 10.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 9.5, /*Angle*/ 72, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 1.8, /*X*/ 8.0, /*Y*/ 0.0, /*Z*/ 0.4, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 9.5, /*Angle*/ 72, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 1.8, /*X*/ 8.0, /*Y*/ 0.0, /*Z*/ 0.4, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 9.5, /*Angle*/ 72, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 1.8, /*X*/ 13.0, /*Y*/ 0.0, /*Z*/ 0.4, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 9.5, /*Angle*/ 72, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 1.8, /*X*/ 13.0, /*Y*/ 0.0, /*Z*/ 0.4, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
#[acmd_script(
    agent = "kamui",
    scripts =  ["game_attacairn"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn corrin_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND,smash::phx::Hash40::new("attack_air_n"),false,0.0);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ -0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.5, /*X*/ 3.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.0, /*X*/ 7.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 5.5, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ -0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 5.5, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.5, /*X*/ 3.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 5.5, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.0, /*X*/ 7.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 5.5, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 5.5, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 5.5, /*Angle*/ 70, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		wait(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 47.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}	
#[acmd_script(
    agent = "kamui",
    scripts =  ["game_attacklw3"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn corrin_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 7.5, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 7.5, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.0, /*X*/ -1.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 7.5, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.0, /*X*/ -1.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "kamui",
    scripts =  ["game_attackdash"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn corrin_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
		frame(fighter.lua_state_agent, 12.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.0, /*Angle*/ 14, /*KBG*/ 100, /*FKB*/ 65, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.0, /*Angle*/ 14, /*KBG*/ 100, /*FKB*/ 25, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.0, /*Angle*/ 14, /*KBG*/ 100, /*FKB*/ 5, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		wait(fighter.lua_state_agent, 1.0);
		for _ in 0..4 {
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.0, /*Angle*/ 14, /*KBG*/ 100, /*FKB*/ 65, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.0, /*Angle*/ 14, /*KBG*/ 100, /*FKB*/ 25, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.0, /*Angle*/ 14, /*KBG*/ 100, /*FKB*/ 5, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}
			wait(fighter.lua_state_agent, 1.0);
		}
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.0, /*Angle*/ 60, /*KBG*/ 160, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.0, /*Angle*/ 60, /*KBG*/ 160, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.0, /*Angle*/ 60, /*KBG*/ 160, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
#[acmd_script(
    agent = "kamui",
    scripts =  ["effect_dash", "effect_turndash"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn corrin_dash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script( 
	agent = "kamui", 
	script = "effect_run", 
	category = ACMD_EFFECT,
	low_priority)]
unsafe fn corrin_run_eff(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_sibuki_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}
#[acmd_script(
    agent = "kamui",
    scripts =  ["effect_turnrun"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn corrin_turnrun_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script(
    agent = "kamui",
    scripts =  ["effect_runbrakel", "effect_runbraker"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn corrin_runbrake_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), 7.5, 0, 0, 0, 180, 0, 0.62, 0, 0, 0, 0, 0, 0, false);
		}
		wait(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), 7.5, 0, 0, 0, 180, 0, 0.62, 0, 0, 0, 0, 0, 0, false);
		}
		wait(fighter.lua_state_agent, 6.0);
}
#[acmd_script(
    agent = "kamui",
    scripts =  ["effect_attackhi4"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn corrin_usmash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFFECT_SWORD_AURA);
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_attack_line_hi4"), Hash40::new("handr"), -3, 0, 0, 0, 90, 0, 0.65, true);
			macros::EFFECT(fighter, Hash40::new("kamui_attack_flash"), Hash40::new("top"), 0, 27, 8, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("kamui_attack_line_hi4"), false, false);
		}
}
#[acmd_script(
    agent = "kamui",
    scripts =  ["sound_attackhi4"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn corrin_usmash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_kamui_special_h01"));
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kamui_rnd_special_h"));
			macros::PLAY_SE(fighter, Hash40::new("se_kamui_special_n07"));
			macros::PLAY_SE(fighter, Hash40::new("se_kamui_horn_start"));
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_kamui_horn_end"));
		}
}	
#[acmd_script(
    agent = "kamui",
    scripts =  ["expression_attackhi4"],
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn corrin_usmash_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFFECT_SWORD_AURA);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
		}
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFFECT_SWORD_AURA);
		}
}		
#[acmd_script(
    agent = "kamui",
    scripts =  ["game_attackhi4"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn corrin_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2);
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFFECT_SWORD_AURA);
			macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(3.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
		}
		frame(fighter.lua_state_agent, 4.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		if macros::is_excute(fighter) {
			macros::SET_SPEED_EX(fighter, 2.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		}
		frame(fighter.lua_state_agent, 8.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2);
		if macros::is_excute(fighter) {
			macros::SET_SPEED_EX(fighter, 0.3, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 10.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		frame(fighter.lua_state_agent, 16.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(27.0), /*Z2*/ Some(9.0), /*Hitlag*/ 2.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 17.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 29.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFFECT_SWORD_AURA);
		}
}			

#[acmd_script(
    agent = "kamui",
    script =  "game_throwb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn corrin_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 95, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 25, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 8.0, /*Angle*/ 60, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.5, /*X*/ 10.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-2.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 12.0, /*Angle*/ 60, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.0, /*X*/ 20.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-2.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::REVERSE_LR(fighter);
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
			AttackModule::clear_all(fighter.module_accessor);
		}
}

#[acmd_script(
    agent = "kamui_ryusensya",
    script =  "game_regular",
    category = ACMD_GAME,
	low_priority)]
unsafe fn corrin_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 135, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_NONE);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 135, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_NONE);
			attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
			WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
			macros::ATK_LERP_RATIO(fighter, 1026626521);
			AttackModule::enable_safe_pos(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "kamui_ryusensya",
    script =  "game_shotmax",
    category = ACMD_GAME,
	low_priority)]
unsafe fn corrin_neutralb_max(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 125, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_NONE);
		}
}
#[acmd_script(
    agent = "kamui_waterdragon",
    scripts =  ["game_speciallwhit", "game_specialairlwhit", "game_speciallwhitturn", "game_specialairlwhitturn"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn corrin_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_water"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_water"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
			AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
			AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 90, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_water"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 90, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ -11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_water"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
			AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
			AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
		}
		frame(fighter.lua_state_agent, 31.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]
fn kamui_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if status_kind == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP){
				if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
				};
		};
		if [*FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) {
			EffectModule::kill_kind(boma, smash::phx::Hash40::new("sys_smash_flash"), false, false);
			EffectModule::kill_kind(boma, smash::phx::Hash40::new("sys_smash_flash_s"), false, false);
		};
		if [*FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) {
			if MotionModule::frame(boma) > 8.0 && MotionModule::frame(boma) < 33.0 {
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_rspearhand"),true);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_weapon"),false);
			} else {
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_rspearhand"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_weapon"),true);
			};
		};
		if (StopModule::is_hit(boma) || StopModule::is_damage(boma) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) == false) || smash::app::sv_information::is_ready_go() == false {
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_rspearhand"),false);
			if [*FIGHTER_STATUS_KIND_LOSE].contains(&status_kind) {
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_weapon"),false);
			} else {
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("kamui_weapon"),true);
			};
		};
    }
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
		corrin_jab,
		corrin_da,
		corrin_usmash,
		corrin_dtilt,
		corrin_nair,
		corrin_usmash_effect,
		corrin_usmash_expr,
		corrin_usmash_sound,
		corrin_run_eff,
		corrin_dash_eff,
		corrin_turnrun_eff,
		corrin_runbrake_eff,
		corrin_throwb,
		corrin_neutralb,
		corrin_neutralb_max,
		corrin_downb
    );
    smashline::install_agent_frames!(
        kamui_frame
    );
}
