mod hitstun;
mod dacus;
mod landing;
mod wavedash;
mod jab;
mod movement;
mod bone;
mod projectile_invuln;
mod remove_quake;
mod melee;
mod faf_change;
mod cancel;
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
    hitstun::install();
    dacus::install();
    landing::install();
    wavedash::install();
    jab::install();
    movement::install();
    bone::install();
    melee::install();
	projectile_invuln::install();
	remove_quake::install();
	faf_change::install();
	cancel::install();

    //Setting values for everybody!
    let all: Vec<i32> = vec![-1];
    param_config::update_attribute_mul_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("damage_fly_top_air_accel_y"), 0, 1.05));
    param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("damage_fly_top_speed_y_stable"), 0, 1.84));
}