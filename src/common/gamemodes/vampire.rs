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

static mut RESET : [bool; 8] = [true; 8];
static mut TO_HEAL : [f32; 8] = [0.0; 8];
static mut VAMPIRE_COUNTER : [i32; 8] = [0; 8];
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

const HEAL_FACTOR : f32 = 0.75;
const COUNTER_MAX : i32 = 90;
const SELF_DAMAGE : f32 = 2.5;
unsafe extern "C" fn vampire(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("vampire".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(boma);
        let attacker_entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID) - 1;
        let damage_dealt = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
		if attacker_entry_id == -1 {
            RESET[ENTRY_ID] = true;
        } else if RESET[ENTRY_ID] {
            TO_HEAL[attacker_entry_id as usize] += damage_dealt * HEAL_FACTOR;
            RESET[ENTRY_ID] = false;
        }
        //println!("{}, {}", attacker_entry_id, damage_dealt);
        if TO_HEAL[ENTRY_ID] > 0.0 {
			DamageModule::add_damage(boma, -TO_HEAL[ENTRY_ID], 0);
            TO_HEAL[ENTRY_ID] = 0.0;
        }
        if VAMPIRE_COUNTER[ENTRY_ID] <= 0 {
            VAMPIRE_COUNTER[ENTRY_ID] = COUNTER_MAX;
			DamageModule::add_damage(boma, SELF_DAMAGE, 0);
			EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_curse"), smash::phx::Hash40::new("hip"), &NONE, &NONE, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
			EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_curse"), smash::phx::Hash40::new("haver"), &NONE, &NONE, 0.35, true, 0, 0, 0, 0, 0, true, true) as u32;
			EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_curse"), smash::phx::Hash40::new("havel"), &NONE, &NONE, 0.35, true, 0, 0, 0, 0, 0, true, true) as u32;
			EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_curse"), smash::phx::Hash40::new("footr"), &NONE, &NONE, 0.35, true, 0, 0, 0, 0, 0, true, true) as u32;
			EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_curse"), smash::phx::Hash40::new("footl"), &NONE, &NONE, 0.35, true, 0, 0, 0, 0, 0, true, true) as u32;
        } else {
            VAMPIRE_COUNTER[ENTRY_ID] -= 1;
        }
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, vampire)
	.install();
}