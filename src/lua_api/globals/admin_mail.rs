//! Rilua A_Admin handlers — Mail.
//!
//! Extracted from rilua_admin_world.rs per the 750-line file cap and to keep
//! each sub-module focused on a single concern. The parent entry
//! point in admin.rs imports these as pub(super) and weaves
//! them into the A_Admin TableBuilder chain.

use super::admin::{build_mail, opt_string_stack};
use crate::items;
use crate::lua_api::methods::{borrow_state_mut, table_get};
use crate::lua_api::state_types::{MailAttachment, MailStationery, MailStationeryTexture};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaError, LuaResult, RuntimeError, Val};

fn invalid_stationery(message: &str) -> LuaError {
    LuaError::Runtime(RuntimeError {
        message: format!("SetStationeryCatalog: {message}"),
        level: 1,
        traceback: vec![],
    })
}

fn stationery_string(state: &LuaState, value: Val) -> Option<String> {
    let Val::Str(value) = value else { return None };
    state
        .gc
        .string_arena
        .get(value)
        .map(|value| String::from_utf8_lossy(value.data()).into_owned())
}

fn parse_stationery(state: &mut LuaState, entry: Val) -> LuaResult<MailStationery> {
    let name_value = table_get(state, entry, "name");
    let name = stationery_string(state, name_value)
        .ok_or_else(|| invalid_stationery("name must be a string"))?;
    let texture = match table_get(state, entry, "texture") {
        Val::Str(value) => MailStationeryTexture::Path(
            stationery_string(state, Val::Str(value))
                .ok_or_else(|| invalid_stationery("invalid texture"))?,
        ),
        Val::Num(value) if value.is_finite() && value > 0.0 => MailStationeryTexture::FileId(value),
        _ => {
            return Err(invalid_stationery(
                "texture must be a path or positive file ID",
            ));
        }
    };
    let cost = match table_get(state, entry, "cost") {
        Val::Nil => None,
        Val::Num(value) if value.is_finite() && value >= 0.0 => Some(value),
        _ => return Err(invalid_stationery("cost must be nonnegative or nil")),
    };
    Ok(MailStationery {
        name,
        texture,
        cost,
    })
}

pub(super) fn set_stationery_catalog(state: &mut LuaState) -> LuaResult<u32> {
    let Val::Table(table) = stack_val(state, 1) else {
        return Err(invalid_stationery("expected a catalog table"));
    };
    let entries = state
        .gc
        .tables
        .get(table)
        .ok_or_else(|| invalid_stationery("invalid catalog table"))?
        .array_slice()
        .to_vec();
    let catalog = entries
        .into_iter()
        .map(|entry| parse_stationery(state, entry))
        .collect::<LuaResult<Vec<_>>>()?;
    borrow_state_mut(state)?.player.stationery_catalog = catalog;
    Ok(0)
}

// ── Mail ──────────────────────────────────────────────────────────────────────

fn parse_mail_items(state: &mut LuaState, value: Val) -> Vec<MailAttachment> {
    let Val::Table(items_ref) = value else {
        return Vec::new();
    };
    let Some(entries) = state
        .gc
        .tables
        .get(items_ref)
        .map(|table| table.array_slice().to_vec())
    else {
        return Vec::new();
    };

    entries
        .iter()
        .filter_map(|entry| {
            let item_id = match table_get(state, *entry, "item_id") {
                Val::Num(value) if value > 0.0 => value as u32,
                _ => return None,
            };
            let count = match table_get(state, *entry, "count") {
                Val::Num(value) if value > 0.0 => value as i32,
                _ => 1,
            };
            let quality = match table_get(state, *entry, "quality") {
                Val::Num(value) if value >= 0.0 => value as i32,
                _ => items::get_item(item_id)
                    .map(|item| item.quality as i32)
                    .unwrap_or(1),
            };
            Some(MailAttachment {
                item_id,
                count,
                quality,
            })
        })
        .collect()
}

pub(super) fn add_mail(state: &mut LuaState) -> LuaResult<u32> {
    let sender = opt_string_stack(state, 1, "Unknown");
    let subject = opt_string_stack(state, 2, "No Subject");
    let body = opt_string_stack(state, 3, "");
    let money = match stack_val(state, 4) {
        Val::Num(n) => n as u64,
        _ => 0,
    };
    let items = parse_mail_items(state, stack_val(state, 5));

    let mut st = borrow_state_mut(state)?;
    let id = st.player.next_mail_id;
    st.player.next_mail_id += 1;
    st.player
        .inbox
        .push(build_mail(id, sender, subject, body, money, items));
    Ok(0)
}

pub(super) fn clear_inbox(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.player.inbox.clear();
    Ok(0)
}

pub(super) fn set_inbox_count(state: &mut LuaState) -> LuaResult<u32> {
    let count = i32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.player.inbox.clear();
    for i in 0..count {
        let id = st.player.next_mail_id;
        st.player.next_mail_id += 1;
        let sender = format!("Player{}", i + 1);
        let subject = format!("Test Mail #{}", i + 1);
        let body = format!("This is test mail message {}.", i + 1);
        st.player
            .inbox
            .push(build_mail(id, sender, subject, body, 0, Vec::new()));
    }
    Ok(0)
}
