use smash::app::sv_animcmd::*;
use smash::phx::{Hash40, Vector2f, Vector3f};
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
use crate::util::*;

static mut CURRENT_PIKMIN : [i32; 8] = [0; 8];
static mut SET_UPB_FREEFALL: [bool; 8] = [false; 8];
static mut IS_SLIDE_MOVE: [bool; 8] = [false; 8];
static mut PULL_DISTANCE: [i32; 8] = [0; 8];
static mut DO_WALLJUMP_FORCE: [bool; 8] = [false; 8];
static mut HAS_DEADED: [bool; 8] = [false; 8];
static mut WAS_SLIDE: [bool; 8] = [false; 8];
static mut FINAL_DURATION : [i32; 8] = [0; 8];
static mut X : [f32; 8] = [0.0; 8];
static mut Y : [f32; 8] = [0.0; 8];
static mut X_MAX : f32 = 1.155;
static mut X_ACCEL_ADD : f32 = 0.06;
static mut X_ACCEL_MUL : f32 = 0.12;
static mut Y_MAX : f32 = 1.155;
static mut Y_ACCEL_ADD : f32 = 0.06;
static mut Y_ACCEL_MUL : f32 = 0.12;


#[acmd_script(
    agent = "pikmin",
    script =  "game_attack11rayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_jab_1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 0.0, 361, 100, 70, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 3.0, 361, 15, 0, 35, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}		

#[acmd_script( 
	agent = "pikmin", 
	script = "effect_attack11rayman", 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn rayman_jab_1_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.8, true, 0.5);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4, 12, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 360, true, 0.4);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pikmin_thrown_white"), false, false);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_attack11rayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_jab_1_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s04"));
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "game_attack12rayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_jab_2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 3.0, 361, 15, 0, 35, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}		
#[acmd_script( 
	agent = "pikmin", 
	script = "effect_attack12rayman", 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn rayman_jab_2_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.8, true, 0.5);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4, 13, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 360, true, 0.4);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_attack12rayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_jab_2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s04"));
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "game_attack13rayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_jab_3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 3.5, 45, 50, 0, 75, 7.0, 0.0, 0.0, 0.0, None, None, None, 2.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}	
#[acmd_script( 
	agent = "pikmin", 
	script = "effect_attack13rayman", 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn rayman_jab_3_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 1.0, true, 0.5);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.4);
    }
	frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_attack13rayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_jab_3_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
        attack_vc(agent);
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "game_attackairfrayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	macros::FT_MOTION_RATE(fighter, 0.5);
	wait(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("toer"), 12.0, 361, 86, 0, 40, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("toer"), 8.5, 50, 80, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}		
#[acmd_script(
    agent = "pikmin",
    script =  "effect_attackairfrayman",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn rayman_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2.25, 1, -10, 0, 0, 0.5, true);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.8);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.69, 1.29);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.39, 0.99);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.05, 0.05, 0.05);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.14, 0.07, 0.0);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
        }
    }
}		
#[acmd_script(
    agent = "pikmin",
    script =  "sound_attackairfrayman",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn rayman_fair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_sword_swing_m"));
        attack_vc(fighter);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "game_attackairnrayman", 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn rayman_nair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 8.0, 361, 112, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("handl"), 8.0, 361, 112, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 8.0, 361, 112, 0, 20, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 6.0, 361, 112, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("handl"), 6.0, 361, 112, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 6.0, 361, 112, 0, 0, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}	
#[acmd_script( 
	agent = "pikmin", 
	script = "effect_attackairnrayman", 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn rayman_nair_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("trans"), 0, 7, 0, 210, 0, 0, 0.6, true, 0.5);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("trans"), 0, 9.35, 2, 210, 0, 0, 0.35, true, 0.5);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("trans"), 0, 5, -1.8, 210, 0, 0, 0.25, true, 0.5);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("trans"), 0, 6.5, 0, 210, 0, 0, 0.6, true, 0.3);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "sound_attackairnrayman", 
	category = ACMD_SOUND, 
	low_priority )]
unsafe fn rayman_nair_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_smash_s02"));
        attack_vc(fighter);
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "game_attackairbrayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	macros::FT_MOTION_RATE(fighter, 0.6);
	wait(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 15.0, 361, 96, 0, 37, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}			
#[acmd_script(
    agent = "pikmin",
    script =  "effect_attackairbrayman",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn rayman_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 4, 0, 176, 0, 230, 0.7, true, *EF_FLIP_YZ);
        } else {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 4, 0, 176, 0, 230, 0.7, true, *EF_FLIP_YZ);
        }
        macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 28.0);
	if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc"), false, true);
	}
}	
#[acmd_script(
    agent = "pikmin",
    script =  "sound_attackairbrayman",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn rayman_bair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_08"));
        attack_vc(fighter);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "game_attackairlwrayman", 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn rayman_dair(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.4);
    frame(fighter.lua_state_agent, 16.0);
	macros::FT_MOTION_RATE(fighter, 0.8);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 100, 70, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 367, 100, 70, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
	}
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 100, 70, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 367, 100, 70, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
	}
    frame(fighter.lua_state_agent, 33.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 25, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
	}
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}	
#[acmd_script(
    agent = "pikmin",
    script =  "effect_attackairlwrayman",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn rayman_dair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 360, true, 0.4);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 360, true, 0.4);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 360, true, 0.4);
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "sound_attackairlwrayman",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn rayman_dair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_smash_s04"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_smash_s04"));
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_smash_s04"));
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "game_attackhi4rayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 16.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
#[acmd_script(
    agent = "pikmin",
    script =  "effect_attackhi4rayman",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn rayman_usmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("trans"), 2, 9, 0, -3, 23.6, 96.3, 0.65, true);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.69, 1.29);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.39, 0.99);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.05, 0.05, 0.05);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.14, 0.07, 0.0);
            }
            else {
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
            }
		}
		frame(fighter.lua_state_agent, 33.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script(
    agent = "pikmin",
    script =  "game_attacklw4rayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		macros::FT_MOTION_RATE(fighter, 0.5);
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 17.0);
		macros::FT_MOTION_RATE(fighter, 1.5);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 30, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(-4.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 11.5, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(-11.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		

#[acmd_script(
    agent = "pikmin",
    script =  "game_attacks4rayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 15.0, /*Angle*/ 55, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 18.5, /*Angle*/ 55, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 15.0, /*Angle*/ 55, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 33.0);
		macros::FT_MOTION_RATE(fighter, 1.5);
}		

#[acmd_script(
    agent = "pikmin",
    script =  "effect_attacks4rayman",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn rayman_fsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), -5, 4, -6.5, -34.857, 18.764, 94.68, 0.25, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.55);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), -5, 4, -6.5, -34.857, 18.764, 94.68, 0.12, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.55);
        }
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_spin_wind"), false, true);
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -9, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 360, true, 0.5);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
        }
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
		}
}	
#[acmd_script(
    agent = "pikmin",
    script =  "effect_attacks4chargerayman",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn rayman_fsmash_charge_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_spin_wind"), false, true);
    }
	for _ in 0..25 {
		if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 8, 0, 4, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -6.75, -16.857, 18.764, 94.68, 0.275, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.05);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
            }
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -6.75, -16.857, 18.764, 94.68, 0.25, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.05);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
            }
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -6.75, -16.857, 18.764, 94.68, 0.225, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.05);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
            }
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -6.75, -16.857, 18.764, 94.68, 0.2, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.05);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
            }
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -6.75, -16.857, 18.764, 94.68, 0.175, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.05);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
            }
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -6.75, -16.857, 18.764, 94.68, 0.15, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.05);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
            }
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -6.75, -16.857, 18.764, 94.68, 0.125, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.05);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
            }
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, -6.75, -16.857, 18.764, 94.68, 0.1, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.05);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
            }
		}
		wait(fighter.lua_state_agent, 5.0);
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 6, 4, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
	}
}
#[acmd_script( agent = "pikmin", script = "sound_attacks4chargerayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_fsmash_charge_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smash_start_04"));
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s03"));
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "effect_attackhi4chargerayman",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn rayman_usmash_charge_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    for _ in 0..25 {
		if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.8, 8, 0, 4, 0, 0, 0, false);
		}
		wait(fighter.lua_state_agent, 5.0);
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 6, 4, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
	}
}
#[acmd_script(
    agent = "pikmin",
    script =  "effect_attacklw4chargerayman",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn rayman_dsmash_charge_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    for _ in 0..25 {
		if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 8, 0, 4, 0, 0, 0, false);
		}
		wait(fighter.lua_state_agent, 5.0);
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 9, 1.25, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
	}
}
#[acmd_script( 
    agent = "pikmin",
    script =  "effect_attacklw4rayman",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn rayman_dsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
	}
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
		macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
		macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
		macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
	}	
}
#[acmd_script( agent = "pikmin", script = "sound_attacklw4rayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_dsmash_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_h01"));
        attack_vc(agent);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_attacks4rayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_fsmash_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_appeal_h01"));
        macros::STOP_SE(agent, Hash40::new("se_pikmin_smash_s03"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s01"));
        macros::PLAY_SE(agent, Hash40::new("se_common_smashswing_04"));
    }
}
#[acmd_script( agent = "pikmin", script = "sound_attackhi4rayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_usmash_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_pikmin_attackdash01"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_07"));
        attack_vc(agent);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "game_attacks3rayman", 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn rayman_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 11.0, 361, 70, 0, 42, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "effect_attacks3rayman", 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn rayman_ftilt_eff(fighter: &mut L2CAgentBase) {
}	
#[acmd_script( agent = "pikmin", script = "sound_attacks3", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_ftilt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s01"));
        attack_vc(agent);
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "game_attackairhirayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	macros::FT_MOTION_RATE(fighter, 0.7);
	wait(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("toer"), 12.0, 85, 75, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("toer"), 8.5, 85, 75, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}	
#[acmd_script(
    agent = "pikmin",
    script =  "effect_attackairhirayman",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn rayman_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8, -0.5, -85, 0, 0, 0.5, true);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.8);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.69, 1.29);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.39, 0.99);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.05, 0.05, 0.05);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.14, 0.07, 0.0);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
        }
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "sound_attackairhirayman",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn rayman_uair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_smash_h02"));
        attack_vc(fighter);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "game_attackdashrayman", 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn rayman_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 12.0, 50, 70, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("handl"), 12.0, 50, 70, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("waist"), 12.0, 50, 70, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 8.0, 50, 60, 0, 80, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("handl"), 8.0, 50, 60, 0, 80, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("waist"), 8.0, 50, 60, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "effect_attackdashrayman", 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn rayman_da_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 6.0, 0, 0, 0, -7, 0.75, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 6.0, 0, 0, 0, -7, 0.75, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_spin_wind"), false, true);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_attackdashrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_da_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
        attack_vc(agent);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "game_attackhi3rayman", 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn rayman_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 9.0, 81, 40, 0, 85, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 9.0, 81, 40, 0, 85, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    	macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.1);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 0.1);
	}  
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
		StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
		StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 8.0, 81, 40, 0, 85, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 8.0, 81, 40, 0, 85, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    	macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.1);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 0.1);
	}
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
		StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
	}
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
		KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "effect_attackhi3rayman", 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn rayman_utilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
}
#[acmd_script( 
	agent = "pikmin", 
	script = "sound_attackhi3rayman", 
	category = ACMD_SOUND, 
	low_priority )]
unsafe fn rayman_utilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_b01"));
	}
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
		let rand_val = smash::app::sv_math::rand(hash40("fighter"), 5);
	    match rand_val {
            0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_n02")),
            _ => println!("rayman is silent"),
        }
	}
}
#[acmd_script( 
	agent = "pikmin", 
	script = "game_attacklw3rayman", 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn rayman_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 9.0, 361, 88, 0, 30, 8.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    } 
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_attacklw3rayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 2, 4.3, 0, 22.5, 0, 0.5, true, 1.0);
        }
        else {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 2, 4.3, 180, 157.5, 0, 0.5, true, 1.0);
        }
        macros::LAST_EFFECT_SET_RATE(fighter, 0.45);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.69, 1.29);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.39, 0.99);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 124 { //medieval
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 125 { //spy
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.05, 0.05, 0.05);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.14, 0.07, 0.0);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
        }
    }
}
#[acmd_script( agent = "pikmin", script = "sound_attacklw3rayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_dtilt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_h04"));
        attack_vc(agent);
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "game_catchrayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_grab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 3.3, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(9.7), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
			macros::CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 1.65, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ 2.35, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(10.8), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
		}
		if true{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
		}
}		
#[acmd_script( agent = "pikmin", script = "sound_catchrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_grab_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_swing_05"));
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "game_catchdashrayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_dashgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 3.3, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(10.7), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
			macros::CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 1.65, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ 2.35, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(12.8), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
		}
		if true{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
		}
}		
#[acmd_script( agent = "pikmin", script = "sound_catchdashrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_dashgrab_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_swing_05"));
    }
}
#[acmd_script(
    agent = "pikmin",
    script =  "game_catchturnrayman",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rayman_pivotgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.625);
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
		}
		frame(fighter.lua_state_agent, 9.0);
        macros::FT_MOTION_RATE(fighter, 5.0);
		if macros::is_excute(fighter) {
			macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("haver"), /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
			macros::CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("haver"), /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
		}
		if true{
			macros::game_CaptureCutCommon(fighter);
		}
		frame(fighter.lua_state_agent, 11.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
		if macros::is_excute(fighter) {
			grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
		}
}		
#[acmd_script( agent = "pikmin", script = "sound_catchturnrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_pivotgrab_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_swing_05"));
    }
}	
#[acmd_script(
    agent = "pikmin",
    scripts =  ["effect_specialhirayman", "effect_specialairhiwait1rayman", "effect_specialairhirayman"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn rayman_upb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "pikmin",
    scripts =  ["sound_specialhirayman", "sound_specialairhiwait1rayman", "sound_specialairhirayman"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn rayman_upb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_pikmin_special_l03"));
        }
        wait(fighter.lua_state_agent, 13.0);
    }
}	
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_catchrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_grab_eff(fighter: &mut L2CAgentBase) {
    
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_catchturnrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_pivotgrab_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3.8, 0, 0, 180, 0, 0.6, false, 0.75);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "game_throwhirayman", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_uthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 85, 30, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 21, 85);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("handr"), 6.0, 75, 55, 0, 80, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, false);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_throwhirayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_uthrow_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 360, true, 0.4);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_throwhirayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_uthrow_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_appeal_h01"));
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "game_throwfrayman", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_fthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 45, 50, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
	frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_throwfrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_fthrow_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "game_throwbrayman", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_bthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 40, 66, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_throwbrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_bthrow_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "game_throwlwrayman", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_dthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 93, 100, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 7, 0);
        //lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.7);
        //lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
	frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_throwlwrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_dthrow_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
		macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_throwlwrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_dthrow_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_03"));
        macros::PLAY_SE(agent, Hash40::new("se_common_down_l_01"));
        attack_vc(agent);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "game_slideattacklw", 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn rayman_slide_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    macros::FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 6.0, 50, 50, 0, 70, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    } 
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "effect_slideattacklw", 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn rayman_slide_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, 3, 0, 0, 0, 0.35, true);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 { //raymesis
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 { //glowbox
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
        }
    }
}
#[acmd_script( agent = "pikmin", script = "sound_slideattacklw", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_slide_dtilt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_l02"));
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "game_slideattackhi", 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn rayman_slide_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 9.0, 81, 40, 0, 85, 8.0, 0.0, 3.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 9.0, 81, 40, 0, 85, 8.0, 0.0, 3.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    	macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.1);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 0.1);
	}  
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
		StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
		StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 8.0, 81, 40, 0, 85, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 8.0, 81, 40, 0, 85, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    	macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.1);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 0.1);
	}
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
		StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
	}
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
		KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "effect_slideattackhi", 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn rayman_slide_utilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
}
#[acmd_script( 
	agent = "pikmin", 
	script = "sound_slideattackhi", 
	category = ACMD_SOUND, 
	low_priority )]
unsafe fn rayman_slide_utilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_b01"));
	}
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
		let rand_val = smash::app::sv_math::rand(hash40("fighter"), 5);
	    match rand_val {
            0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_n02")),
            _ => println!("rayman is silent"),
        }
	}
}
#[acmd_script( 
	agent = "pikmin", 
	script = "game_slideattack", 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn rayman_slide_attack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("waist"), 2.5, 365, 100, 80, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 2.5, 365, 100, 80, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footr"), 2.5, 365, 100, 80, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    } 
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 23.0);
    macros::FT_MOTION_RATE(fighter, 2.2);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("waist"), 4.0, 361, 125, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.0, 361, 125, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footr"), 4.0, 361, 125, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    } 
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( 
	agent = "pikmin", 
	script = "effect_slideattack", 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn rayman_slide_attack_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 4.0, 0, 0, 0, -7, 0.65, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 2.0, 0, 0, 180, 7, 0.65, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 4.0, 0, 0, 0, -7, 0.65, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 2.0, 0, 0, 180, 7, 0.65, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_slideattack", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_slide_attack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        attack_vc(agent);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "game_cliffattackrayman", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_cliffattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 1.25);
    frame(fighter.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 45, 20, 0, 90, 5.0, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_cliffattackrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_cliffattack_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, -2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4, 10.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_cliffattackrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_cliffattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_dash_start"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s04"));
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_runrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_run_eff(fighter: &mut L2CAgentBase) {
    for _ in 0..35 {
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "sound_runrayman", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn rayman_run_snd(fighter: &mut L2CAgentBase) {
    for _ in 0..35 {
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STEP(fighter, Hash40::new("se_pikmin_step_right_ll"));
        }
        wait(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STEP(fighter, Hash40::new("se_pikmin_step_left_ll"));
        }
        wait(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STEP(fighter, Hash40::new("se_pikmin_step_right_ll"));
        }
        wait(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STEP(fighter, Hash40::new("se_pikmin_step_left_ll"));
        }
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "sound_walkfastrayman", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn rayman_walkfast_snd(fighter: &mut L2CAgentBase) {
    
}
#[acmd_script( 
    agent = "pikmin", 
    script = "sound_walkfastrayman", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn rayman_walkmiddle_snd(fighter: &mut L2CAgentBase) {
    
}
#[acmd_script( 
    agent = "pikmin", 
    script = "sound_walkfastrayman", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn rayman_walkslow_snd(fighter: &mut L2CAgentBase) {
    
}
#[acmd_script( 
    agent = "pikmin", 
    scripts = ["effect_appealhirrayman","effect_appealhilrayman"],
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_utaunt_eff(fighter: &mut L2CAgentBase) {
    
}
#[acmd_script( agent = "pikmin", scripts = ["sound_appealhilrayman","sound_appealhirrayman"], category = ACMD_SOUND, low_priority )]
unsafe fn rayman_utaunt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_appeal_h01"));
    }
}
#[acmd_script( 
    agent = "pikmin", 
    scripts = ["effect_appeallwrrayman","effect_appeallwlrayman"],
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_dtaunt_eff(fighter: &mut L2CAgentBase) {
    for _ in 0..55 {
        wait(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.1, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.1, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.1, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.1, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.1, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}
#[acmd_script( agent = "pikmin", scripts = ["sound_appeallwrrayman","sound_appeallwlrayman"], category = ACMD_SOUND, low_priority )]
unsafe fn rayman_dtaunt_snd(agent: &mut L2CAgentBase) {
    for _ in 0..55 {
        wait(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_pikmin_appeal_l01"));
        }
        wait(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_pikmin_appeal_l01"));
        }
        wait(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_pikmin_appeal_l01"));
        }
        wait(agent.lua_state_agent, 21.0);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "game_specialsrayman", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_sideb(fighter: &mut L2CAgentBase) {
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
#[acmd_script( 
    agent = "pikmin", 
    script = "game_speciallwrayman", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_downb(fighter: &mut L2CAgentBase) {
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
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_speciallwrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_downb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "sound_speciallwrayman", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn rayman_downb_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_07"));
        attack_vc(fighter);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "game_specialairlwrayman", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_air_downb(fighter: &mut L2CAgentBase) {
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
#[acmd_script( 
    agent = "pikmin", 
    script = "sound_specialairlwrayman", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn rayman_air_downb_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_blowaway_m"));
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_final06"));
        attack_vc(fighter);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_specialairlwrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_air_downb_eff(agent: &mut L2CAgentBase) {
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
#[acmd_script( 
    agent = "pikmin", 
    script = "sound_speciallwland", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn rayman_downb_land_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_blowaway_m"));
        macros::STOP_SE(fighter, Hash40::new("se_pikmin_final06"));
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_l01"));
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_pikmin_attackair_b01"));
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_speciallwland", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_downb_land_eff(agent: &mut L2CAgentBase) {
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

#[acmd_script( 
    agent = "pikmin", 
    scripts = ["game_specialnstartrayman", "game_specialairnfailurerayman"], 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_neutralb(fighter: &mut L2CAgentBase) {
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
#[acmd_script( 
    agent = "pikmin", 
    scripts = ["effect_specialnstartrayman"], 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
	}
}
#[acmd_script( 
    agent = "pikmin", 
    scripts = ["effect_specialairnfailurerayman"], 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_neutralb_air_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
}
#[acmd_script( 
    agent = "pikmin", 
    scripts = ["sound_specialnstartrayman", "sound_specialairnfailurerayman"], 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn rayman_neutralb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_special_l01"));
    }
}
#[acmd_script( 
    agent = "pikmin", 
    scripts = ["sound_specialnpull", "sound_specialairnpull"], 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn rayman_neutralb_pull_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_special_l02"));
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_specialsrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_sideb_eff(fighter: &mut L2CAgentBase) {
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
#[acmd_script( 
    agent = "pikmin", 
    script = "sound_specialsrayman", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn rayman_sideb_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_n01"));
    }
    frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_blowaway_m"));
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_turnrunrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_turnrun_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -7, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -7, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_runbrakerrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_runbraker_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 13, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 13, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 8.0);
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_runbrakelrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_runbrakel_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 13, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 13, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 8.0);
}
#[acmd_script( 
    agent = "pikmin", 
    script = "game_downattackurayman", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_downattacku(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 4.5, 0.0, 5.0, -23.5, Some(0.0), Some(5.0), Some(-13.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 4.5, 0.0, 5.0, 23.0, Some(0.0), Some(5.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_downattackurayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_downattacku_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line"), true, true);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1, 4.5, -4, 0, 180, 0, 0.3, false, 0.6);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 4.5, -2, 0, 180, 0, 0.4, true, 0.4);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, -14, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true, 1);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 5, 4, 0, 0, 0, 0.3, false, 0.6);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 5, 2, 0, 0, 0, 0.4, true, 0.4);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, 13.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true, 1);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line"), true, true);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_downattackurayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_downattacku_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_dash_start"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s04"));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s04"));
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "game_downattackdrayman", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_downattackd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 4.5, 0.0, 5.0, -23.5, Some(0.0), Some(5.0), Some(-13.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 4.5, 0.0, 5.0, 23.0, Some(0.0), Some(5.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_downattackdrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_downattackd_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line"), true, true);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1, 4.5, -4, 0, 180, 0, 0.3, false, 0.6);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 4.5, -2, 0, 180, 0, 0.4, true, 0.4);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, -14, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true, 1);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 5, 4, 0, 0, 0, 0.3, false, 0.6);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 5, 2, 0, 0, 0, 0.4, true, 0.4);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, 13.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true, 1);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line"), true, true);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_downattackdrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_downattackd_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_dash_start"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s04"));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s04"));
    }
}
#[acmd_script( agent = "pikmin", script = "sound_appealslrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_ftauntl_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_hit_nodamage"));
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_hit_nodamage"));
    }
}
#[acmd_script( agent = "pikmin", script = "sound_appealsrrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_ftauntr_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_hit_nodamage"));
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_hit_nodamage"));
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_escapefrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_escapef_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_escapefrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_escapef_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_escapeair"));
    }
    wait(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_step_right_m"));
    }
}
#[acmd_script( agent = "pikmin", script = "sound_cliffescaperayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_cliffescape_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_dash_start"));
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_step_left_m"));
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_escapeair"));
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_landing02"));
    }
}
#[acmd_script( agent = "pikmin", script = "sound_cliffjump2rayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_cliffjump2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_jump01"));
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_escapebrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_escapeb_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_escapebrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_escapeb_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_escapeair"));
    }
    wait(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_step_right_m"));
    }
}
#[acmd_script( 
    agent = "pikmin", 
    script = "effect_escapenrayman", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_escapen_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "pikmin_pikmin", script = "effect_flashingrayman", category = ACMD_EFFECT, low_priority )]
unsafe fn pikmin_flashing_eff(agent: &mut L2CAgentBase) {
    
}
#[acmd_script( agent = "pikmin", script = "sound_squatrvrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_squatrv_snd(agent: &mut L2CAgentBase) {
    
}
#[acmd_script(
    agent = "pikmin",
    scripts =  ["sound_damagehi1rayman", "sound_damagehi2rayman", "sound_damagehi3rayman", "sound_damagen1rayman", "sound_damagen2rayman", "sound_damagen3rayman", "sound_damagelw1rayman", "sound_damagelw2rayman", "sound_damagelw3rayman", "sound_damageair1rayman", "sound_damageair2rayman", "sound_damageair3rayman", "sound_damageelecrayman"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn rayman_dmg_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		dmg_vc(fighter);
	};
}	
#[acmd_script(
    agent = "pikmin",
    scripts =  ["sound_damageflyhirayman", "sound_damageflynrayman", "sound_damageflylwrayman", "sound_damageflytoprayman", "sound_damageflyrollrayman", "sound_damageflymeteorrayman"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn rayman_dmg_fly_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		dmg_fly_vc(fighter);
	};
}	
#[acmd_script(
    agent = "pikmin",
    scripts =  ["effect_damageflyhirayman", "effect_damageflynrayman", "effect_damageflylwrayman", "effect_damageflytoprayman", "effect_damageflyrollrayman", "effect_damageflymeteorrayman"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn rayman_dmg_fly_eff(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	
}	
#[acmd_script(
    agent = "pikmin",
    scripts =  ["sound_damagefallrayman"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn rayman_star_ko_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DEAD_UP_FALL) {
			macros::PLAY_SE(fighter, Hash40::new("se_pikmin_final01"));
		};
	};
}
#[acmd_script( agent = "pikmin", script = "sound_jumpfrontminirayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_jumpfrontmini_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_jump03"));
        jump_vc(fighter);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_jumpbackminirayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_jumpbackmini_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_jump03"));
        jump_vc(fighter);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_jumpaerialbackrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_jumpaerialback_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_jump02"));
        jump_aerial_vc(fighter);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_jumpaerialfrontrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_jumpaerialfront_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_jump02"));
        jump_aerial_vc(fighter);
    }
}
#[acmd_script( agent = "pikmin", script = "effect_slipattackrayman", category = ACMD_EFFECT, low_priority )]
unsafe fn rayman_slipattack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 5, 4, 0, 0, 12, 0.7, false, 0.2);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4.5, 13, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 360, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -2, 0, 180, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 5, -1, 0, 180, 0, 0.4, false, 0.6);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4.5, -8, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 360, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_slipattackrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_slipattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s04"));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s04"));
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_pikmin_landing01"));
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_pikmin_step_right_m"));
    }
}
#[acmd_script( agent = "pikmin", script = "game_entrylrayman", category = ACMD_GAME, low_priority )]
unsafe fn rayman_entryl(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
#[acmd_script( agent = "pikmin", script = "effect_entrylrayman", category = ACMD_EFFECT, low_priority )]
unsafe fn rayman_entryl_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_entrylrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_entryl_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_pikmin_landing01"));
    }
}
#[acmd_script( agent = "pikmin", script = "game_entryrrayman", category = ACMD_GAME, low_priority )]
unsafe fn rayman_entryr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
#[acmd_script( agent = "pikmin", script = "effect_entryrrayman", category = ACMD_EFFECT, low_priority )]
unsafe fn rayman_entryr_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "pikmin", script = "sound_entryrrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_entryr_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_pikmin_landing01"));
    }
}
#[acmd_script( agent = "pikmin_dolfin", script = "effect_entrylrayman", category = ACMD_EFFECT, low_priority )]
unsafe fn dolfin_entryl_eff(agent: &mut L2CAgentBase) {
    
}
#[acmd_script( agent = "pikmin_dolfin", script = "effect_entryrrayman", category = ACMD_EFFECT, low_priority )]
unsafe fn dolfin_entryr_eff(agent: &mut L2CAgentBase) {
    
}
#[acmd_script( agent = "pikmin_dolfin", scripts = ["game_finalstartrayman", "game_finalrayman", "game_finalattackrayman"], category = ACMD_GAME, low_priority )]
unsafe fn dolfin_blank(agent: &mut L2CAgentBase) {
    
}
#[acmd_script( agent = "pikmin", script = "game_finalrayman", category = ACMD_GAME, low_priority )]
unsafe fn rayman_final_wait(agent: &mut L2CAgentBase) {
    
}
#[acmd_script( agent = "pikmin", script = "effect_finalrayman", category = ACMD_EFFECT, low_priority )]
unsafe fn rayman_final_wait_eff(agent: &mut L2CAgentBase) {
    
}
#[acmd_script( agent = "pikmin", script = "sound_finalrayman", category = ACMD_SOUND, low_priority )]
unsafe fn rayman_final_wait_snd(agent: &mut L2CAgentBase) {
   
}
#[acmd_script( agent = "pikmin", script = "expression_finalrayman", category = ACMD_EXPRESSION, low_priority )]
unsafe fn rayman_final_wait_expr(agent: &mut L2CAgentBase) {
    
}
#[acmd_script( agent = "pikmin", script = "game_finalshootrayman", category = ACMD_GAME, low_priority )]
unsafe fn rayman_final_shoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ItemModule::remove_item(fighter.module_accessor, 0);
        ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_SUPERSCOPE), 0, 0, false, false);
        macros::STOP_SE(fighter, Hash40::new("se_item_item_get"));
		EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_item_get"), false, false);
        fighter.clear_lua_stack();
		lua_args!(fighter, 12);
        smash::app::sv_animcmd::SHOOT_ITEM_BULLET_CHARGE(fighter.lua_state_agent);
    }
}
#[acmd_script( agent = "pikmin", 
scripts = [
    "effect_damagen1rayman", "effect_damagen2rayman", "effect_damagen3rayman",
    "effect_damagelw1rayman", "effect_damagelw2rayman", "effect_damagelw3rayman",
    "effect_damagehi1rayman", "effect_damagehi2rayman", "effect_damagehi3rayman",
    "effect_damageair1rayman", "effect_damageair2rayman", "effect_damageair3rayman"
], 
category = ACMD_EFFECT)]
unsafe fn rayman_dmg_eff(agent: &mut L2CAgentBase) {
}
#[acmd_script( 
    agent = "pikmin", 
    scripts = ["game_win1rayman", "game_win1waitrayman", "game_win2rayman", "game_win2waitrayman", "game_win3rayman", "game_win3waitrayman"], 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn rayman_win2(fighter: &mut L2CAgentBase) {
}
#[acmd_script( 
    agent = "pikmin", 
    scripts = ["effect_win1rayman", "effect_win1waitrayman", "effect_win2rayman", "effect_win2waitrayman", "effect_win3rayman", "effect_win3waitrayman"], 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn rayman_win_eff(fighter: &mut L2CAgentBase) {
}
#[acmd_script( 
    agent = "pikmin", 
    scripts = ["sound_win1rayman", "sound_win1waitrayman", "sound_win2rayman", "sound_win2waitrayman", "sound_win3rayman", "sound_win3waitrayman"], 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn rayman_win_snd(fighter: &mut L2CAgentBase) {
}

pub(crate) unsafe fn attack_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 6);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_l01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_h01")),
		2 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_f01")),
        3 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackdash02")),
		_ => println!("rayman is silent"),
	}
}
pub(crate) unsafe fn dmg_vc(fighter: &mut L2CAgentBase) -> () {
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_h01"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_h02"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_h03"));
    macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_l01"));
    macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_s01"));
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 5);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h02")),
        2 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h03")),
        3 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_l01")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_s01")),
	}
}
pub(crate) unsafe fn dmg_fly_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackdash01"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackair_n03"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackair_n02"));
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackdash01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_n03")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_n02")),
	}
}
pub(crate) unsafe fn jump_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 2);
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackdash02"));
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackdash02")),
        _ => println!("rayman is silent"),
	}
}
pub(crate) unsafe fn jump_aerial_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 6);
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_s02"));
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_s02")),
		_ => println!("rayman is silent"),
	}
}
#[acmd_script( 
    agent = "kirby", 
    scripts = ["effect_pikminspecialn", "effect_pikminspecialairn"], 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn kirby_copy(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
            StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        }
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
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
#[acmd_script( 
    agent = "kirby", 
    scripts = ["sound_pikminspecialn", "sound_pikminspecialairn"], 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn kirby_copy_fx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, 13, 0, 0, 0, 0.4, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        SoundModule::stop_all_sound(fighter.module_accessor);
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_damagefly02"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_blowaway_m"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_blowaway_m"));
    }
    frame(fighter.lua_state_agent, 21.0);
    for _ in 0..7 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2, 9.0*1.5, 93, 0, 0, 0.231*1.5, true);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.8);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2, 6.0*1.5, 87, 0, 30, 0.35*1.5, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.8);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2.5, 0, 100, 0, 60, 0.5*1.5, true);
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

#[fighter_frame( agent = FIGHTER_KIND_KIRBY)]
fn kirby_rayman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let copy_kind = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
        let situation_kind = StatusModule::situation_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let frame = MotionModule::frame(boma);
        let end_frame = MotionModule::end_frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true);
        
        if copy_kind == *FIGHTER_KIND_PIKMIN {
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), true, true);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), false, true);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), true, false);

            if situation_kind != *SITUATION_KIND_AIR {
                CAN_NEUTRALB[ENTRY_ID] = 0;
            }
            if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PIKMIN_SPECIAL_N {

                    CAN_NEUTRALB[ENTRY_ID] = 1;
                    if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                    }
                    if frame < 111.0 {
                        if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1){
                            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
                                KineticModule::clear_speed_all(fighter.module_accessor);
                                let lr = PostureModule::lr(fighter.module_accessor);
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
                                PostureModule::set_lr(fighter.module_accessor, lr*-1.0);
                                PostureModule::update_rot_y_lr(boma);
                                KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
                            } else {
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
                            }
                        }
                    }
            }
        } else {
            CAN_NEUTRALB[ENTRY_ID] = 0;
        }
    }
}
#[fighter_frame( agent = FIGHTER_KIND_PIKMIN)]
fn rayman(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let situation_kind = StatusModule::situation_kind(boma);
        let fighter_kind = smash::app::utility::get_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let end_frame = MotionModule::end_frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true);
        let stick_y = ControlModule::get_stick_y(boma);
        let lr = PostureModule::lr(boma);
        let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);

        if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
            if status_kind == *FIGHTER_STATUS_KIND_ENTRY || is_reset() {
                let custom_hurtboxes = [
                    //["bone", x1, y1, z1, x2, y2, z2, scale, collision_part, hit height]
                    [hash40("waist") as f64, 0.2,-0.3, 0.0, 1.4, -0.3, 0.0,   2.2, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("bag") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("head") as f64, 2.0, 0.5, 0.0, 1.2, 0.5, 0.0,   3.0, *COLLISION_PART_HEAD as f64, *HIT_HEIGHT_HIGH as f64],
                    [hash40("kneer") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("kneel") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("armr") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("arml") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("kneer") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("footr") as f64, 0.4, 0.3, 0.0, 0.4, -1.2, 0.0,  1.2, *COLLISION_PART_BODY_LEGS as f64, *HIT_HEIGHT_LOW as f64],
                    [hash40("footl") as f64, 0.4, 0.3, 0.0, 0.4, -1.2, 0.0,  1.2, *COLLISION_PART_BODY_LEGS as f64, *HIT_HEIGHT_LOW as f64]
                ];
                let mut f = 0;
                for i in custom_hurtboxes {
                    let mut vec1 = Vector3f{x: i[1] as f32, y: i[2] as f32, z: i[3] as f32};
                    let mut vec2 = Vector3f{x: i[4] as f32, y: i[5] as f32, z: i[6] as f32};
                    FighterUtil::set_hit_data(boma,f,0,&vec1,&vec2,i[7] as f32,Hash40::new_raw(i[0] as u64),smash::app::CollisionPart(i[8] as i32),smash::app::HitHeight(i[9] as i32),smash::app::HitStatus(*HIT_STATUS_NORMAL),smash::app::CollisionShapeType(*COLLISION_SHAPE_TYPE_CAPSULE));    
                    f += 1;
                }
            }
            if ModelModule::scale(boma) == WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0) {
                ModelModule::set_scale(boma, 1.75);
                AttackModule::set_attack_scale(boma, 1.0, true);
                GrabModule::set_size_mul(boma, 1.75);
            }
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna"), false, false);

            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), true, true);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), false, true);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), true, false);

            EffectModule::kill_kind(boma, Hash40::new("pikmin_wingpikmin_end"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_wingpikmin_wing"), false, false); 
            EffectModule::kill_kind(boma, Hash40::new("pikmin_wingpikmin2_line"), false, false);

            //Stops final smash sound
            if ![hash40("final"), hash40("final_shoot")].contains(&motion_kind) {
                macros::STOP_SE(fighter, Hash40::new("se_common_spirits_pfog_loop"));
            }

            //Stops Charge sound from playing after rayman is hit
            if ![*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START].contains(&status_kind) {
                macros::STOP_SE(fighter, Hash40::new("se_pikmin_smash_s03"));
            }

            //Star KO Shit
            if status_kind == *FIGHTER_STATUS_KIND_DEAD {
                if MotionModule::motion_kind(boma) == hash40("fall_damage") && !HAS_DEADED[ENTRY_ID] {
                    macros::PLAY_SE(fighter, Hash40::new("se_pikmin_final01"));
                    HAS_DEADED[ENTRY_ID] = true;
                };
            } else {
                HAS_DEADED[ENTRY_ID] = false;
            };

            //Neutralb
            if ![hash40("special_air_n_pull"), hash40("special_n_pull")].contains(&motion_kind){
                DO_WALLJUMP_FORCE[ENTRY_ID] = false;
            }
            if [hash40("special_air_n_failure"), hash40("special_n_failure"), hash40("special_n_start")].contains(&motion_kind){
                StatusModule::set_keep_situation_air(boma, true);
                if is_near_ground == 1 && situation_kind == *SITUATION_KIND_AIR {
                    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    if frame > 40.0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
                    }
                }
                let mut into_frame = 0.0;
                let mut check_dist = 0.0;
                //Later into the pull anim if you hit neutralb earlier in the shot... because rayman gotta go less distance
                if PULL_DISTANCE[ENTRY_ID] == 0 {
                    into_frame = 7.0;
                    check_dist = 12.0;
                } else if PULL_DISTANCE[ENTRY_ID] == 1 {
                    into_frame = 5.0;
                    check_dist = 23.0;
                } else if PULL_DISTANCE[ENTRY_ID] == 2 {
                    into_frame = 4.0;
                    check_dist = 27.0;
                } else {
                    into_frame = 0.0;
                    check_dist = 32.0;
                }
                //let dist_pos = &mut Vector3f{x: 0.0, y: 0.0, z: check_dist};
                //let joint = ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("throw"), dist_pos, false);
                let is_wall = (
                    (
                        (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+((6.0+check_dist)*lr), y: PostureModule::pos_y(boma)+10.0}, &Vector2f{ x: 6.0*lr, y: 0.0}, true) == 1)
                    ) ||
                    (
                        (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+((6.0+(check_dist*0.75))*lr), y: PostureModule::pos_y(boma)+10.0}, &Vector2f{ x: 6.0*lr, y: 0.0}, true) == 1)
                    ) ||
                    (
                        (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+((6.0+(check_dist*0.5))*lr), y: PostureModule::pos_y(boma)+10.0}, &Vector2f{ x: 6.0*lr, y: 0.0}, true) == 1)
                    ) ||
                    (
                        (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+((6.0+(check_dist*0.25))*lr), y: PostureModule::pos_y(boma)+10.0}, &Vector2f{ x: 6.0*lr, y: 0.0}, true) == 1)
                    ) ||
                    (
                        (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+((6.0+(check_dist*0.0))*lr), y: PostureModule::pos_y(boma)+10.0}, &Vector2f{ x: 6.0*lr, y: 0.0}, true) == 1)
                    )
                ) && frame < 22.0 && frame > 13.0;
                //println!("X:{}, Y:{}, Z:{}, is_wall:{}", joint.x, joint.y, joint.z, is_wall);
                if is_wall {
                    DO_WALLJUMP_FORCE[ENTRY_ID] = true;
                }
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || is_wall {
                    AttackModule::clear_all(fighter.module_accessor);
                    if situation_kind == *SITUATION_KIND_AIR {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_pull"), into_frame, 0.75, false, 0.0, false, false);
                    } else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_pull"), into_frame, 0.75, false, 0.0, false, false);
                    }
                }
            }
            if [hash40("special_air_n_pull"), hash40("special_n_pull")].contains(&motion_kind){
                StatusModule::set_keep_situation_air(boma, true);
                if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                }
                if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
                    CAN_NEUTRALB[ENTRY_ID] = 1;
                }
                if DO_WALLJUMP_FORCE[ENTRY_ID] {
                    let the_speed = smash::phx::Vector3f { x: 0.5, y: 0.0, z: 0.0 };
                    KineticModule::add_speed(boma, &the_speed);
                }
            };
            //Sideb
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                CAN_SIDEB[ENTRY_ID] = 1;
                if motion_kind != hash40("slide_jump_fall") {
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    StatusModule::set_keep_situation_air(boma, true);
                    if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                    }
                    if end_frame - frame < 3.0 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_jump_fall"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    if frame < 111.0 {
                        if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1){
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
                            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
                                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
                            }
                        }
                    }
                } else {
                    StatusModule::set_keep_situation_air(boma, false);
                    if is_near_ground == 1 {
                        StatusModule::set_keep_situation_air(boma, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
                    }
                    if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                    }
                }
            }
            if situation_kind != *SITUATION_KIND_AIR {
                CAN_SIDEB[ENTRY_ID] = 0;
                CAN_NEUTRALB[ENTRY_ID] = 0;
            }
            if ![hash40("slide_jump_fall"),hash40("capture_jump"),hash40("special_s"),hash40("special_air_s")].contains(&MotionModule::motion_kind(boma)) {
                macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackair_n01"));
            }
            if [hash40("slide_jump_fall")].contains(&MotionModule::motion_kind(boma)) {
                if MotionModule::frame(boma) > 10.0 {
                    macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackair_n01"));
                }
            }
            if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_CLIFF_CATCH].contains(&status_kind) || situation_kind == *SITUATION_KIND_GROUND {
                macros::STOP_SE(fighter, Hash40::new("se_pikmin_special_l03"));
            }
            //Slide Stuff
            if WAS_SLIDE[ENTRY_ID] {
                if motion_kind == hash40("fall") || motion_kind == hash40("fall_f") || motion_kind == hash40("fall_b"){
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_fall"), 0.0, 1.0, false, 0.0, false, false);
                    WAS_SLIDE[ENTRY_ID] = false;
                }
            }
            if motion_kind == hash40("slide_fall"){
                if MotionModule::frame(boma) >= 18.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_jump_fall"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE {
                if motion_kind != hash40("slide") && motion_kind != hash40("slide_stand"){
                    if  stick_y <= -0.5 &&
                    (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD)) {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide"), 0.0, 1.0, false, 0.0, false, false);
                    }
                } else {
                    let lr = PostureModule::lr(boma);
                    let speed = get_speed_x(boma) * lr;
                    MotionModule::set_rate(boma, 0.4);
                    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
                    //WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
                    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
                    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
                    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
                    IS_SLIDE_MOVE[ENTRY_ID] = true;
                    WAS_SLIDE[ENTRY_ID] = true;
                }
            }
            if [hash40("slide")].contains(&motion_kind) {
                let speed = get_speed_x(boma) * lr;
                if speed < 0.1 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_stand"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            if [hash40("slide_attack")].contains(&motion_kind) {
                if end_frame-frame < 3.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_stand"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            if [hash40("slide_attack_lw")].contains(&motion_kind) {
                if end_frame-frame < 3.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
                }
                if frame >= (cancel_frame - 2.0) {
                    if (
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 ||
                        ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) ||
                        ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) ||
                        ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) 
                    ) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
                    }
                }
                if frame > 30.0 {
                    MotionModule::set_rate(boma, 0.5);
                }
            }
            if [hash40("slide"), hash40("slide_attack_lw"), hash40("slide_attack")].contains(&motion_kind) {
                let desired_brake = 0.025;
                let lr = PostureModule::lr(boma);
                let brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
                let speed = get_speed_x(boma) * lr;
                let mut added_speed = brake - desired_brake;
                if speed < 0.0 {
                    added_speed *= -1.0;
                };
                if (speed <= 0.0 && (speed + added_speed) > 0.0) || (speed >= 0.0 && (speed + added_speed) < 0.0) {
                    added_speed = 0.0;
                };
                let the_speed = smash::phx::Vector3f { x: added_speed, y: 0.0, z: 0.0 };
                KineticModule::add_speed(boma, &the_speed);
            }
            if motion_kind == hash40("slide_jump") && end_frame - frame < 3.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_jump_fall"), 0.0, 1.0, false, 0.0, false, false);
            }
            if ![*FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind) {
                IS_SLIDE_MOVE[ENTRY_ID] = false;
                WAS_SLIDE[ENTRY_ID] = false;
            } else if IS_SLIDE_MOVE[ENTRY_ID]{
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
                    if motion_kind != hash40("slide_attack_lw") {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack_lw"), -1.0, 1.0, false, 0.0, false, false);
                        IS_SLIDE_MOVE[ENTRY_ID] = false;
                        WAS_SLIDE[ENTRY_ID] = false;
                    }
                }
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
                }
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                    if motion_kind != hash40("slide_attack_hi") {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack_hi"), -1.0, 1.0, false, 0.0, false, false);
                        IS_SLIDE_MOVE[ENTRY_ID] = false;
                        WAS_SLIDE[ENTRY_ID] = false;
                    }
                }
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
                    if motion_kind != hash40("slide_attack") {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack"), -1.0, 1.0, false, 0.0, false, false);
                        IS_SLIDE_MOVE[ENTRY_ID] = false;
                        WAS_SLIDE[ENTRY_ID] = false;
                    }
                }
                if status_kind == *FIGHTER_STATUS_KIND_JUMP {
                    if motion_kind != hash40("slide_jump") {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_jump"), -1.0, 1.0, false, 0.0, false, false);
                        IS_SLIDE_MOVE[ENTRY_ID] = false;
                        WAS_SLIDE[ENTRY_ID] = false;
                    }
                }
            }
            /*if [hash40("slide_attack_lw")].contains(&motion_kind) {
                if MotionModule::frame(boma) >= 28.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_RUN_BRAKE, false);
                }
            }
            if [hash40("slide_attack")].contains(&motion_kind) {
                if MotionModule::frame(boma) >= 30.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_stand"), 0.0, 1.0, false, 0.0, false, false);
                }
            }*/
            if [hash40("slide_attack_hi")].contains(&motion_kind) {
                if MotionModule::frame(boma) >= 46.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                }
            }
            if [hash40("slide_stand")].contains(&motion_kind) {
                if MotionModule::frame(boma) >= 10.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                }
            }
            if [hash40("slide_jump_fall")].contains(&motion_kind) {
                if stick_y <= -0.5 {
                    GroundModule::pass_floor(boma);
                    if ray_check_pos(boma, 0.0, -0.3, false) == 1 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                        macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    };
                } 
                else {
                    GroundModule::clear_pass_floor(boma);
                    if ray_check_pos(boma, 0.0, -0.3, true) == 1 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                        macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    };
                };
            }
            if [*FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
                if MotionModule::frame(boma) >= 78.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                }
            }
            if ![*FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
                SET_UPB_FREEFALL[ENTRY_ID] = false;
            } else if (frame >= cancel_frame - 5.0 || frame >= end_frame - 5.0) && SET_UPB_FREEFALL[ENTRY_ID]{
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
            }
            if [*FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT,  *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK){
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                SET_UPB_FREEFALL[ENTRY_ID] = true;
            }
            if [*FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind)  {
                if ![124, 127].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)) {
                    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("pikmin_hair"),false);
                    let lr = PostureModule::lr(fighter.module_accessor);
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.25, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.23, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 2.8*lr, 0, 0, 0, 0, 90, 0.21, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.17, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.13, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.09, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.05, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                }
                WorkModule::off_flag(boma, *FIGHTER_PIKMIN_STATUS_SPECIAL_HI_COMMON_FLAG_TURN);
            } else {
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("pikmin_hair"),true);
            }
        }
    }
}
#[weapon_frame( agent = WEAPON_KIND_PIKMIN_PIKMIN)]
fn kill_pikmin(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
		let boma_reference = &mut *boma;
	    let fighter_kind = smash::app::utility::get_kind(boma);
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	    let ENTRY_ID = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let is_rayman_weapon = (WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
        
        if fighter_kind == *WEAPON_KIND_PIKMIN_PIKMIN && is_rayman_weapon {
           ModelModule::set_scale(weapon.module_accessor, 0.00001);
            PostureModule::set_scale(weapon.module_accessor, 0.00001, false);
            let pos = smash::phx::Vector3f { x: 0.0, y: 160.0, z: 0.0 };
            PostureModule::set_pos(weapon.module_accessor, &pos);
            PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
            SoundModule::stop_all_sound(weapon.module_accessor);
            //WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HP);
            //if StatusModule::status_kind(weapon.module_accessor) != *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_DEATH {
            StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_LANDING, false);
            //};
        } else {
            if smash::app::sv_battle_object::is_active(weapon.battle_object_id) {
                CURRENT_PIKMIN[ENTRY_ID] = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
            }
        }
    }
}

#[acmd_script( agent = "pikmin_pikmin", 
scripts = [
    "game_attackairb_brayman", "game_attackairb_vrayman", "game_attackairb_wrayman", "game_attackairb_yrayman", "game_attackairbrayman",
    "game_attackairf_brayman", "game_attackairf_vrayman", "game_attackairf_wrayman", "game_attackairf_yrayman", "game_attackairfrayman",
    "game_attackairlw_brayman", "game_attackairlw_vrayman", "game_attackairlw_wrayman", "game_attackairlw_yrayman", "game_attackairlwrayman",
    "game_attackairhi_brayman", "game_attackairhi_vrayman", "game_attackairhi_wrayman", "game_attackairhi_yrayman", "game_attackairhirayman",
    "game_catchstart_brayman", "game_catchstart_vrayman", "game_catchstart_wrayman", "game_catchstart_yrayman", "game_catchstartrayman",
    "game_catchdash_brayman", "game_catchdash_vrayman", "game_catchdash_wrayman", "game_catchdash_yrayman", "game_catchdashrayman"
], 
category = ACMD_GAME)]
unsafe fn remove_pikmin_scripts(agent: &mut L2CAgentBase) {
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_catch_pull(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchPull();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn catch_pull_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_catch_pull_uniq_process_init(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn catch_pull_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_catch_pull_uniq_process_exit(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_CUT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_catch_cut(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchCut();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_CUT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn catch_cut_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_catch_cut_uniq_process_init(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_catch_wait(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchWait();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn catch_wait_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_catch_wait_uniq_process_init(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn catch_wait_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_catch_wait_uniq_process_exit(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_catch_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchAttack();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_throw(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Throw();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn exec_throw(fighter: &mut L2CFighterCommon) -> L2CValue {
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_throw(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Throw();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn throw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_throw_uniq_process_init(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn throw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_throw_uniq_process_exit(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_dtilt(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        if !IS_SLIDE_MOVE[ENTRY_ID] && motion_kind != hash40("slide_attack_lw"){
            original!(fighter)
        } else {
            if motion_kind != hash40("slide_attack_lw") && motion_kind != hash40("slide_stand") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack_lw"), -1.0, 1.0, false, 0.0, false, false);
                IS_SLIDE_MOVE[ENTRY_ID] = false;
                WAS_SLIDE[ENTRY_ID] = false;
            }
            0.into() 
        }
    }
    else{
        original!(fighter)
    }
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_jab(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        if !IS_SLIDE_MOVE[ENTRY_ID] && motion_kind != hash40("slide_attack"){
            original!(fighter)
        } else {
            if motion_kind != hash40("slide_attack") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack"), -1.0, 1.0, false, 0.0, false, false);
                IS_SLIDE_MOVE[ENTRY_ID] = false;
                WAS_SLIDE[ENTRY_ID] = false;
            }
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            0.into() 
        }
    }
    else{
        original!(fighter)
    }
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        if !IS_SLIDE_MOVE[ENTRY_ID] && motion_kind != hash40("slide_jump_squat"){
            original!(fighter)
        } else {
            if motion_kind != hash40("slide_jump_squat") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_jump_squat"), -1.0, (end_frame/3.0)*2.0, false, 0.0, false, false);
            }
            original!(fighter)
        }
    }
    else{
        original!(fighter)
    }
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_FINAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_final(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let is_near_ground = GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true) == 1;
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32; //Cancel frame
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        if ![hash40("final"), hash40("final_shoot")].contains(&motion_kind) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("final"), -1.0, 1.0, false, 0.0, false, false);
            macros::CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
            macros::FT_SET_FINAL_FEAR_FACE(fighter, 40);
            macros::FT_START_CUTIN(fighter, );
            macros::EFFECT(fighter, Hash40::new("sys_item_arrival"), Hash40::new("top"), 9, 3.5, 1.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 9, 3.5, 1, 0, 0, 0, 4.0, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            macros::PLAY_SE(fighter, Hash40::new("se_common_final_cutin"));
            macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_pfog_loop"));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            FINAL_DURATION[ENTRY_ID] = 900;
            X[ENTRY_ID] = 0.0;
            Y[ENTRY_ID] = 0.0;
            ItemModule::remove_item(fighter.module_accessor, 0);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_SUPERSCOPE), 0, 0, false, false);
            macros::STOP_SE(fighter, Hash40::new("se_item_item_get"));
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        } else {
            if MotionModule::is_end(fighter.module_accessor) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("final"), 0.0, 1.0, false, 0.0, false, false);
            }
            if FINAL_DURATION[ENTRY_ID] < 990 {
                if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_shoot"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            let mut y_add = 0.0;
            let mut x_add = 0.0;
            if stick_x > 0.2 {
                x_add = ((stick_x-0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
                if speed_x > X_MAX || speed_x < -X_MAX{
                    x_add = 0.0;
                };
            };
            if stick_x < -0.2 {
                x_add = ((stick_x+0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
                if speed_x > X_MAX || speed_x < -X_MAX{
                    x_add = 0.0;
                };
            };
            if stick_y > 0.2 {
                y_add = ((stick_y-0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
                if speed_y > Y_MAX || speed_y < -Y_MAX{
                    y_add = 0.0;
                };
            };
            if stick_y < -0.2 {
                y_add = ((stick_y+0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
                if speed_y > Y_MAX || speed_y < -Y_MAX{
                    y_add = 0.0;
                };
            };
            if stick_x > -0.2 && stick_x < 0.2 && stick_y > -0.2 && stick_y < 0.2 {
                if speed_y > 0.0 {
                    y_add = -Y_ACCEL_MUL - Y_ACCEL_ADD;
                } else if speed_y < 0.0{
                    y_add = Y_ACCEL_MUL + Y_ACCEL_ADD;
                };
                let mut x_add = 0.0;
                if speed_x > 0.0 {
                    x_add = -X_ACCEL_MUL - X_ACCEL_ADD;
                } else if speed_x < 0.0{
                    x_add = X_ACCEL_MUL + X_ACCEL_ADD;
                };
            };
            x_add = (stick_x)*X_ACCEL_MUL;
            y_add = (stick_y)*X_ACCEL_MUL;
            if x_add > 0.0 && X[ENTRY_ID] > X_MAX {
                x_add = 0.0;
            };
            if x_add < 0.0 && X[ENTRY_ID] < X_MAX*-1.0 {
                x_add = 0.0;
            };
            if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
                y_add = 0.0;
            };
            if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*-1.0 {
                y_add = 0.0;
            };
            println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
            println!("x_add{}, y_add{}", x_add, y_add);
            let speed = smash::phx::Vector3f { x: x_add, y: y_add, z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speed);
            X[ENTRY_ID] += x_add;
            Y[ENTRY_ID] += y_add;
            FINAL_DURATION[ENTRY_ID] -= 1;
            if FINAL_DURATION[ENTRY_ID] <= 0 {
                macros::STOP_SE(fighter, Hash40::new("se_common_spirits_pfog_loop"));
                macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_l01"));
                macros::PLAY_SE(fighter, Hash40::new("se_common_blowaway_s"));
                macros::EFFECT(fighter, Hash40::new("sys_item_arrival"), Hash40::new("top"), 9, 3.5, 1.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 9, 3.5, 1, 0, 0, 0, 4.0, 0, 0, 0, 0, 0, 0, false);
                ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
                ItemModule::remove_item(fighter.module_accessor, 0);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, false);
            }
        }
        0.into() 
    }
    else{
        original!(fighter)
    }
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_downb(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let is_near_ground = GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true) == 1;
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32; //Cancel frame
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        if ![hash40("special_lw"), hash40("special_air_lw"), hash40("special_air_lw_loop"), hash40("special_lw_land")].contains(&motion_kind) {
            if situation_kind == *SITUATION_KIND_AIR {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, false, 0.0, false, false);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, false, 0.0, false, false);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            }
        } else {
            if motion_kind == hash40("special_air_lw_loop") {
                if (
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 ||
                    ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
                    (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)) ||
                    (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) && ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_stick_y(fighter.module_accessor) >= 0.7 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))
                ) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                }
                if situation_kind == *SITUATION_KIND_GROUND || is_near_ground{
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_land"), -1.0, 1.0, false, 0.0, false, false);
                }
            }
            if motion_kind == hash40("special_lw") {
                if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                }
            }
            if motion_kind == hash40("special_air_lw") {
                if situation_kind == *SITUATION_KIND_GROUND {
                    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_land"), -1.0, 1.0, false, 0.0, false, false);
                }
            }
            if motion_kind == hash40("special_lw_land") {
                KineticModule::clear_speed_all(fighter.module_accessor);
                AttackModule::clear_all(fighter.module_accessor);
                if frame >= cancel_frame || (frame >= 15.0 && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)) {
                    if (
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 ||
                        ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
                        (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))||
                        (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) && ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_stick_y(fighter.module_accessor) >= 0.7 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))
                    ) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                    }
                }
                if MotionModule::is_end(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                }
            }
            if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) && MotionModule::is_end(fighter.module_accessor) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_loop"), -1.0, 1.0, false, 0.0, false, false);
            }
        }
        0.into() 
    }
    else{
        original!(fighter)
    }
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_utilt(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32; //Cancel frame
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        if !IS_SLIDE_MOVE[ENTRY_ID] && motion_kind != hash40("slide_attack_hi"){
            if motion_kind != hash40("attack_hi3") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi3"), -1.0, 1.0, false, 0.0, false, false);
            }
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        } else {
            if motion_kind != hash40("slide_attack_hi") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack_hi"), -1.0, 1.0, false, 0.0, false, false);
                IS_SLIDE_MOVE[ENTRY_ID] = false;
                WAS_SLIDE[ENTRY_ID] = false;
            }
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
        0.into() 
    }
    else{
        original!(fighter)
    }
} 


#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn utilt_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        fighter.sub_status_pre_SpecialNCommon();
        StatusModule::init_settings(
            fighter.module_accessor,
            smash::app::SituationKind(*SITUATION_KIND_NONE),
            *FIGHTER_KINETIC_TYPE_MOTION_AIR,
            *GROUND_CORRECT_KIND_AIR as u32,
            smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0
        );

        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false,
            *FIGHTER_TREADED_KIND_NO_REAC,
            false,
            false,
            false,
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
            0
        );

        0.into()
    }
    else{
        original!(fighter)
    }
}
#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_PIKMIN_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn kirby_copy_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        fighter.sub_status_pre_SpecialNCommon();
        StatusModule::init_settings(
            fighter.module_accessor,
            smash::app::SituationKind(*SITUATION_KIND_NONE),
            *FIGHTER_KINETIC_TYPE_MOTION_AIR,
            *GROUND_CORRECT_KIND_AIR as u32,
            smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0
        );

        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false,
            *FIGHTER_TREADED_KIND_NO_REAC,
            false,
            false,
            false,
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
            0
        );

        0.into()
    }
    else{
        original!(fighter)
    }
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn downb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        fighter.sub_status_pre_SpecialNCommon();
        StatusModule::init_settings(
            fighter.module_accessor,
            smash::app::SituationKind(*SITUATION_KIND_NONE),
            *FIGHTER_KINETIC_TYPE_MOTION_AIR,
            *GROUND_CORRECT_KIND_AIR as u32,
            smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0
        );

        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false,
            *FIGHTER_TREADED_KIND_NO_REAC,
            false,
            false,
            false,
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
            0
        );

        0.into()
    }
    else{
        original!(fighter)
    }
}
pub fn install() {
	install_status_scripts!(
        utilt_pre, main_utilt, //Puts Rayman in the air for utilt

        //Should fix olimar related rayman grab and throw bugs
        main_catch_attack, 
        main_catch_cut, catch_cut_init,
        main_catch_pull, catch_pull_exit, catch_pull_init,
        main_catch_wait, catch_wait_exit, catch_wait_init,
        main_throw, exec_throw, throw_init, throw_exit,

        //Slide Shit
        main_jab, main_dtilt, 
        main_jumpsquat,

        //Downb
        main_downb, downb_pre,

        //Final
        main_final,

        //Kirby
        kirby_copy_pre
    );
    smashline::install_acmd_scripts!(
        //Jab and Dash Attack
		rayman_jab_1, rayman_jab_1_eff, rayman_jab_1_snd,
		rayman_jab_2, rayman_jab_2_eff, rayman_jab_2_snd,
		rayman_jab_3, rayman_jab_3_eff, rayman_jab_3_snd,
		rayman_da, rayman_da_eff, rayman_da_snd,

        //Tilts
		rayman_ftilt, rayman_ftilt_eff, rayman_ftilt_snd,
		rayman_utilt, rayman_utilt_eff, rayman_utilt_snd,
		rayman_dtilt, rayman_dtilt_eff, rayman_dtilt_snd,

        //Smashes
		rayman_usmash, rayman_usmash_charge_eff, rayman_usmash_eff, rayman_usmash_snd,
		rayman_fsmash, rayman_fsmash_charge_eff, rayman_fsmash_charge_snd, rayman_fsmash_eff, rayman_fsmash_snd,
		rayman_dsmash, rayman_dsmash_charge_eff, rayman_dsmash_eff, rayman_dsmash_snd, 

        //Aerials
		rayman_nair, rayman_nair_eff, rayman_nair_snd,
		rayman_fair, rayman_fair_eff, rayman_fair_snd,
        rayman_bair, rayman_bair_eff, rayman_bair_snd,
		rayman_uair, rayman_uair_eff, rayman_uair_snd,
		rayman_dair, rayman_dair_eff, rayman_dair_snd,

        //Specials
        rayman_neutralb, rayman_neutralb_eff, rayman_neutralb_snd,
        rayman_neutralb_air_eff, rayman_neutralb_pull_snd,
        rayman_upb_eff, rayman_upb_snd,
        rayman_sideb, rayman_sideb_eff, rayman_sideb_snd,
        rayman_downb, rayman_air_downb,
        rayman_downb_eff, rayman_air_downb_eff, rayman_downb_land_eff,
        rayman_downb_snd, rayman_air_downb_snd, rayman_downb_land_snd,

        //Grabs
        rayman_grab, rayman_grab_eff, rayman_grab_snd,
        rayman_dashgrab, rayman_dashgrab_snd,
        rayman_pivotgrab, rayman_pivotgrab_eff, rayman_pivotgrab_snd,

        //Throws
        rayman_uthrow, rayman_uthrow_eff, rayman_uthrow_snd,
        rayman_bthrow, rayman_bthrow_eff,
        rayman_fthrow, rayman_fthrow_eff,
        rayman_dthrow, rayman_dthrow_eff, rayman_dthrow_snd,

        //Slide
        rayman_slide_attack, rayman_slide_attack_eff, rayman_slide_attack_snd,
        rayman_slide_dtilt, rayman_slide_dtilt_eff, rayman_slide_dtilt_snd,
        rayman_slide_utilt, rayman_slide_utilt_eff, rayman_slide_utilt_snd,

        //Final Smash
        rayman_final_shoot, rayman_final_wait, rayman_final_wait_eff, rayman_final_wait_expr, rayman_final_wait_snd,
        dolfin_blank,

        //Win
        rayman_win2,
        rayman_win_eff,
        rayman_win_snd,

        //Misc
        rayman_cliffescape_snd,
        rayman_cliffjump2_snd,
        rayman_cliffattack, rayman_cliffattack_eff, rayman_cliffattack_snd,
        rayman_run_eff, rayman_run_snd,
        rayman_walkfast_snd, rayman_walkmiddle_snd, rayman_walkslow_snd,
        rayman_utaunt_eff, rayman_utaunt_snd,
        rayman_dtaunt_eff, rayman_dtaunt_snd,
        rayman_ftauntl_snd, rayman_ftauntr_snd,
        rayman_turnrun_eff,
        rayman_runbraker_eff,
        rayman_runbrakel_eff,
        rayman_downattacku_eff, rayman_downattacku_snd,
        rayman_downattackd_eff, rayman_downattackd_snd,
        rayman_escapef_eff, rayman_escapef_snd,
        rayman_escapeb_eff, rayman_escapeb_snd,
        rayman_escapen_eff,
        pikmin_flashing_eff,
        rayman_dmg_fly_snd, rayman_dmg_fly_eff,
        rayman_dmg_snd, rayman_dmg_eff,
        rayman_star_ko_snd,
        rayman_jumpfrontmini_snd,
        rayman_jumpbackmini_snd,
        rayman_jumpaerialback_snd,
        rayman_jumpaerialfront_snd,
        rayman_squatrv_snd,
        rayman_slipattack_eff, rayman_slipattack_snd,
        rayman_entryl, rayman_entryl_eff, rayman_entryl_snd,
        rayman_entryr, rayman_entryr_eff, rayman_entryr_snd,
        dolfin_entryl_eff, dolfin_entryr_eff,
        remove_pikmin_scripts,

        //Kirby
        kirby_copy, kirby_copy_fx
    );
    smashline::install_agent_frames!(
		kill_pikmin,
		rayman,
        kirby_rayman_frame
    );
}
