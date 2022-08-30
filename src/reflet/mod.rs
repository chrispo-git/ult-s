use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;



//Grima Install
static mut IS_GRIMA : [bool; 8] = [false; 8];
static mut DMG_COUNTER : [i32; 8] = [0; 8];
static mut DMG_COUNTER_MAX : i32 = 60;
static mut DMG_ADD : f32 = 1.5;

//Float Stuff
static mut FLOAT : [i32; 8] = [0; 8]; //Logs Float Time
static mut START_FLOAT : [bool; 8] = [false; 8];
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
static mut CHECK_FLOAT_MAX : i32 = 15; //Frames where jump needs to be held to start floating
static mut X : [f32; 8] = [0.0; 8]; //Logs speed
static mut Y : [f32; 8] = [0.0; 8]; //Logs speed
static mut FLOAT_MAX : i32 = 95; //Frames this bitch can float (In frames, 60 frames = 1 second)
static mut X_MAX : f32 = 1.375; //Max Horizontal movespeed
static mut X_ACCEL_ADD : f32 = 0.06; //Air Accel Add
static mut X_ACCEL_MUL : f32 = 0.12; //Air Accel Mul
static mut Y_MAX : f32 = 0.65; //Max Vertical movespeed
static mut Y_ACCEL_ADD : f32 = 0.06;
static mut Y_ACCEL_MUL : f32 = 0.06;

#[acmd_script(
    agent = "reflet",
    script =  "game_attacklw3",
    category = ACMD_GAME)]
unsafe fn robin_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=75, KBG=35, FKB=0, BKB=70, Size=3.5, X=0.0, Y=3.0, Z=11.0, X2=0.0, Y2=5.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=9)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "reflet",
    script =  "game_attacks3",
    category = ACMD_GAME)]
unsafe fn robin_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=70, FKB=0, BKB=85, Size=5.0, X=0.0, Y=8.5, Z=14.0, X2=0.0, Y2=8.5, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=11)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "reflet",
    scripts =  ["effect_speciallwstart", "effect_specialairlwstart"],
    category = ACMD_EFFECT)]
unsafe fn robin_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "reflet",
    scripts =  ["effect_speciallwend", "effect_specialairlwend"],
    category = ACMD_EFFECT)]
unsafe fn robin_downb_end_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "reflet",
    scripts =  ["sound_speciallwstart", "sound_specialairlwstart"],
    category = ACMD_SOUND)]
unsafe fn robin_downb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "reflet",
    scripts =  ["game_speciallwstart", "game_specialairlwstart"],
    category = ACMD_GAME)]
unsafe fn robin_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=2)
		frame(Frame=7)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=14)
		FT_MOTION_RATE(FSM=1.5)
    });
}	
#[fighter_frame_callback]
pub fn robin(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		let speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma);
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		if fighter_kind == *FIGHTER_KIND_REFLET {
			WorkModule::set_int(boma, 10, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
			if [hash40("special_hi"), hash40("special_air_hi")].contains(&motion_kind) && IS_GRIMA[ENTRY_ID] == false {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) && MotionModule::frame(boma) >= 8.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
				};
			};
			if [hash40("special_lw_start"), hash40("special_air_lw_start"), hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) {
				if MotionModule::frame(boma) == 8.0 {
					if IS_GRIMA[ENTRY_ID] == true {
						IS_GRIMA[ENTRY_ID] = false;
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.725, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.15, 1.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.15, 1.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.15, 1.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
					} else {
						IS_GRIMA[ENTRY_ID] = true;
						macros::FT_ADD_DAMAGE(fighter, DMG_ADD*5.0);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.725, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 0.25, 0.0, 3.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 0.25, 0.0, 3.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 0.25, 0.0, 3.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
					};
					acmd!(lua_state, {PLAY_FLY_VOICE(hash40("seq_reflet_rnd_futtobi01"), hash40("seq_reflet_rnd_futtobi02"))});
					macros::PLAY_SE(fighter, Hash40::new("se_reflet_special_l01"));
					if StatusModule::is_situation_changed(boma) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					};
				};
				if end_frame-frame < 5.0 && situation_kind != *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
				};
				if  end_frame-frame < 3.0 && situation_kind == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
			};
			if status_kind == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END {
					if situation_kind == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
					};
			};
			if StatusModule::situation_kind(boma) != SITUATION_KIND_AIR {
				FLOAT[ENTRY_ID] = 0;
				START_FLOAT[ENTRY_ID] = false;
				CHECK_FLOAT[ENTRY_ID] = 0;
			};
			if smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
				IS_GRIMA[ENTRY_ID] = false;
				DMG_COUNTER[ENTRY_ID] = DMG_COUNTER_MAX;
			};
			if IS_GRIMA[ENTRY_ID] == true {
				if DMG_COUNTER[ENTRY_ID] > 0 {
					DMG_COUNTER[ENTRY_ID] -= 1;
				} else {
					DMG_COUNTER[ENTRY_ID] = DMG_COUNTER_MAX;
					macros::FT_ADD_DAMAGE(fighter, DMG_ADD);
				};
				//Dash Speed
				if [*FIGHTER_STATUS_KIND_DASH].contains(&status_kind) {
					if MotionModule::frame(boma) == 3.0 {
							let speed = smash::phx::Vector3f { x: 0.2, y: 0.0, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
					};
				};
				//Effect Code
				if DMG_COUNTER[ENTRY_ID] % 40 == 0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.375, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.25, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
					if FLOAT[ENTRY_ID] > 1 {
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.2, true);
					};
				};
				//Landing Lag Reduce Code
				if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
					if motion_kind == hash40("landing_air_f") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_f"), 0)-2.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					} else if motion_kind == hash40("landing_air_b") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_b"), 0)-2.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					} else if motion_kind == hash40("landing_air_hi") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_hi"), 0)-2.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					} else if motion_kind == hash40("landing_air_lw") {
						let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_lw"), 0)-2.0))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
						MotionModule::set_rate(boma, landing);
					};
				};
				//Float Code
				if FLOAT[ENTRY_ID] == 1{
					if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_MOTION_AIR && [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) == false {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
					};
				};
				if situation_kind == *SITUATION_KIND_AIR && (!(*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) && status_kind != *FIGHTER_STATUS_KIND_FALL_SPECIAL){
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
						CHECK_FLOAT[ENTRY_ID] += 1;
					} else {
						CHECK_FLOAT[ENTRY_ID] = 0;
					};
					if CHECK_FLOAT[ENTRY_ID] >= CHECK_FLOAT_MAX && FLOAT[ENTRY_ID] == 0 {
						START_FLOAT[ENTRY_ID] = true;
					};
				};
				if [
					*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_SPECIAL_N, 
					*FIGHTER_STATUS_KIND_SPECIAL_S,*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW,
					*FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL,
					*FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_JUMP_CANCEL
				].contains(&status_kind) && FLOAT[ENTRY_ID] > 1{
					FLOAT[ENTRY_ID] = 1;
				};
				if FLOAT[ENTRY_ID] > 1 {
					FLOAT[ENTRY_ID] -= 1;
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
					if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) {
						FLOAT[ENTRY_ID] = 1;
					};
					if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						FLOAT[ENTRY_ID] = 1;
					};
					let mut y_add = 0.0;
					let mut x_add = 0.0;
					if stick_x > 0.2 {
						x_add = ((stick_x-0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
						if speed_x > X_MAX || speed_x < -X_MAX{{}
							x_add = 0.0;
						};
					};
					if stick_x < -0.2 {
						x_add = ((stick_x+0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
						if speed_x > X_MAX || speed_x < -X_MAX{
							x_add = 0.0;
						};
					};
					if stick_y > 0.2 {
						y_add = ((stick_y-0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
						if speed_y > Y_MAX || speed_y < -Y_MAX{
							y_add = 0.0;
						};
					};
					if stick_y < -0.2 {
						y_add = ((stick_y+0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
						if speed_y > Y_MAX || speed_y < -Y_MAX{
							y_add = 0.0;
						};
					};
					if stick_x > -0.2 && stick_x < 0.2 && stick_y > -0.2 && stick_y < 0.2 {
						if speed_y > 0.0 {
							y_add = -Y_ACCEL_MUL - Y_ACCEL_ADD;
						} else if speed_y < 0.0{
							y_add = Y_ACCEL_MUL + Y_ACCEL_ADD;
						};
						let mut x_add = 0.0;
						if speed_x > 0.0 {
							x_add = -X_ACCEL_MUL - X_ACCEL_ADD;
						} else if speed_x < 0.0{
							x_add = X_ACCEL_MUL + X_ACCEL_ADD;
						};
					};
					x_add = (stick_x)*X_ACCEL_MUL;
					y_add = (stick_y)*X_ACCEL_MUL;
					if x_add > 0.0 && X[ENTRY_ID] > X_MAX {
						x_add = 0.0;
					};
					if x_add < 0.0 && X[ENTRY_ID] < X_MAX*-1.0 {
						x_add = 0.0;
					};
					if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
						y_add = 0.0;
					};
					if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*-1.0 {
						y_add = 0.0;
					};
					println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
					println!("x_add{}, y_add{}", x_add, y_add);
					X[ENTRY_ID] += x_add;
					Y[ENTRY_ID] += y_add;
					macros::SET_SPEED_EX(fighter, X[ENTRY_ID], Y[ENTRY_ID], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				} else {
					X[ENTRY_ID] = 0.0;
					Y[ENTRY_ID] = 0.0;
				};
				if status_kind == *FIGHTER_STATUS_KIND_THROW {
					if FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 > 6.0 {
						if MotionModule::frame(boma) >= (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 - 6.0) {
							CancelModule::enable_cancel(boma);
						};
					};
				};
				if START_FLOAT[ENTRY_ID] == true {
					FLOAT[ENTRY_ID] = FLOAT_MAX;
					START_FLOAT[ENTRY_ID] = false;
					if status_kind == *FIGHTER_STATUS_KIND_JUMP {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
					if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
				};
				if [*FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_JUMP_CANCEL, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_FAIL, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
					MotionModule::set_rate(boma, 0.75);
				};
			};
		};
    };
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
		robin_dtilt,
		robin_ftilt,
		robin_downb_eff,
		robin_downb_snd,
		robin_downb
    );
	smashline::install_agent_frame_callbacks!(robin);
}
