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
use crate::masked::*;
use super::*;
pub fn install() {
    Agent::new("lucas")
        .on_line(Main, maskedman_frame)
        .install();
}


unsafe extern "C" fn maskedman_frame(agent: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lua_state = agent.lua_state_agent;
        let module_accessor = sv_system::battle_object_module_accessor(lua_state);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
        let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
        let stick_y = ControlModule::get_stick_y(boma);
        let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        if is_added(boma) && fighter_kind == *FIGHTER_KIND_LUCAS {
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("normalbatm"),false);
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("normalbatmflip"),true);
            //ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("wings"),false);

            //removes snake cause its never needed
            ArticleModule::remove_exist(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

            if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false {
                HAS_NEUTRALB[ENTRY_ID] = true;
            };
            if StatusModule::situation_kind(boma) != SITUATION_KIND_AIR || (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) {
                HAS_NEUTRALB[ENTRY_ID] = true;
            };

            //neutral b shit
            if [hash40("special_n_start")].contains(&MotionModule::motion_kind(boma)) {
                if MotionModule::frame(boma) >= 15.0 {
                    MotionModule::change_motion(boma, Hash40::new("special_n_dash"), 0.0, 1.0, false, 0.0, false, false);
                };
            };
            if [hash40("special_air_n_start")].contains(&MotionModule::motion_kind(boma)) {
                //VarModule::on_flag(agent.battle_object, DISABLE_SPECIAL_N);
                WorkModule::set_flag(boma, true, DISABLE_SPECIAL_N);
                if MotionModule::frame(boma) >= 15.0 {
                    MotionModule::change_motion(boma, Hash40::new("special_air_n_dash"), 0.0, 1.0, false, 0.0, false, false);
                };
            };
            if [hash40("special_n_dash")].contains(&MotionModule::motion_kind(boma)) {
                if MotionModule::frame(boma) >= 5.0 && MotionModule::frame(boma) <= 11.0 {
                    macros::SET_SPEED_EX(agent, 5.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                else {
                    macros::SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                };
                if MotionModule::frame(boma) >= 23.0 {
                    MotionModule::change_motion(boma, Hash40::new("special_n_end"), 0.0, 1.0, false, 0.0, false, false);
                };
            };
            if [hash40("special_air_n_dash")].contains(&MotionModule::motion_kind(boma)) {
                if MotionModule::frame(boma) >= 5.0 && MotionModule::frame(boma) <= 11.0 {
                    macros::SET_SPEED_EX(agent, 5.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                else {
                    macros::SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                };
                if MotionModule::frame(boma) >= 23.0 {
                    MotionModule::change_motion(boma, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
                };
            };
            if [hash40("special_n_end")].contains(&MotionModule::motion_kind(boma)) {
                if MotionModule::frame(boma) >= 20.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                };
            };
            if [hash40("special_air_n_end")].contains(&MotionModule::motion_kind(boma)) {
                if stick_y <= -0.5 {
                    GroundModule::pass_floor(boma);
                    if ray_check_pos(boma, 0.0, -0.6, false) == 1 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                        macros::SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    };
                } 
                else {
                    GroundModule::clear_pass_floor(boma);
                    if ray_check_pos(boma, 0.0, -0.6, true) == 1 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                        macros::SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    };
                };
                if MotionModule::frame(boma) >= 20.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
            };
        };
    };
}