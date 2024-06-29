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
    Agent::new("diddy")
    .acmd("game_appealsr", diddy_sidetaunt, Priority::Low)    
    .acmd("game_appealsl", diddy_sidetaunt, Priority::Low)    
    .acmd("effect_appealsr", diddy_sidetaunt_eff, Priority::Low)    
    .acmd("effect_appealsl", diddy_sidetaunt_eff, Priority::Low)    
    .acmd("sound_appealsr", diddy_sidetaunt_snd, Priority::Low)    
    .acmd("sound_appealsl", diddy_sidetaunt_snd, Priority::Low)    
    .install();
}

unsafe extern "C" fn diddy_sidetaunt(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, 0);
		}
}
unsafe extern "C" fn diddy_sidetaunt_eff(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("havel"), 0, 3, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
		}
}
unsafe extern "C" fn diddy_sidetaunt_snd(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_diddy_001"));
		}
}
	