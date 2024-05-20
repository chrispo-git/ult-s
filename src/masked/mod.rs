mod status;
mod frame;
mod acmd;

pub const MAX_HITS : i32 = 6;
pub const DISABLE_SPECIAL_N: i32 = 0x0100;
pub const USE_SPECIAL_N_CALLBACK: i32 = 0x38;
pub const USE_SPECIAL_S_CALLBACK: i32 = 0x39;
pub const USE_SPECIAL_HI_CALLBACK: i32 = 0x3A;
pub const USE_SPECIAL_LW_CALLBACK: i32 = 0x3B;
pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C;
pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D;
pub const STATUS_CHANGE_CALLBACK: i32 = 0x3E;
pub const DAMAGE_MOTION_KIND_CALLBACK: i32 = 0x42;
pub const DASH_POST_TRANSITION_CALLBACK: i32 = 0x57;
pub static mut HAS_NEUTRALB : [bool; 8] = [true; 8];
pub static mut NEUTRALB_CHARGE : [i32; 8] = [0; 8];
pub static mut TIMER_TO_DOWNB : [i32; 8] = [0; 8];
pub static mut DOWNB_COOLDOWN : [i32; 8] = [0; 8];



pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}