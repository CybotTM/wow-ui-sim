//! rilua-side frame method helpers for Phase 3 migration.
//!
//! Production frame methods need `SimState` access (behind `Rc<RefCell<>>`),
//! which doesn't fit the `define_methods!` macro's `FrameArena` pattern.
//! Instead, methods are raw `RustFn`s that use these helpers to extract
//! the frame ID from a rilua-backed table and borrow `SimState`.

use super::env::WowLuaAppData;
use super::SimState;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};
use std::cell::{Ref, RefMut};

/// Extract the frame ID (u64) from a Lua argument (a backed table).
///
/// The table's backing `(index, generation)` encodes the widget ID as
/// `(id as u32, (id >> 32) as u32)`.
pub fn frame_id_from_stack(state: &LuaState, index: i32) -> LuaResult<u64> {
    let val = stack_val(state, index);
    let Val::Table(table_ref) = val else {
        return Err(runtime_error("expected frame table as self argument"));
    };
    let backing = state
        .gc
        .tables
        .get(table_ref)
        .and_then(|t| t.backing())
        .ok_or_else(|| runtime_error("expected frame-backed table"))?;
    Ok((backing.0 as u64) | ((backing.1 as u64) << 32))
}

/// Borrow `SimState` immutably from rilua's app_data.
pub fn borrow_state(state: &LuaState) -> LuaResult<Ref<'_, SimState>> {
    let app = state
        .app_data::<WowLuaAppData>()
        .ok_or_else(|| runtime_error("missing WowLuaAppData"))?;
    Ok(app.sim_state.borrow())
}

/// Borrow `SimState` mutably from rilua's app_data.
pub fn borrow_state_mut(state: &LuaState) -> LuaResult<RefMut<'_, SimState>> {
    let app = state
        .app_data::<WowLuaAppData>()
        .ok_or_else(|| runtime_error("missing WowLuaAppData"))?;
    Ok(app.sim_state.borrow_mut())
}

/// Pack a u64 widget ID into the (u32, u32) backing slot format.
pub fn pack_id(lo: u32, hi: u32) -> u64 {
    (lo as u64) | ((hi as u64) << 32)
}

/// Unpack a u64 widget ID into (lo, hi) for table backing.
pub fn unpack_id(id: u64) -> (u32, u32) {
    (id as u32, (id >> 32) as u32)
}
