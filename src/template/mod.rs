use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use crate::util::*;
use smash::app::sv_animcmd::*;

pub fn install() {
	smashline::install_acmd_scripts!(
        
    );
}
