use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use crate::util::*;

//universal static muts
static mut CHARGE : [i32; 8] = [0; 8]; //Your Charge Level
static mut CHARGE_LENIENCY : [i32; 8] = [0; 8]; //Charge Leniency frames
static mut CHARGE_LENIENCY_MAX : i32 = 12; //Max frames of leniency
static mut USE_CHARGE : [bool; 8] = [false; 8]; //When your charge is used
static mut MAX_CHARGE : i32 = 30; //Charge threshold where you can use a charge move
static mut STICK_DIRECTION : [f32; 8] = [0.0; 8];
static mut STICK_NUM : [i32; 8] = [0; 8];
static mut DIR_MULT : f32 = 57.295776842880464966688235343549; //Very fun number that turns direction that spits out ControlModule::get_stick_dir(boma) as an angle in degrees
static mut INPUT_NUM : [i32; 8] = [0; 8];
static mut INPUT_WINDOW : [i32; 8] = [0; 8];
static mut INPUT_START : [bool; 8] = [false; 8];
static mut BAN_CHARGE : [bool; 8] = [false; 8];
static mut INPUT_MAX : i32 = 35;
pub static mut ACTIVATE_MOTION_CHANGE : [bool; 8] = [false; 8];


//char specific
static mut MARTH_IS_COMMAND : [bool; 8] = [false; 8];
static mut GUNNER_TIMER : [i32; 8] = [0; 8]; // Timer till gunner can reverse again
static mut POPO_DANCE : [i32; 8] = [0; 8];
static mut POPO_DANCE_MAX : i32 = 30;
static mut INCIN_BAN_AIRDASH : [bool; 8] = [false; 8];
static mut INCIN_IS_AIRDASH : [bool; 8] = [false; 8];
static mut INCIN_AIRDASH_CANCEL : i32 = 17;
static mut GUNNER_TIMER_MAX : i32 = 90;
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut INC :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 3.5, y: 1.2, z: 0.0 };
static mut TAUNT :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 5.5, y: 0.0, z: 0.0 };



// Use this for general per-frame fighter-level hooks
unsafe extern "C" fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        if StatusModule::is_situation_changed(module_accessor) {
            let situation_kind = &format!("{}", StatusModule::situation_kind(module_accessor));
            println!(
                "[Fighter Hook]\nFighter Kind: {}\nStatus Kind: {}\nSituation Kind: {}",
                get_kind(module_accessor),
                StatusModule::status_kind(module_accessor),
                match StatusModule::situation_kind(module_accessor) {
                    0 => "SITUATION_KIND_GROUND",
                    1 => "SITUATION_KIND_CLIFF",
                    2 => "SITUATION_KIND_AIR",
                    _ => situation_kind
                }
            );
        }
    }
}
//Edge Cancel List
pub(crate) fn can_charge(fighter_kind : i32) -> bool {
	let input_list = [*FIGHTER_KIND_JACK, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_LINK, *FIGHTER_KIND_KIRBY, *FIGHTER_KIND_ROSETTA];
	if input_list.contains(&fighter_kind){
		return true
	} else {
		return false
	}
}
pub(crate) unsafe fn is_attack_btn(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	return (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)  && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW)) &&  
	((ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD)) || 
	StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND )
}

//input_check
unsafe extern "C" fn input_check(fighter : &mut L2CFighterCommon) {
    unsafe {
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
			let fighter_kind = smash::app::utility::get_kind(boma);  
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			STICK_DIRECTION[ENTRY_ID] = ControlModule::get_stick_dir(boma) * DIR_MULT;
			let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
			let stick_y = ControlModule::get_stick_y(boma);
			if ControlModule::get_stick_x(boma) >= -0.2 && ControlModule::get_stick_x(boma) <= 0.2 && ControlModule::get_stick_y(boma) >= -0.2 && ControlModule::get_stick_y(boma) <= 0.2 {
				STICK_DIRECTION[ENTRY_ID] = 361.0;
			} else if STICK_DIRECTION[ENTRY_ID] <= -67.5 {
				STICK_DIRECTION[ENTRY_ID] *= -1.0;
			};
			if STICK_DIRECTION[ENTRY_ID] >= -67.5 && STICK_DIRECTION[ENTRY_ID] < -22.5 && stick_x < 0.0 {
				STICK_NUM[ENTRY_ID] = 1;
				//println!("stick num: 1");
			}else if STICK_DIRECTION[ENTRY_ID] >= 67.5 && STICK_DIRECTION[ENTRY_ID] <= 90.0 && stick_y < 0.0 {
				STICK_NUM[ENTRY_ID] = 2;
				//println!("stick num: 2");
			}else if STICK_DIRECTION[ENTRY_ID] >= -67.5 && STICK_DIRECTION[ENTRY_ID] < -22.5 && stick_x > 0.0 {
					STICK_NUM[ENTRY_ID] = 3;
				//println!("stick num: 3");
			}else if STICK_DIRECTION[ENTRY_ID] >= -22.5 && STICK_DIRECTION[ENTRY_ID] <= 22.5 && stick_x < 0.0 {
				STICK_NUM[ENTRY_ID] = 4;
				//println!("stick num: 4");
			}else if STICK_DIRECTION[ENTRY_ID] == 361.0 {
				STICK_NUM[ENTRY_ID] = 5;
				//println!("stick num: 5");
			}else if STICK_DIRECTION[ENTRY_ID] >= -22.5 && STICK_DIRECTION[ENTRY_ID] <= 22.5 && stick_x > 0.0 {
					STICK_NUM[ENTRY_ID] = 6;
				//println!("stick num: 6");
			}else if STICK_DIRECTION[ENTRY_ID] > 22.5 && STICK_DIRECTION[ENTRY_ID] <= 67.5 && stick_x < 0.0 {
				STICK_NUM[ENTRY_ID] = 7;
				//println!("stick num: 7");
			}else if STICK_DIRECTION[ENTRY_ID] > 67.5 && STICK_DIRECTION[ENTRY_ID] <= 90.0 && stick_y > 0.0 {
				STICK_NUM[ENTRY_ID] = 8;
				//println!("stick num: 8");
			}else  {
					STICK_NUM[ENTRY_ID] = 9;
				//println!("stick num: 9");
			};
    };
}

//Charge_Check
unsafe extern "C" fn charge_check(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		if can_charge(fighter_kind) {
			//println!("Is C-Stick on? {}", ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON)); 
			//println!("Cstick Y? {}", SUB_STICK[ENTRY_ID].y); 
			if /*[*FIGHTER_STATUS_KIND_OTTOTTO, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_F, *FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE, *FIGHTER_STATUS_KIND_WALK_BRAKE].contains(&status_kind)
			   || /*Aerial statuses*/ [*FIGHTER_STATUS_KIND_PASS, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind)
			   || (fighter_kind == *FIGHTER_KIND_PITB && status_kind == 13)*/ true{
				if stick_y <= -0.625 || SUB_STICK[ENTRY_ID].y <= -0.625 || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON){
					if stick_y <= -0.625 {
						CHARGE[ENTRY_ID] += 1;
					};
					CHARGE_LENIENCY[ENTRY_ID] = 0;
				} else {
					CHARGE_LENIENCY[ENTRY_ID] += 1;
					if CHARGE_LENIENCY[ENTRY_ID] >= CHARGE_LENIENCY_MAX {
						CHARGE[ENTRY_ID] = 0;
						CHARGE_LENIENCY[ENTRY_ID] = 0;
					};
				};
			}else{
				CHARGE[ENTRY_ID] = 0;
			};
		};
    };
}


//Charge_Use
unsafe extern "C" fn charge_use(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		let mut scale = ModelModule::scale(boma) * 0.5;
		//Line Checks if you are eligible for charge
		if can_charge(fighter_kind) {
			if [*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_ROSETTA].contains(&fighter_kind){
				scale *= 2.15;
			};
			if CHARGE[ENTRY_ID] == MAX_CHARGE {
				EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("hip"), &HANDS, &HANDS, scale, true, 0, 0, 0, 0, 0, true, true);
			};
			if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_END, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START, *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_JUMP, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON, *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_ITEM, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_BEETLE, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_DRIVER, *FIGHTER_STATUS_KIND_CAPTURE_NABBIT, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP, *FIGHTER_STATUS_KIND_CAPTURE_KAWASAKI, *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU, *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE, *FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE, *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA, *FIGHTER_STATUS_KIND_CAPTURE_MASTERCORE, *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD, *FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_ICE, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_SLIP, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_FINAL, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_REBOUND, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_PASSIVE, *FIGHTER_STATUS_KIND_KILLER, *FIGHTER_STATUS_KIND_ICE_JUMP, *FIGHTER_STATUS_KIND_LAY_DOWN, *FIGHTER_STATUS_KIND_PIT_FALL, *FIGHTER_STATUS_KIND_ROULETTE, *FIGHTER_STATUS_KIND_WARPSTAR, *FIGHTER_STATUS_KIND_BURY_JUMP, *FIGHTER_STATUS_KIND_BURY_WAIT, *FIGHTER_STATUS_KIND_SLIP_WAIT, *FIGHTER_STATUS_KIND_SLEEP_END, *FIGHTER_STATUS_KIND_STOP_CEIL, *FIGHTER_STATUS_KIND_STOP_WALL, *FIGHTER_STATUS_KIND_SWALLOWED, *FIGHTER_STATUS_KIND_YOSHI_EGG, *FIGHTER_STATUS_KIND_KASEY_WARP, *FIGHTER_STATUS_KIND_SLIP_STAND, *FIGHTER_STATUS_KIND_TREAD_FALL, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_DEMON_DIVED, *FIGHTER_STATUS_KIND_DETACH_WALL, *FIGHTER_STATUS_KIND_BITTEN_WARIO, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, *FIGHTER_STATUS_KIND_DRAGOON_RIDE, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_KAMUI_PIERCE, *FIGHTER_STATUS_KIND_PASSIVE_CEIL, *FIGHTER_STATUS_KIND_PASSIVE_WALL, *FIGHTER_STATUS_KIND_REBOUND_JUMP, *FIGHTER_STATUS_KIND_REBOUND_STOP, *FIGHTER_STATUS_KIND_SLIP_STAND_B, *FIGHTER_STATUS_KIND_SLIP_STAND_F, *FIGHTER_STATUS_KIND_SMASH_APPEAL, *FIGHTER_STATUS_KIND_SUICIDE_BOMB, *FIGHTER_STATUS_KIND_TREAD_DAMAGE, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_STAND_FB, *FIGHTER_STATUS_KIND_GENESIS_SHOOT, *FIGHTER_STATUS_KIND_GIMMICK_EATEN, *FIGHTER_STATUS_KIND_GLIDE_LANDING, *FIGHTER_STATUS_KIND_ITEM_STARRING, *FIGHTER_STATUS_KIND_MEWTWO_THROWN].contains(&status_kind) {
				USE_CHARGE[ENTRY_ID] = false;
				MARTH_IS_COMMAND[ENTRY_ID] = false;
				INCIN_IS_AIRDASH[ENTRY_ID] = false;
				ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
				INPUT_NUM[ENTRY_ID] = 0;
				CHARGE[ENTRY_ID] = 0;
			};
			if StatusModule::situation_kind(boma) == *SITUATION_KIND_CLIFF  || [*FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_WAIT, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE].contains(&status_kind){
				CHARGE[ENTRY_ID] = 0;
				USE_CHARGE[ENTRY_ID] = false;
			};
			if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
				BAN_CHARGE[ENTRY_ID] = false;
			};
			if BAN_CHARGE[ENTRY_ID] == true {
				CHARGE[ENTRY_ID] = 0;
			};
			if CHARGE[ENTRY_ID] >= MAX_CHARGE && ([*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_F, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_WALK].contains(&status_kind) || (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 != 0.0 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false)) ){
				if stick_y >= 0.625 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
					USE_CHARGE[ENTRY_ID] = true;
					CHARGE[ENTRY_ID] = 0;
					BAN_CHARGE[ENTRY_ID] = true;
					//println!("Charge being used!: {}", CHARGE[ENTRY_ID]);
				};
			};
			if fighter_kind == *FIGHTER_KIND_KAMUI {
				if CHARGE[ENTRY_ID] >= MAX_CHARGE && fighter_kind == *FIGHTER_KIND_KAMUI && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && stick_y >= 0.625 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK){
					if stick_y >= 0.625 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
						USE_CHARGE[ENTRY_ID] = true;
						CHARGE[ENTRY_ID] = 0;
						BAN_CHARGE[ENTRY_ID] = true;
						//println!("Charge being used by corrin!: {}", CHARGE[ENTRY_ID]);
					};
				};
			};
		};
    };
}
//Char_Charge
unsafe extern "C" fn char_charge(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let situation_kind = StatusModule::situation_kind(boma);
		let frame = MotionModule::frame(boma);
		//Line Checks if you are eligible for charge
		if can_charge(fighter_kind) {
			if fighter_kind == *FIGHTER_KIND_ROSETTA {
				if USE_CHARGE[ENTRY_ID] == true && ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) {
					if situation_kind == *SITUATION_KIND_GROUND {
						if status_kind != *FIGHTER_STATUS_KIND_ATTACK_S3 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
						};
					} else {
						if status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
							WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
						};
					};
				};
				if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) && USE_CHARGE[ENTRY_ID] == true {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("explode"), 1.0, 1.0, false, 0.0, false, false);
						USE_CHARGE[ENTRY_ID] = false;
				};
			};
			/*if fighter_kind == *FIGHTER_KIND_MARIOD {
				if USE_CHARGE[ENTRY_ID] == true && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && USE_CHARGE[ENTRY_ID] == true {
						let stop  = smash::phx::Vector3f { x: 1.0, y: 0.385, z: 1.0 };
						KineticModule::mul_speed(boma, &stop, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
						KineticModule::mul_speed(boma, &stop, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
						if MotionModule::frame(boma) < 3.0 {
							if situation_kind == *SITUATION_KIND_GROUND {
								MotionModule::set_rate(boma, 5.0);
							} else {
								MotionModule::set_rate(boma, 2.0);
							};
						};
						if MotionModule::frame(boma) > 19.0 {
							USE_CHARGE[ENTRY_ID] = false;
						};
				};
			};*/
			if fighter_kind == *FIGHTER_KIND_JACK {
				if USE_CHARGE[ENTRY_ID] == true {
						if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP && USE_CHARGE[ENTRY_ID] == true {
							if frame > 2.0 {
								USE_CHARGE[ENTRY_ID] = false;
							};
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, true);
						};
				};
			};
			if fighter_kind == *FIGHTER_KIND_CAPTAIN {
				if USE_CHARGE[ENTRY_ID] == true  {
						if status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_S_END && USE_CHARGE[ENTRY_ID] == true {
							if frame >= 2.0 {
								USE_CHARGE[ENTRY_ID] = false;
							} else {
								let stop  = smash::phx::Vector3f { x: 0.2, y: 0.2, z: 1.0 };
								KineticModule::mul_speed(boma, &stop, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
								KineticModule::mul_speed(boma, &stop, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
								macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
								macros::PLAY_SE(fighter, Hash40::new("se_captain_swing_ll"));
							};
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_S_END, true);
						};
				};
			};
			//Link Stasis
			if fighter_kind == *FIGHTER_KIND_LINK {
				if USE_CHARGE[ENTRY_ID] == true && !StatusModule::is_situation_changed(boma) {
					if situation_kind == *SITUATION_KIND_GROUND {
						if status_kind != *FIGHTER_STATUS_KIND_ATTACK_S3 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
						};
					} else {
						if status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
							WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
						};
					};
				};
				if [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) && USE_CHARGE[ENTRY_ID] == true {
					if [hash40("special_lw_blast"), hash40("special_air_lw_blast")].contains(&motion_kind) == false {
						if situation_kind == *SITUATION_KIND_GROUND {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_blast"), 0.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_lw_blast"), 0.0, 1.0, false, 0.0, false, false);
						};
					};
					USE_CHARGE[ENTRY_ID] = false;
				};
			};
			//Kirby Spark Copy
			if fighter_kind == *FIGHTER_KIND_KIRBY {
				if USE_CHARGE[ENTRY_ID] == true && !StatusModule::is_situation_changed(boma) {
					if situation_kind == *SITUATION_KIND_GROUND {
						if status_kind != *FIGHTER_STATUS_KIND_ATTACK_S3 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
						};
					} else {
						if status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
							WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
						};
					};
					if [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) && USE_CHARGE[ENTRY_ID] == true {
						if [hash40("special_input")].contains(&motion_kind) == false {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_input"), 0.0, 1.0, false, 0.0, false, false);
						};
						USE_CHARGE[ENTRY_ID] = false;
					};
				};
			};
			//Corrin Roman Cancel
			if fighter_kind == *FIGHTER_KIND_KAMUI {
				if USE_CHARGE[ENTRY_ID] == true && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && USE_CHARGE[ENTRY_ID] == true {
					if [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind) == false {
						if PostureModule::lr(boma) == 1.0 {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("appeal_hi_r"), 1.0, 2.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("appeal_hi_l"), 1.0, 2.0, false, 0.0, false, false);
						};
					} else {
						if MotionModule::frame(boma) >= 17.0 {
							CancelModule::enable_cancel(boma);
							USE_CHARGE[ENTRY_ID] = false;
						};
					};
				};
			};
		} else {
			USE_CHARGE[ENTRY_ID] = false;
		};
    };
}	
//Char_Input
unsafe extern "C" fn char_input(fighter : &mut L2CFighterCommon) {
    unsafe {	
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let mut stick_x = ControlModule::get_stick_x(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		stick_x *= PostureModule::lr(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if true {
			if smash::app::sv_information::is_ready_go() == false {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						MARTH_IS_COMMAND[ENTRY_ID] = false;
						GUNNER_TIMER[ENTRY_ID] = 0;
						USE_CHARGE[ENTRY_ID] = false;
						INCIN_IS_AIRDASH[ENTRY_ID] = false;
						ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
						INCIN_BAN_AIRDASH[ENTRY_ID] = false;
			};
			if !([*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) || ([*FIGHTER_KIND_PIT, *FIGHTER_KIND_PITB, *FIGHTER_KIND_PURIN, *FIGHTER_KIND_KIRBY, *FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DEDEDE].contains(&fighter_kind) && status_kind == 13)) 
			&& MotionModule::frame(boma) >= 2.0 {
				INPUT_NUM[ENTRY_ID] = 0;
			};
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) && [*FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_GAOGAEN].contains(&fighter_kind) == false{
				INPUT_NUM[ENTRY_ID] = 0;
			};
			//Mii Gunner 
			if fighter_kind == *FIGHTER_KIND_MIIGUNNER {
				if [*FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_PASS, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_ESCAPE,*FIGHTER_STATUS_KIND_PASSIVE, *FIGHTER_STATUS_KIND_REBOUND, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CLIFF_WAIT, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DOWN_STAND, *FIGHTER_STATUS_KIND_TREAD_FALL, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_DOWN_DAMAGE, *FIGHTER_STATUS_KIND_SLIP_DAMAGE, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_RUN].contains(&status_kind) == false {
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 1{
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_DIRECTION[ENTRY_ID] <= -35.0 && STICK_DIRECTION[ENTRY_ID] >= -55.0 && stick_x > 0.0 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 && GUNNER_TIMER[ENTRY_ID] == 0 {
					println!("Reversal!");
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					let rev  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
					KineticModule::mul_speed(boma, &rev, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
					EffectModule::req_follow(boma, smash::phx::Hash40::new("miigunner_appeal_lw"), smash::phx::Hash40::new("armr"), &TAUNT, &TAUNT, 2.2, true, 0, 0, 0, 0, 0, true, true);
					macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_attack02"));
					macros::PLAY_SE(fighter, Hash40::new("se_miigunner_special_c3_h01"));
					macros::PLAY_SE(fighter, Hash40::new("se_miigunner_attackdash"));
					GUNNER_TIMER[ENTRY_ID] = GUNNER_TIMER_MAX;
				};
				if GUNNER_TIMER[ENTRY_ID] > 0 {
					GUNNER_TIMER[ENTRY_ID] -= 1;
				};
				if GUNNER_TIMER[ENTRY_ID] == 1 {
					let m1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
					let m2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
					let m3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("footr"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
					let m4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("footl"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, m1, 2.0, 0.75, 3.8);
					EffectModule::set_rgb(boma, m2, 2.0, 0.75, 3.8);
					EffectModule::set_rgb(boma, m3, 2.0, 0.75, 3.8);
					EffectModule::set_rgb(boma, m4, 2.0, 0.75, 3.8);
				};
			};
			//Fox
			if fighter_kind == *FIGHTER_KIND_FOX {
				if true {
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 3{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 6{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if (INPUT_WINDOW[ENTRY_ID] > INPUT_MAX && INPUT_NUM[ENTRY_ID] != 1) || (INPUT_WINDOW[ENTRY_ID] > INPUT_MAX + 3 && INPUT_NUM[ENTRY_ID] == 1){
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && MotionModule::motion_kind(boma) != hash40("attack_hi4"){
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
					macros::COL_NORMAL(fighter);
					MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("attack_hi4"), 0.0, 1.0, 0.0, false, false);
				};
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && MotionModule::motion_kind(boma) == hash40("attack_hi4") {
					if MotionModule::frame(boma) < 7.0 {
						KineticModule::clear_speed_all(boma);
					}else if MotionModule::frame(boma) == 7.0 {
						macros::SET_SPEED_EX(fighter, 0.5, 3.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
				};
				if MotionModule::frame(boma) >= 15.0 && ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true {
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
						ACTIVATE_MOTION_CHANGE[ENTRY_ID] = true;
						//println!("Input 4!");
					};
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
				};
			};
			//Brawler
			/*if fighter_kind == *FIGHTER_KIND_MIIFIGHTER {
				if true {
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 3{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 6{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					let rev  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
					KineticModule::mul_speed(boma, &rev, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
				};
				if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING {
					let stop  = smash::phx::Vector3f { x: 0.5, y: 0.0, z: 1.0 };
					KineticModule::mul_speed(boma, &stop, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK, true);
						//println!("Input 4!");
					};
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK, true);
					println!("Feint Dive Kick!!");
				};
			};*/
			//Palu
			/*if fighter_kind == *FIGHTER_KIND_PALUTENA {
				if true {
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 6{
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 3 {
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false){
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT, true);
						//println!("Input 6!");
					};
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT, true);
					println!("Reflect!");
				};
				if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
			};*/
			//Wii Fit
			/*if fighter_kind == *FIGHTER_KIND_WIIFIT {
				if true {
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 4 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 1{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_NUM[ENTRY_ID] == 3 && STICK_NUM[ENTRY_ID] == 3{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 4!");
					};
					if INPUT_NUM[ENTRY_ID] == 4 && STICK_NUM[ENTRY_ID] == 6 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 5!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 5 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING, true);
						//println!("Input 6!");
					};
				};
				if INPUT_NUM[ENTRY_ID] == 6 && [*FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_LANDING].contains(&status_kind) == false {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING, true);
				};
				if [*FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_LANDING].contains(&status_kind) {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
			};*/
			//Pyra
			/*if fighter_kind == *FIGHTER_KIND_EFLAME {
				if true {
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 3{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 6 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && WorkModule::is_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_HAS_ESWORD){
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
						//println!("Input 4!");
					};
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true && status_kind == *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW && WorkModule::is_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_HAS_ESWORD){
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
					println!("Dancing Edge!");
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW{
					MotionModule::set_rate(boma, 1.75);
				};
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
			};*/
			//Mythra
			/* if fighter_kind == *FIGHTER_KIND_ELIGHT {
				if true {
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 3{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 6 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) {
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
						//println!("Input 4!");
					};
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true && status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_OUT {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2, true);
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true && (status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL || status_kind ==  *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL){
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
					println!("Dancing Edge!");
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW{
					MotionModule::set_rate(boma, 1.75);
				};
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
			};*/
			//Wario
			/*if fighter_kind == *FIGHTER_KIND_WARIO {
      if true{
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 3{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 6 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 {
					WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
						//println!("Input 4!");
					};
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true && MotionModule::motion_kind(boma) != hash40("attack_dash") && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("attack_dash"), 0.0, 1.0, 0.0, false, false);
					macros::SET_SPEED_EX(fighter, 1.35, -0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR  && MotionModule::motion_kind(boma) == hash40("attack_dash") && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) {
					macros::SET_SPEED_EX(fighter, 0.0, 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR  && MotionModule::motion_kind(boma) == hash40("attack_dash") && MotionModule::frame(boma) > 27.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
				};
			};*/
			//Dedede
			/*if fighter_kind == *FIGHTER_KIND_DEDEDE {
				if true{
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 3{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 6 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) && (StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND || StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR){
					if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)  && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)) &&  ((ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD)) || (StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND || StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR)) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						//println!("Input 4!");
					};
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND{
					MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_input"), 0.0, 1.0, 0.0, false, false);
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
					MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_air_input"), 0.0, 1.0, 0.0, false, false);
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI {
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					println!("Dancing Edge!");
				};
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
				/*if  (StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND || StatusModule::is_situation_changed(boma)) && MotionModule::motion_kind(boma) == hash40("special_input"){
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && MotionModule::motion_kind(boma) == hash40("special_input") && MotionModule::frame(boma) >= 45.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
					macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};*/
			};*/
			//Dr Mario
			if fighter_kind == *FIGHTER_KIND_MARIOD {
				if true{
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 1{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 4 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						//println!("Input 4!");
					};
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true {
					MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_input"), 0.0, 1.0, 0.0, false, false);
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI {
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					println!("Dancing Edge!");
				};
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
				/*if  (StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND || StatusModule::is_situation_changed(boma)) && MotionModule::motion_kind(boma) == hash40("special_input"){
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && MotionModule::motion_kind(boma) == hash40("special_input") && MotionModule::frame(boma) >= 45.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
					macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};*/
			};
			//Sheik
			/*if fighter_kind == *FIGHTER_KIND_SHEIK {
				if true{
					if INPUT_NUM[ENTRY_ID] == 0 && (STICK_NUM[ENTRY_ID] == 6 || STICK_NUM[ENTRY_ID] == 9) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] <= 2{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && (STICK_NUM[ENTRY_ID] == 6 || STICK_NUM[ENTRY_ID] == 3) {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						//println!("Input 4!");
					};
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true {
					MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_input"), 0.0, 1.0, 0.0, false, false);
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI {
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					println!("Dancing Edge!");
				};
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
				if  (StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND || StatusModule::is_situation_changed(boma)) && MotionModule::motion_kind(boma) == hash40("special_input"){
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && MotionModule::motion_kind(boma) == hash40("special_input") && MotionModule::frame(boma) >= 45.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
					macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
				if MotionModule::motion_kind(boma) == hash40("special_input") {
					StatusModule::set_keep_situation_air(boma, true);
				};
			};*/
			//Sora
			if fighter_kind == *FIGHTER_KIND_TRAIL {
				if true{
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2{
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 3{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 6 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND{
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
						//println!("Input 4!");
					};
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true {
					MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_input"), 0.0, 1.0, 0.0, false, false);
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI3 {
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
					println!("Dancing Edge!");
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
			};
			//Richter
			/*if fighter_kind == *FIGHTER_KIND_RICHTER {
				if true{
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 3{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 6 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						//println!("Input 4!");
					};
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true {
					MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("attack_dash"), 0.0, 1.0, 0.0, false, false);
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
					macros::SET_SPEED_EX(fighter, 2.0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_NONE);
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI {
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					println!("Dancing Edge!");
				};
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
				if StatusModule::is_situation_changed(boma) && MotionModule::motion_kind(boma) == hash40("attack_dash"){
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && MotionModule::motion_kind(boma) == hash40("attack_dash") && MotionModule::frame(boma) > 30.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
				};
			}; */
			//Kirby
			/*if fighter_kind == *FIGHTER_KIND_KIRBY {
				if true{
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 3{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 6 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
						//println!("Input 4!");
					};
				};
				if ACTIVATE_MOTION_CHANGE[ENTRY_ID] == true {
					MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("attack_dash"), 0.0, 1.0, 0.0, false, false);
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = false;
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && MotionModule::motion_kind(boma) == hash40("attack_dash") {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
					StatusModule::set_keep_situation_air(boma, true);
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
					ACTIVATE_MOTION_CHANGE[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
					println!("Dancing Edge!");
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
				if StatusModule::is_situation_changed(boma) && MotionModule::motion_kind(boma) == hash40("attack_dash"){
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && MotionModule::motion_kind(boma) == hash40("attack_dash") && MotionModule::frame(boma) > 37.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
				};
			};*/
			//Ics
			/*if fighter_kind == *FIGHTER_KIND_POPO {
				if true {
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = true;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 3{
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 6 {
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 3 && (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32 <= MotionModule::frame(boma) || MotionModule::frame(boma) < 4.0) && !AttackModule::is_attack(fighter.module_accessor, 0, false) {
					if is_attack_btn(boma) {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE, true);
						//println!("Input 4!");
					};
				};
				if INPUT_NUM[ENTRY_ID] == 4 && status_kind != *FIGHTER_STATUS_KIND_ESCAPE {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE, true);
					println!("Dancing Edge!");
					MARTH_IS_COMMAND[ENTRY_ID] = true;
				};
				if status_kind == *FIGHTER_STATUS_KIND_ESCAPE {
					INPUT_NUM[ENTRY_ID] = 0;
					INPUT_START[ENTRY_ID] = false;
					INPUT_WINDOW[ENTRY_ID] = 0;
				};
			};*/
			//Incineroar
			/*if fighter_kind == *FIGHTER_KIND_GAOGAEN {
				if [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) {
					if INPUT_NUM[ENTRY_ID] == 0 && STICK_NUM[ENTRY_ID] == 2 && INCIN_BAN_AIRDASH[ENTRY_ID] == false {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 1!");
					};
					if INPUT_NUM[ENTRY_ID] == 1 && STICK_NUM[ENTRY_ID] == 1{
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 2!");
					};
					if INPUT_NUM[ENTRY_ID] == 2 && STICK_NUM[ENTRY_ID] == 4 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_NUM[ENTRY_ID] == 3 && STICK_NUM[ENTRY_ID] == 6 {
						INPUT_WINDOW[ENTRY_ID] = 0;
						INPUT_NUM[ENTRY_ID] += 1;
						//println!("Input 3!");
					};
					if INPUT_WINDOW[ENTRY_ID] > INPUT_MAX {
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
						//println!("Input missed!");
					}; 
				};
				if INPUT_START[ENTRY_ID] == true {
					INPUT_WINDOW[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 4 {
					INPUT_NUM[ENTRY_ID] += 1;
				};
				if INPUT_NUM[ENTRY_ID] == 5 && [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) == false && StatusModule::situation_kind(boma) != *SITUATION_KIND_GROUND {
					println!("Lariat Airdash!");
					INCIN_IS_AIRDASH[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
				};
				if [*FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) == false{
					if INCIN_IS_AIRDASH[ENTRY_ID] == true {
						if MotionModule::motion_kind(boma) != hash40("attack_dash") {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_dash"), 1.6, 1.0, false, 0.0, false, false);
							macros::SET_SPEED_EX(fighter, 0.0, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_NONE);
							EffectModule::req_follow(boma, smash::phx::Hash40::new("gaogaen_chop_arm"), smash::phx::Hash40::new("legl"), &INC, &INC, 2.0, true, 0, 0, 0, 0, 0, true, true);
						} else if MotionModule::frame(boma) >= 14.0 {
							CancelModule::enable_cancel(boma);
							INCIN_BAN_AIRDASH[ENTRY_ID] = true;
							INCIN_IS_AIRDASH[ENTRY_ID] = false;
						};
						INPUT_NUM[ENTRY_ID] = 0;
						INPUT_START[ENTRY_ID] = false;
						INPUT_WINDOW[ENTRY_ID] = 0;
					};
					if MotionModule::motion_kind(boma) == hash40("attack_dash") && StatusModule::situation_kind(boma) != *SITUATION_KIND_GROUND && MotionModule::frame(boma) >= 20.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
						INCIN_BAN_AIRDASH[ENTRY_ID] = true;
					};
				} else {
					INCIN_IS_AIRDASH[ENTRY_ID] = false;
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && INCIN_IS_AIRDASH[ENTRY_ID] == true{
					INCIN_IS_AIRDASH[ENTRY_ID] = false;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					INCIN_BAN_AIRDASH[ENTRY_ID] = false;
				};
			};*/
		};
    };
}	
	

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, charge_check)
	.on_line(Main, charge_use)
	.on_line(Main, char_charge)
	.on_line(Main, input_check)
	.on_line(Main, char_input)
	.install();
}