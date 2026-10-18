//! Original Wrath available-gossip projection and shared index selection.

use crate::lua_api::methods::{borrow_state, create_string};
use rilua::{LuaApiMut, LuaResult, Val, vm::state::LuaState};

pub(super) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.register_function("GetGossipAvailableQuests", get_available_quests)?;
    lua.register_function(
        "SelectGossipAvailableQuest",
        super::legacy_globals::select_available_quest,
    )?;
    Ok(())
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
