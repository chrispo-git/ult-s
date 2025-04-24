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
    Agent::new("fox")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_throwlw", fox_dthrow, Priority::Low)    
    .install();
}
unsafe extern "C" fn fox_dthrow(agent: &mut L2CAgentBase) {
        if macros::is_excute(agent) {
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 1.0, 90, 100, 104, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            AttackModule::set_add_reaction_frame(agent.module_accessor, /*ID*/ 0, /*Frames*/ -30.0, /*Unk*/ false);
        }
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
            if macros::is_excute(agent) {
                ArticleModule::change_motion(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
            }
        };
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        }
        frame(agent.lua_state_agent, 29.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        }
        frame(agent.lua_state_agent, 32.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::CHECK_FINISH_CAMERA(agent, 1, 1);
        }
		frame(agent.lua_state_agent, 34.0);
		if macros::is_excute(agent) {
            set_knockdown_throw(agent);
			macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(agent.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
			AttackModule::clear_all(agent.module_accessor);
		}
		frame(agent.lua_state_agent, 35.0);
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
            if macros::is_excute(agent) {
                ArticleModule::change_motion(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
            }
        };
}