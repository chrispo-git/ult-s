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
static mut PARRY_DUATION : [i32; 8] = [0; 8];
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

unsafe extern "C" fn parry(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("parry".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = StatusModule::status_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&status_kind) {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, false);
        }
        if [*FIGHTER_STATUS_KIND_GUARD_OFF].contains(&status_kind) {
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);

            if MotionModule::frame(boma) > 4.0 {
                WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
            }

            if MotionModule::frame(boma) > 6.0 {
                macros::COL_NORMAL(fighter);
                WorkModule::set_float(boma, 1.0, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_REBOUND, true);
            }
            if (MotionModule::frame(boma) as i32) == 1 {
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_snow"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_sand"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_step_sand"));

			    macros::FLASH(fighter, 0.92, 0.99, 1, 0.5);
                EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_just_shield_hit"), smash::phx::Hash40::new("hip"), &NONE, &NONE, 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
            }
        }
        if PARRY_DUATION[ENTRY_ID] > 0 {
            PARRY_DUATION[ENTRY_ID] -= 1;
        }
        if [hash40("just_shield"), hash40("just_shield_off")].contains(&MotionModule::motion_kind(boma)) {
            PARRY_DUATION[ENTRY_ID] = 10;
            println!("Parries");
        }
        if PARRY_DUATION[ENTRY_ID] == 1 && !(*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind){
			StopModule::end_stop(boma);
            println!("End Stun Early");
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
    };
}
//Parry Reflects
#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    if is_gamemode("parry".to_string()) {return true; }
    original!()(module_accessor)
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, parry)
	.install();
	skyline::install_hooks!(
		is_valid_just_shield_reflector
    );
}