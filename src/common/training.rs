use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use crate::common::*;


#[fighter_frame_callback]
pub fn training(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32; //Cancel frame
		let frame = MotionModule::frame(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        if !smash::app::smashball::is_training_mode() {
            IS_GLOW = false;
        } else {
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                if IS_GLOW {
                    IS_GLOW = false;
                } else {
                    IS_GLOW = true;
                };
            };
        };
        if IS_GLOW {
                macros::COL_NORMAL(fighter);
                //Resets glow so it doesnt stack after each frame
                let banned_check = ![
                    *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_DASH, 
                    *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE, *FIGHTER_STATUS_KIND_RUN_BRAKE, 
                    *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
                    *FIGHTER_STATUS_KIND_WAIT
                ].contains(&status_kind);    //These statuses do not need to glow for endlag
                if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0 && (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) {
                    macros::FLASH(fighter, 2.52, 1.86, 0.03, 0.7);
                }else if ((cancel_frame > 0.0 && frame >= cancel_frame) || (CancelModule::is_enable_cancel(boma))) && !(*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) && banned_check {
                     macros::FLASH(fighter, 1.59, 0.0, 2.22, 0.7);
                } else if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF && frame <=  4.0 {
                    macros::FLASH(fighter, 0.31, 2.01, 2.07, 0.7);
                };
                if (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) {
                   if !(WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0) {
                        if situation_kind == *SITUATION_KIND_GROUND {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD, true);
                        } else {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                        }
                   }
                }
        };
    };
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(
		training
	);
}
