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

//bowling ball
unsafe extern "C" fn bob_omb_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = StatusModule::status_kind(weapon.module_accessor);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let is_toad_weapon = (WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
        
        if fighter_kind == *WEAPON_KIND_MURABITO_BOWLING_BALL && is_toad_weapon {
			if status_kind == *WEAPON_MURABITO_BOWLING_BALL_STATUS_KIND_FALL {
				if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) {
				};
			};
		}
    }
}
unsafe extern "C" fn final_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = StatusModule::status_kind(weapon.module_accessor);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let is_toad_weapon = (WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
        
        if fighter_kind == *WEAPON_KIND_MURABITO_CLAYROCKET && is_toad_weapon {
			if status_kind == *WEAPON_MURABITO_CLAYROCKET_STATUS_KIND_READY {
				StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_STATUS_KIND_FLY, false);
			};
			if PostureModule::lr(weapon.module_accessor) < 0.0 {
				let mut rotation = Vector3f{x: 0.0, y: -25.0 , z: 0.0 };
				ModelModule::set_joint_rotate(boma, Hash40::new("trans"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});	
			}
			macros::SET_SPEED_EX(weapon, 2.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_NONE);
		}
	}
}
unsafe extern "C" fn toad(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let end_frame = MotionModule::end_frame(boma);
		//Kirby neutralb logic
		/*if fighter_kind == *FIGHTER_KIND_KIRBY {
			if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_MURABITO {
				if status_kind == *FIGHTER_KIRBY_STATUS_KIND_MURABITO_SPECIAL_N_SEARCH {
					if frame > 38.0 {
						if situation_kind == *SITUATION_KIND_GROUND {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
						};
					};
				};
				if ItemModule::is_have_item(boma, 0) {
					CAN_NEUTRALB[ENTRY_ID] = 1;
				} else {
					CAN_NEUTRALB[ENTRY_ID] = 0;
				};
			} else {
				CAN_NEUTRALB[ENTRY_ID] = 0;
			};
		};*/
		if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) && fighter_kind == *FIGHTER_KIND_MURABITO {
			if ModelModule::scale(boma) == WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0) {
                ModelModule::set_scale(boma, 0.9);
                AttackModule::set_attack_scale(boma, 0.9, true);
                GrabModule::set_size_mul(boma, 0.9);
            }
			let scale = smash::phx::Vector3f { x: 0.8, y: 1.0, z: 1.0 };
			ModelModule::set_joint_scale(boma, Hash40::new("shoulderl"), &scale);
			ModelModule::set_joint_scale(boma, Hash40::new("shoulderr"), &scale);
			WorkModule::set_int(boma, 1, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_TIME_LIMIT);
			if status_kind == *FIGHTER_STATUS_KIND_DEAD {
				if MotionModule::motion_kind(boma) == hash40("fall_damage") && !HAS_DEADED[ENTRY_ID] {
					macros::PLAY_SE(fighter, Hash40::new("se_murabito_catch_net"));
					HAS_DEADED[ENTRY_ID] = true;
				};
			} else {
				HAS_DEADED[ENTRY_ID] = false;
			};
			if [*FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {
				if BOUNCE_DA[ENTRY_ID] {
					BOUNCE_DA[ENTRY_ID] = false;
					MotionModule::change_motion(fighter.module_accessor, smash::phx::Hash40::new("attack_dash"), 8.0, 1.0, false, 0.0, false, false);
				};
				let dist = 4.0*PostureModule::lr(boma);
				if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) && (6..30).contains(&(MotionModule::frame(boma) as i32)) && ray_check_pos(boma, dist, 0.0, false) == 1 {
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					BOUNCE_DA[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
				};
			} else {
				BOUNCE_DA[ENTRY_ID] = false;
			};
			if ![*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if ![*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_FIREWORK,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
					if frame < 32.0 {
						MotionModule::set_frame(boma, 32.0, false);
					};
				};
			};
			if ![*FIGHTER_STATUS_KIND_APPEAL].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if ![*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_HELMET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			} else {
				if GroundModule::can_entry_cliff(boma) == 1 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) < 7 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME) < 1 && ControlModule::get_stick_y(boma) > -0.5{
					fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(),false.into());
				};
			}
			if ![*FIGHTER_STATUS_KIND_THROW].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_WEEDS,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT].contains(&status_kind) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP, true);
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT_FAIL].contains(&status_kind) && situation_kind == *SITUATION_KIND_GROUND {
				if frame < 2.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP, true);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				};
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP].contains(&status_kind) {
				if frame >= 66.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
				if frame > 30.0 && situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				}
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && frame < 50.0{
					macros::SET_SPEED_EX(fighter, SPEED_X[ENTRY_ID]*PostureModule::lr(boma), 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_lw_water"), 51.0, 1.0, false, 0.0, false, false);
				};
				if frame > 5.0 && frame < 7.0{
					macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackdash01"));
				}
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_LANDING, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT].contains(&status_kind) {
				if situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				};
			};
			if situation_kind != *SITUATION_KIND_AIR {
				HAS_DOWNB[ENTRY_ID] = false;
			};
			if [hash40("special_air_hi"), hash40("special_hi"), hash40("special_air_hi_wait1"), hash40("special_air_hi_wait2"), hash40("special_air_hi_flap1"), hash40("special_air_hi_flap2"), hash40("special_air_hi_flap1")].contains(&MotionModule::motion_kind(boma)) {
				fighter.clear_lua_stack();
				lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
				smash::app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, true);
			};
			if [hash40("throw_f")].contains(&MotionModule::motion_kind(boma)) {
				if status_kind != *FIGHTER_STATUS_KIND_THROW_KIRBY {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_THROW_KIRBY, true);
				};
				if frame > 60.0 {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
					};
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					};
				};
			};
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
				if frame > 40.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
				};
			};
			if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
				if frame > 38.0 {
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
					};
				};
			};
			if ItemModule::is_have_item(boma, 0) {
				CAN_NEUTRALB[ENTRY_ID] = 1;
			} else {
				CAN_NEUTRALB[ENTRY_ID] = 0;
			};
			//ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_CLAYROCKET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_BALLOON,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			WorkModule::off_flag(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_CATCHING);
			if [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
				if BEFORE_SIDEB_BOUNCE[ENTRY_ID] > 0 {
					BEFORE_SIDEB_BOUNCE[ENTRY_ID] -= 1;
				};
				if situation_kind == *SITUATION_KIND_GROUND {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
					};
					if LAND_SIDEB_BOUNCE[ENTRY_ID] > 0 {
						LAND_SIDEB_BOUNCE[ENTRY_ID] -= 1;
						println!("Land Sideb: {}", LAND_SIDEB_BOUNCE[ENTRY_ID]);
					};
					if (11..20).contains(&(frame as i32)) || (LAND_SIDEB_BOUNCE[ENTRY_ID] > 0 && frame < 52.0){
						if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_JUMP, true);
						};
					};
					if BEFORE_SIDEB_BOUNCE[ENTRY_ID] > 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_JUMP, true);
					};
				} else {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
					};
					if frame >= 15.0 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
					}
					if frame > 30.0 {
						let cat1 = ControlModule::get_command_flag_cat(boma, 0);
						if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0{
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						};
						if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
							BEFORE_SIDEB_BOUNCE[ENTRY_ID] = 3;
						};
					};
					LAND_SIDEB_BOUNCE[ENTRY_ID] = 8;
					CAN_SIDEB[ENTRY_ID] = 1;
				};
			} else {
				LAND_SIDEB_BOUNCE[ENTRY_ID] = 0;
				BEFORE_SIDEB_BOUNCE[ENTRY_ID] = 0;
			};
			if situation_kind != *SITUATION_KIND_AIR || (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind){ 
				CAN_SIDEB[ENTRY_ID] = 0;
			}
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_JUMP].contains(&status_kind) {
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_FALL {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
				};
				if frame >= 13.0 {
					CancelModule::enable_cancel(boma);
				};
				if frame >= 14.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
			};
		};
	};
}

unsafe extern "C" fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_MURABITO {
            return;
        }
		/*if !(WorkModule::get_int(&mut *fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(&mut *fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127) {
			return;
		}*/
        fighter.global_table[0x45].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
    }
}

pub fn install() {
    Agent::new("murabito")
		.on_start(agent_reset)
        .on_line(Main, toad)
        .install();

    Agent::new("murabito_clayrocket")
        .on_line(Main, final_frame)
        .install();
}