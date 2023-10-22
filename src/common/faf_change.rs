use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::phx::Hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;

// FaF Changes Master
#[fighter_frame_callback]
pub fn faf_change_master(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32; //Cancel frame
		let fighter_kind = smash::app::utility::get_kind(boma);
		if fighter_kind == *FIGHTER_KIND_PZENIGAME {
			if motion_kind == hash40("attack_lw3") && frame >= 24.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_s3_s"), hash40("attack_s3_lw"), hash40("attack_s3_hi")].contains(&motion_kind) {
				let new_cancel = 22.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
		}else if fighter_kind == *FIGHTER_KIND_WOLF {
			if motion_kind == hash40("attack_hi3") && frame >= 23.0 {
				CancelModule::enable_cancel(boma);
			};
			if motion_kind == hash40("attack_lw3") && frame >= 21.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_IKE {
			if motion_kind == hash40("attack_11") && frame >= 15.0 {
				CancelModule::enable_cancel(boma);
			};
			if motion_kind == hash40("attack_lw3") && frame >= 25.0 {
				CancelModule::enable_cancel(boma);
			};
			if motion_kind == hash40("attack_hi3") && frame >= 34.0 {
				CancelModule::enable_cancel(boma);
			};
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
		}else if fighter_kind == *FIGHTER_KIND_PALUTENA {
			if [hash40("attack_air_f")].contains(&motion_kind) && frame >= 28.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_hi3")].contains(&motion_kind) && frame >= 34.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("throw_lw")].contains(&motion_kind) && frame >= 38.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_MIIFIGHTER {
			if [hash40("special_lw2_landing")].contains(&motion_kind) && frame >= 15.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_lw2_kick_landing")].contains(&motion_kind) && frame >= 22.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_lw2_start")].contains(&motion_kind) {
				if frame > 2.0 && frame < 8.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				} else {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
			};
			if [hash40("special_air_n1"), hash40("special_n1")].contains(&motion_kind) && frame >= 58.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_air_n2_start"), hash40("special_n2_start")].contains(&motion_kind) {
				if frame < 12.0 {
					MotionModule::set_rate(boma, 1.5);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("special_s1_start")].contains(&motion_kind) && frame >= 60.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_air_hi")].contains(&motion_kind) && frame >= 32.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_MIIGUNNER {
			if [hash40("special_hi1"), hash40("special_air_hi1")].contains(&motion_kind){
				if frame > 3.0 && frame < 9.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				} else {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
			};
			if [hash40("special_s1"), hash40("special_air_s1")].contains(&motion_kind) && frame >= 58.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_s3")].contains(&motion_kind) && frame >= 30.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_lw3")].contains(&motion_kind) && frame >= 33.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_hi3")].contains(&motion_kind) && frame >= 30.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_s4")].contains(&motion_kind) && frame >= 73.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_hi4")].contains(&motion_kind) && frame >= 60.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_lw4")].contains(&motion_kind) && frame >= 60.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("throw_lw")].contains(&motion_kind) && frame >= 32.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("throw_hi")].contains(&motion_kind) && frame >= 39.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("throw_f")].contains(&motion_kind) && frame >= 28.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_n2_end")].contains(&motion_kind) && frame >= 4.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_s2_end"), hash40("special_air_s2_end")].contains(&motion_kind) && frame >= 30.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_lw3_end"), hash40("special_air_lw3_end")].contains(&motion_kind) && frame >= 5.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_lw1_end"), hash40("special_air_lw1_end")].contains(&motion_kind) && frame >= 23.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_s3_1"), hash40("special_air_s3_1")].contains(&motion_kind) {
				let new_cancel = 43.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("special_s1_start")].contains(&motion_kind) && frame >= 60.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_air_s1_start")].contains(&motion_kind) && frame >= 50.0 {
				CancelModule::enable_cancel(boma);
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
		}else if fighter_kind == *FIGHTER_KIND_PACMAN {
			if [hash40("attack_dash")].contains(&motion_kind) {
				let new_cancel = 53.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("attack_13")].contains(&motion_kind) && frame >= 25.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_RYU {
			if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) && frame >= 42.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_KAMUI {
			if [hash40("attack_air_n")].contains(&motion_kind) && frame >= 36.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_lw3")].contains(&motion_kind) && frame >= 23.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_11")].contains(&motion_kind) && frame >= 20.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_s_jump_landing")].contains(&motion_kind) && frame >= 8.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_s_jump")].contains(&motion_kind){
				if frame > 2.0 && frame < 15.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				} else {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
			};
		}else if fighter_kind == *FIGHTER_KIND_INKLING {
			if [hash40("attack_air_b")].contains(&motion_kind) && frame >= 33.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_hi_landing")].contains(&motion_kind) {
				let new_cancel = 40.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("special_n_end")].contains(&motion_kind) {
				let new_cancel = 29.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
		}else if fighter_kind == *FIGHTER_KIND_SIMON {
			if [hash40("special_hi")].contains(&motion_kind) {
				if frame > 3.0 && frame < 15.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				} else {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
			};
			if [hash40("attack_lw3")].contains(&motion_kind) {
				let new_cancel = 50.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("attack_lw3_2")].contains(&motion_kind) {
				let new_cancel = 55.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("attack_hold_end")].contains(&motion_kind) && frame >= 1.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("throw_lw")].contains(&motion_kind) && frame >= 32.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_air_f_hi"), hash40("attack_air_f_lw"), hash40("attack_air_f")].contains(&motion_kind) && frame >= 27.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_RICHTER {
			if [hash40("attack_air_hi")].contains(&motion_kind) && frame >= 30.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_air_lw"), hash40("special_lw")].contains(&motion_kind) && frame >= 40.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_air_b_hi"), hash40("attack_air_b"), hash40("attack_air_b_lw"), hash40("attack_air_f_hi"), hash40("attack_air_f"), hash40("attack_air_f_lw")].contains(&motion_kind) {
				let new_cancel = 48.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
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
		}else if fighter_kind == *FIGHTER_KIND_SHIZUE {
			if [hash40("attack_air_f")].contains(&motion_kind) {
				let new_cancel = 50.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("attack_air_b")].contains(&motion_kind) {
				let new_cancel = 46.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("attack_air_n")].contains(&motion_kind) && frame >= 25.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_13")].contains(&motion_kind) && frame >= 22.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_lw4")].contains(&motion_kind) && frame >= 35.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_hi3")].contains(&motion_kind) && frame >= 24.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("throw_hi")].contains(&motion_kind) && frame >= 44.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_air_n_failure"), hash40("special_n_failure")].contains(&motion_kind) && frame >= 10.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_lw_fire"), hash40("special_air_lw_fire")].contains(&motion_kind) && frame >= 3.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_s_throw_f"), hash40("special_air_s_throw_f")].contains(&motion_kind) && frame >= 40.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_s_throw_lw"), hash40("special_air_s_throw_lw")].contains(&motion_kind) && frame >= 30.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("special_s_throw_hi"), hash40("special_air_s_throw_hi")].contains(&motion_kind) {
				let new_cancel = 58.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("special_s_throw_b"), hash40("special_air_s_throw_b")].contains(&motion_kind) {
				let new_cancel = 46.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("special_n_fire"), hash40("special_air_n_fire")].contains(&motion_kind) && frame >= 48.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_BUDDY {
			if [hash40("throw_f")].contains(&motion_kind) && frame >= 29.0 {
				CancelModule::enable_cancel(boma);
			};
		}else if fighter_kind == *FIGHTER_KIND_DOLLY {
			if [hash40("attack_s3_s")].contains(&motion_kind) {
				let new_cancel = 31.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
		}else if fighter_kind == *FIGHTER_KIND_MASTER{
			if [hash40("throw_lw")].contains(&motion_kind) && frame >= 29.0 {
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
		}/*else if fighter_kind == *FIGHTER_KIND_PICKEL {
			if [*FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK].contains(&status_kind) && WorkModule::is_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_HI3)  && cancel_frame > 0.0 {
				let mut new_cancel = 24.0;
				if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
					new_cancel = 19.0;
				};
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate_partial(boma, 1.0/((new_cancel - cancel_frame)+1.0)*0.5); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [*FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL].contains(&status_kind) && WorkModule::is_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI) && cancel_frame > 0.0{
				let mut new_cancel = 23.0;
				if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
					new_cancel = 18.0;
				};
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("attack_s4_s"), hash40("attack_s4_s_arm")].contains(&motion_kind) {
				let mut new_cancel = 52.0;
				if WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
					new_cancel = 47.0;
				};
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
		}*/else if fighter_kind == *FIGHTER_KIND_EDGE {
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
		}else if fighter_kind == *FIGHTER_KIND_TRAIL {
			if [hash40("attack_s3")].contains(&motion_kind) && frame >= 34.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_hi3")].contains(&motion_kind) && frame >= 50.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_lw3")].contains(&motion_kind) && frame >= 24.0 {
				CancelModule::enable_cancel(boma);
			};
			if [hash40("attack_dash")].contains(&motion_kind) {
				let new_cancel = 50.0;
				if frame > (cancel_frame - 2.0) &&  frame < (cancel_frame) {
					MotionModule::set_rate(boma, 1.0/((new_cancel - cancel_frame)+1.0)); // Sets the motion rate to add a set number of frames of endlag on the very end of the move
				} else if frame >= cancel_frame {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("attack_air_f")].contains(&motion_kind)&& frame >= 33.0 {
				CancelModule::enable_cancel(boma);
			};
		};
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(faf_change_master);
}