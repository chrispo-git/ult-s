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
    .acmd("effect_appeallwl", peppy_dtaunt_eff, Priority::Low)    
    .acmd("effect_appeallwr", peppy_dtaunt_eff, Priority::Low)    
    .acmd("sound_appeallwl", peppy_dtaunt_snd, Priority::Low)    
    .acmd("sound_appeallwr", peppy_dtaunt_snd, Priority::Low)    
    .acmd("effect_appealsl", peppy_staunt_eff, Priority::Low)    
    .acmd("effect_appealsr", peppy_staunt_eff, Priority::Low)    
    .acmd("sound_appealsl", peppy_staunt_snd, Priority::Low)    
    .acmd("sound_appealsr", peppy_staunt_snd, Priority::Low)    
    .acmd("effect_win1", peppy_win1_eff, Priority::Low)    
    .acmd("effect_win2", peppy_win2_eff, Priority::Low)    
    .acmd("effect_win3", peppy_win3_eff, Priority::Low)    
    .acmd("sound_win1", peppy_win1_sound, Priority::Low)    
    .acmd("sound_win2", peppy_win2_sound, Priority::Low)    
    .acmd("sound_win2_default", peppy_win2_sound, Priority::Low)    
    .acmd("sound_win3", peppy_win3_sound, Priority::Low)    
    .install();
}
unsafe extern "C" fn peppy_utaunt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0); 
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_hit_rush"), Hash40::new("havel"), 0,0,0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn peppy_utaunt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0); 
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_falco_appeal03"));
    }
}
unsafe extern "C" fn peppy_dtaunt_eff(agent: &mut L2CAgentBase) {
}
unsafe extern "C" fn peppy_dtaunt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 40.0); 
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_landing_soil"));
    }
    frame(agent.lua_state_agent, 340.0); 
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_landing_soil"));
    }
}
unsafe extern "C" fn peppy_staunt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 25.0); 
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_falco_appeal02"));
    }
}
unsafe extern "C" fn peppy_staunt_eff(agent: &mut L2CAgentBase) {
}
unsafe extern "C" fn peppy_win1_eff(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        wait(agent.lua_state_agent, 2.0); 
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_fireflower_shot"), Hash40::new("haver"), -0.8, 11.0, 0.8, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
}
unsafe extern "C" fn peppy_win1_sound(agent: &mut L2CAgentBase) {
}
unsafe extern "C" fn peppy_win2_eff(agent: &mut L2CAgentBase) {
}
unsafe extern "C" fn peppy_win2_sound(agent: &mut L2CAgentBase) {
}
unsafe extern "C" fn peppy_win3_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 128.0); 
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn peppy_win3_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_falco_win02"));
    }
}

