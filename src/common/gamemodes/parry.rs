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

unsafe extern "C" fn parry(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("parry".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = StatusModule::status_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
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
            if MotionModule::frame(boma) > 6.0 {
                MotionModule::set_rate(boma, 2.5);
            }
        }
        let just_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        if just_frame > 0 && WorkModule::is_flag(boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
            if just_frame == 2 {
            	shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 0.0, /*X2*/ 0.0, /*Y2*/ 6.0, /*Z2*/ 0.0, /*Power*/ 1.25, /*Speed*/ 1.0, /*Max Damage*/ 999.0, false, /*Lifetime*/ 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
			}
        }
        if MotionModule::motion_kind(boma) == hash40("just_shield_off") {
            if (StopModule::get_hit_stop_real_frame(fighter.module_accessor) as i32) < 12 {
				StopModule::end_stop(boma);
            }
        }
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, parry)
	.install();
}