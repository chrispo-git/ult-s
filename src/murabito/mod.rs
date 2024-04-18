mod status;
mod frame;
mod acmd;
			
static mut TREE_POS_X : [f32; 8] = [0.0; 8];
static mut TREE_POS_Y : [f32; 8] = [0.0; 8];
static mut IS_FALLEN : [bool; 8] = [false; 8];
static mut DO_BOUNCE : [bool; 8] = [false; 8];
static mut CHANGE_FRAME : [bool; 8] = [false; 8];
static mut HAS_BEEN_AIR : [bool; 8] = [false; 8];
static mut Y_DIST : f32 = 10.0;
static mut X_DIST : f32 = 10.0;

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}