use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;use smash::app::lua_bind::*;

#[acmd_script(
    agent = "popo",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME)]
unsafe fn ics_shot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=6)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE)
		}
		frame(Frame=18)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ArticleModule::shoot_exist(FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
		}
    });
}
#[acmd_script(
    agent = "popo",
    scripts =  ["game_specialn_nana", "game_specialairn_nana"],
    category = ACMD_GAME)]
unsafe fn ics_shot2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=6)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE)
		}
		frame(Frame=18)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ArticleModule::shoot_exist(FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
		}
		frame(Frame=50)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_ENABLE_COUPLE)
		}
    });
}
pub fn install() {
    smashline::install_acmd_scripts!(
		ics_shot,
		ics_shot2
    );
}
