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

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, snake_side_special_status_main)
    .status(Main, *FIGHTER_STATUS_KIND_APPEAL, snake_taunt_status_main)
    .status(End, *FIGHTER_STATUS_KIND_APPEAL, snake_taunt_status_end)
    .status(Main, *FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT, snake_down_taunt_wait_status_main)
    .status(End, *FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT, snake_down_taunt_wait_status_end)
    .status(Main, *FIGHTER_SNAKE_STATUS_KIND_APPEAL_END, snake_down_taunt_end_status_main)
    .status(End, *FIGHTER_SNAKE_STATUS_KIND_APPEAL_END, snake_down_taunt_end_status_end)
    .install();
}

unsafe extern "C" fn snake_side_special_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_side_main_loop as *const () as _))
    // 0.into()
}

/*unsafe extern "C" fn snake_side_smash_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    original!(fighter)
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_side_smash_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
    SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = false;
    SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] = 0;
    original!(fighter)
}*/


////added new up-taunt and side-taunt
unsafe extern "C" fn snake_taunt_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn snake_taunt_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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

//added down-taunt box walk and c4 place/explode
unsafe extern "C" fn snake_down_taunt_wait_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn snake_down_taunt_wait_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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
unsafe extern "C" fn snake_down_taunt_end_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn snake_down_taunt_end_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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