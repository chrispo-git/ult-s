mod attacks;
mod defense;
mod general;
mod movement;
mod special;


use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use crate::util::*;
use crate::config;

unsafe extern "C" fn config_apply(fighter : &mut L2CFighterCommon) {
    unsafe {
        
        let config = config::get();
        attacks::opff(fighter, &config);
        defense::opff(fighter, &config);
        general::opff(fighter, &config);
        movement::opff(fighter, &config);
        special::opff(fighter, &config);
    }
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, config_apply)
	.install();
}