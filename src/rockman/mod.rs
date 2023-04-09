use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smashline::*;
use smash::app::lua_bind::*;
use smash_script::*;
static mut MEGA_AERIAL : [bool; 8] = [false; 8];

#[acmd_script(
    agent = "rockman",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mm_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ArticleModule::generate_article_enable(FIGHTER_ROCKMAN_GENERATE_ARTICLE_ROCKBUSTER, false, 0)
			WorkModule::on_flag(Flag=FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=70, KBG=150, FKB=0, BKB=60, Size=3.0, X=0.0, Y=12.6, Z=5.5, X2=0.0, Y2=6.7, Z2=5.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-0.5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=70, KBG=150, FKB=0, BKB=60, Size=3.0, X=0.0, Y=9.6, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-0.5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=34)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[fighter_frame_callback]
pub fn megaman(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let mut stick_x = ControlModule::get_stick_x(boma) ;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let nair_landing = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_n"), 0);
		if fighter_kind == *FIGHTER_KIND_ROCKMAN {
			let is_shooting = [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_AIR, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_LANDING, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE].contains(&status_kind);
			if status_kind == 54 {
				MEGA_AERIAL[ENTRY_ID] = true;
			};
			if status_kind != 54 && !is_shooting{
				MEGA_AERIAL[ENTRY_ID] = false;
			};
			if is_shooting && MEGA_AERIAL[ENTRY_ID] {
				WorkModule::set_float(boma, nair_landing-1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
			};
			//println!("Is Shooting: {}, Is Landing: {}, Status: {}", is_shooting, is_land_cancel, status_kind);
		};
	};
}
		
		
		
pub fn install() {
    smashline::install_acmd_scripts!(
		mm_nair
    );
	smashline::install_agent_frame_callbacks!(megaman);
}
