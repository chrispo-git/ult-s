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

use std::time::Instant;

static mut FRAME_COUNT : i32 = 0;
static mut OPERATION_COUNT : f32 = 0.0;
static mut MAX_SPIKE : f32 = 0.0;

unsafe extern "C" fn config_apply(fighter : &mut L2CFighterCommon) {
    unsafe {

        let config = config::get();
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        attacks::opff(fighter, &config, status_kind, entry_id);
        defense::opff(fighter, &config, status_kind, entry_id);
        general::opff(fighter, &config, status_kind, entry_id);
        movement::opff(fighter, &config, status_kind);
        special::opff(fighter, &config, status_kind, entry_id);
    }
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, config_apply)
	.install();
}