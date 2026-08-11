mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_DOWNB_JUMP : i32 = 0;
static FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_UPB_ANGLE : i32 = 1;
//0 - Inwards
//1 - Middle
//2 - Outwards
static FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_IS_FINAL : i32 = 2;
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 5.0, z: 0.0 };
static FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_variance : i32 = 3;
static mut N1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 3.0, z: -15.0 };
static mut N2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.0, z: -24.0 };
static mut F3 : [u32; 8] = [0; 8];
static mut F4 : [u32; 8] = [0; 8];			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_int_2(-*WEAPON_KIND_KIRBY_FINALCUTTERSHOT, get_marked_costumes("kirby","kirby").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_finalcuttershot"), smash::hash40("is_penetration"), 1));
	param_config::update_float_2(-*WEAPON_KIND_KIRBY_FINALCUTTERSHOT, get_marked_costumes("kirby","kirby").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_finalcuttershot"), smash::hash40("life"), 40.0));
	param_config::update_float_2(-*WEAPON_KIND_KIRBY_FINALCUTTERSHOT, get_marked_costumes("kirby","kirby").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_finalcuttershot"), smash::hash40("speed"), 3.4));
	param_config::update_float_2(-*WEAPON_KIND_KIRBY_FINALCUTTERSHOT, get_marked_costumes("kirby","kirby").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_finalcuttershot"), smash::hash40("brake"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_KIRBY, get_marked_costumes("kirby","kirby").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.775));
	param_config::update_float_2(*FIGHTER_KIND_KIRBY, get_marked_costumes("kirby","kirby").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_KIRBY, get_marked_costumes("kirby","kirby").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 0.95));

}
