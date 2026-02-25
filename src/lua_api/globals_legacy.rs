//! Global WoW API functions.
//!
//! Orchestrates registration of all WoW API globals by delegating to
//! sub-modules and registering core Lua overrides (print, ipairs, getmetatable).

use super::frame::{frame_lud, get_sim_state, lud_to_id};
use super::globals::addon_api::register_addon_api;
use super::globals::c_collection_api::register_c_collection_api;
use super::globals::c_item_api::register_c_item_api;
use super::globals::c_map_api::register_c_map_api;
use super::globals::c_misc_api::register_c_misc_api;
use super::globals::c_editmode_api::register_c_editmode_api;
use super::globals::c_event_utils_api::register_c_event_utils_api;
use super::globals::c_stubs_api::register_c_stubs_api;
use super::globals::c_quest_api::register_c_quest_api;
use super::globals::c_system_api::register_c_system_api;
use super::globals::constants_api::register_constants_api;
use super::globals::create_frame::create_frame_function;
use super::globals::cvar_api::register_cvar_api;
use super::globals::dropdown_api::register_dropdown_api;
use super::globals::enum_api::register_enum_api;
use super::globals::font_api::{create_standard_font_objects, register_font_api};
use super::globals::global_frames::register_global_frames;
use super::globals::item_api::register_item_api;
use super::globals::locale_api::register_locale_api;
use super::globals::mixin_api::register_mixin_api;
use super::globals::player_api::register_player_api;
use super::globals::quest_frames::register_quest_frames;
use super::globals::register_all_ui_strings;
use super::globals::settings_api::register_settings_api;
use super::globals::sound_api::register_sound_api;
use super::globals::cursor_api;
use super::globals::spell_api::register_spell_api;
use super::globals::early_globals::register_early_globals;
use super::globals::frame_enumerate::register_frame_enumerate;
use super::globals::frame_level_api::register_frame_level_helpers;
use super::globals::system_api::register_system_api;
use super::globals::timer_api::register_timer_api;
use super::globals::tooltip_api::register_tooltip_frames;
use super::globals::unit_api::register_unit_api;
use super::globals::utility_api::register_utility_api;
use super::globals::abbreviate_config::register_abbreviate_config;
use super::globals::lua_duration_object::register_lua_duration_object;
use super::globals::unit_heal_prediction::register_unit_heal_prediction;
use super::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Register all global WoW API functions.
pub fn register_globals(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    // Store SimState in Lua app_data for LightUserData methods to access
    lua.set_app_data(Rc::clone(&state));
    // Set up the shared LightUserData metatable for all frames
    super::frame::metatable::setup_frame_metatable(lua)?;

    register_print(lua, Rc::clone(&state))?;
    register_custom_ipairs(lua, Rc::clone(&state))?;
    register_custom_getmetatable(lua)?;
    register_custom_setmetatable(lua)?;
    register_create_frame(lua, Rc::clone(&state))?;
    register_submodule_apis(lua, &state)?;
    register_ui_strings_and_fonts(lua)?;
    patch_string_format(lua)?;
    Ok(())
}

/// Lua source for patching string.format to support:
/// - %F (uppercase float) which Lua 5.1 lacks; converted to %f
/// - Positional arguments (%1$s, %2$d, %11$s) which WoW's patched LuaJIT supports
///   but standard Lua 5.1 does not; converted by reordering arguments
const STRING_FORMAT_PATCH: &str = r#"
    local _format = string.format
    local function _clean_format_error(msg)
        -- Strip source location prefix e.g. "[string \"...\"]:6: " added by lua.load
        msg = msg:gsub("^%[string [^%]]*%]:%d+: ", "")
        -- Replace internal function name '_format' with '?' to match WoW error style
        msg = msg:gsub("'_format'", "'?'")
        return msg
    end
    local function _safe_format(fmt, ...)
        local ok, result = pcall(_format, fmt, ...)
        if ok then return result end
        error(_clean_format_error(result), 2)
    end
    string.format = function(fmt, ...)
        if type(fmt) ~= "string" then return _safe_format(fmt, ...) end
        fmt = fmt:gsub("%%(%d*%.?%d*)F", "%%%1f")
        if not fmt:find("%%%d+%$") then return _safe_format(fmt, ...) end
        local args = {...}
        local out, new_args, seq = {}, {}, 0
        local i, len = 1, #fmt
        while i <= len do
            if fmt:sub(i,i) ~= "%" then
                out[#out+1] = fmt:sub(i,i); i = i + 1
            elseif fmt:sub(i+1,i+1) == "%" then
                out[#out+1] = "%%"; i = i + 2
            else
                local n, a = fmt:match("^%%(%d+)%$()", i)
                if n then
                    if tonumber(n) >= 100 then
                        error("invalid format (width or precision too long)", 2)
                    end
                    new_args[#new_args+1] = args[tonumber(n)]
                    out[#out+1] = "%"; i = a
                else
                    seq = seq + 1
                    new_args[#new_args+1] = args[seq]
                    out[#out+1] = "%"; i = i + 1
                end
                while i <= len and fmt:sub(i,i):find("[%-+ #0]") do
                    out[#out+1] = fmt:sub(i,i); i = i + 1
                end
                while i <= len and fmt:sub(i,i):find("%d") do
                    out[#out+1] = fmt:sub(i,i); i = i + 1
                end
                if i <= len and fmt:sub(i,i) == "." then
                    out[#out+1] = "."; i = i + 1
                    while i <= len and fmt:sub(i,i):find("%d") do
                        out[#out+1] = fmt:sub(i,i); i = i + 1
                    end
                end
                if i <= len and fmt:sub(i,i):find("[diouxXeEfgGaAcspqn]") then
                    out[#out+1] = fmt:sub(i,i); i = i + 1
                end
            end
        end
        return _safe_format(table.concat(out), unpack(new_args))
    end
    format = string.format
"#;

/// Patch string.format to handle %F and positional arguments.
fn patch_string_format(lua: &Lua) -> Result<()> {
    lua.load(STRING_FORMAT_PATCH).exec()
}

/// Override `print` to capture output to the console buffer (shown in GUI log panel).
fn register_print(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let print_func = lua.create_function(move |_lua, args: mlua::Variadic<Value>| {
        let output = format_print_args(&args);
        state.borrow_mut().console_output.push(output);
        Ok(())
    })?;
    lua.globals().set("print", print_func)
}

/// Format variadic print arguments with tab separators, matching WoW's print behavior.
fn format_print_args(args: &[Value]) -> String {
    let mut output = String::new();
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            output.push('\t');
        }
        match arg {
            Value::Nil => output.push_str("nil"),
            Value::Boolean(b) => output.push_str(if *b { "true" } else { "false" }),
            Value::Integer(n) => output.push_str(&n.to_string()),
            Value::Number(n) => output.push_str(&n.to_string()),
            Value::String(s) => output.push_str(&s.to_string_lossy()),
            Value::Table(_) => output.push_str("table"),
            Value::Function(_) => output.push_str("function"),
            Value::UserData(_) => output.push_str("userdata"),
            _ => output.push_str(&format!("{:?}", arg)),
        }
    }
    output
}

/// Override `ipairs` to support iterating over frame LightUserData children.
///
/// WoW addons iterate frame children with `for i, child in ipairs(frame)`.
/// Falls back to the original `ipairs` for regular tables.
fn register_custom_ipairs(lua: &Lua, _state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    let original_ipairs: mlua::Function = globals.get("ipairs")?;

    let custom_ipairs = lua.create_function(move |lua, value: Value| {
        if let Value::LightUserData(lud) = &value {
            return create_frame_children_iterator(lua, lud_to_id(*lud));
        }
        let original_ipairs: mlua::Function = lua.named_registry_value("__original_ipairs")?;
        original_ipairs.call(value)
    })?;

    lua.set_named_registry_value("__original_ipairs", original_ipairs)?;
    globals.set("ipairs", custom_ipairs)
}

/// Create a stateless iterator over a frame's children for use with `ipairs`.
///
/// Returns `(iterator_fn, nil, 0)` matching Lua's generic for protocol.
fn create_frame_children_iterator(lua: &Lua, frame_id: u64) -> Result<mlua::MultiValue> {
    let state_rc = get_sim_state(lua);
    let children: Vec<u64> = {
        let st = state_rc.borrow();
        st.widgets.get(frame_id).map(|f| f.children.clone()).unwrap_or_default()
    };

    let iterator = lua.create_function(move |_lua, (_, idx): (Value, i32)| {
        let next_idx = idx + 1;
        if next_idx as usize > children.len() {
            return Ok(mlua::MultiValue::new());
        }
        let child_id = children[(next_idx - 1) as usize];
        Ok(mlua::MultiValue::from_vec(vec![
            Value::Integer(next_idx as i64),
            frame_lud(child_id),
        ]))
    })?;

    Ok(mlua::MultiValue::from_vec(vec![
        Value::Function(iterator),
        Value::Nil,
        Value::Integer(0),
    ]))
}

/// Override `getmetatable` to return a proper metatable for frame LightUserData.
///
/// WoW addons expect `getmetatable(frame).__index` to be an iterable table
/// of method names mapped to functions.
fn register_custom_getmetatable(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    let custom_getmetatable = lua.create_function(|lua, value: Value| {
        if let Value::LightUserData(_) = &value {
            return build_frame_metatable(lua);
        }
        let real_getmetatable: mlua::Function = lua.named_registry_value("__real_getmetatable")?;
        real_getmetatable.call(value)
    })?;

    let real_getmetatable: mlua::Function = globals.get("getmetatable")?;
    lua.set_named_registry_value("__real_getmetatable", real_getmetatable)?;
    globals.set("getmetatable", custom_getmetatable)
}

/// Override `setmetatable` to support per-frame custom metatables on LightUserData.
///
/// WoW frames are tables with metatables, but our simulator uses LightUserData.
/// This stores per-frame metatables in a registry table `__frame_custom_mt`
/// so that `__newindex` can delegate to per-frame `__newindex` metamethods.
fn register_custom_setmetatable(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    // Create the per-frame custom metatable storage
    let custom_mt_store = lua.create_table()?;
    lua.set_named_registry_value("__frame_custom_mt", custom_mt_store)?;

    let custom_setmetatable = lua.create_function(|lua, (value, mt): (Value, Value)| {
        if let Value::LightUserData(lud) = &value {
            let id = lud.0 as u64;
            let store: mlua::Table = lua.named_registry_value("__frame_custom_mt")?;
            match &mt {
                Value::Table(_) => store.set(id, mt)?,
                Value::Nil => store.set(id, Value::Nil)?,
                _ => {}
            }
            return Ok(value);
        }
        let real_setmetatable: mlua::Function = lua.named_registry_value("__real_setmetatable")?;
        real_setmetatable.call((value, mt))
    })?;

    let real_setmetatable: mlua::Function = globals.get("setmetatable")?;
    lua.set_named_registry_value("__real_setmetatable", real_setmetatable)?;
    globals.set("setmetatable", custom_setmetatable)
}

/// Build a fake metatable for frame LightUserData with `__index` from the methods table.
fn build_frame_metatable(lua: &Lua) -> Result<Value> {
    use crate::lua_api::frame::method_registry;
    let mt = lua.create_table()?;
    let all_methods: mlua::Table = lua.named_registry_value("__frame_methods_table")?;
    let index_table = lua.create_table()?;
    // Include all methods from all widget types (union).
    for pair in all_methods.pairs::<String, Value>() {
        let (name, func) = pair?;
        // Only include methods from discovery data — skip Mixin/sim methods.
        if method_registry::is_known_method(&name) {
            index_table.set(name, func)?;
        }
    }
    mt.set("__index", index_table)?;
    Ok(Value::Table(mt))
}

/// Register `CreateFrame` from its dedicated sub-module.
fn register_create_frame(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let create_frame = create_frame_function(lua, state)?;
    lua.globals().set("CreateFrame", create_frame)
}

/// Register all sub-module APIs (locale, addon, unit, timer, etc.).
fn register_submodule_apis(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    register_stateless_apis(lua, state)?;
    register_cursor_apis(lua, state)?;
    register_stateful_apis(lua, state)?;
    register_frame_globals(lua, state)?;
    super::globals::generated_stubs::register_generated_stubs(lua)
}

/// Register APIs that don't require mutable SimState access.
fn register_stateless_apis(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    super::globals::event_query_api::register(lua)?;
    register_locale_api(lua)?;
    register_player_api(lua, state.clone())?;
    register_enum_api(lua)?;
    register_constants_api(lua)?;
    register_c_map_api(lua)?;
    register_c_quest_api(lua)?;
    register_c_collection_api(lua)?;
    register_c_item_api(lua)?;
    register_c_misc_api(lua, Rc::clone(state))?;
    register_c_system_api(lua)?;
    register_c_stubs_api(lua, Rc::clone(state))?;
    register_c_editmode_api(lua)?;
    register_c_event_utils_api(lua)?;
    register_mixin_api(lua)?;
    register_utility_api(lua)?;
    register_settings_api(lua)?;
    register_spell_api(lua, Rc::clone(state))?;
    register_item_api(lua)?;
    register_font_api(lua)?;
    register_abbreviate_config(lua)?;
    register_lua_duration_object(lua)?;
    register_unit_heal_prediction(lua)
}

/// Register cursor/drag-and-drop APIs (must run after C_Spell and C_ActionBar).
fn register_cursor_apis(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    cursor_api::register_cursor_functions(lua, Rc::clone(state))?;
    cursor_api::register_c_spell_pickup(lua, state)?;
    cursor_api::register_c_action_bar_put(lua, state)
}

/// Register stateful APIs that need SimState.
fn register_stateful_apis(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    register_sound_api(lua, Rc::clone(state))?;
    register_unit_api(lua, Rc::clone(state))?;
    register_addon_api(lua, Rc::clone(state))?;
    register_timer_api(lua, Rc::clone(state))?;
    register_dropdown_api(lua, Rc::clone(state))?;
    register_cvar_api(lua, Rc::clone(state))?;
    register_system_api(lua, Rc::clone(state))?;
    register_frame_level_helpers(lua)?;
    register_frame_enumerate(lua)?;
    register_early_globals(lua)
}

/// Register global frame objects and sync named frames to _G.
fn register_frame_globals(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    register_global_frames(lua, Rc::clone(state))?;
    register_tooltip_frames(lua, Rc::clone(state))?;
    register_quest_frames(lua, Rc::clone(state))?;
    // Ensure all named frames have _G entries. Covers frames created by
    // builtin_frames.rs (no Lua access) and any registration site that
    // only sets the widget registry name without calling raw_set on _G.
    sync_named_frames_to_globals(lua, state)
}

/// Set `_G[name]` and `_G["__frame_{id}"]` for every named frame in the registry
/// that doesn't already have a `_G` entry.
fn sync_named_frames_to_globals(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    let st = state.borrow();
    for (id, name) in st.widgets.named_frames() {
        let lud = super::frame::frame_lud(id);
        // Only set if not already present (avoid overwriting setup done above)
        if globals.raw_get::<Value>(name.as_str())?.is_nil() {
            globals.raw_set(name.as_str(), lud.clone())?;
        }
        let frame_key = format!("__frame_{}", id);
        if globals.raw_get::<Value>(frame_key.as_str())?.is_nil() {
            globals.raw_set(frame_key.as_str(), lud)?;
        }
    }
    Ok(())
}

/// Register UI string constants and create standard font objects.
fn register_ui_strings_and_fonts(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    register_all_ui_strings(lua, &globals)?;
    create_standard_font_objects(lua)
}

