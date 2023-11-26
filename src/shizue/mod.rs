mod status;
mod frame;
mod acmd;
			
static mut ISA_RESHOOT_TIME: [i32; 8] = [0; 8];
static mut ISA_SHOT_KIND: [i32; 8] = [1; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}