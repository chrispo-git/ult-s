use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::BattleObjectModuleAccessor;
use smash::app::FighterEntryID;
use skyline::nn::ro::LookupSymbol;
use core::f32::consts::PI;
use smash::app;
use crate::common::*;
use crate::util::*;


pub static mut FIGHTER_MANAGER: usize = 0;

static mut ANGLE_DEGREES: f32 = 0.0;

pub unsafe fn fighter_info(fighter_boma: &mut BattleObjectModuleAccessor) -> *mut smash::app::FighterInformation {
    let entry_id = WorkModule::get_int(fighter_boma,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    return smash::app::lua_bind::FighterManager::get_fighter_information(fighter_manager,FighterEntryID(entry_id));
}


pub(crate) unsafe fn main_logic(fighter: &mut L2CFighterCommon) -> () {
    let cpu = FighterInformation::is_operation_cpu(fighter_info(&mut *fighter.module_accessor));
    let training = smash::app::smashball::is_training_mode();

    let spd_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);
    let spd_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);

    let vel = (spd_x * spd_x + spd_y * spd_y).sqrt();
    let angle = spd_y.atan2(spd_x);

    let mut x = 0.0;
    let mut y = 0.0;

    if MotionModule::frame(fighter.module_accessor) == 0.0 {
        ANGLE_DEGREES = (angle * 180.0 / PI + 360.0) % 360.0;
    }
    if cpu && (!training || IS_GLOW){
        let rand_num = smash::app::sv_math::rand(hash40("fighter"), 100);
        let rand_num2 = smash::app::sv_math::rand(hash40("fighter"), 100);
        let rand_1_val = if rand_num % 2 == 0 {1.0} else {-1.0};
        let rand_2_val = if rand_num2 % 2 == 0 {1.0} else {-1.0};

        if ANGLE_DEGREES >= 120.0 && ANGLE_DEGREES <= 221.0 {
            // Hit is strong, DI the hit in
            if vel >= 4.0 {
                // Left DI
                x = 1.0;
            } else {
                // Random DI with 1/100 chance of doing specific DI, checking if the number is even
                x = rand_1_val;
            }
        } else if ANGLE_DEGREES >= 0.0 && ANGLE_DEGREES <= 60.0 || ANGLE_DEGREES <= 359.0 && ANGLE_DEGREES >= 300.0 {
            // Hit is strong, DI the hit in
            if vel >= 4.0 {
                // Right DI
                x = -1.0;
            } else {
                // Random DI with 1/100 chance of doing specific DI, checking if the number is even
                x = rand_1_val;
            }
        } else if ANGLE_DEGREES == 90.0 {
            x = rand_1_val;
            y = 0.0;
        } else if ANGLE_DEGREES >= 91.0 && ANGLE_DEGREES < 120.0 {
            // Hit is strong, DI the hit in
            if vel >= 4.0 {
                x = 1.0;
                y = -1.0;
            } else {
                // Random DI with 1/100 chance of doing specific DI, checking if the number is even
                x = rand_1_val;
        
                // Random DI with 1/100 chance of doing specific DI, checking if the number is even
                y = rand_2_val;
            }
        } else if ANGLE_DEGREES <= 89.0 && ANGLE_DEGREES > 60.0 {
            // Hit is strong, DI the hit in
            if vel >= 4.0 {
                // Right DI
                x = -1.0;
                y = -1.0;
            } else {
                // Random DI with 1/100 chance of doing specific DI, checking if the number is even
                x = rand_1_val;
                // Random DI with 1/100 chance of doing specific DI, checking if the number is even
                y = rand_2_val;
            }
        }
        if DI_DIR != 0 {
            let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            if POS_X[ENTRY_ID] > POS_X[0] {
                x = DI_DIR as f32;
            } else {
                x = -DI_DIR as f32;
            }
        }
        WorkModule::set_float(fighter.module_accessor, x, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X);
        WorkModule::set_float(fighter.module_accessor, y, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y);
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageFly_Main)]
pub unsafe fn dmg_fly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    main_logic(fighter);
    original!()(fighter)
}
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageFlyRoll_Main)]
pub unsafe fn dmg_fly_roll_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    main_logic(fighter);
    original!()(fighter)
}
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Damage_Main)]
pub unsafe fn dmg_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    main_logic(fighter);
    original!()(fighter)
}
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageAir_Main)]
pub unsafe fn dmg_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    main_logic(fighter);
    original!()(fighter)
}

pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
    }
}