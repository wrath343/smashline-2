/*Head: Do not touch - Encabezado: No tocar*/
use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
		hash40
    },
    smash_script::*,
	smashline::*
};
/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -*/


//------------------------------ HITBOXES ----------------------------------------------------------------------------------------------------


//AIRHITBOXES/////////////////////////////////////////////////////////////////////////

unsafe extern "C" fn game_specialairnend_tanjiro(agent: &mut L2CAgentBase) {/
    if macros::is_excute(agent) {
    //WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
    /*-------->*/macros::SET_SPEED_EX(agent, 1.1, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); //Move
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
    macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 6.8, 20, 110, 0, 68, 4.6, 0.0, 0.0, -1.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 6.8, 20, 110, 0, 68, 4.6, 0.0, 6.2, -1.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 6.8, 20, 110, 0, 68, 4.6, 0.0, 10.6, -1.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
    AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
    //WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn game_specialairnendmax_tanjiro(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    //WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
    /*-------->*/macros::SET_SPEED_EX(agent, 2.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); //Move
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
    macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 50.0, 20, 110, 0, 68, 4.6, 0.0, 0.0, -1.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 50.0, 20, 110, 0, 68, 4.6, 0.0, 6.2, -1.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 50.0, 20, 110, 0, 68, 4.6, 0.0, 10.6, -1.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
    AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
    //WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//HITBOXES/////////////////////////////////////////////////////////////////////////

unsafe extern "C" fn game_specialnend_tanjiro(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
    /*-------->*/macros::SET_SPEED_EX(agent, 1.8, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); //Move
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 13.0, 20, 75, 0, 48, 3.5, 1.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 13.0, 20, 75, 0, 48, 3.0, 0.0, 1.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("bust"), 13.0, 20, 75, 0, 48, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 18.0, 20, 80, 0, 80, 3.5, 1.0, 0.0, 7.3, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);    
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    }


unsafe extern "C" fn game_specialnendmax_tanjiro(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
    /*-------->*/macros::SET_SPEED_EX(agent, 1.8, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); //Move
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 36.0, 20, 75, 0, 48, 3.5, 1.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 36.0, 20, 75, 0, 48, 3.0, 0.0, 1.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("bust"), 36.0, 20, 75, 0, 48, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 36.0, 20, 80, 0, 80, 3.5, 1.0, 0.0, 7.3, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);    
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    }


//AIR EFFECTS/////////////////////////////////////////////////////////////////////////

unsafe extern "C" fn effect_specialairnend_tanjiro(agent: &mut L2CAgentBase) {

    //FIRE TRAIL TEXTURE!
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_sword_red"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_marth_sword3"), Hash40::new("tex_marth_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }

    //FIRE TRAIL!
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ////////
        macros::EFFECT_FOLLOW(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }

    //FIRE TRAIL SLASH!
    frame(agent.lua_state_agent, 9.5);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("roy_blazer_slash"), Hash40::new("top"), 7, 10, 4, 0 /*rotacion eje y 110*/, 0 /*rotacion eje y 180*/, -10, 1.1, 0, 0, 0, 0, 0, 0, true);
    }

    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);   
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 17, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("roy_fire"), false, false);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("marth_sword_red"), false, true);
    }
}

unsafe extern "C" fn effect_specialairnendmax_tanjiro(agent: &mut L2CAgentBase) {

    //FIRE TRAIL TEXTURE!
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_sword_red"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_marth_sword3"), Hash40::new("tex_marth_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }

    //FIRE TRAIL!
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ////////
        macros::EFFECT_FOLLOW(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }


    //FIRE TRAIL SLASH!
    frame(agent.lua_state_agent, 9.5);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("roy_blazer_slash"), Hash40::new("top"), 3, 10, 4, 0 /*rotacion eje y 110*/, 0 /*rotacion eje y 180*/, -10, 1.1, 0, 0, 0, 0, 0, 0, true);
    }

    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 17, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
        //macros::EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_purple"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("marth_sword_red"), false, true);
    }
}

//EFFECTS/////////////////////////////////////////////////////////////////////////

unsafe extern "C" fn effect_specialairnloop_tanjiro(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_sword_red"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("roy_erupution_hold"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::FLASH(agent, 1, 1, 1, 0.863);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.392, 0, 0.941, 0);
        macros::FLASH_FRM(agent, 90, 0.392, 0, 0.941, 0.706);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 8, 0, 0, 0, true);
    }
    wait(agent.lua_state_agent, 5.0);
    macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 8, 0, 0, 0, true);
}

unsafe extern "C" fn effect_specialnloop_tanjiro(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_sword_red"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("roy_erupution_hold"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::FLASH(agent, 1, 1, 1, 0.863);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.392, 0, 0.941, 0);
        macros::FLASH_FRM(agent, 90, 0.392, 0, 0.941, 0.706);
    }
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 8, 0, 7, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 8, 0, 0, 0, true);
    }
    wait(agent.lua_state_agent, 5.0);
    macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 8, 0, 0, 0, true);
}

unsafe extern "C" fn effect_specialnend_tanjiro(agent: &mut L2CAgentBase) { // EFFECT SPECIAL N

    //FIRE TRAIL TEXTURE!
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_sword_red"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_marth_sword3"), Hash40::new("tex_marth_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }

    //FIRE TRAIL!
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ////////
        macros::EFFECT_FOLLOW(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }

    //FIRE TRAIL SLASH!
    frame(agent.lua_state_agent, 9.5);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("roy_blazer_slash"), Hash40::new("top"), 15, 10, 4, 0 /*rotacion eje y 110*/, 0 /*rotacion eje y 180*/, -10, 1.1, 0, 0, 0, 0, 0, 0, true);
    }

    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);   
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 17, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("roy_fire"), false, false);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("marth_sword_red"), false, true);
    }
}

unsafe extern "C" fn effect_specialnendmax_tanjiro(agent: &mut L2CAgentBase) { // EFFECT SPECIAL N MAX

    //FIRE TRAIL TEXTURE!
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_sword_red"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_marth_sword3"), Hash40::new("tex_marth_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }

    //FIRE TRAIL!
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ////////
        macros::EFFECT_FOLLOW(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }


    //FIRE TRAIL SLASH!
    frame(agent.lua_state_agent, 9.5);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("roy_blazer_slash"), Hash40::new("top"), 3, 10, 4, 0 /*rotacion eje y 110*/, 0 /*rotacion eje y 180*/, -10, 1.1, 0, 0, 0, 0, 0, 0, true);
    }

    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 17, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
        //macros::EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_purple"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("marth_sword_red"), false, true);
    }
}

/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -*/
pub fn install() {
	Agent::new("marth")

    //AIRHITBOXES/////////////////////////////////////////////////////////////////////////

    .game_acmd("game_specialairnend_tanjiro_script", game_specialairnend_tanjiro)
    .game_acmd("game_specialairnendmax_tanjiro_script", game_specialairnendmax_tanjiro)

    //HITBOXES/////////////////////////////////////////////////////////////////////////

    .game_acmd("game_specialnend_tanjiro_script", game_specialnend_tanjiro)
    .game_acmd("game_specialnendmax_tanjiro_script", game_specialnendmax_tanjiro)

    //AIR EFFECTS/////////////////////////////////////////////////////////////////////////

    .effect_acmd("effect_specialairnloop_tanjiro_script", effect_specialairnloop_tanjiro)
    .effect_acmd("effect_specialnloop_tanjiro_script", effect_specialnloop_tanjiro)

    //EFFECTS/////////////////////////////////////////////////////////////////////////

    .effect_acmd("effect_specialairnend_tanjiro_script", effect_specialairnend_tanjiro)
    .effect_acmd("effect_specialairnendmax_tanjiro_script", effect_specialairnendmax_tanjiro)

    .effect_acmd("effect_specialnend_tanjiro_script", effect_specialnend_tanjiro)
    .effect_acmd("effect_specialnendmax_tanjiro_script", effect_specialnendmax_tanjiro)

	.install();
}

/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -*/

