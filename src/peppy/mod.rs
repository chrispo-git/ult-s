mod status;
mod frame;
mod acmd;
pub static mut FIGHTER_FALCO_GENERATE_ARTICLE_MISSILE: i32 = 6;


static mut AIR_SHOT : [bool; 8] = [false; 8];
static mut HAS_DOWNB : [bool; 8] = [false; 8];
static mut DO_STALL : [bool; 8] = [false; 8];
static mut SUPER_LAUNCH : [bool; 8] = [false; 8];
static mut TETHER_EFFECTS:Vec<u32> = Vec::new();

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}