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
use smash::phx::Vector2f;
use super::*;
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 5.0, z: 0.0 };

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
unsafe extern "C" fn dsamus_upb_eff(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, 0.5, 0, 0, 0, 2, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 2, true);
			macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::LAST_EFFECT_SET_RATE(fighter, 1.0);
			macros::LAST_EFFECT_SET_ALPHA(fighter, 1.0);	
			macros::EFFECT(fighter, Hash40::new("sys_magicball_warp_disappear"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
			VisibilityModule::set_whole(fighter.module_accessor, false);
			EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40::new("samusd_win3_aura"), false, false);
		}
		frame(fighter.lua_state_agent, 50.0);
		if macros::is_excute(fighter) {
    		macros::EFFECT(fighter, Hash40::new("sys_magicball_warp_disappear"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 52.0);
		if macros::is_excute(fighter) {
    		VisibilityModule::set_whole(fighter.module_accessor, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, 0.5, 0, 0, 0, 2, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 2, true);
        }
		frame(fighter.lua_state_agent, 79.0);
		if macros::is_excute(fighter) {
			macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
			macros::BURN_COLOR_NORMAL(fighter);
			EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40::new("samusd_win3_aura"), false, false);
		}
}		
unsafe extern "C" fn dsamus_upb(fighter: &mut L2CAgentBase) {
		macros::FT_MOTION_RATE(fighter, 0.5);
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {	
			StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
            JostleModule::set_status(fighter.module_accessor, false);
			notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
		frame(fighter.lua_state_agent, 15.0);
		macros::FT_MOTION_RATE(fighter, 0.5);
		if macros::is_excute(fighter) {	
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
		}
		frame(fighter.lua_state_agent, 51.0);
		macros::FT_MOTION_RATE(fighter, 1.0);
		frame(fighter.lua_state_agent, 55.0);
		if macros::is_excute(fighter) {
			let stop_rise  = smash::phx::Vector3f { x: 0.5, y: 0.5, z: 1.0 };
			KineticModule::mul_speed(fighter.module_accessor, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		}		
}		
unsafe extern "C" fn dsamus_upb_snd(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_critical_l_tail"));
		}
		frame(fighter.lua_state_agent, 52.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_critical_m_tail"));
		}			
}				
unsafe extern "C" fn dsamus_downb(fighter: &mut L2CAgentBase) {
}					

pub fn install() {
    Agent::new("samusd")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
        .acmd("game_specialhi", dsamus_upb, Priority::Low)
        .acmd("effect_specialhi", dsamus_upb_eff, Priority::Low)
        .acmd("sound_specialhi", dsamus_upb_snd, Priority::Low)
        .acmd("game_specialairhi", dsamus_upb, Priority::Low)
        .acmd("effect_specialairhi", dsamus_upb_eff, Priority::Low)
        .acmd("sound_specialairhi", dsamus_upb_snd, Priority::Low)
        .acmd("effect_special", dsamus_downb, Priority::Low)
        .acmd("effect_specialair", dsamus_downb, Priority::Low)
        .acmd("expression_special", dsamus_downb, Priority::Low)
        .acmd("expression_specialair", dsamus_downb, Priority::Low)
        .acmd("sound_special", dsamus_downb, Priority::Low)
        .acmd("sound_specialair", dsamus_downb, Priority::Low)
        .install();

	Agent::new("samusd_missile")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
		.game_acmd("game_homing", dsamus_homing, Priority::Low)
		.sound_acmd("sound_homing", sound_dsamus_homing, Priority::Low)
		.effect_acmd("effect_special", dsamus_special_eff, Priority::Low)
		.effect_acmd("effect_specialair", dsamus_special_eff, Priority::Low)
		.effect_acmd("effect_hburst", eff_dsamus_burst, Priority::Low)
		.effect_acmd("effect_homing", eff_dsamus_homing, Priority::Low)
		.install();
}