//! Wrath-only no-op frame method stubs.
//!
//! These methods existed in the Wrath client but were removed or moved in
//! later retail builds. Stubbed here so wrath addon code does not crash on
//! missing method errors.
//!
//! Only compiled when `--features client-wrath` is active.

use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;

pub fn register_all(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, mt, "IgnoreDepth", ignore_depth)?;
    table_set_rust_fn_static(state, mt, "SetBackdropColor", set_backdrop_color)?;
    table_set_rust_fn_static(state, mt, "SetBackdropBorderColor", set_backdrop_border_color)?;
    table_set_rust_fn_static(state, mt, "SetPlayerTextureHeight", set_player_texture_height)?;
    Ok(())
}

/// Wrath frame method that controlled depth-buffer interactions. No-op stub.
fn ignore_depth(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// Wrath had `SetBackdropBorderColor` directly on Frame; retail moved it to
/// `BackdropTemplateMixin`. No-op stub — real backdrop rendering is out of scope.
fn set_backdrop_border_color(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// Wrath had `SetBackdropColor` directly on Frame; same migration as the
/// border-color counterpart. No-op stub.
fn set_backdrop_color(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// Wrath PlayerModel method. No-op stub.
fn set_player_texture_height(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
