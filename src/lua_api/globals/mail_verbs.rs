//! Mail verbs that mutate `SimState.player.inbox` and dispatch
//! `MAIL_INBOX_UPDATE` / `MAIL_SEND_SUCCESS` / `MAIL_CLOSED` events.
//!
//! Migrates 4 entries off `GLOBAL_NIL_STUBS`:
//!
//! - `SendMail(recipient, subject, body)` — delivers outgoing mail to the
//!   recipient's inbox. Clears `send_mail_items` and `send_mail_money` /
//!   `send_mail_cod`. Fires `MAIL_SEND_SUCCESS`. For the simulator,
//!   "delivery" means the mail lands in the player's own inbox so admin
//!   flows can observe it — retail routes by account but the sim has
//!   a single account view.
//! - `DeleteMail(index)` — removes the 1-based inbox entry. Fires
//!   `MAIL_INBOX_UPDATE` when the index was valid.
//! - `ForwardMail(index, recipient, [subject])` — removes the inbox
//!   entry and re-enqueues a copy in the inbox with a new sender
//!   (the forwarder) and the supplied subject. Fires `MAIL_INBOX_UPDATE`.
//! - `CloseInbox()` — fires `MAIL_CLOSED`. No state change; the inbox
//!   list persists.
//!
//! Registered from `register_tail_globals` after `missing_surface` so the
//! Rust impls overwrite any pre-existing stub_nil entries.

use crate::event::Event;
use crate::lua_api::globals::admin::build_mail;
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_api::state_types::MailAttachment;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn push_event(state: &mut LuaState, name: &str) -> LuaResult<()> {
    borrow_state_mut(state)?.events.push(Event {
        name: name.to_string(),
        args: Vec::new(),
    });
    Ok(())
}

fn stack_i32(state: &mut LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn opt_string(state: &mut LuaState, index: i32) -> Option<String> {
    Option::<String>::from_stack(state, index).ok().flatten()
}

fn required_string(state: &mut LuaState, index: i32) -> Option<String> {
    let value = opt_string(state, index)?;
    (!value.is_empty()).then_some(value)
}

/// `SendMail(recipient, subject, body)` — delivers a copy to the player's
/// inbox (single-account sim), attaches pending `send_mail_*` items / money,
/// then clears the pending-send slots.
fn send_mail(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(recipient), Some(subject)) = (required_string(state, 1), required_string(state, 2))
    else {
        return Ok(0);
    };
    let body = opt_string(state, 3).unwrap_or_default();

    {
        let mut st = borrow_state_mut(state)?;
        let items = drain_send_mail_items(&mut st.player.send_mail_items);
        let money = std::mem::take(&mut st.player.send_mail_money);
        st.player.send_mail_cod = 0;

        let id = st.player.next_mail_id;
        st.player.next_mail_id = st.player.next_mail_id.saturating_add(1);
        st.player
            .inbox
            .push(build_mail(id, recipient, subject, body, money, items));
    }
    push_event(state, "MAIL_SEND_SUCCESS")?;
    push_event(state, "MAIL_INBOX_UPDATE")?;
    Ok(0)
}

fn drain_send_mail_items(slots: &mut [Option<MailAttachment>; 12]) -> Vec<MailAttachment> {
    let mut items = Vec::new();
    for slot in slots.iter_mut() {
        if let Some(item) = slot.take() {
            items.push(item);
        }
    }
    items
}

/// `DeleteMail(index)` — remove the 1-based inbox entry. Silent no-op on
/// out-of-range or non-numeric input.
fn delete_mail(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let removed = {
        let mut st = borrow_state_mut(state)?;
        let Some(zero_based) = index.checked_sub(1).and_then(|n| usize::try_from(n).ok()) else {
            return Ok(0);
        };
        if zero_based >= st.player.inbox.len() {
            return Ok(0);
        }
        st.player.inbox.remove(zero_based);
        true
    };
    if removed {
        push_event(state, "MAIL_INBOX_UPDATE")?;
    }
    Ok(0)
}

/// `ForwardMail(index, recipient, [subject])` — re-enqueue a copy of the
/// mail with a new sender/subject, remove the original. Silent no-op when
/// index or recipient is missing.
fn forward_mail(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(index), Some(recipient)) = (stack_i32(state, 1), required_string(state, 2)) else {
        return Ok(0);
    };
    let new_subject = opt_string(state, 3);

    let changed = {
        let mut st = borrow_state_mut(state)?;
        let Some(zero_based) = index.checked_sub(1).and_then(|n| usize::try_from(n).ok()) else {
            return Ok(0);
        };
        if zero_based >= st.player.inbox.len() {
            return Ok(0);
        }
        let original = st.player.inbox.remove(zero_based);
        let subject = new_subject.unwrap_or_else(|| format!("Fwd: {}", original.subject));
        let id = st.player.next_mail_id;
        st.player.next_mail_id = st.player.next_mail_id.saturating_add(1);
        st.player.inbox.push(build_mail(
            id,
            recipient,
            subject,
            original.body.clone(),
            original.money,
            original.items.clone(),
        ));
        true
    };
    if changed {
        push_event(state, "MAIL_INBOX_UPDATE")?;
    }
    Ok(0)
}

/// `CloseInbox()` — fire `MAIL_CLOSED`. The inbox itself is not cleared.
fn close_inbox(state: &mut LuaState) -> LuaResult<u32> {
    push_event(state, "MAIL_CLOSED")?;
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "SendMail", send_mail)?;
    LuaApiMut::register_function(lua, "DeleteMail", delete_mail)?;
    LuaApiMut::register_function(lua, "ForwardMail", forward_mail)?;
    LuaApiMut::register_function(lua, "CloseInbox", close_inbox)?;
    Ok(())
}
