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

//Jab Cancel
pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind : i32, motion_kind : u64, ENTRY_ID : usize) {
    unsafe { 
		let fighter_kind = smash::app::utility::get_kind(boma(fighter));
		let stick_x = ControlModule::get_stick_x(boma(fighter));
		let stick_y = ControlModule::get_stick_y(boma(fighter));
		let frame = MotionModule::frame(boma(fighter));
		if !is_mechanics_enabled() && !is_gamemode("fgmode".to_string()) && !is_gamemode("rivals".to_string()) {
			if HAS_ENABLE_100_ON[ENTRY_ID] {
					WorkModule::set_flag(boma(fighter), true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
			};
				if HAS_ENABLE_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma(fighter), true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
					if fighter_kind == *FIGHTER_KIND_DEMON {
						WorkModule::set_flag(boma(fighter), true, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
					};
				};
				if HAS_ENABLE_NO_HIT_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma(fighter), true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
			return;
		}
		//Speeds up jab 2s and 3s for certain chars
		if [*FIGHTER_KIND_DOLLY, *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_KROOL].contains(&fighter_kind) {
			if [hash40("attack_12")].contains(&motion_kind) && frame < 2.0 {
					MotionModule::change_motion(boma(fighter), Hash40::new("attack_12"), 2.0, 1.0, false, 0.0, false, false);
			};
			if [hash40("attack_13")].contains(&motion_kind) && frame < 2.0 {
					MotionModule::change_motion(boma(fighter), Hash40::new("attack_13"), 2.0, 1.0, false, 0.0, false, false);
			};
		};
		//Prevents jab overriding
		let cat1 = ControlModule::get_command_flag_cat(boma(fighter), 0);
		if [*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) {
			if ((stick_x <= 0.2 && stick_x >= -0.2) && (stick_y <= 0.2 && stick_y >= -0.2)) && ControlModule::check_button_on(boma(fighter), *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(boma(fighter), *CONTROL_PAD_BUTTON_CATCH) && 
			(
				(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) == 0 &&
				(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) == 0 &&
				(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0 &&
				(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) == 0 &&
				(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) == 0 &&
				(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 &&
				(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) == 0 &&
				ControlModule::check_button_off(boma(fighter), *CONTROL_PAD_BUTTON_JUMP)
			){
				CAN_JAB[ENTRY_ID] = 0;
				CAN_RAPID_JAB[ENTRY_ID] = 0;
				if HAS_ENABLE_100_ON[ENTRY_ID] {
					WorkModule::set_flag(boma(fighter), true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
				};
				if HAS_ENABLE_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma(fighter), true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
					if fighter_kind == *FIGHTER_KIND_DEMON {
						WorkModule::set_flag(boma(fighter), true, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
					};
				};
				if HAS_ENABLE_NO_HIT_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma(fighter), true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
			} else {
				CAN_JAB[ENTRY_ID] = 1;
				CAN_RAPID_JAB[ENTRY_ID] = 1;
				if HAS_ENABLE_100_ON[ENTRY_ID] {
					WorkModule::set_flag(boma(fighter), false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
				};
				if HAS_ENABLE_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma(fighter), false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
				if HAS_ENABLE_NO_HIT_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma(fighter), false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
			};
		} else {
			CAN_JAB[ENTRY_ID] = 0;
			CAN_RAPID_JAB[ENTRY_ID] = 0;
		};
		if [*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) && !is_hitlag(boma(fighter)) && (HAS_ENABLE_100_ON[ENTRY_ID] || HAS_ENABLE_COMBO_ON[ENTRY_ID]){
				if  (AttackModule::is_infliction_status(boma(fighter), *COLLISION_KIND_MASK_HIT) || (fighter_kind == *FIGHTER_KIND_SHEIK && AttackModule::is_infliction_status(boma(fighter), *COLLISION_KIND_MASK_ALL))) {
						if (ControlModule::get_command_flag_cat(boma(fighter), 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 || 
						(ControlModule::get_command_flag_cat(boma(fighter), 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || 
						(ControlModule::get_command_flag_cat(boma(fighter), 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0
						{
							println!("No Cancel");
						} else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
							StatusModule::change_status_request_from_script(boma(fighter), *FIGHTER_STATUS_KIND_ATTACK_S3, true);
						} else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
							StatusModule::change_status_request_from_script(boma(fighter), *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
						} else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
							StatusModule::change_status_request_from_script(boma(fighter), *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
						};
				};
		};
    };
}

