//! C_ChatInfo temporary shim — emote/caution/chat-line state is not modeled.
//!
//! Channel lookup and message sending are Rust-backed elsewhere. This shim only
//! owns no-state compatibility methods that return safe defaults.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_string_static;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

type LuaTableRef = GcRef<Table>;

pub(crate) fn register_c_chat_info_shims(state: &mut LuaState) -> LuaResult<()> {
    let chat_info = ensure_namespace(state, "C_ChatInfo")?;
    register_chat_action_shims(state, chat_info)?;
    register_chat_query_shims(state, chat_info)
}

fn register_chat_action_shims(state: &mut LuaState, chat_info: LuaTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(state, chat_info, "PerformEmote", return_false)?;
    table_set_rust_fn_static(state, chat_info, "CancelEmote", noop)?;
    table_set_rust_fn_static(state, chat_info, "IsValidChatLine", return_false)?;
    table_set_rust_fn_static(
        state,
        chat_info,
        "ReplaceIconAndGroupExpressions",
        return_first_arg,
    )?;
    table_set_rust_fn_static(state, chat_info, "UncensorChatLine", noop)?;
    table_set_rust_fn_static(state, chat_info, "DropCautionaryChatMessage", noop)?;
    table_set_rust_fn_static(state, chat_info, "SendCautionaryChatMessage", noop)
}

fn register_chat_query_shims(state: &mut LuaState, chat_info: LuaTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        chat_info,
        "AreOutgoingAddonChatMessagesRestricted",
        return_false,
    )?;
    table_set_rust_fn_static(state, chat_info, "GetNumReservedChatWindows", return_zero)?;
    table_set_rust_fn_static(
        state,
        chat_info,
        "GetChannelRulesetForChannelID",
        return_zero,
    )?;
    table_set_rust_fn_static(state, chat_info, "GetChannelRuleset", return_zero)?;
    table_set_rust_fn_static(state, chat_info, "GetChatLineText", return_nil)?;
    table_set_rust_fn_static(state, chat_info, "IsTimerunningPlayer", return_false)?;
    table_set_rust_fn_static(
        state,
        chat_info,
        "GetChannelShortcutForChannelID",
        return_empty_string,
    )
}

fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn return_nil(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn return_empty_string(state: &mut LuaState) -> LuaResult<u32> {
    let text = create_string_static(state, "");
    state.push(text);
    Ok(1)
}

fn return_first_arg(state: &mut LuaState) -> LuaResult<u32> {
    state.push(stack_val(state, 1));
    Ok(1)
}
