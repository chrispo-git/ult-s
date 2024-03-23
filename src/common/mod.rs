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
mod command;
mod cancel;
mod training;

pub static mut IS_GLOW: bool = false;

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
	command::install();
	cancel::install();
    training::install();
}