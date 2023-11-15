
mod elight;
mod eflame;

static mut FAST_SWITCH : [bool; 8] = [false; 8];

pub fn install() {
	elight::install();
	eflame::install();
}