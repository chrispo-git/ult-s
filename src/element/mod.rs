
mod elight;
mod eflame;

static FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_FAST_SWITCH : i32 = 0;

pub fn install() {
	elight::install();
	eflame::install();
}