//! State-backed character combat-rating globals backed by `PlayerState.stats`.

use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

const CRIT_RATING_INDEX: i32 = 9;
const HASTE_RATING_INDEX: i32 = 6;
const MASTERY_RATING_INDEX: i32 = 26;
const VERSATILITY_DAMAGE_RATING_INDEX: i32 = 14;
const VERSATILITY_DAMAGE_TAKEN_RATING_INDEX: i32 = 29;
const SPEED_RATING_INDEX: i32 = 13;
const LIFESTEAL_RATING_INDEX: i32 = 17;
const AVOIDANCE_RATING_INDEX: i32 = 21;

const CRIT_RATING_PER_PERCENT: f64 = 180.0;
const HASTE_RATING_PER_PERCENT: f64 = 170.0;
const MASTERY_RATING_PER_PERCENT: f64 = 130.0;
const VERSATILITY_RATING_PER_PERCENT: f64 = 205.0;

fn combat_rating_for(state: &mut LuaState, rating_index: i32) -> i32 {
    let Ok(sim) = borrow_state(state) else {
        return 0;
    };
    match rating_index {
        CRIT_RATING_INDEX => sim.player.stats.crit_rating,
        HASTE_RATING_INDEX => sim.player.stats.haste_rating,
        MASTERY_RATING_INDEX => sim.player.stats.mastery_rating,
        VERSATILITY_DAMAGE_RATING_INDEX | VERSATILITY_DAMAGE_TAKEN_RATING_INDEX => {
            sim.player.stats.versatility_rating
        }
        _ => 0,
    }
}

fn rating_bonus_for_value(rating_index: i32, rating_value: f64) -> f64 {
    let divisor = match rating_index {
        CRIT_RATING_INDEX => CRIT_RATING_PER_PERCENT,
        HASTE_RATING_INDEX => HASTE_RATING_PER_PERCENT,
        MASTERY_RATING_INDEX => MASTERY_RATING_PER_PERCENT,
        VERSATILITY_DAMAGE_RATING_INDEX | VERSATILITY_DAMAGE_TAKEN_RATING_INDEX => {
            VERSATILITY_RATING_PER_PERCENT
        }
        _ => 180.0,
    };
    (rating_value / divisor).max(0.0)
}

fn get_combat_rating(state: &mut LuaState) -> LuaResult<u32> {
    let rating_index = i32::from_stack(state, 1)?;
    let rating = combat_rating_for(state, rating_index);
    state.push(Val::Num(rating as f64));
    Ok(1)
}

fn get_combat_rating_bonus(state: &mut LuaState) -> LuaResult<u32> {
    let rating_index = i32::from_stack(state, 1)?;
    let bonus = {
        let Ok(sim) = borrow_state(state) else {
            return Ok(0);
        };
        match rating_index {
            CRIT_RATING_INDEX => sim.player.stats.crit_pct(),
            HASTE_RATING_INDEX => sim.player.stats.haste_pct(),
            MASTERY_RATING_INDEX => sim.player.stats.mastery_pct(),
            VERSATILITY_DAMAGE_RATING_INDEX | VERSATILITY_DAMAGE_TAKEN_RATING_INDEX => {
                sim.player.stats.versatility_pct()
            }
            _ => 0.0,
        }
    };
    state.push(Val::Num(bonus));
    Ok(1)
}

fn get_combat_rating_bonus_for_value(state: &mut LuaState) -> LuaResult<u32> {
    let rating_index = i32::from_stack(state, 1)?;
    let rating_value = f64::from_stack(state, 2)?;
    state.push(Val::Num(rating_bonus_for_value(rating_index, rating_value)));
    Ok(1)
}

fn get_crit_chance(state: &mut LuaState) -> LuaResult<u32> {
    let crit = borrow_state(state)?.player.stats.crit_pct() + 5.0;
    state.push(Val::Num(crit));
    Ok(1)
}

fn get_spell_crit_chance(state: &mut LuaState) -> LuaResult<u32> {
    let _school = i32::from_stack(state, 1)?;
    get_crit_chance(state)
}

fn get_ranged_crit_chance(state: &mut LuaState) -> LuaResult<u32> {
    get_crit_chance(state)
}

fn get_crit_chance_provides_parry_effect(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn get_haste(state: &mut LuaState) -> LuaResult<u32> {
    let haste = borrow_state(state)?.player.stats.haste_pct();
    state.push(Val::Num(haste));
    Ok(1)
}

fn get_mastery_effect(state: &mut LuaState) -> LuaResult<u32> {
    let mastery = borrow_state(state)?.player.stats.mastery_pct();
    state.push(Val::Num(8.0 + mastery));
    state.push(Val::Num(mastery));
    Ok(2)
}

fn get_versatility_bonus(state: &mut LuaState) -> LuaResult<u32> {
    let vers = borrow_state(state)?.player.stats.versatility_pct();
    state.push(Val::Num(vers));
    Ok(1)
}

fn get_speed(state: &mut LuaState) -> LuaResult<u32> {
    let rating = combat_rating_for(state, SPEED_RATING_INDEX) as f64;
    state.push(Val::Num(rating_bonus_for_value(SPEED_RATING_INDEX, rating)));
    Ok(1)
}

fn get_lifesteal(state: &mut LuaState) -> LuaResult<u32> {
    let rating = combat_rating_for(state, LIFESTEAL_RATING_INDEX) as f64;
    state.push(Val::Num(rating_bonus_for_value(
        LIFESTEAL_RATING_INDEX,
        rating,
    )));
    Ok(1)
}

fn get_avoidance(state: &mut LuaState) -> LuaResult<u32> {
    let rating = combat_rating_for(state, AVOIDANCE_RATING_INDEX) as f64;
    state.push(Val::Num(rating_bonus_for_value(
        AVOIDANCE_RATING_INDEX,
        rating,
    )));
    Ok(1)
}

fn get_mana_regen(state: &mut LuaState) -> LuaResult<u32> {
    let intellect = borrow_state(state)?.player.stats.intellect.max(0.0);
    let base = 1.0 + intellect / 500.0;
    state.push(Val::Num(base));
    state.push(Val::Num(base * 0.5));
    Ok(2)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetCombatRating", get_combat_rating)?;
    LuaApiMut::register_function(lua, "GetCombatRatingBonus", get_combat_rating_bonus)?;
    LuaApiMut::register_function(
        lua,
        "GetCombatRatingBonusForCombatRatingValue",
        get_combat_rating_bonus_for_value,
    )?;
    LuaApiMut::register_function(lua, "GetCritChance", get_crit_chance)?;
    LuaApiMut::register_function(lua, "GetSpellCritChance", get_spell_crit_chance)?;
    LuaApiMut::register_function(lua, "GetRangedCritChance", get_ranged_crit_chance)?;
    LuaApiMut::register_function(
        lua,
        "GetCritChanceProvidesParryEffect",
        get_crit_chance_provides_parry_effect,
    )?;
    LuaApiMut::register_function(lua, "GetHaste", get_haste)?;
    LuaApiMut::register_function(lua, "GetMasteryEffect", get_mastery_effect)?;
    LuaApiMut::register_function(lua, "GetVersatilityBonus", get_versatility_bonus)?;
    LuaApiMut::register_function(lua, "GetSpeed", get_speed)?;
    LuaApiMut::register_function(lua, "GetLifesteal", get_lifesteal)?;
    LuaApiMut::register_function(lua, "GetAvoidance", get_avoidance)?;
    LuaApiMut::register_function(lua, "GetManaRegen", get_mana_regen)?;
    Ok(())
}
