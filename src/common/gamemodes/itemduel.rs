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
use smash2::app::ItemManager;
use std::ptr;
static mut ITEM_OPTION : [i32; 8] = [-1; 8];
static mut ITEM_HELD : [*mut smash::app::Item; 8] = [ptr::null_mut(); 8];
static mut HAS_CHOSEN : [bool; 8] = [false; 8];
static mut IS_IN_ENTRY : [bool; 8] = [false; 8];
static mut REBIRTH_DO_NOW : [bool; 8] = [false; 8];
static mut DELAY_UNTIL_ACT : [i32; 8] = [30; 8];
unsafe extern "C" fn itemduel(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("itemduel".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
        if [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_NONE].contains(&status_kind) || 
        ([*FIGHTER_STATUS_KIND_STANDBY, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) && !smash::app::sv_information::is_ready_go()) ||
        [
            hash40("win_1"), hash40("win_1_wait"),
            hash40("win_2"), hash40("win_2_wait"),
            hash40("win_3"), hash40("win_3_wait"),
            hash40("lose")
            ].contains(&MotionModule::motion_kind(boma)) {
            for i in 0..8 {
                ITEM_OPTION[i] = -1;
                ITEM_HELD[i] = ptr::null_mut();
                HAS_CHOSEN[i] = false;
                IS_IN_ENTRY[i] = false;
                REBIRTH_DO_NOW[i] = false;
                DELAY_UNTIL_ACT[i] = 30;
            }
            return;
        }
        if DELAY_UNTIL_ACT[ENTRY_ID] > 0 {
            DELAY_UNTIL_ACT[ENTRY_ID] -= 1;
            return;
        }
        if smash::app::sv_information::is_ready_go() {
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD);
        }
        if smash::app::sv_information::is_ready_go() || status_kind == *FIGHTER_STATUS_KIND_WAIT {
            if ![*FIGHTER_STATUS_KIND_ITEM_SWING, *FIGHTER_STATUS_KIND_ITEM_SWING_S3, *FIGHTER_STATUS_KIND_ITEM_SWING_S4,
                *FIGHTER_STATUS_KIND_ITEM_SHOOT_AIR, *FIGHTER_STATUS_KIND_ITEM_SHOOT_FLY, *FIGHTER_STATUS_KIND_ITEM_SHOOT_JUMP,
                *FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT, *FIGHTER_STATUS_KIND_ITEM_SWING_DASH, *FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_B,
                *FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_F, *FIGHTER_STATUS_KIND_ITEM_SHOOT_LANDING, *FIGHTER_STATUS_KIND_ITEM_SWING_S4_HOLD,
                *FIGHTER_STATUS_KIND_ITEM_SWING_S4_START, *FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_BRAKE_B, *FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_BRAKE_F,
                *FIGHTER_STATUS_KIND_ITEM_STARRING
            ].contains(&status_kind) {
                if smash::app::sv_math::rand(hash40("fighter"), 500) == 0 {
                    println!("Player {} having their item refreshed", ENTRY_ID);
                    setItem(fighter, ITEM_OPTION[ENTRY_ID]);
                    let item_manager = ItemManager::instance().unwrap();
                    ITEM_HELD[ENTRY_ID] = item_manager.find_active_item_from_id(ItemModule::get_have_item_id(boma, 0) as u32) as *mut smash::app::Item;
                }
            }
        }
        if [*FIGHTER_STATUS_KIND_ITEM_THROW].contains(&status_kind) {
            if situation_kind != *SITUATION_KIND_AIR {
			    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
            } else {
			    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
        }
        if [*FIGHTER_STATUS_KIND_ITEM_THROW_DASH].contains(&status_kind) {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_DASH, true);
        }
        if !ItemModule::is_have_item(boma, 0) && ![*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY].contains(&status_kind) {
            if !ITEM_HELD[ENTRY_ID].is_null() {
                //println!("Player {} retrieving their item...", ENTRY_ID);
                ItemModule::remove_item(boma, 0);
                ItemModule::have_item_instance(boma, ITEM_HELD[ENTRY_ID], 0, false, false, false, false);
            } else {
                println!("Player {} can't find their item! creating new one...", ENTRY_ID);
                setItem(fighter, ITEM_OPTION[ENTRY_ID]);
                let item_manager = ItemManager::instance().unwrap();
                ITEM_HELD[ENTRY_ID] = item_manager.find_active_item_from_id(ItemModule::get_have_item_id(boma, 0) as u32) as *mut smash::app::Item;
            }
        }
        if (!smash::app::sv_information::is_ready_go() && status_kind != *FIGHTER_STATUS_KIND_WAIT && !IS_IN_ENTRY[ENTRY_ID]) || 
        (REBIRTH_DO_NOW[ENTRY_ID] && status_kind == *FIGHTER_STATUS_KIND_REBIRTH) {
            if !HAS_CHOSEN[ENTRY_ID] && ![*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE].contains(&status_kind){
                println!("Player {} choosing their item...", ENTRY_ID);
                ITEM_OPTION[ENTRY_ID] = smash::app::sv_math::rand(hash40("fighter"), 11);
                setItem(fighter, ITEM_OPTION[ENTRY_ID]);
                HAS_CHOSEN[ENTRY_ID] = true;
                REBIRTH_DO_NOW[ENTRY_ID] = false;
            }
        }
        if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY].contains(&status_kind){
            HAS_CHOSEN[ENTRY_ID] = false;
            REBIRTH_DO_NOW[ENTRY_ID] = true;
        }

        if status_kind == *FIGHTER_STATUS_KIND_ENTRY {
            IS_IN_ENTRY[ENTRY_ID] = true;
        } else {
            IS_IN_ENTRY[ENTRY_ID] = false;
        }
    };
}

unsafe fn setItem(fighter : &mut L2CFighterCommon, val : i32) -> () {
    let item = match val {
        0 => smash::app::ItemKind(*ITEM_KIND_KILLSWORD),
        1 => smash::app::ItemKind(*ITEM_KIND_HOMERUNBAT),
        2 => smash::app::ItemKind(*ITEM_KIND_FIREFLOWER),
        3 => smash::app::ItemKind(*ITEM_KIND_REVENGESHOOTER),
        4 => smash::app::ItemKind(*ITEM_KIND_STAFF),
        5 => smash::app::ItemKind(*ITEM_KIND_STARROD),
        6 => smash::app::ItemKind(*ITEM_KIND_STEELDIVER),
        7 => smash::app::ItemKind(*ITEM_KIND_RAYGUN),
        8 => smash::app::ItemKind(*ITEM_KIND_DEATHSCYTHE),
        9 => smash::app::ItemKind(*ITEM_KIND_RIPSTICK),
        _ => smash::app::ItemKind(*ITEM_KIND_BEAMSWORD)
    };
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    ItemModule::remove_item(boma, 0);
    ItemModule::have_item(boma, item, 0, 0, false, false);
    macros::STOP_SE(fighter, Hash40::new("se_item_item_get"));
	//EffectModule::kill_kind(boma, smash::phx::Hash40::new("sys_item_get"), false, false);
}

#[skyline::hook(replace = ItemModule::drop_item)]
pub unsafe fn drop_item_hook(boma: &mut smash::app::BattleObjectModuleAccessor, arg1: f32, arg2: f32, arg3: i32) -> u64 {
        if is_gamemode("itemduel".to_string()) {
            return 0;
        }
        original!()(boma, arg1, arg2, arg3)
}
//

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, itemduel)
	.install();
	skyline::install_hooks!(
		drop_item_hook
    );
}