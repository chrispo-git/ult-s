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

unsafe extern "C" fn rayman_upb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
unsafe extern "C" fn rayman_upb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_pikmin_special_l03"));
        }
        wait(fighter.lua_state_agent, 13.0);
    }
}	

unsafe extern "C" fn rayman_sideb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, -0.5, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 0.3670886075949367);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 4.0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 20.0, 361, 95, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 30.0);
    for x in 0..70 {
        if macros::is_excute(fighter) {
            macros::SET_SPEED_EX(fighter, 4.0 - (((MotionModule::frame(fighter.module_accessor)-30.0)/70.0)as f32 *2.2), 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 100.0);
    macros::FT_MOTION_RATE(fighter, 1.5);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn rayman_downb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("waist"), 15.0, 52, 60, 0, 80, 11.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("waist"), 11.0, 52, 60, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        StatusModule::set_keep_situation_air(fighter.module_accessor, false);
    }
}
unsafe extern "C" fn rayman_downb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn rayman_downb_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_07"));
        attack_vc(fighter);
    }
}
unsafe extern "C" fn rayman_air_downb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::clear_speed_all(fighter.module_accessor);
        macros::SET_SPEED_EX(fighter, 0.0, 0.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.0, -5.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(fighter, 0, 0, Hash40::new("waist"), 15.0, 270, 100, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("waist"), 11.0, 70, 55, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn rayman_air_downb_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_blowaway_m"));
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_final06"));
        attack_vc(fighter);
    }
}
unsafe extern "C" fn rayman_air_downb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.5, false);
    }
    frame(agent.lua_state_agent, 13.0);
    for _ in 0..15 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 18, 2.5, 90, 0, 0, 1.2, true, *EF_FLIP_YZ, 0.3);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}
unsafe extern "C" fn rayman_downb_land_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_blowaway_m"));
        macros::STOP_SE(fighter, Hash40::new("se_pikmin_final06"));
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_l01"));
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_pikmin_attackair_b01"));
    }
}
unsafe extern "C" fn rayman_downb_land_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn rayman_neutralb(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = smash::app::utility::get_kind(boma);
    if macros::is_excute(fighter) {
        PULL_DISTANCE[ENTRY_ID] = 0;
        if fighter_kind == *FIGHTER_KIND_PIKMIN {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        PULL_DISTANCE[ENTRY_ID] = 0;
        macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 5.0, 361, 100, 30, 0, 5.0, 0.0, 0.0, 5.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ -3.0, /*Unk*/ false);
        macros::ATTACK(fighter, 1, 0, Hash40::new("throw"), 5.0, 90, 100, 30, 0, 5.0, 0.0, 0.0, 5.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ -3.0, /*Unk*/ false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.2);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        PULL_DISTANCE[ENTRY_ID] = 1;
        macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 5.0, 361, 100, 30, 0, 5.0, 0.0, 0.0, 16.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("throw"), 5.0, 90, 100, 30, 0, 5.0, 0.0, 0.0, 16.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.2);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        PULL_DISTANCE[ENTRY_ID] = 2;
        macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 5.0, 361, 100, 30, 0, 5.0, 0.0, 0.0, 20.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1.0, /*Unk*/ false);
        macros::ATTACK(fighter, 1, 0, Hash40::new("throw"), 5.0, 90, 100, 30, 0, 5.0, 0.0, 0.0, 20.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1.0, /*Unk*/ false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.2);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) { 
        PULL_DISTANCE[ENTRY_ID] = 3;
        macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 5.0, 361, 100, 30, 0, 5.0, 0.0, 0.0, 25.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 3.0, /*Unk*/ false);
        macros::ATTACK(fighter, 1, 0, Hash40::new("throw"), 5.0, 90, 100, 30, 0, 5.0, 0.0, 0.0, 25.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 3.0, /*Unk*/ false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.2);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        PULL_DISTANCE[ENTRY_ID] = 0;
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn rayman_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
	}
}
unsafe extern "C" fn rayman_neutralb_air_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
}
unsafe extern "C" fn rayman_neutralb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_special_l01"));
    }
}
unsafe extern "C" fn rayman_neutralb_pull_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_special_l02"));
    }
}
unsafe extern "C" fn rayman_sideb_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, 13, 0, 0, 0, 0.4, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    for _ in 0..7 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2, 9, 93, 0, 0, 0.231, true);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.8);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2, 6, 87, 0, 30, 0.35, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.8);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2.5, 0, 100, 0, 60, 0.5, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.8);
        }
        wait(fighter.lua_state_agent, 3.0);
    }
    frame(fighter.lua_state_agent, 110.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 12.5, 3.5, 1, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
	}
}
unsafe extern "C" fn rayman_sideb_snd(fighter: &mut L2CAgentBase) {
    let rand_val = smash::app::sv_math::rand(hash40("fighter"), 10);
    frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
        if rand_val <= 7 {
            macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_n01"));
        }
    }
    frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_blowaway_m"));
    }
}

pub fn install() {
    Agent::new("pikmin")
        .sound_acmd("sound_specialhirayman", rayman_upb_snd, Priority::Low)
        .sound_acmd("sound_specialairhiwait1rayman", rayman_upb_snd, Priority::Low)
        .sound_acmd("sound_specialairhirayman", rayman_upb_snd, Priority::Low)
        .game_acmd("game_specialsrayman", rayman_sideb, Priority::Low)
        .game_acmd("game_speciallwrayman", rayman_downb, Priority::Low)
        .effect_acmd("effect_speciallwrayman", rayman_downb_eff, Priority::Low)
        .sound_acmd("sound_speciallwrayman", rayman_downb_snd, Priority::Low)
        .game_acmd("game_specialairlwrayman", rayman_air_downb, Priority::Low)
        .sound_acmd("sound_specialairlwrayman", rayman_air_downb_snd, Priority::Low)
        .effect_acmd("effect_specialairlwrayman", rayman_air_downb_eff, Priority::Low)
        .sound_acmd("sound_speciallwland", rayman_downb_land_snd, Priority::Low)
        .effect_acmd("effect_speciallwland", rayman_downb_land_eff, Priority::Low)
        .game_acmd("game_specialnstartrayman", rayman_neutralb, Priority::Low)
        .game_acmd("game_specialairnfailurerayman", rayman_neutralb, Priority::Low)
        .effect_acmd("effect_specialnstartrayman", rayman_neutralb_eff, Priority::Low)
        .effect_acmd("effect_specialairnfailurerayman", rayman_neutralb_air_eff, Priority::Low)
        .sound_acmd("sound_specialnstartrayman", rayman_neutralb_snd, Priority::Low)
        .sound_acmd("sound_specialairnfailurerayman", rayman_neutralb_snd, Priority::Low)
        .sound_acmd("sound_specialnpull", rayman_neutralb_pull_snd, Priority::Low)
        .sound_acmd("sound_specialairnpull", rayman_neutralb_pull_snd, Priority::Low)
        .effect_acmd("effect_specialsrayman", rayman_sideb_eff, Priority::Low)
        .sound_acmd("sound_specialsrayman", rayman_sideb_snd, Priority::Low)
        .install();
}