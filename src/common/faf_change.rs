use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::phx::Hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;

// FaF Changes Master
unsafe extern "C" fn faf_change_master(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32; //Cancel frame
		let fighter_kind = smash::app::utility::get_kind(boma);
		if fighter_kind == *FIGHTER_KIND_IKE {
			if [hash40("attack_hi4")].contains(&motion_kind) && frame < 2.0 {
				MotionModule::change_motion(boma, Hash40::new("attack_hi4"), 5.0, 1.0, false, 0.0, false, false);
			};
		}else if fighter_kind == *FIGHTER_KIND_ROBOT {
			if [hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw")].contains(&motion_kind) && frame >= 24.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_lw3")].contains(&motion_kind) {
				let new_cancel = 17.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
		}else if fighter_kind == *FIGHTER_KIND_SHULK {
			if [hash40("attack_11")].contains(&motion_kind) && frame < 2.0 {
				MotionModule::change_motion(boma, Hash40::new("attack_11"), 3.0, 1.0, false, 0.0, false, false);
			};
		}else if fighter_kind == *FIGHTER_KIND_MIIFIGHTER {
			if [hash40("special_air_n2_start"), hash40("special_n2_start")].contains(&motion_kind) {
				if frame < 12.0 {
					MotionModule::set_rate(boma, 1.5);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};

		}else if fighter_kind == *FIGHTER_KIND_KOOPAJR {
			if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) && frame >= 40.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("throw_hi")].contains(&motion_kind) && frame >= 33.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("throw_f")].contains(&motion_kind) && frame >= 30.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("throw_lw")].contains(&motion_kind) && frame >= 63.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
			if [hash40("attack_air_b")].contains(&motion_kind) && frame < 5.0 && frame > 3.0{
				MotionModule::change_motion(boma, Hash40::new("attack_air_b"), 10.0, 1.0, false, 0.0, false, false);
			};
		}else if fighter_kind == *FIGHTER_KIND_DUCKHUNT {
			if [hash40("throw_lw")].contains(&motion_kind) && frame >= 43.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("throw_hi")].contains(&motion_kind) && frame >= 32.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_RYU {
			if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) && frame >= 42.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_KROOL {
			if [hash40("attack_air_hi")].contains(&motion_kind) && frame >= 58.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_hi3")].contains(&motion_kind) && frame >= 32.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_air_n")].contains(&motion_kind) && frame >= 30.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_13")].contains(&motion_kind) && frame >= 22.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_dash")].contains(&motion_kind) && frame >= 54.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_n_fire"), hash40("special_air_n_fire")].contains(&motion_kind) && frame >= 48.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_s_throw"), hash40("special_air_s_throw")].contains(&motion_kind){
				if MotionModule::frame(boma) < 12.0 {
					WorkModule::off_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_S_FLAG_ENABLE_SUPER_ARMOR);
				};
				if MotionModule::frame(boma) >= 12.0 && MotionModule::frame(boma) <= 64.0 {
					WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_S_FLAG_ENABLE_SUPER_ARMOR);
				};
				if MotionModule::frame(boma) > 64.0 {
					WorkModule::off_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_S_FLAG_ENABLE_SUPER_ARMOR);
				};
			};
		}else if fighter_kind == *FIGHTER_KIND_BUDDY {
			if [hash40("throw_f")].contains(&motion_kind) && frame >= 29.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_TANTAN {
			if [hash40("attack_lw3")].contains(&motion_kind) && frame >= 29.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_hi3")].contains(&motion_kind) && frame >= 28.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_air_hi")].contains(&motion_kind) && frame >= 27.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_dash")].contains(&motion_kind) {
				let new_cancel = 46.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
		}else if fighter_kind == *FIGHTER_KIND_EDGE {
			if [hash40("attack_13")].contains(&motion_kind) && frame >= 31.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_lw3")].contains(&motion_kind) && frame >= 35.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_ELIGHT {
			if [hash40("attack_13")].contains(&motion_kind) && frame >= 20.0 {
				CancelModule::enable_cancel(boma);
			};
		};
    }
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, faf_change_master)
	.install();
}