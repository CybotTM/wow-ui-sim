//! Enum and constant globals: `Enum.*`, `Constants.*`, LE_* values.

use crate::lua_api::globals::enum_data::{EXPLICIT_ENUMS, SEQUENTIAL_ENUMS};
use crate::lua_api::methods::{create_table, table_get, table_set};
use rilua::LuaApiMut;
use rilua::Val;

const MISSING_ENUMS_LUA: &str = include_str!("../globals/enum_data/missing_enums.lua");
const COMPAT_ENUMS_LUA: &str = include_str!("../globals/enum_data/compat_enums.lua");
const MISSING_CONSTANTS_LUA: &str = include_str!("../globals/enum_data/missing_constants.lua");
const CONSTANTS_VALUES_LUA: &str = include_str!("../globals/enum_data/constants_values.lua");
const COMPAT_CONSTANTS_LUA: &str = include_str!("../globals/enum_data/compat_constants.lua");

pub(crate) fn init_enum_globals(lua: &mut rilua::Lua) -> crate::Result<()> {
    {
        let state = lua.state_mut();
        let enum_table = ensure_global_table(state, "Enum");
        // Ensure semantics: this also runs from `restore_post_cleanup_globals`
        // after Blizzard_EnvironmentCleanup, in the middle of the addon load
        // and again after it. Replacing a sub-table there dropped the members
        // the PTR bootstrap had appended (Enum.CooldownViewerCategory's
        // GroupBuff..EquipSlotTracked) and the ones Blizzard Lua adds itself
        // (HiddenActive / HiddenPassive), so three CooldownViewer files
        // aborted at file scope on a nil table key. An existing table is kept
        // and only missing members are filled in.
        for &(enum_name, entries) in EXPLICIT_ENUMS.iter() {
            let enum_values = ensure_enum_table(state, enum_table, enum_name);
            for &(variant_name, value) in entries {
                if matches!(table_get(state, enum_values, variant_name), Val::Nil) {
                    table_set(state, enum_values, variant_name, Val::Num(value as f64));
                }
            }
        }
        for &(enum_name, entries) in SEQUENTIAL_ENUMS.iter() {
            let enum_values = ensure_enum_table(state, enum_table, enum_name);
            for (index, &variant_name) in entries.iter().enumerate() {
                if matches!(table_get(state, enum_values, variant_name), Val::Nil) {
                    table_set(state, enum_values, variant_name, Val::Num(index as f64));
                }
            }
        }
    }
    lua.exec(MISSING_ENUMS_LUA)?;
    lua.exec(COMPAT_ENUMS_LUA)?;
    #[cfg(feature = "retail-12-1-0")]
    {
        let state = lua.state_mut();
        let enum_table = ensure_global_table(state, "Enum");
        ensure_on_update_mode_enum(state, enum_table);
    }
    lua.exec(
        r#"
        Constants = Constants or {}
        setmetatable(Constants, {
            __index = function(t, key)
                local value = {}
                rawset(t, key, value)
                return value
            end,
        })
        "#,
    )?;
    lua.exec(MISSING_CONSTANTS_LUA)?;
    lua.exec(CONSTANTS_VALUES_LUA)?;
    lua.exec(COMPAT_CONSTANTS_LUA)?;
    Ok(())
}

/// `Enum[name]`, created when absent and kept when a table already exists.
fn ensure_enum_table(state: &mut rilua::vm::state::LuaState, enum_table: Val, name: &str) -> Val {
    let existing = table_get(state, enum_table, name);
    if matches!(existing, Val::Table(_)) {
        return existing;
    }
    let created = create_table(state);
    table_set(state, enum_table, name, created);
    created
}

#[cfg(feature = "retail-12-1-0")]
fn ensure_on_update_mode_enum(state: &mut rilua::vm::state::LuaState, enum_table: Val) {
    let existing = table_get(state, enum_table, "OnUpdateMode");
    if matches!(existing, Val::Table(_)) {
        return;
    }
    let mode = create_table(state);
    for name in [
        "Disabled",
        "RunWhenVisible",
        "RunWhenVisibleOnce",
        "RunOnce",
        "RunAlways",
    ] {
        let value = crate::lua_api::methods::create_string(state, name);
        table_set(state, mode, name, value);
    }
    table_set(state, enum_table, "OnUpdateMode", mode);
    table_set(state, enum_table, "ScriptObjectOnUpdateMode", mode);
}

fn ensure_global_table(state: &mut rilua::vm::state::LuaState, key: &str) -> Val {
    let global = Val::Table(state.global);
    let existing = table_get(state, global, key);
    if matches!(existing, Val::Table(_)) {
        return existing;
    }
    let table = create_table(state);
    table_set(state, global, key, table);
    table
}
