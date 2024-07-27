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
pub static mut DI_DIR: i32 = 0;
pub static mut LEDGE_OPTION: i32 = 0;
// 0 - Neutral Getup
// 1 - Ledge Attack
// 2 - Ledge Roll
// 3 - Ledge Jump
// 4 - Ledge Drop -> Double Jump
// 5 - Wait at ledge for 30 more frames
pub static mut LEDGE_OPTION_AFTER: i32 = 0;
// 0 - None
// 1 - Shield/Airdodge
// 2 - Aerial/Tilt (Have 2 lists of whether to ftilt/dtilt)
pub static mut LEDGE_DELAY : [i32; 8] = [0; 8];
pub static mut DJ_DELAY : [i32; 8] = [0; 8];
pub static mut DELAY_FRAMES: i32 = 30;
pub static mut DJ_DELAY_FRAMES: i32 = 21;


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