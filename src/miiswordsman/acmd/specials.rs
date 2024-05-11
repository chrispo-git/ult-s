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

pub fn install() {
    Agent::new("miiswordsman")
    .acmd("game_specialn2", sword_neutralb_2, Priority::Low)    
    .acmd("game_specialairn2", sword_air_neutralb_2, Priority::Low)    
    .acmd("effect_specialn2", sword_neutralb_2_eff, Priority::Low)    
    .acmd("effect_specialairn2", sword_neutralb_2_eff, Priority::Low)    
    .acmd("sound_specialn2", sword_neutralb_2_snd, Priority::Low)    
    .acmd("sound_specialairn2", sword_neutralb_2_snd, Priority::Low)    
    .acmd("game_specialhi3start", sword_hs_start, Priority::Low)    
    .acmd("game_specialn1", sword_nado, Priority::Low)    
    .acmd("game_specialairn1", sword_nado, Priority::Low)    
    .acmd("game_specials2attack", sword_gale_stab, Priority::Low)    
    .acmd("game_specialairs2attack", sword_air_gale_stab, Priority::Low)    
    .acmd("game_speciallw1hit", sword_counter, Priority::Low)    
    .acmd("game_specialairlw1hit", sword_counter, Priority::Low)    
    .acmd("game_specialhi1", sword_ss_rise, Priority::Low)    
    .acmd("game_specialairhi1", sword_ss_rise, Priority::Low)    
    .acmd("game_specialairn3start", sword_airgrab_start, Priority::Low)    
    .acmd("game_specialn3start", sword_airgrab_start, Priority::Low)    
    .acmd("effect_specialairn3start", sword_airgrab_start_eff, Priority::Low)    
    .acmd("effect_specialn3start", sword_airgrab_start_eff, Priority::Low)    
    .acmd("effect_specialairn3loop", sword_airgrab_eff, Priority::Low)    
    .acmd("effect_specialn3loop", sword_airgrab_eff, Priority::Low)    
    .acmd("effect_specialairn3end", sword_airgrab_end_eff, Priority::Low)    
    .acmd("effect_specialn3end", sword_airgrab_end_eff, Priority::Low)    
    .acmd("game_specialairn3loop", sword_airgrab, Priority::Low)    
    .acmd("game_specialairn3end", sword_airgrab_end, Priority::Low)    
    .acmd("sound_specialairn3end", sword_airgrab_end_snd, Priority::Low)    
    .acmd("expression_specialairn3end", sword_airgrab_end_expr, Priority::Low)    
    .acmd("game_specials1", sword_sideb1_start, Priority::Low)    
    .acmd("effect_specials1", sword_sideb1_start_eff, Priority::Low)    
    .acmd("effect_specialairs1", sword_sideb1_start_eff, Priority::Low)    
    .acmd("game_specials1hit", sword_sideb1, Priority::Low)    
    .acmd("game_specials1air", sword_sideb1_air_start, Priority::Low)    
    .acmd("game_specialairs1hit", sword_sideb1_air, Priority::Low)    
    .acmd("effect_specials1hit", sword_sideb1_effect, Priority::Low)    
	.acmd("sound_specials1hit", sword_sideb1_snd, Priority::Low)    
    .acmd("sound_specialairs1hit", sword_sideb1_snd, Priority::Low)    
	.acmd("effect_specialairs1hit", sword_sideb1_air_effect, Priority::Low)    
    .install();

	Agent::new("miiswordsman_tornadoshot")
    .acmd("game_fly", sword_nadoshot, Priority::Low)    
    .install();
}

unsafe extern "C" fn sword_neutralb_2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
    	macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 30, 8.5, 0.0, 6.5, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
	frame(agent.lua_state_agent, 17.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
}	
unsafe extern "C" fn sword_air_neutralb_2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
    	macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 30, 8.5, 0.0, 6.5, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
	frame(agent.lua_state_agent, 17.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
}	
unsafe extern "C" fn sword_neutralb_2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
		macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 6.5, 14.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
		macros::LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 6.5, 14.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
		macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 5.5, 15.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 9.5, 14.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) && !AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_ALL){
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 5.5, 13.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 9.5, 14.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) && !AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_ALL){
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 7.5, 15.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 3.5, 16.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) && !AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_ALL){
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 5.5, 10.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 7.5, 12.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) && !AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_ALL){
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 5.5, 9.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 7.5, 15.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
		macros::EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 6.5, 14.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
		macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_bomb_a"), false, true);
    }
}
unsafe extern "C" fn sword_neutralb_2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
		macros::PLAY_SE(agent, Hash40::new("se_common_bomb_s"));
		macros::PLAY_SE(agent, Hash40::new("se_common_electric_hit_ll"));
		macros::PLAY_SE(agent, Hash40::new("se_common_electric_fire_s"));
		macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack03"));
    }
}
unsafe extern "C" fn sword_nadoshot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		macros::AREA_WIND_2ND_RAD_arg9(fighter, 0, 2, 0.05, 200, 1, 3, 3, 25, 30);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 45, 100, 40, 0, 5.0, 0.0, 11.0, 1.2, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
    }
}
unsafe extern "C" fn sword_hs_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.25);
}

unsafe extern "C" fn sword_nado(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.7058823529411765);
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			if NADO_COOLDOWN[ENTRY_ID] <= 0 {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_TORNADOSHOT, false, 0);
				NADO_COOLDOWN[ENTRY_ID] = NADO_MAX;
			} else {
				macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 7, 14, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
			}
		}
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
}
unsafe extern "C" fn sword_gale_stab(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 5.0, 0.0, 5.5, 16.0, Some(0.0), Some(5.5), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 3.5, 0.0, 5.5, 15.0, Some(0.0), Some(5.5), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_ATTACK_END);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
		if !AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
			StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END, false);
		}
	}
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
unsafe extern "C" fn sword_air_gale_stab(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 5.0, 0.0, 5.5, 16.0, Some(0.0), Some(5.5), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 3.5, 0.0, 5.5, 15.0, Some(0.0), Some(5.5), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_ATTACK_END);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
		if !AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
			StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END, false);
		}
	}
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn sword_counter(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.4);
			if macros::is_excute(fighter) {
				WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
			}
			frame(fighter.lua_state_agent, 10.0);
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
			frame(fighter.lua_state_agent, 21.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 8.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(3.0), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
				if true{
					let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
					COUNTER_STORE[ENTRY_ID] = false;
				}
			}
			frame(fighter.lua_state_agent, 23.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
				WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
			}
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.65);
}		

unsafe extern "C" fn sword_ss_rise(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			let speed = smash::phx::Vector3f { x: 0.0, y: 0.65, z: 0.0 };
			KineticModule::add_speed(module_accessor, &speed);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 92, /*KBG*/ 100, /*FKB*/ 96, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.0), /*Z2*/ Some(6.0), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 92, /*KBG*/ 100, /*FKB*/ 96, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.0), /*Z2*/ Some(13.0), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 10.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 92, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(6.0), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 92, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(13.0), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 10.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.0, /*Angle*/ 268, /*KBG*/ 180, /*FKB*/ 100, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_ROKET_UNDER_DISABLE_CONTROL_X);
		}
		frame(fighter.lua_state_agent, 45.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
unsafe extern "C" fn sword_airgrab_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.5);
			frame(fighter.lua_state_agent, 6.0);
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 14.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(6.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
				AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
				if true{
					let hit = smash::phx::Vector2f { x: 8.0, y: 0.0 };
					AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &hit, 0, false);
				}
			}
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}
			frame(fighter.lua_state_agent, 10.0);
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.2333);
}	
unsafe extern "C" fn sword_airgrab_start_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("havel"), -0.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		wait(fighter.lua_state_agent, 1.0);
		for _ in 0..10 {
			if macros::is_excute(fighter) {
				macros::EFFECT(fighter, Hash40::new("sys_fireflower_shot"), Hash40::new("havel"), -0.0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
			}
			wait(fighter.lua_state_agent, 1.0);
		}
}	
unsafe extern "C" fn sword_airgrab_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		for _ in 0..50 
 {
			if macros::is_excute(fighter) {
				macros::EFFECT(fighter, Hash40::new("sys_fireflower_shot"), Hash40::new("havel"), -0.0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
			}
			wait(fighter.lua_state_agent, 3.0)	
		}
}	
unsafe extern "C" fn sword_airgrab_end_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, -1, 10, 0, 0, 0, 2.75, 0, 0, 0, 0, 0, 0, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_fireflower_shot"), false, true);
		}
}
unsafe extern "C" fn sword_airgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
				macros::SET_SPEED_EX(fighter, 0.8, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 0.5, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 1, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_nomal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 0.5, /*Angle*/ 60, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 1, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_nomal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
				AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
				if true{
					let hit = smash::phx::Vector2f { x: 8.0, y: 0.0 };
					AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &hit, 0, false);
				}
			}
			frame(fighter.lua_state_agent, 7.0);
			if macros::is_excute(fighter) {
				macros::SET_SPEED_EX(fighter, 2, -2.625, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			}
}	
unsafe extern "C" fn sword_airgrab_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
			if macros::is_excute(fighter) {
				macros::SET_SPEED_EX(fighter, -0.8, 0.7, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				AttackModule::clear_all(fighter.module_accessor);
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 10.0, /*Angle*/ 50, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
			}
			wait(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}
}	
unsafe extern "C" fn sword_airgrab_end_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
			if macros::is_excute(fighter) {
				macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_miiswordsman_rnd_attack03"));
				macros::PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n02"));
			}
}
unsafe extern "C" fn sword_airgrab_end_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
		}
}

unsafe extern "C" fn sword_sideb1_start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
		macros::FT_MOTION_RATE(agent, 0.6);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.7, 0.0, 12.0, 6.0, Some(0.0), Some(3.5), Some(6.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}
unsafe extern "C" fn sword_sideb1_start_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
		macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
		let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
		let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
		let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 5, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
	}
	wait(agent.lua_state_agent, 3.0);
	for _ in 0..i32::MAX {
		if macros::is_excute(agent) {
			macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
		}
		wait(agent.lua_state_agent, 3.0);
	}
}	
unsafe extern "C" fn sword_sideb1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        frame(fighter.lua_state_agent, 1.0);	
		if macros::is_excute(fighter) {
        	JostleModule::set_status(fighter.module_accessor, true);
        	damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
        }
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 12.0, /*Angle*/ 85, /*KBG*/ 79, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 7, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 12.0, /*Angle*/ 85, /*KBG*/ 79, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 7, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 12.0, /*Angle*/ 85, /*KBG*/ 79, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 7, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_NORMAL, /*DamageThreshold*/ 0);
			JostleModule::set_status(fighter.module_accessor, false);
		}
}	
unsafe extern "C" fn sword_sideb1_air_start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
		macros::FT_MOTION_RATE(agent, 0.5);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.7, 0.0, 12.0, 6.0, Some(0.0), Some(3.5), Some(6.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}
unsafe extern "C" fn sword_sideb1_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 45, 80, 0, 80, 7.0, 0.0, 4.0, 11.0, Some(0.0), Some(1.5), Some(11.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
		}
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
unsafe extern "C" fn sword_sideb1_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
		let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
		let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
		let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 5, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 6);
    }
	
}	
unsafe extern "C" fn sword_sideb1_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_special_s05"));
    }
}	
unsafe extern "C" fn sword_sideb1_air_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
		let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
		let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
		let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 5, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("miiswordsman_hensoku_flash"), Hash40::new("haver"), 0, 8, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 6);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, false, false);
    }
}	