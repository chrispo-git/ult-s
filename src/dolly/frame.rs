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
	smashline::install_agent_frame_callbacks!(terry_frame);
}
#[fighter_frame_callback]
pub fn terry_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let mut stick_x = ControlModule::get_stick_x(boma) ;
		let motion_kind = MotionModule::motion_kind(boma);
		stick_x *= PostureModule::lr(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
        
		if fighter_kind == *FIGHTER_KIND_DOLLY && is_default(boma){
			if [*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B].contains(&status_kind) && [*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND].contains(&status_kind) == false {
				PostureModule::reverse_lr(boma);
				PostureModule::update_rot_y_lr(boma);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
			};
			if [hash40("special_f_start"), hash40("special_air_f_start")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) == 1.0 && stick_x < -0.1 {
				PostureModule::reverse_lr(boma);
				PostureModule::update_rot_y_lr(boma);
			};
			if [hash40("attack_hi4"), hash40("attack_lw4")].contains(&motion_kind) && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)){
                //Rising Tackle CI
                if (ControlModule::get_command_flag_cat(boma, 3) & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND) != 0{ 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, true);
                };
                //Rising Tackle
                if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0{ 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
                };
                //Crack Shoot CI
                if (ControlModule::get_command_flag_cat(boma, 3) & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND) != 0{ 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, true);
                };
                //Burning Knuckle CI
                if (ControlModule::get_command_flag_cat(boma, 3) & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) != 0{ 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, true);
                };
                //Side Special
                if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0{ 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
                };
                //Power Dunk CI
                if (ControlModule::get_command_flag_cat(boma, 3) & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND) != 0{ 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, true);
                };
                //Power Dunk
                if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0{ 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                };
                //Power Wave
                if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{ 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
                };
				if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL){
				//Power Geyser
				if (ControlModule::get_command_flag_cat(boma, 3) & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND) != 0{ 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, true);
                };
				//Buster Wolf
				if (ControlModule::get_command_flag_cat(boma, 3) & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND) != 0{ 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, true);
                };
            };
		};
		};
	};
}