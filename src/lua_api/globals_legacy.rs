//! Global WoW API functions.
//!
//! Orchestrates registration of all WoW API globals by delegating to
//! sub-modules and registering core Lua overrides (print, ipairs, getmetatable).

use super::frame::{extract_frame_id, frame_ref};
use super::frame::get_sim_state;
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
    // Store SimState in Lua app_data for UserData methods to access
    lua.set_app_data(Rc::clone(&state));
    // Set up shared frame helpers (assign fn, index helper, forbidden proxy mt, methods table)
    super::frame::metatable::setup_frame_helpers(lua)?;

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

/// Override `ipairs` to support iterating over frame UserData (FrameRef) children.
///
/// WoW addons iterate frame children with `for i, child in ipairs(frame)`.
/// Falls back to the original `ipairs` for regular tables.
fn register_custom_ipairs(lua: &Lua, _state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    let original_ipairs: mlua::Function = globals.get("ipairs")?;

    let custom_ipairs = lua.create_function(move |lua, value: Value| {
        if let Some(id) = extract_frame_id(&value) {
            return create_frame_children_iterator(lua, id);
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

    let iterator = lua.create_function(move |lua, (_, idx): (Value, i32)| {
        let next_idx = idx + 1;
        if next_idx as usize > children.len() {
            return Ok(mlua::MultiValue::new());
        }
        let child_id = children[(next_idx - 1) as usize];
        Ok(mlua::MultiValue::from_vec(vec![
            Value::Integer(next_idx as i64),
            frame_ref(lua, child_id)?,
        ]))
    })?;

    Ok(mlua::MultiValue::from_vec(vec![
        Value::Function(iterator),
        Value::Nil,
        Value::Integer(0),
    ]))
}

/// Override `getmetatable` to return a proper metatable for frame UserData (FrameRef).
///
/// WoW addons expect `getmetatable(frame).__index` to be an iterable table
/// of method names mapped to functions. Two frames of the same widget type
/// return the same metatable (identity check passes).
fn register_custom_getmetatable(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    // Build and cache per-type metatables now that __frame_methods_table is ready.
    build_per_type_metatables(lua)?;

    let custom_getmetatable = lua.create_function(|lua, value: Value| {
        if let Some(frame_id) = extract_frame_id(&value) {
            let widget_type = {
                let state_rc = get_sim_state(lua);
                let state = state_rc.borrow();
                state
                    .widgets
                    .get(frame_id)
                    .map(|f| f.widget_type)
                    .unwrap_or(crate::widget::WidgetType::Frame)
            };
            let type_key = widget_type.as_str();
            let per_type: mlua::Table = lua.named_registry_value("__per_type_metatables")?;
            let mt: Value = per_type.raw_get(type_key)?;
            return Ok(mt);
        }
        let real_getmetatable: mlua::Function = lua.named_registry_value("__real_getmetatable")?;
        real_getmetatable.call(value)
    })?;

    let real_getmetatable: mlua::Function = globals.get("getmetatable")?;
    lua.set_named_registry_value("__real_getmetatable", real_getmetatable)?;
    globals.set("getmetatable", custom_getmetatable)
}

/// Pre-build one metatable per widget type and store in `__per_type_metatables`.
///
/// Each metatable has exactly one key `__index` whose value is a table containing:
/// - All methods from `__frame_methods_table` that `is_method_allowed(type, name)` permits.
/// - Methods NOT in `is_known_method` (Mixin/sim-specific) are also included.
///
/// Two frames of the same widget type will receive the same table object from this
/// cache, so `getmetatable(frame1) == getmetatable(frame2)` passes in Lua.
fn build_per_type_metatables(lua: &Lua) -> Result<()> {
    let method_pairs = collect_method_pairs(lua)?;
    let per_type = lua.create_table()?;

    for widget_type in all_widget_types() {
        let type_key = widget_type.as_str();
        // Skip if already built (WorldFrame shares "Frame" key with Frame).
        if per_type.raw_get::<Value>(type_key)? != Value::Nil {
            continue;
        }
        let mt = build_metatable_for_type(lua, widget_type, &method_pairs)?;
        per_type.raw_set(type_key, mt)?;
    }

    lua.set_named_registry_value("__per_type_metatables", per_type)?;
    Ok(())
}

/// Collect all (name, func) pairs from `__frame_methods_table` into a Vec.
fn collect_method_pairs(lua: &Lua) -> Result<Vec<(String, Value)>> {
    let all_methods: mlua::Table = lua.named_registry_value("__frame_methods_table")?;
    let mut pairs = Vec::new();
    for pair in all_methods.pairs::<String, Value>() {
        let (name, func) = pair?;
        pairs.push((name, func));
    }
    Ok(pairs)
}

/// Build a `{ __index = { allowed methods } }` metatable for one widget type.
///
/// Each function in the `__index` table is wrapped in a unique thin closure so that
/// the cfuncs identity checker sees distinct function objects per type. In real WoW,
/// the C engine creates separate function wrappers per widget type metatable.
fn build_metatable_for_type(
    lua: &Lua,
    widget_type: crate::widget::WidgetType,
    method_pairs: &[(String, Value)],
) -> Result<mlua::Table> {
    use crate::lua_api::frame::method_registry;

    let index_table = lua.create_table()?;
    for (name, func) in method_pairs {
        if method_registry::is_method_allowed(widget_type, name) {
            let wrapped = lua.create_function({
                let f = func.clone();
                move |_, args: mlua::MultiValue| {
                    match &f {
                        Value::Function(func) => func.call::<mlua::MultiValue>(args),
                        _ => Ok(mlua::MultiValue::new()),
                    }
                }
            })?;
            index_table.set(name.clone(), wrapped)?;
        }
    }
    let mt = lua.create_table()?;
    mt.set("__index", index_table)?;
    Ok(mt)
}

/// All widget type variants (used to pre-build per-type metatables).
fn all_widget_types() -> [crate::widget::WidgetType; 20] {
    use crate::widget::WidgetType;
    [
        WidgetType::Frame,
        WidgetType::WorldFrame,
        WidgetType::Button,
        WidgetType::CheckButton,
        WidgetType::Texture,
        WidgetType::FontString,
        WidgetType::EditBox,
        WidgetType::ScrollFrame,
        WidgetType::Slider,
        WidgetType::StatusBar,
        WidgetType::Cooldown,
        WidgetType::Model,
        WidgetType::ModelScene,
        WidgetType::PlayerModel,
        WidgetType::ColorSelect,
        WidgetType::MessageFrame,
        WidgetType::SimpleHTML,
        WidgetType::GameTooltip,
        WidgetType::Minimap,
        WidgetType::Line,
    ]
}

/// Override `setmetatable` to support per-frame custom metatables on UserData (FrameRef).
///
/// WoW frames are tables with metatables, but our simulator uses UserData.
/// This stores per-frame metatables in a registry table `__frame_custom_mt`
/// so that `__newindex` can delegate to per-frame `__newindex` metamethods.
fn register_custom_setmetatable(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    // Create the per-frame custom metatable storage
    let custom_mt_store = lua.create_table()?;
    lua.set_named_registry_value("__frame_custom_mt", custom_mt_store)?;

    let custom_setmetatable = lua.create_function(|lua, (value, mt): (Value, Value)| {
        if let Some(id) = extract_frame_id(&value) {
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
    register_c_map_api(lua, Rc::clone(state))?;
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
    // Admin API for simulator state control from Lua.
    super::globals::admin_api::register_admin_api(lua, Rc::clone(state))?;

    sync_named_frames_to_globals(lua, state)
}

/// Set `_G[name]` and `_G["__frame_{id}"]` for every named frame in the registry
/// that doesn't already have a `_G` entry.
fn sync_named_frames_to_globals(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    let ids_and_names: Vec<(u64, String)> = state.borrow().widgets.named_frames()
        .map(|(id, name)| (id, name.clone()))
        .collect();
    for (id, name) in ids_and_names {
        let val = frame_ref(lua, id)?;
        if globals.raw_get::<Value>(name.as_str())?.is_nil() {
            globals.raw_set(name.as_str(), val.clone())?;
        }
        let frame_key = format!("__frame_{}", id);
        if globals.raw_get::<Value>(frame_key.as_str())?.is_nil() {
            globals.raw_set(frame_key.as_str(), val)?;
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

