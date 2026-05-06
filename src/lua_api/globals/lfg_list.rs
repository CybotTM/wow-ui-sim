//! `C_LFGList` namespace — Group Finder search, category, and activity APIs.
//!
//! Backed by `SimState::lfg_category_info`, `lfg_activity_groups`,
//! `lfg_activities`, and `world.premade_listings`.

use crate::event::Event;
use crate::lua_api::env::WowLuaAppData;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, create_table, frame_ref,
    table_set,
};
use crate::lua_api::script_helpers::{get_event_listeners, get_script};
use crate::lua_api::state_types::{
    LfgActivityInfo, LfgApplication, LfgCategoryInfo, PendingTimer, PremadeListing,
};
use crate::lua_api::{next_timer_id, timer_layout};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::closure::{Closure, RustClosure};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val, runtime_error};
use std::collections::HashMap;
use std::time::Instant;

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

fn fire_event_with_args(state: &mut LuaState, event_name: &str, args: Vec<Val>) -> LuaResult<()> {
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
        let mut call_args = Vec::with_capacity(2 + args.len());
        call_args.push(frame);
        call_args.push(event_name_val);
        call_args.extend(args.iter().cloned());
        let _ = call_function_state(state, handler, &call_args);
    }
    Ok(())
}

fn defer_lfg_search_results_event(state: &mut LuaState) -> LuaResult<()> {
    let callback = Val::Function(state.gc.alloc_closure(Closure::Rust(RustClosure::new(
        dispatch_lfg_search_results_received,
        "C_LFGList.SearchResultsReady",
    ))));
    let id = next_timer_id();
    timer_layout::store_timer_callback(state, id, callback);

    let app = state
        .app_data::<WowLuaAppData>()
        .ok_or_else(|| runtime_error("missing WowLuaAppData"))?;
    let owner_addon = {
        let sim = app.sim_state.borrow();
        sim.loading_addon_index.or(sim.executing_addon_index)
    };
    app.sim_state
        .borrow_mut()
        .rilua_timers
        .push_back(PendingTimer {
            id,
            fire_at: Instant::now(),
            interval: None,
            remaining: None,
            cancelled: false,
            owner_addon,
        });
    Ok(())
}

fn dispatch_lfg_search_results_received(state: &mut LuaState) -> LuaResult<u32> {
    fire_event_with_args(state, "LFG_LIST_SEARCH_RESULTS_RECEIVED", Vec::new())?;
    Ok(0)
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

fn find_activity(state: &LuaState, activity_id: u32) -> Option<LfgActivityInfo> {
    borrow_state(state)
        .ok()?
        .lfg_activities
        .iter()
        .find(|a| a.activity_id == activity_id)
        .cloned()
}

fn build_search_result_info(state: &mut LuaState, listing: &PremadeListing) -> Val {
    let info = create_table(state);
    set_search_result_identity_fields(state, info, listing);
    set_search_result_activity_fields(state, info, listing);
    set_search_result_size_fields(state, info, listing);
    set_search_result_social_fields(state, info, listing);
    set_search_result_requirement_fields(state, info);
    info
}

fn set_search_result_identity_fields(state: &mut LuaState, info: Val, listing: &PremadeListing) {
    let name = create_string(state, &listing.name);
    let comment = create_string(state, &listing.comment);
    let leader_name = create_string(state, &listing.leader_name);
    let voice_chat = create_string(state, &listing.voice_chat);
    let party_guid = create_string(state, &listing.party_guid);
    table_set(
        state,
        info,
        "searchResultID",
        Val::Num(listing.search_result_id as f64),
    );
    table_set(state, info, "name", name);
    table_set(state, info, "comment", comment);
    table_set(state, info, "leaderName", leader_name);
    table_set(state, info, "voiceChat", voice_chat);
    table_set(state, info, "autoAccept", Val::Bool(listing.auto_accept));
    table_set(state, info, "isDelisted", Val::Bool(listing.is_delisted));
    table_set(state, info, "partyGUID", party_guid);
}

fn set_search_result_activity_fields(state: &mut LuaState, info: Val, listing: &PremadeListing) {
    // `activityID` is the legacy single-value form; `activityIDs` is the
    // current shape addons read (`searchResultInfo.activityIDs[1]`).
    table_set(
        state,
        info,
        "activityID",
        Val::Num(listing.activity_id as f64),
    );
    let activity_ids = activity_ids_table(state, listing);
    table_set(state, info, "activityIDs", activity_ids);
    table_set(
        state,
        info,
        "generalPlaystyle",
        Val::Num(listing.general_playstyle as f64),
    );
    table_set(
        state,
        info,
        "crossFactionListing",
        Val::Bool(listing.cross_faction_listing),
    );
    table_set(
        state,
        info,
        "leaderFactionGroup",
        Val::Num(listing.leader_faction_group as f64),
    );
}

fn activity_ids_table(state: &mut LuaState, listing: &PremadeListing) -> Val {
    let activity_ids = create_table(state);
    let Val::Table(activity_ids_ref) = activity_ids else {
        return activity_ids;
    };
    if let Some(t) = state.gc.tables.get_mut(activity_ids_ref) {
        let _ = t.raw_set(
            Val::Num(1.0),
            Val::Num(listing.activity_id as f64),
            &state.gc.string_arena,
        );
    }
    state.gc.barrier_back(activity_ids_ref);
    activity_ids
}

fn set_search_result_size_fields(state: &mut LuaState, info: Val, listing: &PremadeListing) {
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
}

fn set_search_result_social_fields(state: &mut LuaState, info: Val, listing: &PremadeListing) {
    table_set(
        state,
        info,
        "numBNetFriends",
        Val::Num(listing.num_bnet_friends as f64),
    );
    table_set(
        state,
        info,
        "numCharFriends",
        Val::Num(listing.num_char_friends as f64),
    );
    table_set(
        state,
        info,
        "numGuildMates",
        Val::Num(listing.num_guild_mates as f64),
    );
}

fn set_search_result_requirement_fields(state: &mut LuaState, info: Val) {
    table_set(state, info, "requiredItemLevel", Val::Num(0.0));
    table_set(state, info, "requiredHonorLevel", Val::Num(0.0));
    table_set(state, info, "requiredDungeonScore", Val::Num(0.0));
    table_set(state, info, "requiredPvpRating", Val::Num(0.0));
    table_set(state, info, "questID", Val::Num(0.0));
    table_set(state, info, "age", Val::Num(0.0));
    table_set(state, info, "isWarMode", Val::Bool(false));
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

fn has_search_result_info(state: &mut LuaState) -> LuaResult<u32> {
    let search_result_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let found = premade_listing(state, search_result_id).is_some();
    state.push(Val::Bool(found));
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
    defer_lfg_search_results_event(state)?;
    Ok(0)
}

/// `GetFilteredSearchResults()` → `(totalResults, results)`. Mirrors
/// `GetSearchResults` once the user has applied filters in the panel —
/// the sim does not model server-side filter state, so the panel sees
/// every seeded listing.
fn get_filtered_search_results(state: &mut LuaState) -> LuaResult<u32> {
    get_search_results(state)
}

/// `GetSearchResultMemberCounts(resultID)` → display data table.
///
/// Shape consumed by `LFGListGroupDataDisplay_Update`:
///   `{ TANK = n, HEALER = n, DAMAGER = n, NOROLE = n,
///      classesByRole = { TANK = { WARRIOR = n, ... }, ... },
///      leaversByClass = { WARRIOR = n, ... } }`
///
/// `LFGListGroupDataDisplayEnumerate_Update` reads `displayData.NOROLE`
/// without a nil-guard (`numPlayers = TANK + HEALER + DAMAGER + NOROLE`),
/// so every key must be a number, not nil.
fn get_search_result_member_counts(state: &mut LuaState) -> LuaResult<u32> {
    let search_result_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let Some(listing) = premade_listing(state, search_result_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let display = create_table(state);
    set_search_result_role_counts(state, display, &listing);
    let classes_by_role = classes_by_role_table(state, &listing.classes_by_role);
    table_set(state, display, "classesByRole", classes_by_role);
    let leavers = create_table(state);
    table_set(state, display, "leaversByClass", leavers);
    state.push(display);
    Ok(1)
}

fn set_search_result_role_counts(state: &mut LuaState, display: Val, listing: &PremadeListing) {
    table_set(state, display, "TANK", Val::Num(listing.tanks as f64));
    table_set(state, display, "HEALER", Val::Num(listing.healers as f64));
    table_set(state, display, "DAMAGER", Val::Num(listing.damagers as f64));
    table_set(state, display, "NOROLE", Val::Num(listing.no_role as f64));
}

fn classes_by_role_table(
    state: &mut LuaState,
    classes_by_role: &HashMap<String, HashMap<String, i32>>,
) -> Val {
    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        return table;
    };
    for (role, class_counts) in classes_by_role {
        let inner = class_counts_table(state, class_counts);
        raw_set_string_key(state, table_ref, role, inner);
    }
    state.gc.barrier_back(table_ref);
    table
}

fn class_counts_table(state: &mut LuaState, class_counts: &HashMap<String, i32>) -> Val {
    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        return table;
    };
    for (class_name, count) in class_counts {
        raw_set_string_key(state, table_ref, class_name, Val::Num(*count as f64));
    }
    state.gc.barrier_back(table_ref);
    table
}

fn raw_set_string_key(state: &mut LuaState, table_ref: GcRef<Table>, key: &str, value: Val) {
    let Val::Str(key_ref) = create_string(state, key) else {
        return;
    };
    if let Some(table) = state.gc.tables.get_mut(table_ref) {
        let _ = table.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
}

/// `GetGroupLeaverCountsByRole()` → `(tank, healer, damager)`. Sim has
/// no group-history model, so report zero leavers in every role.
fn get_group_leaver_counts_by_role(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(3)
}

/// `GetApplications()` → array of `searchResultID` for every pending
/// application the player has submitted.
fn get_applications(state: &mut LuaState) -> LuaResult<u32> {
    let result_ids = {
        let sim = borrow_state(state)?;
        sim.lfg_applications
            .iter()
            .map(|app| app.search_result_id)
            .collect::<Vec<_>>()
    };
    let result = create_table(state);
    if let Val::Table(table_ref) = result {
        for (index, rid) in result_ids.iter().enumerate() {
            if let Some(t) = state.gc.tables.get_mut(table_ref) {
                let _ = t.raw_set(
                    Val::Num(index as f64 + 1.0),
                    Val::Num(*rid as f64),
                    &state.gc.string_arena,
                );
            }
        }
        state.gc.barrier_back(table_ref);
    }
    state.push(result);
    Ok(1)
}

/// `GetPremadeGroupFinderStyle()` → style selector.
fn get_premade_group_finder_style(_state: &mut LuaState) -> LuaResult<u32> {
    state_push_num_zero(_state)
}

fn state_push_num_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(rilua::Val::Num(0.0));
    Ok(1)
}

fn can_create_quest_group(_state: &mut LuaState) -> LuaResult<u32> {
    _state.push(rilua::Val::Bool(false));
    Ok(1)
}

fn can_create_scenario_group(_state: &mut LuaState) -> LuaResult<u32> {
    _state.push(rilua::Val::Bool(false));
    Ok(1)
}

fn is_premade_group_finder_enabled(_state: &mut LuaState) -> LuaResult<u32> {
    _state.push(rilua::Val::Bool(false));
    Ok(1)
}

fn remove_listing(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// `GetApplicationInfo(searchResultID)` →
///   `(applicationID, applicationStatus, pendingStatus, applicationDuration, role)`.
///
/// Returns the `"none"` status (not nil) when no application exists for
/// `resultID` — the panel's search-entry render does
/// `isApplication = (appStatus ~= "none" or pendingStatus)`, so a nil
/// status would mark every browsed result as an active application.
fn get_application_info(state: &mut LuaState) -> LuaResult<u32> {
    let search_result_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let app_opt = borrow_state(state)?
        .lfg_applications
        .iter()
        .find(|a| a.search_result_id == search_result_id)
        .cloned();
    let Some(app) = app_opt else {
        state.push(Val::Num(0.0));
        let none_str = create_string(state, "none");
        state.push(none_str);
        state.push(Val::Nil);
        state.push(Val::Num(0.0));
        let empty = create_string(state, "");
        state.push(empty);
        return Ok(5);
    };
    state.push(Val::Num(app.application_id as f64));
    let status_val = create_string(state, &app.status);
    state.push(status_val);
    match &app.pending_status {
        Some(s) => {
            let v = create_string(state, s);
            state.push(v);
        }
        None => state.push(Val::Nil),
    }
    state.push(Val::Num(app.duration as f64));
    let role_val = create_string(state, &app.role);
    state.push(role_val);
    Ok(5)
}

/// `ApplyToGroup(searchResultID, tank, healer, damager)` — submit an
/// application. Idempotent: re-applying to a result that already has a
/// pending application is a no-op (matches retail's "you have a pending
/// application" guard, exposed as the gray Sign Up button).
fn apply_to_group(state: &mut LuaState) -> LuaResult<u32> {
    let search_result_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let tank = Option::<bool>::from_stack(state, 2)?.unwrap_or(false);
    let healer = Option::<bool>::from_stack(state, 3)?.unwrap_or(false);
    let damager = Option::<bool>::from_stack(state, 4)?.unwrap_or(false);
    let role = selected_application_role(tank, healer, damager);
    let listing_name = apply_to_group_listing_name(state, search_result_id)?;
    let Some(listing_name) = listing_name else {
        return Ok(0);
    };
    create_lfg_application(state, search_result_id, role)?;
    fire_lfg_application_events(state, search_result_id, &listing_name)?;
    Ok(0)
}

fn selected_application_role(tank: bool, healer: bool, damager: bool) -> String {
    if tank {
        "TANK"
    } else if healer {
        "HEALER"
    } else if damager {
        "DAMAGER"
    } else {
        ""
    }
    .to_string()
}

fn apply_to_group_listing_name(
    state: &mut LuaState,
    search_result_id: u32,
) -> LuaResult<Option<String>> {
    let sim = borrow_state(state)?;
    if sim
        .lfg_applications
        .iter()
        .any(|a| a.search_result_id == search_result_id)
    {
        return Ok(None);
    }
    Ok(sim
        .world
        .premade_listings
        .iter()
        .find(|l| l.search_result_id == search_result_id)
        .map(|l| l.name.clone()))
}

fn create_lfg_application(
    state: &mut LuaState,
    search_result_id: u32,
    role: String,
) -> LuaResult<()> {
    let now = borrow_state(state)?.start_time.elapsed().as_secs_f64();
    let mut sim = borrow_state_mut(state)?;
    let app_id = sim.lfg_next_application_id;
    sim.lfg_next_application_id += 1;
    sim.lfg_applications.push(LfgApplication {
        application_id: app_id,
        search_result_id,
        status: "applied".to_string(),
        pending_status: None,
        start_time: now,
        duration: 120.0,
        role,
    });
    Ok(())
}

fn fire_lfg_application_events(
    state: &mut LuaState,
    search_result_id: u32,
    listing_name: &str,
) -> LuaResult<()> {
    let new_status = create_string(state, "applied");
    let old_status = create_string(state, "none");
    let group_name = create_string(state, listing_name);
    fire_event_with_args(
        state,
        "LFG_LIST_APPLICATION_STATUS_UPDATED",
        vec![
            Val::Num(search_result_id as f64),
            new_status,
            old_status,
            group_name,
        ],
    )?;
    fire_event_with_args(
        state,
        "LFG_LIST_SEARCH_RESULT_UPDATED",
        vec![Val::Num(search_result_id as f64)],
    )?;
    Ok(())
}

/// `CancelApplication(searchResultID)` — cancel a pending application.
/// Removes the entry from `lfg_applications` and fires the status-update
/// event so the panel transitions the row back to its non-application
/// rendering branch.
fn cancel_application(state: &mut LuaState) -> LuaResult<u32> {
    let search_result_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let listing_name = {
        let mut sim = borrow_state_mut(state)?;
        let pos = sim
            .lfg_applications
            .iter()
            .position(|a| a.search_result_id == search_result_id);
        let Some(idx) = pos else {
            return Ok(0);
        };
        sim.lfg_applications.remove(idx);
        sim.world
            .premade_listings
            .iter()
            .find(|l| l.search_result_id == search_result_id)
            .map(|l| l.name.clone())
            .unwrap_or_default()
    };
    let new_status = create_string(state, "cancelled");
    let old_status = create_string(state, "applied");
    let group_name = create_string(state, &listing_name);
    fire_event_with_args(
        state,
        "LFG_LIST_APPLICATION_STATUS_UPDATED",
        vec![
            Val::Num(search_result_id as f64),
            new_status,
            old_status,
            group_name,
        ],
    )?;
    Ok(0)
}

fn get_activity_info_table(state: &mut LuaState) -> LuaResult<u32> {
    let activity_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let Some(act) = find_activity(state, activity_id) else {
        return Ok(0);
    };
    let info = create_table(state);
    set_activity_identity_fields(state, info, &act);
    set_activity_group_fields(state, info, &act);
    set_activity_filter_fields(state, info, &act);
    state.push(info);
    Ok(1)
}

fn set_activity_identity_fields(state: &mut LuaState, info: Val, act: &LfgActivityInfo) {
    let full_name = create_string(state, &act.full_name);
    let short_name = create_string(state, &act.short_name);
    table_set(state, info, "activityID", Val::Num(act.activity_id as f64));
    table_set(state, info, "fullName", full_name);
    table_set(state, info, "shortName", short_name);
}

fn set_activity_group_fields(state: &mut LuaState, info: Val, act: &LfgActivityInfo) {
    table_set(state, info, "categoryID", Val::Num(act.category_id as f64));
    table_set(
        state,
        info,
        "groupFinderActivityGroupID",
        Val::Num(act.group_id as f64),
    );
    table_set(state, info, "maxPlayers", Val::Num(act.max_players as f64));
}

fn set_activity_filter_fields(state: &mut LuaState, info: Val, act: &LfgActivityInfo) {
    table_set(
        state,
        info,
        "displayType",
        Val::Num(act.display_type as f64),
    );
    table_set(state, info, "filters", Val::Num(act.filters as f64));
    // M+ activities are group 295 (Mythic+)
    let is_mplus = act.group_id == 295;
    table_set(state, info, "isMythicPlusActivity", Val::Bool(is_mplus));
    table_set(
        state,
        info,
        "allowCrossFaction",
        Val::Bool(act.allow_cross_faction),
    );
    table_set(
        state,
        info,
        "ilvlSuggestion",
        Val::Num(act.item_level as f64),
    );
    table_set(state, info, "useHonorLevel", Val::Bool(act.use_honor_level));
}

/// `GetAvailableCategories(filters?)` → array of category IDs ordered by
/// `LfgCategoryInfo.order`. If filters is 0/nil every category is returned.
fn get_available_categories(state: &mut LuaState) -> LuaResult<u32> {
    let _filters = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let mut cats = {
        let sim = borrow_state(state)?;
        sim.lfg_category_info
            .iter()
            .map(|(id, info)| (*id, info.order))
            .collect::<Vec<_>>()
    };
    cats.sort_by_key(|(_, order)| *order);
    let result = create_table(state);
    if let Val::Table(table_ref) = result {
        for (index, (cat_id, _)) in cats.iter().enumerate() {
            if let Some(table) = state.gc.tables.get_mut(table_ref) {
                let _ = table.raw_set(
                    Val::Num(index as f64 + 1.0),
                    Val::Num(*cat_id as f64),
                    &state.gc.string_arena,
                );
            }
        }
        state.gc.barrier_back(table_ref);
    }
    state.push(result);
    Ok(1)
}

/// `GetLfgCategoryInfo(categoryID)` → table or nil.
fn get_lfg_category_info(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let info_opt = borrow_state(state)?
        .lfg_category_info
        .get(&category_id)
        .cloned();
    let Some(info) = info_opt else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let t = create_table(state);
    set_lfg_category_info_fields(state, t, &info);
    state.push(t);
    Ok(1)
}

fn set_lfg_category_info_fields(state: &mut LuaState, table: Val, info: &LfgCategoryInfo) {
    let name = create_string(state, &info.name);
    table_set(state, table, "name", name);
    set_lfg_category_info_flags(state, table, info);
    table_set(state, table, "searchPromptOverride", Val::Nil);
}

fn set_lfg_category_info_flags(state: &mut LuaState, table: Val, info: &LfgCategoryInfo) {
    table_set(
        state,
        table,
        "separateRecommended",
        Val::Bool(info.separate_recommended),
    );
    table_set(
        state,
        table,
        "preferCurrentArea",
        Val::Bool(info.prefer_current_area),
    );
    table_set(
        state,
        table,
        "allowCrossFaction",
        Val::Bool(info.allow_cross_faction),
    );
    table_set(
        state,
        table,
        "autoChooseActivity",
        Val::Bool(info.auto_choose_activity),
    );
    table_set(
        state,
        table,
        "showPlaystyleDropdown",
        Val::Bool(info.show_playstyle_dropdown),
    );
}

/// `GetAvailableActivityGroups(categoryID, filters?)` → array of groupIDs.
fn get_available_activity_groups(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let filters = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as u32;
    let mut groups = {
        let sim = borrow_state(state)?;
        sim.lfg_activity_groups
            .iter()
            .filter(|g| g.category_id == category_id && (filters == 0 || g.filters & filters != 0))
            .map(|g| (g.group_id, g.order_index))
            .collect::<Vec<_>>()
    };
    groups.sort_by_key(|(_, order)| *order);
    let result = create_table(state);
    if let Val::Table(table_ref) = result {
        for (index, (group_id, _)) in groups.iter().enumerate() {
            if let Some(table) = state.gc.tables.get_mut(table_ref) {
                let _ = table.raw_set(
                    Val::Num(index as f64 + 1.0),
                    Val::Num(*group_id as f64),
                    &state.gc.string_arena,
                );
            }
        }
        state.gc.barrier_back(table_ref);
    }
    state.push(result);
    Ok(1)
}

/// `GetAvailableActivities(categoryID?, groupID?, filters?, searchTerm?)` → array of activityIDs.
fn get_available_activities(state: &mut LuaState) -> LuaResult<u32> {
    let criteria = AvailableActivityCriteria::from_stack(state)?;
    let mut activities = {
        let sim = borrow_state(state)?;
        sim.lfg_activities
            .iter()
            .filter(|activity| criteria.matches(activity))
            .map(|a| (a.activity_id, a.order_index))
            .collect::<Vec<_>>()
    };
    activities.sort_by_key(|(_, order)| *order);
    let result = ordered_activity_id_table(state, &activities);
    state.push(result);
    Ok(1)
}

struct AvailableActivityCriteria {
    category_id: Option<i32>,
    group_id: Option<u32>,
    filters: u32,
    search_term: Option<String>,
}

impl AvailableActivityCriteria {
    fn from_stack(state: &mut LuaState) -> LuaResult<Self> {
        let category_id = Option::<f64>::from_stack(state, 1)?.map(|v| v as i32);
        let group_id = Option::<f64>::from_stack(state, 2)?.map(|v| v as u32);
        let filters = Option::<f64>::from_stack(state, 3)?.unwrap_or(0.0) as u32;
        let search_term = Option::<String>::from_stack(state, 4)?.map(|s| s.to_lowercase());
        Ok(Self {
            category_id,
            group_id,
            filters,
            search_term,
        })
    }

    fn matches(&self, activity: &LfgActivityInfo) -> bool {
        self.matches_category(activity)
            && self.matches_group(activity)
            && self.matches_filters(activity)
            && self.matches_search_term(activity)
    }

    fn matches_category(&self, activity: &LfgActivityInfo) -> bool {
        self.category_id
            .is_none_or(|category_id| activity.category_id == category_id)
    }

    fn matches_group(&self, activity: &LfgActivityInfo) -> bool {
        self.group_id
            .is_none_or(|group_id| activity.group_id == group_id)
    }

    fn matches_filters(&self, activity: &LfgActivityInfo) -> bool {
        self.filters == 0 || activity.filters & self.filters != 0
    }

    fn matches_search_term(&self, activity: &LfgActivityInfo) -> bool {
        self.search_term
            .as_ref()
            .is_none_or(|term| activity.full_name.to_lowercase().contains(term.as_str()))
    }
}

fn ordered_activity_id_table(state: &mut LuaState, activities: &[(u32, i32)]) -> Val {
    let result = create_table(state);
    if let Val::Table(table_ref) = result {
        for (index, (activity_id, _)) in activities.iter().enumerate() {
            if let Some(table) = state.gc.tables.get_mut(table_ref) {
                let _ = table.raw_set(
                    Val::Num(index as f64 + 1.0),
                    Val::Num(*activity_id as f64),
                    &state.gc.string_arena,
                );
            }
        }
        state.gc.barrier_back(table_ref);
    }
    result
}

/// `GetActivityGroupInfo(groupID)` → (name, orderIndex) or nothing.
fn get_activity_group_info(state: &mut LuaState) -> LuaResult<u32> {
    let group_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let found = borrow_state(state)?
        .lfg_activity_groups
        .iter()
        .find(|g| g.group_id == group_id)
        .map(|g| (g.name.clone(), g.order_index));
    let Some((name, order_index)) = found else {
        return Ok(0);
    };
    let name_val = create_string(state, &name);
    state.push(name_val);
    state.push(Val::Num(order_index as f64));
    Ok(2)
}

/// `GetActivityFullName(activityID, questID?, showWarmode?)` → string.
fn get_activity_full_name(state: &mut LuaState) -> LuaResult<u32> {
    let activity_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let name = borrow_state(state)?
        .lfg_activities
        .iter()
        .find(|a| a.activity_id == activity_id)
        .map(|a| a.full_name.clone())
        .unwrap_or_default();
    let val = create_string(state, &name);
    state.push(val);
    Ok(1)
}

/// `GetPlaystyleString(playstyle, generalPlaystyle, activityInfo)` → string.
///
/// Retail always returns a string. The current sim seeds modern
/// `generalPlaystyle` values, so map those first and fall back to the legacy
/// `playstyle` enum only when no general playstyle is provided.
fn get_playstyle_string(state: &mut LuaState) -> LuaResult<u32> {
    let playstyle = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let general_playstyle = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as i32;
    let label = match general_playstyle {
        1 => "Learning",
        2 => "Relaxed",
        3 => "Focused",
        4 => "Expert",
        _ => match playstyle {
            1 => "Standard",
            2 => "Casual",
            3 => "Hardcore",
            _ => "",
        },
    };
    let val = create_string(state, label);
    state.push(val);
    Ok(1)
}

/// `HasActivityList()` → bool. True when the activity catalog is seeded.
fn has_activity_list(state: &mut LuaState) -> LuaResult<u32> {
    let has = !borrow_state(state)?.lfg_activities.is_empty();
    state.push(Val::Bool(has));
    Ok(1)
}

/// `HasActiveEntryInfo()` → bool. False — sim doesn't model the player's own listing.
fn has_active_entry_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// `GetActiveEntryInfo()` → nil. No active listing.
fn get_active_entry_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

/// `GetAvailableRoles()` → (tank, healer, dps). Always true.
fn get_available_roles(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    state.push(Val::Bool(true));
    state.push(Val::Bool(true));
    Ok(3)
}

/// `GetAvailableLanguageSearchFilter()` → array of language strings.
fn get_available_language_search_filter(state: &mut LuaState) -> LuaResult<u32> {
    let result = create_table(state);
    if let Val::Table(table_ref) = result {
        let lang = state.gc.intern_string_static(b"enUS");
        if let Some(table) = state.gc.tables.get_mut(table_ref) {
            let _ = table.raw_set(Val::Num(1.0), Val::Str(lang), &state.gc.string_arena);
        }
        state.gc.barrier_back(table_ref);
    }
    state.push(result);
    Ok(1)
}

/// `GetLanguageSearchFilter()` → `{ enUS = true, ... }`. Map keyed by
/// language code; the panel's filter dropdown reads bool values to
/// decide which checkboxes are ticked.
fn get_language_search_filter(state: &mut LuaState) -> LuaResult<u32> {
    let entries = {
        let sim = borrow_state(state)?;
        sim.lfg_language_filter
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<_>>()
    };
    let result = create_table(state);
    if let Val::Table(table_ref) = result {
        for (lang, enabled) in &entries {
            let key = create_string(state, lang);
            if let Val::Str(s) = key {
                if let Some(t) = state.gc.tables.get_mut(table_ref) {
                    let _ = t.raw_set(Val::Str(s), Val::Bool(*enabled), &state.gc.string_arena);
                }
            }
        }
        state.gc.barrier_back(table_ref);
    }
    state.push(result);
    Ok(1)
}

/// `GetDefaultLanguageSearchFilter()` → same shape as
/// `GetLanguageSearchFilter`, but reflects the player's default
/// (locale-derived) language set rather than current selections.
fn get_default_language_search_filter(state: &mut LuaState) -> LuaResult<u32> {
    get_language_search_filter(state)
}

/// `GetAdvancedFilter()` → table mirroring `LfgAdvancedFilter`.
///
/// Fields consumed by `EntryStillSatisfiesFilters` and the filter
/// dropdown helpers. All-false / zero / empty defaults pass every
/// search result through.
fn get_advanced_filter(state: &mut LuaState) -> LuaResult<u32> {
    let f = borrow_state(state)?.lfg_advanced_filter.clone();
    let info = create_table(state);
    table_set(state, info, "needsTank", Val::Bool(f.needs_tank));
    table_set(state, info, "needsHealer", Val::Bool(f.needs_healer));
    table_set(state, info, "needsDamage", Val::Bool(f.needs_damage));
    table_set(state, info, "needsMyClass", Val::Bool(f.needs_my_class));
    table_set(state, info, "hasTank", Val::Bool(f.has_tank));
    table_set(state, info, "hasHealer", Val::Bool(f.has_healer));
    table_set(
        state,
        info,
        "minimumRating",
        Val::Num(f.minimum_rating as f64),
    );
    let activities = create_table(state);
    if let Val::Table(act_ref) = activities {
        for (i, aid) in f.activities.iter().enumerate() {
            if let Some(t) = state.gc.tables.get_mut(act_ref) {
                let _ = t.raw_set(
                    Val::Num(i as f64 + 1.0),
                    Val::Num(*aid as f64),
                    &state.gc.string_arena,
                );
            }
        }
        state.gc.barrier_back(act_ref);
    }
    table_set(state, info, "activities", activities);
    table_set(
        state,
        info,
        "difficultyNormal",
        Val::Bool(f.difficulty_normal),
    );
    table_set(
        state,
        info,
        "difficultyHeroic",
        Val::Bool(f.difficulty_heroic),
    );
    table_set(
        state,
        info,
        "difficultyMythic",
        Val::Bool(f.difficulty_mythic),
    );
    table_set(
        state,
        info,
        "difficultyMythicPlus",
        Val::Bool(f.difficulty_mythic_plus),
    );
    table_set(
        state,
        info,
        "generalPlaystyle1",
        Val::Bool(f.general_playstyle1),
    );
    table_set(
        state,
        info,
        "generalPlaystyle2",
        Val::Bool(f.general_playstyle2),
    );
    table_set(
        state,
        info,
        "generalPlaystyle3",
        Val::Bool(f.general_playstyle3),
    );
    table_set(
        state,
        info,
        "generalPlaystyle4",
        Val::Bool(f.general_playstyle4),
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
    table_set_rust_fn_static(
        state,
        table_ref,
        "HasSearchResultInfo",
        has_search_result_info,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetSearchResults", get_search_results)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetFilteredSearchResults",
        get_filtered_search_results,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetPremadeGroupFinderStyle",
        get_premade_group_finder_style,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "CanCreateQuestGroup",
        can_create_quest_group,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "CanCreateScenarioGroup",
        can_create_scenario_group,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsPremadeGroupFinderEnabled",
        is_premade_group_finder_enabled,
    )?;
    table_set_rust_fn_static(state, table_ref, "RemoveListing", remove_listing)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSearchResultMemberCounts",
        get_search_result_member_counts,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetGroupLeaverCountsByRole",
        get_group_leaver_counts_by_role,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetApplications", get_applications)?;
    table_set_rust_fn_static(state, table_ref, "GetApplicationInfo", get_application_info)?;
    table_set_rust_fn_static(state, table_ref, "ApplyToGroup", apply_to_group)?;
    table_set_rust_fn_static(state, table_ref, "CancelApplication", cancel_application)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetActivityInfoTable",
        get_activity_info_table,
    )?;
    table_set_rust_fn_static(state, table_ref, "Search", search)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAvailableCategories",
        get_available_categories,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetLfgCategoryInfo",
        get_lfg_category_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAvailableActivityGroups",
        get_available_activity_groups,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAvailableActivities",
        get_available_activities,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetActivityGroupInfo",
        get_activity_group_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetActivityFullName",
        get_activity_full_name,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetPlaystyleString", get_playstyle_string)?;
    table_set_rust_fn_static(state, table_ref, "HasActivityList", has_activity_list)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "HasActiveEntryInfo",
        has_active_entry_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetActiveEntryInfo",
        get_active_entry_info,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetAvailableRoles", get_available_roles)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAvailableLanguageSearchFilter",
        get_available_language_search_filter,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetLanguageSearchFilter",
        get_language_search_filter,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetDefaultLanguageSearchFilter",
        get_default_language_search_filter,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetAdvancedFilter", get_advanced_filter)?;
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
