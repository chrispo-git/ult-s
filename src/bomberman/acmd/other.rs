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
	.acmd("game_cliffattackbomb", bomb_cliffattack, Priority::Low)
	.acmd("effect_cliffattackbomb", bomb_cliffattack_eff, Priority::Low)
	.acmd("sound_jumpfrontboom", bomb_jump_snd, Priority::Low)
	.acmd("effect_win1boom", bomb_win1, Priority::Low)
	.acmd("effect_win2boom", bomb_win2, Priority::Low)
	.acmd("effect_win3boom", bomb_win3, Priority::Low)
	.acmd("effect_entryrboom", bomb_entry_eff, Priority::Low)
	.acmd("sound_entryrboom", bomb_entry_snd, Priority::Low)
    .acmd("effect_appealhibomb", bomb_utaunt_eff, Priority::Low)
	.acmd("sound_appealhibomb", bomb_utaunt_snd, Priority::Low)
    .acmd("sound_appeallwbomb", bomb_dtaunt_snd, Priority::Low)
    .acmd("effect_appealsbomb", bomb_ftaunt_eff, Priority::Low)
	.acmd("sound_appealsbomb", bomb_ftaunt_snd, Priority::Low)
	.acmd("game_finalstartbomb", bomb_final_smash, Priority::Low)
	.acmd("game_finalairstartbomb", bomb_final_smash, Priority::Low)
	.acmd("effect_finalstartbomb", bomb_final_smash_eff, Priority::Low)
	.acmd("effect_finalairstartbomb", bomb_final_smash_eff, Priority::Low)
	.acmd("sound_finalstartbomb", bomb_final_smash_snd, Priority::Low)
	.acmd("sound_finalairstartbomb", bomb_final_smash_snd, Priority::Low)
    .install();
}
unsafe extern "C" fn bomb_final_smash_eff(fighter: &mut L2CAgentBase) {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let is_ground = situation_kind == *SITUATION_KIND_GROUND;
    if macros::is_excute(fighter) {
        EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_pacman_final"), false, false, false);
    }
    frame(fighter.lua_state_agent, 145.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 30, 0, 0, 0, 0, 7.5, 0, 0, 0, 0, 0, 0, true);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
    }
    frame(fighter.lua_state_agent, 175.0);
    if macros::is_excute(fighter) {
        if is_ground {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(fighter.lua_state_agent, 177.0);
    if macros::is_excute(fighter) {
        EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_pacman_final_end"), false, false, false);
    }
}
unsafe extern "C" fn bomb_final_smash(fighter: &mut L2CAgentBase) {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let is_ground = situation_kind == *SITUATION_KIND_GROUND;
    if macros::is_excute(fighter) {
        macros::CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(fighter, 5.0, 20.0);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            macros::FT_SET_FINAL_FEAR_FACE(fighter, 200);
            fighter.clear_lua_stack();
            lua_args!(fighter, hash40("d04finalstart.nuanmb"), true, false);
            sv_animcmd::REQ_FINAL_START_CAMERA_arg3(fighter.lua_state_agent);
            macros::FT_START_CUTIN(fighter);
            fighter.clear_lua_stack();
            lua_args!(fighter, true);
            sv_animcmd::FT_REMOVE_FINAL_AURA(fighter.lua_state_agent);
        }
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        macros::CAM_ZOOM_OUT(fighter);
    }
    frame(fighter.lua_state_agent, 146.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 40.0, 361, 40, 0, 120, 60, 0.0, 30.0, 0.0, None, None, None, 1.2, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 149.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn bomb_final_smash_snd(fighter: &mut L2CAgentBase) {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let is_ground = situation_kind == *SITUATION_KIND_GROUND;
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_wind_loop"));
        macros::PLAY_SE(fighter, Hash40::new("se_pacman_final01"));
    }
    frame(fighter.lua_state_agent, 146.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_spirits_wind_loop"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}

unsafe extern "C" fn bomb_utaunt_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 53.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn bomb_utaunt_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pacman_attackdash"));
    }
    frame(fighter.lua_state_agent, 53.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pacman_landing01"));
    }
}

unsafe extern "C" fn bomb_dtaunt_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pacman_jump01"));
    }
}

unsafe extern "C" fn bomb_ftaunt_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 64.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.92, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn bomb_ftaunt_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pacman_swing_m"));
    }
    frame(fighter.lua_state_agent, 64.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pacman_landing01"));
    }
}

unsafe extern "C" fn bomb_cliffattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 45, 20, 0, 90, 5.0, 0.0, 5.0, 12.0, Some(0.0), Some(5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn bomb_cliffattack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2, 0.5, 0.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 3, 3, 0, 0, 10, 1.1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}
unsafe extern "C" fn bomb_jump_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }   
}
unsafe extern "C" fn bomb_win1(fighter: &mut L2CAgentBase) {
    for _ in 0..30 {
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0, -6, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(fighter, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0, -6, -6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
    frame(fighter.lua_state_agent, 68.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 94.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn bomb_win2(fighter: &mut L2CAgentBase) {

}


unsafe extern "C" fn bomb_win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 92.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}



unsafe extern "C" fn bomb_entry_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}



unsafe extern "C" fn bomb_entry_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}