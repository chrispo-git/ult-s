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
use crate::kirby::*;
pub fn install() {
    smashline::install_acmd_scripts!(
		kirby_upb1,
		kirby_air_upb2,
		kirby_air_upb2_effect,
		kirby_upb2_sound,
		kirby_upb_sound,
		kirby_upb4,
		kirby_ground_downb,
		kirby_ground_downb_eff,
		kirby_ground_downb_snd,
		kirby_ground_downb2,
		kirby_ground_downb2_eff,
		kirby_ground_downb2_snd,
		kirby_upb4_exp,
		kirby_upb4_sound,
		kirby_beam_eff,
		kirby_beam,
		kirby_sideb,
		kirby_sideb_eff,
		kirby_sideb_expr,
		kirby_sideb_start,
		kirby_sideb_snd,
		kirby_downb_end_air,
		kirby_special_input,
		kirby_special_input_snd,
		kirby_special_input_eff
    );
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["game_specialhi", "game_specialairhi"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_upb1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			if true{
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
				StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, true);
			}
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_specialhi4"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_upb4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "kirby",
    scripts =  ["expression_specialhi4"],
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn kirby_upb4_exp(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "kirby",
    script =  "game_specialhi4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_upb4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "kirby",
    script =  "effect_specialairhi2",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_air_upb2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
		}
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_attack_arc"), Hash40::new("kirby_attack_arc"), Hash40::new("top"), -20.0, 6, 0, 0, 0, 90, 1.0, true, *EF_FLIP_NONE);
			macros::LAST_EFFECT_SET_COLOR(fighter, 2.016, 0.648, 1.536);
			macros::LAST_EFFECT_SET_RATE(fighter, 0.275);
		}
		frame(fighter.lua_state_agent, 3.0);
		for _ in 0..3 {
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
				if true{
					let boma = fighter.module_accessor;
					let distance2 = smash::phx::Vector3f { x: 0.0, y: 6.0, z: 7.0 };
					let empty = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
					let fire: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &distance2, &empty, 0.65, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, fire, 1.0, 0.25, 0.25);
					EffectModule::set_alpha(boma, fire, 0.5);
					EffectModule::set_rate(boma, fire, 0.5);
					let fire2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &distance2, &empty, 0.45, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, fire2, 3.0, 0.5, 0.5);
					EffectModule::set_rate(boma, fire2, 0.5);
					EffectModule::set_rate(boma, fire, 0.75);
				}
			}
			wait(fighter.lua_state_agent, 2.0);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_fire"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_fireflower_shot"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("kirby_attack_arc"), false, true);
		}
}
		
#[acmd_script(
    agent = "kirby",
    script =  "game_specialairhi2",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_air_upb2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 4.9, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(7.9), /*Hitlag*/ 0.7, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 2, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 4.9, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(7.9), /*Hitlag*/ 0.7, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 2, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 4.9, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(7.9), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 2, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 4.9, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(7.9), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 2, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 83, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ -1.5, /*Z*/ 4.9, /*X2*/ Some(0.0), /*Y2*/ Some(4.8), /*Z2*/ Some(4.9), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			if true{
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			}
		}
}			
#[acmd_script(
    agent = "kirby",
    script =  "sound_specialairhi2",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_upb2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, /*Frames*/ 1.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_kirby_attack07"));
		}
}		



#[acmd_script(
    agent = "kirby",
    script =  "game_speciallw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_ground_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 65, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    	}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 65, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    	}
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "kirby",
    script =  "effect_speciallw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_ground_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
		}
}	
#[acmd_script(
    agent = "kirby",
    script =  "sound_speciallw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_ground_downb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kirby_rnd_attack"));
		}
}	
#[acmd_script(
    agent = "kirby",
    script =  "game_speciallw2",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_ground_downb2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
		}
}
#[acmd_script(
    agent = "kirby",
    script =  "effect_speciallw2",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_ground_downb2_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
		}
}	
#[acmd_script(
    agent = "kirby",
    script =  "sound_speciallw2",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_ground_downb2_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		
}	

#[acmd_script(
    agent = "kirby",
    script =  "game_specialairlw2",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_downb_end_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE,smash::phx::Hash40::new("special_air_lw2"),false,0.0);
			macros::SET_SPEED_EX(fighter, 0.0, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		}
}	
#[acmd_script(
    agent = "kirby_finalcuttershot",
    script =  "effect_finalcutterregular",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_beam_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "kirby_finalcuttershot",
    script =  "game_finalcutterregular",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_beam(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 140, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_ENERGY);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 140, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_ENERGY);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["game_specialsstart", "game_specialairsstart"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_sideb_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["game_specials", "game_specialairs", "game_specialss", "game_specialairss"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::phx::Hash40::new("special_hi3"),false,0.0);
		}
		frame(fighter.lua_state_agent, /*Frames*/ 17.0);
		if macros::is_excute(fighter) {
			macros::SET_SPEED_EX(fighter, -1.0, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTERSHOT, false, 0);
		}
		frame(fighter.lua_state_agent, 46.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_specials", "effect_specialairs", "effect_specialss", "effect_specialairss"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_magic"), Hash40::new("have"), 0.0, 5.2, 13.5, 0, 0, 0, 0.3, true);
		}
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_specials", "sound_specialairs", "sound_specialss", "sound_specialairss"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_kirby_002"));
		}	
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_swing_l"));
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_frieze_l"));
			macros::PLAY_SE(fighter, Hash40::new("se_common_sleep"));
		}
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["expression_specials", "expression_specialairs", "expression_specialss", "expression_specialairss"],
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn kirby_sideb_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_specialhi", "sound_specialairhi"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_upb_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, /*Frames*/ 1.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kirby_rnd_attack"));
		}
}

#[acmd_script(
    agent = "kirby",
    scripts =  ["game_specialinput"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_special_input(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 12.2, /*Angle*/ 46, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 37.0);
		if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 45.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_specialinput"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_special_input_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_electric_hit_ll"));
		}
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_specialinput"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_special_input_eff(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_screw"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.5, true);
		macros::LAST_EFFECT_SET_COLOR(fighter, 0.0119, 1.0, 0.4);
		macros::LAST_EFFECT_SET_RATE(fighter, 0.45);
	}
	frame(fighter.lua_state_agent, 37.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_screw"), false, true);
	}
}