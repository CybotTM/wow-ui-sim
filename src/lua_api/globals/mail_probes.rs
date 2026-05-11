//! Mail-state probe globals backed by `SimState.player.inbox`.
//!
//! Currently exposes:
//! - `HasNewMail()` / `C_Mail.HasNewMail()` — true when any inbox message is
//!   still unread.

use crate::items;
use crate::lua_api::globals::missing_surface::item_link_for_id;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_string_static, create_table,
};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
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

fn mailbox_index(raw_index: f64) -> Option<usize> {
    let index = raw_index as i32;
    index
        .checked_sub(1)
        .and_then(|value| usize::try_from(value).ok())
}

fn inbox_entry(
    state: &LuaState,
    raw_index: f64,
) -> Option<crate::lua_api::state_types::MailMessage> {
    let index = mailbox_index(raw_index)?;
    borrow_state(state).ok()?.player.inbox.get(index).cloned()
}

fn attachment_at(
    state: &LuaState,
    mail_index: f64,
    attachment_index: f64,
) -> Option<crate::lua_api::state_types::MailAttachment> {
    let attachment_index = mailbox_index(attachment_index)?;
    inbox_entry(state, mail_index)?
        .items
        .get(attachment_index)
        .cloned()
}

fn send_mail_attachment_at(
    state: &LuaState,
    raw_index: f64,
) -> Option<crate::lua_api::state_types::MailAttachment> {
    let index = mailbox_index(raw_index)?;
    borrow_state(state)
        .ok()?
        .player
        .send_mail_items
        .get(index)?
        .clone()
}

fn icon_for_item(item_id: u32) -> f64 {
    items::get_item(item_id)
        .map(|item| {
            if item.icon_file_data_id == 0 {
                134400.0
            } else {
                item.icon_file_data_id as f64
            }
        })
        .unwrap_or(134400.0)
}

fn push_mail_attachment(
    state: &mut LuaState,
    attachment: crate::lua_api::state_types::MailAttachment,
) -> LuaResult<u32> {
    let item = items::get_item(attachment.item_id);
    let name = item.map(|row| row.name).unwrap_or("Unknown");
    let name = create_string_static(state, name);
    state.push(name);
    state.push(Val::Num(attachment.item_id as f64));
    state.push(Val::Num(icon_for_item(attachment.item_id)));
    state.push(Val::Num(attachment.count as f64));
    state.push(Val::Num(
        item.map(|row| row.quality as f64)
            .unwrap_or(attachment.quality as f64),
    ));
    Ok(5)
}

fn get_inbox_num_items(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.player.inbox.len() as f64;
    state.push(Val::Num(count));
    state.push(Val::Num(count));
    Ok(2)
}

fn get_inbox_header_info(state: &mut LuaState) -> LuaResult<u32> {
    let raw_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    let Some(mail) = inbox_entry(state, raw_index) else {
        return Ok(0);
    };
    let package_icon = mail
        .items
        .first()
        .map(|attachment| Val::Num(icon_for_item(attachment.item_id)))
        .unwrap_or(Val::Nil);
    state.push(package_icon);
    state.push(Val::Num(mail.stationery_icon as f64));
    let sender = create_string(state, &mail.sender);
    let subject = create_string(state, &mail.subject);
    state.push(sender);
    state.push(subject);
    state.push(Val::Num(mail.money as f64));
    state.push(Val::Num(mail.cod_amount as f64));
    state.push(Val::Num(mail.days_left as f64));
    state.push(Val::Num(mail.items.len() as f64));
    state.push(Val::Bool(mail.was_read));
    state.push(Val::Bool(mail.was_returned));
    state.push(Val::Bool(
        !mail.body.is_empty() || !mail.items.is_empty() || mail.money > 0,
    ));
    state.push(Val::Bool(mail.can_reply));
    state.push(Val::Bool(mail.is_gm));
    Ok(13)
}

fn get_inbox_item(state: &mut LuaState) -> LuaResult<u32> {
    let mail_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    let attachment_index = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0);
    let Some(attachment) = attachment_at(state, mail_index, attachment_index) else {
        return Ok(0);
    };
    push_mail_attachment(state, attachment)?;
    state.push(Val::Bool(true));
    state.push(Val::Bool(false));
    Ok(7)
}

fn get_inbox_item_link(state: &mut LuaState) -> LuaResult<u32> {
    let mail_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    let attachment_index = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0);
    let Some(attachment) = attachment_at(state, mail_index, attachment_index) else {
        return Ok(0);
    };
    match item_link_for_id(attachment.item_id) {
        Some(link) => {
            let link = create_string(state, &link);
            state.push(link);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_inbox_text(state: &mut LuaState) -> LuaResult<u32> {
    let raw_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    let Some(mail) = inbox_entry(state, raw_index) else {
        return Ok(0);
    };
    let body = create_string(state, &mail.body);
    let empty_sender = create_string_static(state, "");
    let empty_subject = create_string_static(state, "");
    state.push(body);
    state.push(empty_sender);
    state.push(empty_subject);
    state.push(Val::Bool(true));
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    Ok(6)
}

fn get_inbox_invoice_info(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn has_inbox_item(state: &mut LuaState) -> LuaResult<u32> {
    let mail_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    let attachment_index = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0);
    state.push(Val::Bool(
        attachment_at(state, mail_index, attachment_index).is_some(),
    ));
    Ok(1)
}

fn has_send_mail_item(state: &mut LuaState) -> LuaResult<u32> {
    let raw_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    state.push(Val::Bool(
        send_mail_attachment_at(state, raw_index).is_some(),
    ));
    Ok(1)
}

fn get_send_mail_item(state: &mut LuaState) -> LuaResult<u32> {
    let raw_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    let Some(attachment) = send_mail_attachment_at(state, raw_index) else {
        return Ok(0);
    };
    push_mail_attachment(state, attachment)
}

fn get_send_mail_item_link(state: &mut LuaState) -> LuaResult<u32> {
    let raw_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    let Some(attachment) = send_mail_attachment_at(state, raw_index) else {
        return Ok(0);
    };
    match item_link_for_id(attachment.item_id) {
        Some(link) => {
            let link = create_string(state, &link);
            state.push(link);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn inbox_item_can_delete(state: &mut LuaState) -> LuaResult<u32> {
    let raw_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    let deletable = inbox_entry(state, raw_index)
        .map(|mail| mail.money == 0 && mail.items.is_empty())
        .unwrap_or(false);
    state.push(Val::Bool(deletable));
    Ok(1)
}

fn can_check_inbox(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    state.push(Val::Num(0.0));
    Ok(2)
}

fn has_inbox_money(state: &mut LuaState) -> LuaResult<u32> {
    let raw_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    let has_money = inbox_entry(state, raw_index)
        .map(|mail| mail.money > 0)
        .unwrap_or(false);
    state.push(Val::Bool(has_money));
    Ok(1)
}

fn get_num_items(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.player.inbox.len() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

fn set_opening_all(state: &mut LuaState) -> LuaResult<u32> {
    let opening = matches!(crate::lua_bridge::stack_val(state, 1), Val::Bool(true));
    borrow_state_mut(state)?.player.opening_all_mail = opening;
    Ok(0)
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
    LuaApiMut::register_function(lua, "GetInboxNumItems", get_inbox_num_items)?;
    LuaApiMut::register_function(lua, "GetInboxHeaderInfo", get_inbox_header_info)?;
    LuaApiMut::register_function(lua, "GetInboxItem", get_inbox_item)?;
    LuaApiMut::register_function(lua, "GetInboxItemLink", get_inbox_item_link)?;
    LuaApiMut::register_function(lua, "GetInboxText", get_inbox_text)?;
    LuaApiMut::register_function(lua, "GetInboxInvoiceInfo", get_inbox_invoice_info)?;
    LuaApiMut::register_function(lua, "HasInboxItem", has_inbox_item)?;
    LuaApiMut::register_function(lua, "HasSendMailItem", has_send_mail_item)?;
    LuaApiMut::register_function(lua, "GetSendMailItem", get_send_mail_item)?;
    LuaApiMut::register_function(lua, "GetSendMailItemLink", get_send_mail_item_link)?;
    LuaApiMut::register_function(lua, "InboxItemCanDelete", inbox_item_can_delete)?;
    let state = lua.state_mut();
    let table_ref = ensure_c_mail_table(state);
    table_set_rust_fn_static(state, table_ref, "HasNewMail", has_new_mail)?;
    table_set_rust_fn_static(state, table_ref, "CanCheckInbox", can_check_inbox)?;
    table_set_rust_fn_static(state, table_ref, "HasInboxMoney", has_inbox_money)?;
    table_set_rust_fn_static(state, table_ref, "GetNumItems", get_num_items)?;
    table_set_rust_fn_static(state, table_ref, "SetOpeningAll", set_opening_all)?;
    Ok(())
}
