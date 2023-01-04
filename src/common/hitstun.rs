use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lib::L2CAgent;
use smash::lib::L2CValue;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::phx::Hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::*;


#[fighter_frame_callback]
pub fn non_tumble_di(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
        let status_kind = StatusModule::status_kind(boma);
        let status_kind_prev = StatusModule::prev_status_kind(boma, 0);
        let lua_state = fighter.lua_state_agent;
        let mut l2c_agent = L2CAgent::new(lua_state);
        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR].contains(&status_kind) {
            println!("hitlag: {}", StopModule::get_hit_stop_real_frame(boma));
            if StopModule::get_hit_stop_real_frame(boma) as i32 == 1{
                smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
                println!("opff for hitstun being triggered");
            }
            if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR].contains(&status_kind)
                && [*FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_THROWN].contains(&status_kind_prev) {
                let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
                let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
                let hitstun_passed = total_hitstun - remaining_hitstun;
                if total_hitstun > 0.0 && hitstun_passed == 1.0 {
                    println!("opff for throwstun being triggered");
                    smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
                }
            }
        }
    }
}
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon::ftStatusUniqProcessDamage_exec_common)]
pub unsafe fn ftStatusUniqProcessDamage_exec_common_hook(fighter: &mut L2CFighterCommon){
    let ret = original!()(fighter);

    smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
    println!("hook for hitstun being triggered");

    ret
}
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon::ftStatusUniqProcessDamage_exec)]
pub unsafe fn ftStatusUniqProcessDamage_exec_hook(fighter: &mut L2CFighterCommon){
    let ret = original!()(fighter);

    smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
    println!("hook2 for hitstun being triggered");

    ret
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(non_tumble_di);
    skyline::install_hooks!(ftStatusUniqProcessDamage_exec_common_hook, ftStatusUniqProcessDamage_exec_hook);
}