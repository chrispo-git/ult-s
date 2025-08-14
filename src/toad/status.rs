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
unsafe extern "C" fn landing_fall_special_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("propeller"),false);
    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("mushroom"),true);
	0.into()
}
unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
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
//Init, End and Exit don't exist for Mario's Fireball

unsafe extern "C" fn regular_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn regular_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let lr = PostureModule::lr(weapon.module_accessor);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
    let owner_pos_x = PostureModule::pos_x(&mut *owner_boma);
    let owner_pos_y = PostureModule::pos_y(&mut *owner_boma);
    let owner_pos_z = PostureModule::pos_z(&mut *owner_boma);
    WorkModule::set_int(weapon.module_accessor, 90, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(8.0*lr), y: owner_pos_y+7.0, z: owner_pos_z});
    let speed_x = 1.38;
    let speed_y = -0.3;
    let gravity = 0.07;
    weapon.clear_lua_stack();
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*lr, speed_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*lr, speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

unsafe extern "C" fn regular_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("throwed"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(regular_main_loop as *const () as _))
}

unsafe extern "C" fn regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    ModelModule::set_scale(weapon.module_accessor, 1.5);
    let remaining_life = life <= 0;
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
        return 0.into();
    }
    if !remaining_life {
        if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            return 0.into();
        }
        /*notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        weapon.pop_lua_stack(1);*/
        if remaining_life ||  GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            weapon.pop_lua_stack(1);
            return 0.into();
        }
    }
    else {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn regular_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let is_near_ground = GroundModule::ray_check(weapon.module_accessor, &Vector2f{ x: PostureModule::pos_x(weapon.module_accessor), y: PostureModule::pos_y(weapon.module_accessor)}, &Vector2f{ x: 0.0, y: -4.0}, true) == 1;

    if is_near_ground {
        let speed_x = 1.38;
        let bounce = 1.3;
        let lr = PostureModule::lr(weapon.module_accessor);
        weapon.clear_lua_stack();
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*lr, bounce);
        sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*lr, bounce);
		macros::EFFECT_FOLLOW(weapon, Hash40::new("popo_iceshot_cold_b"), Hash40::new("have"), 0, 0, 0, 0, 0, 0, 1, true);
    } else {
        macros::EFFECT_OFF_KIND(weapon, Hash40::new("popo_iceshot_cold_b"), false, true);
    }
    0.into()
}

unsafe extern "C" fn regular_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
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
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre)
        .status(Exit, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, landing_fall_special_exit)
        .install();
    Agent::new("murabito_flowerpot")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
        .status(Init, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED, regular_init)
        .status(Pre, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED, regular_pre)
        .status(Main, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED, regular_main)
        .status(Exec, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED, regular_exec)
        .status(End, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED, regular_end)
        .install();
} 