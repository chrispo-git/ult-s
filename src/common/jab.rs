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
unsafe extern "C" fn jabcancel(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let stick_x = ControlModule::get_stick_x(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		let frame = MotionModule::frame(boma);
		//Speeds up jab 2s and 3s for certain chars
		if [*FIGHTER_KIND_DOLLY, *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_KROOL].contains(&fighter_kind) {
			if [hash40("attack_12")].contains(&motion_kind) && frame < 2.0 {
					MotionModule::change_motion(boma, Hash40::new("attack_12"), 2.0, 1.0, false, 0.0, false, false);
			};
			if [hash40("attack_13")].contains(&motion_kind) && frame < 2.0 {
					MotionModule::change_motion(boma, Hash40::new("attack_13"), 2.0, 1.0, false, 0.0, false, false);
			};
		};
		//Prevents jab overriding
		if [*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) {
			if ((stick_x <= 0.2 && stick_x >= -0.2) && (stick_y <= 0.2 && stick_y >= -0.2)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH) && 
			(
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) == 0 &&
				ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP)
			){
				CAN_JAB[ENTRY_ID] = 0;
				CAN_RAPID_JAB[ENTRY_ID] = 0;
				if HAS_ENABLE_100_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
				};
				if HAS_ENABLE_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
					if fighter_kind == *FIGHTER_KIND_DEMON {
						WorkModule::set_flag(boma, true, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
					};
				};
				if HAS_ENABLE_NO_HIT_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
			} else {
				CAN_JAB[ENTRY_ID] = 1;
				CAN_RAPID_JAB[ENTRY_ID] = 1;
				if HAS_ENABLE_100_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
				};
				if HAS_ENABLE_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
				if HAS_ENABLE_NO_HIT_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
			};
		} else {
			CAN_JAB[ENTRY_ID] = 0;
			CAN_RAPID_JAB[ENTRY_ID] = 0;
		};
		if [*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) && !is_hitlag(boma) && (HAS_ENABLE_100_ON[ENTRY_ID] || HAS_ENABLE_COMBO_ON[ENTRY_ID]){
				if  (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || (fighter_kind == *FIGHTER_KIND_SHEIK && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL))) {
						if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
						};
				};
		};
    };
}


pub fn install() {
    Agent::new("fighter")
	.on_line(Main, jabcancel)
	.install();
}