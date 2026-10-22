//! Native legacy autocomplete argument/tuple boundary over the C API model.
//! Build 12340 authority: `autocomplete-results-12340.md`, laptop relay 2026-10-21.

use crate::c_api::c_auto_complete::wrath::{Query, query_names};
use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

const USAGE: &str = "Usage: GetAutoCompleteResults(text, includeBitField, excludeBitField, numReturns[, cursorPosition[, allowFullMatch]])";

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetAutoCompleteResults", get_results)?;
    // Preserve Lua 5.1 numeric-string coercion without a second number parser.
    lua.exec(
        r#"
        local query, number = GetAutoCompleteResults, tonumber
        local valueType, stringValue = type, tostring
        function GetAutoCompleteResults(text, include, exclude, count, cursor, allowFullMatch)
            if valueType(text) == "number" then text = stringValue(text) end
            return query(text, number(include), number(exclude), number(count), number(cursor), allowFullMatch)
        end
        "#,
    )?;
    Ok(())
}

fn get_results(state: &mut LuaState) -> LuaResult<u32> {
    let query = read_query(state)?;
    let names = {
        let sim = borrow_state(state)?;
        query_names(&sim, &query)
    };
    let count = names.len() as u32;
    for name in names {
        let value = create_string(state, &name);
        state.push(value);
    }
    Ok(count)
}

fn read_query(state: &LuaState) -> LuaResult<Query> {
    let mut text = String::from_stack(state, 1).map_err(|_| runtime_error(USAGE))?;
    if let Some(end) = text.find('\0') {
        text.truncate(end);
    }
    let include = read_number(state, 2)? as i64 as u32;
    let exclude = read_number(state, 3)? as i64 as u32;
    let max_results = read_number(state, 4)? as i32;
    let length = text.chars().count();
    let cursor = read_cursor(state, length);
    let allow_full_match = !matches!(stack_val(state, 6), Val::Nil | Val::Bool(false));
    Ok(Query {
        text,
        include,
        exclude,
        max_results,
        cursor,
        allow_full_match,
    })
}

fn read_number(state: &LuaState, index: i32) -> LuaResult<f64> {
    f64::from_stack(state, index).map_err(|_| runtime_error(USAGE))
}

fn read_cursor(state: &LuaState, length: usize) -> usize {
    f64::from_stack(state, 5)
        .ok()
        .filter(|position| *position >= 0.0 && *position <= length as f64)
        .map_or(length, |position| position as usize)
}
