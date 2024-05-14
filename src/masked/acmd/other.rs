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
use crate::masked::*;
use super::*;
pub fn install() {
    Agent::new("lucas")
        .game_acmd("game_appealsrmaskedman", maskedman_ftaunt, Priority::Low) 
        .sound_acmd("sound_appealsrmaskedman", maskedman_ftaunt_snd, Priority::Low)    
        .sound_acmd("sound_appeallwrmaskedman", maskedman_dtaunt_snd, Priority::Low)    
        .expression_acmd("expression_turnmaskedman", maskedman_turn_expr, Priority::Low)    
        .install();
}

unsafe extern "C" fn maskedman_ftaunt(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn maskedman_ftaunt_snd(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn maskedman_dtaunt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_lucas_appeal02"));
    }
    wait(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucas_appeal02_02"));
    }
}

unsafe extern "C" fn maskedman_turn_expr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
