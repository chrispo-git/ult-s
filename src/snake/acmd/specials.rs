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
    .acmd("game_specialsstart", snake_side_special_game, Priority::Low)    
    .acmd("game_specialairsstart", snake_side_special_air_game, Priority::Low)    
    .acmd("expression_specialsstart", snake_side_special_expr, Priority::Low)    
    .acmd("expression_specialairsstart", snake_side_special_air_expr, Priority::Low)    
    .acmd("sound_specialsstart", snake_side_special_snd, Priority::Low)    
    .acmd("sound_specialairsstart", snake_side_special_air_snd, Priority::Low)    
    .acmd("effect_specialsstart", snake_side_special_eff, Priority::Low)    
    .acmd("effect_specialairsstart", snake_side_special_air_eff, Priority::Low)    
    .install();

    Agent::new("snake_nikita")
    .acmd("sound_start", snake_tranq_gun_start_snd, Priority::Low)    
    .acmd("sound_shoot", snake_tranq_gun_shoot_snd, Priority::Low)    
    .install();

    Agent::new("snake_nikitamissile")
    .acmd("game_fly", snake_tranq_dart_fly_game, Priority::Low)    
    .acmd("game_flyattackcommon", snake_tranq_dart_fly_game, Priority::Low)    
    .acmd("game_stopfall", snake_tranq_dart_fly_game, Priority::Low)    
    .acmd("sound_fly", snake_tranq_dart_fly_snd, Priority::Low)    
    .acmd("sound_stopfall", snake_tranq_dart_fly_snd, Priority::Low)    
    .acmd("effect_fly", snake_tranq_dart_fly_eff, Priority::Low)    
    .acmd("effect_stopfall", snake_tranq_dart_fall_eff, Priority::Low)    
    .acmd("game_explosion", snake_tranq_dart_explode_game, Priority::Low)    
    .acmd("game_fallexplosion", snake_tranq_dart_explode_game, Priority::Low)    
    .acmd("game_hiexplosion", snake_tranq_dart_explode_game, Priority::Low)    
    .acmd("sound_explosion", snake_tranq_dart_explode_snd, Priority::Low)    
    .acmd("sound_fallexplosion", snake_tranq_dart_explode_snd, Priority::Low)    
    .acmd("sound_hiexplosion", snake_tranq_dart_explode_snd, Priority::Low)    
    .acmd("effect_explosion", snake_tranq_dart_explode_eff, Priority::Low)    
    .acmd("effect_hiexplosion", snake_tranq_dart_explode_eff, Priority::Low)    
    .acmd("effect_fallexplosion", snake_tranq_dart_land_eff, Priority::Low)    
    .install();
}

unsafe extern "C" fn snake_side_special_game(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, false, 0);
    }
    // frame(lua_state_agent, 25.0);
    // if macros::is_excute(fighter) {
    //     FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 8, 4);
    // }
    // frame(lua_state_agent, 27.0);
    // if macros::is_excute(fighter) {
    //     ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, true, *WEAPON_SNAKE_NIKITA_INSTANCE_WORK_ID_FLAG_LIGHT_ON);
    // }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, true, *WEAPON_SNAKE_NIKITA_INSTANCE_WORK_ID_FLAG_SHOOT);
    }
    frame(fighter.lua_state_agent, 83.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, ArticleOperationTarget(0));
    }
}
unsafe extern "C" fn snake_side_special_air_game(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, false, 0);
    }
    // frame(fighter.lua_state_agent, 10.0);
    // if macros::is_excute(fighter) {
    //     GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{x:0.0, y:3.0});
    // }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, true, *WEAPON_SNAKE_NIKITA_INSTANCE_WORK_ID_FLAG_SHOOT);
    }
    // frame(fighter.lua_state_agent, 70.0);
    // if macros::is_excute(fighter) {
    //     GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{x:0.0, y:0.0});
    // }
    frame(fighter.lua_state_agent, 83.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, ArticleOperationTarget(0));
    }
}
unsafe extern "C" fn snake_side_special_expr(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
}
unsafe extern "C" fn snake_side_special_air_expr(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    // frame(lua_state_agent, 25.0);
    // if macros::is_excute(fighter) {
    //     ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false);
    // }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
}
unsafe extern "C" fn snake_side_special_snd(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_s07"))
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_s01"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_s02"));
    }
    wait(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_s03"));
    }
    frame(fighter.lua_state_agent, 80.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_s07"))
    }
}
unsafe extern "C" fn snake_side_special_air_snd(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_s07"))
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_s01"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_s02"));
    }
    wait(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_s03"));
    }
    frame(fighter.lua_state_agent, 80.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_s07"))
    }
}
unsafe extern "C" fn snake_side_special_eff(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -3, 10, 0, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        // EFFECT(fighter, Hash40::new("sys_bananagun_shot"), Hash40::new("haver"), 3, 1, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_bananagun_shot"), Hash40::new("haver"), 0, 0.5, 3, 0, 0, 0, 0.4, true);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        // EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 4.5, 1, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 0, 1, 4.5, 0, 0, 0, 0.2, true);
    }
}
unsafe extern "C" fn snake_side_special_air_eff(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -3, 11, -2, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        // EFFECT(fighter, Hash40::new("sys_bananagun_shot"), Hash40::new("haver"), 3, 1, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_bananagun_shot"), Hash40::new("haver"), 0, 0.5, 3, 0, 0, 0, 0.4, true);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        // EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 4.5, 1, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 0, 1, 4.5, 0, 0, 0, 0.2, true);
    }
}
//tranq gun
unsafe extern "C" fn snake_tranq_gun_start_snd(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
}
unsafe extern "C" fn snake_tranq_gun_shoot_snd(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
}
//tranq dart
unsafe extern "C" fn snake_tranq_dart_fly_game(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep_ex"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        // ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
    }
}
unsafe extern "C" fn snake_tranq_dart_fly_snd(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
}
unsafe extern "C" fn snake_tranq_dart_fly_eff(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.3, true);
        // LAST_PARTICLE_SET_COLOR(fighter, 0.6, 0.6, 2.8);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 2.5, 2.5, 0.0);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.001);

        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 3, Hash40::new("top"), 0.0, 0.35, -1.5, Hash40::new("haver"), 0.0, -0.25, 1.45, true, Hash40::new("null"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    for _ in 0..5 {
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("snake_missile_smoke"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("snake_missile_smoke2"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("snake_missile_smoke3"), true, true);
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}
unsafe extern "C" fn snake_tranq_dart_fall_eff(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 3, Hash40::new("top"), 0.0, 0.35, -1.5, Hash40::new("haver"), 0.0, -0.25, 1.45, true, Hash40::new("null"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
}
unsafe extern "C" fn snake_tranq_dart_explode_game(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
}
unsafe extern "C" fn snake_tranq_dart_explode_snd(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
}
unsafe extern "C" fn snake_tranq_dart_explode_eff(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn snake_tranq_dart_land_eff(fighter : &mut L2CAgentBase) {
let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
    }
}