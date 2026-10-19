//! Missing NPC progress-dialogue / turn-in requirements and portrait-input model.
//! Retire these typed empties when supplied quest-progress-requirements state lands.
//! Reward items/currencies already have state and are not defaulted here.

use crate::lua_api::methods::create_string_static;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    let supports_dialog = crate::c_api::quest_objectives::ENABLED || cfg!(feature = "client-wrath");
    if !supports_dialog {
        return Ok(());
    }
    #[cfg(feature = "client-wrath")]
    for name in ["QuestFlagsPVP", "QuestGetAutoAccept"] {
        // Missing supplied quest PvP/auto-accept flags. Retire when that model lands.
        LuaApiMut::register_function(lua, name, ordinary_quest_flag)?;
    }
    #[cfg(feature = "client-wrath")]
    // Missing legacy gossip option icon-type projection. Retire when that model lands.
    // Only the no-option quest NPC is supported by this temporary surface.
    LuaApiMut::register_function(lua, "GetGossipOptions", no_gossip_options)?;
    LuaApiMut::register_function(lua, "GetProgressText", empty_progress_text)?;
    for name in [
        "GetQuestMoneyToGet",
        "GetNumQuestItems",
        "GetNumQuestCurrencies",
    ] {
        LuaApiMut::register_function(lua, name, empty_requirement_count)?;
    }
    LuaApiMut::register_function(lua, "GetQuestPortraitTurnIn", absent_turn_in_portrait)?;
    Ok(())
}

#[cfg(feature = "client-wrath")]
fn ordinary_quest_flag(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

#[cfg(feature = "client-wrath")]
fn no_gossip_options(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn empty_progress_text(state: &mut LuaState) -> LuaResult<u32> {
    let text = create_string_static(state, "");
    state.push(text);
    Ok(1)
}

fn empty_requirement_count(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn absent_turn_in_portrait(state: &mut LuaState) -> LuaResult<u32> {
    // Cached Retail Mainline/QuestFrame.lua:129-136 tests displayID ~= 0;
    // :523 compares it numerically. Text/name are strings (:459, :499-504).
    // Zero takes the original hide/detach branch (:535-540), not a fake portrait.
    state.push(Val::Num(0.0));
    let empty = create_string_static(state, "");
    state.push(empty);
    state.push(empty);
    Ok(3)
}
