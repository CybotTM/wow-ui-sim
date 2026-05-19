//! `C_Social` probe surface backed by `SimState.social_friends`.
//!
//! Migrates 2 entries off the namespace stub tables:
//!
//! - `C_Social.GetFriendInfo(index)` — returns a `C_FriendList.FriendInfo`
//!   table for a 1-based friend index, or nil when out of range.
//! - `C_Social.GetFriends()` — returns an array table of all
//!   `C_FriendList.FriendInfo` tables (one per seeded friend).

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set_static};
use crate::lua_api::state_types::SocialFriend;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_social_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Social")?;
    table_set_rust_fn_static(state, table_ref, "GetFriendInfo", c_social_get_friend_info)?;
    table_set_rust_fn_static(state, table_ref, "GetFriends", c_social_get_friends)?;
    Ok(())
}

fn c_social_get_friend_info(state: &mut LuaState) -> LuaResult<u32> {
    let friend_index = i32::from_stack(state, 1)?;
    let entry = {
        let sim = borrow_state(state)?;
        let idx = usize::try_from(friend_index - 1).unwrap_or(usize::MAX);
        sim.social_friends.get(idx).cloned()
    };
    let Some(friend) = entry else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let t = push_friend_info_table(state, &friend);
    state.push(t);
    Ok(1)
}

fn c_social_get_friends(state: &mut LuaState) -> LuaResult<u32> {
    let friends = {
        let sim = borrow_state(state)?;
        sim.social_friends.clone()
    };
    let arr = create_table(state);
    for (i, friend) in friends.iter().enumerate() {
        let entry = push_friend_info_table(state, friend);
        let Val::Table(arr_ref) = arr else { continue };
        if let Some(tbl) = state.gc.tables.get_mut(arr_ref) {
            let _ = tbl.raw_set(Val::Num((i + 1) as f64), entry, &state.gc.string_arena);
        }
        state.gc.barrier_back(arr_ref);
    }
    state.push(arr);
    Ok(1)
}

/// Build a `C_FriendList.FriendInfo` Lua table from a `SocialFriend`.
fn push_friend_info_table(state: &mut LuaState, friend: &SocialFriend) -> Val {
    let t = create_table(state);

    let name = create_string(state, &friend.name);
    let area = create_string(state, &friend.area);
    let class_name = create_string(state, &friend.class_name);
    let note = create_string(state, &friend.note);
    let guid = create_string(state, &friend.guid);

    table_set_static(state, t, "name", name);
    table_set_static(state, t, "level", Val::Num(friend.level as f64));
    table_set_static(state, t, "area", area);
    table_set_static(state, t, "className", class_name);
    table_set_static(state, t, "notes", note);
    table_set_static(state, t, "connected", Val::Bool(friend.is_online));
    table_set_static(state, t, "guid", guid);
    table_set_static(state, t, "afk", Val::Bool(false));
    table_set_static(state, t, "dnd", Val::Bool(false));
    table_set_static(state, t, "rafLinkType", Val::Num(0.0));

    t
}
