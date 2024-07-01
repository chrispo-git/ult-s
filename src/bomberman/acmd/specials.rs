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
use crate::bomberman::*;

pub fn install() {
	Agent::new("pacman")
	.acmd("sound_speciallwbomb", bomb_downb_snd, Priority::Low)
	.acmd("game_specialairhiboom", bomb_upb, Priority::Low)
	.acmd("sound_specialairhiboom", bomb_upb_snd, Priority::Low)
	.acmd("sound_specialhiboom", bomb_upb_snd, Priority::Low)
	.acmd("game_specialhiboom", bomb_upb, Priority::Low)
	.acmd("effect_specialairhiboom", bomb_upb_eff, Priority::Low)
	.acmd("effect_specialhiboom", bomb_upb_eff, Priority::Low)
	.acmd("game_specialairnshootboom", bomb_neutralb, Priority::Low)
	.acmd("game_specialnshootboom", bomb_neutralb, Priority::Low)
	.acmd("effect_specialairnshootboom", bomb_neutralb_eff, Priority::Low)
	.acmd("effect_specialnshootboom", bomb_neutralb_eff, Priority::Low)
	.acmd("sound_specialairnshootboom", bomb_neutralb_snd, Priority::Low)
	.acmd("sound_specialnshootboom", bomb_neutralb_snd, Priority::Low)
	.acmd("game_speciallwfailurebomb", bomb_detonate, Priority::Low)
	.acmd("game_specialairlwfailurebomb", bomb_detonate, Priority::Low)
	.acmd("effect_speciallwfailurebomb", bomb_detonate_eff, Priority::Low)
	.acmd("effect_specialairlwfailurebomb", bomb_detonate_eff, Priority::Low)
	.acmd("sound_speciallwfailurebomb", bomb_detonate_snd, Priority::Low)
	.acmd("sound_specialairlwfailurebomb", bomb_detonate_snd, Priority::Low)
	.acmd("game_specialsdashboom", bomb_sideb_catch, Priority::Low)
	.acmd("game_specialsdashboom", bomb_sideb_catch, Priority::Low)
	.acmd("sound_specialsdashboom", bomb_sideb_catch_snd, Priority::Low) 
	.acmd("effect_specialsdashboom", bomb_sideb_catch_eff, Priority::Low) 
	.acmd("effect_specialairsdashboom", bomb_sidebair_catch_eff, Priority::Low) 
	.acmd("game_specialsstartboom", bomb_sideb_start, Priority::Low)
	.acmd("game_specialairsstartboom", bomb_sideb_start, Priority::Low)
	.acmd("game_specialsbomb", bomb_sideb_throw, Priority::Low)
	.acmd("game_specialsendboom", bomb_sideb_end, Priority::Low)
	.acmd("effect_specialsendboom", bomb_sideb_end_eff, Priority::Low)
	.acmd("sound_specialsendboom", bomb_sideb_end_snd, Priority::Low)
	.acmd("effect_specialnholdbomb", bomb_neutralbcharge_eff, Priority::Low)
	.acmd("sound_specialnholdbomb", bomb_neutralbcharge_snd, Priority::Low)
	.acmd("effect_specialairnholdbomb", bomb_neutralbaircharge_eff, Priority::Low)
	.acmd("effect_specialsmissboom", bomb_sideb_miss_eff, Priority::Low)
	.install();

    Agent::new("pacman_firehydrant")
    .acmd("game_fly", bomb_bomb_fly, Priority::Low)
	.acmd("game_fall", bomb_bomb_fall, Priority::Low)
	.acmd("game_down", bomb_bomb_down, Priority::Low)
	.acmd("game_wait", bomb_bomb_wait, Priority::Low)
	.install();
}

unsafe extern "C" fn bomb_neutralbcharge_eff(agent: &mut L2CAgentBase) {
    for _ in 0..100 {
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 20, 0, 12, 0, 0, 0, false);
            macros::FLASH(agent, 1, 125.0/255.0, 125.0/255.0, 0.259);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 60.0/255.0, 79.0/255.0, 0.133);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 100.0/255.0, 3.0/255.0, 0, 0.384);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 4.0);
    }
}
unsafe extern "C" fn bomb_neutralbaircharge_eff(agent: &mut L2CAgentBase) {
    for _ in 0..100 {
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            macros::FLASH(agent, 1, 125.0/255.0, 125.0/255.0, 0.259);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 60.0/255.0, 79.0/255.0, 0.133);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 100.0/255.0, 3.0/255.0, 0, 0.384);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 4.0);
    }
}
unsafe extern "C" fn bomb_neutralbcharge_snd(agent: &mut L2CAgentBase) {
    for _ in 0..100 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

unsafe extern "C" fn bomb_downb_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_landing_famicom"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"));
    }   
}

unsafe extern "C" fn bomb_upb_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_blowaway_m"));
    }   
}
unsafe extern "C" fn bomb_upb(fighter: &mut L2CAgentBase) {
    for _ in 0..10 {
        if macros::is_excute(fighter) {
            KineticModule::clear_speed_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::SET_SPEED_EX(fighter, 1.0, 3.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 45, /*KBG*/ 53, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
	}
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 45, /*KBG*/ 53, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
	}
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn bomb_upb_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    for _ in 0..10 {
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0, -6, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(fighter, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0, -6, -6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
}
unsafe extern "C" fn bomb_neutralb(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 350, 100, 50, 0, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        if smash::app::utility::get_kind(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) != *FIGHTER_KIND_PACMAN {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_FLAG_THROW);
        }
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 6.0 + (10.0*((NEUTRALB_CHARGE[ENTRY_ID] as f32)/(NEUTRALB_MAX as f32))), 361, 80, 0, 45, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        NEUTRALB_CHARGE[ENTRY_ID] = 0;
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn bomb_neutralb_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_smashswing_01"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_07"));
    }   
}
unsafe extern "C" fn bomb_neutralb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1, 6.5, -5, 0, -5, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("handr"), 3, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.9);
    }
}
unsafe extern "C" fn bomb_detonate(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let is_left = PostureModule::lr(fighter.module_accessor) < 0.0;
    if macros::is_excute(fighter) {
        if is_left {
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhavel"),false);
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhaver"),true);
        } else {
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhavel"),true);
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhaver"),false);
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        EXPLODE[ENTRY_ID] = true;
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhavel"),false);
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("bomber_switchhaver"),false);
    }
}
unsafe extern "C" fn bomb_detonate_eff(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        }
    }
}
unsafe extern "C" fn bomb_detonate_snd(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) {
            macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
        }
    }
}
unsafe extern "C" fn bomb_bomb_fly(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let is_bomb = (WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_bomb {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("rot2"), 1.0, 361, 100, 5, 0, 5.0, 0.0, -1.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 3.0, /*Unk*/ false);
        }
    } else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("rot2"), 13.0, 45, 90, 0, 30, 5.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}
unsafe extern "C" fn bomb_bomb_fall(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let is_bomb = (WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_bomb {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 100, 5, 0, 5.0, 0.0, 6.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 3.0, /*Unk*/ false);
        }
    } else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 60, 80, 0, 60, 5.0, 0.0, 6.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(agent.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
}
unsafe extern "C" fn bomb_bomb_down(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let is_bomb = (WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_bomb {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("rot2"), 1.0, 361, 100, 5, 0, 5.0, 0.0, -1.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 3.0, /*Unk*/ false);
        }
    } else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("rot2"), 10.0, 45, 90, 0, 30, 5.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}
unsafe extern "C" fn bomb_bomb_wait(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let is_bomb = (WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if !is_bomb {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 5.0, 0.0, 6.0, 0.0, Some(0.0), Some(4.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        }
    }
}
unsafe extern "C" fn bomb_sideb_catch_snd(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, /*FSM*/ 0.5);
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..6 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_wallhit"));
        }
        wait(agent.lua_state_agent, 3.0);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_step_soft"));
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}
unsafe extern "C" fn bomb_sideb_catch_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}
unsafe extern "C" fn bomb_sidebair_catch_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn bomb_sideb_miss_eff(agent: &mut L2CAgentBase) {
    for _ in 0..2 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}
unsafe extern "C" fn bomb_sideb_catch(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, /*FSM*/ 0.5);
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 6.6, 9.3, Some(0.0), Some(6.6), Some(10.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.2, 0.0, 6.6, 8.7, Some(0.0), Some(6.6), Some(12.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
    macros::game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, /*FSM*/ 1.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}
unsafe extern "C" fn bomb_sideb_start(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
}

unsafe extern "C" fn bomb_sideb_throw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_THROW_FLAG_START_AIR);
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 80, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 72, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 1, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 50.0);
    for _ in 0..1000 {
        if macros::is_excute(fighter) {
            macros::SET_SPEED_EX(fighter, 0.0, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
}
unsafe extern "C" fn bomb_sideb_end(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
        LinkModule::unlink_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 12.0, 80, 40, 0, 120, 20.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn bomb_sideb_end_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_bomb_b"), Hash40::new("top"), 16, -2, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
    }
}
unsafe extern "C" fn bomb_sideb_end_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}