use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;

#[acmd_script(
    agent = "gaogaen",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn incin_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			JostleModule::set_status(false)
			rust {
				if StatusModule::situation_kind(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) != *SITUATION_KIND_AIR {  
					acmd!(lua_state, {
						ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=13.0, Angle=55, KBG=81, FKB=0, BKB=70, Size=5.4, X=3.5, Y=1.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KNEE)
					});
				};
			}
		}
		wait(Frames=4)
		if(is_excute){
			rust {
				if StatusModule::situation_kind(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) != *SITUATION_KIND_AIR {  
					acmd!(lua_state, {
						ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=9.0, Angle=55, KBG=81, FKB=0, BKB=70, Size=4.8, X=3.5, Y=1.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KNEE)
					});
				};
			}
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=19)
		if(is_excute){
			JostleModule::set_status(true)
		}
		frame(Frame=20)
		if(is_excute){
			rust {
				if StatusModule::situation_kind(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) != *SITUATION_KIND_AIR {  
					acmd!(lua_state, {
						WorkModule::on_flag(Flag=FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_FOLLOW_THROUGH)
					});
				};
			}
		}
    });
}	
		
pub fn install() {
    smashline::install_acmd_scripts!(
		incin_da
    );
}
