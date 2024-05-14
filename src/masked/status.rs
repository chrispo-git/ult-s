use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use smash::phx::Vector3f;
use crate::util::*;
use super::*;
use crate::masked::*;
pub fn install() {
    Agent::new("lucas")
        .status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO, zair_main)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_main)
        .status(Init, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD, specialhi_hold_init)
        .status(Pre, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD, specialhi_hold_pre)
        .status(Main, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD, specialhi_hold_main)
        .status(Exec, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD, specialhi_hold_exec)
        .status(Init, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, specialhi_end_init)
        //.status(Init, *FIGHTER_STATUS_KIND_FALL_SPECIAL, special_fall_init)
        .status(Pre, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, specialhi_end_pre)
        .status(Main, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, specialhi_end_main)
        .status(Exec, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD, speciallw_hold_exec)
        .install();
}


unsafe extern "C" fn zair_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);    
    let fighter_kind = smash::app::utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCAS && is_added(boma) {	
        agent.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), false.into());
        return 0.into();
    }
    else {
        return smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_AIR_LASSO)(agent);
    }	
}

unsafe extern "C" fn specials_situation_helper(fighter: &mut L2CFighterCommon, is_start: bool, status: i32) {
    let is_Ground = StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND;

    let motion_g;
    let motion_a;

    if status == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD {
        motion_g = Hash40::new("special_hi_hold");
        motion_a = Hash40::new("special_hi_hold");
    }
    else if status == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END {
        motion_g = Hash40::new("special_hi_end");
        motion_a = Hash40::new("special_hi_end");
    }
    else {
        motion_g = Hash40::new("special_hi_start");
        motion_a = Hash40::new("special_air_hi_start");
    }
    if is_start && false{
        let motion = if is_Ground {motion_g} else {motion_a};
        MotionModule::change_motion(fighter.module_accessor,motion,0.0,1.0,false,0.0,false,false);
    }
    else{
        fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), (!is_start).into());
    }

    let correct = if is_Ground {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(correct));

    if status == *FIGHTER_STATUS_KIND_SPECIAL_HI || !is_start {
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    }
    
}

unsafe extern "C" fn specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    if !is_added(boma) {
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    if !is_added(boma) {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
    }
    specials_situation_helper(fighter,true,*FIGHTER_STATUS_KIND_SPECIAL_HI);

    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_main_loop as *const () as _))
}

unsafe extern "C" fn specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        //fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD.into(),false.into());
        return 0.into();
    }
    
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        specials_situation_helper(fighter,false,*FIGHTER_STATUS_KIND_SPECIAL_HI);
    }

    0.into()
}


unsafe extern "C" fn specialhi_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    if !is_added(boma) {
        return smashline::original_status(Pre, fighter, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter);
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MIIGUNNER_SPECIAL_HI2_JUMP,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn specialhi_hold_init_kinetic(fighter: &mut L2CFighterCommon){
    let is_air = StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND;
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    let stick_x =  ControlModule::get_stick_x(fighter.module_accessor)*PostureModule::lr(fighter.module_accessor);
    //speed_x range from 0.0 to 1.0
    let speed_x = (((stick_x+1.0)/2.0))*PostureModule::lr(fighter.module_accessor);
    let speed_x_mul = 0.75;
    let speed_y = 6.42*0.625;//WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("attack_speed"));
    let air_speed_y_mul = if is_air {1.03} else {1.0};
    let accel_y = 0.15;//WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("accel_y"));
    let accel_x_mul = 0.1; //WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_xmul"));
    let stable_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let stable_speed_x_mul = 0.875;
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        speed_x*speed_x_mul
    );
    sv_kinetic_energy!(
        set_accel_x_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        accel_x_mul
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        stable_speed_x*stable_speed_x_mul,
        0.0
    );

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        speed_y*air_speed_y_mul
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -accel_y
    );
}

unsafe extern "C" fn specialhi_hold_init(fighter: &mut L2CFighterCommon) -> L2CValue {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    if !is_added(boma) {
        return smashline::original_status(Init, fighter, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter);
    }
    specialhi_hold_init_kinetic(fighter);
    0.into()
}

unsafe extern "C" fn specialhi_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    if !is_added(boma) {
        return smashline::original_status(Main, fighter, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter);
    }
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

    specials_situation_helper(fighter,true,*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD);
    
    let max_frame = 180;//WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("stop_time"));
    WorkModule::set_int(fighter.module_accessor, max_frame, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_STOP_TIME);
    WorkModule::set_int(fighter.module_accessor, max_frame, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_TIME);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_hold_main_loop as *const () as _))
}

unsafe extern "C" fn specialhi_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let max_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_TIME) <= 0;
    if speed_y <= 0.5 || max_frame {
        fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END.into(),false.into());
        return 0.into();
    }
    

    0.into()
}

unsafe extern "C" fn specialhi_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    if !is_added(boma) {
        return 0.into();
    }
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_TIME);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let speed_x = smash::app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);

    let stick_x =  ControlModule::get_stick_x(fighter.module_accessor)*PostureModule::lr(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_x.abs()>0.0 
    && speed_x.signum() != lr {
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
        macros::SET_SPEED_EX(fighter, 0, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    else {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    0.into()
}

unsafe extern "C" fn specialhi_end_init_kinetic(fighter: &mut L2CFighterCommon){
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("accel_y"));
    let accel_y_mul = 1.5;
    let accel_x_mul = 0.05;
    let stable_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let stable_speed_x_mul = 0.35;
    sv_kinetic_energy!(
        set_accel_x_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        accel_x_mul
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        stable_speed_x*stable_speed_x_mul,
        0.0
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -accel_y*accel_y_mul
    );
}
unsafe extern "C" fn specialhi_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    if !is_added(boma) {
        return 0.into();
    }
    specialhi_end_init_kinetic(fighter);
    0.into()
}
unsafe extern "C" fn special_fall_init(fighter: &mut L2CFighterCommon) -> L2CValue {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    if !is_added(boma) {
        return 0.into();
    }
    specialhi_end_init_kinetic(fighter);
    0.into()
}

unsafe extern "C" fn specialhi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    if !is_added(boma) {
        return smashline::original_status(Pre, fighter, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END)(fighter);
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn specialhi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    if !is_added(boma) {
        return smashline::original_status(Main, fighter, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END)(fighter);
    }
    specials_situation_helper(fighter,true,*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END);

    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_end_main_loop as *const () as _))
}

unsafe extern "C" fn specialhi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        specials_situation_helper(fighter,false,*FIGHTER_STATUS_KIND_SPECIAL_HI);
    }

    0.into()
}

unsafe extern "C" fn speciallw_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    if !is_added(boma) {
        return 0.into();
    }

    let mut time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_TIME);
    let inflict = AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT);
    if inflict {
        if time > 0 {
            WorkModule::set_int(fighter.module_accessor, 0,*FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_TIME);
            time = 0;
        }
        else {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_TIME);
        }
    }
    if time <= 0 && inflict {
        println!("Time:{time}");
        if time-1 <= -MAX_HITS {
            fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END.into(),false.into());
            return 1.into();
        }
    }
    0.into()
}