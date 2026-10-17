mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

/*static mut SIDEB_COOLDOWN : i32 = 300;*/
static FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_CHARGE_FRAMES: i32 = 0;
static FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_NO_FP: i32 = 1;
static MAX_FRAMES: i32 = 60;
static FP_DELAY: i32 = 100;

pub fn install() {
    frame::install();
    status::install();
    acmd::install();

    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MIIGUNNER,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec(),
        (smash::hash40("ground_brake"), 0, 0.07)
    );
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MIIGUNNER,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec(),
        (smash::hash40("dash_speed"), 0, 1.7)
    );
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MIIGUNNER,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec(),
        (smash::hash40("run_speed_max"), 0, 1.45)
    );
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MIIGUNNER,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec(),
        (smash::hash40("jump_speed_x_mul"), 0, 0.8)
    );
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MIIGUNNER,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec(),
        (smash::hash40("air_speed_x_stable"), 0, 0.89)
    );
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MIIGUNNER,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec(),
        (smash::hash40("weight"), 0, 102.0)
    );
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MIIGUNNER,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec(),
        (smash::hash40("landing_attack_air_frame_n"), 0, 9.0)
    );
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MIIGUNNER,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec(),
        (smash::hash40("landing_attack_air_frame_f"), 0, 12.0)
    );
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MIIGUNNER,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec(),
        (smash::hash40("landing_attack_air_frame_lw"), 0, 15.0)
    );

    crate::param_cache::update_int_2(
        -*WEAPON_KIND_MIIGUNNER_GROUNDBOMB,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec(),
        (
            smash::hash40("param_groundbomb"),
            smash::hash40("life"),
            180,
        )
    );
}
