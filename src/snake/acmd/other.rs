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
    Agent::new("snake")
    .acmd("sound_appealsr", snake_side_taunt_snd)    
    .acmd("sound_appealhir", snake_up_taunt_snd)    
    .acmd("game_appealendexplode", snake_down_taunt_explode_game)    
    .acmd("expression_appealendexplode", snake_down_taunt_explode_exp)    
    .acmd("sound_appealendexplode", snake_down_taunt_explode_snd)    
    .acmd("effect_appealendexplode", snake_down_taunt_explode_eff)    
    .install();
}

unsafe extern "C" fn snake_side_taunt_snd(fighter : &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("vc_snake_win03"));
        }
}

unsafe extern "C" fn snake_up_taunt_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::AREA_WIND_2ND_arg10(fighter, 0, 2, 360/*angle*/, 10/*size*/, 1, 0, 12, 30, 30, 80);
        // physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.2, 0.2, -1, 0.7, 0.5, -1, Hash40::new("invalid"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_snake_appealhi"));
    }
}


unsafe extern "C" fn snake_down_taunt_explode_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0 );
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, 0);
    }
    frame(fighter.lua_state_agent, 80.0);
    if macros::is_excute(fighter) {
        // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
        // SNAKE_C4_FLAG_IS_SHOWTIME[entry_id] = true;
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 90.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
unsafe extern "C" fn snake_down_taunt_explode_exp(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0 );
    }
    frame(fighter.lua_state_agent, 30.0);
    slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    frame(fighter.lua_state_agent, 75.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 5, false, 0);
    }
}
unsafe extern "C" fn snake_down_taunt_explode_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_snake_appealend"));
    }
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_l04"));
        macros::PLAY_SE(fighter, Hash40::new("se_snake_squat"));
    }
    // frame(fighter.lua_state_agent, 70.0);
    // if is_excute(fighter) {
    // }
    frame(fighter.lua_state_agent, 75.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_l05"));
    }
}
unsafe extern "C" fn snake_down_taunt_explode_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 75.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}