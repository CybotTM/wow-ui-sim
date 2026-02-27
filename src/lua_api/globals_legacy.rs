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

/// Patch `string.format` with a Rust implementation that handles:
/// - `%F` (uppercase float) which Lua 5.1 lacks; converted to `%f`
/// - Positional arguments (`%1$s`, `%2$d`) which WoW's patched LuaJIT supports
///
/// Being a Rust/mlua function, it appears as a C function to Lua's `coroutine.create`,
/// matching WoW's real behavior where `string.format` is a C function.
fn patch_string_format(lua: &Lua) -> Result<()> {
    let string_table: mlua::Table = lua.globals().get("string")?;
    let original: mlua::Function = string_table.get("format")?;
    lua.set_named_registry_value("__original_string_format", original)?;

    let format_fn = lua.create_function(wow_string_format)?;
    string_table.set("format", format_fn.clone())?;
    lua.globals().set("format", format_fn)?;
    Ok(())
}

/// Rust implementation of WoW's extended `string.format`.
fn wow_string_format(lua: &mlua::Lua, mut args: mlua::MultiValue) -> mlua::Result<mlua::MultiValue> {
    let original: mlua::Function = lua.named_registry_value("__original_string_format")?;

    // Non-string first arg: pass through to original C string.format
    let fmt = match args.iter().next() {
        Some(Value::String(s)) => match s.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return original.call(args),
        },
        _ => return original.call(args),
    };

    // Fast path: no %F or positional args
    if !fmt.contains('F') && !fmt.contains('$') {
        return original.call(args);
    }

    args.pop_front();
    let rest: Vec<Value> = args.into_vec();
    let (new_fmt, new_rest) = process_wow_format(&fmt, &rest)?;
    call_with_processed_args(lua, &original, &new_fmt, new_rest)
}

/// Build MultiValue from processed format + args and call original string.format.
fn call_with_processed_args(
    lua: &mlua::Lua,
    original: &mlua::Function,
    fmt: &str,
    rest: Vec<Value>,
) -> mlua::Result<mlua::MultiValue> {
    let mut new_args = mlua::MultiValue::new();
    new_args.push_back(Value::String(lua.create_string(fmt)?));
    for arg in rest {
        new_args.push_back(arg);
    }
    original.call(new_args)
}

/// Parse format string: replace `%F` → `%f` and reorder positional args (`%1$s`).
fn process_wow_format(fmt: &str, args: &[Value]) -> mlua::Result<(String, Vec<Value>)> {
    let bytes = fmt.as_bytes();
    let mut out = String::with_capacity(bytes.len());
    let mut reordered: Vec<Value> = Vec::new();
    let mut seq: usize = 0;
    let mut has_positional = false;
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] != b'%' {
            out.push(bytes[i] as char);
            i += 1;
        } else if i + 1 < bytes.len() && bytes[i + 1] == b'%' {
            out.push_str("%%");
            i += 2;
        } else {
            i = parse_format_specifier(bytes, i, args, &mut out, &mut reordered, &mut seq, &mut has_positional)?;
        }
    }

    if has_positional { Ok((out, reordered)) } else { Ok((out, args.to_vec())) }
}

/// Parse one format specifier starting at `%`, appending to `out` and collecting args.
/// Returns the index after the specifier.
fn parse_format_specifier(
    bytes: &[u8],
    start: usize,
    args: &[Value],
    out: &mut String,
    reordered: &mut Vec<Value>,
    seq: &mut usize,
    has_positional: &mut bool,
) -> mlua::Result<usize> {
    let mut i = start + 1; // skip the '%'

    // Check for positional %N$
    if let Some((n, after)) = parse_positional_index(bytes, i) {
        if n >= 100 {
            return Err(mlua::Error::RuntimeError(
                "invalid format (width or precision too long)".to_string(),
            ));
        }
        *has_positional = true;
        reordered.push(args.get(n - 1).cloned().unwrap_or(Value::Nil));
        out.push('%');
        i = after;
    } else {
        *seq += 1;
        reordered.push(args.get(*seq - 1).cloned().unwrap_or(Value::Nil));
        out.push('%');
    }

    i = skip_flags_width_precision(bytes, i, out);
    // Conversion character — %F → %f
    if i < bytes.len() && is_format_conversion(bytes[i]) {
        out.push(if bytes[i] == b'F' { 'f' } else { bytes[i] as char });
        i += 1;
    }
    Ok(i)
}

/// Skip flags (`-+ #0`), width digits, and precision (`.N`) — appending to `out`.
fn skip_flags_width_precision(bytes: &[u8], start: usize, out: &mut String) -> usize {
    let mut i = start;
    while i < bytes.len() && matches!(bytes[i], b'-' | b'+' | b' ' | b'#' | b'0') {
        out.push(bytes[i] as char);
        i += 1;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        out.push(bytes[i] as char);
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        out.push('.');
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    i
}

/// Try to parse `N$` (digits followed by `$`) at `start`. Returns `(N, index_after_$)`.
fn parse_positional_index(bytes: &[u8], start: usize) -> Option<(usize, usize)> {
    let mut i = start;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == start || i >= bytes.len() || bytes[i] != b'$' {
        return None;
    }
    let n: usize = std::str::from_utf8(&bytes[start..i]).ok()?.parse().ok()?;
    Some((n, i + 1))
}

fn is_format_conversion(b: u8) -> bool {
    matches!(
        b,
        b'd' | b'i' | b'o' | b'u' | b'x' | b'X'
            | b'e' | b'E' | b'f' | b'F'
            | b'g' | b'G' | b'a' | b'A'
            | b'c' | b's' | b'p' | b'q' | b'n'
    )
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
    // Set a built-in pseudo-addon as owner for pre-created frames.
    set_builtin_addon_owner(state);
    register_global_frames(lua, Rc::clone(state))?;
    register_tooltip_frames(lua, Rc::clone(state))?;
    register_quest_frames(lua, Rc::clone(state))?;
    // Ensure all named frames have _G entries. Covers frames created by
    // builtin_frames.rs (no Lua access) and any registration site that
    // only sets the widget registry name without calling raw_set on _G.
    // Admin API for simulator state control from Lua.
    super::globals::admin_api::register_admin_api(lua, Rc::clone(state))?;
    state.borrow_mut().loading_addon_index = None;

    sync_named_frames_to_globals(lua, state)
}

/// Set loading_addon_index to the existing `__BuiltIn` pseudo-addon.
fn set_builtin_addon_owner(state: &Rc<RefCell<SimState>>) {
    let mut s = state.borrow_mut();
    let idx = s.addons.iter().position(|a| a.folder_name == "__BuiltIn")
        .expect("__BuiltIn addon must be registered by init_builtin_frames");
    s.loading_addon_index = Some(idx as u16);
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

