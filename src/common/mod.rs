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



pub fn install() {
	css::install();
	gamemodes::install();
    hitstun::install();
    dacus::install();
    landing::install();
    wavedash::install();
    jab::install();
    movement::install();
    bone::install();
	projectile_invuln::install();
	remove_quake::install();
	cancel::install();
}