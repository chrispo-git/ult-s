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

pub fn install() {

    Agent::new("trail_thunder")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_fall", thundaga_1, Priority::Low)    
	.acmd("game_fallair", thundaga_2, Priority::Low)    
	.acmd("game_falllast", thundaga_3, Priority::Low)    
	.game_acmd("0x10983531cc", thundaga_4, Priority::Low)    
	.install();
}

unsafe extern "C" fn thundaga_1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 10, /*KBG*/ 26, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 50, /*KBG*/ 26, /*FKB*/ 0, /*BKB*/ 94, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
        }
}
unsafe extern "C" fn thundaga_2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 344, /*KBG*/ 26, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 68, /*KBG*/ 26, /*FKB*/ 0, /*BKB*/ 98, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
        }
}
unsafe extern "C" fn thundaga_3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 64, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 64, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
        }
        wait(fighter.lua_state_agent, 10.0);
}
unsafe extern "C" fn thundaga_4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 64, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 64, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
        }
        wait(fighter.lua_state_agent, 10.0);
}