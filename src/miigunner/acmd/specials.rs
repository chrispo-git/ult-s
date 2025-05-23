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
use crate::miigunner::*;
use super::*;

pub fn install() {
    Agent::new("miigunner")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialairs1", gunner_s1, Priority::Low)    
    .acmd("game_specials1", gunner_s1, Priority::Low)    
    .acmd("game_specials31", gunner_s3, Priority::Low)    
    .acmd("game_specialairs31", gunner_air_s3, Priority::Low)    
    .acmd("game_specials32", gunner_s32, Priority::Low)    
    .acmd("game_specialairs32", gunner_air_s32, Priority::Low)    
    .acmd("game_speciallw1start", gunner_shine, Priority::Low)    
    .acmd("game_specialairlw1start", gunner_shine, Priority::Low)    
    .acmd("game_speciallw3hold", gunner_mag, Priority::Low)    
    .acmd("game_specialairlw3hold", gunner_mag, Priority::Low)   
    .install();

    Agent::new("miigunner_gunnercharge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_shoot", gunner_chargeshot, Priority::Low)    
    .install();

    Agent::new("miigunner_flamepillar")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_pillar", gunner_fp, Priority::Low)    
    .install();

    Agent::new("miigunner_groundbomb")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_burstattack", gunner_bomb, Priority::Low)    
    .install();
}

unsafe extern "C" fn gunner_chargeshot(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 361, 42, 0, 14, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 27.0, 45, 70, 0, 30, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
        attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}

unsafe extern "C" fn gunner_laser(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 100, 20, 0, 1.4, 0.0, 0.0, 0.8, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 100, 20, 0, 1.4, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(9.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
    }
    wait(fighter.lua_state_agent, 4.0);
}	


unsafe extern "C" fn gunner_s1(agent: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        if NO_FP[ENTRY_ID] <= 0 {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FLAMEPILLAR, false, -1);
            NO_FP[ENTRY_ID] = FP_DELAY;
        }
    }
}
unsafe extern "C" fn gunner_fp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 110, 100, 15, 0, 6.0, 0.0, 2.0, 0.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.1, 0.0, 10, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 110, 100, 15, 0, 4.2, 0.0, 3.0, -1.5, Some(0.0), Some(3.0), Some(1.5), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.1, 0.0, 10, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 110, 100, 15, 0, 4.5, 0.0, 9.0, 0.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.1, 0.0, 10, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
    wait(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 70, 86, 0, 54, 6.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 70, 86, 0, 54, 4.2, 0.0, 3.0, -1.5, Some(0.0), Some(3.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 70, 86, 0, 54, 4.5, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn gunner_s3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.304347826086957);
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON);
    }
}

unsafe extern "C" fn gunner_air_s3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.304347826086957);
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON);
    }
}

unsafe extern "C" fn gunner_s32(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.08695652173913);
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON);
    }
}

unsafe extern "C" fn gunner_air_s32(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.08695652173913);
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON);
    }
}

unsafe extern "C" fn gunner_shine(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 10, 40, 0, 60, 8.0, 0.0, 6.5, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 24, 40, 0, 60, 8.0, 0.0, 6.5, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 3.0, /*Unk*/ false);
    }
}

unsafe extern "C" fn gunner_bomb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 80, 70, 0, 60, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::AREA_WIND_2ND_RAD(fighter, 0, 1, 0.02, 1000, 1, 0, 0, 25);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}		
unsafe extern "C" fn gunner_mag(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 65, /*FKB*/ 40, /*BKB*/ 35, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 10, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_ENERGY);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 4.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	