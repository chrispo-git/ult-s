use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use crate::util::*;

unsafe extern "C" fn projectile_invuln_master(fighter : &mut L2CFighterCommon) {
    unsafe {
		if !is_mechanics_enabled() {
			return;
		}
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let life_mul = 0.1;
		let speed_mul = 0.0;
		let dmg_mul = 0.0;
		let reflector_max = 50.0;
		//Example Character - Super Fucking Luigi
		if fighter_kind == *FIGHTER_KIND_LUIGI {
			if [hash40("special_s"), hash40("special_air_s"), hash40("special_s_discharge")].contains(&MotionModule::motion_kind(boma)){ // When within these motions, luigi has a reflector on his neck bone that despawns projectiles
				shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("neck"), /*Size*/ 1.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ 0.0, /*Y2*/ 1.2, /*Z2*/ 0.0, /*Power*/ dmg_mul, /*Speed*/ speed_mul, /*Max Damage*/ reflector_max, false, /*Lifetime*/ life_mul, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
			};
		}else if fighter_kind == *FIGHTER_KIND_DEDEDE {
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 15.0 && MotionModule::frame(boma) < 28.0 {
					shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("hip"), /*Size*/ 12.0, /*X*/ -5.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ 14.0, /*Y2*/ 0.0, /*Z2*/ 0.0, /*Power*/ dmg_mul, /*Speed*/ speed_mul, /*Max Damage*/ reflector_max, false, /*Lifetime*/ life_mul, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		} else if fighter_kind == *FIGHTER_KIND_MURABITO && is_default(boma) {
			if [hash40("special_n")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 3.0 && MotionModule::frame(boma) < 10.0 {
					shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("stickr"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 3.0, /*X2*/ 0.0, /*Y2*/ 0.0, /*Z2*/ 11.0, /*Power*/ 1.2, /*Speed*/ 1.2, /*Max Damage*/ 80.0, false, /*Lifetime*/ 1.1, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		}else if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
			if [hash40("special_s1"), hash40("special_air_s1")].contains(&MotionModule::motion_kind(boma)){
					shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 6.0, /*X2*/ 0.0, /*Y2*/ 3.5, /*Z2*/ 6.0, /*Power*/ dmg_mul, /*Speed*/ speed_mul, /*Max Damage*/ reflector_max, false, /*Lifetime*/ life_mul, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
			};
		} else if fighter_kind == *FIGHTER_KIND_RYU {
			if [hash40("special_s")].contains(&MotionModule::motion_kind(boma)){
				shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), /*Size*/ 6.0, 0.0, 12.5, -11.0, 0.0, 12.5, 12.5, /*Power*/ dmg_mul, /*Speed*/ speed_mul, /*Max Damage*/ reflector_max, false, /*Lifetime*/ life_mul, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
			};
		} else if fighter_kind == *FIGHTER_KIND_GAOGAEN {
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) > 7.0 && MotionModule::frame(boma) < 19.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("legl"), 10.0, 3.5, 1.2, 0.0, 3.5, 1.2, 0.0, /*Power*/ dmg_mul, /*Speed*/ speed_mul, /*Max Damage*/ reflector_max, false, /*Lifetime*/ life_mul, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		} else if fighter_kind == *FIGHTER_KIND_KROOL {
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 10.0 && MotionModule::frame(boma) < 29.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 9.5, 4.0, 3.0, 3.0, 4.0, 3.0, 3.0, /*Power*/ dmg_mul, /*Speed*/ speed_mul, /*Max Damage*/ reflector_max, false, /*Lifetime*/ life_mul, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		} else if fighter_kind == *FIGHTER_KIND_BRAVE {
            if [hash40("attack_s3")].contains(&MotionModule::motion_kind(boma)) {
                if MotionModule::frame(boma) > 8.0 && MotionModule::frame(boma) < 11.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                    shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("shield"), 6.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, /*Power*/ dmg_mul, /*Speed*/ speed_mul, /*Max Damage*/ reflector_max, false, /*Lifetime*/ life_mul, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
                } else {
                    shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
                };
            };
		};
	}
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, projectile_invuln_master)
	.install();
}
