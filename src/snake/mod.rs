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
use smash::phx::Vector2f;
use crate::util::*;
use super::*;

mod status;
mod frame;
mod acmd;
			
static mut STATIC_MUT : [i32; 8] = [6; 8];
static mut SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE : [bool; 8] = [false; 8];
static mut SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED : [bool; 8] = [false; 8];
static mut SNAKE_INT_ATTACK_S4_COMBO_COUNT : [i32; 8] = [0; 8];
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

pub unsafe fn special_side_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }else {
        fun_7100018800(fighter, false);
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return true.into()
            }
        }
    }
    false.into()
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
        macros::PLAY_SE(fighter, Hash40::new("se_snake_special_n01"));
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
        macros::SET_SPEED_EX(fighter, velocity_x*0.6, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    return false.into()
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

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}