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

pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind : i32, motion_kind : u64, entry_id : usize) {

    airdash::opff(fighter, status_kind, entry_id);
    attacks::critical_opff(fighter, entry_id);
    fgmode::opff(fighter, status_kind, motion_kind, entry_id);
    hitfall::opff(fighter, status_kind);
    hitstunchange::opff(fighter, status_kind, entry_id);
    itemduel::opff(fighter, status_kind, motion_kind, entry_id);
    parry::opff(fighter, status_kind, motion_kind, entry_id);
    rivals::opff(fighter, status_kind, entry_id);
    turbo::opff(fighter, status_kind);
    sixtyfour::opff(fighter, status_kind);
    smym::opff(fighter, status_kind, entry_id);
    vampire::opff(fighter, status_kind, entry_id);
}
pub fn install() {
	attacks::install();
    superboss::install();
    itemduel::install();
    parry::install();
}