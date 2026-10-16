//! Wrath projection of existing dungeon state; no changes to other profiles.
use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn get_lfg_dungeon_info(state: &mut LuaState) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    let row = borrow_state(state)?
        .lfd_dungeons
        .iter()
        .find(|d| d.dungeon_id == id)
        .cloned();
    let Some(row) = row else {
        state.push(Val::Nil);
        return Ok(1);
    };
    push_dungeon_info(state, &row);
    Ok(14)
}

fn push_dungeon_info(state: &mut LuaState, row: &crate::lua_api::state_types::LfdDungeonInfo) {
    let name = create_string(state, &row.name);
    let texture = create_string(state, &row.texture_filename);
    let description = create_string(state, &row.description);
    state.push(name);
    push_dungeon_levels(state, row);
    // LFDFrame.lua:814 reads these at 10,11,13,14. Slot 12 max_players and
    // unused prefix fields retain existing model meaning as simulator policy.
    state.push(texture);
    state.push(Val::Num(row.difficulty as f64));
    state.push(Val::Num(row.max_players as f64));
    state.push(description);
    state.push(Val::Bool(row.is_holiday));
}

fn push_dungeon_levels(state: &mut LuaState, row: &crate::lua_api::state_types::LfdDungeonInfo) {
    for value in [
        row.type_id,
        row.subtype_id,
        row.min_level,
        row.max_level,
        row.rec_level,
        row.min_rec_level,
        row.max_rec_level,
        row.expansion_level,
    ] {
        state.push(Val::Num(value as f64));
    }
}
pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetLFGDungeonInfo", get_lfg_dungeon_info)?;
    Ok(())
}
