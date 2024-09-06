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
use smash::phx::Vector2f;


pub fn install() {
    Agent::new("wario_coin")
        .status(Pre, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT, shoot_pre)
        .status(Init, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT, shoot_init)
        .status(Main, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT, shoot_main)
        .status(Exec, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT, shoot_exec)
        .status(End, WEAPON_WARIO_COIN_STATUS_KIND_SHOOT, shoot_end)
        .install();
    Agent::new("wario_counter")
        .status(Main, WEAPON_WARIO_COUNTER_STATUS_KIND_APPEAR, counter_main)
        .install();
    Agent::new("wario")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, downb_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, downb_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, downb_end)
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
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(17.0*lr), y: owner_pos_y+7.0, z: owner_pos_z});
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


unsafe extern "C" fn counter_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(weapon.module_accessor);
    let coin_count = COIN_COUNT[ENTRY_ID];
    if true {
        if PostureModule::rot_y_lr(&mut *boma) < 0.0 {
            let mut rotation = Vector3f{x: 0.0, y: 180.0 , z: 0.0};
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("pacmanapple"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});    
        }

        println!("Coin Count: {}", coin_count);
        let tens = (coin_count / 10) as i32;
        let ones = (coin_count % 10) as i32;
        

        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_0"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_1"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_2"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_3"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_4"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_5"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_6"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_7"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_8"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_9"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("1_0"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("1_1"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("1_2"),false);
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("1_3"),false);
        
        ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("coin"),true);
        match tens {
            1 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("1_1"),true),
            2 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("1_2"),true),
            3 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("1_3"),true),
            _ => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("1_0"),true),
        };

        match ones {
            1 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_1"),true),
            2 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_2"),true),
            3 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_3"),true),
            4 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_4"),true),
            5 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_5"),true),
            6 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_6"),true),
            7 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_7"),true),
            8 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_8"),true),
            9 => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_9"),true),
            _ => ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("0_0"),true),
        };
        if SHOW_COUNT[ENTRY_ID] {
            if ALPHA_COUNTER[ENTRY_ID] < 1.0 {
                ALPHA_COUNTER[ENTRY_ID] += 0.075;
            } else {
                ALPHA_COUNTER[ENTRY_ID] = 1.0;
            }
        } else {
            if ALPHA_COUNTER[ENTRY_ID] > 0.0 {
                ALPHA_COUNTER[ENTRY_ID] -= 0.1;
            } else {
                ALPHA_COUNTER[ENTRY_ID] = 0.0;
            }
        }
        ModelModule::set_alpha(weapon.module_accessor, ALPHA_COUNTER[ENTRY_ID]);
    };
    0.into()
}

unsafe extern "C" fn downb_pre(fighter: &mut L2CWeaponCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let tens = (COIN_COUNT[ENTRY_ID] / 10) as i32;
    let fly = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_GASS_LEVEL_FLY);
    let l = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_GASS_LEVEL_L);
    let m = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_GASS_LEVEL_M);
    match tens {
        1 => WorkModule::set_int(fighter.module_accessor, m, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL),
        2 => WorkModule::set_int(fighter.module_accessor, l, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL),
        3 => WorkModule::set_int(fighter.module_accessor, fly, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL),
        _ => WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL),
    };
    StatusModule::init_settings(
		fighter.module_accessor,
		smash::app::SituationKind(*SITUATION_KIND_NONE),
		*FIGHTER_KINETIC_TYPE_FALL,
		*GROUND_CORRECT_KIND_AIR_TRANS as u32,
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
		0,
		0,
		*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
		0
	);
	0.into()
}
unsafe extern "C" fn downb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
	let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let fly = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_GASS_LEVEL_FLY);
    let l = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_GASS_LEVEL_L);
    let m = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_GASS_LEVEL_M);
    let gas = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL);
    StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
	if gas == fly {
		MotionModule::change_motion(fighter.module_accessor, smash::phx::Hash40::new("special_air_lw_fly_r"), 0.0, 1.0, false, 0.0, false, false);
	} else if gas == l {
    	MotionModule::change_motion(fighter.module_accessor, smash::phx::Hash40::new("special_air_lw_lr"), 0.0, 1.0, false, 0.0, false, false);
	} else {
    	MotionModule::change_motion(fighter.module_accessor, smash::phx::Hash40::new("special_air_lw_mr"), 0.0, 1.0, false, 0.0, false, false);
	}
    fighter.sub_shift_status_main(L2CValue::Ptr(downb_main_loop as *const () as _))
}
unsafe extern "C" fn downb_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
	let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
	let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let is_near_ground = GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true) == 1;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
		fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
	if is_near_ground && KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0  && MotionModule::frame(fighter.module_accessor) > 15.0 {
		StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
		WorkModule::set_float(fighter.module_accessor, 15.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
		macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
		return 0.into();
	}
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let tens = (COIN_COUNT[ENTRY_ID] / 10) as i32;
    let fly = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_GASS_LEVEL_FLY);
    let l = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_GASS_LEVEL_L);
    let m = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_GASS_LEVEL_M);
    if  AttackModule::is_attack(fighter.module_accessor, 0, false) {
        match tens {
            1 => AttackModule::set_power(fighter.module_accessor, 0, 12.0, false),
            2 => AttackModule::set_power(fighter.module_accessor, 0, 20.0, false),
            3 => AttackModule::set_power(fighter.module_accessor, 0, 27.0, false),
            _ => AttackModule::set_power(fighter.module_accessor, 0, 5.0, false),
        };
    }
    if  AttackModule::is_attack(fighter.module_accessor, 1, false) {
        match tens {
            1 => AttackModule::set_power(fighter.module_accessor, 1, 12.0, false),
            2 => AttackModule::set_power(fighter.module_accessor, 1, 20.0, false),
            3 => AttackModule::set_power(fighter.module_accessor, 1, 27.0, false),
            _ => AttackModule::set_power(fighter.module_accessor, 1, 5.0, false),
        };
    }
    if  AttackModule::is_attack(fighter.module_accessor, 2, false) {
        match tens {
            1 => AttackModule::set_power(fighter.module_accessor, 2, 12.0, false),
            2 => AttackModule::set_power(fighter.module_accessor, 2, 20.0, false),
            3 => AttackModule::set_power(fighter.module_accessor, 2, 27.0, false),
            _ => AttackModule::set_power(fighter.module_accessor, 2, 5.0, false),
        };
    }
    0.into()
}
unsafe extern "C" fn downb_end(fighter: &mut L2CWeaponCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    COIN_COUNT[ENTRY_ID] = 0;
    0.into()
}