//! Mail-state probe globals backed by `SimState.player.inbox`.
//!
//! Currently exposes:
//! - `HasNewMail()` / `C_Mail.HasNewMail()` — true when any inbox message is
//!   still unread.

use crate::lua_api::methods::{borrow_state, create_table};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val};

fn has_new_mail(state: &mut LuaState) -> LuaResult<u32> {
    let unread = borrow_state(state)?
        .player
        .inbox
        .iter()
        .any(|mail| !mail.was_read);
    state.push(Val::Bool(unread));
    Ok(1)
}

fn ensure_c_mail_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(b"C_Mail");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(r)) = existing {
        return r;
    }
    let new_val = create_table(state);
    let Val::Table(new_ref) = new_val else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    new_ref
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "HasNewMail", has_new_mail)?;
    let state = lua.state_mut();
    let table_ref = ensure_c_mail_table(state);
    table_set_rust_fn_static(state, table_ref, "HasNewMail", has_new_mail)?;
    Ok(())
}
