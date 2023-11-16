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
pub fn install() {
    smashline::install_acmd_scripts!(
		ink_bomb
    );
}
#[acmd_script( agent = "inkling_splashbomb", script = "game_explode", category = ACMD_GAME, low_priority )]
unsafe fn ink_bomb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 120, 47, 0, 60, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_BOMB);
        AttackModule::set_ink_value(agent.module_accessor, 110, 200.0);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 12.0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 14.0);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}