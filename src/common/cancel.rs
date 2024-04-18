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


//Jump Cancel List
//0 for hit_condition means it can always be jump cancelled
//Otherwise, set hit_condition to a value such as *COLLISION_KIND_MASK_HIT
//-1 for jc_start/jc_end means it will always be jump cancellable at any point
pub(crate) fn is_jc(boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind : i32, status_kind : i32, frame : f32) -> bool {
	unsafe {
		//[fighter_kind, status_kind, hit_condition, jc_start, jc_end]
		let jump_cancel = [
			[*FIGHTER_KIND_KAMUI, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD, 0, -1, -1],
			[*FIGHTER_KIND_FALCO, *FIGHTER_STATUS_KIND_SPECIAL_LW, 0, 4, 32],
			[*FIGHTER_KIND_WOLF, *FIGHTER_STATUS_KIND_SPECIAL_LW, 0, -1, -1],
			[*FIGHTER_KIND_WOLF, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, 0, -1, -1],
			[*FIGHTER_KIND_WOLF, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, 0, -1, -1],
			[*FIGHTER_KIND_WOLF, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP, 0, -1, -1],
			[*FIGHTER_KIND_FOX, *FIGHTER_STATUS_KIND_SPECIAL_LW, 0, -1, -1],
			[*FIGHTER_KIND_FOX, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, 0, -1, -1],
			[*FIGHTER_KIND_FOX, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, 0, -1, -1],
			[*FIGHTER_KIND_FOX, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP, 0, -1, -1],
			[*FIGHTER_KIND_MIIGUNNER,  *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_END, 0, -1, -1],
      		[*FIGHTER_KIND_MIIGUNNER,  *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD, 0, -1, -1],
			[*FIGHTER_KIND_MIIGUNNER,  *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HIT, 0, -1, -1],
			[*FIGHTER_KIND_MIIGUNNER,  *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_END, 0, -1, -1],
      		[*FIGHTER_KIND_MIIGUNNER,  *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_LOOP, 0, -1, -1],
      		[*FIGHTER_KIND_MIIGUNNER,  *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_HIT, 0, -1, -1],
			[*FIGHTER_KIND_SZEROSUIT, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_FLIP, 0, -1, -1]
		];
		for i in &jump_cancel {
			if fighter_kind == i[0] && status_kind == i[1] {
				println!("jc status");
				if i[3] != -1 && i[4] != -1 {
					if (frame as i32) < i[3] || (frame as i32) >= i[4] {
						continue;
					};
				};
				if i[2] != 0 {
					if AttackModule::is_infliction_status(boma, i[2]) {
						return true;
					};
				} else {
					return true;
				};
			};
		};
		return false;
	}
}
pub(crate) fn check_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
			return true;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
			if ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_stick_y(boma) >= 0.7 {
				return true;
			};
		};
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
			return true;
		};
		return false;
	}
}

//

unsafe extern "C" fn jump_cancel(fighter : &mut L2CFighterCommon) {
    unsafe {	
		let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let frame = MotionModule::frame(boma);
		if is_jc(boma, fighter_kind, status_kind, frame) && check_jump(boma){
				if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
				};
		};
    }
}
pub fn install() {
	Agent::new("fighter")
	.on_line(Main, jump_cancel)
	.install();
}