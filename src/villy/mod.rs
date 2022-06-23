use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;

#[acmd_script(
    agent = "murabito",
    script =  "game_catch",
    category = ACMD_GAME)]
unsafe fn villy_grab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.642857)
		frame(Frame=13)
		if(is_excute){
			GrabModule::set_rebound(CanCatchRebound=true)
		}
		frame(Frame=14)
		if(is_excute){
			CATCH(ID=0, Bone=hash40("top"), Size=5.0, X=0.0, Y=5.5, Z=4.0, X2=0.0, Y2=5.5, Z2=14.0, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
			CATCH(ID=1, Bone=hash40("top"), Size=2.5, X=0.0, Y=5.5, Z=1.5, X2=0.0, Y2=5.5, Z2=16.5, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
			ENABLE_AREA(FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH)
		}
		FT_MOTION_RATE(FSM=1)
		rust{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(Frames=3)
		if(is_excute){
			sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
			GrabModule::set_rebound(CanCatchRebound=false)
		}
		wait(Frames=2)
		if(is_excute){
			UNABLE_AREA(FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_catchdash",
    category = ACMD_GAME)]
unsafe fn villy_dashgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.625)
		frame(Frame=15)
		if(is_excute){
			GrabModule::set_rebound(CanCatchRebound=true)
		}
		frame(Frame=16)
		if(is_excute){
			CATCH(ID=0, Bone=hash40("top"), Size=4.0, X=0.0, Y=5.5, Z=4.0, X2=0.0, Y2=5.5, Z2=13.0, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
			CATCH(ID=1, Bone=hash40("top"), Size=2.0, X=0.0, Y=5.5, Z=2.0, X2=0.0, Y2=5.5, Z2=15.0, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
			ENABLE_AREA(FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH_DASH)
		}
		FT_MOTION_RATE(FSM=1)
		rust{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(Frames=3)
		if(is_excute){
			sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
			GrabModule::set_rebound(CanCatchRebound=false)
		}
		wait(Frames=2)
		if(is_excute){
			UNABLE_AREA(FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH_DASH)
		}
    });
}		
#[acmd_script(
    agent = "murabito",
    script =  "game_catchturn",
    category = ACMD_GAME)]
unsafe fn villy_pivotgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.625)
		frame(Frame=16)
		if(is_excute){
			GrabModule::set_rebound(CanCatchRebound=true)
		}
		FT_MOTION_RATE(FSM=1)
		frame(Frame=17)
		if(is_excute){
			CATCH(ID=0, Bone=hash40("top"), Size=5.0, X=0.0, Y=5.5, Z=-5.0, X2=0.0, Y2=5.5, Z2=-14.0, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
			CATCH(ID=1, Bone=hash40("top"), Size=2.5, X=0.0, Y=5.5, Z=-2.5, X2=0.0, Y2=5.5, Z2=-16.5, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
			ENABLE_AREA(FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH_TURN)
		}
		rust{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(Frames=3)
		if(is_excute){
			sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
			GrabModule::set_rebound(CanCatchRebound=false)
		}
		wait(Frames=2)
		if(is_excute){
			UNABLE_AREA(FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH_TURN)
		}
    });
}		
#[acmd_script(
    agent = "murabito",
    script =  "game_attacks3",
    category = ACMD_GAME)]
unsafe fn villy_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA, true, FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA)
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=14.0, Angle=45, KBG=50, FKB=0, BKB=40, Size=4.5, X=0.0, Y=9.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=14.0, Angle=45, KBG=50, FKB=0, BKB=40, Size=2.5, X=-5.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=14.0, Angle=45, KBG=50, FKB=0, BKB=40, Size=2.5, X=5.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=14.0, Angle=45, KBG=50, FKB=0, BKB=40, Size=2.0, X=0.0, Y=4.0, Z=0.0, X2=0.0, Y2=1.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=12)
		if(is_excute){
			AttackModule::clear_all()
		}
		FT_MOTION_RATE(FSM=0.44)
		wait(frames=10)
		FT_MOTION_RATE(FSM=0.88)
		frame(Frame=37)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=48)
    });
}			
#[acmd_script(
    agent = "murabito",
    script =  "game_attacklw3",
    category = ACMD_GAME)]
unsafe fn villy_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=90, KBG=95, FKB=0, BKB=50, Size=5.0, X=0.0, Y=3.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=100, KBG=95, FKB=0, BKB=50, Size=6.0, X=0.0, Y=3.0, Z=10.0, X2=0.0, Y2=3.0, Z2=2.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		FT_MOTION_RATE(FSM=0.5)
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackairf",
    category = ACMD_EFFECT)]
unsafe fn villy_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=3)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 5.6, -4, 0, 0, 0, 1.1, true, EF_FLIP_YZ)
			EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), 0, 5.6, 0, 0, 0, 0, 0.8, true)
		}
		frame(Frame=4)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 13, 5.6, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false)
			LAST_EFFECT_SET_ALPHA(0.7)
		}
    });
}		
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackairf",
    category = ACMD_SOUND)]
unsafe fn villy_fair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_s_finish"))
		}
    });
}		
#[acmd_script(
    agent = "murabito",
    script =  "game_attackairf",
    category = ACMD_GAME)]
unsafe fn villy_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			rust{
				if PostureModule::lr(fighter.module_accessor) == 1.0 {
					ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_glove_l"),true);
				} else {
					ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_glove_r"),true);
				};
			}
		}
		FT_MOTION_RATE(FSM=1.5)
		frame(Frame=4)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=110, FKB=0, BKB=30, Size=3.4, X=0.0, Y=5.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=110, FKB=0, BKB=30, Size=3.8, X=0.0, Y=5.5, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=30)
		FT_MOTION_RATE(FSM=1.5)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=30)
		if(is_excute){
			rust{
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_glove_l"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_glove_r"),false);
			}
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_specialairnfire",
    category = ACMD_EFFECT)]
unsafe fn villy_fair_help1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_specialairnfire",
    category = ACMD_SOUND)]
unsafe fn villy_fair_help2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[fighter_frame_callback]
pub fn villy(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		if fighter_kind == *FIGHTER_KIND_MURABITO {
			if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) == false {
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_glove_l"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_glove_r"),false);
			};
			if motion_kind == hash40("attack_air_f") && frame >= 32.0 {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n_fire"), 28.0, 1.0, false, 0.0, false, false);
				CancelModule::enable_cancel(boma);
				macros::EFFECT_OFF_KIND(fighter, Hash40::new("murabito_putaway_falsh"), false, true);
				macros::STOP_SE(fighter, Hash40::new("se_murabito_swing_s"));
			};
		};
	};
}
	
pub fn install() {
    smashline::install_acmd_scripts!(
		villy_grab,
		villy_dashgrab,
		villy_pivotgrab,
		villy_ftilt,
		villy_dtilt,
		villy_fair,
		villy_fair_snd,
		villy_fair_eff,
		villy_fair_help1,
		villy_fair_help2
    );
    smashline::install_agent_frame_callbacks!(villy);
}
