//! Seeded WoW Labs / Plunderstorm namespaces used by Blizzard UI tests.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, create_table, table_set};
use crate::lua_api::state::{
    WowLabsAreaInfo, WowLabsCircleInfo, WowLabsDataManagerState, WowLabsMatchmakingState,
    WowLabsPartyInvite, WowLabsPartyMember, WowLabsPoint,
};
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const C_WOWLABS: &str = "C_WowLabs";
const C_WOWLABS_DATA_MANAGER: &str = "C_WowLabsDataManager";
const C_WOWLABS_MATCHMAKING: &str = "C_WoWLabsMatchmaking";
const LOCAL_WOWLABS_GUID: &str = "WoWLabsPlayer-Local";

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    let state = lua.state_mut();
    register_namespace(state, C_WOWLABS, wowlabs_functions())?;
    register_namespace(state, C_WOWLABS_DATA_MANAGER, data_manager_functions())?;
    register_namespace(state, C_WOWLABS_MATCHMAKING, matchmaking_functions())?;
    Ok(())
}

fn wowlabs_functions() -> [(&'static str, rilua::RustFn); 3] {
    [
        ("GetAvailableQueues", get_available_queues),
        ("GetMatchmakingEnabled", get_matchmaking_enabled),
        ("IsEnabled", is_enabled),
    ]
}

fn data_manager_functions() -> [(&'static str, rilua::RustFn); 7] {
    [
        ("GetConfirmedWoWLabsArea", get_confirmed_wowlabs_area),
        ("GetWoWLabsAreaInfo", get_wowlabs_area_info),
        ("IsInPrematch", is_in_prematch),
        ("PushCircleInfoToLua", push_circle_info_to_lua),
        ("QuerySelectedWoWLabsArea", query_selected_wowlabs_area),
        ("QueryWoWLabsAreaInfo", query_wowlabs_area_info),
        ("SelectWoWLabsArea", select_wowlabs_area),
    ]
}

fn matchmaking_functions() -> [(&'static str, rilua::RustFn); 25] {
    [
        ("AcceptPartyInvite", accept_party_invite),
        ("CanEnterMatchmaking", can_enter_matchmaking),
        ("ClearFastLogin", clear_fast_login),
        ("DeclinePartyInvite", decline_party_invite),
        ("GetAutoQueueOnLogout", get_auto_queue_on_logout),
        ("GetCurrentParty", get_current_party),
        ("GetInQueueTimeStart", get_in_queue_time_start),
        ("GetNumPartyInvites", get_num_party_invites),
        ("GetPartyInviteByIndex", get_party_invite_by_index),
        ("GetPartyPlaylistEntry", get_party_playlist_entry),
        ("GetPartySize", get_party_size),
        ("IsAloneInWoWLabsParty", is_alone_in_wowlabs_party),
        ("IsFastLogin", is_fast_login),
        ("IsFindingMatch", is_finding_match),
        ("IsPartyFull", is_party_full),
        ("IsPartyLeader", is_party_leader),
        ("IsPlayer", is_player),
        ("IsPlayerReady", is_player_ready),
        ("IsWowLabsMatchmakingMember", is_wowlabs_matchmaking_member),
        ("LeaveParty", leave_party),
        ("RemovePlayerFromParty", remove_player_from_party),
        ("SendPartyInvite", send_party_invite),
        ("SetAutoQueueOnLogout", set_auto_queue_on_logout),
        ("SetPartyPlaylistEntry", set_party_playlist_entry),
        ("SetPlayerReady", set_player_ready),
    ]
}

fn register_namespace(
    state: &mut LuaState,
    namespace: &'static str,
    methods: impl IntoIterator<Item = (&'static str, rilua::RustFn)>,
) -> LuaResult<()> {
    let table_ref = ensure_namespace_table(state, namespace);
    for (name, func) in methods {
        table_set_rust_fn(state, table_ref, name, func)?;
    }
    Ok(())
}

fn ensure_namespace_table(state: &mut LuaState, namespace: &'static str) -> GcRef<Table> {
    let key = state.gc.intern_string_static(namespace.as_bytes());
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|table| table.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(table_ref)) = existing {
        return table_ref;
    }

    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), table, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    table_ref
}

fn set_array_entry(state: &mut LuaState, table_ref: GcRef<Table>, index: i64, value: Val) {
    if let Some(table) = state.gc.tables.get_mut(table_ref) {
        let _ = table.raw_set(Val::Num(index as f64), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(table_ref);
}

fn local_member(player_name: &str, is_party_leader: bool) -> WowLabsPartyMember {
    WowLabsPartyMember {
        player_name: player_name.to_string(),
        party_member_guid: LOCAL_WOWLABS_GUID.to_string(),
        is_local_player: true,
        is_party_leader,
        is_ready: false,
    }
}

fn queue_capacity(playlist_entry: i32) -> usize {
    match playlist_entry {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 1,
        _ => 0,
    }
}

fn reset_queue_state(state: &mut WowLabsMatchmakingState) {
    state.is_player_ready = false;
    state.is_finding_match = false;
    state.in_queue_time_start = 0.0;
}

fn can_queue(state: &WowLabsMatchmakingState) -> bool {
    state.party_members.len() <= queue_capacity(state.party_playlist_entry)
}

fn push_array_numbers(state: &mut LuaState, values: &[i32]) -> LuaResult<u32> {
    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        unreachable!("create_table must return a table");
    };
    for (index, value) in values.iter().enumerate() {
        set_array_entry(state, table_ref, index as i64 + 1, Val::Num(*value as f64));
    }
    state.push(table);
    Ok(1)
}

fn push_party_members(state: &mut LuaState, members: &[WowLabsPartyMember]) -> LuaResult<u32> {
    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        unreachable!("create_table must return a table");
    };
    for (index, member) in members.iter().enumerate() {
        let member_table = create_table(state);
        table_set(state, member_table, "playerName", create_string(state, &member.player_name));
        table_set(
            state,
            member_table,
            "partyMemberGUID",
            create_string(state, &member.party_member_guid),
        );
        table_set(
            state,
            member_table,
            "isLocalPlayer",
            Val::Bool(member.is_local_player),
        );
        table_set(
            state,
            member_table,
            "isPartyLeader",
            Val::Bool(member.is_party_leader),
        );
        table_set(state, member_table, "isReady", Val::Bool(member.is_ready));
        set_array_entry(state, table_ref, index as i64 + 1, member_table);
    }
    state.push(table);
    Ok(1)
}

fn push_area_info(state: &mut LuaState, areas: &[WowLabsAreaInfo]) -> LuaResult<u32> {
    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        unreachable!("create_table must return a table");
    };
    for (index, area) in areas.iter().enumerate() {
        let area_table = create_table(state);
        table_set(
            state,
            area_table,
            "wowLabsAreaID",
            Val::Num(area.wow_labs_area_id as f64),
        );
        table_set(state, area_table, "x", Val::Num(area.x));
        table_set(state, area_table, "y", Val::Num(area.y));
        table_set(
            state,
            area_table,
            "areaType",
            Val::Num(area.area_type as f64),
        );
        set_array_entry(state, table_ref, index as i64 + 1, area_table);
    }
    state.push(table);
    Ok(1)
}

fn push_point_table(state: &mut LuaState, point: WowLabsPoint) {
    let table = create_table(state);
    table_set(state, table, "x", Val::Num(point.x));
    table_set(state, table, "y", Val::Num(point.y));
    state.push(table);
}

fn snapshot_party_members(state: &mut LuaState) -> LuaResult<Vec<WowLabsPartyMember>> {
    let sim = borrow_state(state)?;
    let local_name = sim.player.name.clone();
    let members = sim
        .wowlabs
        .matchmaking
        .party_members
        .iter()
        .cloned()
        .map(|mut member| {
            if member.is_local_player {
                member.player_name = local_name.clone();
            }
            member
        })
        .collect();
    Ok(members)
}

fn snapshot_invites(state: &mut LuaState) -> LuaResult<Vec<WowLabsPartyInvite>> {
    Ok(borrow_state(state)?.wowlabs.matchmaking.party_invites.clone())
}

fn snapshot_matchmaking(state: &mut LuaState) -> LuaResult<WowLabsMatchmakingState> {
    let mut snapshot = borrow_state(state)?.wowlabs.matchmaking.clone();
    if let Some(member) = snapshot.party_members.iter_mut().find(|member| member.is_local_player) {
        member.player_name = borrow_state(state)?.player.name.clone();
    }
    Ok(snapshot)
}

fn snapshot_data_manager(state: &mut LuaState) -> LuaResult<WowLabsDataManagerState> {
    Ok(borrow_state(state)?.wowlabs.data_manager.clone())
}

fn get_available_queues(state: &mut LuaState) -> LuaResult<u32> {
    let queues = borrow_state(state)?.wowlabs.available_queues.clone();
    push_array_numbers(state, &queues)
}

fn get_matchmaking_enabled(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(borrow_state(state)?.wowlabs.matchmaking_enabled));
    Ok(1)
}

fn is_enabled(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(borrow_state(state)?.wowlabs.enabled));
    Ok(1)
}

fn get_confirmed_wowlabs_area(state: &mut LuaState) -> LuaResult<u32> {
    match snapshot_data_manager(state)?.confirmed_area_id {
        Some(area_id) => state.push(Val::Num(area_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_wowlabs_area_info(state: &mut LuaState) -> LuaResult<u32> {
    let areas = snapshot_data_manager(state)?.areas;
    push_area_info(state, &areas)
}

fn is_in_prematch(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(snapshot_data_manager(state)?.in_prematch));
    Ok(1)
}

fn push_circle_info_to_lua(state: &mut LuaState) -> LuaResult<u32> {
    let WowLabsCircleInfo {
        start_lerp_time,
        time_to_lerp,
        outer_position,
        inner_position,
        base_radius,
        outer_scale,
        inner_scale,
        prediction_position,
        prediction_scale,
        initial_base_size,
    } = snapshot_data_manager(state)?.circle_info;
    state.push(Val::Num(start_lerp_time));
    state.push(Val::Num(time_to_lerp));
    push_point_table(state, outer_position);
    push_point_table(state, inner_position);
    state.push(Val::Num(base_radius));
    state.push(Val::Num(outer_scale));
    state.push(Val::Num(inner_scale));
    push_point_table(state, prediction_position);
    state.push(Val::Num(prediction_scale));
    state.push(Val::Num(initial_base_size));
    Ok(10)
}

fn query_selected_wowlabs_area(state: &mut LuaState) -> LuaResult<u32> {
    match snapshot_data_manager(state)?.selected_area_id {
        Some(area_id) => state.push(Val::Num(area_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn query_wowlabs_area_info(state: &mut LuaState) -> LuaResult<u32> {
    get_wowlabs_area_info(state)
}

fn select_wowlabs_area(state: &mut LuaState) -> LuaResult<u32> {
    let area_id = i32::from_stack(state, 1)?;
    let selected = {
        let mut sim = borrow_state_mut(state)?;
        let data = &mut sim.wowlabs.data_manager;
        if data.areas.iter().any(|area| area.wow_labs_area_id == area_id) {
            data.selected_area_id = Some(area_id);
            data.confirmed_area_id = Some(area_id);
            true
        } else {
            false
        }
    };
    state.push(Val::Bool(selected));
    Ok(1)
}

fn accept_party_invite(state: &mut LuaState) -> LuaResult<u32> {
    let invite_id = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let accepted = {
        let mut sim = borrow_state_mut(state)?;
        let local_name = sim.player.name.clone();
        let matchmaking = &mut sim.wowlabs.matchmaking;
        if let Some(index) = matchmaking
            .party_invites
            .iter()
            .position(|invite| invite.invite_id == invite_id)
        {
            let invite = matchmaking.party_invites.remove(index);
            matchmaking.party_members = vec![
                WowLabsPartyMember {
                    player_name: invite.inviter_name,
                    party_member_guid: invite.inviter_guid,
                    is_local_player: false,
                    is_party_leader: true,
                    is_ready: false,
                },
                local_member(&local_name, false),
            ];
            reset_queue_state(matchmaking);
            true
        } else {
            false
        }
    };
    state.push(Val::Bool(accepted));
    Ok(1)
}

fn can_enter_matchmaking(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(can_queue(&snapshot_matchmaking(state)?)));
    Ok(1)
}

fn clear_fast_login(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.wowlabs.matchmaking.fast_login = false;
    Ok(0)
}

fn decline_party_invite(state: &mut LuaState) -> LuaResult<u32> {
    let invite_id = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let declined = {
        let mut sim = borrow_state_mut(state)?;
        let invites = &mut sim.wowlabs.matchmaking.party_invites;
        if let Some(index) = invites.iter().position(|invite| invite.invite_id == invite_id) {
            invites.remove(index);
            true
        } else {
            false
        }
    };
    state.push(Val::Bool(declined));
    Ok(1)
}

fn get_auto_queue_on_logout(state: &mut LuaState) -> LuaResult<u32> {
    let snapshot = snapshot_matchmaking(state)?;
    state.push(Val::Bool(snapshot.auto_queue_on_logout));
    state.push(Val::Num(snapshot.auto_queue_queue_type as f64));
    Ok(2)
}

fn get_current_party(state: &mut LuaState) -> LuaResult<u32> {
    let members = snapshot_party_members(state)?;
    push_party_members(state, &members)
}

fn get_in_queue_time_start(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(snapshot_matchmaking(state)?.in_queue_time_start));
    Ok(1)
}

fn get_num_party_invites(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(snapshot_invites(state)?.len() as f64));
    Ok(1)
}

fn get_party_invite_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0).max(0.0) as usize;
    match snapshot_invites(state)?.get(index) {
        Some(invite) => {
            state.push(create_string(state, &invite.inviter_name));
            state.push(create_string(state, &invite.invite_id));
            Ok(2)
        }
        None => Ok(0),
    }
}

fn get_party_playlist_entry(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(snapshot_matchmaking(state)?.party_playlist_entry as f64));
    Ok(1)
}

fn get_party_size(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(snapshot_matchmaking(state)?.party_members.len() as f64));
    Ok(1)
}

fn is_alone_in_wowlabs_party(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(snapshot_matchmaking(state)?.party_members.len() <= 1));
    Ok(1)
}

fn is_fast_login(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(snapshot_matchmaking(state)?.fast_login));
    Ok(1)
}

fn is_finding_match(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(snapshot_matchmaking(state)?.is_finding_match));
    Ok(1)
}

fn is_party_full(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(snapshot_matchmaking(state)?.party_members.len() >= 3));
    Ok(1)
}

fn is_party_leader(state: &mut LuaState) -> LuaResult<u32> {
    let is_leader = snapshot_matchmaking(state)?
        .party_members
        .iter()
        .find(|member| member.is_local_player)
        .map(|member| member.is_party_leader)
        .unwrap_or(false);
    state.push(Val::Bool(is_leader));
    Ok(1)
}

fn is_player(state: &mut LuaState) -> LuaResult<u32> {
    let guid = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let is_member = snapshot_matchmaking(state)?
        .party_members
        .iter()
        .any(|member| member.party_member_guid == guid);
    state.push(Val::Bool(is_member));
    Ok(1)
}

fn is_player_ready(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(snapshot_matchmaking(state)?.is_player_ready));
    Ok(1)
}

fn is_wowlabs_matchmaking_member(state: &mut LuaState) -> LuaResult<u32> {
    is_player(state)
}

fn leave_party(state: &mut LuaState) -> LuaResult<u32> {
    let left = {
        let mut sim = borrow_state_mut(state)?;
        let local_name = sim.player.name.clone();
        let matchmaking = &mut sim.wowlabs.matchmaking;
        matchmaking.party_members = vec![local_member(&local_name, true)];
        reset_queue_state(matchmaking);
        true
    };
    state.push(Val::Bool(left));
    Ok(1)
}

fn remove_player_from_party(state: &mut LuaState) -> LuaResult<u32> {
    let guid = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let removed = {
        let mut sim = borrow_state_mut(state)?;
        let local_name = sim.player.name.clone();
        let matchmaking = &mut sim.wowlabs.matchmaking;
        if guid == LOCAL_WOWLABS_GUID {
            matchmaking.party_members = vec![local_member(&local_name, true)];
            reset_queue_state(matchmaking);
            true
        } else if let Some(index) = matchmaking
            .party_members
            .iter()
            .position(|member| member.party_member_guid == guid && !member.is_local_player)
        {
            matchmaking.party_members.remove(index);
            if let Some(local) = matchmaking
                .party_members
                .iter_mut()
                .find(|member| member.is_local_player)
            {
                local.is_party_leader = true;
            }
            true
        } else {
            false
        }
    };
    state.push(Val::Bool(removed));
    Ok(1)
}

fn send_party_invite(state: &mut LuaState) -> LuaResult<u32> {
    let can_send = !is_party_full_now(state)?;
    state.push(Val::Bool(can_send));
    Ok(1)
}

fn is_party_full_now(state: &mut LuaState) -> LuaResult<bool> {
    Ok(snapshot_matchmaking(state)?.party_members.len() >= 3)
}

fn set_auto_queue_on_logout(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = Option::<bool>::from_stack(state, 1)?.unwrap_or(false);
    let queue_type = Option::<f64>::from_stack(state, 2)?.map(|value| value as i32);
    let mut sim = borrow_state_mut(state)?;
    let matchmaking = &mut sim.wowlabs.matchmaking;
    matchmaking.auto_queue_on_logout = enabled;
    if let Some(queue_type) = queue_type {
        matchmaking.auto_queue_queue_type = queue_type;
    }
    Ok(0)
}

fn set_party_playlist_entry(state: &mut LuaState) -> LuaResult<u32> {
    let playlist_entry = i32::from_stack(state, 1)?;
    let updated = {
        let mut sim = borrow_state_mut(state)?;
        let matchmaking = &mut sim.wowlabs.matchmaking;
        let is_valid = (0..=3).contains(&playlist_entry);
        let fits_party = matchmaking.party_members.len() <= queue_capacity(playlist_entry);
        if is_valid && fits_party {
            matchmaking.party_playlist_entry = playlist_entry;
            true
        } else {
            false
        }
    };
    state.push(Val::Bool(updated));
    Ok(1)
}

fn set_player_ready(state: &mut LuaState) -> LuaResult<u32> {
    let is_ready = Option::<bool>::from_stack(state, 1)?.unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    let matchmaking = &mut sim.wowlabs.matchmaking;
    matchmaking.is_player_ready = is_ready;
    if is_ready && can_queue(matchmaking) && local_player_is_leader(matchmaking) {
        matchmaking.is_finding_match = true;
        matchmaking.in_queue_time_start = 1.0;
    } else if !is_ready {
        matchmaking.is_finding_match = false;
        matchmaking.in_queue_time_start = 0.0;
    }
    Ok(0)
}

fn local_player_is_leader(matchmaking: &WowLabsMatchmakingState) -> bool {
    matchmaking
        .party_members
        .iter()
        .find(|member| member.is_local_player)
        .map(|member| member.is_party_leader)
        .unwrap_or(false)
}
