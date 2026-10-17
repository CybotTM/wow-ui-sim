//! Wrath-only defaults for unmodeled legacy startup domains.
//! Retire each default when its named backing system can supply the value.

use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetQuestTimers", empty_quest_timers)?;
    LuaApiMut::register_function(lua, "GetNumMapOverlays", empty_map_overlay_count)?;
    LuaApiMut::register_function(lua, "GetNumMapDebugObjects", empty_map_debug_object_count)?;
    LuaApiMut::register_function(lua, "GetNumDungeonMapLevels", empty_dungeon_map_level_count)?;
    LuaApiMut::register_function(lua, "HasDebugZoneMap", has_debug_zone_map)?;
    LuaApiMut::register_function(lua, "UnitIsTalking", unit_is_talking)?;
    LuaApiMut::register_function(
        lua,
        "IsPlayerResolutionAvailable",
        is_player_resolution_available,
    )?;
    // No RAF summon cooldown state: reuse the existing zero/zero producer.
    // Retire this alias when a modeled RAF cooldown supplies the legacy API.
    lua.exec("GetSummonFriendCooldown = C_RecruitAFriend.GetSummonFriendCooldown")?;
    Ok(())
}

// Missing system: legacy discovered-map overlay catalog. No supplied overlays
// means zero; retire when a map-overlay model owns count and records together.
fn empty_map_overlay_count(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

// Missing system: map debug objects. No supplied catalog means zero objects;
// retire when that model owns the count and corresponding debug-object records.
fn empty_map_debug_object_count(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

// Missing system: selected-map dungeon-floor catalog. Unsupplied floors mean
// zero, never a total inferred from the selected-floor scalar. Retire when a
// supplied floor catalog owns both records and count.
fn empty_dungeon_map_level_count(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

// Missing system: debug-zone map tiles/capability. Retire when that model owns
// availability. User-approved temporary policy: false, not native capability.
fn has_debug_zone_map(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

// Missing system: voice-chat speaking model. Original PlayerFrame passes UnitName,
// not a unit token. Accept every Lua string, including non-UTF-8 bytes; no lookup.
// Retire when supplied voice activity owns this query. Temporary policy: false.
fn unit_is_talking(state: &mut LuaState) -> LuaResult<u32> {
    let name = Val::from_stack(state, 1)?;
    if !matches!(name, Val::Str(_)) {
        return Err(runtime_error(
            "UnitIsTalking expects a character name string",
        ));
    }
    state.push(Val::Bool(false));
    Ok(1)
}

// Missing system: player-texture-resolution capability. Retire when its supplied
// model owns availability. User-approved true preserves the control's normal
// enabled state; it does not assert native hardware/rendering support.
fn is_player_resolution_available(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

// No timed-quest model exists: empty domain means zero values, never one nil.
fn empty_quest_timers(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

// AuraInfo has no legacy spell-rank field. Keep the positional slot, without
// claiming a rank or moving the modeled icon into that slot.
pub(crate) fn push_unmodeled_aura_rank(state: &mut LuaState) {
    state.push(Val::Nil);
}

// AuraInfo has no consolidation policy/input. Explicitly opt out for now.
pub(crate) fn push_unmodeled_aura_consolidation(state: &mut LuaState) {
    state.push(Val::Bool(false));
}

// Missing equipped-item catalog metadata cannot establish a weapon; retire
// this policy when item classification covers the supplied equipped identity.
pub(crate) fn unknown_offhand_is_weapon() -> bool {
    false
}
