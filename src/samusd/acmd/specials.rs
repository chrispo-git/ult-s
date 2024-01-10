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

#[acmd_script(
    agent = "samusd_missile",
    script =  "game_homing",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_homing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 10.0, /*Angle*/ 60, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(4.0), /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BODY);
		}
		wait(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 6.0, /*Angle*/ 60, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BODY);
		}
		wait(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "samusd_missile",
    script =  "sound_homing",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sound_dsamus_homing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_samusd_special_n04"));
		}
}		

#[acmd_script(
    agent = "samusd",
    scripts =  ["effect_special", "effect_specialair"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_special_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}		
	
#[acmd_script(
    agent = "samusd_missile",
    script =  "effect_hburst",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn eff_dsamus_burst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}		
#[acmd_script(
    agent = "samusd_missile",
    script =  "effect_homing",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn eff_dsamus_homing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			ModelModule::set_alpha(fighter.module_accessor, 0.0);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			ModelModule::set_alpha(fighter.module_accessor, 0.33);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			ModelModule::set_alpha(fighter.module_accessor, 0.66);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			ModelModule::set_alpha(fighter.module_accessor, 1.0);
		}
		wait(fighter.lua_state_agent, 36.0);
		if macros::is_excute(fighter) {
			ModelModule::set_alpha(fighter.module_accessor, 0.66);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			ModelModule::set_alpha(fighter.module_accessor, 0.33);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			ModelModule::set_alpha(fighter.module_accessor, 0.0);
		}
}		
#[acmd_script(
    agent = "samusd",
    script =  "effect_speciallw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn eff_dsamus_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}	
#[acmd_script(
    agent = "samusd",
    script =  "expression_speciallw",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn expr_dsamus_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}		
#[acmd_script(
    agent = "samusd",
    script =  "game_speciallw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}	
#[acmd_script(
    agent = "samusd",
    scripts =  ["game_special", "game_specialair"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_sideb(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
		if  !ArticleModule::is_exist(fighter.module_accessor, *WEAPON_KIND_SAMUSD_MISSILE) {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
		}
    }
	
}	


pub fn install() {
    smashline::install_acmd_scripts!(
		dsamus_homing, sound_dsamus_homing,
        dsamus_special_eff,
        eff_dsamus_burst,
        eff_dsamus_homing,
        eff_dsamus_downb, expr_dsamus_downb,
        dsamus_downb,
		dsamus_sideb
    );
}