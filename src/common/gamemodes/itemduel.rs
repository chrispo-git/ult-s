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

static mut ITEM_OPTION : [i32; 8] = [-1; 8];


unsafe extern "C" fn itemduel(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("itemduel".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
        
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD);
        if smash::app::sv_information::is_ready_go() {
            if ![*FIGHTER_STATUS_KIND_ITEM_SWING, *FIGHTER_STATUS_KIND_ITEM_SWING_S3, *FIGHTER_STATUS_KIND_ITEM_SWING_S4,
                *FIGHTER_STATUS_KIND_ITEM_SHOOT_AIR, *FIGHTER_STATUS_KIND_ITEM_SHOOT_FLY, *FIGHTER_STATUS_KIND_ITEM_SHOOT_JUMP,
                *FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT, *FIGHTER_STATUS_KIND_ITEM_SWING_DASH, *FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_B,
                *FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_F, *FIGHTER_STATUS_KIND_ITEM_SHOOT_LANDING, *FIGHTER_STATUS_KIND_ITEM_SWING_S4_HOLD,
                *FIGHTER_STATUS_KIND_ITEM_SWING_S4_START, *FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_BRAKE_B, *FIGHTER_STATUS_KIND_ITEM_WALK_BRAKE_F,
                *FIGHTER_STATUS_KIND_ITEM_STARRING
            ].contains(&status_kind) {
                setItem(fighter, ITEM_OPTION[ENTRY_ID]);
            }
        }
        if !smash::app::sv_information::is_ready_go() || [*FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
            ITEM_OPTION[ENTRY_ID] = smash::app::sv_math::rand(hash40("fighter"), 12);
        }
    };
}

unsafe fn setItem(fighter : &mut L2CFighterCommon, val : i32) -> () {
    let item = match val {
        0 => smash::app::ItemKind(*ITEM_KIND_BEAMSWORD),
        1 => smash::app::ItemKind(*ITEM_KIND_HOMERUNBAT),
        2 => smash::app::ItemKind(*ITEM_KIND_FIREFLOWER),
        3 => smash::app::ItemKind(*ITEM_KIND_REVENGESHOOTER),
        4 => smash::app::ItemKind(*ITEM_KIND_STAFF),
        5 => smash::app::ItemKind(*ITEM_KIND_STARROD),
        6 => smash::app::ItemKind(*ITEM_KIND_STEELDIVER),
        7 => smash::app::ItemKind(*ITEM_KIND_RAYGUN),
        8 => smash::app::ItemKind(*ITEM_KIND_DEATHSCYTHE),
        9 => smash::app::ItemKind(*ITEM_KIND_FIREBAR),
        10 => smash::app::ItemKind(*ITEM_KIND_RIPSTICK),
        _ => smash::app::ItemKind(*ITEM_KIND_KILLSWORD)
    }
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    ItemModule::remove_item(boma, 0);
    ItemModule::have_item(boma, item, 0, 0, false, false);
    macros::STOP_SE(fighter, Hash40::new("se_item_item_get"));
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, itemduel)
	.install();
}