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
use crate::wario::*;
use super::*;

pub fn install() {
    Agent::new("wario")
    .acmd("game_specialssearch", wario_sideb, Priority::Low)    
    .acmd("game_specialairssearch", wario_sideb, Priority::Low)    
    .acmd("effect_specialssearch", wario_sideb_eff, Priority::Low)    
    .acmd("effect_specialairssearch", wario_sideb_eff, Priority::Low)    
    .acmd("sound_specialssearch", wario_sideb_snd, Priority::Low)    
    .acmd("sound_specialairssearch", wario_sideb_snd, Priority::Low)    
    .acmd("sound_specialsdrive", wario_sideb_start_snd, Priority::Low)    
    .acmd("sound_specialsride", wario_sideb_start_snd, Priority::Low)    
    .acmd("sound_specialairs", wario_sideb_start_snd, Priority::Low)    
    .acmd("sound_specialairs", wario_sideb_start_snd, Priority::Low)    
    .acmd("sound_specials", wario_sideb_start_snd, Priority::Low)    
    .acmd("sound_specialsstart", wario_sideb_start_snd, Priority::Low)    
    .acmd("sound_specialairsstart", wario_sideb_start_snd, Priority::Low)    
    .install();
	
    Agent::new("wario_coin")
    .acmd("game_shoot", game_shoot, Priority::Low)
    .acmd("effect_shoot", effect_shoot, Priority::Low)
    .install();
}

unsafe extern "C" fn wario_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			if COIN_COUNT[ENTRY_ID] > 0 {
				COIN_COUNT[ENTRY_ID] -= 1;
                macros::PLAY_SE(fighter, Hash40::new("se_common_coin"));
				ArticleModule::generate_article(fighter.module_accessor, FIGHTER_WARIO_GENERATE_ARTICLE_COIN, false, -1);
			}
		}
}	
unsafe extern "C" fn wario_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}	
unsafe extern "C" fn wario_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_s"));
        macros::PLAY_SE(fighter, Hash40::new("vc_wario_attack04"));
    }
}	
unsafe extern "C" fn wario_sideb_start_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}	

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 48, 52, 0, 78, 3.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_punch"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
	}
}

unsafe extern "C" fn effect_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
}