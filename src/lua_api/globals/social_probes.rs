//! Social / character-sheet probe globals.
//!
//! - `GetNumTitles()`           → `SimState.titles.len()`
//! - `GetTitleName(index)`      → `SimState.titles[index-1]` (nil out
//!   of range).
//! - `IsTitleKnown(index)`      → `1 ≤ index ≤ SimState.titles.len()`.
//! - `GetCurrentTitle()`        → `SimState.current_title` (-1 = none).
//! - `SetCurrentTitle(index)`   → updates `SimState.current_title` and
//!   fires `UNIT_NAME_UPDATE("player")` so the title pane refreshes.
//! - `GetNumClasses()`          → `CLASS_LABELS.len()` (13 — the
//!   canonical retail class count).

use crate::lua_api::game_data::{CLASS_LABELS, class_info_by_index};
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_string_static, create_table, table_set,
};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn stack_i32(state: &LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn get_num_titles(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.titles.len() as f64;
    state.push(Val::Num(n));
    Ok(1)
}

fn get_title_name(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(0);
    let name = {
        let sim = borrow_state(state)?;
        usize::try_from(index.saturating_sub(1))
            .ok()
            .and_then(|idx| sim.titles.get(idx).cloned())
    };
    match name {
        Some(name) => {
            let val = create_string(state, &name);
            state.push(val);
            state.push(Val::Bool(true));
        }
        None => {
            state.push(Val::Nil);
            state.push(Val::Nil);
        }
    }
    Ok(2)
}

fn is_title_known(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(0);
    let known = if index < 1 {
        false
    } else {
        let sim = borrow_state(state)?;
        usize::try_from(index - 1)
            .ok()
            .map(|idx| idx < sim.titles.len())
            .unwrap_or(false)
    };
    state.push(Val::Bool(known));
    Ok(1)
}

fn get_current_title(state: &mut LuaState) -> LuaResult<u32> {
    let current = borrow_state(state)?.current_title;
    state.push(Val::Num(current as f64));
    Ok(1)
}

fn set_current_title(state: &mut LuaState) -> LuaResult<u32> {
    let requested = stack_i32(state, 1).unwrap_or(-1);
    {
        let mut sim = borrow_state_mut(state)?;
        let resolved = if requested < 1 {
            -1
        } else if usize::try_from(requested - 1)
            .ok()
            .map(|idx| idx < sim.titles.len())
            .unwrap_or(false)
        {
            requested
        } else {
            -1
        };
        sim.current_title = resolved;
    }
    let unit = create_string_static(state, "player");
    dispatch_event_now(state, "UNIT_NAME_UPDATE", &[unit])?;
    Ok(0)
}

fn get_num_classes(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(CLASS_LABELS.len() as f64));
    Ok(1)
}

fn get_class_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(1);
    let (class_name, class_file, class_id) = class_info_by_index(index);
    let class_name = create_string_static(state, class_name);
    let class_file = create_string_static(state, class_file);
    state.push(class_name);
    state.push(class_file);
    state.push(Val::Num(class_id as f64));
    Ok(3)
}

fn localized_class_list(state: &mut LuaState) -> LuaResult<u32> {
    let classes = create_table(state);
    for (class_file, class_name) in crate::lua_api::game_data::CLASS_FILES
        .iter()
        .zip(CLASS_LABELS.iter())
    {
        let value = create_string_static(state, class_name);
        table_set(state, classes, class_file, value);
    }
    state.push(classes);
    Ok(1)
}

fn get_shapeshift_form_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetNumTitles", get_num_titles)?;
    LuaApiMut::register_function(lua, "GetTitleName", get_title_name)?;
    LuaApiMut::register_function(lua, "IsTitleKnown", is_title_known)?;
    LuaApiMut::register_function(lua, "GetCurrentTitle", get_current_title)?;
    LuaApiMut::register_function(lua, "SetCurrentTitle", set_current_title)?;
    LuaApiMut::register_function(lua, "GetNumClasses", get_num_classes)?;
    LuaApiMut::register_function(lua, "GetClassInfo", get_class_info)?;
    LuaApiMut::register_function(lua, "LocalizedClassList", localized_class_list)?;
    LuaApiMut::register_function(lua, "GetShapeshiftFormID", get_shapeshift_form_id)?;
    Ok(())
}
