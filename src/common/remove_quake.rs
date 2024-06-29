use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
unsafe extern "C" fn donkey(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
unsafe extern "C" fn koopa(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
unsafe extern "C" fn dedede(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
unsafe extern "C" fn krool(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
unsafe extern "C" fn ridley(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
unsafe extern "C" fn plizardon(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
		
pub fn install() {
    Agent::new("donkey")
    .acmd("expression_landingheavy", donkey, Priority::Low)    
    .install();

    Agent::new("koopa")
    .acmd("expression_landingheavy", koopa, Priority::Low)    
    .install();

    Agent::new("dedede")
    .acmd("expression_landingheavy", dedede, Priority::Low)    
    .install();

    Agent::new("krool")
    .acmd("expression_landingheavy", krool, Priority::Low)    
    .install();

    Agent::new("ridley")
    .acmd("expression_landingheavy", ridley, Priority::Low)    
    .install();

    Agent::new("plizardon")
    .acmd("expression_landingheavy", plizardon, Priority::Low)    
    .install();
}
