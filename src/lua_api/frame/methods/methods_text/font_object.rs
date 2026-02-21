//! SetFontObject, GetFontObject, SetFontObjectsToTry, GetFontObjectForAlphabet, GetNumLines.

use crate::lua_api::frame::handle::{frame_lud, get_sim_state, lud_to_id};
use crate::lua_api::simple_html::TextStyle;
use crate::widget::WidgetType;
use mlua::{LightUserData, Lua, Value};

use super::{is_simple_html, is_text_type};

/// SetFontObject, GetFontObject.
pub(super) fn add_font_object_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    // SetFontObject([textType,] fontObject or fontName) - copy font properties from a font object
    methods.set("SetFontObject", lua.create_function(|lua, (ud, args): (LightUserData, mlua::MultiValue)| {
        let id = lud_to_id(ud);
        let args_vec: Vec<Value> = args.into_iter().collect();
        let is_html = is_simple_html(lua, id);

        // Check for SimpleHTML per-textType call
        if is_html && args_vec.len() >= 2
            && let Some(Value::String(s)) = args_vec.first() {
                let type_str = s.to_string_lossy().to_string();
                if is_text_type(&type_str) {
                    return set_font_object_for_text_type(lua, id, &type_str, &args_vec);
                }
            }

        // Standard path
        let font_object = args_vec.into_iter().next().unwrap_or(Value::Nil);
        let font_table = resolve_font_table(lua, &font_object);
        apply_font_table_to_frame(lua, id, font_table.as_ref());

        let store: mlua::Table = lua
            .load(
                "_G.__fontstring_font_objects = _G.__fontstring_font_objects or {}; return _G.__fontstring_font_objects",
            )
            .eval()?;
        store.set(id, font_object)?;

        Ok(())
    })?)?;

    // GetFontObject([textType]) - return the font object set via SetFontObject
    // MessageFrame auto-creates a sticky Font object on first call.
    methods.set("GetFontObject", lua.create_function(|lua, (ud, args): (LightUserData, mlua::MultiValue)| {
        let id = lud_to_id(ud);
        let args_vec: Vec<Value> = args.into_iter().collect();

        if let Some(Value::String(s)) = args_vec.first() {
            let type_str = s.to_string_lossy().to_string();
            if is_text_type(&type_str) {
                let store: mlua::Table =
                    lua.load("return _G.__fontstring_font_objects or {}").eval()?;
                let key = format!("{}_{}", id, type_str);
                let font: Value = store.get(key)?;
                return Ok(font);
            }
        }

        // MessageFrame: auto-create and cache a bare Font object
        if is_message_frame(lua, id) {
            return get_or_create_messageframe_font(lua, id);
        }

        let store: mlua::Table =
            lua.load("return _G.__fontstring_font_objects or {}").eval()?;
        let font: Value = store.get(id)?;
        Ok(font)
    })?)?;

    Ok(())
}

/// GetFontObjectForAlphabet, SetFontObjectsToTry, GetNumLines.
pub(super) fn add_font_object_extra_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    // GetFontObjectForAlphabet(alphabet) - returns self for font localization
    methods.set("GetFontObjectForAlphabet", lua.create_function(
        |_lua, (ud, _alphabet): (LightUserData, Option<String>)| {
            let id = lud_to_id(ud);
            Ok(frame_lud(id))
        },
    )?)?;

    // SetFontObjectsToTry(fontObject1, fontObject2, ...) - set fallback font objects
    methods.set("SetFontObjectsToTry", lua.create_function(
        |lua, (ud, args): (LightUserData, mlua::MultiValue)| {
            let id = lud_to_id(ud);
            if let Some(first) = args.into_iter().next() {
                let font_table = resolve_font_table(lua, &first);
                apply_font_table_to_frame(lua, id, font_table.as_ref());
            }
            Ok(())
        },
    )?)?;

    // GetNumLines() - return number of visible text lines
    methods.set("GetNumLines", lua.create_function(
        |_lua, _ud: LightUserData| Ok(1_i32),
    )?)?;

    Ok(())
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
        let style = data.text_styles.entry(type_str.to_string()).or_insert_with(TextStyle::default);
        style.font_object = font_name;
    }
    drop(state);
    let store: mlua::Table = lua
        .load("_G.__fontstring_font_objects = _G.__fontstring_font_objects or {}; return _G.__fontstring_font_objects")
        .eval()?;
    let key = format!("{}_{}", id, type_str);
    if let Some(fo) = args_vec.get(1).cloned() {
        store.set(key, fo)?;
    }
    Ok(())
}

/// Check if a frame ID corresponds to a MessageFrame widget.
fn is_message_frame(lua: &Lua, id: u64) -> bool {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .widgets
        .get(id)
        .is_some_and(|f| f.widget_type == WidgetType::MessageFrame)
}

/// Get or create the auto-created Font object for a MessageFrame.
fn get_or_create_messageframe_font(lua: &Lua, id: u64) -> mlua::Result<Value> {
    let store: mlua::Table = lua
        .load("_G.__msgframe_fonts = _G.__msgframe_fonts or {}; return _G.__msgframe_fonts")
        .eval()?;
    let existing: Value = store.get(id)?;
    if !existing.is_nil() {
        return Ok(existing);
    }
    let font = crate::lua_api::globals::font_api::create_bare_font(lua)?;
    store.set(id, font.clone())?;
    Ok(Value::Table(font))
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
///
/// Supports two naming conventions:
/// - XML Font objects: `__font`, `__height`, `__outline`
/// - Lua-created font objects: `__fontPath`, `__fontHeight`, `__fontFlags`
pub(super) fn apply_font_table_to_frame(lua: &Lua, id: u64, font_table: Option<&mlua::Table>) {
    let Some(src) = font_table else { return };
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let Some(frame) = state.widgets.get_mut_visual(id) else { return };

    if let Ok(path) = src.get::<String>("__fontPath").or_else(|_| src.get::<String>("__font")) {
        frame.font = Some(path);
    }
    if let Ok(height) = src.get::<f64>("__fontHeight").or_else(|_| src.get::<f64>("__height")) {
        frame.font_size = height as f32;
    }
    if let Ok(flags) = src.get::<String>("__fontFlags").or_else(|_| src.get::<String>("__outline")) {
        frame.font_outline = crate::widget::TextOutline::from_wow_str(&flags);
    }
    apply_font_table_colors(src, frame);
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
    if let Ok(h) = src.get::<String>("__justifyH") {
        frame.justify_h = crate::widget::TextJustify::from_wow_str(&h);
    }
    if let Ok(v) = src.get::<String>("__justifyV") {
        frame.justify_v = crate::widget::TextJustify::from_wow_str(&v);
    }
}
