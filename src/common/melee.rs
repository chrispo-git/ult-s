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

static mut JUMPSQUAT_SPEED: [f32; 8] = [0.0; 8];
static mut AIR_X: [f32; 8] = [0.0; 8];
static mut HAS_HIT_IN_ATTACK: [bool; 8] = [false; 8];
static mut ASDI_START: [bool; 8] = [false; 8];
static mut LANDING_COUNTER: [i32; 8] = [0; 8];
static mut ASDI : f32 = 3.0; //15 for testing purposes, should be 3
static mut DIR_MULT : f32 = 57.295776842880464966688235343549; //Very fun number that turns direction that spits out ControlModule::get_stick_dir(boma) as an angle in degrees





unsafe extern "C" fn melee_hat(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma); 
		let stickx = ControlModule::get_stick_x(boma);		 
		let sticky = ControlModule::get_stick_y(boma);	
		let substickx = ControlModule::get_sub_stick_x(boma);		 
		let substicky = ControlModule::get_sub_stick_y(boma);	
		let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
		let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);		
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let end_frame = MotionModule::end_frame(boma);
		if ItemModule::is_attach_item(boma, smash::app::ItemKind(*ITEM_KIND_USAGIHAT)) {
			//Removes ground to air limit
			if ![
				*FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_LOOP, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL
			].contains(&status_kind) {
				if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT) == false{
					WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
				};
			}else {
				WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
			};
			//JC Grab
			if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
			};
			//Shield Drop
			if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind) {
				if GroundModule::is_passable_ground(fighter.module_accessor) {
					if ControlModule::get_stick_y(boma) <= -0.6875 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
					};
				}
			};
			//Ledge Snap stuff
			if get_speed_y(boma) > 0.2 {
				CAN_CLIFF[ENTRY_ID] = 1;
			} else {
				CAN_CLIFF[ENTRY_ID] = 0;
			};
			//Disable Airdodge/Attack out of hitstun
			if remaining_hitstun > 0.0 && [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
				CAN_AIRDODGE[ENTRY_ID] = 1;
				CAN_ATTACK_AIR[ENTRY_ID] = 1;
			} else {
				CAN_AIRDODGE[ENTRY_ID] = 0;
				CAN_ATTACK_AIR[ENTRY_ID] = 0;
			};
			//ASDI
			if StopModule::is_damage(boma) {
				ASDI_START[ENTRY_ID] = true;
			};
			if status_kind == *FIGHTER_STATUS_KIND_LANDING {
				if remaining_hitstun > 0.0 {
					LANDING_COUNTER[ENTRY_ID] +=1 ;
					if LANDING_COUNTER[ENTRY_ID] >= 4 {
						CancelModule::enable_cancel(boma);
					};
				};
			} else {
				LANDING_COUNTER[ENTRY_ID] = 0;
			};
			if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE {
				if MotionModule::frame(boma) > 27.0 && MotionModule::frame(boma) < 42.0{
					KineticModule::clear_speed_all(boma);
				};
				if MotionModule::frame(boma) > 3.0 && MotionModule::frame(boma) < 30.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				} else {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
				MotionModule::set_rate(boma, 1.0);
			}else if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
				if MotionModule::frame(boma) > 8.0 && MotionModule::frame(boma) < 42.0{
					KineticModule::clear_speed_all(boma);
				};
				if MotionModule::frame(boma) > 3.0 && MotionModule::frame(boma) < 30.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				} else {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
				MotionModule::set_rate(boma, 1.0);
			};
			if ASDI_START[ENTRY_ID] && !StopModule::is_damage(boma){
				let mut stick_dir = ControlModule::get_stick_dir(boma) * DIR_MULT;
				let sub_stick_dir = ControlModule::get_sub_stick_dir(boma) * DIR_MULT;
				let mut hypotenuse = ASDI;
				if stickx < 0.2 && stickx > -0.2 && sticky > -0.2 && sticky < 0.2 {
					hypotenuse = 0.001;
				};
				if (substickx < 0.2 && substickx > -0.2 && substicky > -0.2 && substicky < 0.2) == false {
					stick_dir = sub_stick_dir;
				};
				let mut stick_lr = 1.0;
				if stickx < 0.0 {
					stick_lr = -1.0;
				};
				let mut stick_ud = 1.0;
				if sticky < 0.0 {
					stick_ud = -1.0;
				};
				let x_distance = stick_dir.cos()*hypotenuse*stick_lr;
				let y_distance = stick_dir.sin()*hypotenuse*stick_ud;
				//println!("Hitstop {}, Stick_Dir {}, X {}, Y {}", StopModule::get_hit_stop_real_frame(boma), stick_dir, x_distance, y_distance);
				if ray_check_pos(boma, x_distance, y_distance, false) == 0 {
					let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)+x_distance, y: PostureModule::pos_y(boma)+y_distance, z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
				} else if ray_check_pos(boma, x_distance/2.0, y_distance/2.0, false) == 0{
					let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)+x_distance*0.5, y: PostureModule::pos_y(boma)+y_distance*0.5, z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
				};
				ASDI_START[ENTRY_ID] = false;
			};
		} else {
			if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT) != false {
				WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
			};
			LANDING_COUNTER[ENTRY_ID] = 0;
		};
	};
}	


pub fn install() {
    Agent::new("fighter")
	.on_line(Main, melee_hat)
	.install();
}