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
pub fn install() {
	Agent::new("inkling_squid")
	.status(Exec, *WEAPON_INKLING_SQUID_STATUS_KIND_SPECIAL_HI_JUMP, squid_exec_jump)
	.status(Exec, *WEAPON_INKLING_SQUID_STATUS_KIND_SPECIAL_HI_ROT, squid_exec_rot)
	.status(Exec, *WEAPON_INKLING_SQUID_STATUS_KIND_SPECIAL_HI_FALL, squid_exec_fall)
    .install();
}
unsafe extern "C" fn squid_exec_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Exec, fighter, *WEAPON_INKLING_SQUID_STATUS_KIND_SPECIAL_HI_JUMP)(fighter);
    let otarget_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_INKLING && MotionModule::motion_kind(&mut *boma) == hash40("special_hi_down") {
        let mut rotation = Vector3f{x: 0.0, y: 30.0 , z: 0.0};
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        println!("squid down");
    } else {
        println!("squid up");
    }
    0.into()
}
unsafe extern "C" fn squid_exec_rot(fighter: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Exec, fighter, *WEAPON_INKLING_SQUID_STATUS_KIND_SPECIAL_HI_ROT)(fighter);
    let otarget_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_INKLING && MotionModule::motion_kind(&mut *boma) == hash40("special_hi_down") {
        let mut rotation = Vector3f{x: 0.0, y: 30.0 , z: 0.0};
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        println!("squid down");
    } else {
        println!("squid up");
    }
    0.into()
}
unsafe extern "C" fn squid_exec_fall(fighter: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Exec, fighter, *WEAPON_INKLING_SQUID_STATUS_KIND_SPECIAL_HI_FALL)(fighter);
    let otarget_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_INKLING && MotionModule::motion_kind(&mut *boma) == hash40("special_hi_down") {
        let mut rotation = Vector3f{x: 0.0, y: 30.0 , z: 0.0};
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        println!("squid down");
    } else {
        println!("squid up");
    }
    0.into()
}