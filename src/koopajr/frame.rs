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
use crate::koopajr::*;
use super::*;

pub fn install() {
	Agent::new("koopajr_remainclown")
    .set_costume(get_marked_costumes("koopajr","koopajr"))
    .on_line(Main, clowncar_frame)
    .install();
}
unsafe extern "C" fn clowncar_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = StatusModule::status_kind(weapon.module_accessor);
        if status_kind == *WEAPON_KOOPAJR_REMAINCLOWN_STATUS_KIND_FALL && AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
            StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_KOOPAJR_REMAINCLOWN_STATUS_KIND_BURST, false);
        }
    }
}