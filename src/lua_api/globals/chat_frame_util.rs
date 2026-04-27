//! `ChatFrameUtil.AddSystemMessage(messageText)` — Blizzard helper that
//! appends a yellow system-channel line to the default chat frame.
//!
//! Real impl (Blizzard_ChatFrameBase/Shared/ChatFrameUtil.lua:245):
//! ```lua
//! function ChatFrameUtil.AddSystemMessage(messageText)
//!     local info = ChatTypeInfo["SYSTEM"];
//!     DEFAULT_CHAT_FRAME:AddMessage(messageText, info.r, info.g, info.b, info.id);
//! end
//! ```
//!
//! The simulator records every message in `SimState.system_chat_log` so
//! headless tests can assert on what was emitted, then best-effort routes
//! the line to `DEFAULT_CHAT_FRAME` (or `ChatFrame1` as a fallback) when a
//! chat frame is available.

use crate::lua_api::methods::{borrow_state_mut, call_function_state, create_string, table_get};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val};

const SYSTEM_R: f64 = 1.0;
const SYSTEM_G: f64 = 1.0;
const SYSTEM_B: f64 = 0.0;
const SYSTEM_ID: f64 = 1.0;

fn ensure_chat_frame_util_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(b"ChatFrameUtil");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(r)) = existing {
        return r;
    }
    let new_val = crate::lua_api::methods::create_table(state);
    let Val::Table(new_ref) = new_val else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    new_ref
}

fn default_chat_frame(state: &mut LuaState) -> Val {
    let globals = Val::Table(state.global);
    let preferred = table_get(state, globals, "DEFAULT_CHAT_FRAME");
    if let Val::Table(_) = preferred {
        return preferred;
    }
    table_get(state, globals, "ChatFrame1")
}

fn route_to_chat_frame(state: &mut LuaState, message_text: &str) -> LuaResult<()> {
    let chat_frame = default_chat_frame(state);
    let Val::Table(_) = chat_frame else {
        return Ok(());
    };
    let add_message = table_get(state, chat_frame, "AddMessage");
    let Val::Function(_) = add_message else {
        return Ok(());
    };
    let message_val = create_string(state, message_text);
    let _ = call_function_state(
        state,
        add_message,
        &[
            chat_frame,
            message_val,
            Val::Num(SYSTEM_R),
            Val::Num(SYSTEM_G),
            Val::Num(SYSTEM_B),
            Val::Num(SYSTEM_ID),
        ],
    )?;
    Ok(())
}

fn add_system_message(state: &mut LuaState) -> LuaResult<u32> {
    let message = Option::<String>::from_stack(state, 1)
        .ok()
        .flatten()
        .unwrap_or_default();
    borrow_state_mut(state)?
        .system_chat_log
        .push(message.clone());
    route_to_chat_frame(state, &message)?;
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let table_ref = ensure_chat_frame_util_table(state);
    table_set_rust_fn_static(state, table_ref, "AddSystemMessage", add_system_message)?;
    Ok(())
}
