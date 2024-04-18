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

unsafe extern "C" fn dsamus_homing(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn sound_dsamus_homing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_samusd_special_n04"));
		}
}		

unsafe extern "C" fn dsamus_special_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}		
	
unsafe extern "C" fn eff_dsamus_burst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}		
unsafe extern "C" fn eff_dsamus_homing(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn eff_dsamus_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}	
unsafe extern "C" fn expr_dsamus_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}		
unsafe extern "C" fn dsamus_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}	

pub fn install() {
    Agent::new("samusd")
        .effect_acmd("effect_speciallw", eff_dsamus_downb)
        .expression_acmd("expression_speciallw", expr_dsamus_downb)
        .game_acmd("game_speciallw", dsamus_downb)
        .install();

	Agent::new("samusd_missile")
		.game_acmd("game_homing", dsamus_homing)
		.sound_acmd("sound_homing", sound_dsamus_homing)
		.effect_acmd("effect_special", dsamus_special_eff)
		.effect_acmd("effect_specialair", dsamus_special_eff)
		.effect_acmd("effect_hburst", eff_dsamus_burst)
		.effect_acmd("effect_homing", eff_dsamus_homing)
		.install();
}