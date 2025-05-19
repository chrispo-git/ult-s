use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
unsafe extern "C" fn no_quake(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}			
		
pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("expression_landingheavy", no_quake, Priority::Low)    
    .install();

    Agent::new("koopa")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("expression_landingheavy", no_quake, Priority::Low)    
    .install();

    Agent::new("dedede")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("expression_landingheavy", no_quake, Priority::Low)    
    .install();

    Agent::new("krool")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("expression_landingheavy", no_quake, Priority::Low)    
    .install();

    Agent::new("ridley")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("expression_landingheavy", no_quake, Priority::Low)    
    .install();

    Agent::new("plizardon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("expression_landingheavy", no_quake, Priority::Low)    
    .install();
}
