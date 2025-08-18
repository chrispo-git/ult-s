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

unsafe extern "C" fn toad_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	macros::FT_MOTION_RATE(fighter, 0.5);
	KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
}	
unsafe extern "C" fn toad_sideb_hitbox(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	macros::FT_MOTION_RATE(fighter, 1.0);
	KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("throw"), /*Damage*/ 2.0, /*Angle*/ 361, /*KBG*/ 123, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X*/ None, /*Y*/ None, /*Z*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -8, /*Trip*/ 0.0, /*Rehit*/ 12, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
	};	
}	
unsafe extern "C" fn toad_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_l"));
	};
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_step_snow"));
	};
}	
unsafe extern "C" fn toad_sideb_loop_snd(fighter: &mut L2CAgentBase) {
		for _ in 0..i32::MAX {
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_sand"));
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_snow"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_sand"));
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_snow"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_sand"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_sand"));
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_sand"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_sand"));
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_snow"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_snow"));
			}
			wait(fighter.lua_state_agent, 1.0);
		}
}
unsafe extern "C" fn toad_sideb_air_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_l"));
	};
}	

unsafe extern "C" fn toad_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT, true, 0);
			ArticleModule::shoot(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
		}
}	
unsafe extern "C" fn toad_iceball(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("have"), 7.0, 361, 20, 0, 28, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("have"), 6.0, 361, 20, 0, 23, 1.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("have"), 6.0, 361, 20, 0, 28, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("have"), 5.0, 361, 20, 0, 23, 1.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -1.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("have"), 5.0, 361, 20, 0, 28, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -1.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("have"), 4.0, 361, 20, 0, 23, 1.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
    }
}	
unsafe extern "C" fn toad_iceball_eff(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
		macros::EFFECT_FOLLOW(agent, Hash40::new("popo_iceshot_cold_a"), Hash40::new("top"), 0, -0.5, 0, 0, 0, 0, 1.0, true);
	}
}	
unsafe extern "C" fn toad_iceball_end_eff(agent: &mut L2CAgentBase) {
}	
unsafe extern "C" fn toad_neutralb_eff(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("popo_iceshot_cold_a"), Hash40::new("handl"), 0, 0.0, 0, 0, 0, 0, 0.4, true);
	}
	frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("popo_iceshot_cold_a"), false, true);
	}
}	
unsafe extern "C" fn toad_neutralb_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_special_h02"));
	};
}	

unsafe extern "C" fn toad_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			KineticModule::clear_speed_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			if true{
				let x_speed = PostureModule::lr(fighter.module_accessor) * ControlModule::get_stick_x(fighter.module_accessor) * 0.75;
				macros::SET_SPEED_EX(fighter, x_speed, 3.35, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			}
			notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07 as u64), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07u64), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 150, /*BKB*/ 0, /*Size*/ 6.7, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(3.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(3.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(3.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 89, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 7.7, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(3.0), /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 89, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 7.7, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(3.0), /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
	}
	unsafe extern "C" fn toad_upb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_smash_s01"));
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_smash_s02"));
		}
	}
	unsafe extern "C" fn toad_upb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
		}
	}
	unsafe extern "C" fn toad_upb_screw(fighter: &mut L2CAgentBase) {
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 6.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 160, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 6.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 160, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BODY);
		}
	}
	unsafe extern "C" fn toad_upb_screw_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		for _ in 0..i32::MAX {
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.0, 0, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
				macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
				macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
				macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.0, 0, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
				macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2.0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
				macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
			}
			wait(fighter.lua_state_agent, 10.0);
		}
		frame(fighter.lua_state_agent, 1.0);
	}
	unsafe extern "C" fn toad_upb_screw_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackdash02"));
		}
	}

	unsafe extern "C" fn toad_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 8.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 8.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 55.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
	}
	unsafe extern "C" fn toad_downb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
unsafe extern "C" fn toad_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2.5, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2.5, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 33.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2.5, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 46.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2.5, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
		}
	}
unsafe extern "C" fn toad_downb_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			macros::SET_SPEED_EX(fighter, 0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			macros::SET_SPEED_EX(fighter, 0, -4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 280, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 280, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 12.0, /*Angle*/ 280, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 90, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 90, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 12.0, /*Angle*/ 90, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 37.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 48.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
			KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		}
}
unsafe extern "C" fn toad_downb_air_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 20.0);
		for _ in 0..8  
 		{
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 24, 1, 90, 0, 0, 1.4, true, *EF_FLIP_YZ, 0.3);
			}
			wait(fighter.lua_state_agent, 2.0);
		}
}
unsafe extern "C" fn toad_downb_air_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackdash03"));
		}
}
pub fn install() {
	Agent::new("murabito_flowerpot")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
		.acmd("game_throwed", toad_iceball, Priority::Low)
		.acmd("effect_throwed", toad_iceball_eff, Priority::Low)
		.acmd("game_bound", toad_iceball, Priority::Low)
		.acmd("effect_bound", toad_iceball_eff, Priority::Low)
		.acmd("effect_burst", toad_iceball_end_eff, Priority::Low)
		.install();
	Agent::new("murabito")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
		.effect_acmd("effect_specialairhiscrew", toad_upb_screw_eff, Priority::Low)
		.game_acmd("game_specialairhiscrew", toad_upb_screw, Priority::Low)
		.sound_acmd("sound_specialairhiscrew", toad_upb_screw_snd, Priority::Low)
		.acmd("game_specialsloop", toad_sideb_hitbox, Priority::Low)
		.acmd("sound_specialsloop", toad_sideb_loop_snd, Priority::Low)
		.acmd("game_specialairsloop", toad_sideb_hitbox, Priority::Low)
		.acmd("game_specialsjump", toad_sideb_hitbox, Priority::Low)
		.game_acmd("game_specialstoad", toad_sideb, Priority::Low)
		.game_acmd("game_specialairstoad", toad_sideb, Priority::Low)
		.sound_acmd("sound_specialstoad", toad_sideb_snd, Priority::Low)
		.sound_acmd("sound_specialairstoad", toad_sideb_air_snd, Priority::Low)
		.game_acmd("game_specialntoad", toad_neutralb, Priority::Low)
		.game_acmd("game_specialairntoad", toad_neutralb, Priority::Low)
		.effect_acmd("effect_specialntoad", toad_neutralb_eff, Priority::Low)
		.effect_acmd("effect_specialairntoad", toad_neutralb_eff, Priority::Low)
		.sound_acmd("sound_specialntoad", toad_neutralb_snd, Priority::Low)
		.sound_acmd("sound_specialairntoad", toad_neutralb_snd, Priority::Low)
		.game_acmd("game_specialairhidetachtoad", toad_upb, Priority::Low)
		.sound_acmd("sound_specialairhidetachtoad", toad_upb_snd, Priority::Low)
		.effect_acmd("effect_specialairhidetachtoad", toad_upb_eff, Priority::Low)
		.game_acmd("game_specialairlw2toad", toad_downb, Priority::Low)
		.game_acmd("game_speciallw2toad", toad_downb, Priority::Low)
		.sound_acmd("sound_specialairlw2toad", toad_downb_snd, Priority::Low)
		.sound_acmd("sound_speciallw2toad", toad_downb_snd, Priority::Low)
		.effect_acmd("effect_specialairlw2toad", toad_downb_eff, Priority::Low)
		.effect_acmd("effect_speciallw2toad", toad_downb_eff, Priority::Low)
		.game_acmd("game_specialairlw1failuretoad", toad_downb_air, Priority::Low)
		.sound_acmd("sound_specialairlw1failuretoad", toad_downb_air_snd, Priority::Low)
		.effect_acmd("effect_specialairlw1failuretoad", toad_downb_air_eff, Priority::Low)
		.install();
}