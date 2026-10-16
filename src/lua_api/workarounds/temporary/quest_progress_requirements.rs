//! Missing NPC progress-dialogue / turn-in requirements and portrait-input model.
//! Retire these typed empties when supplied quest-progress-requirements state lands.
//! Reward items/currencies already have state and are not defaulted here.

use crate::lua_api::methods::create_string_static;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    if !crate::c_api::quest_objectives::ENABLED {
        return Ok(());
    }
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

fn empty_progress_text(state: &mut LuaState) -> LuaResult<u32> {
    let text = create_string_static(state, "");
    state.push(text);
    Ok(1)
}

fn empty_requirement_count(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn absent_turn_in_portrait(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
