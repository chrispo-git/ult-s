mod turbo;
mod airdash;
mod attacks;
mod vampire;
mod superboss;
mod itemduel;
mod hitfall;
mod parry;
mod hitstunchange;
mod sixtyfour;
mod fgmode;
mod rivals;
mod smym;

use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use crate::util::*;

unsafe extern "C" fn gamemodes(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    airdash::opff(fighter, status_kind, entry_id);
    attacks::critical_opff(fighter, entry_id);
    fgmode::opff(fighter, status_kind, motion_kind, entry_id);
    hitfall::opff(fighter, status_kind);
    hitstunchange::opff(fighter, status_kind, entry_id);
    itemduel::opff(fighter, status_kind, motion_kind, entry_id);
    parry::opff(fighter, status_kind, motion_kind, entry_id);
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, gamemodes)
	.install();

	turbo::install();
	attacks::install();
    vampire::install();
    superboss::install();
    itemduel::install();
    rivals::install();
    parry::install();
    sixtyfour::install();
    smym::install();
}