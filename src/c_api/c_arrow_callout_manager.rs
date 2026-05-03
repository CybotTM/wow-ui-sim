//! `C_ArrowCalloutManager` surface consumed by
//! `Blizzard_ArrowCalloutFrame/ArrowCalloutFrame.lua`.
//!
//! State source: `state.arrow_callouts` (`ArrowCalloutState`).
//!
//! - `ShowCallout(calloutInfo)` registers the callout in the live set
//!   keyed by `calloutID` and fires `SHOW_ARROW_CALLOUT` through the
//!   simulator event queue with the original `calloutInfo` table as
//!   the payload (mirrors `ArrowCalloutFrameManager:OnEvent` at
//!   lua:78-83).
//! - `HideCallout(calloutID)` clears the entry and fires
//!   `HIDE_ARROW_CALLOUT` with the id; no-op if the id was not active.
//! - `AcknowledgeCallout(calloutID)` is the only function the addon
//!   itself calls (close button OnClick at lua:174). It records the
//!   id in the persistent acknowledged set, hides the callout, and
//!   round-trips the union into the `acknowledgedArrowCallouts` cvar
//!   so a reload preserves dismissals.
//! - `IsCalloutActive` / `IsCalloutAcknowledged` are simple lookups
//!   exposed for the addon (and tests) to probe state without touching
//!   the underlying tables.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{borrow_state_mut, table_get, val_to_string};
use crate::lua_api::sim_substates::ArrowCalloutInfo;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_arrow_callout_manager_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_ArrowCalloutManager")?;
    table_set_rust_fn_static(state, ns, "ShowCallout", show_callout)?;
    table_set_rust_fn_static(state, ns, "HideCallout", hide_callout)?;
    table_set_rust_fn_static(state, ns, "AcknowledgeCallout", acknowledge_callout)?;
    table_set_rust_fn_static(state, ns, "IsCalloutActive", is_callout_active)?;
    table_set_rust_fn_static(state, ns, "IsCalloutAcknowledged", is_callout_acknowledged)?;
    Ok(())
}

fn show_callout(state: &mut LuaState) -> LuaResult<u32> {
    let payload = stack_val(state, 1);
    let Some(info) = read_callout_info(state, payload) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let callout_id = info.callout_id;
    borrow_state_mut(state)?
        .arrow_callouts
        .active
        .insert(callout_id, info);
    dispatch_event_now(state, "SHOW_ARROW_CALLOUT", &[payload])?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn hide_callout(state: &mut LuaState) -> LuaResult<u32> {
    let Some(callout_id) = read_callout_id(state, 1) else {
        return Ok(0);
    };
    let was_active = borrow_state_mut(state)?
        .arrow_callouts
        .active
        .remove(&callout_id)
        .is_some();
    if was_active {
        dispatch_event_now(state, "HIDE_ARROW_CALLOUT", &[Val::Num(callout_id as f64)])?;
    }
    Ok(0)
}

fn acknowledge_callout(state: &mut LuaState) -> LuaResult<u32> {
    let Some(callout_id) = read_callout_id(state, 1) else {
        return Ok(0);
    };
    sync_acknowledged_from_cvar(state)?;
    let cvar_value = {
        let mut sim = borrow_state_mut(state)?;
        sim.arrow_callouts.acknowledged.insert(callout_id);
        sim.arrow_callouts.acknowledged_cvar_value()
    };
    persist_acknowledged_cvar(state, &cvar_value)?;
    let was_active = borrow_state_mut(state)?
        .arrow_callouts
        .active
        .remove(&callout_id)
        .is_some();
    if was_active {
        dispatch_event_now(state, "HIDE_ARROW_CALLOUT", &[Val::Num(callout_id as f64)])?;
    }
    Ok(0)
}

fn is_callout_active(state: &mut LuaState) -> LuaResult<u32> {
    let active = match read_callout_id(state, 1) {
        Some(id) => borrow_state_mut(state)?
            .arrow_callouts
            .active
            .contains_key(&id),
        None => false,
    };
    state.push(Val::Bool(active));
    Ok(1)
}

fn is_callout_acknowledged(state: &mut LuaState) -> LuaResult<u32> {
    let acknowledged = match read_callout_id(state, 1) {
        Some(id) => {
            sync_acknowledged_from_cvar(state)?;
            borrow_state_mut(state)?
                .arrow_callouts
                .acknowledged
                .contains(&id)
        }
        None => false,
    };
    state.push(Val::Bool(acknowledged));
    Ok(1)
}

fn sync_acknowledged_from_cvar(state: &mut LuaState) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    if let Some(value) = sim.cvars.get("acknowledgedArrowCallouts") {
        sim.arrow_callouts.sync_acknowledged_cvar_value(&value);
    }
    Ok(())
}

fn persist_acknowledged_cvar(state: &mut LuaState, value: &str) -> LuaResult<()> {
    let sim = borrow_state_mut(state)?;
    sim.cvars.set("acknowledgedArrowCallouts", value);
    Ok(())
}

fn read_callout_id(state: &LuaState, index: i32) -> Option<i64> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i64),
        _ => None,
    }
}

/// Translate a `calloutInfo` Lua table into the typed Rust record used
/// by `state.arrow_callouts`. Returns `None` when the table is missing
/// or has no `calloutID` (matches the Lua stub's early-return guard).
fn read_callout_info(state: &mut LuaState, payload: Val) -> Option<ArrowCalloutInfo> {
    let Val::Table(_) = payload else { return None };
    let callout_id = match table_get(state, payload, "calloutID") {
        Val::Num(n) => n as i64,
        _ => return None,
    };
    let frame_val = table_get(state, payload, "calloutFrame");
    let callout_frame = val_to_string(state, frame_val).unwrap_or_default();
    let text_val = table_get(state, payload, "calloutText");
    let callout_text = val_to_string(state, text_val).unwrap_or_default();
    let callout_type = read_int_field(state, payload, "calloutType");
    let callout_direction = read_int_field(state, payload, "calloutDirection");
    let offset_x = read_float_field(state, payload, "offsetX");
    let offset_y = read_float_field(state, payload, "offsetY");
    let ui_widget_set_id = match table_get(state, payload, "uiWidgetSetID") {
        Val::Num(n) => Some(n as u32),
        _ => None,
    };
    Some(ArrowCalloutInfo {
        callout_id,
        callout_frame,
        callout_type,
        callout_direction,
        offset_x,
        offset_y,
        callout_text,
        ui_widget_set_id,
    })
}

fn read_int_field(state: &mut LuaState, table: Val, key: &str) -> i32 {
    match table_get(state, table, key) {
        Val::Num(n) => n as i32,
        _ => 0,
    }
}

fn read_float_field(state: &mut LuaState, table: Val, key: &str) -> f32 {
    match table_get(state, table, key) {
        Val::Num(n) => n as f32,
        _ => 0.0,
    }
}
