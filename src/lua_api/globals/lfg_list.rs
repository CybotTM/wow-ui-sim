//! `C_LFGList` namespace — Group Finder search, category, and activity APIs.
//!
//! Backed by `SimState::lfg_category_info`, `lfg_activity_groups`,
//! `lfg_activities`, and `world.premade_listings`.

use crate::event::Event;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, create_table, frame_ref,
    table_set,
};
use crate::lua_api::script_helpers::{get_event_listeners, get_script};
use crate::lua_api::state_types::{LfgActivityInfo, PremadeListing};
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
    fire_named_event(state, "LFG_LIST_SEARCH_RESULTS_RECEIVED")?;
    Ok(0)
}

fn get_activity_info_table(state: &mut LuaState) -> LuaResult<u32> {
    let activity_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    let Some(act) = find_activity(state, activity_id) else {
        return Ok(0);
    };
    let info = create_table(state);
    let full_name = create_string(state, &act.full_name.clone());
    let short_name = create_string(state, &act.short_name.clone());
    table_set(state, info, "activityID", Val::Num(activity_id as f64));
    table_set(state, info, "fullName", full_name);
    table_set(state, info, "shortName", short_name);
    table_set(
        state,
        info,
        "categoryID",
        Val::Num(act.category_id as f64),
    );
    table_set(
        state,
        info,
        "groupFinderActivityGroupID",
        Val::Num(act.group_id as f64),
    );
    table_set(
        state,
        info,
        "maxPlayers",
        Val::Num(act.max_players as f64),
    );
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
    table_set(
        state,
        info,
        "useHonorLevel",
        Val::Bool(act.use_honor_level),
    );
    state.push(info);
    Ok(1)
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
    let name = create_string(state, &info.name);
    table_set(state, t, "name", name);
    table_set(
        state,
        t,
        "separateRecommended",
        Val::Bool(info.separate_recommended),
    );
    table_set(
        state,
        t,
        "preferCurrentArea",
        Val::Bool(info.prefer_current_area),
    );
    table_set(
        state,
        t,
        "allowCrossFaction",
        Val::Bool(info.allow_cross_faction),
    );
    table_set(
        state,
        t,
        "autoChooseActivity",
        Val::Bool(info.auto_choose_activity),
    );
    table_set(
        state,
        t,
        "showPlaystyleDropdown",
        Val::Bool(info.show_playstyle_dropdown),
    );
    table_set(state, t, "searchPromptOverride", Val::Nil);
    state.push(t);
    Ok(1)
}

/// `GetAvailableActivityGroups(categoryID, filters?)` → array of groupIDs.
fn get_available_activity_groups(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let filters = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as u32;
    let mut groups = {
        let sim = borrow_state(state)?;
        sim.lfg_activity_groups
            .iter()
            .filter(|g| {
                g.category_id == category_id
                    && (filters == 0 || g.filters & filters != 0)
            })
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
    let category_id = Option::<f64>::from_stack(state, 1)?.map(|v| v as i32);
    let group_id = Option::<f64>::from_stack(state, 2)?.map(|v| v as u32);
    let filters = Option::<f64>::from_stack(state, 3)?.unwrap_or(0.0) as u32;
    let search_term = Option::<String>::from_stack(state, 4)?;
    let st = search_term.as_deref().map(|s| s.to_lowercase());
    let mut acts = {
        let sim = borrow_state(state)?;
        sim.lfg_activities
            .iter()
            .filter(|a| {
                if let Some(cat) = category_id {
                    if a.category_id != cat {
                        return false;
                    }
                }
                if let Some(grp) = group_id {
                    if a.group_id != grp {
                        return false;
                    }
                }
                if filters != 0 && a.filters & filters == 0 {
                    return false;
                }
                if let Some(ref term) = st {
                    if !a.full_name.to_lowercase().contains(term.as_str()) {
                        return false;
                    }
                }
                true
            })
            .map(|a| (a.activity_id, a.order_index))
            .collect::<Vec<_>>()
    };
    acts.sort_by_key(|(_, order)| *order);
    let result = create_table(state);
    if let Val::Table(table_ref) = result {
        for (index, (activity_id, _)) in acts.iter().enumerate() {
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
    state.push(result);
    Ok(1)
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
            let _ = table.raw_set(
                Val::Num(1.0),
                Val::Str(lang),
                &state.gc.string_arena,
            );
        }
        state.gc.barrier_back(table_ref);
    }
    state.push(result);
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
