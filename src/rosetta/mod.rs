use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use smash::app::ArticleOperationTarget;
use smash::lib::L2CValue;
use crate::util::*;

static mut TICO_X : [f32; 8] = [0.0; 8];
static mut TICO_Y : [f32; 8] = [0.0; 8];
static mut ROSA_X : [f32; 8] = [0.0; 8];
static mut ROSA_Y : [f32; 8] = [0.0; 8];
static mut IS_TELEPORT : [bool; 8] = [false; 8];
static mut IS_TICO_DEAD : [bool; 8] = [false; 8];
static mut INVIS_FRAMES : [i32; 8] = [0; 8];
static mut MAX_INVIS_FRAMES : i32 = 20;
static mut COOLDOWN : [i32; 8] = [0; 8];
static mut TELEPORT_COOLDOWN : i32 = 300;
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

#[acmd_script(
    agent = "rosetta",
    script =  "game_attack11",
    category = ACMD_GAME)]
unsafe fn rosa_jab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=35, FKB=0, BKB=25, Size=3.2, X=0.0, Y=11.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=20, FKB=0, BKB=20, Size=3.2, X=0.0, Y=11.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=20, Size=3.2, X=0.0, Y=11.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			AttackModule::set_add_reaction_frame(ID=0, Frames=6.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=6.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=6.0, Unk=false)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=10)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
    });
}
#[acmd_script(
    agent = "rosetta",
    scripts =  ["game_specialhistart", "game_specialairhistart"],
    category = ACMD_GAME)]
unsafe fn rosa_upb_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=5.0, Angle=367, KBG=100, FKB=90, BKB=0, Size=13.5, X=0.0, Y=11.0, Z=10.0, X2=0.0, Y2=11.0, Z2=15.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
    });
}	
#[acmd_script(
    agent = "rosetta",
    scripts =  ["game_specialhi", "game_specialairhi"],
    category = ACMD_GAME)]
unsafe fn rosa_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			JostleModule::set_status(false)
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=2.5, Angle=367, KBG=100, FKB=90, BKB=0, Size=13.5, X=0.0, Y=11.0, Z=10.0, X2=0.0, Y2=11.0, Z2=15.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		frame(Frame=12)
		if(is_excute){
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
		}
		frame(Frame=17)
		if(is_excute){	
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=8.5, Angle=363, KBG=70, FKB=0, BKB=74, Size=17.5, X=0.0, Y=11.0, Z=10.0, X2=0.0, Y2=11.0, Z2=15.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		frame(Frame=19)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "rosetta",
    script =  "game_attackhi3",
    category = ACMD_GAME)]
unsafe fn rosa_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		FT_MOTION_RATE(FSM=3)
		wait(Frames=1)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=4)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_ROSETTA_GENERATE_ARTICLE_RING, false, 0)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("throw"), Damage=10.0, Angle=88, KBG=65, FKB=0, BKB=75, Size=4.0, X=0.0, Y=0.0, Z=-3.5, X2=0.0, Y2=0.0, Z2=3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("throw"), Damage=8.0, Angle=88, KBG=66, FKB=0, BKB=75, Size=4.0, X=0.0, Y=0.0, Z=-3.5, X2=0.0, Y2=0.0, Z2=3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		frame(Frame=18)
		if(is_excute){
			AttackModule::clear_all()
			ArticleModule::remove(FIGHTER_ROSETTA_GENERATE_ARTICLE_RING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[fighter_frame( agent = FIGHTER_KIND_ROSETTA )]
fn rosa_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let frame = MotionModule::frame(boma);
			if IS_TICO_DEAD[ENTRY_ID] == true || COOLDOWN[ENTRY_ID] > 0{
				CAN_DOWNB[ENTRY_ID] = 1;
			} else {
				CAN_DOWNB[ENTRY_ID] = 0;
			};
			if smash::app::sv_information::is_ready_go() == false {
				COOLDOWN[ENTRY_ID] = 0;
				IS_TICO_DEAD[ENTRY_ID] = false;
			};
			//Teleport!
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && IS_TICO_DEAD[ENTRY_ID] == false && COOLDOWN[ENTRY_ID] == 0{
				if frame > 11.0 && frame < 13.0 {
					macros::EFFECT(fighter, Hash40::new("rosetta_escape"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
					IS_TELEPORT[ENTRY_ID] = true;
					INVIS_FRAMES[ENTRY_ID] = 1;
				};
				if frame > 16.0 && frame < 19.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					VisibilityModule::set_whole(boma, false);
					JostleModule::set_status(boma, false);	
					let pos = smash::phx::Vector3f { x: TICO_X[ENTRY_ID], y: TICO_Y[ENTRY_ID], z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
					INVIS_FRAMES[ENTRY_ID] = 2;
				};
				if frame > 24.0 && frame < 26.0 {
					macros::EFFECT(fighter, Hash40::new("rosetta_escape_end"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
					INVIS_FRAMES[ENTRY_ID] = 3;
				};
				if frame > 25.0{
					VisibilityModule::set_whole(boma, true);
					JostleModule::set_status(boma, true);	
					INVIS_FRAMES[ENTRY_ID] = 4;
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					CancelModule::enable_cancel(boma);
				};
			} else {
				ROSA_Y[ENTRY_ID] = PostureModule::pos_y(boma);
				ROSA_X[ENTRY_ID] = PostureModule::pos_x(boma);
				INVIS_FRAMES[ENTRY_ID] = 0;
			};
			if COOLDOWN[ENTRY_ID] > 0 {
				COOLDOWN[ENTRY_ID] -= 1;
			};
			if COOLDOWN[ENTRY_ID] == 1 {
				smash::app::FighterUtil::flash_eye_info(boma);
				EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
			};
			if status_kind == *FIGHTER_STATUS_KIND_DEAD {
				IS_TICO_DEAD[ENTRY_ID] = false;
			};
	}
}
#[weapon_frame( agent = WEAPON_KIND_ROSETTA_TICO )]
fn tico_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_ROSETTA {
			if IS_TELEPORT[ENTRY_ID] == false {
				TICO_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
				TICO_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
			};
			if [*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, *WEAPON_ROSETTA_TICO_STATUS_KIND_NONE].contains(&status_kind) {
				IS_TICO_DEAD[ENTRY_ID] = true;
			};
			if [*WEAPON_ROSETTA_TICO_STATUS_KIND_REBIRTH].contains(&status_kind) {
				IS_TICO_DEAD[ENTRY_ID] = false;
			};
			println!("TICO_X {}, TICO_Y {}",TICO_X[ENTRY_ID], TICO_Y[ENTRY_ID] );
			if IS_TELEPORT[ENTRY_ID] == true {
				if INVIS_FRAMES[ENTRY_ID] == 1 {
					macros::EFFECT(weapon, Hash40::new("rosetta_escape"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
				};
				if INVIS_FRAMES[ENTRY_ID] == 2 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					VisibilityModule::set_whole(weapon.module_accessor, false);
					JostleModule::set_status(weapon.module_accessor, false);	
					let pos = smash::phx::Vector3f { x: ROSA_X[ENTRY_ID], y: ROSA_Y[ENTRY_ID], z: 0.0 };
					PostureModule::set_pos(weapon.module_accessor, &pos);
					PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
				};
				if INVIS_FRAMES[ENTRY_ID] == 3 {
					macros::EFFECT(weapon, Hash40::new("rosetta_escape_end"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				};
				if INVIS_FRAMES[ENTRY_ID] == 4 {
					JostleModule::set_status(weapon.module_accessor, true);	
					VisibilityModule::set_whole(weapon.module_accessor, true);
					IS_TELEPORT[ENTRY_ID] = false;
					COOLDOWN[ENTRY_ID] = TELEPORT_COOLDOWN;
					INVIS_FRAMES[ENTRY_ID] = 0;
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
			};
		};
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
		rosa_jab,
		rosa_utilt,
		rosa_upb,
		rosa_upb_start
    );
    smashline::install_agent_frames!(
        tico_frame,
		rosa_frame
    );
}
