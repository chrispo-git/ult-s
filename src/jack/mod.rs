mod status;
mod frame;
mod acmd;

//Joker Gun Cancel Constants 
pub const NONE : i32 = 100;
pub const ATTACK_N : i32 = 101;
pub const ATTACK_S3 : i32 = 104;
pub const ATTACK_HI3 : i32 = 105;
pub const ATTACK_LW3 : i32 = 106;
pub const ATTACK_S4 : i32 = 107;
pub const ATTACK_HI4 : i32 = 108;
pub const ATTACK_LW4 : i32 = 110;
pub const ATTACK_AIR_N : i32 = 111;
pub const ATTACK_AIR_F : i32 = 112;
pub const ATTACK_AIR_B : i32 = 113;
pub const ATTACK_AIR_HI : i32 = 114;
pub const ATTACK_AIR_LW : i32 = 115;

//Joker Gun Cancel 
static mut GUN_C: [i32; 8] = [100; 8];
static mut IS_ARSENE: [bool; 8] = [false; 8];
static mut X: [f32; 8] = [0.0; 8];
static mut Y: [f32; 8] = [0.0; 8];
static mut BATON_TYPE: [i32; 8] = [0; 8];
static BATON_MAX : i32 = 2;


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}