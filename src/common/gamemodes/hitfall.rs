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

unsafe extern "C" fn hitfall(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("hitfall".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = StatusModule::status_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && is_hitlag(boma) {
			let cat2 = ControlModule::get_command_flag_cat(boma, 1);
			if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && ControlModule::get_stick_y(boma) < -0.66 {
				WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
			}
        }
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, hitfall)
	.install();
}