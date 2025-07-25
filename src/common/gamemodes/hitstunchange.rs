
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

unsafe extern "C" fn hitstun(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("hitstun".to_string()) && !is_gamemode("sixtyfour".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = StatusModule::status_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
        let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
        let hitstun_mul = 1.5;
        if remaining_hitstun > 0.0 {
            if remaining_hitstun == total_hitstun {
                println!("Old hitstun {}, New Hitstun {}", remaining_hitstun, remaining_hitstun*hitstun_mul);
                WorkModule::set_float(boma, remaining_hitstun*hitstun_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
                WorkModule::set_float(boma, (remaining_hitstun*hitstun_mul)+1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
            } else {
                println!("hitstun countdown! {}", remaining_hitstun);
            }
        }
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, hitstun)
	.install();
}