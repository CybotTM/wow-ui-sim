//! `C_LFGList.GetNumApplications` / `GetNumApplicants` — two-value probes
//! backed by `SimState::lfg_list_counts`.
//!
//! Each returns `(total, viewed)`:
//!
//! - `GetNumApplications()` — how many of the player's listings the server
//!                            knows about, and how many are still within the
//!                            scroll viewport.
//! - `GetNumApplicants()`   — applicant count + viewed-applicant count.
//!
//! Shape matters because `LFGListFrame` callsites do
//! `local total, viewed = C_LFGList.GetNumApplications()`. The earlier Lua
//! stub returned `(0, 0)`, which is correct for an idle sim but doesn't
//! allow tests to exercise non-empty applicant lists; the new admin API
//! `A_Admin.SetLfgApplicationCounts(total, viewed)` /
//! `A_Admin.SetLfgApplicantCounts(total, viewed)` drives the values.

use crate::event::Event;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, create_table, frame_ref,
    table_set,
};
use crate::lua_api::script_helpers::{get_event_listeners, get_script};
use crate::lua_api::state_types::PremadeListing;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub fn get_num_applications(state: &mut LuaState) -> LuaResult<u32> {
    let (total, viewed) = {
        let sim = borrow_state(state)?;
        (
            sim.lfg_list_counts.applications_total,
            sim.lfg_list_counts.applications_viewed,
        )
    };
    state.push(Val::Num(total as f64));
    state.push(Val::Num(viewed as f64));
    Ok(2)
}

pub fn get_num_applicants(state: &mut LuaState) -> LuaResult<u32> {
    let (total, viewed) = {
        let sim = borrow_state(state)?;
        (
            sim.lfg_list_counts.applicants_total,
            sim.lfg_list_counts.applicants_viewed,
        )
    };
    state.push(Val::Num(total as f64));
    state.push(Val::Num(viewed as f64));
    Ok(2)
}

fn fire_named_event(state: &mut LuaState, event_name: &str) -> LuaResult<()> {
    borrow_state_mut(state)?.events.push(Event {
        name: event_name.to_string(),
        args: Vec::new(),
    });
    for widget_id in get_event_listeners(state, event_name) {
        let Some(handler) = get_script(state, widget_id, "OnEvent") else {
            continue;
        };
        let Ok(frame) = frame_ref(state, widget_id) else {
            continue;
        };
        let event_name_val = create_string(state, event_name);
        let _ = call_function_state(state, handler, &[frame, event_name_val]);
    }
    Ok(())
}

fn premade_listing(state: &LuaState, search_result_id: u32) -> Option<PremadeListing> {
    borrow_state(state)
        .ok()?
        .world
        .premade_listings
        .iter()
        .find(|listing| listing.search_result_id == search_result_id)
        .cloned()
}

fn activity_max_players(state: &LuaState, activity_id: u32) -> Option<i32> {
    borrow_state(state)
        .ok()?
        .world
        .premade_listings
        .iter()
        .find(|listing| listing.activity_id == activity_id)
        .map(|listing| listing.max_members)
}

fn build_search_result_info(state: &mut LuaState, listing: &PremadeListing) -> Val {
    let info = create_table(state);
    let name = create_string(state, &listing.name);
    let comment = create_string(state, &listing.comment);
    let leader_name = create_string(state, &listing.leader_name);
    table_set(
        state,
        info,
        "searchResultID",
        Val::Num(listing.search_result_id as f64),
    );
    table_set(state, info, "name", name);
    table_set(state, info, "comment", comment);
    table_set(state, info, "leaderName", leader_name);
    table_set(
        state,
        info,
        "activityID",
        Val::Num(listing.activity_id as f64),
    );
    table_set(
        state,
        info,
        "numMembers",
        Val::Num(listing.num_members as f64),
    );
    table_set(
        state,
        info,
        "maxMembers",
        Val::Num(listing.max_members as f64),
    );
    table_set(state, info, "voiceChat", Val::Bool(listing.voice_chat));
    table_set(state, info, "autoAccept", Val::Bool(listing.auto_accept));
    table_set(state, info, "isDelisted", Val::Bool(listing.is_delisted));
    table_set(state, info, "requiredItemLevel", Val::Num(0.0));
    table_set(state, info, "requiredHonorLevel", Val::Num(0.0));
    table_set(state, info, "requiredDungeonScore", Val::Num(0.0));
    table_set(state, info, "questID", Val::Num(0.0));
    table_set(state, info, "age", Val::Num(0.0));
    table_set(state, info, "isWarMode", Val::Bool(false));
    info
}

fn get_search_result_info(state: &mut LuaState) -> LuaResult<u32> {
    let search_result_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let Some(listing) = premade_listing(state, search_result_id) else {
        return Ok(0);
    };
    let info = build_search_result_info(state, &listing);
    state.push(info);
    Ok(1)
}

fn get_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let search_result_ids = {
        let sim = borrow_state(state)?;
        sim.world
            .premade_listings
            .iter()
            .map(|listing| listing.search_result_id)
            .collect::<Vec<_>>()
    };
    let results = create_table(state);
    if let Val::Table(table_ref) = results {
        for (index, search_result_id) in search_result_ids.iter().enumerate() {
            if let Some(table) = state.gc.tables.get_mut(table_ref) {
                let _ = table.raw_set(
                    Val::Num(index as f64 + 1.0),
                    Val::Num(*search_result_id as f64),
                    &state.gc.string_arena,
                );
            }
        }
        state.gc.barrier_back(table_ref);
    }
    state.push(Val::Num(search_result_ids.len() as f64));
    state.push(results);
    Ok(2)
}

fn search(state: &mut LuaState) -> LuaResult<u32> {
    fire_named_event(state, "LFG_LIST_SEARCH_RESULTS_RECEIVED")?;
    Ok(0)
}

fn get_activity_info_table(state: &mut LuaState) -> LuaResult<u32> {
    let activity_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let Some(max_players) = activity_max_players(state, activity_id) else {
        return Ok(0);
    };
    let info = create_table(state);
    table_set(state, info, "activityID", Val::Num(activity_id as f64));
    let full_name = create_string(state, "Premade Activity");
    let short_name = create_string(state, "Premade");
    table_set(state, info, "fullName", full_name);
    table_set(state, info, "shortName", short_name);
    table_set(state, info, "maxPlayers", Val::Num(max_players as f64));
    table_set(state, info, "groupFinderActivityGroupID", Val::Num(0.0));
    table_set(
        state,
        info,
        "isMythicPlusActivity",
        Val::Bool(max_players == 5),
    );
    state.push(info);
    Ok(1)
}

fn ensure_c_lfg_list_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(b"C_LFGList");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(r)) = existing {
        return r;
    }
    let new_val = create_table(state);
    let Val::Table(new_ref) = new_val else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    new_ref
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let table_ref = ensure_c_lfg_list_table(state);
    table_set_rust_fn_static(state, table_ref, "GetNumApplications", get_num_applications)?;
    table_set_rust_fn_static(state, table_ref, "GetNumApplicants", get_num_applicants)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSearchResultInfo",
        get_search_result_info,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetSearchResults", get_search_results)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetActivityInfoTable",
        get_activity_info_table,
    )?;
    table_set_rust_fn_static(state, table_ref, "Search", search)?;
    Ok(())
}

/// `A_Admin.SetLfgApplicationCounts(total?, viewed?)` — missing args default
/// to 0; negatives clamp to 0.
pub fn admin_set_application_counts(state: &mut LuaState) -> LuaResult<u32> {
    let total = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let viewed = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as i32;
    let mut st = borrow_state_mut(state)?;
    st.lfg_list_counts.applications_total = total.max(0);
    st.lfg_list_counts.applications_viewed = viewed.max(0);
    Ok(0)
}

/// `A_Admin.SetLfgApplicantCounts(total?, viewed?)` — missing args default
/// to 0; negatives clamp to 0.
pub fn admin_set_applicant_counts(state: &mut LuaState) -> LuaResult<u32> {
    let total = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let viewed = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as i32;
    let mut st = borrow_state_mut(state)?;
    st.lfg_list_counts.applicants_total = total.max(0);
    st.lfg_list_counts.applicants_viewed = viewed.max(0);
    Ok(0)
}
