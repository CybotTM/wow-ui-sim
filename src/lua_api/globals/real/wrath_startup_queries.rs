//! Legacy readers over existing simulator state; Wrath registration only.

use crate::lua_api::globals::group_queries::relationships::is_group_leader;
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

const OFFHAND_SLOT: i32 = 17;
const SIMULATED_RAID_MIN_COMPANIONS: usize = 6;

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    register_legacy_friend_count(lua)?;
    LuaApiMut::register_function(lua, "GetTrackedAchievements", get_tracked_achievements)?;
    LuaApiMut::register_function(lua, "OffhandHasWeapon", offhand_has_weapon)?;
    LuaApiMut::register_function(lua, "GetTrainerServiceTypeFilter", get_trainer_filter)?;
    LuaApiMut::register_function(lua, "SetTrainerServiceTypeFilter", set_trainer_filter)?;
    LuaApiMut::register_function(lua, "IsRaidLeader", is_raid_leader)?;
    // The Wrath consumer combines this with IsRaidLeader. Do not read the
    // unrelated WoWLabs matchmaking party used by the earlier registration.
    LuaApiMut::register_function(lua, "IsPartyLeader", is_party_leader)?;
    LuaApiMut::register_function(lua, "GetPartyMember", get_party_member)?;
    LuaApiMut::register_function(lua, "GetPartyLeaderIndex", get_party_leader_index)?;
    super::wrath_voice::register_all(lua)?;
    crate::lua_api::workarounds::wrath_startup_defaults::register_all(lua)?;
    Ok(())
}

fn register_legacy_friend_count(lua: &mut rilua::Lua) -> crate::Result<()> {
    // The namespace owns social_friends reads; Wrath consumes a two-value tuple.
    lua.exec(
        r#"
        function GetNumFriends()
            return C_FriendList.GetNumFriends(), C_FriendList.GetNumOnlineFriends()
        end
        "#,
    )?;
    Ok(())
}

fn get_tracked_achievements(state: &mut LuaState) -> LuaResult<u32> {
    let mut ids: Vec<_> = borrow_state(state)?
        .tracked_achievements
        .iter()
        .copied()
        .collect();
    // Deterministic simulator order, not a measured native ordering contract.
    ids.sort_unstable();
    let count = ids.len() as u32;
    for id in ids {
        state.push(Val::Num(f64::from(id)));
    }
    Ok(count)
}

fn offhand_has_weapon(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = borrow_state(state)?
        .player
        .equipped_items
        .get(&OFFHAND_SLOT)
        .map(|item| item.item_id);
    let weapon = item_id.is_some_and(catalogued_item_is_weapon);
    state.push(Val::Bool(weapon));
    Ok(1)
}

fn catalogued_item_is_weapon(item_id: u32) -> bool {
    let Some(item) = crate::items::get_item(item_id) else {
        return crate::lua_api::workarounds::wrath_startup_defaults::unknown_offhand_is_weapon();
    };
    // Weapon, two-hand weapon, main-hand weapon and off-hand weapon. Shields
    // (14) and holdables (23) are not weapons; identity comes from equipped slot 17.
    matches!(item.inventory_type, 13 | 17 | 21 | 22)
}

fn trainer_filter_index(state: &LuaState) -> LuaResult<usize> {
    let category = String::from_stack(state, 1)?;
    match category.as_str() {
        "available" => Ok(0),
        "unavailable" => Ok(1),
        "used" => Ok(2),
        _ => Err(runtime_error("unknown trainer service filter category")),
    }
}

fn get_trainer_filter(state: &mut LuaState) -> LuaResult<u32> {
    let index = trainer_filter_index(state)?;
    let shown = borrow_state(state)?.wrath_trainer_filters[index];
    state.push(Val::Bool(shown));
    Ok(1)
}

fn set_trainer_filter(state: &mut LuaState) -> LuaResult<u32> {
    let index = trainer_filter_index(state)?;
    // Original Wrath dropdown supplies numeric 0/1, not Lua truthiness (0 is true).
    let shown = i32::from_stack(state, 2)? != 0;
    borrow_state_mut(state)?.wrath_trainer_filters[index] = shown;
    Ok(0)
}

fn is_raid_leader(state: &mut LuaState) -> LuaResult<u32> {
    let leader = {
        let sim = borrow_state(state)?;
        let raid = sim.party_members.len() >= SIMULATED_RAID_MIN_COMPANIONS;
        raid && is_group_leader(&sim, "player")
    };
    state.push(Val::Bool(leader));
    Ok(1)
}

fn get_party_leader_index(state: &mut LuaState) -> LuaResult<u32> {
    let leader = {
        let sim = borrow_state(state)?;
        // Shared leader queries use zero-based member indices; None is player.
        sim.party_leader_index
            .filter(|&index| sim.party_group_active && index < 4 && index < sim.party_members.len())
            .map_or(0, |index| index + 1)
    };
    state.push(Val::Num(leader as f64));
    Ok(1)
}

fn get_party_member(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let valid_slot = (1..=4).contains(&index);
    let present = {
        let sim = borrow_state(state)?;
        valid_slot && sim.party_group_active && index as usize <= sim.party_members.len()
    };
    state.push(Val::Bool(present));
    Ok(1)
}

fn is_party_leader(state: &mut LuaState) -> LuaResult<u32> {
    let leader = {
        let sim = borrow_state(state)?;
        let party = sim.party_members.len() < SIMULATED_RAID_MIN_COMPANIONS;
        party && is_group_leader(&sim, "player")
    };
    state.push(Val::Bool(leader));
    Ok(1)
}
