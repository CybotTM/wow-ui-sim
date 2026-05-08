//! `C_LFGInfo` probe surface backed by `SimState.lfg_category_info` and
//! `SimState.lfg_active_categories`.
//!
//! Migrates 4 entries off the namespace stub tables:
//!
//! - `C_LFGInfo.CanPlayerUseLFD()` — returns `(true, nil)` matching the
//!   existing `c_lfg_info_can_player_use` behaviour in `c_model_info.rs`.
//! - `C_LFGInfo.CanPlayerUseGroupFinder()` — returns `(true, nil)` so the
//!   LFG micro menu button is available in the default simulator state.
//! - `C_LFGInfo.GetLFGCategoryInfo(categoryID)` — returns a table with
//!   `name` and `order` fields from `lfg_category_info`, or nil for
//!   unknown categories.
//! - `C_LFGInfo.GetSystemPanelData()` — returns a minimal table with
//!   `isAvailable = true` and `isAvailableAndEnabled = true`.
//! - `C_LFGInfo.IsLFGModeActiveForCategory(categoryID)` — returns true
//!   when the category id is in `lfg_active_categories`, false otherwise.

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_api::state_types::LfgCategoryInfo;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::collections::HashSet;

pub(super) fn register_lfg_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_LFGInfo")?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "CanPlayerUseGroupFinder",
        can_player_use_group_finder,
    )?;
    table_set_rust_fn_static(state, table_ref, "CanPlayerUseLFD", can_player_use_lfd)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetLFGCategoryInfo",
        get_lfg_category_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSystemPanelData",
        get_system_panel_data,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsLFGModeActiveForCategory",
        is_lfg_mode_active_for_category,
    )?;
    Ok(())
}

fn can_player_use_group_finder(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    state.push(Val::Nil);
    Ok(2)
}

fn can_player_use_lfd(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    state.push(Val::Nil);
    Ok(2)
}

fn get_lfg_category_info(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = i32::from_stack(state, 1)?;
    let info = lookup_lfg_category_info(state, category_id)?;
    push_lfg_category_info(state, info);
    Ok(1)
}

fn lookup_lfg_category_info(
    state: &LuaState,
    category_id: i32,
) -> LuaResult<Option<LfgCategoryInfo>> {
    let sim = borrow_state(state)?;
    Ok(sim.lfg_category_info.get(&category_id).cloned())
}

fn push_lfg_category_info(state: &mut LuaState, info: Option<LfgCategoryInfo>) {
    let Some(info) = info else {
        state.push(Val::Nil);
        return;
    };
    let t = create_table(state);
    populate_lfg_category_table(state, t, &info);
    state.push(t);
}

fn populate_lfg_category_table(state: &mut LuaState, table: Val, info: &LfgCategoryInfo) {
    populate_lfg_category_identity_fields(state, table.clone(), info);
    populate_lfg_category_flag_fields(state, table, info);
}

fn populate_lfg_category_identity_fields(state: &mut LuaState, table: Val, info: &LfgCategoryInfo) {
    let name_val = create_string(state, &info.name);
    table_set(state, table.clone(), "name", name_val);
    table_set(state, table.clone(), "order", Val::Num(info.order as f64));
}

fn populate_lfg_category_flag_fields(state: &mut LuaState, table: Val, info: &LfgCategoryInfo) {
    table_set(
        state,
        table.clone(),
        "separateRecommended",
        Val::Bool(info.separate_recommended),
    );
    table_set(
        state,
        table.clone(),
        "preferCurrentArea",
        Val::Bool(info.prefer_current_area),
    );
    table_set(
        state,
        table.clone(),
        "allowCrossFaction",
        Val::Bool(info.allow_cross_faction),
    );
    table_set(
        state,
        table.clone(),
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

fn get_system_panel_data(state: &mut LuaState) -> LuaResult<u32> {
    let t = create_table(state);
    table_set(state, t, "isAvailable", Val::Bool(true));
    table_set(state, t, "isAvailableAndEnabled", Val::Bool(true));
    state.push(t);
    Ok(1)
}

fn is_lfg_mode_active_for_category(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = i32::from_stack(state, 1)?;
    let active = {
        let sim = borrow_state(state)?;
        is_lfg_category_active(&sim.lfg_active_categories, category_id)
    };
    state.push(Val::Bool(active));
    Ok(1)
}

fn is_lfg_category_active(active_categories: &HashSet<i32>, category_id: i32) -> bool {
    active_categories.contains(&category_id)
}
