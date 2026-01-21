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

//Dash Attack Cancel Smashes
unsafe fn dacus(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize) {
	let fighter_kind = smash::app::utility::get_kind(boma(fighter));
	if status_kind != *FIGHTER_STATUS_KIND_ATTACK_DASH ||
	AttackModule::is_infliction_status(boma(fighter), *COLLISION_KIND_MASK_SHIELD){ 
		return;
	}
	let f6 = [
		*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_WARIO, *FIGHTER_KIND_EDGE,
		*FIGHTER_KIND_LUCAS, *FIGHTER_KIND_PACMAN, *FIGHTER_KIND_MIIFIGHTER
	];
	let f10 = [
		*FIGHTER_KIND_CHROM, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_TRAIL, *FIGHTER_KIND_REFLET,
		*FIGHTER_KIND_FOX, *FIGHTER_KIND_ELIGHT, *FIGHTER_KIND_PITB, *FIGHTER_KIND_MARIO,
		*FIGHTER_KIND_MARIOD, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_JACK, *FIGHTER_KIND_SHEIK,
	];
	let f11 = [
		*FIGHTER_KIND_MIIGUNNER
	];
	let f12 = [
		*FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_KAMUI, *FIGHTER_KIND_DIDDY, 
		*FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_DUCKHUNT, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_YOSHI,
		*FIGHTER_KIND_SIMON, *FIGHTER_KIND_ROY, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_ROCKMAN
	];
	let f14 = [
		*FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_NESS, *FIGHTER_KIND_DEDEDE, 
		*FIGHTER_KIND_BRAVE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_LUCINA,
		*FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_SHIZUE
	];
	let f16 = [
		*FIGHTER_KIND_IKE, *FIGHTER_KIND_SHULK, *FIGHTER_KIND_EFLAME, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_LINK
	];
	let frame_max = match fighter_kind {
		n if f6.contains(&n) => 6,
		n if f10.contains(&n) => 10,
		n if f11.contains(&n) => 11,
		n if f12.contains(&n) => 12,
		n if f14.contains(&n) => 14,
		n if f16.contains(&n) => 16,
		_ => 8,
	};
	if motion_duration(boma(fighter)) <= frame_max {
		if (ControlModule::get_command_flag_cat(boma(fighter), 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
			JC_GRAB_LOCKOUT[ENTRY_ID] = MAX_LOCKOUT;
			StatusModule::change_status_request_from_script(boma(fighter), *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
		} else if (ControlModule::get_command_flag_cat(boma(fighter), 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
			JC_GRAB_LOCKOUT[ENTRY_ID] = MAX_LOCKOUT;
			StatusModule::change_status_request_from_script(boma(fighter), *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
		};
	}

}
pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind: i32, ENTRY_ID : usize) {
	if !is_mechanics_enabled() {
		return;
	}
	dacus(fighter, status_kind, ENTRY_ID);
}