//! Modifier-key probes backed by `SimState::modifier_keys`.
//!
//! WoW exposes five globals: `IsShiftKeyDown` / `IsControlKeyDown` /
//! `IsAltKeyDown` / `IsMetaKeyDown` read individual keys; `IsModifierKeyDown`
//! returns true iff any of shift/control/alt is held (meta is *not* included
//! — Blizzard keeps that on its own probe).
//!
//! All keys default to `false` in the sim (no real input). Admin API
//! `A_Admin.SetShiftKeyDown(b)` / `SetControlKeyDown` / `SetAltKeyDown` /
//! `SetMetaKeyDown` toggles them for tests that want to exercise
//! modifier-aware UI paths (e.g. `IsModifiedClick("CHATLINK")`).

use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn is_shift_key_down(state: &mut LuaState) -> LuaResult<u32> {
    let down = borrow_state(state)?.modifier_keys.shift;
    state.push(Val::Bool(down));
    Ok(1)
}

pub fn is_control_key_down(state: &mut LuaState) -> LuaResult<u32> {
    let down = borrow_state(state)?.modifier_keys.control;
    state.push(Val::Bool(down));
    Ok(1)
}

pub fn is_alt_key_down(state: &mut LuaState) -> LuaResult<u32> {
    let down = borrow_state(state)?.modifier_keys.alt;
    state.push(Val::Bool(down));
    Ok(1)
}

pub fn is_meta_key_down(state: &mut LuaState) -> LuaResult<u32> {
    let down = borrow_state(state)?.modifier_keys.meta;
    state.push(Val::Bool(down));
    Ok(1)
}

pub fn is_modifier_key_down(state: &mut LuaState) -> LuaResult<u32> {
    let any = borrow_state(state)?.modifier_keys.any_modifier();
    state.push(Val::Bool(any));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let g = state.global;
    table_set_rust_fn(state, g, "IsShiftKeyDown", is_shift_key_down)?;
    table_set_rust_fn(state, g, "IsControlKeyDown", is_control_key_down)?;
    table_set_rust_fn(state, g, "IsAltKeyDown", is_alt_key_down)?;
    table_set_rust_fn(state, g, "IsMetaKeyDown", is_meta_key_down)?;
    table_set_rust_fn(state, g, "IsModifierKeyDown", is_modifier_key_down)?;
    Ok(())
}
