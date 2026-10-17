//! Wrath-only defaults for missing timed-quest, spell-rank, aura-consolidation
//! and uncatalogued equipped-item classification systems. Retire each default
//! when its backing system can supply the corresponding value.

use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetQuestTimers", empty_quest_timers)?;
    LuaApiMut::register_function(lua, "GetNumMapOverlays", empty_map_overlay_count)?;
    Ok(())
}

// Missing system: legacy discovered-map overlay catalog. No supplied overlays
// means zero; retire when a map-overlay model owns count and records together.
fn empty_map_overlay_count(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
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
