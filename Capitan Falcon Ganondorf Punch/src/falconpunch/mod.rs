use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*
};
//START HEAD ----------------------------------------------------------------------------------------

unsafe extern "C" fn captain_game_specialn(agent: &mut L2CAgentBase) {

        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0); //SuperArmor
        }
  
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
        }
        frame(agent.lua_state_agent, 53.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
            macros::ATTACK(agent, /*do not edit*/
                0,  /*id*/
                0, /*part*/
                Hash40::new("armr"), /*bone*/
                25.0,  /*damage*/ //Original 25/
                20,  /*angle*/ //Original 361/
                59, 0, 93,  /*kgb, fkb, bkb*/ //Original 59,  0, 93, /
                5.0,  /*size*/ //Original 4.5 /
                -1.0,  0.0, 0.0, /*position*/ 
                None, None, None, /*position 2*/
                1.5,  /*hitlag*/
                0.0,  /*sdi*/
                *ATTACK_SETOFF_KIND_OFF,  /*clang rebound*/
                *ATTACK_LR_CHECK_POS,  /*facing restriction*/
                false,  /*set weight*/
                1.8, /*shield damage*/ //Original 0/
                0.0, /*trip chance*/
                0, /*rehit*/
                false,  /*reflectable*/
                false,  /*absorbable*/
                false,  /*flincheness*/
                false, /*disable hitlag*/
                true,  /*direct hitbox*/
                *COLLISION_SITUATION_MASK_GA, /*ground or air*/
                *COLLISION_CATEGORY_MASK_ALL,  /*hitbits*/
                *COLLISION_PART_MASK_ALL, /*collision part*/ 
                false,  /*friendly fire*/
                //Hash40::new("collision_attr_fire"),  /*effect*/
                Hash40::new("collision_attr_purple"),  /*effect*/
                *ATTACK_SOUND_LEVEL_L,  /*sfx level*/
                *COLLISION_SOUND_ATTR_KICK,  /*sfx type*/
                *ATTACK_REGION_PUNCH); /*type*/

            macros::ATTACK(agent,
                1,   /*id*/
                0,  /*part*/
                Hash40::new("shoulderr"),  /*bone*/
                25.0,  /*damage*/ //Original 25/
                20,  /*angle*/ //Original 361/ 
                59, 0, 93,   /*kgb, fkb, bkb*/  //Original 59,  0, 93, /
                4.7,   /*size*/  //Original 2.5 / 
                -2.5,  0.0,  0.0,  /*position*/ 
                None, None, None,  /*position 2*/ 
                1.5,   /*hitlag*/
                0.0,   /*sdi*/
                *ATTACK_SETOFF_KIND_OFF,   /*clang rebound*/
                *ATTACK_LR_CHECK_POS,   /*facing restriction*/
                false,   /*set weight*/
                1.8,  /*shield damage*/  //Original 0/
                0.0,  /*trip chance*/
                0,  /*rehit*/
                false,   /*reflectable*/
                false,   /*absorbable*/
                false,   /*flincheness*/
                false,  /*disable hitlag*/
                true,   /*direct hitbox*/
                *COLLISION_SITUATION_MASK_GA, /*ground or air*/
                *COLLISION_CATEGORY_MASK_ALL,   /*hitbits*/
                *COLLISION_PART_MASK_ALL,  /*collision part*/ 
                false,   /*friendly fire*/
                //Hash40::new("collision_attr_fire"),  /*effect*/
                Hash40::new("collision_attr_purple"),  /*effect*/
                *ATTACK_SOUND_LEVEL_L,   /*sfx level*/
                *COLLISION_SOUND_ATTR_KICK,   /*sfx type*/
                *ATTACK_REGION_PUNCH); /*type*/

            macros::ATTACK(agent, 
                2,  /*id*/
                0, /*part*/
                Hash40::new("armr"),  /*bone*/
                25.0,   /*damage*/ //Original 25/
                20,   /*angle*/ //Original 361/
                59, 0, 93,   /*kgb, fkb, bkb*/ //Original 59,  0, 93, /
                4.0,  /*size*/
                4.2, 0.0,  0.0,  /*position*/
                None, None, None,  /*position 2*/
                1.5,   /*hitlag*/
                0.0,  /*sdi*/
                *ATTACK_SETOFF_KIND_OFF,  /*clang rebound*/
                *ATTACK_LR_CHECK_POS,  /*facing restriction*/
                false,   /*set weight*/
                1.8,  /*shield damage*/  //Original 0/
                0.0,  /*trip chance*/
                0,  /*rehit*/
                false,   /*reflectable*/
                false,   /*absorbable*/
                false,   /*flincheness*/
                false,  /*disable hitlag*/
                true,   /*direct hitbox*/
                *COLLISION_SITUATION_MASK_GA, /*ground or air*/
                *COLLISION_CATEGORY_MASK_ALL,   /*hitbits*/
                *COLLISION_PART_MASK_ALL,  /*collision part*/ 
                false,   /*friendly fire*/
                //Hash40::new("collision_attr_fire"),  /*effect*/
                Hash40::new("collision_attr_purple"),  /*effect*/
                *ATTACK_SOUND_LEVEL_L,   /*sfx level*/
                *COLLISION_SOUND_ATTR_KICK, /*sfx type*/
                *ATTACK_REGION_PUNCH); /*type*/

                
        }
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 2.5, -3.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
                macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 5.0, 1.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
                macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 4.0, 5.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            }
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 0);
        }
}
    
unsafe extern "C" fn captain_effect_specialn(agent: &mut L2CAgentBase) {

        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("captain_fp_flash"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("captain_fp_hold"), Hash40::new("haver"), 0, 0, 0, 3.119, -0.79, -0.543, 1, true);
        }
        frame(agent.lua_state_agent, 33.0);
        for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("captain_fp_hold"), false, true);
    }
}

//END INSTALL ----------------------------------------------------------------------------------------------------------------
pub fn install() {
	Agent::new("captain")
		.game_acmd("game_attackairlw", captain_game_specialn)
        .effect_acmd("effect_attackairlw", captain_effect_specialn)
		.install();
}