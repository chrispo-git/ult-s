use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{L2CValue, L2CAgent};
use std::{fs, path::Path};
use smash::phx::Vector2f;
use once_cell::sync::Lazy;
use crate::util::*;
static mut STALE_MAX : f32 = 1.0;
static mut STALE_TIMER_MAX : i32 = 480;
static mut FOOTSTOOL_STALE: [f32; 8] = [21.0; 8];
static mut FOOTSTOOL_STALE_TIMER: [i32; 8] = [0; 8];
static mut PERFECT_PIVOT: [bool; 8] = [false; 8];

static HOLD_BUFFER_LIMIT : i32 = 20; //Max frames for hold buffer

static HAS_NESS_FLAG: Lazy<bool> = Lazy::new(|| Path::new("sd:/ultimate/ult-s/ness.flag").exists());
static HAS_LUCAS_FLAG: Lazy<bool> = Lazy::new(|| Path::new("sd:/ultimate/ult-s/lucas.flag").exists());
static HAS_MEWTWO_FLAG: Lazy<bool> = Lazy::new(|| Path::new("sd:/ultimate/ult-s/mewtwo.flag").exists());
static HAS_TRAIL_FLAG: Lazy<bool> = Lazy::new(|| Path::new("sd:/ultimate/ult-s/trail.flag").exists());

static BUTTONS_LIST: Lazy<Vec<i32>> = Lazy::new(|| {
    vec![
        *CONTROL_PAD_BUTTON_ATTACK,
        *CONTROL_PAD_BUTTON_JUMP,
        *CONTROL_PAD_BUTTON_CATCH,
        *CONTROL_PAD_BUTTON_GUARD,
        *CONTROL_PAD_BUTTON_SMASH,
        *CONTROL_PAD_BUTTON_SPECIAL,
        *CONTROL_PAD_BUTTON_CSTICK_ON,
        *CONTROL_PAD_BUTTON_JUMP_MINI,
        *CONTROL_PAD_BUTTON_ATTACK_RAW,
        *CONTROL_PAD_BUTTON_SPECIAL_RAW,
        *CONTROL_PAD_BUTTON_SPECIAL_RAW2
    ]
});
//Perfect Pivot
pub unsafe fn perfectpivot(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize) {
    unsafe {
		if !is_mechanics_enabled() || is_gamemode("rivals".to_string()) {
			return;
		}
		if status_kind == *FIGHTER_STATUS_KIND_TURN {
			JostleModule::set_status(fighter.module_accessor, false);
		};
        if !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH) {
            crate::transition_reset!(ENTRY_ID, can_dash);
            crate::transition_reset!(ENTRY_ID, can_turndash);
            return;
        }
        let frame = MotionModule::frame(fighter.module_accessor) as i32;
        if (3..5).contains(&frame) {
			crate::transition_set!(ENTRY_ID, can_dash);
			crate::transition_set!(ENTRY_ID, can_turndash);
		    let lr = PostureModule::lr(fighter.module_accessor);
            let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;	
            if stick_x <= -0.5 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN, true);
            };
        } else {
            crate::transition_reset!(ENTRY_ID, can_dash);
            crate::transition_reset!(ENTRY_ID, can_turndash);
        };
    };
}

//Moonwalk
pub unsafe fn moonwalk(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize) {
    unsafe {
		if !is_mechanics_enabled() && !is_gamemode("rivals".to_string()) {
			return;
		}
        if !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH) {
            return;
        }
		let lr = PostureModule::lr(fighter.module_accessor);
		let mut stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;	
        if stick_x >= -0.2 {	
            return;
        }
        let walk_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_add"), 0);
        let walk_accel_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_mul"), 0);
        let walk_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_speed_max"), 0);
        let max_moonwalk = walk_speed_max * 1.8;
        let mw_modifier = 2.0;
        let moonwalk_speed = (stick_x*walk_accel_mul - walk_accel_add)*mw_modifier;
        let speed_x = get_speed_x(boma(fighter))*lr;
        if (speed_x)+moonwalk_speed > -max_moonwalk {
            let speed = smash::phx::Vector3f { x: moonwalk_speed, y: 0.0, z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speed);
        } else {
            let current_back = speed_x;
            let speed = smash::phx::Vector3f { x:-(current_back+max_moonwalk), y: 0.0, z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speed);
        }
    };
}
//JC Grab
pub unsafe fn jc_grab(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize) {
    unsafe {
		if !is_mechanics_enabled() || is_gamemode("rivals".to_string()) {
			return;
		}
        if status_kind != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            return;
        }
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
            return;
        }
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
            return;
        }
        if JC_GRAB_LOCKOUT[ENTRY_ID] == 0{
            JC_GRAB_LOCKOUT[ENTRY_ID] = MAX_LOCKOUT;
            GroundModule::attach_ground(fighter.module_accessor, true);
            GroundModule::set_attach_ground(fighter.module_accessor, true);
            StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_CATCH, true);
        };
    };
}

//DJC
pub unsafe fn djc(fighter : &mut L2CFighterCommon, status_kind : i32) {
    unsafe {
		if !is_mechanics_enabled() {
			return;
		}
        if crate::is_in!(status_kind, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_AIR_LASSO) {
            return;
        }
        let kinetic_type = KineticModule::get_kinetic_type(fighter.module_accessor);
        if crate::is_in!(kinetic_type, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL) {
            return;
        }
		let fighter_kind = smash::app::utility::get_kind(boma(fighter));
        if !crate::is_in!(fighter_kind, *FIGHTER_KIND_NESS, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_TRAIL) {
            return;
        }
        if 
            (fighter_kind == *FIGHTER_KIND_NESS && !*HAS_NESS_FLAG) ||
            (fighter_kind == *FIGHTER_KIND_LUCAS && !*HAS_LUCAS_FLAG) ||
            (fighter_kind == *FIGHTER_KIND_MEWTWO && !*HAS_MEWTWO_FLAG) ||
            (fighter_kind == *FIGHTER_KIND_TRAIL && !*HAS_TRAIL_FLAG)
        {
            return;
        }
        if fighter_kind == *FIGHTER_KIND_TRAIL && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            return;
        }
        if crate::is_in!(fighter_kind, *FIGHTER_KIND_NESS, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_MEWTWO) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
				if kinetic_type == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL {
					KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION);
				};
                return;
            }
            if !is_tap_djc(boma(fighter)) {
                return;
            }
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        } else {
            if !is_tap_djc(boma(fighter)) {
                return;
            }
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            if get_speed_y(boma(fighter)) > 2.5 {
                let new_speed = get_speed_x(boma(fighter))*PostureModule::lr(fighter.module_accessor);
                macros::SET_SPEED_EX(fighter, new_speed, 3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            };
        }
    };
}

pub unsafe fn hold_buffer_killer(fighter : &mut L2CFighterCommon, status_kind : i32) {
    unsafe {
        if crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_JUMP_SQUAT) {
            return;
        }
        if  ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || 
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                return;
        }
        let mut hold_buffer_lim = HOLD_BUFFER_LIMIT;
        let precede = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("precede"));
        if IS_SMALL_HOLD_BUFFER {
            hold_buffer_lim = precede; //hold buffer now matches precede
        }

        //Multiplies hold buffer duration by 2x during damage states to allow for pressing buttons out of hitstun as per usual
        if (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind){
            hold_buffer_lim *= 2;
        }
        //If time since you've pressed the button exceeds hold buffer limit, kills the input
        for i in BUTTONS_LIST.iter() {
                if ControlModule::get_trigger_count(fighter.module_accessor, *i as u8) > hold_buffer_lim && ControlModule::check_button_on(fighter.module_accessor, *i) 
                && !(ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && ItemModule::is_have_item(fighter.module_accessor, 0))
                && !(ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) && ItemModule::is_have_item(fighter.module_accessor, 0))
                {
                    ControlModule::reset_trigger(fighter.module_accessor);
                    ControlModule::clear_command(fighter.module_accessor, true);
                }
        }
    };
}

//Dash changes
pub unsafe fn dash(fighter : &mut L2CFighterCommon, status_kind : i32) {
    unsafe {
		if !is_mechanics_enabled() && !is_gamemode("rivals".to_string())  {
			return;
		}
        if !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, 
            *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN, 
            *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE) {
            return;
        }
		if crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH) && get_to_run_flag(boma(fighter)) {
			if  ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && 
                ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
			};
		};
		if GroundModule::is_passable_ground(fighter.module_accessor) {
            let flick_y = ControlModule::get_flick_y(fighter.module_accessor);
            if ControlModule::get_stick_y(fighter.module_accessor) <= -0.6875 && flick_y >= 5 && flick_y < 20 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
                return;
            };
        }
        let mut is_taunt_pressed = false;
        for i in [*CONTROL_PAD_BUTTON_APPEAL_S_L, *CONTROL_PAD_BUTTON_APPEAL_HI, *CONTROL_PAD_BUTTON_APPEAL_LW, *CONTROL_PAD_BUTTON_APPEAL_S_R] {
            if ControlModule::check_button_on_trriger(fighter.module_accessor, i) {
                is_taunt_pressed = true;
                break;
            }
        }
        if is_taunt_pressed {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_APPEAL, false);
        }
    };
}
//Parry Cancellable into a dash
pub unsafe fn parrycanceldash(fighter : &mut L2CFighterCommon, status_kind : i32, motion_kind : u64) {
    unsafe {
		if !is_mechanics_enabled() {
			return;
		}
        if motion_kind == hash40("just_shield_off") {
		    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
				StopModule::end_stop(fighter.module_accessor);
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
			};
		};
    };
}


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_air_tread_jump)]
pub unsafe fn sub_transition_group_check_air_tread_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cont;
    if fighter.global_table[0x30].get_bool() == false {
        cont = false;
    }
    else {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x30].get_ptr());
        cont = callable(fighter).get_bool();
    }
    if cont == false {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
			let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 
			|| (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 
			|| (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 
			|| (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0 
			|| ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_BUTTON) {
                    let do_footstool;
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
                        do_footstool = false;
                    }
                    else {
                        let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
                        let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
                        if !(tread_jump_speed_limit <= tread_speed_y) {
                            do_footstool = false;
                        }
                        else {
                            fighter.clear_lua_stack();
                            lua_args!(fighter, 0x21bfbd3f83u64);
                            smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                            do_footstool = fighter.pop_lua_stack(1).get_bool();
                        }
                    }
                    if do_footstool {
                        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
                        return true.into();
                    }
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_NO_TRIGGER) {
                fighter.clear_lua_stack();
                lua_args!(fighter, 0x21bfbd3f83u64, true);
                smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), false.into());
                    return true.into();
                }
            }
        }
    }
    else {
        return true.into();
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_TreadJump)]
unsafe fn status_treadjump(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Added taunt buttons to the "Is Button Footstool" check
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_BUTTON);
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    else {
        ControlModule::reset_flick_y(fighter.module_accessor);
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_TREAD_JUMP_COUNT);
    let tread_jump_disable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("tread_jump_disable_frame"));
    WorkModule::set_int(fighter.module_accessor, tread_jump_disable_frame, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FROM_TREAD, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_JUMP_FROM);
    fighter.sub_tread_jump_unique_process_init_inner();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_tread_jump_uniq_check();
    }
    fighter.global_table[0x14].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_tread_jump_uniq_check as *const () as _));
    let mut tread_attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("tread_attack_frame"));
    if MotionModule::is_flag_start_1_frame(fighter.module_accessor) {
        tread_attack_frame -= 1;
    }
    WorkModule::set_float(fighter.module_accessor, tread_attack_frame as f32, *FIGHTER_STATUS_TREAD_WORK_FLOAT_ATTACK_FRAME);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_TreadJump_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_sub_tread_jump_uniq_check)]
unsafe fn sub_tread_jump_uniq_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_NO_REACTION) && !is_gamemode("rivals".to_string()) {
        let jump_mini = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_BUTTON) {
            // If any valid footstool button is held, do not turn on the short hop flag
            ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
        }
        else {
            let jump_neutral_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y"));
            fighter.global_table[0x1B].get_f32() < jump_neutral_y
        };
        if jump_mini {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_transition_group_check_air_tread_jump,
            status_treadjump,
            sub_tread_jump_uniq_check
        );
    }
}

pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind : i32, motion_kind : u64, ENTRY_ID : usize) {
    perfectpivot(fighter, status_kind, ENTRY_ID);
    parrycanceldash(fighter, status_kind, motion_kind);
    dash(fighter, status_kind);
    djc(fighter, status_kind);
    hold_buffer_killer(fighter, status_kind);
    moonwalk(fighter, status_kind, ENTRY_ID);
    jc_grab(fighter, status_kind, ENTRY_ID);
}
pub fn install() {
    skyline::nro::add_hook(nro_hook);
}