mod status;
mod frame;
mod acmd;
use smash::lib::lua_const::*;
use smash::hash40;
use crate::util::*;

static mut TICO_X : [f32; 8] = [0.0; 8];
static mut TICO_Y : [f32; 8] = [0.0; 8];
static mut ROSA_X : [f32; 8] = [0.0; 8];
static mut ROSA_Y : [f32; 8] = [0.0; 8];
static mut IS_TELEPORT : [bool; 8] = [false; 8];
static mut IS_TICO_DEAD : [bool; 8] = [false; 8];
static mut INVIS_FRAMES : [i32; 8] = [0; 8];
static mut MAX_INVIS_FRAMES : i32 = 20;
static mut COOLDOWN : [i32; 8] = [0; 8];
static mut TELEPORT_COOLDOWN : i32 = 300;
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_ROSETTA, get_marked_costumes("rosetta","rosetta"), (smash::hash40("param_special_lw"), smash::hash40("weapon_search_radius"), 0.01));
	param_config::update_float_2(*FIGHTER_KIND_ROSETTA, get_marked_costumes("rosetta","rosetta"), (smash::hash40("param_special_lw"), smash::hash40("item_search_radius"), 0.01));
	param_config::update_float_2(-*FIGHTER_KIND_ROSETTA, get_marked_costumes("rosetta","rosetta"), (smash::hash40("param_tico"), smash::hash40("hp"), 30.0));
	param_config::update_float_2(-*FIGHTER_KIND_ROSETTA, get_marked_costumes("rosetta","rosetta"), (smash::hash40("param_tico"), smash::hash40("follow_damage_air_brake"), 0.075));
	param_config::update_float_2(-*FIGHTER_KIND_ROSETTA, get_marked_costumes("rosetta","rosetta"), (smash::hash40("param_tico"), smash::hash40("free_damage_air_brake"), 0.07));
	param_config::update_float_2(*FIGHTER_KIND_ROSETTA, get_marked_costumes("rosetta","rosetta"), (smash::hash40("dash_speed"), 0, 1.8));
	param_config::update_float_2(*FIGHTER_KIND_ROSETTA, get_marked_costumes("rosetta","rosetta"), (smash::hash40("run_speed_max"), 0, 1.7));
	param_config::update_float_2(*FIGHTER_KIND_ROSETTA, get_marked_costumes("rosetta","rosetta"), (smash::hash40("jump_speed_x_mul"), 0, 1.1));
}