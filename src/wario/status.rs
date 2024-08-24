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
    Agent::new("wario_coin")
        .status(Pre, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT, shoot_pre)
        .status(Init, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT, shoot_init)
        .status(Main, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT, shoot_main)
        .status(Exec, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT, shoot_exec)
        .status(End, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT, shoot_end)
        .install();
}


unsafe extern "C" fn shoot_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let angle: f32 = 10.0;
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_coin"), hash40("life"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_coin"), hash40("speed_max"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let owner_pos_x = PostureModule::pos_x(&mut *owner_boma);
    let owner_pos_y = PostureModule::pos_y(&mut *owner_boma);
    let owner_pos_z = PostureModule::pos_z(&mut *owner_boma);
    let speed_y = 0.0;//angle.to_radians().cos()*speed_max;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    ModelModule::set_scale(weapon.module_accessor, 0.65);
    weapon.clear_lua_stack();
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, speed_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x, y: owner_pos_y+7.0, z: owner_pos_z+4.0});
    0.into()
}

unsafe extern "C" fn shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_disarmingvoice"), hash40("speed_max"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let pos = *PostureModule::pos(weapon.module_accessor);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL)
    || life <= 0 {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
        EffectModule::kill_kind(weapon.module_accessor, Hash40::new("poke_meloetta_bullet"), false, false);
        EffectModule::kill_kind(weapon.module_accessor, Hash40::new("rosetta_ring_erase"), false, false);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    if  AttackModule::is_infliction_status(weapon.module_accessor, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT)
    || StopModule::is_stop(weapon.module_accessor)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_ATTACK) {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_flash"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
        EffectModule::kill_kind(weapon.module_accessor, Hash40::new("poke_meloetta_bullet"), false, false);
        EffectModule::kill_kind(weapon.module_accessor, Hash40::new("rosetta_ring_erase"), false, false);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    weapon.fastshift(L2CValue::Ptr(shoot_main_loop as *const () as _))
}

unsafe extern "C" fn shoot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let situation_kind = weapon.global_table[0x16].get_i32();
    let prev_situation_kind = weapon.global_table[0x17].get_i32();
    let pos = *PostureModule::pos(weapon.module_accessor);
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL)
    || life <= 0
    || (situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR) {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
        EffectModule::kill_kind(weapon.module_accessor, Hash40::new("poke_meloetta_bullet"), false, false);
        EffectModule::kill_kind(weapon.module_accessor, Hash40::new("rosetta_ring_erase"), false, false);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    if AttackModule::is_infliction_status(weapon.module_accessor, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT)
    || StopModule::is_stop(weapon.module_accessor)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_ATTACK) {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_flash"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
        EffectModule::kill_kind(weapon.module_accessor, Hash40::new("poke_meloetta_bullet"), false, false);
        EffectModule::kill_kind(weapon.module_accessor, Hash40::new("rosetta_ring_erase"), false, false);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn shoot_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let mut rotation = Vector3f{x: 0.0, y: 0.0 , z: 90.0};
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("have"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    0.into()
}

unsafe extern "C" fn shoot_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("poke_meloetta_bullet"), false, false);
    0.into()
}