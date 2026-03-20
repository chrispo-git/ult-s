mod status;
mod frame;
mod acmd;
use crate::util::*;

static mut SIDEB : [bool; 8] = [false; 8];
static mut SPIN : [bool; 8] = [false; 8];
static mut SPIN_EFF : [i32; 8] = [0; 8];
static mut RUNLOOPCOUNT : [i32; 8] = [0; 8];
static mut SPIN1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 0.0 };
static mut SPIN2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.2, z: 0.0 };
static mut SPIN3 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.15, z: 0.0 };
static mut SPIN4 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.3, z: 0.0 };
static mut SPIN5 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.1, z: 0.0 };
static mut STAR1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 0.0 };
static mut STAR2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 9.0 };
static mut STAR3 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: -9.0 };
static mut STAR4 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 4.5 };
static mut STAR5 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: -4.5 };
static mut NOSPIN :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_MARIO, get_marked_costumes("mario","mario"), (smash::hash40("jump_speed_x_mul"), 0, 1.2));
}
