//! `C_FriendList` probe surface backed by social friend state.
//!
//! Blizzard's Friends frame only needs a tiny read surface in the sim:
//! friend counts, friend-by-index/name lookups, and the who-list row
//! probe used by the search UI.

use super::ensure_namespace;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_set_static,
};
use crate::lua_api::state::SocialFriend;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

#[derive(Clone, Copy)]
struct FriendListRow {
    name: &'static str,
    level: i32,
    area: &'static str,
    class_name: &'static str,
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
        race_str: "Human",
        gender: 2,
        full_guild_name: "Heroes of Azeroth",
    },
    FriendListRow {
        name: "Brennor",
        level: 80,
        area: "Duskwood",
        class_name: "Warrior",
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
    table_set_rust_fn_static(state, table_ref, "GetNumWhoResults", get_num_who_results)?;
    table_set_rust_fn_static(state, table_ref, "SetWhoToUi", set_who_to_ui)?;
    table_set_rust_fn_static(state, table_ref, "ShowFriends", show_friends)?;
    Ok(())
}

fn get_num_friends(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.social_friends.len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn get_num_online_friends(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?
        .social_friends
        .iter()
        .filter(|friend| friend.is_online)
        .count();
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
    let Some(friend) = social_friend_by_display_index(state, index)? else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let table = build_friend_info_table(state, &friend);
    state.push(table);
    Ok(1)
}

fn get_friend_info_by_name(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let friend = {
        let sim = borrow_state(state)?;
        sim.social_friends
            .iter()
            .find(|friend| friend.name == name)
            .cloned()
    };
    let Some(friend) = friend else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let table = build_friend_info_table(state, &friend);
    state.push(table);
    Ok(1)
}

fn is_friend(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let is_friend = borrow_state(state)?
        .social_friends
        .iter()
        .any(|friend| friend.name == name);
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

fn get_num_who_results(state: &mut LuaState) -> LuaResult<u32> {
    let who_count = FRIEND_LIST_ROWS.len() as f64;
    state.push(Val::Num(who_count));
    state.push(Val::Num(who_count));
    Ok(2)
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

fn social_friend_by_display_index(
    state: &mut LuaState,
    index: i32,
) -> LuaResult<Option<SocialFriend>> {
    let Some(zero_based) = usize::try_from(index - 1).ok() else {
        return Ok(None);
    };
    let mut friends = borrow_state(state)?.social_friends.clone();
    friends.sort_by_key(|friend| !friend.is_online);
    Ok(friends.get(zero_based).cloned())
}

fn build_friend_info_table(state: &mut LuaState, friend: &SocialFriend) -> Val {
    let table = create_table(state);
    let name = create_string(state, &friend.name);
    let full_name = create_string(state, &friend.name);
    let area = create_string(state, &friend.area);
    let class_name = create_string(state, &friend.class_name);
    let class_str = create_string(state, &friend.class_name);
    let filename = create_string(state, &friend.class_name.to_uppercase());
    let notes = create_string(state, &friend.note);
    let guid = create_string(state, &friend.guid);
    let race_str = create_string(state, "Human");
    let full_guild_name = create_string(state, "Heroes of Azeroth");
    table_set_static(state, table, "name", name);
    table_set_static(state, table, "fullName", full_name);
    table_set_static(state, table, "level", Val::Num(friend.level as f64));
    table_set_static(state, table, "area", area);
    table_set_static(state, table, "className", class_name);
    table_set_static(state, table, "classStr", class_str);
    table_set_static(state, table, "filename", filename);
    table_set_static(state, table, "notes", notes);
    table_set_static(state, table, "connected", Val::Bool(friend.is_online));
    table_set_static(state, table, "guid", guid);
    table_set_static(state, table, "raceStr", race_str);
    table_set_static(state, table, "gender", Val::Num(2.0));
    table_set_static(state, table, "fullGuildName", full_guild_name);
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
    table_set_static(state, table, "fullName", full_name);
    table_set_static(state, table, "name", name);
    table_set_static(state, table, "level", Val::Num(row.level as f64));
    table_set_static(state, table, "classStr", class_str);
    table_set_static(state, table, "filename", filename);
    table_set_static(state, table, "area", area);
    table_set_static(state, table, "raceStr", race_str);
    table_set_static(state, table, "gender", Val::Num(row.gender as f64));
    table_set_static(state, table, "fullGuildName", full_guild_name);
    table
}
