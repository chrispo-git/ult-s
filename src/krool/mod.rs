mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_KROOL, get_marked_costumes("krool","krool").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_frame"), 0, 4.0));
}
