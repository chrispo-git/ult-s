use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use smash::phx::Vector3f;
use crate::util::*;
use super::*;
use crate::bomberman::*;

pub fn install() {
	Agent::new("pacman")
	.acmd("game_catchbomb", bomb_catch, Priority::Low)
	.acmd("game_catchdashbomb", bomb_catchdash, Priority::Low)
	.acmd("game_catchturnbomb", bomb_catchturn, Priority::Low)
	.acmd("game_catchattackbomb", bomb_catchattack, Priority::Low)
	.acmd("effect_catchbomb", bomb_catch_eff, Priority::Low)
	.acmd("effect_catchdashbomb", bomb_catch_eff, Priority::Low)
	.acmd("effect_catchturnbomb", bomb_catch_eff, Priority::Low)
	.acmd("effect_catchattackbomb", bomb_catchattack_eff, Priority::Low)
	.acmd("game_throwlwbomb", bomb_dthrow, Priority::Low)
	.acmd("effect_throwlwbomb", bomb_dthrow_eff, Priority::Low)
	.acmd("sound_throwlwbomb", bomb_dthrow_snd, Priority::Low)
	.acmd("expression_throwlwbomb", bomb_dthrow_expr, Priority::Low)
	.acmd("game_throwfbomb", bomb_fthrow, Priority::Low)
	.acmd("effect_throwfbomb", bomb_fthrow_eff, Priority::Low)
	.acmd("sound_throwfbomb", bomb_fthrow_snd, Priority::Low)
	.acmd("game_throwbbomb", bomb_bthrow, Priority::Low)
	.acmd("effect_throwbbomb", bomb_bthrow_eff, Priority::Low)
	.acmd("sound_throwbbomb", bomb_bthrow_snd, Priority::Low)
	.acmd("game_throwhibomb", bomb_uthrow, Priority::Low)
	.acmd("effect_throwhibomb", bomb_uthrow_eff, Priority::Low)
	.acmd("sound_throwhibomb", bomb_uthrow_snd, Priority::Low)
	.install();
}

unsafe extern "C" fn bomb_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.2, 0.0, 6.6, 7.3, Some(0.0), Some(6.6), Some(11.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 1.6, 0.0, 6.6, 5.7, Some(0.0), Some(6.6), Some(13.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}
unsafe extern "C" fn bomb_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 2.6, 0.0, 6.7, 4.6, Some(0.0), Some(6.7), Some(11.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 1.3, 0.0, 6.7, 3.3, Some(0.0), Some(6.7), Some(12.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}
unsafe extern "C" fn bomb_catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.2, 0.0, 6.4, -4.0, Some(0.0), Some(6.4), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 1.6, 0.0, 6.4, -2.4, Some(0.0), Some(6.4), Some(-16.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}
unsafe extern "C" fn bomb_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 100, 30, 0, 5.0, 0.0, 10.0, 10.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn bomb_catch_eff(agent: &mut L2CAgentBase) {

}
unsafe extern "C" fn bomb_catchattack_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 11, -2.5, 15, -20, 0, 0.5, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}
unsafe extern "C" fn bomb_dthrow(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 74, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 65, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 31.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 3.0, 45, 90, 0, 30, 5.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        }
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		}
}
unsafe extern "C" fn bomb_dthrow_eff(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
		}
}
unsafe extern "C" fn bomb_dthrow_snd(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
        }
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
		}
}
unsafe extern "C" fn bomb_dthrow_expr(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 30.0);
}
unsafe extern "C" fn bomb_fthrow(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 9.0, /*Angle*/ 45, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 90, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 3.0, 45, 90, 0, 30, 5.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        }
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		}
}
unsafe extern "C" fn bomb_fthrow_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn bomb_fthrow_snd(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
        }
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
		}
}
unsafe extern "C" fn bomb_bthrow(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 45, /*KBG*/ 130, /*FKB*/ 0, /*BKB*/ 50, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
			macros::REVERSE_LR(fighter);
		}
}
unsafe extern "C" fn bomb_bthrow_eff(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 1.0);
}
unsafe extern "C" fn bomb_bthrow_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}
unsafe extern "C" fn bomb_uthrow(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 7.0, /*Angle*/ 90, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		}
}
unsafe extern "C" fn bomb_uthrow_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.925, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn bomb_uthrow_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}