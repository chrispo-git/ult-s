mod status;
mod frame;
mod acmd;

static mut FLOAT : [i32; 8] = [0; 8];
static mut X : [f32; 8] = [0.0; 8];
static mut Y : [f32; 8] = [0.0; 8];
static mut FLOAT_MAX : i32 = 90;
static mut X_MAX : f32 = 1.155;
static mut X_ACCEL_ADD : f32 = 0.06;
static mut X_ACCEL_MUL : f32 = 0.12;
static mut Y_MAX : f32 = 0.4;
static mut Y_ACCEL_ADD : f32 = 0.03;
static mut Y_ACCEL_MUL : f32 = 0.03;



pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
