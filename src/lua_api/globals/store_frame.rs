//! `StoreFrame_IsShown` global backed by `SimState::store_frame_shown`.
//!
//! The sim doesn't render the in-game Store, but `MainMenuBarMicroButtons`
//! colours the Store micro-button as pushed when the probe returns true. A
//! SimState-backed flag lets tests drive that pushed-state rendering via
//! `A_Admin.SetStoreFrameShown(true)`.

use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_bridge::FromStack;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn store_frame_is_shown(state: &mut LuaState) -> LuaResult<u32> {
    let shown = borrow_state(state)?.store_frame_shown;
    state.push(Val::Bool(shown));
    Ok(1)
}

pub fn store_frame_set_shown(state: &mut LuaState) -> LuaResult<u32> {
    let shown = Option::<bool>::from_stack(state, 1)?.unwrap_or(true);
    let _context_key = Option::<String>::from_stack(state, 2)?;
    borrow_state_mut(state)?.store_frame_shown = shown;
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    table_set_rust_fn_static(
        state,
        state.global,
        "StoreFrame_IsShown",
        store_frame_is_shown,
    )?;
    table_set_rust_fn_static(
        state,
        state.global,
        "StoreFrame_SetShown",
        store_frame_set_shown,
    )?;
    Ok(())
}
