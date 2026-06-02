//! `C_BattleNet` probe surface backed by `SimState.bnet_friends`.
//!
//! Migrates 5 entries off the namespace stub tables:
//!
//! - `C_BattleNet.GetNumFriends()` — returns the count of seeded bnet
//!   friends.
//! - `C_BattleNet.GetFriendAccountInfo(friendIndex, [wowAccountGUID])`
//!   — returns a `BNetAccountInfo` table for a 1-based friend index,
//!   or nil when out of range. The optional `wowAccountGUID` parameter
//!   is accepted but ignored (retail uses it to pick among multiple game
//!   accounts; we always return the first game-account entry).
//! - `C_BattleNet.GetAccountInfoByGUID(bnetAccountGUID)` — returns a
//!   `BNetAccountInfo` table for the friend matching the given bnet
//!   account GUID, or nil when unknown.
//! - `C_BattleNet.GetGameAccountInfoByGUID(wowAccountGUID)` — returns a
//!   `BNetGameAccountInfo` table for the game account matching the given
//!   WoW account GUID, or nil when unknown.
//! - `C_BattleNet.GetFriendNumAccounts(friendIndex)` — returns the
//!   number of game accounts for a 1-based friend index, or 0 when out
//!   of range.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_set,
};
use crate::lua_api::state_types::{BnetFriend, BnetGameAccount};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

type BattleNetTable = GcRef<Table>;

pub(crate) fn register_c_battle_net_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_BattleNet")?;
    register_texture_methods(state, table_ref)?;
    register_friend_query_methods(state, table_ref)
}

fn register_texture_methods(state: &mut LuaState, table_ref: BattleNetTable) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "AreHighResTexturesInstalled",
        c_bnet_are_high_res_textures_installed,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "InstallHighResTextures",
        c_bnet_install_high_res_textures,
    )
}

fn register_friend_query_methods(state: &mut LuaState, table_ref: BattleNetTable) -> LuaResult<()> {
    table_set_rust_fn_static(state, table_ref, "GetNumFriends", c_bnet_get_num_friends)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetFriendAccountInfo",
        c_bnet_get_friend_account_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAccountInfoByGUID",
        c_bnet_get_account_info_by_guid,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetGameAccountInfoByGUID",
        c_bnet_get_game_account_info_by_guid,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetFriendNumAccounts",
        c_bnet_get_friend_num_accounts,
    )
}

fn c_bnet_are_high_res_textures_installed(state: &mut LuaState) -> LuaResult<u32> {
    let installed = borrow_state(state)?.cvars.get_bool("useHighResTextures");
    state.push(Val::Bool(installed));
    Ok(1)
}

fn c_bnet_get_num_friends(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.bnet_friends.len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn c_bnet_get_friend_account_info(state: &mut LuaState) -> LuaResult<u32> {
    let friend_index = i32::from_stack(state, 1)?;
    let entry = {
        let sim = borrow_state(state)?;
        let idx = usize::try_from(friend_index - 1).unwrap_or(usize::MAX);
        sim.bnet_friends.get(idx).cloned()
    };
    let Some(friend) = entry else {
        state.push(Val::Nil);
        return Ok(1);
    };
    // Pick the first game account (retail: caller can pass wowAccountGUID
    // as arg 2 to select a specific one; we return the first online one
    // or the first available).
    let game_account = friend.game_accounts.first().cloned();
    let t = push_account_info_table(state, &friend, game_account.as_ref());
    state.push(t);
    Ok(1)
}

fn c_bnet_get_account_info_by_guid(state: &mut LuaState) -> LuaResult<u32> {
    let guid = match crate::lua_bridge::stack_val(state, 1) {
        Val::Str(_) => {
            crate::lua_api::methods::val_to_string(state, crate::lua_bridge::stack_val(state, 1))
                .unwrap_or_default()
        }
        _ => {
            state.push(Val::Nil);
            return Ok(1);
        }
    };
    let entry = {
        let sim = borrow_state(state)?;
        sim.bnet_friends
            .iter()
            .find(|f| f.bnet_account_guid == guid)
            .cloned()
    };
    let Some(friend) = entry else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let game_account = friend.game_accounts.first().cloned();
    let t = push_account_info_table(state, &friend, game_account.as_ref());
    state.push(t);
    Ok(1)
}

fn c_bnet_get_game_account_info_by_guid(state: &mut LuaState) -> LuaResult<u32> {
    let guid = match crate::lua_bridge::stack_val(state, 1) {
        Val::Str(_) => {
            crate::lua_api::methods::val_to_string(state, crate::lua_bridge::stack_val(state, 1))
                .unwrap_or_default()
        }
        _ => {
            state.push(Val::Nil);
            return Ok(1);
        }
    };
    let entry = {
        let sim = borrow_state(state)?;
        sim.bnet_friends
            .iter()
            .flat_map(|f| f.game_accounts.iter())
            .find(|g| g.wow_account_guid == guid)
            .cloned()
    };
    let Some(ga) = entry else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let t = push_game_account_info_table(state, &ga);
    state.push(t);
    Ok(1)
}

fn c_bnet_get_friend_num_accounts(state: &mut LuaState) -> LuaResult<u32> {
    let friend_index = i32::from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        usize::try_from(friend_index - 1)
            .ok()
            .and_then(|idx| sim.bnet_friends.get(idx))
            .map(|f| f.game_accounts.len())
            .unwrap_or(0)
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn c_bnet_install_high_res_textures(state: &mut LuaState) -> LuaResult<u32> {
    let _ = borrow_state_mut(state)?
        .cvars
        .set("useHighResTextures", "1");
    Ok(0)
}

/// Build a `BNetAccountInfo` Lua table from `friend` + optional
/// `game_account`. The `gameAccountInfo` nested table is set when a
/// game account is provided, or `nil` otherwise.
fn push_account_info_table(
    state: &mut LuaState,
    friend: &BnetFriend,
    game_account: Option<&BnetGameAccount>,
) -> Val {
    let t = create_table(state);
    write_account_identity_fields(state, t, friend);
    write_account_status_fields(state, t, friend);
    attach_game_account_field(state, t, game_account);
    t
}

/// Build a `BNetGameAccountInfo` Lua table from a `BnetGameAccount`.
fn push_game_account_info_table(state: &mut LuaState, ga: &BnetGameAccount) -> Val {
    let t = create_table(state);
    write_game_account_character(state, t, ga);
    write_game_account_presence(state, t, ga);
    write_game_account_meta(state, t, ga);
    t
}

fn write_game_account_character(state: &mut LuaState, t: Val, ga: &BnetGameAccount) {
    let character_name = create_string(state, &ga.character_name);
    let realm_name = create_string(state, &ga.realm_name);
    let realm_display_name = create_string(state, &ga.realm_display_name);
    let class_name = create_string(state, &ga.class_name);
    let faction_name = create_string(state, &ga.faction_name);
    let race_name = create_string(state, &ga.race_name);
    table_set(state, t, "characterName", character_name);
    table_set(state, t, "realmName", realm_name);
    table_set(state, t, "realmDisplayName", realm_display_name);
    table_set(state, t, "realmID", Val::Num(ga.realm_id as f64));
    table_set(state, t, "classID", Val::Num(ga.class_id as f64));
    table_set(state, t, "className", class_name);
    table_set(
        state,
        t,
        "characterLevel",
        Val::Num(ga.character_level as f64),
    );
    table_set(state, t, "factionName", faction_name);
    table_set(state, t, "raceName", race_name);
}

fn write_game_account_presence(state: &mut LuaState, t: Val, ga: &BnetGameAccount) {
    let area_name = create_string(state, &ga.area_name);
    let client_program = create_string(state, &ga.client_program);
    let rich_presence = create_string(state, &ga.rich_presence);
    table_set(state, t, "areaName", area_name);
    table_set(state, t, "isOnline", Val::Bool(ga.is_online));
    table_set(state, t, "isGameAFK", Val::Bool(ga.is_game_afk));
    table_set(state, t, "isGameBusy", Val::Bool(ga.is_game_busy));
    table_set(state, t, "clientProgram", client_program);
    table_set(state, t, "richPresence", rich_presence);
    table_set(state, t, "hasFocus", Val::Bool(ga.has_focus));
}

fn write_game_account_meta(state: &mut LuaState, t: Val, ga: &BnetGameAccount) {
    let player_guid = create_string(state, &ga.player_guid);
    table_set(state, t, "canSummon", Val::Bool(ga.can_summon));
    table_set(
        state,
        t,
        "isInCurrentRegion",
        Val::Bool(ga.is_in_current_region),
    );
    table_set(
        state,
        t,
        "gameAccountID",
        Val::Num(ga.game_account_id as f64),
    );
    table_set(state, t, "wowProjectID", Val::Num(ga.wow_project_id as f64));
    table_set(
        state,
        t,
        "timerunningSeasonID",
        Val::Num(ga.timerunning_season_id as f64),
    );
    table_set(state, t, "regionID", Val::Num(ga.region_id as f64));
    table_set(state, t, "playerGuid", player_guid);
}

fn write_account_identity_fields(state: &mut LuaState, t: Val, friend: &BnetFriend) {
    let battle_tag = create_string(state, &friend.battle_tag);
    let account_name = create_string(state, &friend.account_name);
    let note = create_string(state, &friend.note);
    let guid = create_string(state, &friend.bnet_account_guid);
    table_set(state, t, "battleTag", battle_tag);
    table_set(state, t, "accountName", account_name);
    table_set(state, t, "note", note);
    table_set(state, t, "bnetAccountGUID", guid);
    table_set(
        state,
        t,
        "bnetAccountID",
        Val::Num(friend.bnet_account_id as f64),
    );
}

fn write_account_status_fields(state: &mut LuaState, t: Val, friend: &BnetFriend) {
    write_custom_message_fields(state, t, friend);
    write_friend_relationship_flags(state, t, friend);
    write_presence_and_link_fields(state, t, friend);
}

/// `customMessage` + `customMessageTime` + `appearOffline` — the
/// user-set status block visible in the Battle.net friends panel.
fn write_custom_message_fields(state: &mut LuaState, t: Val, friend: &BnetFriend) {
    let custom_message = create_string(state, &friend.custom_message);
    table_set(state, t, "customMessage", custom_message);
    table_set(
        state,
        t,
        "customMessageTime",
        Val::Num(friend.custom_message_time as f64),
    );
    table_set(state, t, "appearOffline", Val::Bool(friend.appear_offline));
}

/// `isBattleTagFriend` / `isFriend` / `isFavorite` — the relationship
/// classification flags consumed by the friends list filter UI.
fn write_friend_relationship_flags(state: &mut LuaState, t: Val, friend: &BnetFriend) {
    table_set(
        state,
        t,
        "isBattleTagFriend",
        Val::Bool(friend.is_battle_tag_friend),
    );
    table_set(state, t, "isFriend", Val::Bool(friend.is_friend));
    table_set(state, t, "isFavorite", Val::Bool(friend.is_favorite));
}

/// `isAFK` / `isDND` / `lastOnlineTime` / `rafLinkType` — presence
/// state + Recruit-A-Friend link type returned by GetFriendAccountInfo.
fn write_presence_and_link_fields(state: &mut LuaState, t: Val, friend: &BnetFriend) {
    table_set(state, t, "isAFK", Val::Bool(friend.is_afk));
    table_set(state, t, "isDND", Val::Bool(friend.is_dnd));
    table_set(
        state,
        t,
        "lastOnlineTime",
        Val::Num(friend.last_online_time as f64),
    );
    table_set(
        state,
        t,
        "rafLinkType",
        Val::Num(friend.raf_link_type as f64),
    );
}

fn attach_game_account_field(state: &mut LuaState, t: Val, game_account: Option<&BnetGameAccount>) {
    let value = game_account
        .map(|ga| push_game_account_info_table(state, ga))
        .unwrap_or(Val::Nil);
    table_set(state, t, "gameAccountInfo", value);
}
