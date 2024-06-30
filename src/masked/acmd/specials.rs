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
use crate::masked::*;
use crate::util::*;
use super::*;
pub fn install() {
    Agent::new("lucas")
        .game_acmd("game_specialhistartmaskedman", game_specialhistart, Priority::Low)    
        .game_acmd("game_specialairhistartmaskedman", game_specialhistart, Priority::Low)    
        .effect_acmd("effect_specialhistartmaskedman", effect_specialhistart, Priority::Low)    
        .effect_acmd("effect_specialairhistartmaskedman", effect_specialhistart, Priority::Low)    
        .sound_acmd("sound_specialhistartmaskedman", sound_specialhistart, Priority::Low)    
        .sound_acmd("sound_specialairhistartmaskedman", sound_specialhistart, Priority::Low)    
        .expression_acmd("expression_specialhistartmaskedman", expression_specialhistart, Priority::Low)    
        .expression_acmd("expression_specialairhistartmaskedman", expression_specialhistart, Priority::Low)    
        .game_acmd("game_specialhiholdmaskedman", game_specialhihold, Priority::Low)    
        .effect_acmd("effect_specialhiholdmaskedman", effect_specialhihold, Priority::Low)    
        .sound_acmd("sound_specialhiholdmaskedman", sound_specialhihold, Priority::Low)    
        .expression_acmd("expression_specialhiholdmaskedman", expression_specialhihold, Priority::Low)    
        .effect_acmd("effect_specialhiendmaskedman", effect_specialhiend, Priority::Low)    
        .effect_acmd("effect_fallspecialmaskedman", effect_specialhiend, Priority::Low)    
        .effect_acmd("effect_speciallwstartmaskedman", effect_downb, Priority::Low)    
        .game_acmd("game_specialairsmaskedman", maskedman_sideb, Priority::Low)    
        .effect_acmd("effect_specialairsmaskedman", maskedman_sideb_eff, Priority::Low)    
        .sound_acmd("sound_specialairsmaskedman", maskedman_sideb_snd, Priority::Low)    
        .game_acmd("game_specialsmaskedman", maskedman_sideb, Priority::Low)    
        .effect_acmd("effect_specialsmaskedman", maskedman_sideb_eff, Priority::Low)    
        .sound_acmd("sound_specialsmaskedman", maskedman_sideb_snd, Priority::Low)    
        .game_acmd("game_specialndash", maskedman_neutralb_dash, Priority::Low)    
        .effect_acmd("effect_specialndash", maskedman_neutralb_dash_eff, Priority::Low)    
        .sound_acmd("sound_specialndash", maskedman_neutralb_dash_snd, Priority::Low)    
        .install();

    Agent::new("lucas_pkfire")
        .game_acmd("game_regularlight", lightningsword_regular, Priority::Low)    
        .effect_acmd("effect_regularlight", lightningsword_regular_eff, Priority::Low)    
        .sound_acmd("sound_regularlight", lightningsword_regular_snd, Priority::Low)    
        .install();
}


unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_AIR {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 76, 53, 0, 80, 6.0, 0.0, 6.0, 6.0, Some(0.0), Some(11.25), Some(6.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 76, 53, 0, 80, 6.0, 0.0, 18.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 76, 53, 0, 80, 6.0, 0.0, 6.0, -2.5, Some(0.0), Some(10.25), Some(-2.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 76, 53, 0, 80, 6.0, 0.0, 6.0, 6.0, Some(0.0), Some(11.25), Some(6.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 76, 53, 0, 80, 6.0, 0.0, 18.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 76, 53, 0, 80, 6.0, 0.0, 6.0, -2.5, Some(0.0), Some(10.25), Some(-2.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        }
    }
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_AIR {
            let bomb_fx = if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {Hash40::new("sys_bomb_a")} else {Hash40::new("sys_bomb_b")};
            macros::EFFECT(agent, bomb_fx, Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            let bomb_fx = if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {Hash40::new("sys_bomb_a")} else {Hash40::new("sys_bomb_b")};
            macros::EFFECT(agent, bomb_fx, Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}
unsafe extern "C" fn sound_specialhistart(agent: &mut L2CAgentBase) {

}
unsafe extern "C" fn expression_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

unsafe extern "C" fn game_specialhihold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 76, 50, 0, 45, 6.0, 0.0, 18.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATK_POWER(agent, 0, 12);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATK_POWER(agent, 0, 9);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialhihold(agent: &mut L2CAgentBase) {
    for i in 1..i32::MAX {
        if macros::is_excute(agent) {
            //sys_steam
            macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}
unsafe extern "C" fn sound_specialhihold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_ll"));
    }
}
unsafe extern "C" fn expression_specialhihold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_80_special_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_80_attackl"), 0);
    }
}

unsafe extern "C" fn effect_specialhiend(agent: &mut L2CAgentBase) {
	let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);    
    if is_added(boma) {
        for i in 1..i32::MAX {
            if macros::is_excute(agent) {
                //sys_steam
                macros::EFFECT(agent, Hash40::new("sys_steam"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
            }
            wait(agent.lua_state_agent, 5.0);
        }
    }
}

unsafe extern "C" fn effect_downb(agent: &mut L2CAgentBase) {
    let team_color = smash::app::FighterUtil::get_team_color(agent.module_accessor);
    let color = smash::app::FighterUtil::get_effect_team_color(
        smash::app::EColorKind(team_color as i32),
        Hash40::new("direction_effect_color")
    );
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_direction"), Hash40::new("throw"), 0, -5, 0, 0, 90, 0, 1, true);
        EffectModule::set_rgb_partial_last(agent.module_accessor, color.x, color.y, color.z);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_direction"), Hash40::new("throw"), 0, 5, 0, 180, 90, 0, 1, true);
        EffectModule::set_rgb_partial_last(agent.module_accessor, color.x, color.y, color.z);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_direction"), Hash40::new("throw"), 0, 0, -5, 90, 0, 90, 1, true);
        EffectModule::set_rgb_partial_last(agent.module_accessor, color.x, color.y, color.z);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_direction"), Hash40::new("throw"), 0, 0, 5, -90, 0, 90, 1, true);
        EffectModule::set_rgb_partial_last(agent.module_accessor, color.x, color.y, color.z);
    }
}

unsafe extern "C" fn maskedman_sideb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        //KineticModule::add_speed(agent.module_accessor, &Vector3f{x: -0.9, y: 0.1, z: 0.0});
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FIRE, false, -1);
    }
}
unsafe extern "C" fn maskedman_sideb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn maskedman_sideb_snd(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn lightningsword_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 35, 0, 55, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -1.9, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}
unsafe extern "C" fn lightningsword_regular_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_vector"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_vector"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_vector"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_vector"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 0.5, 0, 0, 0, 0, 0.3, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, -0.5, 0, 0, 0, 0, 0.4, true);
    }
    wait(agent.lua_state_agent, 1.0);
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 0.5, 0, 0, 0, 0, 0.3, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, -0.5, 0, 0, 0, 0, 0.4, true);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}
unsafe extern "C" fn lightningsword_regular_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucas_special_s02"));
    }
}

unsafe extern "C" fn maskedman_neutralb_dash(agent: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.5 + (10.0/60.0)*(NEUTRALB_CHARGE[ENTRY_ID] as f32), 361, 20+(NEUTRALB_CHARGE[ENTRY_ID]), 0, 20+((NEUTRALB_CHARGE[ENTRY_ID]/2) as i32), 4.5, 0.0, 6.0, 15.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0 + (10.0/60.0)*(NEUTRALB_CHARGE[ENTRY_ID] as f32), 361, 20+(NEUTRALB_CHARGE[ENTRY_ID]), 0, 20+((NEUTRALB_CHARGE[ENTRY_ID]/2) as i32), 8.0, 0.0, 6.0, 4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn maskedman_neutralb_dash_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_assist_steam_max"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_assist_steam_max"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 2.0, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::COL_NORMAL(agent);
        macros::FLASH(agent, 2.5, 2.5, 0.0, 0.25);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ColorBlendModule::off_flash(agent.module_accessor, false);
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_assist_steam_max"), false, false);
    }
}
unsafe extern "C" fn maskedman_neutralb_dash_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_assist_vanish"));
    }
}