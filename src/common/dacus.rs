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
#[fighter_frame_callback]
pub fn dacus(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let f6 = [
			*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_WARIO, *FIGHTER_KIND_EDGE,
			*FIGHTER_KIND_LUCAS, *FIGHTER_KIND_PACMAN, *FIGHTER_KIND_MIIFIGHTER
		];
		let f8 = [*FIGHTER_KIND_SZEROSUIT
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
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) == false && MotionModule::motion_kind(boma) == hash40("attack_dash"){
				if ((f16.contains(&fighter_kind) && motion_duration(boma) <= 16) || 
				(f14.contains(&fighter_kind) && motion_duration(boma) <= 14) || 
				(f12.contains(&fighter_kind) && motion_duration(boma) <= 12) ||
				(f11.contains(&fighter_kind) && motion_duration(boma) <= 11) ||
				(f10.contains(&fighter_kind) && motion_duration(boma) <= 10) ||
				(f8.contains(&fighter_kind) && motion_duration(boma) <= 8) ||
				(f6.contains(&fighter_kind) && motion_duration(boma) <= 6) ||
				(f6.contains(&fighter_kind) == false && motion_duration(boma) <= 8)){
					if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
					};
				};
        };
    };
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(
		dacus
	);
}