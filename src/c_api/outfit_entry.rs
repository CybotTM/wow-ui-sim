//! Complete supplied outfit records and fresh snapshots; no native values inferred.

use crate::lua_api::methods::{create_string, create_table, table_set, table_set_num};
use rilua::Val;
use rilua::vm::state::LuaState;

#[derive(Debug, Clone)]
pub struct OutfitEntry {
    pub outfit_id: i64,
    pub name: String,
    pub situation_categories: Vec<String>,
    pub icon: i64,
    pub is_event_outfit: bool,
    pub is_disabled: bool,
    pub player_facing_outfit_index: i64,
}

pub(crate) fn build_entry_array(state: &mut LuaState, entries: &[OutfitEntry]) -> Val {
    let array = create_table(state);
    let Val::Table(table) = array else {
        unreachable!()
    };
    for (index, entry) in entries.iter().enumerate() {
        let row = build_entry_table(state, entry);
        table_set_num(state, table, (index + 1) as f64, row);
    }
    array
}

pub(crate) fn build_entry_table(state: &mut LuaState, entry: &OutfitEntry) -> Val {
    let table = create_table(state);
    let name = create_string(state, &entry.name);
    let categories = build_categories_table(state, &entry.situation_categories);
    table_set(state, table, "outfitID", Val::Num(entry.outfit_id as f64));
    table_set(state, table, "name", name);
    table_set(state, table, "situationCategories", categories);
    table_set(state, table, "icon", Val::Num(entry.icon as f64));
    table_set(
        state,
        table,
        "isEventOutfit",
        Val::Bool(entry.is_event_outfit),
    );
    table_set(state, table, "isDisabled", Val::Bool(entry.is_disabled));
    table_set(
        state,
        table,
        "playerFacingOutfitIndex",
        Val::Num(entry.player_facing_outfit_index as f64),
    );
    table
}

fn build_categories_table(state: &mut LuaState, categories: &[String]) -> Val {
    let array = create_table(state);
    let Val::Table(table) = array else {
        unreachable!()
    };
    for (index, category) in categories.iter().enumerate() {
        let value = create_string(state, category);
        table_set_num(state, table, (index + 1) as f64, value);
    }
    array
}
