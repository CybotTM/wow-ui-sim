//! Social / character-sheet probe globals.
//!
//! Migrates 4 entries off `GLOBAL_ZERO_STUBS`:
//!
//! - `GetNumTitles()`           → `SimState.titles.len()`
//! - `GetTitleName(index)`      → `SimState.titles[index-1]` (nil out
//!   of range).
//! - `GetNumClasses()`          → `CLASS_LABELS.len()` (13 — the
//!   canonical retail class count).
//! - `GetNumShapeshiftForms()`  → `SimState.shapeshift_forms.len()`.
//!
//! Titles and shapeshift forms are backed by simple `Vec<String>`
//! fields on `SimState`. Empty by default; tests seed them via direct
//! SimState access.

use crate::lua_api::game_data::{CLASS_LABELS, class_info_by_index};
use crate::lua_api::methods::{borrow_state, create_string};
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
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_num_classes(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(CLASS_LABELS.len() as f64));
    Ok(1)
}

fn get_class_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(1);
    let (class_name, class_file, class_id) = class_info_by_index(index);
    let class_name = create_string(state, class_name);
    let class_file = create_string(state, class_file);
    state.push(class_name);
    state.push(class_file);
    state.push(Val::Num(class_id as f64));
    Ok(3)
}

fn get_num_shapeshift_forms(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.shapeshift_forms.len() as f64;
    state.push(Val::Num(n));
    Ok(1)
}

fn get_shapeshift_form_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetNumTitles", get_num_titles)?;
    LuaApiMut::register_function(lua, "GetTitleName", get_title_name)?;
    LuaApiMut::register_function(lua, "GetNumClasses", get_num_classes)?;
    LuaApiMut::register_function(lua, "GetClassInfo", get_class_info)?;
    LuaApiMut::register_function(lua, "GetNumShapeshiftForms", get_num_shapeshift_forms)?;
    LuaApiMut::register_function(lua, "GetShapeshiftFormID", get_shapeshift_form_id)?;
    Ok(())
}
