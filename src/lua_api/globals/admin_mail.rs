//! Rilua A_Admin handlers — Mail.
//!
//! Extracted from rilua_admin_world.rs per the 750-line file cap and to keep
//! each sub-module focused on a single concern. The parent entry
//! point in admin.rs imports these as pub(super) and weaves
//! them into the A_Admin TableBuilder chain.

use super::admin::{build_mail, opt_string_stack};
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

// ── Mail ──────────────────────────────────────────────────────────────────────

pub(super) fn add_mail(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_bridge::stack_val;

    let sender = opt_string_stack(state, 1, "Unknown");
    let subject = opt_string_stack(state, 2, "No Subject");
    let body = opt_string_stack(state, 3, "");
    let money = match stack_val(state, 4) {
        Val::Num(n) => n as u64,
        _ => 0,
    };
    // items table at arg 5 — parsed as empty for now (no mlua Table access in rilua path)
    let items = Vec::new();

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
