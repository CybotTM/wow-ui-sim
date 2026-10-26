//! Native 12340 reset/validation paths with an absent profession filter catalog.
//! Retire when legacy profession subclass rows and inventory masks are modeled.
//! Modern recipe fixtures do not provide that catalog; no ordinal is fabricated.
use crate::lua_api::globals::real::wrath_spellbook::numeric_argument;
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_api::script_helpers::fire_named_event_state;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

pub fn register(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "GetTradeSkillSubClasses", empty_catalog)?;
    LuaApiMut::register_function(lua, "GetTradeSkillInvSlots", empty_catalog)?;
    LuaApiMut::register_function(lua, "GetTradeSkillSubClassFilter", subclass_filter)?;
    LuaApiMut::register_function(lua, "GetTradeSkillInvSlotFilter", inventory_filter)?;
    LuaApiMut::register_function(lua, "SetTradeSkillSubClassFilter", reset_subclass)?;
    LuaApiMut::register_function(lua, "SetTradeSkillInvSlotFilter", reset_inventory)
}

// Fresh native catalog counters/eligible slots are zero; filter masks are FFFFFFFF.
// Neither native catalog getter checks window-open
// state; when a populated catalog is modeled, it must survive CloseTradeSkill.
fn empty_catalog(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn subclass_filter(state: &mut LuaState) -> LuaResult<u32> {
    let index = read_filter_index(state, "Usage: GetTradeSkillSubClassFilter(index)")?;
    if index >= 0 {
        return Err(runtime_error(
            "Bad sub class in GetTradeSkillSubClassFilter",
        ));
    }
    // With no subclass records, all current relevant bits are vacuously set.
    push_filter_result(state, true)
}

fn inventory_filter(state: &mut LuaState) -> LuaResult<u32> {
    let index = read_filter_index(state, "Usage: GetTradeSkillInvSlotFilter(index)")?;
    let mask = borrow_state(state)?
        .wrath_legacy
        .trade_skill_filters
        .inventory_mask;
    let all_slots = mask == u32::MAX;
    if index < 0 {
        return push_filter_result(state, all_slots);
    }
    if all_slots {
        // Native sentinel exits before validating the individual ordinal.
        return push_filter_result(state, false);
    }
    Err(runtime_error("Bad inv type in GetTradeSkillInvSlotFilter"))
}

fn read_filter_index(state: &LuaState, usage: &str) -> LuaResult<i64> {
    let value = numeric_argument(state, 1).ok_or_else(|| runtime_error(usage))?;
    Ok(i64::from(value as i32) - 1)
}

fn push_filter_result(state: &mut LuaState, checked: bool) -> LuaResult<u32> {
    state.push(if checked { Val::Num(1.0) } else { Val::Nil });
    Ok(1)
}

fn reset_subclass(state: &mut LuaState) -> LuaResult<u32> {
    validate_reset(
        state,
        "Usage: SetTradeSkillSubClassFilter(index [, on\\off, exclusive])",
        "Bad sub class in SetTradeSkillSubClassFilter",
    )?;
    {
        let mut sim = borrow_state_mut(state)?;
        let filters = &mut sim.wrath_legacy.trade_skill_filters;
        filters.subclass_mask = u32::MAX;
        filters.selected_subclass = u32::MAX;
        // SetInvTypeFilter(1) rebuilds the list; it does not assign inventory_mask.
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
        .trade_skill_filters
        .inventory_mask = u32::MAX;
    fire_named_event_state(state, "TRADE_SKILL_UPDATE", &[]);
    Ok(0)
}

fn validate_reset(state: &LuaState, usage: &str, bad_ordinal: &str) -> LuaResult<()> {
    if !matches!(stack_val(state, 1), Val::Str(_) | Val::Num(_)) {
        return Err(runtime_error(usage));
    }
    // lua_tonumber maps nonnumeric strings to zero, so they reset too.
    let ordinal = numeric_argument(state, 1).unwrap_or(0.0) as i32;
    if ordinal > 0 {
        return Err(runtime_error(bad_ordinal));
    }
    Ok(())
}
