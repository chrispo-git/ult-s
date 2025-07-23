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
use std::collections::VecDeque;
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };


static mut IS_SUPERBOSS : [bool; 8] = [false; 8];
static mut UPDATE_COUNTER : [i32; 8] = [0; 8];
static mut SUPERBOSS_CHOSEN : bool = false;
static mut ACTIVE_PARTICIPANTS : VecDeque<i32> = VecDeque::new();
static mut DEFENDERS : VecDeque<i32> = VecDeque::new();
static mut DESIGNATED_WINNER : i32 = -1;
static mut SUPERBOSS_DEAD : bool = false;
unsafe extern "C" fn superboss(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("superboss".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(boma);
        if !smash::app::sv_information::is_ready_go() && ![*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE].contains(&status_kind) && ![hash40("lose")].contains(&MotionModule::motion_kind(boma)) {
            UPDATE_COUNTER[ENTRY_ID] += 1;

            if UPDATE_COUNTER[ENTRY_ID] == 1 {
                SUPERBOSS_CHOSEN = false;
                SUPERBOSS_DEAD = false;
                ACTIVE_PARTICIPANTS.push_back(WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID));
                DEFENDERS = VecDeque::new();
                println!("Player {} participating", ENTRY_ID);
            } else if UPDATE_COUNTER[ENTRY_ID] == (2+WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID)) {
                if !SUPERBOSS_CHOSEN {
                    let val = smash::app::sv_math::rand(hash40("stage"), ACTIVE_PARTICIPANTS.len() as i32);
                    println!("There are {} players involved", ACTIVE_PARTICIPANTS.len());
                    if val == 0 {
                        TeamModule::set_hit_team(boma, 0);
                        TeamModule::set_team(boma, 0, true);
                        println!("Player {} is the superboss!", ENTRY_ID);
			            EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_special_all_up"), smash::phx::Hash40::new("hip"), &NONE, &NONE, 1.25, true, 0, 0, 0, 0, 0, true, true) as u32;
                        IS_SUPERBOSS[ENTRY_ID] = true;
                        SUPERBOSS_CHOSEN = true;
                        ACTIVE_PARTICIPANTS.pop_front();
                    } else {
                        TeamModule::set_hit_team(boma, 1);
                        TeamModule::set_team(boma, 1, true);
                        println!("Player {} is NOT the superboss! next one (btw the value they got was {})", ENTRY_ID, val);
                        let next_win = ACTIVE_PARTICIPANTS.pop_front();
                        if DESIGNATED_WINNER == -1 {
                            match next_win {
                                Some(x) => {DESIGNATED_WINNER = x; println!("Player {} will be the winner if the superboss loses", DESIGNATED_WINNER);},
                                None => println!("Nobody left?"),
                            }
                        } else {
                            match next_win {
                                Some(x) => {DEFENDERS.push_back(x); println!("Player {} is a backup", x);},
                                None => println!("Nobody left?"),
                            }
                        }
                    }
                } else {
                        TeamModule::set_hit_team(boma, 1);
                        TeamModule::set_team(boma, 1, true);
                        println!("Superboss already chosen, skipping...");
                        let next_win = ACTIVE_PARTICIPANTS.pop_front();
                        if DESIGNATED_WINNER == -1 {
                            match next_win {
                                Some(x) => {DESIGNATED_WINNER = x; println!("Player {} will be the winner if the superboss loses", DESIGNATED_WINNER);},
                                None => println!("Nobody left?"),
                            }
                        } else {
                            match next_win {
                                Some(x) => {DEFENDERS.push_back(x); println!("Player {} is a backup", x);},
                                None => println!("Nobody left?"),
                            }
                        }
                }
            }
        } else {
            UPDATE_COUNTER[ENTRY_ID] = 0;
            if !ACTIVE_PARTICIPANTS.is_empty() {
                ACTIVE_PARTICIPANTS = VecDeque::new();
            }
        }
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == DESIGNATED_WINNER && stock_count(boma) == 0 {
            let next_up = DEFENDERS.pop_front();
            match next_up {
                Some(x) => {DESIGNATED_WINNER = x;  println!("Player {} is now next up!", x)}
                None => println!("Nobody left?"),
            }
        }
        if IS_SUPERBOSS[ENTRY_ID] {
            ModelModule::set_scale(boma, 1.8);
            AttackModule::set_attack_scale(boma, 1.0, true);
            GrabModule::set_size_mul(boma, 1.8);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KINOKO);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);

            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);

            if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_OFF, false);
            }

            if ![*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind) {
                MotionModule::set_rate(boma, 0.65);
            }

            if [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4
            ].contains(&status_kind) {
			    damage!(fighter, /*MSC*/ *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, /*DamageThreshold*/ 0);
            } else {
			    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10.0);
            }
            println!("Superboss stock count : {}", stock_count(boma));
            if stock_count(boma) == 0 {
                SUPERBOSS_DEAD = true;
            } else {
                SUPERBOSS_DEAD = false;
            }
        }
        if smash::app::sv_information::is_ready_go() && SUPERBOSS_DEAD {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) != DESIGNATED_WINNER && stock_count(boma) > 0 {
                println!("tough luck! time to die");
                if ![*FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, false);
                }
            }
        }
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, superboss)
	.install();
}