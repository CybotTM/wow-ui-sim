//! Legacy state readers and tracked-achievement mutations; Wrath registration only.

use crate::lua_api::globals::group_queries::relationships::is_group_leader;
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

const MAX_TRACKED_ACHIEVEMENTS: usize = 10;
const OFFHAND_SLOT: i32 = 17;
const SIMULATED_RAID_MIN_COMPANIONS: usize = 6;

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    register_legacy_friend_count(lua)?;
    super::wrath_friends::register_all(lua)?;
    LuaApiMut::register_function(lua, "GetTrackedAchievements", get_tracked_achievements)?;
    LuaApiMut::register_function(
        lua,
        "GetNumTrackedAchievements",
        get_num_tracked_achievements,
    )?;
    LuaApiMut::register_function(lua, "IsTrackedAchievement", is_tracked_achievement)?;
    LuaApiMut::register_function(lua, "AddTrackedAchievement", add_tracked_achievement)?;
    LuaApiMut::register_function(lua, "RemoveTrackedAchievement", remove_tracked_achievement)?;
    LuaApiMut::register_function(lua, "OffhandHasWeapon", offhand_has_weapon)?;
    LuaApiMut::register_function(lua, "GetTrainerServiceTypeFilter", get_trainer_filter)?;
    LuaApiMut::register_function(lua, "SetTrainerServiceTypeFilter", set_trainer_filter)?;
    LuaApiMut::register_function(lua, "IsRaidLeader", is_raid_leader)?;
    // The Wrath consumer combines this with IsRaidLeader. Do not read the
    // unrelated WoWLabs matchmaking party used by the earlier registration.
    LuaApiMut::register_function(lua, "IsPartyLeader", is_party_leader)?;
    LuaApiMut::register_function(lua, "GetPartyMember", get_party_member)?;
    LuaApiMut::register_function(lua, "GetPartyLeaderIndex", get_party_leader_index)?;
    super::legacy_gamma::register_all(lua)?;
    super::wrath_voice::register_all(lua)?;
    crate::lua_api::workarounds::wrath_startup_defaults::register_all(lua)?;
    crate::lua_api::workarounds::register_legacy_quest_timers(lua)?;
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
    let ids = borrow_state(state)?.tracked_achievements.clone();
    let count = ids.len() as u32;
    for id in ids {
        state.push(Val::Num(f64::from(id)));
    }
    Ok(count)
}

fn get_num_tracked_achievements(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.tracked_achievements.len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn tracked_achievement_id(state: &mut LuaState) -> LuaResult<i32> {
    let argument = stack_val(state, 1);
    let original_base = state.base;
    let original_top = state.top;
    // Use the VM's numeric conversion, isolated from extras and global overrides.
    state.base = original_top;
    state.push(argument);
    let converted = rilua::stdlib::base::lua_tonumber(state)
        .map(|_| state.stack_get(original_top + 1).as_number().unwrap_or(0.0) as i32);
    state.base = original_base;
    state.top = original_top;
    converted
}

fn is_tracked_achievement(state: &mut LuaState) -> LuaResult<u32> {
    let id = tracked_achievement_id(state)?;
    let tracked = borrow_state(state)?.tracked_achievements.contains(&id);
    state.push(Val::Bool(tracked));
    Ok(1)
}

fn add_tracked_achievement(state: &mut LuaState) -> LuaResult<u32> {
    let id = tracked_achievement_id(state)?;
    {
        let mut sim = borrow_state_mut(state)?;
        if id == 0 || !sim.achievements.contains_key(&id) {
            return Err(runtime_error("Usage: AddTrackedAchievement(achievementID)"));
        }
        if !sim.tracked_achievements.contains(&id) {
            if sim.tracked_achievements.len() >= MAX_TRACKED_ACHIEVEMENTS {
                return Ok(0);
            }
            sim.tracked_achievements.push(id);
        }
    }
    // Build 12340 also dispatches for duplicate adds, including at capacity.
    dispatch_event_now(
        state,
        "TRACKED_ACHIEVEMENT_UPDATE",
        &[Val::Num(f64::from(id))],
    )?;
    Ok(0)
}

fn remove_tracked_achievement(state: &mut LuaState) -> LuaResult<u32> {
    let id = tracked_achievement_id(state)?;
    if id == 0 {
        return Err(runtime_error(
            "Usage: RemoveTrackedAchievement(achievementID)",
        ));
    }
    let removed = {
        let mut sim = borrow_state_mut(state)?;
        let previous_count = sim.tracked_achievements.len();
        sim.tracked_achievements
            .retain(|tracked_id| *tracked_id != id);
        sim.tracked_achievements.len() != previous_count
    };
    if removed {
        dispatch_event_now(
            state,
            "TRACKED_ACHIEVEMENT_UPDATE",
            &[Val::Num(f64::from(id))],
        )?;
    }
    Ok(0)
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
