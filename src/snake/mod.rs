use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::{
            Hash40,
            Vector2f//,
            // Vector3f
        },
        app::{
            sv_animcmd::frame,
            sv_animcmd::wait,
            lua_bind::*,
            *
        },
        lib::{
            lua_const::*,
            *
        }
    },
    smashline::*,
    smash_script::{
        macros::*,
        *
    }
};

use crate::util::*;

static mut STATIC_MUT : [i32; 8] = [6; 8];
static mut SNAKE_FLAG_APPEAL_LW_C4_EXLPODE : [bool; 8] = [false; 8];
static mut SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT : [i32; 8] = [0; 8];
static mut SNAKE_FLAG_CATCH_WAIT_IS_WALK : [bool; 8] = [false; 8];
static SNAKE_APPEAL_LW_GRENADE_WAIT_MAX : i32 = 30;

//implimented function for checking if an article is "constrained" to snake
extern "C" {
    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Snake21is_constraint_articleERNS_7FighterEiNS_22ArticleOperationTargetE"]
    pub fn is_constraint_article(
        arg1: *mut smash::app::Fighter,
        arg2: i32,
        arg3: smash::app::ArticleOperationTarget,
    ) -> bool;
}

#[acmd_script( agent = "snake", script = "game_attackdashlightthrow", category = ACMD_GAME, low_priority)]
unsafe fn snake_dash_attack_throw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_THROW_DASH, false);
		}
}
pub unsafe fn fun_7100018800(fighter: &mut L2CFighterCommon, skip_check: bool) {
    if skip_check == false {
        if StatusModule::situation_kind(fighter.module_accessor) == StatusModule::prev_situation_kind(fighter.module_accessor) {
            return
        }
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{x:0.0, y:0.0});
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
}


#[fighter_frame( agent = FIGHTER_KIND_SNAKE )]
fn snake_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if ItemModule::is_have_item(boma, 0) {
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
        }
		if motion_kind == hash40("attack_dash")
        && frame >= 12.0 {
            if ItemModule::is_have_item(boma, 0) {
				if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
					MotionModule::change_motion(boma, Hash40::new("attack_dash_item_light_throw"), 0.0, 1.0, false, 0.0, false, false);
					AttackModule::clear_all(boma);
				}
            }
        }
        if motion_kind == hash40("attack_dash_item_light_throw") {
            if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR && stick_y <= -0.6875 {
                if GroundModule::is_passable_ground(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
                }
            }
        }
    }
}



////fixed unwanted buffered throws and walking
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_pull_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_pull_main_loop as *const () as _))
    // 0.into()
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_DASH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_dash_pull_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_pull_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_grab_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }else if MotionModule::is_end(fighter.module_accessor) {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1
        || PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1{
            SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
        }
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
        return true.into()
    }else if MotionModule::frame(fighter.module_accessor) >= 1.0 {
        if fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW != 0 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }else if fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI != 0 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        //check stick directly for easier instant f-throw
        }else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
        }else if fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B != 0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
        }else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), false.into());
            return true.into()
        }
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_attack_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_attack"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_attack_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_grab_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if ControlModule::get_stick_y(fighter.module_accessor) < -0.7 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }else if ControlModule::get_stick_y(fighter.module_accessor) > 0.7 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()

        }else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }else{
                SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
                return true.into()
            }
        }else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.7 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }else{
                SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
                return true.into()
            }
        }else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_attack"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    return false.into()
}

////added grab walk
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_wait_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    ControlModule::reset_trigger(fighter.module_accessor);
    if SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_f"), 0.0, 1.0, false, 0.0, false, false);
        }else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_b"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = false;
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
        }
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_wait_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_grab_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }else if fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW != 0 {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return true.into()
    }else if fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI != 0 {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return true.into()
    }else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.7 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), false.into());
            return true.into()
        }
    }
    else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1 {
        if SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] == false {
            SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_b"), 0.0, 1.0, false, 0.0, false, false);
        }
        let walk_speed:f32 = 1.6*(PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor)*-1.0);
        MotionModule::set_rate(fighter.module_accessor, walk_speed);
    }
    else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1 {
        if SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] == false {
            SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_f"), 0.0, 1.0, false, 0.0, false, false);
        }
        let walk_speed:f32 = 1.4*(PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor));
        MotionModule::set_rate(fighter.module_accessor, walk_speed);
    }
    else if SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] {
        SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = false;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_grab_wait_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = false;
    original!(fighter)
}
////

////added new up-taunt and side-taunt
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_taunt_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    if ControlModule::get_command_flag_cat(fighter.module_accessor, 1) == *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r"), 0.0, 1.0, false, 0.0, false, false);
    }else if ControlModule::get_command_flag_cat(fighter.module_accessor, 1) == *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L
    || ControlModule::get_command_flag_cat(fighter.module_accessor, 1) == *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_taunt_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_taunt_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }else if  MotionModule::is_end(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("appeal_lw_r") {
            fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT.into(), false.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }else if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX)
    && MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("appeal_lw_r") {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_taunt_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT {
        fighter.clear_lua_stack();
        let object = sv_system::battle_object(fighter.lua_state_agent);
        let fighta : *mut Fighter = std::mem::transmute(object);
        if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    return 0.into()
}

#[acmd_script( agent = "snake", script = "sound_appealsr", category = ACMD_SOUND )]
unsafe fn snake_side_taunt_snd(fighter : &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("vc_snake_win03"));
        }
}

#[acmd_script( agent = "snake", script = "sound_appealhir", category = ACMD_SOUND )]
unsafe fn snake_up_taunt_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_arg10(fighter, 0, 2, 360/*angle*/, 10/*size*/, 1, 0, 12, 30, 30, 80);
        // physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.2, 0.2, -1, 0.7, 0.5, -1, Hash40::new("invalid"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_snake_appealhi"));
    }
}


#[acmd_script( agent = "snake", script = "game_appealendexplode", category = ACMD_GAME )]
unsafe fn snake_down_taunt_explode_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0 );
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, 0);
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
        // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
        // SNAKE_C4_FLAG_IS_SHOWTIME[entry_id] = true;
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 90.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
#[acmd_script( agent = "snake", script = "expression_appealendexplode", category = ACMD_EXPRESSION )]
unsafe fn snake_down_taunt_explode_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0 );
    }
    frame(fighter.lua_state_agent, 30.0);
    slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 5, false, 0);
    }
}
#[acmd_script( agent = "snake", script = "sound_appealendexplode", category = ACMD_SOUND )]
unsafe fn snake_down_taunt_explode_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_snake_appealend"));
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_l04"));
        PLAY_SE(fighter, Hash40::new("se_snake_squat"));
    }
    // frame(fighter.lua_state_agent, 70.0);
    // if is_excute(fighter) {
    // }
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_l05"));
    }
}
#[acmd_script( agent = "snake", script = "effect_appealendexplode", category = ACMD_EFFECT )]
unsafe fn snake_down_taunt_explode_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
//added down-taunt box walk and c4 place/explode
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_taunt_wait_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_wait"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
    let appeal_wait_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("appeal_wait_frame"));
    WorkModule::set_int(fighter.module_accessor, appeal_wait_frame, *FIGHTER_SNAKE_STATUS_APPEAL_WORK_INT_WAIT_COUNTER);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    SNAKE_FLAG_APPEAL_LW_C4_EXLPODE[entry_id] = false;
    SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT[entry_id] = 0;
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_down_taunt_wait_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_down_taunt_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT[entry_id] > 0 {
        SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT[entry_id] -= 1;
    }
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_WORK_INT_WAIT_COUNTER);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    || WorkModule::get_int(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_WORK_INT_WAIT_COUNTER) <= 0 {
        fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_APPEAL_END.into(), false.into());
        return true.into()
    //place c4
    }else if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) == *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4) {
            SNAKE_FLAG_APPEAL_LW_C4_EXLPODE[entry_id] = true;
            fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_APPEAL_END.into(), false.into());
        }else {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, false, 0);
            ArticleModule::have(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, Hash40::new("havel"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), 0, false);
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    //spawn grenade
    }else if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) == *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE)
    && SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT[entry_id] <= 0
    {
        SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT[entry_id] = SNAKE_APPEAL_LW_GRENADE_WAIT_MAX;

        ////adjusts first grenade position only
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, 0);
        PLAY_SE(fighter, Hash40::new("se_snake_special_n01"));
        ArticleModule::have(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, Hash40::new("havel"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), 0, false);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        // let pos_x = PostureModule::pos_x(fighter.module_accessor);
        // let pos_y = PostureModule::pos_y(fighter.module_accessor);
        // ArticleModule::set_pos(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, Vector3f{x:pos_x, y:pos_y+1.0, z:0.0});

        ////doesn't set player as owner of grenade
        // ItemModule::attach_item(fighter.module_accessor, ItemKind(*ITEM_KIND_SNAKEGRENADE), *ATTACH_ITEM_GROUP_BODY, true);
        // EFFECT_OFF_KIND(fighter, Hash40::new("sys_item_get"), true, true);
        // PLAY_SE(fighter, Hash40::new("se_snake_special_n01"));
        // ItemModule::drop_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_SNAKEGRENADE), 0.0, 0.0);

        // DamageModule::add_damage(fighter.module_accessor, *LINK_NO_ARTICLE as f32, 0);

        ////adjusts first article only
        // ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, 0);
        // let article_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CONSTRAINT);
        // let article_boma = sv_battle_object::module_accessor(article_id as u32);
        // LinkModule::set_model_constraint_target_joint(article_boma, Hash40::new("kneer"));
    }else {
        let velocity_x :f32 = PostureModule::lr(fighter.module_accessor) * ControlModule::get_stick_x(fighter.module_accessor);
        SET_SPEED_EX(fighter, velocity_x*0.6, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_taunt_wait_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_APPEAL_END {
        fighter.clear_lua_stack();
        let object = sv_system::battle_object(fighter.lua_state_agent);
        let fighta : *mut Fighter = std::mem::transmute(object);
        if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    return 0.into()
}
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_taunt_end_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if SNAKE_FLAG_APPEAL_LW_C4_EXLPODE[entry_id] {
        SNAKE_FLAG_APPEAL_LW_C4_EXLPODE[entry_id] = false;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_end_explode"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_down_taunt_end_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_down_taunt_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_taunt_end_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    let object = sv_system::battle_object(fighter.lua_state_agent);
    let fighta : *mut Fighter = std::mem::transmute(object);
    if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return 0.into()
}
////


pub fn install() {
    smashline::install_status_scripts!(
        snake_grab_pull_status_main,
        snake_grab_dash_pull_status_main,
        snake_grab_attack_status_main,
        snake_grab_wait_status_main,
        snake_grab_wait_status_end,
        snake_taunt_status_main,
        snake_taunt_status_end,
        snake_down_taunt_wait_status_main,
        snake_down_taunt_wait_status_end,
        snake_down_taunt_end_status_main,
        snake_down_taunt_end_status_end
    );
	smashline::install_acmd_scripts!(
		snake_dash_attack_throw,
        snake_side_taunt_snd,

        snake_up_taunt_snd,

        snake_down_taunt_explode_game,
        snake_down_taunt_explode_exp,
        snake_down_taunt_explode_snd,
        snake_down_taunt_explode_eff
	);
    smashline::install_agent_frames!(
        snake_frame
    );
}