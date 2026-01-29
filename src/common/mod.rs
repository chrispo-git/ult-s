mod hitstun;
mod dacus;
mod landing;
mod wavedash;
mod jab;
mod movement;
mod bone;
mod projectile_invuln;
mod remove_quake;
mod cancel;
mod css;
mod gamemodes;

use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use crate::util::*;

pub const MAX_WEIGHT : i32 = 150;
pub const MIN_WEIGHT : i32 = 60;
pub const MAX_GRAVITY : f32 = 0.1;
pub const MIN_GRAVITY : f32 = 0.065;

unsafe extern "C" fn common(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        dacus::opff(fighter, status_kind, entry_id);
        cancel::opff(fighter, status_kind);
        hitstun::opff(fighter, status_kind);
        jab::opff(fighter, status_kind, motion_kind, entry_id);
        landing::opff(fighter, status_kind, motion_kind, entry_id);
        movement::opff(fighter, status_kind, motion_kind, entry_id);
        projectile_invuln::opff(fighter, motion_kind);
        wavedash::opff(fighter, status_kind, motion_kind, entry_id);
        gamemodes::opff(fighter, status_kind, motion_kind, entry_id);
    }
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, common)
	.install();

	css::install();
    hitstun::install();
    landing::install();
    wavedash::install();
    movement::install();
    bone::install();
	remove_quake::install();
}