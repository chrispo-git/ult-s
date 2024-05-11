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
    Agent::new("szerosuit")
    .acmd("game_catch", zss_grab, Priority::Low)    
    .acmd("sound_catch", zss_grab_snd, Priority::Low)    
    .acmd("effect_catch", zss_grab_eff, Priority::Low)    
    .acmd("game_catchdash", game_catchdash, Priority::Low)    
    .acmd("game_catchturn", game_catchturn, Priority::Low)    
    .acmd("effect_catchdash", effect_catchdash2, Priority::Low)    
    .acmd("effect_catchturn", effect_catchturn2, Priority::Low)    
    .install();

    Agent::new("szerosuit_whip")
    .acmd("effect_catchdash", effect_catchdash, Priority::Low)    
    .acmd("effect_catchturn", effect_catchturn, Priority::Low)    
    .acmd("game_catchdash", game_catchdash2, Priority::Low)    
    .acmd("game_catchturn", game_catchturn2, Priority::Low)    
    .install();
}

unsafe extern "C" fn zss_pummel(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 8.3, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(1.0), /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
			AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 7.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.4);
}	
unsafe extern "C" fn zss_pummel_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 11, 3, -3.907, 12.664, 9.16, 0.6, true, 0.4);
			macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
			macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
}

unsafe extern "C" fn zss_grab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
		}
		frame(fighter.lua_state_agent, 4.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
			macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 3.3, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(13.4), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
			macros::CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 1.65, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(16.05), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
		}
		if true{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
		}
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			CancelModule::enable_cancel(fighter.module_accessor);
		}
}	
unsafe extern "C" fn zss_grab_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_szerosuit_swing_s"));
		}
}
unsafe extern "C" fn zss_grab_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, Hash40::new("catch_dash"), false, -1.0);
    }
	frame(agent.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(agent, 0.5);
	frame(agent.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("handr"), 2.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, Hash40::new("catch_turn"), false, -1.0);
    }
	frame(agent.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(agent, 0.5);
	frame(agent.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("handr"), 2.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 57.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
unsafe extern "C" fn effect_catchdash(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(agent, 0.5);
	frame(agent.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("szero_whip_flash"), Hash40::new("plasmawhip1"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_whip"), Hash40::new("attach"), 0, 0, 0, 0, 0, 0, 0.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("szero_whip_catch"), Hash40::new("attach"), 0, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip2"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip3"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip4"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip5"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip6"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip7"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip8"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("szero_whip_catch"), false, true);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("szero_gbeam_lightning"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("szero_whip_flash"), false, true);
    }
}
unsafe extern "C" fn effect_catchturn(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(agent, 0.5);
	frame(agent.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("szero_whip_flash"), Hash40::new("plasmawhip1"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_whip"), Hash40::new("attach"), 0, 0, 0, 0, 0, 0, 0.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("szero_whip_catch"), Hash40::new("attach"), 0, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip2"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip3"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip4"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip5"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip6"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip7"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip8"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("szero_whip_catch"), false, true);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("szero_gbeam_lightning"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("szero_whip_flash"), false, true);
    }
}
unsafe extern "C" fn game_catchdash2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
	frame(agent.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(agent, 0.5);
	frame(agent.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
}
unsafe extern "C" fn game_catchturn2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
	frame(agent.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(agent, 0.5);
	frame(agent.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
}
unsafe extern "C" fn effect_catchdash2(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(agent, 0.5);
	frame(agent.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("szero_whip_shot"), Hash40::new("haver"), 0, 0, 0, 0, 0, 90, 0.4, true);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("szero_whip_vanish"), Hash40::new("haver"), 0, 0, 0, 0, 0, 90, 0.6, true);
    }
}

unsafe extern "C" fn effect_catchturn2(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(agent, 0.5);
	frame(agent.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("szero_whip_shot"), Hash40::new("haver"), 0, 0, 0, 0, 0, 90, 0.4, true);
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("szero_whip_vanish"), Hash40::new("haver"), 0, 0, 0, 0, 0, 90, 0.6, true);
    }
}