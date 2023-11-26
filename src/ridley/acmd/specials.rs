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
use super::*;

#[acmd_script(
    agent = "ridley",
    script =  "game_specialairsfalljump",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ridley_sideb_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 7.0, /*Angle*/ 88, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 88, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_THROW);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
		}
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			CancelModule::enable_cancel(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 38.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_JUMP);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
}		
#[acmd_script(
    agent = "ridley",
    script =  "effect_specialairsfall",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ridley_sideb_fall_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	

pub fn install() {
    smashline::install_acmd_scripts!(
		ridley_sideb_end,
        ridley_sideb_fall_eff
    );
}