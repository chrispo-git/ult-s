
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


unsafe extern "C" fn sixtyfour(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("sixtyfour".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let mut stickx = ControlModule::get_stick_x(boma);	
        let lr = PostureModule::lr(boma);	
		stickx = stickx * lr;

        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);    
        
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y);
        
        WorkModule::set_float(fighter.module_accessor, 65.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
        
        if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD{
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4, false);
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD{
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4, false);
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD{
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4, false);
        }
        if status_kind == *FIGHTER_STATUS_KIND_CATCH_DASH{
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, false);
        }
        let mut shieldstun_mul = 1.75/0.8;
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            shieldstun_mul = shieldstun_mul/0.33;
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) {
            shieldstun_mul = shieldstun_mul/0.725;
        }
        for i in 0..6 {
            if AttackModule::is_attack(fighter.module_accessor, i, false) {
                macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ i as u64, /*ShieldstunMul*/ shieldstun_mul);
            }
        }
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
            MotionModule::set_rate(boma, 0.8);
		};
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, sixtyfour)
	.install();
}