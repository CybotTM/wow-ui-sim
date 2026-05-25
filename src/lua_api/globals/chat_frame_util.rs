//! `ChatFrameUtil.AddSystemMessage(messageText)` and
//! `ChatFrameUtil.OpenChat(text, chatType?, cursorPosition?)` — Blizzard
//! chat-frame helpers consumed by core UI flows.
//!
//! Real `AddSystemMessage` impl (Blizzard_ChatFrameBase/Shared/ChatFrameUtil.lua:245):
//! ```lua
//! function ChatFrameUtil.AddSystemMessage(messageText)
//!     local info = ChatTypeInfo["SYSTEM"];
//!     DEFAULT_CHAT_FRAME:AddMessage(messageText, info.r, info.g, info.b, info.id);
//! end
//! ```
//!
//! Real `OpenChat` impl (same file, line 358) opens the appropriate chat
//! edit box, parks the cursor, and pre-fills text. The simulator records
//! the args on `SimState.chat_edit_open_state` so tests can assert what
//! would have been pre-filled, and best-effort shows + sets text on
//! `ChatFrame1EditBox` when present.

use crate::lua_api::methods::{
    borrow_state_mut, call_function_state, create_string, create_string_static, create_table,
    table_get, table_set, table_set_num, table_set_static, val_to_string,
};
use crate::lua_api::state::ChatEditOpenState;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val};

const SYSTEM_R: f64 = 1.0;
const SYSTEM_G: f64 = 1.0;
const SYSTEM_B: f64 = 0.0;
const SYSTEM_ID: f64 = 1.0;
const CHAT_TYPE_GROUPS: &[(&str, &[&str])] = &[
    (
        "SYSTEM",
        &[
            "SYSTEM",
            "ERRORS",
            "IGNORED",
            "ACHIEVEMENT",
            "CHANNEL_NOTICE_USER",
        ],
    ),
    ("SAY", &["SAY"]),
    ("YELL", &["YELL"]),
    ("WHISPER", &["WHISPER", "WHISPER_INFORM"]),
    ("PARTY", &["PARTY", "PARTY_LEADER"]),
    ("RAID", &["RAID", "RAID_LEADER", "RAID_WARNING"]),
    ("GUILD", &["GUILD", "OFFICER"]),
    ("CHANNEL", &["CHANNEL", "CHANNEL_JOIN", "CHANNEL_LEAVE"]),
    ("EMOTE", &["EMOTE"]),
    (
        "BN_WHISPER",
        &["BN_WHISPER", "BN_WHISPER_INFORM", "BN_INLINE_TOAST_ALERT"],
    ),
    ("INSTANCE_CHAT", &["INSTANCE_CHAT", "INSTANCE_CHAT_LEADER"]),
];

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

fn install_chat_type_group(state: &mut LuaState) {
    let globals = Val::Table(state.global);
    if !matches!(table_get(state, globals, "ChatTypeGroup"), Val::Nil) {
        return;
    }

    let group_table = create_table(state);
    for (group_name, events) in CHAT_TYPE_GROUPS {
        let event_table = create_table(state);
        let Val::Table(event_table_ref) = event_table else {
            unreachable!("create_table must return a table");
        };
        for (index, event_name) in events.iter().enumerate() {
            let event_value = create_string_static(state, event_name);
            table_set_num(state, event_table_ref, (index + 1) as f64, event_value);
        }
        table_set(state, group_table, group_name, event_table);
    }

    table_set_static(state, globals, "ChatTypeGroup", group_table);
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

fn open_chat(state: &mut LuaState) -> LuaResult<u32> {
    let text = val_to_string(state, stack_val(state, 1)).unwrap_or_default();
    let chat_type = val_to_string(state, stack_val(state, 2));
    let cursor_position = i64::from_stack(state, 3).ok();
    record_open_chat_state(state, &text, chat_type, cursor_position)?;
    apply_to_default_edit_box(state, &text)?;
    Ok(0)
}

fn record_open_chat_state(
    state: &mut LuaState,
    text: &str,
    chat_type: Option<String>,
    cursor_position: Option<i64>,
) -> LuaResult<()> {
    borrow_state_mut(state)?.chat_edit_open_state = Some(ChatEditOpenState {
        text: text.to_string(),
        chat_type,
        cursor_position,
    });
    Ok(())
}

fn default_edit_box(state: &mut LuaState) -> Val {
    let globals = Val::Table(state.global);
    table_get(state, globals, "ChatFrame1EditBox")
}

fn apply_to_default_edit_box(state: &mut LuaState, text: &str) -> LuaResult<()> {
    let edit_box = default_edit_box(state);
    if !matches!(edit_box, Val::Table(_)) {
        return Ok(());
    }
    invoke_edit_box_method(state, edit_box, "Show", &[])?;
    let text_val = create_string(state, text);
    invoke_edit_box_method(state, edit_box, "SetText", &[text_val])?;
    Ok(())
}

fn invoke_edit_box_method(
    state: &mut LuaState,
    edit_box: Val,
    method_name: &str,
    extra_args: &[Val],
) -> LuaResult<()> {
    let method = table_get(state, edit_box, method_name);
    if !matches!(method, Val::Function(_)) {
        return Ok(());
    }
    let mut args = Vec::with_capacity(1 + extra_args.len());
    args.push(edit_box);
    args.extend_from_slice(extra_args);
    let _ = call_function_state(state, method, &args)?;
    Ok(())
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    install_chat_type_group(state);
    let table_ref = ensure_chat_frame_util_table(state);
    table_set_rust_fn_static(state, table_ref, "AddSystemMessage", add_system_message)?;
    table_set_rust_fn_static(state, table_ref, "OpenChat", open_chat)?;
    Ok(())
}
