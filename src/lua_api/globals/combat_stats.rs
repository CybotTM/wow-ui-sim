//! Character combat-rating globals backed by `PlayerState.stats`.

use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn combat_rating_for(state: &mut LuaState, rating_index: i32) -> i32 {
    let Ok(sim) = borrow_state(state) else {
        return 0;
    };
    match rating_index {
        9 => sim.player.stats.crit_rating,
        6 => sim.player.stats.haste_rating,
        26 => sim.player.stats.mastery_rating,
        14 => sim.player.stats.versatility_rating,
        _ => 0,
    }
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
            9 => sim.player.stats.crit_pct(),
            6 => sim.player.stats.haste_pct(),
            26 => sim.player.stats.mastery_pct(),
            14 => sim.player.stats.versatility_pct(),
            _ => 0.0,
        }
    };
    state.push(Val::Num(bonus));
    Ok(1)
}

fn get_crit_chance(state: &mut LuaState) -> LuaResult<u32> {
    let crit = borrow_state(state)?.player.stats.crit_pct() + 5.0;
    state.push(Val::Num(crit));
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

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetCombatRating", get_combat_rating)?;
    LuaApiMut::register_function(lua, "GetCombatRatingBonus", get_combat_rating_bonus)?;
    LuaApiMut::register_function(lua, "GetCritChance", get_crit_chance)?;
    LuaApiMut::register_function(lua, "GetHaste", get_haste)?;
    LuaApiMut::register_function(lua, "GetMasteryEffect", get_mastery_effect)?;
    LuaApiMut::register_function(lua, "GetVersatilityBonus", get_versatility_bonus)?;
    Ok(())
}
