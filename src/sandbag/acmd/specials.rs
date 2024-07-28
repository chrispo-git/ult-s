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
        .game_acmd("game_specialnsandbag", sandbag_neutralb, Priority::Low)
        .game_acmd("game_specialairnsandbag", sandbag_neutralb, Priority::Low)
        .game_acmd("effect_specialnsandbag", sandbag_neutralb_eff, Priority::Low)
        .sound_acmd("sound_specialnsandbag", sandbag_neutralb_snd, Priority::Low)
        .sound_acmd("sound_specialairnsandbag", sandbag_neutralb_snd, Priority::Low)
        .game_acmd("game_specialssandbag", sandbag_sideb, Priority::Low)
        .game_acmd("game_specialairssandbag", sandbag_sideb, Priority::Low)
        .effect_acmd("effect_specialssandbag", sandbag_sideb_eff, Priority::Low)
        .effect_acmd("effect_specialairssandbag", sandbag_sideb_eff, Priority::Low)
        .sound_acmd("sound_specialssandbag", sandbag_sideb_snd, Priority::Low)
        .sound_acmd("sound_specialairssandbag", sandbag_sideb_snd, Priority::Low)
        .game_acmd("game_specialairsfire", sandbag_sideb_fire, Priority::Low)
        .effect_acmd("effect_specialairsfire", sandbag_sideb_fire_eff, Priority::Low)
        .sound_acmd("sound_specialairsfire", sandbag_sideb_fire_snd, Priority::Low)
        .game_acmd("game_speciallwsandbag", sandbag_downb, Priority::Low)
        .game_acmd("game_specialairlwsandbag", sandbag_downb, Priority::Low)
        .effect_acmd("effect_speciallwsandbag", sandbag_downb_eff, Priority::Low)
        .effect_acmd("effect_specialairlwsandbag", sandbag_downb_eff, Priority::Low)
        .sound_acmd("sound_speciallwsandbag", sandbag_downb_snd, Priority::Low)
        .sound_acmd("sound_specialairlwsandbag", sandbag_downb_snd, Priority::Low)
        .game_acmd("game_specialhisandbag", sandbag_upb, Priority::Low)
        .effect_acmd("effect_specialhisandbag", sandbag_upb_eff, Priority::Low)
        .effect_acmd("effect_specialairhisandbag", sandbag_upb_air_eff, Priority::Low)
        .sound_acmd("sound_specialhisandbag", sandbag_upb_snd, Priority::Low)
        .install();

    Agent::new("mariod_drcapsule")
        .game_acmd("game_regularsandbag", disc_regular, Priority::Low)
        .install();
}


unsafe extern "C" fn disc_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 65, 60, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn sandbag_neutralb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, -1);
    }
}
unsafe extern "C" fn sandbag_neutralb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn sandbag_neutralb_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_03"));
    }
}

unsafe extern "C" fn sandbag_sideb(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn sandbag_sideb_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_sign"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.42);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_sign"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 0.4, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.42);
    }
}
unsafe extern "C" fn sandbag_sideb_snd(agent: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(agent.lua_state_agent, 3.0);
	if macros::is_excute(agent) {
        if !SIDEB_SOUND[ENTRY_ID] {
            macros::PLAY_SE(agent, Hash40::new("se_item_superscope_charge"));
            SIDEB_SOUND[ENTRY_ID] = true;
        }
    }
}

unsafe extern "C" fn sandbag_sideb_fire(agent: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, /*FSM*/ 0.8);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 10.0 + 8.0*(SIDEB_CHARGE[ENTRY_ID] as f32/SIDEB_MAX), 361, 60, 0, 70, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), if(SIDEB_CHARGE[ENTRY_ID]<40.0){*ATTACK_SOUND_LEVEL_M}else{*ATTACK_SOUND_LEVEL_L}, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
}
unsafe extern "C" fn sandbag_sideb_fire_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_sign"), false, true);
    }
    frame(agent.lua_state_agent, 5.0);
	if macros::is_excute(agent) {
		macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("top"), -2, 8, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
	}
    frame(agent.lua_state_agent, 9.0);
	if macros::is_excute(agent) {
		macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("top"), -2, 8, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
	}
    frame(agent.lua_state_agent, 13.0);
	if macros::is_excute(agent) {
		macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("top"), -2, 8, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
	}
    frame(agent.lua_state_agent, 17.0);
	if macros::is_excute(agent) {
		macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("top"), -2, 8, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
	}
    frame(agent.lua_state_agent, 21.0);
	if macros::is_excute(agent) {
		macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("top"), -2, 8, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
	}
    frame(agent.lua_state_agent, 25.0);
	if macros::is_excute(agent) {
		macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("top"), -2, 8, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
	}
}
unsafe extern "C" fn sandbag_sideb_fire_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_item_superscope_charge"));
        macros::PLAY_SE(agent, Hash40::new("se_item_superscope_chargeshot_l"));
    }
}

unsafe extern "C" fn sandbag_downb(agent: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    if IS_METAL[ENTRY_ID] || (!IS_METAL[ENTRY_ID] && DOWNB_COUNT[ENTRY_ID] >= 2) { //cant use metal box
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 55, 70, 0, 90, 14.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
            macros::FT_ADD_DAMAGE(agent, 6.5);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else if !IS_METAL[ENTRY_ID] && DOWNB_COUNT[ENTRY_ID] < 2 { //can use metal box
        frame(agent.lua_state_agent, 45.0);
        if macros::is_excute(agent) {
            DOWNB_COUNT[ENTRY_ID] += 1;
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_METALBLOCK), 0, 0, false, false);
        }
    }
}
unsafe extern "C" fn sandbag_downb_eff(agent: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    if IS_METAL[ENTRY_ID] || (!IS_METAL[ENTRY_ID] && DOWNB_COUNT[ENTRY_ID] >= 2) { //cant use metal box
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 8, -1, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        }
    }
    else if !IS_METAL[ENTRY_ID] && DOWNB_COUNT[ENTRY_ID] == 0 { //metal box 2
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_item_get"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_item_get"), Hash40::new("throw"), 0, 0, 12, 0, 0, 0, 1.0, false);
        }
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        frame(agent.lua_state_agent, 44.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("throw"), 0, -3, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 0.85);
            macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("throw"), 12, -3, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 0.85);
        }
    }
    else if !IS_METAL[ENTRY_ID] && DOWNB_COUNT[ENTRY_ID] == 1 { //metal box 1
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_item_get"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, false);
        }
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        frame(agent.lua_state_agent, 44.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("throw"), 0, -3, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 0.85);
        }
    }
}
unsafe extern "C" fn sandbag_downb_snd(agent: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    if IS_METAL[ENTRY_ID] || (!IS_METAL[ENTRY_ID] && DOWNB_COUNT[ENTRY_ID] >= 2) { //cant use metal box
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
        }
    }
    else if !IS_METAL[ENTRY_ID] && DOWNB_COUNT[ENTRY_ID] < 2 { //can use metal box
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_lifeup"));
        }
    }
}

unsafe extern "C" fn sandbag_upb(agent: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        //UPB_CAN_CANCEL[ENTRY_ID] = true;
        WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 7.8);
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        macros::ATTACK(agent, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("waist"), /*Damage*/ 12.0, /*Angle*/ 66, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_THROW);
        AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        //UPB_CAN_CANCEL[ENTRY_ID] = false;
        WorkModule::on_flag(agent.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        if DamageModule::damage(boma, 0) < 300.0 {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 10.0, 60, 80, 0, 50, 5.0, 0.0, 0.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        if DamageModule::damage(boma, 0) < 300.0 {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn sandbag_upb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("hipmar"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
        macros::EFFECT(agent, Hash40::new("mariod_superjump_power"), Hash40::new("hipmar"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_windwave"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mariod_superjump_power"), false, true);
        macros::EFFECT(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("hipmar"), 0, 0, 0, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
        macros::EFFECT(agent, Hash40::new("mariod_superjump_power"), Hash40::new("hipmar"), 0, 0, 0, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mariod_superjump_power"), false, true);
    }
}
unsafe extern "C" fn sandbag_upb_air_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("hipmar"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
        macros::EFFECT(agent, Hash40::new("mariod_superjump_power"), Hash40::new("hipmar"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mariod_superjump_power"), false, true);
        macros::EFFECT(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("hipmar"), 0, 0, 0, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
        macros::EFFECT(agent, Hash40::new("mariod_superjump_power"), Hash40::new("hipmar"), 0, 0, 0, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mariod_superjump_power"), false, true);
    }
}
unsafe extern "C" fn sandbag_upb_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_item_homerunbat_l"));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_homerunbat_l"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_sandbag_dmg_l"));
    }
}