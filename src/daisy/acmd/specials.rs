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
pub fn install() {
	smashline::install_acmd_scripts!(
		daisy_downb,
		daisy_downb_eff,
		daisy_upb,
		daisy_neutralb,
		daisy_neutralb_eff,
		daisy_neutralb_hit,
		daisy_neutralb_hit_eff,
		kirby_daisy_neutralb_eff,
		kirby_daisy_neutralb_hit_eff
	);
}
#[acmd_script(
    agent = "daisy",
    script =  "game_speciallw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		frame(fighter.lua_state_agent, 14.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON, false, -1);
		}
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_speciallw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn daisy_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 8, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["game_specialhistart", "game_specialairhistart"],
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
			if macros::is_excute(fighter) {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, 0);
				ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR,smash::phx::Hash40::new("special_hi_start"),false,0.0);
			}
			frame(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS);
			}
			frame(fighter.lua_state_agent, 7.0);
			if macros::is_excute(fighter) {
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PARASOL);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PARASOL);
			}
			frame(fighter.lua_state_agent, 11.0);
			if macros::is_excute(fighter) {
				AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_PARASOL);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 7.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 1.5, /*X*/ 2.0, /*Y*/ 5.0, /*Z*/ 3.5, /*X2*/ Some(2.0), /*Y2*/ Some(2.5), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_PARASOL);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 7.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-1.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_PARASOL);
			}
			frame(fighter.lua_state_agent, 33.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}			
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
		if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_SPECIAL_N_RAISE) {
			KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_AIR_N_RAISE);
		};
	};
	frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
	};
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["game_specialnhit", "game_specialairnhit", "game_specialnturn", "game_specialairnturn"],
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_neutralb_hit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, false);
	};
    frame(fighter.lua_state_agent, 27.0);
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, true);
		AttackModule::clear_all(fighter.module_accessor);
	};
    frame(fighter.lua_state_agent, 45.0);
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["effect_specialn", "effect_specialairn"],
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6, 7, 0, 0, 0, 1.6, true);
		}
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::FLASH(fighter, 1, 1, 1, 0.75);
		}
		wait(fighter.lua_state_agent, 1.0);
		for _ in 0..3 {
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
			}
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 0.67, 0, 0.78, 0.31);
			}
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				macros::COL_NORMAL(fighter, );
			}
			wait(fighter.lua_state_agent, 2.0);
		}
		if macros::is_excute(fighter) {
			macros::FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::COL_NORMAL(fighter, );
		}
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_daisyspecialn", "effect_daisyspecialairn"],
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn kirby_daisy_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6, 7, 0, 0, 0, 1.6, true);
		}
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::FLASH(fighter, 1, 1, 1, 0.75);
		}
		wait(fighter.lua_state_agent, 1.0);
		for _ in 0..3 {
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
			}
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 0.67, 0, 0.78, 0.31);
			}
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				macros::COL_NORMAL(fighter, );
			}
			wait(fighter.lua_state_agent, 2.0);
		}
		if macros::is_excute(fighter) {
			macros::FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::COL_NORMAL(fighter, );
		}
}

#[acmd_script(
    agent = "daisy",
    scripts =  ["effect_specialnhit", "effect_specialairnhit", "effect_specialnturn", "effect_specialairnturn"],
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_neutralb_hit_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
			macros::LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 210.0/255.0, 46.0/255.0);
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 7, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_daisyspecialnhit", "effect_daisyspecialairnhit", "effect_daisyspecialnturn", "effect_daisyspecialairnturn"],
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn kirby_daisy_neutralb_hit_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
			macros::LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 210.0/255.0, 46.0/255.0);
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 7, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
		}
}