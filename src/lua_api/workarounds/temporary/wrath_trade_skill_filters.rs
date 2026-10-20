//! Native 12340 reset/validation paths with an absent profession filter catalog.
//! Retire when legacy profession subclass rows and inventory masks are modeled.
//! Modern recipe fixtures do not provide that catalog; no ordinal is fabricated.
use crate::lua_api::globals::real::wrath_spellbook::numeric_argument;
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_api::script_helpers::fire_named_event_state;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

pub fn register(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "SetTradeSkillSubClassFilter", reset_subclass)?;
    LuaApiMut::register_function(lua, "SetTradeSkillInvSlotFilter", reset_inventory)
}

fn reset_subclass(state: &mut LuaState) -> LuaResult<u32> {
    validate_reset(
        state,
        "Usage: SetTradeSkillSubClassFilter(index [, on\\off, exclusive])",
        "Bad sub class in SetTradeSkillSubClassFilter",
    )?;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.wrath_legacy.trade_skill_subclass_filter = Some((u32::MAX, u32::MAX));
        sim.wrath_legacy.trade_skill_inventory_filter = Some(1);
    }
    fire_named_event_state(state, "TRADE_SKILL_UPDATE", &[]);
    Ok(0)
}

fn reset_inventory(state: &mut LuaState) -> LuaResult<u32> {
    validate_reset(
        state,
        "Usage: SetTradeSkillInvSlotFilter(index [, on\\off, exclusive])",
        "Bad inv slot in SetTradeSkillInvSlotFilter",
    )?;
    borrow_state_mut(state)?
        .wrath_legacy
        .trade_skill_inventory_filter = Some(u32::MAX);
    fire_named_event_state(state, "TRADE_SKILL_UPDATE", &[]);
    Ok(0)
}

fn validate_reset(state: &LuaState, usage: &str, bad_ordinal: &str) -> LuaResult<()> {
    if !matches!(stack_val(state, 1), Val::Str(_) | Val::Num(_)) {
        return Err(runtime_error(usage));
    }
    // lua_tonumber maps nonnumeric strings to zero, so they reset too.
    let ordinal = numeric_argument(state).unwrap_or(0.0) as i32;
    if ordinal > 0 {
        return Err(runtime_error(bad_ordinal));
    }
    Ok(())
}
