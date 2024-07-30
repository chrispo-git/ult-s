mod status;
mod frame;
mod acmd;


static mut HIT0: [f32; 8] = [0.0; 8];
static mut HIT1: [f32; 8] = [0.0; 8];
static mut HIT2: [f32; 8] = [0.0; 8];
static mut HIT3: [f32; 8] = [0.0; 8];
static mut HIT4: [f32; 8] = [0.0; 8];
static mut HIT5: [f32; 8] = [0.0; 8];
static mut HIT6: [f32; 8] = [0.0; 8];
static mut HIT0S: [f32; 8] = [0.0; 8];
static mut HIT1S: [f32; 8] = [0.0; 8];
static mut HIT2S: [f32; 8] = [0.0; 8];
static mut HIT3S: [f32; 8] = [0.0; 8];
static mut HIT4S: [f32; 8] = [0.0; 8];
static mut HIT5S: [f32; 8] = [0.0; 8];
static mut HIT6S: [f32; 8] = [0.0; 8];
static mut HIT_Y: [f32; 8] = [0.0; 8];
static mut ATK_HEIGHT: [i32; 8] = [0; 8]; //0 Mid, 1 High, 2 Low, 3 Wide
static mut HIGH_ADD: f32 = 10.0;
static mut LOW_ADD: f32 = -10.0;

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}