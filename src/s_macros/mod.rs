
// Macros! Joy!
#[macro_export]
macro_rules! fastfall {
    ($fighter:ident) => {
		unsafe {
			let boma = smash::app::sv_system::battle_object_module_accessor($fighter.lua_state_agent);
			let stick_y = smash::app::lua_bind::ControlModule::get_stick_y(boma);    
			let speed_y = smash::app::lua_bind::KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			if smash::app::lua_bind::StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
				let cat2 = smash::app::lua_bind::ControlModule::get_command_flag_cat(boma, 1);
				if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && speed_y <= 0.0 {
					smash::app::lua_bind::WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
				}
			};
		}
    };
}
#[macro_export]
macro_rules! fastfall_land_cancel {
    ($fighter:ident, $landing_lag:expr) => {
		unsafe {
			let boma = smash::app::sv_system::battle_object_module_accessor($fighter.lua_state_agent);    
			if smash::app::lua_bind::StatusModule::is_situation_changed(boma) {
				smash::app::lua_bind::StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				smash::app::lua_bind::WorkModule::set_float(boma, $landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
			};
            crate::fastfall!($fighter);
		}
    };
}