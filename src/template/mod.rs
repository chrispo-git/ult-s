use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use crate::util::*;
static mut STATIC_MUT : [i32; 8] = [6; 8];

// A Once-Per-Fighter-Frame that only applies to Mario. Neat!
#[fighter_frame( agent = FIGHTER_KIND_MARIO )]
fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        println!("It'sa me, Mario, wahoooooooo!");
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
    }
}

// Uses the acmd! macro, if you're porting directly from Rubendal's data viewer.
#[acmd_script( agent = "mario", scripts = ["game_appealhil", "game_appealhir"], category = ACMD_GAME, low_priority )]
unsafe fn mario_utaunt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			rust {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				if IS_HATLESS[ENTRY_ID] == true {
					IS_HATLESS[ENTRY_ID] = false;
					ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, false, 0);
					ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("mario_nohat"),false);
				} else {
					IS_HATLESS[ENTRY_ID] = true;
				};
			}
		}
    });
}
#[acmd_script( agent = "mario", scripts = ["game_appealsl", "game_appealsr"], category = ACMD_GAME, low_priority )]
unsafe fn mario_staunt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			rust {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				if IS_HATLESS[ENTRY_ID] == true {
					ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, false, 0);
				};
			}
		}
    });
}


pub fn install() {
    smashline::install_agent_frames!(
        mario_frame
    );
	smashline::install_acmd_scripts!(mario_utaunt, mario_staunt);
}
