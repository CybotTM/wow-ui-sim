//! Install `__index` hooks on `_G` and `C_*` namespaces to log missing symbol accesses.

use crate::lua_api::SimState;
use mlua::{Lua, Result, Table, Value};
use std::cell::RefCell;
use std::rc::Rc;

const INSTALL_NIL_LOGGER_LUA: &str = r#"
return function(target, logger)
    local getinfo = debug.getinfo

    local function get_access_location()
        for level = 3, 30 do
            local info = getinfo(level, "Sl")
            if not info then
                break
            end

            if info.source and info.source ~= "[C]" and info.source ~= "=[C]" then
                return info.source, info.currentline
            end
        end

        return nil, nil
    end

    local mt = getmetatable(target)
    if type(mt) ~= "table" then
        mt = {}
    end

    local previous = mt.__index
    mt.__index = function(tbl, key)
        local value = nil
        if type(previous) == "function" then
            value = previous(tbl, key)
        elseif previous ~= nil then
            value = previous[key]
        end

        if value ~= nil then
            return value
        end

        local source, line = get_access_location()
        logger(key, source, line)
        return nil
    end

    setmetatable(target, mt)
end
"#;

/// Install nil-access logging hooks on `_G` and all current `C_*` namespace tables.
pub fn install(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let installer: mlua::Function = lua.load(INSTALL_NIL_LOGGER_LUA).eval()?;
    let globals = lua.globals();

    install_table_nil_logger(lua, &installer, globals.clone(), "_G", Rc::clone(&state))?;

    for (name, table) in collect_c_namespace_tables(&globals)? {
        install_table_nil_logger(lua, &installer, table, &name, Rc::clone(&state))?;
    }

    Ok(())
}

fn collect_c_namespace_tables(globals: &Table) -> Result<Vec<(String, Table)>> {
    let mut namespaces = Vec::new();
    for pair in globals.clone().pairs::<Value, Value>() {
        let (key, value) = pair?;
        let Value::String(key) = key else {
            continue;
        };
        let name = key.to_string_lossy();
        if !name.starts_with("C_") {
            continue;
        }
        let Value::Table(table) = value else {
            continue;
        };
        namespaces.push((name.to_string(), table));
    }
    Ok(namespaces)
}

fn install_table_nil_logger(
    lua: &Lua,
    installer: &mlua::Function,
    table: Table,
    container: &str,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    let container_name = container.to_string();
    let logger = lua.create_function(move |_, (key, source, line): (Value, Value, Value)| {
        let mut state = state.borrow_mut();
        let addon_name = current_addon_name(&state);
        state
            .nil_symbol_accesses
            .push(crate::lua_api::state::NilSymbolAccess {
                addon_name,
                container: container_name.clone(),
                key: format_missing_key(&key),
                source: format_source(source),
                line: format_line(line),
            });
        Ok(())
    })?;

    installer.call::<()>((table, logger))
}

fn current_addon_name(state: &SimState) -> Option<String> {
    state
        .executing_addon_index
        .or(state.loading_addon_index)
        .and_then(|index| {
            state
                .addons
                .get(index as usize)
                .map(|addon| addon.folder_name.clone())
        })
}

fn format_missing_key(key: &Value) -> String {
    match key {
        Value::String(key) => key.to_string_lossy().to_string(),
        Value::Integer(key) => key.to_string(),
        Value::Number(key) => key.to_string(),
        Value::Boolean(key) => key.to_string(),
        Value::Nil => "nil".to_string(),
        _ => format!("{key:?}"),
    }
}

fn format_source(source: Value) -> Option<String> {
    match source {
        Value::String(source) => Some(source.to_string_lossy().to_string()),
        Value::Nil => None,
        _ => None,
    }
}

fn format_line(line: Value) -> Option<i32> {
    match line {
        Value::Integer(line) => i32::try_from(line).ok(),
        Value::Number(line) => i32::try_from(line as i64).ok(),
        Value::Nil => None,
        _ => None,
    }
}
