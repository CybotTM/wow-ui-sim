//! Original Wrath available-gossip projection and shared index selection.

use crate::lua_api::methods::{borrow_state, create_string};
use rilua::{LuaApiMut, LuaResult, Val, vm::state::LuaState};

pub(super) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.register_function("GetGossipAvailableQuests", get_available_quests)?;
    lua.register_function(
        "GetNumGossipAvailableQuests",
        super::legacy_globals::get_num_available_quests,
    )?;
    lua.register_function(
        "GetNumGossipActiveQuests",
        super::legacy_globals::get_num_active_quests,
    )?;
    lua.register_function("GetNumGossipOptions", get_num_options)?;
    lua.register_function("GetTitleText", get_title_text)?;
    lua.register_function(
        "SelectGossipAvailableQuest",
        super::legacy_globals::select_available_quest,
    )?;
    Ok(())
}

fn get_num_options(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.gossip.options.len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn get_title_text(state: &mut LuaState) -> LuaResult<u32> {
    let title = {
        let sim = borrow_state(state)?;
        sim.pending_quest_offer
            .as_ref()
            .map(|row| row.title.clone())
    };
    let Some(title) = title else {
        return super::legacy_globals::get_title_text(state);
    };
    let title = create_string(state, &title);
    state.push(title);
    Ok(1)
}

fn get_available_quests(state: &mut LuaState) -> LuaResult<u32> {
    let rows = borrow_state(state)?.gossip.available_quests.clone();
    let result_count = rows.len() * 5;
    for row in rows {
        let title = create_string(state, &row.title);
        state.push(title);
        state.push(Val::Num(row.quest_level as f64));
        state.push(if row.is_trivial {
            Val::Num(1.0)
        } else {
            Val::Nil
        });
        // No supplied isDaily field: frequency classification is outside this slice.
        state.push(Val::Nil);
        state.push(if row.repeatable == Some(true) {
            Val::Num(1.0)
        } else {
            Val::Nil
        });
    }
    Ok(result_count as u32)
}
