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

unsafe extern "C" fn main_catch_pull(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);

    if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) && fighter_kind == *FIGHTER_KIND_MURABITO { //rayman slots
        fighter.status_CatchPull();
		0.into()
	}
	else{
		return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_PULL)(fighter);
	}
}
unsafe extern "C" fn main_catch_wait(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);

    if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) && fighter_kind == *FIGHTER_KIND_MURABITO { //rayman slots
        fighter.status_CatchWait();
		0.into()
	}
	else{
		return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_WAIT)(fighter);
	}
}
unsafe extern "C" fn main_throw(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);

    //if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) && fighter_kind == *FIGHTER_KIND_MURABITO { //rayman slots
    if motion_kind == hash40("throw_f") {
			fighter.change_status(
				L2CValue::I32(*FIGHTER_STATUS_KIND_THROW_KIRBY),
				L2CValue::Bool(false)
			);
	}
	//}
	return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_THROW)(fighter);
}
unsafe extern "C" fn main_throw_kirby(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);

    //if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) && fighter_kind == *FIGHTER_KIND_MURABITO { //rayman slots
    fighter.status_ThrowKirby();
	0.into()
	//}
	//else{
		//original!(fighter)
	//}
}
unsafe extern "C" fn throw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);

    //if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) && fighter_kind == *FIGHTER_KIND_MURABITO { //rayman slots
    fighter.status_pre_ThrowKirby();
	0.into()
	//}
	//else{
		//original!(fighter)
	//}
}
unsafe extern "C" fn throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);

    //if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) && fighter_kind == *FIGHTER_KIND_MURABITO { //rayman slots
    fighter.status_end_ThrowKirby();
	0.into()
	//}
	//else{
		//original!(fighter)
	//}
}
unsafe extern "C" fn throw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);

    //if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) && fighter_kind == *FIGHTER_KIND_MURABITO { //rayman slots
    L2CFighterCommon::sub_status_uniq_process_ThrowKirby_initStatus(fighter);
	0.into()
	//}
	//else{
		//original!(fighter)
	//}
}
unsafe extern "C" fn throw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);

    //if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) && fighter_kind == *FIGHTER_KIND_MURABITO { //rayman slots
    L2CFighterCommon::sub_status_uniq_process_ThrowKirby_exitStatus(fighter);
	0.into()
	//}
	//else{
		//original!(fighter)
	//}
}

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MURABITO_SPECIAL_HI_DETACH,
        *GROUND_CORRECT_KIND_AIR as u32,
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

pub fn install() {
    Agent::new("murabito")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, main_catch_pull)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, main_catch_wait)
        //.status(Main, *FIGHTER_STATUS_KIND_THROW, main_throw)
        .status(Main, *FIGHTER_STATUS_KIND_THROW_KIRBY, main_throw_kirby)
        .status(Pre, *FIGHTER_STATUS_KIND_THROW_KIRBY, throw_pre)
        .status(End, *FIGHTER_STATUS_KIND_THROW_KIRBY, throw_end)
        .status(Init, *FIGHTER_STATUS_KIND_THROW_KIRBY, throw_init)
        .status(Exit, *FIGHTER_STATUS_KIND_THROW_KIRBY, throw_exit)
        .status(Pre, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, special_hi_pre)
        .install();
}