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
use crate::murabito::*;
use super::*;

pub fn install() {
    Agent::new("murabito")
    .acmd("game_specialairn", villy_neutralb_air, Priority::Low)    
    .acmd("effect_specialairn", villy_neutralb_air_eff, Priority::Low)    
    .acmd("game_specialn", villy_neutralb_1, Priority::Low)    
    .acmd("effect_specialn", villy_neutralb_1_eff, Priority::Low)    
    .acmd("sound_specialn", villy_neutralb_1_snd, Priority::Low)    
    .acmd("game_specialn2", villy_neutralb_2, Priority::Low)    
    .acmd("effect_specialn2", villy_neutralb_2_eff, Priority::Low)    
    .acmd("sound_specialn2", villy_neutralb_2_snd, Priority::Low)    
    .acmd("game_specialn2fail", villy_neutralb_2_fail, Priority::Low)    
    .acmd("effect_specialn2fail", villy_neutralb_2_fail_eff, Priority::Low)    
    .acmd("sound_specialn2fail", villy_neutralb_2_fail_snd, Priority::Low)    
    .acmd("game_specialn3", villy_neutralb_3, Priority::Low)    
    .acmd("effect_specialn3", villy_neutralb_3_eff, Priority::Low)    
    .acmd("sound_specialn3", villy_neutralb_3_snd, Priority::Low)    
    .install();
}

unsafe extern "C" fn villy_neutralb_air(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0 );
        if PostureModule::lr(agent.module_accessor) > 0.0 {
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovel"),true);
        } else {
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovelflip"),true);
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 60, 80, 0, 30, 4.2, 1.0, 1.5, 0.0, Some(1.0), Some(4.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0 );
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovel"),false);
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovelflip"),false);
    }
}	
unsafe extern "C" fn villy_neutralb_air_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true, 0.6);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}	
unsafe extern "C" fn villy_neutralb_1(agent: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let motion_kind = MotionModule::motion_kind(boma);
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let lr = PostureModule::lr(boma);
    let is_facing_tree = lr*(TREE_POS_X[ENTRY_ID]-pos_x) > 0.0;
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_TREE) && ![hash40("special_n3"), hash40("special_n2_fail")].contains(&motion_kind) {
        if (TREE_POS_X[ENTRY_ID]-pos_x).abs() < X_DIST && (TREE_POS_Y[ENTRY_ID]-pos_y).abs() < Y_DIST  && is_facing_tree && !IS_FALLEN[ENTRY_ID] {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_n3"), 0.0, 1.0, false, 0.0, false, false);
            println!("special_n3");
            CHANGE_FRAME[ENTRY_ID] = true;
        } else {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_n2_fail"), 0.0, 1.0, false, 0.0, false, false);
            println!("special_n2_fail");
            CHANGE_FRAME[ENTRY_ID] = true;
        }
    } else if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT) && ![hash40("special_n2"), hash40("special_n2_fail")].contains(&motion_kind) {
        if (TREE_POS_X[ENTRY_ID]-pos_x).abs() < X_DIST && (TREE_POS_Y[ENTRY_ID]-pos_y).abs() < Y_DIST {
            MotionModule::change_motion(boma, Hash40::new("special_n2"), 0.0, 1.0, false, 0.0, false, false);
            println!("special_n2");
            CHANGE_FRAME[ENTRY_ID] = true;
        } else {
            MotionModule::change_motion(boma, Hash40::new("special_n2_fail"), 0.0, 1.0, false, 0.0, false, false);
            println!("special_n2_fail");
            CHANGE_FRAME[ENTRY_ID] = true;
        }
    } else if motion_kind != hash40("special_n") {
        MotionModule::change_motion(boma, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        CHANGE_FRAME[ENTRY_ID] = true;
    }
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0 );
        if PostureModule::lr(agent.module_accessor) > 0.0 {
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovel"),true);
        } else {
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovelflip"),true);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("stickr"), 14.0, 80, 95, 0, 32, 4.5, 0.0, 0.0, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("stickr"), 14.0, 80, 95, 0, 32, 4.5, 0.0, 0.0, 10.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("armr"), 14.0, 80, 95, 0, 32, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0 );
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovel"),false);
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovelflip"),false);
    }
}		
unsafe extern "C" fn villy_neutralb_1_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 11, 0, 1.7, 10, 70, 1.5, true);
    }
}	
unsafe extern "C" fn villy_neutralb_1_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_murabito_swing_s_finish"));
    }
}		
unsafe extern "C" fn villy_neutralb_2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0 );
        if PostureModule::lr(agent.module_accessor) > 0.0 {
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovel"),true);
        } else {
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovelflip"),true);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("stickr"), 7.0, 361, 50, 0, 50, 2.0, 0.8, 0.0, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("stickr"), 7.0, 361, 50, 0, 50, 4.5, 0.8, 0.0, 10.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0 );
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovel"),false);
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovelflip"),false);
    }
}
unsafe extern "C" fn villy_neutralb_2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("murabito_soil2"), Hash40::new("top"), 12, 0, 0.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 0.55, 2, 2, 2, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("murabito_clay"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, false);
    }
}		
unsafe extern "C" fn villy_neutralb_2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_murabito_smash_l02"));
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_murabito_attackdash01"));
    }
}		
unsafe extern "C" fn villy_neutralb_2_fail(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0 );
        if PostureModule::lr(agent.module_accessor) > 0.0 {
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovel"),true);
        } else {
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovelflip"),true);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("stickr"), 7.0, 361, 50, 0, 50, 2.0, 0.8, 0.0, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("stickr"), 7.0, 361, 50, 0, 50, 4.5, 0.8, 0.0, 10.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0 );
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovel"),false);
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("murabito_shovelflip"),false);
    }
}	
unsafe extern "C" fn villy_neutralb_2_fail_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("murabito_soil2"), Hash40::new("top"), 12, 0, 0.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}	
unsafe extern "C" fn villy_neutralb_2_fail_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_murabito_smash_l02"));
    }
}	
unsafe extern "C" fn villy_neutralb_3(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.1);
    if macros::is_excute(agent) {
        ItemModule::remove_item(agent.module_accessor, 0);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0 );
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0 );
        let rand_num = smash::app::sv_math::rand(hash40("fighter"), 100);
        if rand_num < 2 {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_DEKU), 0, 0, false, false);
        } else if rand_num < 4 {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_DOSEISAN), 0, 0, false, false);
        } else if rand_num < 6 {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_MAGICBALL), 0, 0, false, false);
        } else if rand_num < 50 {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_BANANA), 0, 0, false, false);
        } else {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_WOOD), 0, 0, false, false);
        }
        macros::STOP_SE(agent, Hash40::new("se_item_item_get"));
    }
}		
unsafe extern "C" fn villy_neutralb_3_eff(agent: &mut L2CAgentBase) {
    for _ in 0..5 {
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
}	
unsafe extern "C" fn villy_neutralb_3_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_murabito_attackair_h01"));
        macros::PLAY_SE(agent, Hash40::new("se_murabito_swing_m"));
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_murabito_attackair_h02"));
        macros::PLAY_SE(agent, Hash40::new("se_murabito_swing_m"));
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_murabito_special_n04"));
    }
}	