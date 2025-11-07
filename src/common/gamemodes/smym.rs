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

enum Boon {
    NONE,
    SPEED_UP,
    DAMAGE_UP,
    JUMP_UP,
    ARMOR,
    SLOW_DOWN,
    POISON,
    TRIPLE_JUMP
}

static mut CURR_BOON : [Boon; 8] = [const { Boon::NONE }; 8];
static mut BOON_DURATION : [i32; 8] = [0; 8];
const BOON_MAX : i32 = 660;
unsafe extern "C" fn smym(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("smym".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(boma);
        let frame = MotionModule::frame(boma) as i32;

        if !smash::app::sv_information::is_ready_go() {
            CURR_BOON[ENTRY_ID] = Boon::NONE;
            BOON_DURATION[ENTRY_ID] = 0;
        }
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL && frame == 2 {
            let rand_num = smash::app::sv_math::rand(hash40("fighter"), 7);
            CURR_BOON[ENTRY_ID] = match rand_num {
                0 => Boon::SPEED_UP,
                1 => Boon::DAMAGE_UP,
                2 => Boon::JUMP_UP,
                3 => Boon::ARMOR,
                4 => Boon::SLOW_DOWN,
                5 => Boon::POISON,
                6 => Boon::TRIPLE_JUMP,
                _ => Boon::NONE
            };
            BOON_DURATION[ENTRY_ID] = BOON_MAX;
        }

        if BOON_DURATION[ENTRY_ID] > 0 {
            BOON_DURATION[ENTRY_ID] -= 1;
        }
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, smym)
	.install();
}