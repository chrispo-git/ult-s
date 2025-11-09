use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{L2CValue, L2CAgent};
use smash::phx::Vector2f;
use crate::util::*;
use std::mem;

#[derive(PartialEq, Clone, Copy)]
enum Boon {
    NONE,
    SPEED_UP,
    DAMAGE_UP,
    FLOAT_MODE,
    ARMOR,
    DEFENSE_UP,
    POISON,
    TRIPLE_JUMP
}

static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut CURR_BOON : [Boon; 8] = [const { Boon::NONE }; 8];
static mut CURR_EFF : [u32; 8] = [0; 8];
static mut BOON_DURATION : [i32; 8] = [0; 8];
static mut JUMP_MAX : [i32; 8] = [0; 8];
const BOON_MAX : i32 = 1200;
unsafe extern "C" fn smym(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("smym".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(boma);
        let frame = MotionModule::frame(boma) as i32;
        let fighter_kinetic_energy_controller = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)); 
        
        if !smash::app::sv_information::is_ready_go() {
            CURR_BOON[ENTRY_ID] = Boon::NONE;
            BOON_DURATION[ENTRY_ID] = 0;
            CURR_EFF[ENTRY_ID] = 0;
        }
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL && frame == 2 {
			macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_critical_l_tail"));
            let rand_num = smash::app::sv_math::rand(hash40("fighter"), 7);
            CURR_BOON[ENTRY_ID] = match rand_num {
                0 => Boon::SPEED_UP,
                1 => Boon::DAMAGE_UP,
                2 => Boon::FLOAT_MODE,
                3 => Boon::ARMOR,
                4 => Boon::DEFENSE_UP,
                5 => Boon::POISON,
                6 => Boon::TRIPLE_JUMP,
                _ => Boon::NONE
            };
            BOON_DURATION[ENTRY_ID] = BOON_MAX;
			if EffectModule::is_exist_effect(fighter.module_accessor, CURR_EFF[ENTRY_ID]){
				EffectModule::kill(fighter.module_accessor, CURR_EFF[ENTRY_ID], false,false);
			}
            DamageModule::set_damage_mul(boma, 1.0);
			damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_NORMAL, /*DamageThreshold*/ 0);
            AttackModule::set_power_up(boma, 1.0);
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT);
			WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
            smash::app::lua_bind::FighterKineticEnergyController::set_accel_x_mul(fighter_kinetic_energy_controller, 1.0);
			smash::app::lua_bind::FighterKineticEnergyController::set_accel_x_add(fighter_kinetic_energy_controller, 1.0);
            WorkModule::set_int(boma, JUMP_MAX[ENTRY_ID], *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
            CURR_EFF[ENTRY_ID] = EffectModule::req_follow(fighter.module_accessor, get_boon_eff(CURR_BOON[ENTRY_ID]), smash::phx::Hash40::new("hip"), &NONE, &NONE, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        }

        if BOON_DURATION[ENTRY_ID] > 0 {
            BOON_DURATION[ENTRY_ID] -= 1;
        };
        if CURR_BOON[ENTRY_ID] != Boon::TRIPLE_JUMP {
            JUMP_MAX[ENTRY_ID] = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        }
		if BOON_DURATION[ENTRY_ID] > 1 {
            match &CURR_BOON[ENTRY_ID] {
                Boon::SPEED_UP => {
				    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
                    smash::app::lua_bind::FighterKineticEnergyController::set_accel_x_mul(fighter_kinetic_energy_controller, 1.5);
					smash::app::lua_bind::FighterKineticEnergyController::set_accel_x_add(fighter_kinetic_energy_controller, 1.5);
					if [*FIGHTER_STATUS_KIND_DASH].contains(&status_kind) {
						if MotionModule::frame(boma) as i32 == 3 {
								let speed = smash::phx::Vector3f { x: WorkModule::get_param_float(fighter.module_accessor, hash40("dash_speed"), 0), y: 0.0, z: 0.0 };
								KineticModule::add_speed(boma, &speed);
						};
					};
                    if [*FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
						if MotionModule::frame(boma) as i32 == 5 {
								let speed = smash::phx::Vector3f { x: -WorkModule::get_param_float(fighter.module_accessor, hash40("dash_speed"), 0), y: 0.0, z: 0.0 };
								KineticModule::add_speed(boma, &speed);
						};
					};
                },
                Boon::DAMAGE_UP =>  {
                    AttackModule::set_power_up(boma, 1.3);
                },
                Boon::FLOAT_MODE => {
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT);
                }
                Boon::ARMOR => {
                    damage!(fighter, /*MSC=*/*MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, /*DamageThreshold=*/12);
                },
                Boon::DEFENSE_UP =>   {
                    DamageModule::set_damage_mul(boma, 0.65);
                },
                Boon::POISON => {
                    if BOON_DURATION[ENTRY_ID] % 20 == 0 {
                        DamageModule::add_damage(boma, 1.5, 0);
                    }
                },
                Boon::TRIPLE_JUMP =>  {
                    WorkModule::set_int(boma, JUMP_MAX[ENTRY_ID]+1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
                },
                _ => {}
            }
            if CURR_BOON[ENTRY_ID] == Boon::DEFENSE_UP {
                DamageModule::set_damage_mul(boma, 0.65);
            }else if CURR_BOON[ENTRY_ID] == Boon::DAMAGE_UP {
                AttackModule::set_power_up(boma, 1.25);
            }
        } else if BOON_DURATION[ENTRY_ID] > 0{
            DamageModule::set_damage_mul(boma, 1.0);
			damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_NORMAL, /*DamageThreshold*/ 0);
            AttackModule::set_power_up(boma, 1.0);
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT);
			WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
            smash::app::lua_bind::FighterKineticEnergyController::set_accel_x_mul(fighter_kinetic_energy_controller, 1.0);
			smash::app::lua_bind::FighterKineticEnergyController::set_accel_x_add(fighter_kinetic_energy_controller, 1.0);
            WorkModule::set_int(boma, JUMP_MAX[ENTRY_ID], *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
			if EffectModule::is_exist_effect(fighter.module_accessor, CURR_EFF[ENTRY_ID]){
				EffectModule::kill(fighter.module_accessor, CURR_EFF[ENTRY_ID], false,false);
			}
        }
    };
}
unsafe fn get_boon_eff(boonVal: Boon) -> Hash40 {
    match boonVal {
        Boon::NONE => Hash40::new("none"),
        Boon::SPEED_UP => Hash40::new("sys_special_speed_up"),
        Boon::DAMAGE_UP => Hash40::new("sys_special_attack_up"),
        Boon::FLOAT_MODE => Hash40::new("sys_flies_up"),
        Boon::ARMOR => Hash40::new("sys_special_defense_up"),
        Boon::DEFENSE_UP => Hash40::new("sys_status_defense_up"),
        Boon::POISON => Hash40::new("sys_status_down"),
        Boon::TRIPLE_JUMP => Hash40::new("sys_status_all_up")
    }
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, smym)
	.install();
}