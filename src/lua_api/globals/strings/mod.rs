//! UI string registration for the rilua global surface.

pub mod string_data;

use crate::loader::helpers::resolve_lua_escapes;
use crate::lua_api::methods::{create_string, table_set};
use rilua::LuaApiMut;
use rilua::Val;

pub fn register_all_ui_strings(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let global = Val::Table(state.global);
    for (name, value) in crate::global_strings::GLOBAL_STRINGS.entries() {
        let resolved = resolve_lua_escapes(value);
        let lua_value = create_string(state, &resolved);
        table_set(state, global, name, lua_value);
    }
    for defs in INT_DEFS {
        for &(name, value) in defs.iter() {
            table_set(state, global, name, Val::Num(value as f64));
        }
    }
    for defs in FLOAT_DEFS {
        for &(name, value) in defs.iter() {
            table_set(state, global, name, Val::Num(value));
        }
    }
    for defs in STRING_DEFS {
        for &(name, value) in defs.iter() {
            let resolved = resolve_lua_escapes(value);
            let lua_value = create_string(state, &resolved);
            table_set(state, global, name, lua_value);
        }
    }
    Ok(())
}

const INT_DEFS: &[&[crate::lua_api::globals::strings::string_data::IntDef]] = &[
    string_data::GAME_INT_CONSTANTS,
    string_data::EXPANSION_CONSTANTS,
    string_data::AUTOCOMPLETE_CONSTANTS,
    string_data::INVENTORY_SLOT_CONSTANTS,
    string_data::GUILD_NEWS_CONSTANTS,
    string_data::COMBAT_LOG_RAID_TARGET_CONSTANTS,
    string_data::TOTEM_SLOT_CONSTANTS,
    string_data::LFG_CATEGORY_CONSTANTS,
    string_data::GAME_ERROR_STRINGS,
    string_data::ACTIONBAR_STATE_CONSTANTS,
    string_data::FRAME_TUTORIAL_CONSTANTS,
];

const FLOAT_DEFS: &[&[crate::lua_api::globals::strings::string_data::FloatDef]] = &[
    string_data::TAXI_FLOAT_CONSTANTS,
    string_data::MOVEMENT_FLOAT_CONSTANTS,
];

const STRING_DEFS: &[&[crate::lua_api::globals::strings::string_data::StringDef]] =
    &[string_data::FONT_COLOR_CODE_STRINGS];
