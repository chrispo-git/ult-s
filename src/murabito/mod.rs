mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;
			
static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_TREE_POS_X : i32 = 0;
static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_TREE_POS_Y : i32 = 1;
static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_IS_FALLEN : i32 = 2;
static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_DO_BOUNCE : i32 = 3;
static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_CHANGE_FRAME : i32 = 4;
static FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_HAS_BEEN_AIR : i32 = 5;
static mut Y_DIST : f32 = 10.0;
static mut X_DIST : f32 = 10.0;

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}