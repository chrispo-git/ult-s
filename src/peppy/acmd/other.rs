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
	Agent::new("falco")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
    .acmd("effect_appealhil", peppy_utaunt_eff, Priority::Low)    
    .acmd("effect_appealhir", peppy_utaunt_eff, Priority::Low)    
    .acmd("sound_appealhil", peppy_utaunt_snd, Priority::Low)    
    .acmd("sound_appealhir", peppy_utaunt_snd, Priority::Low)    
    .install();
}
unsafe extern "C" fn peppy_utaunt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0); 
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_hit_rush"), Hash40::new("haver"), 0,0,0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn peppy_utaunt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0); 
    if macros::is_excute(agent) {
        let rand_num = smash::app::sv_math::rand(hash40("fighter"), 40);
        if rand_num == 37 {
            macros::PLAY_SE(agent, Hash40::new("vc_falco_appeal03"));
        } else {
            macros::PLAY_SE(agent, Hash40::new("vc_falco_win02"));
        }
    }
}