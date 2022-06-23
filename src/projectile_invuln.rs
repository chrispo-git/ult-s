use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
//This is here because FOR SOME GOD FORSAKEN REASON it doesnt like being put in the hooks file. idk why but it just doesnt???
#[fighter_frame_callback]
pub fn projectile_invuln_master(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let life_mul = 0.05;
		let speed_mul = 0.05;
		let dmg_mul = 0.005;
		let reflector_max = 500.0;
		//Example Character - Super Fucking Luigi
		if fighter_kind == *FIGHTER_KIND_LUIGI {
			if [hash40("special_s"), hash40("special_air_s"), hash40("special_s_discharge")].contains(&MotionModule::motion_kind(boma)){ // When within these motions, luigi has a reflector on his neck bone that despawns projectiles
				acmd!(lua_state, {
					sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("neck")/*Bone*/, 7.0/*Size*/, 1.0/*X1*/, 0.0/*Y1*/, 0.0/*Z1*/, 0.0/*X2*/, 1.2/*Y2*/, 0.0/*Z2*/, dmg_mul/*Damage Mul*/, life_mul/*Life Mul*/, reflector_max/*Max DMG*/, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
				});
			};
		}else if fighter_kind == *FIGHTER_KIND_PURIN {
			if [hash40("special_s"), hash40("special_air_s")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 3.0 && MotionModule::frame(boma) < 10.0 {
					acmd!(lua_state, {
						sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 6.0, 0.0, 4.0, 5.0, 0.0, 4.0, 10.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
					});
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		}else if fighter_kind == *FIGHTER_KIND_FALCO {
			if [hash40("attack_air_f")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 8.0 && MotionModule::frame(boma) < 37.0 {
					acmd!(lua_state, {
						sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 7.0, 0.0, 5.5, 5.0, 0.0, 5.5, 15.5, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
					});
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		}else if fighter_kind == *FIGHTER_KIND_GANON {
			if [hash40("special_lw")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 7.0 && MotionModule::frame(boma) < 36.0 {
					acmd!(lua_state, {
						sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("kneer"), 9.0, 7.0, 0.0, 0.0, 7.0, 0.0, 0.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
					});
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		}else if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
			if [*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) {
				acmd!(lua_state, {
					sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 6.0, 0.0, 6.0, 2.0, 0.0, 6.0, 18.5, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
				});
			};
		} else if fighter_kind == *FIGHTER_KIND_DEDEDE {
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 15.0 && MotionModule::frame(boma) < 28.0 {
					acmd!(lua_state, {
						sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("hip"), 12.0, -5.0, 0.0, 0.0, 14.0, 0.0, 0.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
					});
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		} else if fighter_kind == *FIGHTER_KIND_MIIFIGHTER {
			if [hash40("special_air_s2"), hash40("special_s2")].contains(&MotionModule::motion_kind(boma)){
				acmd!(lua_state, {
					sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("hip"), 12.0, -5.0, 0.0, 0.0, -5.0, 0.0, 0.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
				});
			};
		}else if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
			if [hash40("special_s1"), hash40("special_air_s1")].contains(&MotionModule::motion_kind(boma)){
				acmd!(lua_state, {
					sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 8.0, 0.0, 12.0, 6.0, 0.0, 3.5, 6.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
				});
			};
		}else if fighter_kind == *FIGHTER_KIND_MIIGUNNER {
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 8.0 && MotionModule::frame(boma) < 16.0 {
					acmd!(lua_state, {
						sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("handr"), 7.5, 8.0, 0.0, 0.0, 8.0, 0.0, 0.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
					});
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		} else if fighter_kind == *FIGHTER_KIND_KEN || fighter_kind == *FIGHTER_KIND_RYU {
			if [hash40("special_s")].contains(&MotionModule::motion_kind(boma)){
				acmd!(lua_state, {
					sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 7.0, 0.0, 12.5, -11.0, 0.0, 12.5, 12.5, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
				});
			};
		} else if fighter_kind == *FIGHTER_KIND_BAYONETTA {
			if [hash40("special_s")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) > 14.0 && MotionModule::frame(boma) < 32.0 {
					acmd!(lua_state, {
						sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("footr"), 7.0, -8.0, 0.0, 0.0, 0.0, 1.2, 0.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
					});
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		} else if fighter_kind == *FIGHTER_KIND_RICHTER {
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) > 10.0 && MotionModule::frame(boma) < 17.0 {
					acmd!(lua_state, {
						sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 15.0, 0.0, 7.0, 0.0, 0.0, 7.0, 0.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
					});
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		} else if fighter_kind == *FIGHTER_KIND_GAOGAEN {
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) > 7.0 && MotionModule::frame(boma) < 19.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					acmd!(lua_state, {
						sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("legl"), 10.0, 3.5, 1.2, 0.0, 3.5, 1.2, 0.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
					});
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		} else if fighter_kind == *FIGHTER_KIND_TRAIL {
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 5.0 && MotionModule::frame(boma) < 35.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					acmd!(lua_state, {
						sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 8.0, 0.0, 1.8, 2.0, 0.0, 1.8, 10.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
					});
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		} else if fighter_kind == *FIGHTER_KIND_DOLLY {
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 5.0 && MotionModule::frame(boma) < 23.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					acmd!(lua_state, {
						sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 9.0, 0.0, 10.0, 3.0, 0.0, 4.0, 3.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
					});
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		} else if fighter_kind == *FIGHTER_KIND_KROOL {
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 10.0 && MotionModule::frame(boma) < 29.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					acmd!(lua_state, {
						sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 9.5, 4.0, 3.0, 3.0, 4.0, 3.0, 3.0, dmg_mul, life_mul, reflector_max, false, speed_mul, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
					});
				} else {
					shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
				};
			};
		};
	};
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(projectile_invuln_master);
}
