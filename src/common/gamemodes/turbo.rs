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

unsafe extern "C" fn turbo(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("turbo".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = StatusModule::status_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
        if [*FIGHTER_STATUS_KIND_CATCH_ATTACK].contains(&status_kind) {
            return;
        }
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            CancelModule::enable_cancel(boma);
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.sub_wait_ground_check_common(false.into());
            } else {
                fighter.sub_air_check_fall_common();
            }
        }
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, turbo)
	.install();
}