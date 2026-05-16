//! `C_FriendList` probe surface backed by a small seeded friend list.
//!
//! Blizzard's Friends frame only needs a tiny read surface in the sim:
//! friend counts, friend-by-index/name lookups, and the who-list row
//! probe used by the search UI. The seeded rows here are independent of
//! `SimState.social_friends`, which remains the backing store for
//! `C_Social`.

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state_mut, create_string, create_table, table_set};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

#[derive(Clone, Copy)]
struct FriendListRow {
    name: &'static str,
    level: i32,
    area: &'static str,
    class_name: &'static str,
    notes: &'static str,
    connected: bool,
    guid: &'static str,
    race_str: &'static str,
    gender: i32,
    full_guild_name: &'static str,
}

const FRIEND_LIST_ROWS: &[FriendListRow] = &[
    FriendListRow {
        name: "Alyth",
        level: 80,
        area: "Stormwind City",
        class_name: "Paladin",
        notes: "Testing the FriendsFrame list",
        connected: true,
        guid: "Player-11-00000001",
        race_str: "Human",
        gender: 2,
        full_guild_name: "Heroes of Azeroth",
    },
    FriendListRow {
        name: "Brennor",
        level: 80,
        area: "Duskwood",
        class_name: "Warrior",
        notes: "",
        connected: false,
        guid: "Player-11-00000002",
        race_str: "Human",
        gender: 2,
        full_guild_name: "Heroes of Azeroth",
    },
];

pub(super) fn register_friend_list_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_FriendList")?;
    table_set_rust_fn_static(state, table_ref, "GetNumFriends", get_num_friends)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetNumOnlineFriends",
        get_num_online_friends,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetNumIgnores", get_num_ignores)?;
    table_set_rust_fn_static(state, table_ref, "GetIgnoreName", get_ignore_name)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetFriendInfoByIndex",
        get_friend_info_by_index,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetFriendInfoByName",
        get_friend_info_by_name,
    )?;
    table_set_rust_fn_static(state, table_ref, "IsFriend", is_friend)?;
    table_set_rust_fn_static(state, table_ref, "GetWhoInfo", get_who_info)?;
    table_set_rust_fn_static(state, table_ref, "SetWhoToUi", set_who_to_ui)?;
    table_set_rust_fn_static(state, table_ref, "ShowFriends", show_friends)?;
    Ok(())
}

fn get_num_friends(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(FRIEND_LIST_ROWS.len() as f64));
    Ok(1)
}

fn get_num_online_friends(state: &mut LuaState) -> LuaResult<u32> {
    let count = FRIEND_LIST_ROWS.iter().filter(|row| row.connected).count();
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn get_num_ignores(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn get_ignore_name(state: &mut LuaState) -> LuaResult<u32> {
    let _index = i32::from_stack(state, 1)?;
    state.push(Val::Nil);
    Ok(1)
}

fn get_friend_info_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let Some(row) = friend_row_by_index(index) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let friend = build_friend_info_table(state, row);
    state.push(friend);
    Ok(1)
}

fn get_friend_info_by_name(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let Some(row) = FRIEND_LIST_ROWS.iter().find(|row| row.name == name) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let friend = build_friend_info_table(state, row);
    state.push(friend);
    Ok(1)
}

fn is_friend(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let is_friend = FRIEND_LIST_ROWS.iter().any(|row| row.name == name);
    state.push(Val::Bool(is_friend));
    Ok(1)
}

fn get_who_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let Some(row) = friend_row_by_index(index) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let who_info = build_who_info_table(state, row);
    state.push(who_info);
    Ok(1)
}

fn set_who_to_ui(state: &mut LuaState) -> LuaResult<u32> {
    let _ = bool::from_stack(state, 1)?;
    Ok(0)
}

fn show_friends(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?
        .events
        .push_simple("FRIENDLIST_UPDATE");
    Ok(0)
}

fn friend_row_by_index(index: i32) -> Option<&'static FriendListRow> {
    let zero_based = usize::try_from(index - 1).ok()?;
    FRIEND_LIST_ROWS.get(zero_based)
}

fn build_friend_info_table(state: &mut LuaState, row: &FriendListRow) -> Val {
    let table = create_table(state);
    let name = create_string(state, row.name);
    let full_name = create_string(state, row.name);
    let area = create_string(state, row.area);
    let class_name = create_string(state, row.class_name);
    let class_str = create_string(state, row.class_name);
    let filename = create_string(state, &row.class_name.to_uppercase());
    let notes = create_string(state, row.notes);
    let guid = create_string(state, row.guid);
    let race_str = create_string(state, row.race_str);
    let full_guild_name = create_string(state, row.full_guild_name);
    table_set(state, table, "name", name);
    table_set(state, table, "fullName", full_name);
    table_set(state, table, "level", Val::Num(row.level as f64));
    table_set(state, table, "area", area);
    table_set(state, table, "className", class_name);
    table_set(state, table, "classStr", class_str);
    table_set(state, table, "filename", filename);
    table_set(state, table, "notes", notes);
    table_set(state, table, "connected", Val::Bool(row.connected));
    table_set(state, table, "guid", guid);
    table_set(state, table, "raceStr", race_str);
    table_set(state, table, "gender", Val::Num(row.gender as f64));
    table_set(state, table, "fullGuildName", full_guild_name);
    table
}

fn build_who_info_table(state: &mut LuaState, row: &FriendListRow) -> Val {
    let table = create_table(state);
    let full_name = create_string(state, row.name);
    let name = create_string(state, row.name);
    let class_str = create_string(state, row.class_name);
    let filename = create_string(state, &row.class_name.to_uppercase());
    let area = create_string(state, row.area);
    let race_str = create_string(state, row.race_str);
    let full_guild_name = create_string(state, row.full_guild_name);
    table_set(state, table, "fullName", full_name);
    table_set(state, table, "name", name);
    table_set(state, table, "level", Val::Num(row.level as f64));
    table_set(state, table, "classStr", class_str);
    table_set(state, table, "filename", filename);
    table_set(state, table, "area", area);
    table_set(state, table, "raceStr", race_str);
    table_set(state, table, "gender", Val::Num(row.gender as f64));
    table_set(state, table, "fullGuildName", full_guild_name);
    table
}
