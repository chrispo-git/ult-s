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
use crate::sandbag::*;



pub fn install() {
    Agent::new("mariod")
        .sound_acmd("sound_jumpaerialbacksandbag", sandbag_jumpaerialback_snd, Priority::Low)
        .sound_acmd("sound_jumpaerialfrontsandbag", sandbag_jumpaerialfront_snd, Priority::Low)
        .effect_acmd("effect_walkfastsandbag", sandbag_walkfast_eff, Priority::Low)
        .sound_acmd("sound_walkfastsandbag", sandbag_walkfast_snd, Priority::Low)
        .sound_acmd("sound_walkslowsandbag", sandbag_walkslow_snd, Priority::Low)
        .effect_acmd("effect_wait2sandbag", sandbag_wait2_eff, Priority::Low)
        .game_acmd("game_cliffattacksandbag", sandbag_cliffattack, Priority::Low)
        .effect_acmd("effect_cliffattacksandbag", sandbag_cliffattack_eff, Priority::Low)
        .game_acmd("game_downattackdsandbag", sandbag_downattackd, Priority::Low)
        .effect_acmd("effect_downattackdsandbag", sandbag_downattackd_eff, Priority::Low)
        .sound_acmd("sound_downattackdsandbag", sandbag_downattackd_snd, Priority::Low)
        .game_acmd("game_downattackusandbag", sandbag_downattacku, Priority::Low)
        .effect_acmd("effect_downattackusandbag", sandbag_downattacku_eff, Priority::Low)
        .sound_acmd("sound_downattackusandbag", sandbag_downattacku_snd, Priority::Low)
        .game_acmd("game_slipattacksandbag", sandbag_slipattack, Priority::Low)
        .effect_acmd("effect_slipattacksandbag", sandbag_slipattack_eff, Priority::Low)
        .sound_acmd("sound_slipattacksandbag", sandbag_slipattack_snd, Priority::Low)
        .game_acmd("game_appealslsandbag", sandbag_staunt, Priority::Low)
        .game_acmd("game_appealsrsandbag", sandbag_staunt, Priority::Low)
        .effect_acmd("effect_appealslsandbag", sandbag_staunt_eff, Priority::Low)
        .effect_acmd("effect_appealsrsandbag", sandbag_staunt_eff, Priority::Low)
        .sound_acmd("sound_appealslsandbag", sandbag_staunt_snd, Priority::Low)
        .sound_acmd("sound_appealsrsandbag", sandbag_staunt_snd, Priority::Low)
        .game_acmd("game_appealhilsandbag", sandbag_utaunt, Priority::Low)
        .game_acmd("game_appealhirsandbag", sandbag_utaunt, Priority::Low)
        .effect_acmd("effect_appealhilsandbag", sandbag_utaunt_eff, Priority::Low)
        .effect_acmd("effect_appealhirsandbag", sandbag_utaunt_eff, Priority::Low)
        .sound_acmd("sound_appealhilsandbag", sandbag_utaunt_snd, Priority::Low)
        .sound_acmd("sound_appealhirsandbag", sandbag_utaunt_snd, Priority::Low)
        .game_acmd("game_appeallwlsandbag", sandbag_dtaunt, Priority::Low)
        .game_acmd("game_appeallwrsandbag", sandbag_dtaunt, Priority::Low)
        .effect_acmd("effect_appeallwlsandbag", sandbag_dtaunt_eff, Priority::Low)
        .effect_acmd("effect_appeallwrsandbag", sandbag_dtaunt_eff, Priority::Low)
        .sound_acmd("sound_appeallwlsandbag", sandbag_dtaunt_snd, Priority::Low)
        .sound_acmd("sound_appeallwrsandbag", sandbag_dtaunt_snd, Priority::Low)
        .effect_acmd("effect_win1sandbag", sandbag_win1_eff, Priority::Low)
        .sound_acmd("sound_win1sandbag", sandbag_win1_snd, Priority::Low)
        .effect_acmd("effect_win1waitsandbag", sandbag_win1wait_eff, Priority::Low)
        .sound_acmd("sound_win1waitsandbag", sandbag_win1wait_snd, Priority::Low)
        .game_acmd("game_win2sandbag", sandbag_win2, Priority::Low)
        .effect_acmd("effect_win3sandbag", sandbag_win3_eff, Priority::Low)
        .game_acmd("game_entryrsandbag", sandbag_entry, Priority::Low)
        .game_acmd("game_entrylsandbag", sandbag_entry, Priority::Low)
        .game_acmd("game_escapefsandbag", sandbag_escapef, Priority::Low)
        .game_acmd("game_escapebsandbag", sandbag_escapeb, Priority::Low)
        .game_acmd("game_landingfallspecialsandbag", sandbag_special_land, Priority::Low)
        .game_acmd("game_finalsandbag", sandbag_finalsmash, Priority::Low)
        .install();
}



unsafe extern "C" fn sandbag_jumpaerialback_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_throw_01"));
    }
}

unsafe extern "C" fn sandbag_jumpaerialfront_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_throw_01"));
    }
}

unsafe extern "C" fn sandbag_walkfast_eff(agent: &mut L2CAgentBase) {
    for _ in 0..36 {
        wait(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 26.0);
    }
}
unsafe extern "C" fn sandbag_walkfast_snd(agent: &mut L2CAgentBase) {
    for _ in 0..36 {
        wait(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_mariod_step_left_m"), Hash40::new("se_mariod_step_right_m"));
        }
        wait(agent.lua_state_agent, 27.0);
    }
}

unsafe extern "C" fn sandbag_walkslow_snd(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn sandbag_wait2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 78.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sandbag_cliffattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 45, 20, 0, 90, 5.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn sandbag_cliffattack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_atk_smoke"), Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_1);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5.5, 10, 0, 0, 0, 1.55, 0, 0, 0, 0, 0, 360, true);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sandbag_downattackd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, 15.0, Some(0.0), Some(5.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, -15.0, Some(0.0), Some(5.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn sandbag_downattackd_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), -1, 6, 6, 180, 200, 10, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(agent, 2.0, 2.0, 2.0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, 3, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 5, -5.5, 180, 20, -10, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(agent, 2.0, 2.0, 2.0);
    }
}
unsafe extern "C" fn sandbag_downattackd_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
    }
}

unsafe extern "C" fn sandbag_downattacku(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, -15.0, Some(0.0), Some(5.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, 15.0, Some(0.0), Some(5.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn sandbag_downattacku_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), -1, 6, -6, 180, 20, -10, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(agent, 2.0, 2.0, 2.0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, -3, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 5, 5.5, 180, 211, 10, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(agent, 2.0, 2.0, 2.0);
    }
}
unsafe extern "C" fn sandbag_downattacku_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
    }
}

unsafe extern "C" fn sandbag_slipattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 5.0, 0.0, 5.0, -15.0, Some(0.0), Some(5.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 5.0, 0.0, 5.0, 14.0, Some(0.0), Some(5.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn sandbag_slipattack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), -1, 6, -6, 180, 20, -10, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(agent, 2.0, 2.0, 2.0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, -3, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 5, 5.5, 180, 211, 10, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(agent, 2.0, 2.0, 2.0);
    }
}
unsafe extern "C" fn sandbag_slipattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_m"));
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_m"));
    }
}

unsafe extern "C" fn sandbag_staunt(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn sandbag_staunt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 30, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}
unsafe extern "C" fn sandbag_staunt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_audience_cheer_s_ingame_delay"));
    }
}

unsafe extern "C" fn sandbag_utaunt(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn sandbag_utaunt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 30, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn sandbag_utaunt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_audience_cheer_s_ingame_delay"));
    }
}

unsafe extern "C" fn sandbag_dtaunt(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn sandbag_dtaunt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 30, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn sandbag_dtaunt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_audience_cheer_s_ingame_delay"));
    }
}

unsafe extern "C" fn sandbag_win1_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 126.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_soil_landing"), Hash40::new("throw"), 1, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_soil_landing"), Hash40::new("throw"), -1, 0, 0, 180, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn sandbag_win1_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_homerunbat_l"));
    }
}

unsafe extern "C" fn sandbag_win1wait_eff(agent: &mut L2CAgentBase) {
    for _ in 0..200 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_kusudama"), Hash40::new("throw"), 0, 25, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.9);
        }
        wait(agent.lua_state_agent, 30.0);
    }
}
unsafe extern "C" fn sandbag_win1wait_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mariod_special_n01"));
    }
}

unsafe extern "C" fn sandbag_win2(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn sandbag_win3_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 66.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

unsafe extern "C" fn sandbag_entry(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
}

unsafe extern "C" fn sandbag_escapef(agent: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent); 

    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, /*FSM*/ 0.75);
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        PostureModule::reverse_lr(boma);
        PostureModule::update_rot_y_lr(boma);
    }
}
unsafe extern "C" fn sandbag_escapeb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, /*FSM*/ 0.75);
}

unsafe extern "C" fn sandbag_special_land(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, /*FSM*/ 0.5);
}

unsafe extern "C" fn sandbag_finalsmash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 1, 20, 0, 0, 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
        macros::FT_START_CUTIN(agent);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_HOMERUNBAT), 0, 0, false, false);
    }
}