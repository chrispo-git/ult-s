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

static mut IS_SUPERBOSS : [bool; 8] = [false; 8];
static mut IS_PARTICIPATING : [bool; 8] = [false; 8];
static mut UPDATE_COUNTER : [i32; 8] = [0; 8];
static mut ACTIVES_COUNT : [i32; 8] = [0; 8];
static mut SUPERBOSS_CHOSEN : bool = false;
unsafe extern "C" fn superboss(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("superboss".to_string()) {
            return;
        }
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(boma);
        if !smash::app::sv_information::is_ready_go() && ![*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE].contains(&status_kind) {
            UPDATE_COUNTER[ENTRY_ID] += 1;

            if UPDATE_COUNTER[ENTRY_ID] == 1 {
                SUPERBOSS_CHOSEN = false;
                IS_PARTICIPATING[ENTRY_ID] = true;
                println!("Player {} participating", ENTRY_ID);
            } else if UPDATE_COUNTER[ENTRY_ID] == 2 {
                ACTIVES_COUNT[ENTRY_ID] = 0;
                for _ in 0..8 {
                    if IS_PARTICIPATING[ENTRY_ID] == true {
                        ACTIVES_COUNT[ENTRY_ID] += 1;
                    }
                }
                println!("Player {} counts {} players!", ENTRY_ID, ACTIVES_COUNT[ENTRY_ID]);
            } else if UPDATE_COUNTER[ENTRY_ID] == (3+ENTRY_ID) {
                if !SUPERBOSS_CHOSEN {
                    if smash::app::sv_math::rand(hash40("fighter"), ACTIVES_COUNT[ENTRY_ID]+1) == 0 {
                        println!("Player {} is the superboss!", ENTRY_ID);
                        IS_SUPERBOSS[ENTRY_ID] = true;
                    } else {
                        for i in 0..8 {
                            if ACTIVES_COUNT[i] > 0 {
                                ACTIVES_COUNT[i] -= 1;
                            }
                        }
                    }
                } else {
                    println!("Superboss already chosen, skipping...");
                }
            }
        } else {
            UPDATE_COUNTER[ENTRY_ID] = 0;
            for i in 0..8 {
                IS_PARTICIPATING[i] = false;
            }
            ACTIVES_COUNT[ENTRY_ID] = 0;
        }

        if IS_SUPERBOSS[ENTRY_ID] {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KINOKO);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);

            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);

            MotionModule::set_rate(boma, 0.5);

            if [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4
            ].contains(&status_kind) {
			    damage!(fighter, /*MSC*/ *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, /*DamageThreshold*/ 0);
            } else {
			    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10.0);
            }
        }
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, superboss)
	.install();
}