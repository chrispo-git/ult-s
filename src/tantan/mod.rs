mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;




pub fn install() {
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("punch_length"), 3.3));
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("shift_punch_length"), 3.3));
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("long_punch_length"), 3.6));
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("long_shift_punch_length"), 3.6));
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("punch_length"), 2.6));
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("shift_punch_length"), 2.6));
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("long_punch_length"), 3.1));
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("long_shift_punch_length"), 3.1));
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("punch_length"), 2.75));
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("shift_punch_length"), 2.75));
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("long_punch_length"), 3.4));
    param_config::update_float_2(-*WEAPON_KIND_TANTAN_SPIRALLEFT, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_spiral"), smash::hash40("long_shift_punch_length"), 3.4));
    param_config::update_int_2(-*WEAPON_KIND_TANTAN_PUNCH1, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_punch"), smash::hash40("beam_frame"), 12));
    param_config::update_float_2(*FIGHTER_KIND_TANTAN, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_speed_max"), 0, 1.3));
    param_config::update_float_2(*FIGHTER_KIND_TANTAN, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.07));
    param_config::update_float_2(*FIGHTER_KIND_TANTAN, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.8));
    param_config::update_float_2(*FIGHTER_KIND_TANTAN, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.0));
    param_config::update_float_2(*FIGHTER_KIND_TANTAN, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 85.0));
    param_config::update_float_2(*FIGHTER_KIND_TANTAN, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 10.0));
    param_config::update_float_2(*FIGHTER_KIND_TANTAN, get_marked_costumes("tantan","tantan").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 10.0));

}