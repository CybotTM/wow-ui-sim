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
    register_custom_next(lua)?;
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
        eprintln!("{output}");
        state.borrow_mut().console_output.push(output);
        Ok(())
    })?;
    lua.globals().set("print", print_func.clone())?;
    // Store in registry so A_Print bypasses taint
    lua.set_named_registry_value("__sim_print", print_func)?;
    lua.load(r#"
        function A_Print(...)
            local p = debug.getregistry().__sim_print
            if p then p(...) end
        end
    "#).exec()
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

/// Override `next` to support FrameRef UserData.
///
/// In WoW, frames are tables with a C userdata at key `[0]`. Our frames are pure
/// userdata, so `next(frame)` must return `(0, raw_userdata)` then nil.
fn register_custom_next(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let original_next: mlua::Function = globals.get("next")?;
    lua.set_named_registry_value("__original_next", original_next)?;

    let custom_next = lua.create_function(|lua, (tbl, key): (Value, Value)| {
        // If called on a FrameRef userdata, simulate table with [0]=userdata
        if let Value::UserData(ud) = &tbl {
            if ud.borrow::<super::frame::FrameRef>().is_ok() {
                return match key {
                    Value::Nil => {
                        // First iteration: return (0, lightuserdata)
                        // In WoW, frames are tables with [0]=C_userdata (no metatable).
                        // Use LightUserData so getmetatable returns nil.
                        Ok(mlua::MultiValue::from_vec(vec![
                            Value::Integer(0),
                            Value::LightUserData(mlua::LightUserData(std::ptr::null_mut())),
                        ]))
                    }
                    _ => {
                        // After key 0: no more entries
                        Ok(mlua::MultiValue::new())
                    }
                };
            }
        }
        let original: mlua::Function = lua.named_registry_value("__original_next")?;
        original.call((tbl, key))
    })?;

    globals.set("next", custom_next)
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
            return resolve_frame_metatable(lua, frame_id);
        }
        let real_getmetatable: mlua::Function = lua.named_registry_value("__real_getmetatable")?;
        real_getmetatable.call(value)
    })?;

    let real_getmetatable: mlua::Function = globals.get("getmetatable")?;
    lua.set_named_registry_value("__real_getmetatable", real_getmetatable)?;
    globals.set("getmetatable", custom_getmetatable)
}

/// Resolve the metatable for a frame, handling aliased types.
///
/// For aliased types (e.g. ArchaeologyDigSiteFrame → Frame), creates and caches
/// a unique cloned metatable so per-type identity checks pass.
/// Animation/Actor/ControlPoint types get a restricted method set (not Frame's).
fn resolve_frame_metatable(lua: &Lua, frame_id: u64) -> Result<Value> {
    let (widget_type, obj_type_name) = {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let f = state.widgets.get(frame_id);
        (
            f.map(|f| f.widget_type).unwrap_or(crate::widget::WidgetType::Frame),
            f.and_then(|f| f.object_type_name.clone()),
        )
    };
    let per_type: mlua::Table = lua.named_registry_value("__per_type_metatables")?;
    let type_key = obj_type_name.as_deref().unwrap_or(widget_type.as_str());
    let mt: Value = per_type.raw_get(type_key)?;
    if mt != Value::Nil {
        return Ok(mt);
    }
    // Animation/Actor/ControlPoint types get a restricted metatable.
    if let Some(otn) = obj_type_name.as_deref() {
        if super::frame::methods::methods_core::is_anim_type(otn) {
            let new_mt = build_anim_metatable(lua, &per_type, otn)?;
            per_type.raw_set(type_key, new_mt.clone())?;
            return Ok(Value::Table(new_mt));
        }
    }
    // Clone the base type's metatable into a new unique table for this alias.
    let new_mt = clone_metatable(lua, &per_type, widget_type.as_str())?;
    per_type.raw_set(type_key, new_mt.clone())?;
    Ok(Value::Table(new_mt))
}

/// Clone a base type's metatable, wrapping each method for distinct per-type function identity.
/// Uses raw C closures (via cfunc_wrap) to avoid exhausting mlua's auxiliary stack limit.
fn clone_metatable(lua: &Lua, per_type: &mlua::Table, base_key: &str) -> Result<mlua::Table> {
    let base_mt: mlua::Table = per_type.raw_get(base_key)?;
    let base_idx: mlua::Table = base_mt.raw_get("__index")?;
    let (new_idx, wrap_fn) = (lua.create_table()?, super::cfunc_wrap::create_wrap_factory(lua)?);
    for pair in base_idx.pairs::<String, mlua::Function>() {
        let (k, f) = pair?;
        new_idx.raw_set(k.as_str(), wrap_fn.call::<mlua::Function>(f)?)?;
    }
    let new_mt = lua.create_table()?;
    new_mt.raw_set("__index", new_idx)?;
    Ok(new_mt)
}

/// Common methods for all animation/actor/controlpoint types (UIObject-level).
const ANIM_COMMON_META: &[&str] = &[
    "GetObjectType",
    "IsObjectType",
    "GetName",
    "GetDebugName",
    "GetParent",
    "SetParent",
    "IsForbidden",
    "IsProtected",
    "CanChangeProtectedState",
    "IsObjectLoaded",
    "GetSourceLocation",
];

/// Script methods for AnimationGroup and Animation types.
const ANIM_SCRIPT_META: &[&str] = &[
    "SetScript",
    "GetScript",
    "HasScript",
    "HookScript",
    "ClearScripts",
];

/// Build a restricted metatable for animation/actor/controlpoint types.
///
/// Copies only animation-appropriate methods from Frame's metatable, wrapping
/// each in a unique closure for per-type function identity (cfuncs test).
fn build_anim_metatable(lua: &Lua, per_type: &mlua::Table, otn: &str) -> Result<mlua::Table> {
    let frame_mt: mlua::Table = per_type.raw_get("Frame")?;
    let frame_idx: mlua::Table = frame_mt.raw_get("__index")?;
    let index_table = lua.create_table()?;

    // Common UIObject methods
    for &name in ANIM_COMMON_META {
        if let Value::Function(f) = frame_idx.raw_get::<Value>(name)? {
            index_table.set(name, wrap_method(lua, f)?)?;
        }
    }
    // Script methods for AnimationGroup and Animation subtypes (not ControlPoint/Actor)
    if otn != "ControlPoint" && otn != "Actor" && otn != "ModelSceneActor" {
        for &name in ANIM_SCRIPT_META {
            if let Value::Function(f) = frame_idx.raw_get::<Value>(name)? {
                index_table.set(name, wrap_method(lua, f)?)?;
            }
        }
    }

    let mt = lua.create_table()?;
    mt.set("__index", index_table)?;
    Ok(mt)
}

/// Pre-build one metatable per widget type and store in `__per_type_metatables`.
/// Methods resolved by probing a dummy FrameRef; same type → same table identity.
fn build_per_type_metatables(lua: &Lua) -> Result<()> {
    use crate::lua_api::frame::FrameRef;

    let dummy = lua.create_userdata(FrameRef(0))?;
    // The patched __index checks debug.getfenv(ud)[1] — set up a valid fenv.
    let fenv = lua.create_table()?;
    fenv.raw_set(1, lua.create_table()?)?;
    dummy.set_user_value(fenv)?;
    let per_type = lua.create_table()?;
    let mut resolved: std::collections::HashMap<String, Value> = std::collections::HashMap::new();

    for widget_type in all_widget_types() {
        let type_key = widget_type.as_str();
        if per_type.raw_get::<Value>(type_key)? != Value::Nil {
            continue;
        }
        let mt = build_metatable_for_type(lua, widget_type, &dummy, &mut resolved)?;
        per_type.raw_set(type_key, mt)?;
    }

    lua.set_named_registry_value("__per_type_metatables", per_type)?;
    Ok(())
}

/// Build `{ __index = { methods } }` for one widget type by probing the dummy FrameRef.
///
/// Each function is wrapped in a unique closure so the cfuncs identity checker sees
/// distinct function objects per type (matching real WoW's per-type C wrappers).
fn build_metatable_for_type(
    lua: &Lua,
    widget_type: crate::widget::WidgetType,
    dummy: &mlua::AnyUserData,
    resolved: &mut std::collections::HashMap<String, Value>,
) -> Result<mlua::Table> {
    use crate::lua_api::frame::method_registry;

    let index_table = lua.create_table()?;
    let type_methods = method_registry::methods_for_type(widget_type);
    let allowed = method_registry::global::GLOBAL_METHODS
        .iter()
        .chain(type_methods.iter());

    for &name in allowed {
        if let Some(f) = resolve_method(lua, dummy, name, resolved)? {
            index_table.set(name, wrap_method(lua, f)?)?;
        }
    }

    let mt = lua.create_table()?;
    mt.set("__index", index_table)?;
    Ok(mt)
}

/// Resolve a method name on the dummy FrameRef, caching the result.
///
/// Uses `ud[name]` in Lua (triggering mlua's internal __index) since mlua doesn't
/// expose registered `add_method` entries via `AnyUserData::get`.
fn resolve_method(
    lua: &Lua,
    dummy: &mlua::AnyUserData,
    name: &str,
    cache: &mut std::collections::HashMap<String, Value>,
) -> Result<Option<mlua::Function>> {
    let val = if let Some(v) = cache.get(name) {
        v.clone()
    } else {
        let v: Value = lua
            .load("local ud, k = ...; return ud[k]")
            .call((dummy.clone(), name))?;
        cache.insert(name.to_owned(), v.clone());
        v
    };
    match val {
        Value::Function(f) => Ok(Some(f)),
        _ => Ok(None),
    }
}

/// Wrap a method function in a unique closure for per-type identity.
fn wrap_method(lua: &Lua, f: mlua::Function) -> Result<mlua::Function> {
    Ok(lua.create_function(move |_, args: mlua::MultiValue| {
        f.call::<mlua::MultiValue>(args)
    })?)
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
    register_c_collection_api(lua, Rc::clone(state))?;
    register_c_item_api(lua, Rc::clone(state))?;
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

