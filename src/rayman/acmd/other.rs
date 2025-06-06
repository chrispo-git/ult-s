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

unsafe extern "C" fn rayman_grab(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_grab_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_swing_05"));
    }
}
unsafe extern "C" fn rayman_dashgrab(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_dashgrab_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_swing_05"));
    }
}
unsafe extern "C" fn rayman_pivotgrab(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_pivotgrab_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_swing_05"));
    }
}	

unsafe extern "C" fn rayman_grab_eff(fighter: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn rayman_pivotgrab_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3.8, 0, 0, 180, 0, 0.6, false, 0.75);
    }
}

unsafe extern "C" fn rayman_cliffattack(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_cliffattack_eff(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_cliffattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_dash_start"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s04"));
    }
}
unsafe extern "C" fn rayman_run_eff(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_run_snd(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_walkfast_snd(fighter: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn rayman_walkmiddle_snd(fighter: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn rayman_walkslow_snd(fighter: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn rayman_utaunt_eff(fighter: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn rayman_utaunt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_appeal_h01"));
    }
}
unsafe extern "C" fn rayman_dtaunt_eff(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_dtaunt_snd(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn rayman_turnrun_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -7, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -7, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn rayman_runbraker_eff(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_runbrakel_eff(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_downattacku(agent: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_downattacku_eff(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_downattacku_snd(agent: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_downattackd(agent: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_downattackd_eff(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_downattackd_snd(agent: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_ftauntl_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_hit_nodamage"));
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_hit_nodamage"));
    }
}
unsafe extern "C" fn rayman_ftauntr_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_hit_nodamage"));
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_hit_nodamage"));
    }
}
unsafe extern "C" fn rayman_escapef_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn rayman_escapef_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_escapeair"));
    }
    wait(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_step_right_m"));
    }
}
unsafe extern "C" fn rayman_cliffescape_snd(agent: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_cliffjump2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_jump01"));
    }
}
unsafe extern "C" fn rayman_escapeb_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn rayman_escapeb_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_escapeair"));
    }
    wait(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_step_right_m"));
    }
}
unsafe extern "C" fn rayman_escapen_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn pikmin_flashing_eff(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn rayman_squatrv_snd(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn rayman_dmg_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		dmg_vc(fighter);
	};
}	
unsafe extern "C" fn rayman_dmg_fly_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		dmg_fly_vc(fighter);
	};
}	
unsafe extern "C" fn rayman_dmg_fly_eff(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	
}	
unsafe extern "C" fn rayman_star_ko_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DEAD_UP_FALL) {
			macros::PLAY_SE(fighter, Hash40::new("se_pikmin_final01"));
		};
	};
}
unsafe extern "C" fn rayman_jumpfrontmini_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_jump03"));
        jump_vc(fighter);
    }
}
unsafe extern "C" fn rayman_jumpbackmini_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_jump03"));
        jump_vc(fighter);
    }
}
unsafe extern "C" fn rayman_jumpaerialback_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_jump02"));
        jump_aerial_vc(fighter);
    }
}
unsafe extern "C" fn rayman_jumpaerialfront_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_jump02"));
        jump_aerial_vc(fighter);
    }
}
unsafe extern "C" fn rayman_slipattack_eff(agent: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_slipattack_snd(agent: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_entryl(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
unsafe extern "C" fn rayman_entryl_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn rayman_entryl_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_pikmin_landing01"));
    }
}
unsafe extern "C" fn rayman_entryr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
unsafe extern "C" fn rayman_entryr_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn rayman_entryr_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_pikmin_landing01"));
    }
}
unsafe extern "C" fn dolfin_entryl_eff(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn dolfin_entryr_eff(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn dolfin_blank(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn rayman_final_wait(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn rayman_final_wait_eff(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn rayman_final_wait_snd(agent: &mut L2CAgentBase) {
   
}
unsafe extern "C" fn rayman_final_wait_expr(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn rayman_final_shoot(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn rayman_dmg_eff(agent: &mut L2CAgentBase) {
}
unsafe extern "C" fn rayman_win2(fighter: &mut L2CAgentBase) {
}
unsafe extern "C" fn rayman_win_eff(fighter: &mut L2CAgentBase) {
}
unsafe extern "C" fn rayman_win_snd(fighter: &mut L2CAgentBase) {
}

unsafe extern "C" fn kirby_copy(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn kirby_copy_fx(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn remove_pikmin_scripts(agent: &mut L2CAgentBase) {
}

pub fn install() {
    Agent::new("pikmin")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
        .game_acmd("game_catchrayman", rayman_grab, Priority::Low)
        .sound_acmd("sound_catchrayman", rayman_grab_snd, Priority::Low)
        .game_acmd("game_catchdashrayman", rayman_dashgrab, Priority::Low)
        .sound_acmd("sound_catchdashrayman", rayman_dashgrab_snd, Priority::Low)
        .game_acmd("game_catchturnrayman", rayman_pivotgrab, Priority::Low)
        .sound_acmd("sound_catchturnrayman", rayman_pivotgrab_snd, Priority::Low)
        .effect_acmd("effect_catchrayman", rayman_grab_eff, Priority::Low)
        .effect_acmd("effect_catchturnrayman", rayman_pivotgrab_eff, Priority::Low)
        .game_acmd("game_cliffattackrayman", rayman_cliffattack, Priority::Low)
        .effect_acmd("effect_cliffattackrayman", rayman_cliffattack_eff, Priority::Low)
        .sound_acmd("sound_cliffattackrayman", rayman_cliffattack_snd, Priority::Low)
        .effect_acmd("effect_runrayman", rayman_run_eff, Priority::Low)
        .sound_acmd("sound_runrayman", rayman_run_snd, Priority::Low)
        .sound_acmd("sound_walkfastrayman", rayman_walkfast_snd, Priority::Low)
        .sound_acmd("sound_walkfastrayman", rayman_walkmiddle_snd, Priority::Low)
        .sound_acmd("sound_walkfastrayman", rayman_walkslow_snd, Priority::Low)
        .effect_acmd("effect_appealhirrayman", rayman_utaunt_eff, Priority::Low)
        .effect_acmd("effect_appealhilrayman", rayman_utaunt_eff, Priority::Low)
        .sound_acmd("sound_appealhilrayman", rayman_utaunt_snd, Priority::Low)
        .sound_acmd("sound_appealhirrayman", rayman_utaunt_snd, Priority::Low)
        .effect_acmd("effect_appeallwrrayman", rayman_dtaunt_eff, Priority::Low)
        .effect_acmd("effect_appeallwlrayman", rayman_dtaunt_eff, Priority::Low)
        .sound_acmd("sound_appeallwrrayman", rayman_dtaunt_snd, Priority::Low)
        .sound_acmd("sound_appeallwlrayman", rayman_dtaunt_snd, Priority::Low)
        .effect_acmd("effect_turnrunrayman", rayman_turnrun_eff, Priority::Low)
        .effect_acmd("effect_runbrakerrayman", rayman_runbraker_eff, Priority::Low)
        .effect_acmd("effect_runbrakelrayman", rayman_runbrakel_eff, Priority::Low)
        .game_acmd("game_downattackurayman", rayman_downattacku, Priority::Low)
        .effect_acmd("effect_downattackurayman", rayman_downattacku_eff, Priority::Low)
        .sound_acmd("sound_downattackurayman", rayman_downattacku_snd, Priority::Low)
        .game_acmd("game_downattackdrayman", rayman_downattackd, Priority::Low)
        .effect_acmd("effect_downattackdrayman", rayman_downattackd_eff, Priority::Low)
        .sound_acmd("sound_downattackdrayman", rayman_downattackd_snd, Priority::Low)
        .sound_acmd("sound_appealslrayman", rayman_ftauntl_snd, Priority::Low)
        .sound_acmd("sound_appealsrrayman", rayman_ftauntr_snd, Priority::Low)
        .effect_acmd("effect_escapefrayman", rayman_escapef_eff, Priority::Low)
        .sound_acmd("sound_escapefrayman", rayman_escapef_snd, Priority::Low)
        .sound_acmd("sound_cliffescaperayman", rayman_cliffescape_snd, Priority::Low)
        .sound_acmd("sound_cliffjump2rayman", rayman_cliffjump2_snd, Priority::Low)
        .effect_acmd("effect_escapebrayman", rayman_escapeb_eff, Priority::Low)
        .sound_acmd("sound_escapebrayman", rayman_escapeb_snd, Priority::Low)
        .effect_acmd("effect_escapenrayman", rayman_escapen_eff, Priority::Low)
        .sound_acmd("sound_squatrvrayman", rayman_squatrv_snd, Priority::Low)
        .sound_acmd("sound_damagehi1rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damagehi2rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damagehi3rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damagen1rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damagen2rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damagen3rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damagelw1rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damagelw2rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damagelw3rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damageair1rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damageair2rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damageair3rayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damageelecrayman", rayman_dmg_snd, Priority::Low)
        .sound_acmd("sound_damageflyhirayman", rayman_dmg_fly_snd, Priority::Low)
        .sound_acmd("sound_damageflynrayman", rayman_dmg_fly_snd, Priority::Low)
        .sound_acmd("sound_damageflylwrayman", rayman_dmg_fly_snd, Priority::Low)
        .sound_acmd("sound_damageflytoprayman", rayman_dmg_fly_snd, Priority::Low)
        .sound_acmd("sound_damageflyrollrayman", rayman_dmg_fly_snd, Priority::Low)
        .sound_acmd("sound_damageflymeteorrayman", rayman_dmg_fly_snd, Priority::Low)
        .effect_acmd("effect_damageflyhirayman", rayman_dmg_fly_eff, Priority::Low)
        .effect_acmd("effect_damageflynrayman", rayman_dmg_fly_eff, Priority::Low)
        .effect_acmd("effect_damageflylwrayman", rayman_dmg_fly_eff, Priority::Low)
        .effect_acmd("effect_damageflytoprayman", rayman_dmg_fly_eff, Priority::Low)
        .effect_acmd("effect_damageflyrollrayman", rayman_dmg_fly_eff, Priority::Low)
        .effect_acmd("effect_damageflymeteorrayman", rayman_dmg_fly_eff, Priority::Low)
        .sound_acmd("sound_damagefallrayman", rayman_star_ko_snd, Priority::Low)
        .sound_acmd("sound_jumpfrontminirayman", rayman_jumpfrontmini_snd, Priority::Low)
        .sound_acmd("sound_jumpbackminirayman", rayman_jumpbackmini_snd, Priority::Low)
        .sound_acmd("sound_jumpaerialbackrayman", rayman_jumpaerialback_snd, Priority::Low)
        .sound_acmd("sound_jumpaerialfrontrayman", rayman_jumpaerialfront_snd, Priority::Low)
        .effect_acmd("effect_slipattackrayman", rayman_slipattack_eff, Priority::Low)
        .sound_acmd("sound_slipattackrayman", rayman_slipattack_snd, Priority::Low)
        .game_acmd("game_entrylrayman", rayman_entryl, Priority::Low)
        .effect_acmd("effect_entrylrayman", rayman_entryl_eff, Priority::Low)
        .sound_acmd("sound_entrylrayman", rayman_entryl_snd, Priority::Low)
        .game_acmd("game_entryrrayman", rayman_entryr, Priority::Low)
        .effect_acmd("effect_entryrrayman", rayman_entryr_eff, Priority::Low)
        .sound_acmd("sound_entryrrayman", rayman_entryr_snd, Priority::Low)
        .game_acmd("game_finalrayman", rayman_final_wait, Priority::Low)
        .effect_acmd("effect_finalrayman", rayman_final_wait_eff, Priority::Low)
        .sound_acmd("sound_finalrayman", rayman_final_wait_snd, Priority::Low)
        .expression_acmd("expression_finalrayman", rayman_final_wait_expr, Priority::Low)
        .game_acmd("game_finalshootrayman", rayman_final_shoot, Priority::Low)
        .install();

    Agent::new("pikmin_pikmin")
        .effect_acmd("effect_flashingrayman", pikmin_flashing_eff, Priority::Low)
        .install();

    Agent::new("pikmin_dolfin")
        .effect_acmd("effect_entrylrayman", dolfin_entryl_eff, Priority::Low)
        .effect_acmd("effect_entryrrayman", dolfin_entryr_eff, Priority::Low)
        .game_acmd("game_finalstartrayman", dolfin_blank, Priority::Low)
        .game_acmd("game_finalrayman", dolfin_blank, Priority::Low)
        .game_acmd("game_finalattackrayman", dolfin_blank, Priority::Low)
        .install();
}