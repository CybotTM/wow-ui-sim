//! Auction / message verbs: `CancelAuction`, `SendAddonMessage`,
//! `SendChatMessage`.
//!
//! Migrates 3 entries off `GLOBAL_NIL_STUBS`:
//!
//! - `CancelAuction(index)` — fires `AUCTION_CANCELED` with the index
//!                             preserved on the event args.
//! - `SendAddonMessage(prefix, message, channel, target)` — append to
//!                             `message_log`, fire `CHAT_MSG_ADDON`.
//! - `SendChatMessage(message, chatType, language, target)` — append to
//!                             `message_log` with kind `"chat"`. No event
//!                             fires — retail echoes through a different
//!                             CHAT_MSG_* channel that's already stubbed.
//!
//! Registered from `register_tail_globals` after `missing_surface`.

use crate::event::{Event, EventArg};
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_api::state::MessageLogEntry;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn opt_string(state: &mut LuaState, index: i32) -> String {
    Option::<String>::from_stack(state, index)
        .ok()
        .flatten()
        .unwrap_or_default()
}

fn stack_f64(state: &mut LuaState, index: i32) -> Option<f64> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n),
        _ => None,
    }
}

fn push_event_with_args(state: &mut LuaState, name: &str, args: Vec<EventArg>) -> LuaResult<()> {
    borrow_state_mut(state)?.events.push(Event {
        name: name.to_string(),
        args,
    });
    Ok(())
}

fn append_message(
    state: &mut LuaState,
    kind: &str,
    prefix: String,
    message: String,
    channel: String,
    target: String,
) -> LuaResult<()> {
    borrow_state_mut(state)?.message_log.push(MessageLogEntry {
        kind: kind.to_string(),
        prefix,
        message,
        channel,
        target,
    });
    Ok(())
}

/// `CancelAuction(index)` — fire `AUCTION_CANCELED` carrying the index.
fn cancel_auction(state: &mut LuaState) -> LuaResult<u32> {
    let args = match stack_f64(state, 1) {
        Some(n) => vec![EventArg::Number(n)],
        None => Vec::new(),
    };
    push_event_with_args(state, "AUCTION_CANCELED", args)?;
    Ok(0)
}

/// `SendAddonMessage(prefix, message, channel, target)` — log + fire
/// `CHAT_MSG_ADDON` with four arg values (prefix/message/channel/target).
fn send_addon_message(state: &mut LuaState) -> LuaResult<u32> {
    let prefix = opt_string(state, 1);
    let message = opt_string(state, 2);
    let channel = opt_string(state, 3);
    let target = opt_string(state, 4);
    append_message(
        state,
        "addon",
        prefix.clone(),
        message.clone(),
        channel.clone(),
        target.clone(),
    )?;
    let args = vec![
        EventArg::String(prefix),
        EventArg::String(message),
        EventArg::String(channel),
        EventArg::String(target),
    ];
    push_event_with_args(state, "CHAT_MSG_ADDON", args)?;
    Ok(0)
}

/// `SendChatMessage(message, chatType, language, target)` — log only.
/// Retail echoes via `CHAT_MSG_SAY` / `CHAT_MSG_PARTY` etc.; the sim
/// doesn't route outbound chat through inbound events today.
fn send_chat_message(state: &mut LuaState) -> LuaResult<u32> {
    let message = opt_string(state, 1);
    let chat_type = opt_string(state, 2);
    // arg 3 = language (string, ignored), arg 4 = target.
    let target = opt_string(state, 4);
    append_message(state, "chat", String::new(), message, chat_type, target)?;
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "CancelAuction", cancel_auction)?;
    LuaApiMut::register_function(lua, "SendAddonMessage", send_addon_message)?;
    LuaApiMut::register_function(lua, "SendChatMessage", send_chat_message)?;
    Ok(())
}
