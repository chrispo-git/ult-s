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
use crate::peppy::*;
use super::*;



pub fn install() {
	Agent::new("falco")
    .set_costume([120, 121, 122, 123, 124, 125, 126, 127].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, sideb_end)
    .install();
    Agent::new("falco_missile")
        .status(Pre, 0, regular_pre)
        .status(Init, 0, regular_init)
        .status(Main, 0, regular_main)
        .status(Exec, 0, regular_exec)
        .status(End, 0, regular_end)
        .install();
}
unsafe extern "C" fn sideb_end(fighter: &mut L2CWeaponCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	0.into()
}

unsafe extern "C" fn regular_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn regular_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
    let speed = 2.5;
    let angle: f32 = 45.0;
    
    let speed_x = (angle.to_radians().sin()*speed);
    let speed_y = -(angle.to_radians().cos()*speed);
    let lr = PostureModule::lr(weapon.module_accessor);
    let ENTRY_ID = WorkModule::get_int(&mut *owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let owner_pos_x = PostureModule::pos_x(&mut *owner_boma);
    let owner_pos_y = PostureModule::pos_y(&mut *owner_boma);
    let owner_pos_z = PostureModule::pos_z(&mut *owner_boma);
    let owner_is_grounded = StatusModule::situation_kind(&mut *owner_boma) == *SITUATION_KIND_GROUND;
    ModelModule::set_scale(weapon.module_accessor, 1.5);
    weapon.clear_lua_stack();
    if !owner_is_grounded {
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*lr, speed_y);
        sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*lr, speed_y);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(12.0*lr), y: owner_pos_y, z: owner_pos_z});
        AIR_SHOT[ENTRY_ID] = true;
    } else {
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed*lr, 0.0);
        sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed*lr, 0.0);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(15.0*lr), y: owner_pos_y+7.0, z: owner_pos_z});
        AIR_SHOT[ENTRY_ID] = false;
    }
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

unsafe extern "C" fn regular_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = 45;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(regular_main_loop as *const () as _))
}

unsafe extern "C" fn regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let remaining_life = life <= 0;
    let lr = PostureModule::lr(weapon.module_accessor);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
    let ENTRY_ID = WorkModule::get_int(&mut *owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if AIR_SHOT[ENTRY_ID] {
        let mut rotation = Vector3f{x: 45.0, y: 0.0 , z: 0.0};
        ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
    if 
    (ray_check_pos(&mut *weapon.module_accessor, 4.0, 0.0, true)==1 && lr > 0.0) || 
    (ray_check_pos(&mut *weapon.module_accessor, -4.0, 0.0, true)==1 && lr < 0.0) || 
    (ray_check_pos(&mut *weapon.module_accessor, 0.0, -4.0, true)==1) || 
    (ray_check_pos(&mut *weapon.module_accessor, 0.0, 4.0, true)==1) || 
    remaining_life || AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) || StopModule::is_stop(weapon.module_accessor) {
        if MotionModule::motion_kind(weapon.module_accessor) != hash40("explode") {
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, false, 0.0, false, false);
            sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        }
    }
    if MotionModule::frame(weapon.module_accessor) > 10.0 && MotionModule::motion_kind(weapon.module_accessor) == hash40("explode") {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn regular_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn regular_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}