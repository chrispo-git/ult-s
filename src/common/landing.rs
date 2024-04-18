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

//Landing Lag Platform Cancel
unsafe extern "C" fn llpc(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let sticky = ControlModule::get_stick_y(boma);	
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let frame = MotionModule::frame(boma);
		let situation_kind = StatusModule::situation_kind(boma);
        if ([hash40("landing_air_f"), hash40("landing_air_b"), hash40("landing_air_hi"), hash40("landing_air_n")].contains(&MotionModule::motion_kind(boma))) {
			if GroundModule::is_passable_ground(fighter.module_accessor) && frame/cancel_frame >= (1.0/6.0){
                if sticky <= -0.6875 && ((ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_flick_y(boma) < 20)) {
					if (
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0 &&
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 &&
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) == 0 &&
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) == 0 &&
						(ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP))
					) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
					};
                };
            }
		};
		if situation_kind == *SITUATION_KIND_GROUND && ((cancel_frame != 0.0 && frame >= cancel_frame)|| CancelModule::is_enable_cancel(boma)) {
			if GroundModule::is_passable_ground(fighter.module_accessor) && sticky <= -0.6875 && (ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_flick_y(boma) < 20) {
				if (
					(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0 &&
					(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 &&
					(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) == 0 &&
					(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) == 0 &&
					(ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP))
				) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
				};
			}
		}
    };
}	
unsafe extern "C" fn shielddrop(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let sticky = ControlModule::get_stick_y(boma);	
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let frame = MotionModule::frame(boma);
		let situation_kind = StatusModule::situation_kind(boma);
        if [*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD].contains(&status_kind) &&  sticky <= -0.6875  && GroundModule::is_passable_ground(fighter.module_accessor){
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
		};
    };
}	

//Edge Cancel List
pub(crate) fn is_edge_cancel(fighter_kind : i32, status_kind : i32) -> bool {
	let edge_cancel = [
		[*FIGHTER_KIND_LUCARIO, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_LUCARIO, *FIGHTER_STATUS_KIND_SPECIAL_LW],
		[*FIGHTER_KIND_DIDDY, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_DONKEY, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_BUDDY, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_LITTLEMAC, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_KAMUI, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_PURIN, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_CAPTAIN, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_RIDLEY, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL],
		[*FIGHTER_KIND_RIDLEY, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_RICHTER, *FIGHTER_STATUS_KIND_ATTACK_LW3],
		[*FIGHTER_KIND_SAMUS, *FIGHTER_STATUS_KIND_ATTACK_LW3],
		[*FIGHTER_KIND_SONIC, *FIGHTER_STATUS_KIND_SPECIAL_S],
		[*FIGHTER_KIND_ROY, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_YOUNGLINK, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_CAPTAIN, *FIGHTER_STATUS_KIND_SPECIAL_LW],
		[*FIGHTER_KIND_EDGE, *FIGHTER_STATUS_KIND_ATTACK_LW3],
		[*FIGHTER_KIND_MIIGUNNER, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_FALCO, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_PIKMIN, *FIGHTER_STATUS_KIND_RUN_BRAKE],
		[*FIGHTER_KIND_KIRBY, *FIGHTER_KIRBY_STATUS_KIND_PIKMIN_SPECIAL_N],
		[*FIGHTER_KIND_MIIFIGHTER, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING]
	];
	for i in &edge_cancel {
		if fighter_kind == i[0] && status_kind == i[1] {
			return true;
		};
	};
	return false;
}

//Edge Cancelling Part A
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::init_settings)]
unsafe fn init_settings_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, situation_kind: i32, arg3: i32, arg4: u64, ground_cliff_check_kind: u64, arg6: bool, arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u64 {
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if smash::app::utility::get_category(module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
        return original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
    }
    if is_edge_cancel(fighter_kind, status_kind) && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    } 
    else if [*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else {
        original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
}

//Edge Cancelling Part B
#[skyline::hook(replace = smash::app::lua_bind::GroundModule::correct)]
unsafe fn correct_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, ground_correct_kind: u32) -> u64 {
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if smash::app::utility::get_category(module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
        original!()(module_accessor, ground_correct_kind);
    }
    if [*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
        original!()(module_accessor, 1 as u32)
    }
    else if is_edge_cancel(fighter_kind, status_kind) {
        original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32)
    }
    else {
        original!()(module_accessor, ground_correct_kind)
    }
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, llpc)
	.on_line(Main, shielddrop)
	.install();
	skyline::install_hooks!(
        init_settings_replace,
        correct_replace
    );
}