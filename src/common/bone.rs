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

pub(crate) unsafe fn bone_const(boma: &mut smash::app::BattleObjectModuleAccessor, new_fighter_kind : i32, new_motion_kind : u64, bone : u64,
	start_frame : f32, end_frame : f32, 
	x_rotate : f32, x_rotate_end : f32, y_rotate : f32, y_rotate_end : f32,  z_rotate : f32, z_rotate_end : f32
) -> () {
	let fighter_kind = smash::app::utility::get_kind(boma);
	let motion_kind = MotionModule::motion_kind(boma);
	let frame = MotionModule::frame(boma);
	let duration = end_frame-start_frame;
	if fighter_kind == new_fighter_kind && motion_kind == new_motion_kind && frame >= start_frame && frame <= end_frame {
		let mut rotation = Vector3f{x: x_rotate + (((x_rotate_end-x_rotate)/duration)*(frame-start_frame)), y: y_rotate + (((y_rotate_end-y_rotate)/duration)*(frame-start_frame)) , z: z_rotate + (((z_rotate_end-z_rotate)/duration)*(frame-start_frame))};
	    ModelModule::set_joint_rotate(boma, Hash40::new_raw(bone), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
	};
}
pub(crate) unsafe fn bone_inst(boma: &mut smash::app::BattleObjectModuleAccessor, bone : u64,
	x_rotate : f32, y_rotate : f32, z_rotate : f32
) -> () {
	let fighter_kind = smash::app::utility::get_kind(boma);
	let motion_kind = MotionModule::motion_kind(boma);
	let frame = MotionModule::frame(boma);
	let mut rotation = Vector3f{x: x_rotate, y: y_rotate , z: z_rotate };
	ModelModule::set_joint_rotate(boma, Hash40::new_raw(bone), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
}
unsafe extern "C" fn bone_rot(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		//Example: Diddy Kong Fair Rotation being angled
		//bone_const(boma, *FIGHTER_KIND_DIDDY, hash40("attack_air_f"), hash40("rot"), 0.0, 999.0, -45.0, 0.0, 0.0, 0.0, 0.0, 0.0);

		
		//Dark Samus Fair angle down
		bone_const(boma, *FIGHTER_KIND_SAMUSD, hash40("attack_air_f"), hash40("rot"), 0.0, 47.0, 22.5, 22.5, 0.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_SAMUSD, hash40("attack_air_f"), hash40("rot"), 47.0, 59.0, 22.5, 0.0, 0.0, 0.0, 0.0, 0.0);
		
		/*
		//Dsamus Dair reverse
		bone_const(boma, *FIGHTER_KIND_SAMUSD, hash40("attack_air_lw"), hash40("rot"), 0.0, 31.0, 0.0, 0.0, 180.0, 180.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_SAMUSD, hash40("attack_air_lw"), hash40("rot"), 31.0, 60.0, 0.0, 0.0, 180.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_SAMUSD, hash40("landing_air_lw"), hash40("rot"), 0.0, 15.0, 0.0, 0.0, 180.0, 180.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_SAMUSD, hash40("landing_air_lw"), hash40("rot"), 15.0, 59.0, 0.0, 0.0, 180.0, 0.0, 0.0, 0.0);
		*/
		 
		//MiiGunner bair angle up
		bone_const(boma, *FIGHTER_KIND_MIIGUNNER, hash40("attack_air_b"), hash40("rot"), 0.0, 24.0, 10.0, 10.0, 0.0, 0.0, 0.0, 0.0);
		
		//Falcon nair angle down
		bone_const(boma, *FIGHTER_KIND_CAPTAIN, hash40("attack_air_n"), hash40("rot"), 0.0, 24.0, 10.0, 10.0, 0.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_CAPTAIN, hash40("attack_air_n"), hash40("rot"), 24.0, 38.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0);
		
		//Roy Fair angle down
		bone_const(boma, *FIGHTER_KIND_ROY, hash40("attack_air_f"), hash40("rot"), 0.0, 12.0, 0.0, -8.0, 0.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_ROY, hash40("attack_air_f"), hash40("rot"), 12.0, 16.0, -8.0, 5.0, 0.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_ROY, hash40("attack_air_f"), hash40("rot"), 16.0, 31.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0);
		
		//Roy Dair angle side
		/*bone_const(boma, *FIGHTER_KIND_ROY, hash40("attack_air_lw"), hash40("rot"), 0.0, 37.0, 0.0, 0.0, 45.0, 45.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_ROY, hash40("attack_air_lw"), hash40("rot"), 37.0, 62.0, 0.0, 0.0, 45.0, 0.0, 0.0, 0.0);*/
		
		//Villy fair foot angle down
		bone_const(boma, *FIGHTER_KIND_MURABITO, hash40("attack_air_f"), hash40("footl"), 0.0, 44.0, 35.0, 35.0, 0.0, 0.0, 0.0, 0.0);
		
		//Byleth feet fix
		bone_const(boma, *FIGHTER_KIND_MASTER, hash40("special_air_s_front"), hash40("footl"), 0.0, 180.0, 0.0, 0.0, 0.0, 0.0, 55.0, 55.0);
		bone_const(boma, *FIGHTER_KIND_MASTER, hash40("special_air_s_front"), hash40("footr"), 0.0, 180.0, 0.0, 0.0, 0.0, 0.0, 55.0, 55.0);
		bone_const(boma, *FIGHTER_KIND_MASTER, hash40("special_air_s_front_dash"), hash40("footl"), 0.0, 180.0, 0.0, 0.0, 0.0, 0.0, 55.0, 55.0);
		bone_const(boma, *FIGHTER_KIND_MASTER, hash40("special_air_s_front_dash"), hash40("footr"), 0.0, 180.0, 0.0, 0.0, 0.0, 0.0, 55.0, 55.0);
		
		//Falco 
		bone_const(boma, *FIGHTER_KIND_FALCO, hash40("throw_hi"), hash40("handr"), 13.0, 37.0, 0.0, 0.0, stick_x*20.0 -15.0, stick_x*20.0 -15.0, 0.0, 0.0);
		
		//Snake Fsmash angle up
		bone_const(boma, *FIGHTER_KIND_SNAKE, hash40("attack_s4_s"), hash40("handr"), 21.0, 25.0, 0.0, -30.0, 0.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_SNAKE, hash40("attack_s4_s"), hash40("handr"), 25.0, 45.0, -30.0, -30.0, 0.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_SNAKE, hash40("attack_s4_s"), hash40("handr"), 45.0, 65.0, -30.0, 0.0, 0.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_SNAKE, hash40("attack_s4_hold"), hash40("handr"), 0.0, 60.0, -30.0, -30.0, 0.0, 0.0, 0.0, 0.0);
	
	};
}
unsafe extern "C" fn sword_size(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if [*FIGHTER_KIND_TOONLINK].contains(&fighter_kind) {
			let joint_scale = smash::phx::Vector3f { x: 1.12, y: 1.12, z: 1.12 };
			ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &joint_scale);
		};
		if [*FIGHTER_KIND_DEDEDE].contains(&fighter_kind) {
			let max_size = 1.75;
			if MotionModule::motion_kind(boma) == hash40("attack_air_b") {
				if MotionModule::frame(boma) < 5.0 {
					let mut mul = 1.0;
					mul += (MotionModule::frame(boma) * (max_size/5.0));
					let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
				} else if 4.0 < MotionModule::frame(boma) && 13.0 >= MotionModule::frame(boma) {
					let mut mul = max_size;
					let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
				} else if 13.0 < MotionModule::frame(boma){
					if MotionModule::frame(boma) < 15.0 {
						let mut mul = 1.65;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 16.0  {
						let mut mul = 1.55;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else if MotionModule::frame(boma) < 17.0  {
						let mut mul = 1.45;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 18.0 {
						let mut mul = 1.35;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 19.0  {
						let mut mul = 1.25;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 20.0  {
						let mut mul = 1.15;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else {
						let mut mul = 1.0;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					};
				};
			};
		};
	};
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, bone_rot)
	.on_line(Main, sword_size)
	.install();
}