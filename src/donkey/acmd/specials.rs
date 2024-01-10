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

use crate::donkey::*;

pub fn install() {
	smashline::install_acmd_scripts!(
		dk_sideb, 
		dk_sideb_eff,
		dk_sideb_snd, 
		dk_sideb_expr,
        dk_upb, dk_upb_eff, dk_upb_shoot, dk_upb_shoot_eff, dk_upb_snd, dk_upb_shoot_snd
	);
}

#[acmd_script(
    agent = "donkey",
    scripts =  ["game_specialairlw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dk_air_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 36, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 28.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 6.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.0, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ -2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["game_specialairs", "game_specials"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dk_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
        IS_DK_START_ITEM_CHUCK[ENTRY_ID] = true;
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        ItemModule::throw_item(fighter.module_accessor, 22.5, 4.0, 1.0, 0, true, 0.0);
    } 
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        IS_DK_START_ITEM_CHUCK[ENTRY_ID] = false;
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["game_specialairhi", "game_specialhi"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dk_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        damage!(fighter, /*MSC=*/*MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, /*DamageThreshold=*/7);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ItemModule::remove_item(boma, 0);
        ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["effect_specialairhi", "effect_specialhi"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dk_upb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_item_arrival"), Hash40::new("top"), 0, 13, -1.5, 0, 0, 0, 1.35, 0, 0, 0, 0, 0, 0, true);
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["sound_specialairhi", "sound_specialhi"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dk_upb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_assist_escapeair"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_down_l_02"));
        macros::STOP_SE(fighter, Hash40::new("se_item_item_get"));
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["game_specialhishoot"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dk_upb_shoot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ItemModule::remove_item(boma, 0);
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 11.0, /*Angle*/ 45, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 10.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
    }
} 
#[acmd_script(
    agent = "donkey",
    scripts =  ["effect_specialhishoot"],
    category = ACMD_EFFECT,
	low_priority)] 
unsafe fn dk_upb_shoot_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_box00_break"), Hash40::new("top"), -2, 6, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_box00_break"), Hash40::new("top"), 0, 8, 0, 0, 0, 180, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["sound_specialhishoot"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dk_upb_shoot_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        macros::PLAY_SE(fighter, Hash40::new("se_donkey_appear01"));
    }
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["effect_specialairs", "effect_specials"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dk_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1356a16dd8), Hash40::new("top"), 0, 12, -3, 0, -75, -85, 0.8, true);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
		}
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["sound_specialairs", "sound_specials"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dk_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
		frame(fighter.lua_state_agent, 29.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
			macros::PLAY_SE(fighter, Hash40::new("se_donkey_attackhard_s01"));
		}		
}
#[acmd_script(
    agent = "donkey",
    scripts =  ["expression_specialairs", "expression_specials"],
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dk_sideb_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
}