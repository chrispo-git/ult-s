use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{ L2CValue, L2CAgent };
use smash::phx::Vector2f;
use crate::util::*;

#[derive(Default, Clone, Copy)]
pub struct GamemodeFGModeState {
	pub can_cancel_timer : i32,
	pub can_cancel : bool,
}
const WINDOW: i32 = 20;

pub unsafe fn opff(fighter: &mut L2CFighterCommon, status_kind: i32, motion_kind: u64, ENTRY_ID : usize) {
	if !is_gamemode("fgmode".to_string()) {
		return;
	}
	let lr = PostureModule::lr(fighter.module_accessor);
	let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;
	let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
	let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
	let frame = MotionModule::frame(fighter.module_accessor);
	let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

	if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
		crate::with_state!(ENTRY_ID, GamemodeFGModeState, state, {
			state.can_cancel = false;
		});
	}
	if crate::get_state!(ENTRY_ID, GamemodeFGModeState).can_cancel_timer > 0 {
		if WorkModule::get_int(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 {
			crate::with_state!(ENTRY_ID, GamemodeFGModeState, state, {
				state.can_cancel_timer -= 1;
			});
		}
	} else {
		crate::with_state!(ENTRY_ID, GamemodeFGModeState, state, {
			state.can_cancel = false;
		});
	}
	if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
		crate::with_state!(ENTRY_ID, GamemodeFGModeState, state, {
			state.can_cancel = true;
			state.can_cancel_timer = WINDOW;
		});
	}
	if crate::get_state!(ENTRY_ID, GamemodeFGModeState).can_cancel && !StopModule::is_stop(fighter.module_accessor) &&
		WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 {
		// Gatlings
		if 	[*FIGHTER_STATUS_KIND_ATTACK_S3,*FIGHTER_STATUS_KIND_ATTACK_HI3,
			*FIGHTER_STATUS_KIND_ATTACK_LW3,].contains(&status_kind) {
			match cat1 {
				n if (n & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 => {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true)
					},
				n if (n & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 => {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true)
					},
				n if (n & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 => {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true)
					},
				_ => 0,
			};
		}
		// Special Cancels
		if 	[*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, 
			*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK,
			*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3,
			*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
			match cat1 {
				n if (n & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 => {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true)
					},
				n if (n & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 => {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true)
					},
				n if (n & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 => {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true)
					},
				n if (n & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 => {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true)
					},
				_ => 0,
			};
		}
		// Usmash jump cancel
		if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
			if	check_jump(boma(fighter)) ||
				ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP){
				ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP);
				ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP_BUTTON);
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT,false);
			}
		}
		if [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind)
		{	
			let isSide = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0;
			let isUp = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0;
			let isDown = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0;
			let nair = hash40("attack_air_n");
			let uair = hash40("attack_air_hi");
			let fair = hash40("attack_air_f");
			let bair = hash40("attack_air_b");
			let doCancel = match motion_kind {
				nair if (isSide || isUp || isDown) => true,
				uair if (isSide || isDown) => true,
				bair if (isDown) => true,
				fair if (isDown) => true,
				_ => false,
			};
			if doCancel {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
			}
		}
	}
}
fn check_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
            return true;
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
            if ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_stick_y(boma) >= 0.7 {
                return true;
            }
        }
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
            return true;
        }
        return false;
    }
}
