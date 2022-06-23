use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
#[acmd_script(
    agent = "donkey",
    script =  "expression_landingheavy",
    category = ACMD_EXPRESSION)]
unsafe fn donkey(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "koopa",
    script =  "expression_landingheavy",
    category = ACMD_EXPRESSION)]
unsafe fn koopa(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "dedede",
    script =  "expression_landingheavy",
    category = ACMD_EXPRESSION)]
unsafe fn dedede(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "krool",
    script =  "expression_landingheavy",
    category = ACMD_EXPRESSION)]
unsafe fn krool(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "ridley",
    script =  "expression_landingheavy",
    category = ACMD_EXPRESSION)]
unsafe fn ridley(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "plizardon",
    script =  "expression_landingheavy",
    category = ACMD_EXPRESSION)]
unsafe fn plizardon(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
		
pub fn install() {
    smashline::install_acmd_scripts!(
		donkey, koopa, krool, dedede, ridley, plizardon
    );
}
