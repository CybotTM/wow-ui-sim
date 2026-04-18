//! `C_GossipInfo` probe surface backed by `SimState.gossip`.
//!
//! Migrates 4 entries off the namespace stub tables:
//!
//! - `C_GossipInfo.GetOptions()` — array of `GossipOptionUIInfo` tables
//! - `C_GossipInfo.GetActiveQuests()` — array of `GossipQuestUIInfo` tables
//! - `C_GossipInfo.GetAvailableQuests()` — array of `GossipQuestUIInfo` tables
//! - `C_GossipInfo.GetPoiForUiMapID(uiMapID)` — nil (permissive stub)

use super::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_api::sim_substates::{GossipOption, GossipQuestRow};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_gossip_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_GossipInfo")?;
    table_set_rust_fn_static(state, table_ref, "GetOptions", c_gossip_info_get_options)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetActiveQuests",
        c_gossip_info_get_active_quests,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAvailableQuests",
        c_gossip_info_get_available_quests,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetPoiForUiMapID",
        c_gossip_info_get_poi_for_ui_map_id,
    )?;
    Ok(())
}

fn c_gossip_info_get_options(state: &mut LuaState) -> LuaResult<u32> {
    let options = borrow_state(state)?.gossip.options.clone();
    let array = create_table(state);
    for (i, opt) in options.iter().enumerate() {
        let entry = push_option_table(state, opt);
        set_table_array(state, array, i as i64 + 1, entry);
    }
    state.push(array);
    Ok(1)
}

fn c_gossip_info_get_active_quests(state: &mut LuaState) -> LuaResult<u32> {
    let rows = borrow_state(state)?.gossip.active_quests.clone();
    push_quest_array(state, &rows)
}

fn c_gossip_info_get_available_quests(state: &mut LuaState) -> LuaResult<u32> {
    let rows = borrow_state(state)?.gossip.available_quests.clone();
    push_quest_array(state, &rows)
}

fn c_gossip_info_get_poi_for_ui_map_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn push_option_table(state: &mut LuaState, opt: &GossipOption) -> Val {
    let t = create_table(state);
    table_set(
        state,
        t,
        "gossipOptionID",
        Val::Num(opt.gossip_option_id as f64),
    );
    table_set(state, t, "orderIndex", Val::Num(opt.order_index as f64));
    let name = create_string(state, &opt.name);
    table_set(state, t, "name", name);
    table_set(state, t, "flags", Val::Num(opt.flags as f64));
    table_set(state, t, "icon", Val::Num(opt.icon as f64));
    match opt.spell_id {
        Some(id) => table_set(state, t, "spellID", Val::Num(id as f64)),
        None => table_set(state, t, "spellID", Val::Nil),
    }
    table_set(
        state,
        t,
        "selectOptionWhenOnlyOption",
        Val::Bool(opt.select_option_when_only_option),
    );
    // rewards is always empty in the sim
    let rewards = create_table(state);
    table_set(state, t, "rewards", rewards);
    t
}

fn push_quest_table(state: &mut LuaState, row: &GossipQuestRow) -> Val {
    let t = create_table(state);
    table_set(state, t, "questID", Val::Num(row.quest_id as f64));
    table_set(state, t, "questInfoID", Val::Num(row.quest_info_id as f64));
    table_set(state, t, "questLevel", Val::Num(row.quest_level as f64));
    let title = create_string(state, &row.title);
    table_set(state, t, "title", title);
    table_set(state, t, "isImportant", Val::Bool(row.is_important));
    table_set(state, t, "isLegendary", Val::Bool(row.is_legendary));
    table_set(state, t, "isMeta", Val::Bool(row.is_meta));
    table_set(state, t, "isTrivial", Val::Bool(row.is_trivial));
    table_set(state, t, "isIgnored", Val::Bool(row.is_ignored));
    match row.frequency {
        Some(f) => table_set(state, t, "frequency", Val::Num(f as f64)),
        None => table_set(state, t, "frequency", Val::Nil),
    }
    match row.is_complete {
        Some(b) => table_set(state, t, "isComplete", Val::Bool(b)),
        None => table_set(state, t, "isComplete", Val::Nil),
    }
    match row.repeatable {
        Some(b) => table_set(state, t, "repeatable", Val::Bool(b)),
        None => table_set(state, t, "repeatable", Val::Nil),
    }
    t
}

fn push_quest_array(state: &mut LuaState, rows: &[GossipQuestRow]) -> LuaResult<u32> {
    let array = create_table(state);
    for (i, row) in rows.iter().enumerate() {
        let entry = push_quest_table(state, row);
        set_table_array(state, array, i as i64 + 1, entry);
    }
    state.push(array);
    Ok(1)
}
