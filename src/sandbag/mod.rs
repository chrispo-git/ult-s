mod status;
mod frame;
mod acmd;

pub static mut SPEED_MUL: [f32; 8] = [0.0; 8];
pub static mut UPB_CAN_CANCEL: [bool; 8] = [false; 8];
pub static mut BAN_UPB_TECH: [bool; 8] = [false; 8];
pub static mut DOWNB_COUNT: [i32; 8] = [0; 8];
pub static mut IS_METAL: [bool; 8] = [false; 8];
pub static mut SIDEB_SOUND: [bool; 8] = [false; 8];
pub static mut SIDEB_PRESS: [bool; 8] = [false; 8];
pub static mut SIDEB_CHARGE: [f32; 8] = [0.0; 8];
pub static mut SIDEB_COUNT: [i32; 8] = [0; 8];
pub static mut SIDEB_FIRE_COUNT: [i32; 8] = [0; 8];
pub static mut SIDEB_HIT: [bool; 8] = [false; 8];
pub static mut NEW_SPEED_MUT: [f32; 8] = [0.0; 8];
pub static mut STICK_DIRECTION : [f32; 8] = [0.0; 8];
pub static mut SIDEB_MAX : f32 = 90.0; 
pub static mut DIR_MULT : f32 = 57.295776842880464966688235343549; //Very fun number that turns direction that spits out ControlModule::get_stick_dir(boma) as an angle in degrees

			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
