//! Action-bar transition globals consumed by `Blizzard_ActionBar/Shared/MultiActionBars.lua`,
//! `Shared/StanceBar.lua`, and `Shared/VehicleLeaveButton.lua`.
//!
//! Globals registered:
//!
//! - `ActionBarBusy()` → `state.action_bar_state.busy` — true while a
//!   status-tracking-bar fade or page change is mid-animation.
//! - `ActionBarController_GetCurrentActionBarState()` →
//!   `state.action_bar_state.current_state` — `LE_ACTIONBAR_STATE_MAIN` (1)
//!   or `LE_ACTIONBAR_STATE_OVERRIDE` (2).
//!
//! The `LE_ACTIONBAR_STATE_*` constants are seeded as global numbers in
//! `globals/strings/string_data/game_enums.rs`, so callers can compare
//! results directly without further wiring here.

use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn action_bar_busy(state: &mut LuaState) -> LuaResult<u32> {
    let busy = borrow_state(state)?.action_bar_state.busy;
    state.push(Val::Bool(busy));
    Ok(1)
}

fn action_bar_controller_get_current_action_bar_state(state: &mut LuaState) -> LuaResult<u32> {
    let current = borrow_state(state)?.action_bar_state.current_state;
    state.push(Val::Num(current as f64));
    Ok(1)
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
