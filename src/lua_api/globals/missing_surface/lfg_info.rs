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
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

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
    let entry = {
        let sim = borrow_state(state)?;
        sim.lfg_category_info.get(&category_id).cloned()
    };
    let Some(info) = entry else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let t = create_table(state);
    let name_val = create_string(state, &info.name);
    table_set(state, t, "name", name_val);
    table_set(state, t, "order", Val::Num(info.order as f64));
    state.push(t);
    Ok(1)
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
    let active = borrow_state(state)?
        .lfg_active_categories
        .contains(&category_id);
    state.push(Val::Bool(active));
    Ok(1)
}
