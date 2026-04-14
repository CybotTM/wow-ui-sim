//! WoW taint integration for rilua.

use rilua::vm::state::LuaState;
use rilua::LuaApiMut;

/// Enable taint tracking on the rilua state.
pub fn enable_taint_mode(lua: &mut rilua::Lua) {
    lua.state_mut().taint_mode = true;
}

/// Stamp addon taint on a compiled function.
///
/// In WoW, each addon's compiled closures carry the addon's name as taint.
/// When the function executes, the call frame inherits its taint.
pub fn stamp_addon_taint(lua: &mut rilua::Lua, func: &rilua::Function, addon_name: &str) {
    // Store in registry: __closure_taint[func] = addon_name
    let state = lua.state_mut();
    let _ = (state, func, addon_name);
    // TODO: implement once rilua's taint registry is available
}

/// Set taint for the current frame before dispatching a script handler.
pub fn set_frame_taint(state: &mut LuaState, addon_name: Option<&str>) {
    if let Some(ci) = state.call_stack.get_mut(state.ci) {
        ci.taint = addon_name.map(|s| s.to_string());
    }
}

/// Clear taint for secure (Blizzard) code execution.
pub fn clear_frame_taint(state: &mut LuaState) {
    if let Some(ci) = state.call_stack.get_mut(state.ci) {
        ci.taint = None;
    }
}
