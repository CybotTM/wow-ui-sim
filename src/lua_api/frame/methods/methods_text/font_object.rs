//! SetFontObject, GetFontObject, SetFontObjectsToTry, GetFontObjectForAlphabet, GetNumLines.

use crate::lua_api::frame::handle::{FrameRef, frame_ref, get_sim_state};
use crate::lua_api::simple_html::TextStyle;
use crate::widget::WidgetType;
use mlua::{Lua, Value};

use super::{is_simple_html, is_text_type};

/// SetFontObject, GetFontObject.
pub(super) fn add_font_object_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFontObject", |lua, this, args: mlua::MultiValue| {
        set_font_object_impl(lua, this.0, args)
    });
    methods.add_method("GetFontObject", |lua, this, args: mlua::MultiValue| {
        get_font_object_impl(lua, this.0, args)
    });
}

/// SetFontObject implementation.
fn set_font_object_impl(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    let is_html = is_simple_html(lua, id);

    if is_html
        && args_vec.len() >= 2
        && let Some(Value::String(s)) = args_vec.first()
    {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            return set_font_object_for_text_type(lua, id, &type_str, &args_vec);
        }
    }

    let font_object = args_vec.into_iter().next().unwrap_or(Value::Nil);
    if font_object.is_nil() {
        return Err(mlua::Error::runtime(
            "Usage: SetFontObject(fontObject or \"fontName\")",
        ));
    }
    // Hot path: same Font table/name already stored. AuraButton:OnUpdate calls
    // `self.Duration:SetFontObject(SMALLER_AURA_DURATION_FONT)` every tick
    // with a constant global Font — skip the 12-field table copy + store.
    let store = get_or_create_font_object_store(lua)?;
    if let Ok(existing) = store.get::<Value>(id)
        && font_object_values_equal(&existing, &font_object)
    {
        return Ok(());
    }
    let font_table = resolve_font_table(lua, &font_object);
    apply_font_table_to_frame(lua, id, font_table.as_ref());
    store.set(id, font_object)?;
    Ok(())
}

/// Whether two Font values reference the same Lua object (same table or same
/// string). Used by the SetFontObject fast-path to detect redundant calls.
fn font_object_values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Table(at), Value::Table(bt)) => at == bt,
        (Value::String(a), Value::String(b)) => a
            .to_str()
            .ok()
            .zip(b.to_str().ok())
            .is_some_and(|(a, b)| *a == *b),
        _ => false,
    }
}

/// Return the `_G.__fontstring_font_objects` table, creating it on first access.
///
/// Replaces the per-call `lua.load("...").eval()` which compiled a Lua chunk
/// every time — around 30µs per SetFontObject/GetFontObject call. Hot because
/// `AuraButtonMixin:OnUpdate` calls `self.Duration:SetFontObject(...)` each
/// frame. The table is cached in the Lua registry so repeated lookups are
/// O(1) after the first.
fn get_or_create_font_object_store(lua: &Lua) -> mlua::Result<mlua::Table> {
    if let Ok(t) = lua.named_registry_value::<mlua::Table>("__fontstring_font_objects_cache") {
        return Ok(t);
    }
    let globals = lua.globals();
    let table = match globals.get::<Value>("__fontstring_font_objects")? {
        Value::Table(t) => t,
        _ => {
            let t = lua.create_table()?;
            globals.set("__fontstring_font_objects", t.clone())?;
            t
        }
    };
    lua.set_named_registry_value("__fontstring_font_objects_cache", table.clone())?;
    Ok(table)
}

/// GetFontObject implementation.
fn get_font_object_impl(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<Value> {
    let args_vec: Vec<Value> = args.into_iter().collect();

    if let Some(Value::String(s)) = args_vec.first() {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            return get_font_object_for_type(lua, id, &type_str);
        }
    }

    if needs_auto_font_object(lua, id) {
        return get_or_create_auto_font(lua, id);
    }

    let store = get_or_create_font_object_store(lua)?;
    let font: Value = store.get(id)?;
    Ok(font)
}

pub(super) fn get_frame_font_object(lua: &Lua, id: u64) -> mlua::Result<Option<mlua::Table>> {
    let font_object = get_font_object_impl(lua, id, mlua::MultiValue::new())?;
    Ok(resolve_font_table(lua, &font_object))
}

/// Get the stored font object for a SimpleHTML text type.
fn get_font_object_for_type(lua: &Lua, id: u64, type_str: &str) -> mlua::Result<Value> {
    let store = get_or_create_font_object_store(lua)?;
    let key = format!("{}_{}", id, type_str);
    let font: Value = store.get(key)?;
    Ok(font)
}

/// GetFontObjectForAlphabet, SetFontObjectsToTry, GetNumLines.
pub(super) fn add_font_object_extra_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "GetFontObjectForAlphabet",
        |lua, this, _alphabet: Option<String>| frame_ref(lua, this.0),
    );
    methods.add_method(
        "SetFontObjectsToTry",
        |lua, this, args: mlua::MultiValue| {
            if let Some(first) = args.into_iter().next() {
                let font_table = resolve_font_table(lua, &first);
                apply_font_table_to_frame(lua, this.0, font_table.as_ref());
            }
            Ok(())
        },
    );
    methods.add_method("GetNumLines", |_, _this, ()| Ok(1_i32));
}

/// Handle SetFontObject for a SimpleHTML per-textType call.
fn set_font_object_for_text_type(
    lua: &Lua,
    id: u64,
    type_str: &str,
    args_vec: &[Value],
) -> mlua::Result<()> {
    let font_name = match args_vec.get(1) {
        Some(Value::String(n)) => Some(n.to_string_lossy().to_string()),
        Some(Value::Table(t)) => t.get::<Option<String>>("__fontPath").ok().flatten(),
        _ => None,
    };
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(data) = state.simple_htmls.get_mut(&id) {
        let style = data
            .text_styles
            .entry(type_str.to_string())
            .or_insert_with(TextStyle::default);
        style.font_object = font_name;
    }
    drop(state);
    let store = get_or_create_font_object_store(lua)?;
    let key = format!("{}_{}", id, type_str);
    if let Some(fo) = args_vec.get(1).cloned() {
        store.set(key, fo)?;
    }
    Ok(())
}

/// Check if a frame ID corresponds to a type that auto-creates a Font object.
fn needs_auto_font_object(lua: &Lua, id: u64) -> bool {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state.widgets.get(id).is_some_and(|f| {
        matches!(
            f.widget_type,
            WidgetType::MessageFrame | WidgetType::EditBox
        )
    })
}

/// Get or create the auto-created Font object for a MessageFrame or EditBox.
fn get_or_create_auto_font(lua: &Lua, id: u64) -> mlua::Result<Value> {
    let store = get_or_create_auto_font_store(lua)?;
    let existing: Value = store.get(id)?;
    if !existing.is_nil() {
        return Ok(existing);
    }
    let font = crate::lua_api::globals::font_api::create_bare_font(lua)?;
    store.set(id, font.clone())?;
    Ok(Value::Table(font))
}

/// Return the `_G.__auto_fonts` table, creating it on first access.
/// Cached in the Lua registry so subsequent lookups skip the chunk compile.
fn get_or_create_auto_font_store(lua: &Lua) -> mlua::Result<mlua::Table> {
    if let Ok(t) = lua.named_registry_value::<mlua::Table>("__auto_fonts_cache") {
        return Ok(t);
    }
    let globals = lua.globals();
    let table = match globals.get::<Value>("__auto_fonts")? {
        Value::Table(t) => t,
        _ => {
            let t = lua.create_table()?;
            globals.set("__auto_fonts", t.clone())?;
            t
        }
    };
    lua.set_named_registry_value("__auto_fonts_cache", table.clone())?;
    Ok(table)
}

/// Resolve a font object Value (table or name string) into an optional Table.
pub(super) fn resolve_font_table(lua: &Lua, font_object: &Value) -> Option<mlua::Table> {
    match font_object {
        Value::Table(t) => Some(t.clone()),
        Value::String(name) => {
            let name_str = name.to_string_lossy().to_string();
            lua.globals()
                .get::<Option<mlua::Table>>(name_str)
                .ok()
                .flatten()
        }
        _ => None,
    }
}

/// Apply font properties from a Lua font table to the Rust frame.
pub(super) fn apply_font_table_to_frame(lua: &Lua, id: u64, font_table: Option<&mlua::Table>) {
    let Some(src) = font_table else { return };
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let Some(frame) = state.widgets.get_mut_visual(id) else {
        return;
    };
    apply_font_table_paths(src, frame);
    apply_font_table_colors(src, frame);
}

/// Apply font path/size/outline from a font table to a frame.
fn apply_font_table_paths(src: &mlua::Table, frame: &mut crate::widget::Frame) {
    if let Ok(path) = src
        .get::<String>("__fontPath")
        .or_else(|_| src.get::<String>("__font"))
    {
        frame.font = Some(path);
    }
    if let Ok(height) = src
        .get::<f64>("__fontHeight")
        .or_else(|_| src.get::<f64>("__height"))
    {
        frame.font_size = height as f32;
    }
    if let Ok(flags) = src
        .get::<String>("__fontFlags")
        .or_else(|_| src.get::<String>("__outline"))
    {
        frame.font_outline = crate::widget::TextOutline::from_wow_str(&flags);
    }
}

/// Apply color and alignment properties from a font table to a frame.
fn apply_font_table_colors(src: &mlua::Table, frame: &mut crate::widget::Frame) {
    if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
        src.get::<f64>("__textColorR"),
        src.get::<f64>("__textColorG"),
        src.get::<f64>("__textColorB"),
        src.get::<f64>("__textColorA"),
    ) {
        frame.text_color = crate::widget::Color::new(r as f32, g as f32, b as f32, a as f32);
    }
    apply_font_table_shadow(src, frame);
    apply_font_table_justify(src, frame);
}

/// Apply shadow color and offset from a font table to a frame.
fn apply_font_table_shadow(src: &mlua::Table, frame: &mut crate::widget::Frame) {
    if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
        src.get::<f64>("__shadowColorR"),
        src.get::<f64>("__shadowColorG"),
        src.get::<f64>("__shadowColorB"),
        src.get::<f64>("__shadowColorA"),
    ) {
        frame.shadow_color = crate::widget::Color::new(r as f32, g as f32, b as f32, a as f32);
    }
    if let (Ok(x), Ok(y)) = (
        src.get::<f64>("__shadowOffsetX"),
        src.get::<f64>("__shadowOffsetY"),
    ) {
        frame.shadow_offset = (x as f32, y as f32);
    }
}

/// Apply text justification from a font table to a frame.
fn apply_font_table_justify(src: &mlua::Table, frame: &mut crate::widget::Frame) {
    if let Ok(h) = src.get::<String>("__justifyH") {
        frame.justify_h = crate::widget::TextJustify::from_wow_str(&h);
    }
    if let Ok(v) = src.get::<String>("__justifyV") {
        frame.justify_v = crate::widget::TextJustify::from_wow_str(&v);
    }
}
