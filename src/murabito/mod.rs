mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_TREE_POS_X: i32 = 0;
static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_TREE_POS_Y: i32 = 1;
static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_IS_FALLEN: i32 = 2;
static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_DO_BOUNCE: i32 = 3;
static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_CHANGE_FRAME: i32 = 4;
static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_HAS_BEEN_AIR: i32 = 5;
static Y_DIST: f32 = 10.0;
static X_DIST: f32 = 10.0;

pub fn install() {
    frame::install();
    status::install();
    acmd::install();
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MURABITO,
        get_marked_costumes("murabito", "murabito")
            .into_iter()
            .map(|x| x as i32)
            .collect(),
        (smash::hash40("run_speed_max"), 0, 1.5)
    );
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MURABITO,
        get_marked_costumes("murabito", "murabito")
            .into_iter()
            .map(|x| x as i32)
            .collect(),
        (smash::hash40("air_speed_x_stable"), 0, 1.1)
    );
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MURABITO,
        get_marked_costumes("murabito", "murabito")
            .into_iter()
            .map(|x| x as i32)
            .collect(),
        (smash::hash40("landing_attack_air_frame_f"), 0, 12.0)
    );
    crate::param_cache::update_float_2(
        *FIGHTER_KIND_MURABITO,
        get_marked_costumes("murabito", "murabito")
            .into_iter()
            .map(|x| x as i32)
            .collect(),
        (smash::hash40("landing_attack_air_frame_b"), 0, 12.0)
    );
}
