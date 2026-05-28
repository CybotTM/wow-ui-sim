//! State-backed action-bar transition globals consumed by `Blizzard_ActionBar/Shared/MultiActionBars.lua`,
//! `Shared/StanceBar.lua`, and `Shared/VehicleLeaveButton.lua`.
//!
//! Globals registered:
//!
//! - `ActionBarBusy()` → `state.action_bar_state.busy` — true while a
//!   status-tracking-bar fade or page change is mid-animation.
//! - `ActionBarController_GetCurrentActionBarState()` →
//!   `state.action_bar_state.current_state`, with skinned override/vehicle
//!   state inferred from the backing action-bar flags — `LE_ACTIONBAR_STATE_MAIN`
//!   (1) or `LE_ACTIONBAR_STATE_OVERRIDE` (2).
//!
//! The `LE_ACTIONBAR_STATE_*` constants are seeded as global numbers in
//! `globals/strings/string_data/game_enums.rs`, so callers can compare
//! results directly without further wiring here.

use crate::lua_api::SimState;
use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn action_bar_busy(state: &mut LuaState) -> LuaResult<u32> {
    let busy = borrow_state(state)?.action_bar_state.busy;
    state.push(Val::Bool(busy));
    Ok(1)
}

fn action_bar_controller_get_current_action_bar_state(state: &mut LuaState) -> LuaResult<u32> {
    let current = {
        let sim = borrow_state(state)?;
        current_action_bar_state(&sim)
    };
    state.push(Val::Num(current as f64));
    Ok(1)
}

fn current_action_bar_state(state: &SimState) -> i32 {
    let skinned_override_active = state.has_override_action_bar
        && state
            .override_bar_skin
            .map(|skin| skin != 0)
            .unwrap_or(false);
    let skinned_vehicle_active = state.has_vehicle_action_bar
        && state
            .player
            .vehicle_skin
            .as_deref()
            .map(|skin| !skin.is_empty())
            .unwrap_or(false);
    if skinned_override_active || skinned_vehicle_active {
        2
    } else {
        state.action_bar_state.current_state
    }
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "ActionBarBusy", action_bar_busy)?;
    LuaApiMut::register_function(
        lua,
        "ActionBarController_GetCurrentActionBarState",
        action_bar_controller_get_current_action_bar_state,
    )?;
    Ok(())
}
