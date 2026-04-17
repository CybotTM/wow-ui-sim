//! Font API — CreateFont, GetFonts, GetFontInfo, CreateFontFamily,
//! register_standard_font_objects, and the per-font-object method table.

use crate::lua_api::methods::{
    create_string, create_string_static, create_table, table_get, table_set, val_to_string,
};
use crate::lua_bridge::table_set_rust_fn;
use crate::lua_bridge::{FromStack, IntoStack, stack_val};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table as RiluaTable;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

use super::set_global_val;

type FontTableRef = GcRef<RiluaTable>;

// ── Data ─────────────────────────────────────────────────────────────────────

/// (name, height, flags, r, g, b)
const STANDARD_FONTS: &[(&str, f64, &str, f64, f64, f64)] = &[
    ("GameFontNormal", 12.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalSmall", 10.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalLarge", 16.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalHuge", 20.0, "", 1.0, 0.82, 0.0),
    ("GameFontHighlight", 12.0, "", 1.0, 1.0, 1.0),
    ("GameFontHighlightSmall", 10.0, "", 1.0, 1.0, 1.0),
    (
        "GameFontHighlightSmallOutline",
        10.0,
        "OUTLINE",
        1.0,
        1.0,
        1.0,
    ),
    ("GameFontHighlightLarge", 16.0, "", 1.0, 1.0, 1.0),
    ("GameFontHighlightHuge", 20.0, "", 1.0, 1.0, 1.0),
    ("GameFontHighlightOutline", 12.0, "OUTLINE", 1.0, 1.0, 1.0),
    ("GameFontDisable", 12.0, "", 0.5, 0.5, 0.5),
    ("GameFontDisableSmall", 10.0, "", 0.5, 0.5, 0.5),
    ("GameFontDisableLarge", 16.0, "", 0.5, 0.5, 0.5),
    ("GameFontRed", 12.0, "", 1.0, 0.1, 0.1),
    ("GameFontRedSmall", 10.0, "", 1.0, 0.1, 0.1),
    ("GameFontRedLarge", 16.0, "", 1.0, 0.1, 0.1),
    ("GameFontGreen", 12.0, "", 0.1, 1.0, 0.1),
    ("GameFontGreenSmall", 10.0, "", 0.1, 1.0, 0.1),
    ("GameFontGreenLarge", 16.0, "", 0.1, 1.0, 0.1),
    ("GameFontWhite", 12.0, "", 1.0, 1.0, 1.0),
    ("GameFontWhiteSmall", 10.0, "", 1.0, 1.0, 1.0),
    ("GameFontWhiteTiny", 9.0, "", 1.0, 1.0, 1.0),
    ("GameFontBlack", 12.0, "", 0.0, 0.0, 0.0),
    ("GameFontBlackSmall", 10.0, "", 0.0, 0.0, 0.0),
    ("NumberFontNormal", 14.0, "OUTLINE", 1.0, 1.0, 1.0),
    ("NumberFontNormalSmall", 12.0, "OUTLINE", 1.0, 1.0, 1.0),
    ("NumberFontNormalLarge", 16.0, "OUTLINE", 1.0, 1.0, 1.0),
    ("NumberFontNormalHuge", 24.0, "OUTLINE", 1.0, 1.0, 1.0),
    ("NumberFontNormalRightRed", 14.0, "OUTLINE", 1.0, 0.1, 0.1),
    (
        "NumberFontNormalRightYellow",
        14.0,
        "OUTLINE",
        1.0,
        1.0,
        0.0,
    ),
    ("ChatFontNormal", 14.0, "", 1.0, 1.0, 1.0),
    ("ChatFontSmall", 12.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Small", 10.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Med1", 12.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Med2", 13.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Med3", 14.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Large", 16.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Huge1", 20.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Huge2", 24.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Outline", 12.0, "OUTLINE", 1.0, 1.0, 1.0),
    (
        "SystemFont_OutlineThick_Huge2",
        24.0,
        "OUTLINE, THICKOUTLINE",
        1.0,
        1.0,
        1.0,
    ),
    (
        "SystemFont_OutlineThick_Huge4",
        32.0,
        "OUTLINE, THICKOUTLINE",
        1.0,
        1.0,
        1.0,
    ),
    (
        "SystemFont_OutlineThick_WTF",
        64.0,
        "OUTLINE, THICKOUTLINE",
        1.0,
        1.0,
        1.0,
    ),
    ("SystemFont_Shadow_Small", 10.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Shadow_Med1", 12.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Shadow_Med2", 13.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Shadow_Med3", 14.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Shadow_Large", 16.0, "", 1.0, 1.0, 1.0),
    (
        "SystemFont_Shadow_Large_Outline",
        16.0,
        "OUTLINE",
        1.0,
        1.0,
        1.0,
    ),
    ("SystemFont_Shadow_Huge1", 20.0, "", 1.0, 1.0, 1.0),
    ("GameTooltipHeader", 14.0, "", 1.0, 1.0, 1.0),
    ("GameTooltipText", 12.0, "", 1.0, 1.0, 1.0),
    ("GameTooltipTextSmall", 10.0, "", 1.0, 1.0, 1.0),
    ("SubZoneTextFont", 26.0, "OUTLINE", 1.0, 0.82, 0.0),
    ("PVPInfoTextFont", 20.0, "OUTLINE", 1.0, 0.1, 0.1),
    ("FriendsFont_Normal", 12.0, "", 1.0, 1.0, 1.0),
    ("FriendsFont_Small", 10.0, "", 1.0, 1.0, 1.0),
    ("FriendsFont_Large", 14.0, "", 1.0, 1.0, 1.0),
    ("FriendsFont_UserText", 11.0, "", 1.0, 1.0, 1.0),
];

// ── Low-level field helpers ───────────────────────────────────────────────────

pub(super) fn font_f64(state: &mut LuaState, font: Val, key: &str) -> f64 {
    match table_get(state, font, key) {
        Val::Num(n) => n,
        _ => 0.0,
    }
}

pub(super) fn font_str(state: &mut LuaState, font: Val, key: &str) -> String {
    let value = table_get(state, font, key);
    val_to_string(state, value).unwrap_or_default()
}

pub(super) fn font_set_defaults(state: &mut LuaState, font: Val, name: Option<&str>) {
    table_set(state, font, "__fontHeight", Val::Num(0.0));
    let empty = create_string(state, "");
    table_set(state, font, "__fontFlags", empty);
    table_set(state, font, "__textColorR", Val::Num(1.0));
    table_set(state, font, "__textColorG", Val::Num(1.0));
    table_set(state, font, "__textColorB", Val::Num(1.0));
    table_set(state, font, "__textColorA", Val::Num(1.0));
    table_set(state, font, "__shadowColorR", Val::Num(0.0));
    table_set(state, font, "__shadowColorG", Val::Num(0.0));
    table_set(state, font, "__shadowColorB", Val::Num(0.0));
    table_set(state, font, "__shadowColorA", Val::Num(0.0));
    table_set(state, font, "__shadowOffsetX", Val::Num(0.0));
    table_set(state, font, "__shadowOffsetY", Val::Num(0.0));
    let center = create_string_static(state, "CENTER");
    table_set(state, font, "__justifyH", center);
    let middle = create_string_static(state, "MIDDLE");
    table_set(state, font, "__justifyV", middle);
    let name_val = match name {
        Some(n) => create_string(state, n),
        None => Val::Nil,
    };
    table_set(state, font, "__name", name_val);
}

// ── Method group registrations ────────────────────────────────────────────────

fn add_font_field_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn(state, font_ref, "SetFontHeight", font_set_font_height)?;
    table_set_rust_fn(state, font_ref, "GetFontHeight", font_get_font_height)?;
    table_set_rust_fn(state, font_ref, "SetFont", font_set_font)?;
    table_set_rust_fn(state, font_ref, "GetFont", font_get_font)?;
    Ok(())
}

fn font_set_font_height(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let height = f64::from_stack(state, 2)?;
    table_set(state, font, "__fontHeight", Val::Num(height));
    Ok(0)
}

fn font_get_font_height(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    font_f64(state, font, "__fontHeight").into_stack(state)
}

fn font_set_font(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let path = Option::<String>::from_stack(state, 2)?;
    let height = Option::<f64>::from_stack(state, 3)?;
    let flags = Option::<String>::from_stack(state, 4)?;
    let Some(path) = path else { return Ok(0) };
    let path_val = create_string(state, &path);
    table_set(state, font, "__fontPath", path_val);
    if let Some(h) = height {
        table_set(state, font, "__fontHeight", Val::Num(h));
    }
    let flags_val = create_string(state, flags.as_deref().unwrap_or(""));
    table_set(state, font, "__fontFlags", flags_val);
    Ok(0)
}

fn font_get_font(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let path = table_get(state, font, "__fontPath");
    let path_val = match path {
        Val::Str(_) => path,
        _ => Val::Nil,
    };
    let height = font_f64(state, font, "__fontHeight");
    let flags = font_str(state, font, "__fontFlags");
    let flags_val = create_string(state, &flags);
    state.push(path_val);
    state.push(Val::Num(height));
    state.push(flags_val);
    Ok(3)
}

fn add_font_text_color_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn(state, font_ref, "SetTextColor", font_set_text_color)?;
    table_set_rust_fn(state, font_ref, "GetTextColor", font_get_text_color)?;
    Ok(())
}

fn font_set_text_color(state: &mut LuaState) -> LuaResult<u32> {
    set_rgba_component_fields(state, "__textColor")
}

fn font_get_text_color(state: &mut LuaState) -> LuaResult<u32> {
    get_rgba_component_fields(state, "__textColor")
}

fn add_font_shadow_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn(state, font_ref, "SetShadowColor", font_set_shadow_color)?;
    table_set_rust_fn(state, font_ref, "GetShadowColor", font_get_shadow_color)?;
    table_set_rust_fn(state, font_ref, "SetShadowOffset", font_set_shadow_offset)?;
    table_set_rust_fn(state, font_ref, "GetShadowOffset", font_get_shadow_offset)?;
    Ok(())
}

fn font_set_shadow_color(state: &mut LuaState) -> LuaResult<u32> {
    set_rgba_component_fields(state, "__shadowColor")
}

fn font_get_shadow_color(state: &mut LuaState) -> LuaResult<u32> {
    get_rgba_component_fields(state, "__shadowColor")
}

fn font_set_shadow_offset(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let x = f64::from_stack(state, 2)?;
    let y = f64::from_stack(state, 3)?;
    table_set(state, font, "__shadowOffsetX", Val::Num(x));
    table_set(state, font, "__shadowOffsetY", Val::Num(y));
    Ok(0)
}

fn font_get_shadow_offset(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let x = font_f64(state, font, "__shadowOffsetX");
    let y = font_f64(state, font, "__shadowOffsetY");
    (x, y).into_stack(state)
}

/// Read r,g,b,a from stack (2..=5, a defaulting to 1.0) and store on the
/// font under `{prefix}R`, `{prefix}G`, `{prefix}B`, `{prefix}A`.
fn set_rgba_component_fields(state: &mut LuaState, prefix: &str) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let r = f64::from_stack(state, 2)?;
    let g = f64::from_stack(state, 3)?;
    let b = f64::from_stack(state, 4)?;
    let a = Option::<f64>::from_stack(state, 5)?.unwrap_or(1.0);
    let components = [("R", r), ("G", g), ("B", b), ("A", a)];
    for (suffix, value) in components {
        let key = format!("{prefix}{suffix}");
        table_set(state, font, &key, Val::Num(value));
    }
    Ok(0)
}

fn get_rgba_component_fields(state: &mut LuaState, prefix: &str) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let r = font_f64(state, font, &format!("{prefix}R"));
    let g = font_f64(state, font, &format!("{prefix}G"));
    let b = font_f64(state, font, &format!("{prefix}B"));
    let a = font_f64(state, font, &format!("{prefix}A"));
    (r, g, b, a).into_stack(state)
}

fn font_set_justify_h(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let j = String::from_stack(state, 2)?;
    let jv = create_string(state, &j);
    table_set(state, font, "__justifyH", jv);
    Ok(0)
}

fn font_get_justify_h(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let j = font_str(state, font, "__justifyH");
    create_string(state, &j).into_stack(state)
}

fn font_set_justify_v(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let j = String::from_stack(state, 2)?;
    let jv = create_string(state, &j);
    table_set(state, font, "__justifyV", jv);
    Ok(0)
}

fn font_get_justify_v(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let j = font_str(state, font, "__justifyV");
    create_string(state, &j).into_stack(state)
}

fn font_set_spacing(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let spacing = f64::from_stack(state, 2)?;
    table_set(state, font, "__spacing", Val::Num(spacing));
    Ok(0)
}

fn font_get_spacing(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    font_f64(state, font, "__spacing").into_stack(state)
}

fn add_font_justify_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn(state, font_ref, "SetJustifyH", font_set_justify_h)?;
    table_set_rust_fn(state, font_ref, "GetJustifyH", font_get_justify_h)?;
    table_set_rust_fn(state, font_ref, "SetJustifyV", font_set_justify_v)?;
    table_set_rust_fn(state, font_ref, "GetJustifyV", font_get_justify_v)?;
    table_set_rust_fn(state, font_ref, "SetSpacing", font_set_spacing)?;
    table_set_rust_fn(state, font_ref, "GetSpacing", font_get_spacing)?;
    Ok(())
}

fn add_font_wrap_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn(state, font_ref, "SetIndentedWordWrap", |state| {
        let font = stack_val(state, 1);
        let v = bool::from_stack(state, 2)?;
        table_set(state, font, "__indentedWordWrap", Val::Bool(v));
        Ok(0)
    })?;
    table_set_rust_fn(state, font_ref, "GetIndentedWordWrap", |state| {
        let font = stack_val(state, 1);
        let v = matches!(
            table_get(state, font, "__indentedWordWrap"),
            Val::Bool(true)
        );
        v.into_stack(state)
    })?;
    table_set_rust_fn(state, font_ref, "SetMaxLines", |state| {
        let font = stack_val(state, 1);
        let v = i32::from_stack(state, 2)?;
        table_set(state, font, "__maxLines", Val::Num(v as f64));
        Ok(0)
    })?;
    table_set_rust_fn(state, font_ref, "GetMaxLines", |state| {
        let font = stack_val(state, 1);
        let v = match table_get(state, font, "__maxLines") {
            Val::Num(n) => n as i32,
            _ => 0,
        };
        v.into_stack(state)
    })?;
    Ok(())
}

fn font_get_name(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let name = table_get(state, font, "__name");
    match name {
        Val::Str(_) => name.into_stack(state),
        _ => Val::Nil.into_stack(state),
    }
}

fn font_get_font_object_for_alphabet(state: &mut LuaState) -> LuaResult<u32> {
    stack_val(state, 1).into_stack(state)
}

// TODO: full cycle-safe property copy (mlua: copy_font_properties)
fn font_copy_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let _font = stack_val(state, 1);
    let _src = stack_val(state, 2);
    Ok(0)
}

fn font_get_object_type(state: &mut LuaState) -> LuaResult<u32> {
    create_string(state, "Font").into_stack(state)
}

fn font_is_object_type(state: &mut LuaState) -> LuaResult<u32> {
    let name = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    name.eq_ignore_ascii_case("Font").into_stack(state)
}

fn font_get_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    table_get(state, font, "__fontObject").into_stack(state)
}

// TODO: cycle detection (mlua: detect_font_object_cycle)
fn font_set_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let target = stack_val(state, 2);
    if matches!(target, Val::Table(_)) {
        table_set(state, font, "__fontObject", target);
    }
    Ok(0)
}

fn add_font_meta_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn(state, font_ref, "GetName", font_get_name)?;
    table_set_rust_fn(
        state,
        font_ref,
        "GetFontObjectForAlphabet",
        font_get_font_object_for_alphabet,
    )?;
    table_set_rust_fn(state, font_ref, "CopyFontObject", font_copy_font_object)?;
    table_set_rust_fn(state, font_ref, "GetObjectType", font_get_object_type)?;
    table_set_rust_fn(state, font_ref, "IsObjectType", font_is_object_type)?;
    table_set_rust_fn(state, font_ref, "GetFontObject", font_get_font_object)?;
    table_set_rust_fn(state, font_ref, "SetFontObject", font_set_font_object)?;
    Ok(())
}

/// Register all font methods on a font table Val.
pub(super) fn add_font_methods(state: &mut LuaState, font: Val) -> LuaResult<()> {
    let Val::Table(font_ref) = font else {
        return Err(runtime_error("add_font_methods: expected table"));
    };
    add_font_field_methods(state, font_ref)?;
    add_font_text_color_methods(state, font_ref)?;
    add_font_shadow_methods(state, font_ref)?;
    add_font_justify_methods(state, font_ref)?;
    add_font_wrap_methods(state, font_ref)?;
    add_font_meta_methods(state, font_ref)?;
    Ok(())
}

// ── Top-level Font API functions ──────────────────────────────────────────────

pub fn create_font(state: &mut LuaState) -> LuaResult<u32> {
    // Coerce name arg (string or number); nil/other → error
    let name_val = stack_val(state, 1);
    let name = match name_val {
        Val::Str(_) => val_to_string(state, name_val)
            .ok_or_else(|| runtime_error("CreateFont: invalid string"))?,
        Val::Num(n) => (n as i64).to_string(),
        _ => return Err(runtime_error("Usage: CreateFont(\"name\")")),
    };
    // TODO: check __font_registry for existing object
    let font = create_table(state);
    font_set_defaults(state, font, Some(&name));
    add_font_methods(state, font)?;
    set_global_val(state, &name, font);
    font.into_stack(state)
}

pub fn get_fonts(state: &mut LuaState) -> LuaResult<u32> {
    create_table(state).into_stack(state)
}

pub fn get_font_info(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: resolve font object from arg, populate fields
    let info = create_table(state);
    let empty = create_string(state, "");
    table_set(state, info, "name", empty);
    table_set(state, info, "height", Val::Num(12.0));
    let outline = create_string(state, "");
    table_set(state, info, "outline", outline);
    info.into_stack(state)
}

pub fn create_font_family(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let _members = stack_val(state, 2);
    // TODO: extract first member's file/height/flags from members table
    let font = create_table(state);
    font_set_defaults(state, font, Some(&name));
    add_font_methods(state, font)?;
    set_global_val(state, &name, font);
    font.into_stack(state)
}

fn apply_standard_font_colors(
    state: &mut LuaState,
    font: Val,
    height: f64,
    flags: &str,
    r: f64,
    g: f64,
    b: f64,
) {
    table_set(state, font, "__fontHeight", Val::Num(height));
    let flags_val = create_string(state, flags);
    table_set(state, font, "__fontFlags", flags_val);
    table_set(state, font, "__textColorR", Val::Num(r));
    table_set(state, font, "__textColorG", Val::Num(g));
    table_set(state, font, "__textColorB", Val::Num(b));
}

/// Create and register all standard WoW font globals on the rilua state.
pub fn register_standard_font_objects(lua: &mut rilua::Lua) -> LuaResult<()> {
    for &(name, height, flags, r, g, b) in STANDARD_FONTS {
        let state = lua.state_mut();
        let font = create_table(state);
        font_set_defaults(state, font, Some(name));
        apply_standard_font_colors(state, font, height, flags, r, g, b);
        add_font_methods(state, font)?;
        set_global_val(state, name, font);
    }
    Ok(())
}

/// Register font API globals (CreateFont, GetFonts, GetFontInfo, CreateFontFamily)
/// plus all standard font objects.
pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "CreateFont", create_font)?;
    LuaApiMut::register_function(lua, "GetFonts", get_fonts)?;
    LuaApiMut::register_function(lua, "GetFontInfo", get_font_info)?;
    LuaApiMut::register_function(lua, "CreateFontFamily", create_font_family)?;
    register_standard_font_objects(lua)?;
    Ok(())
}
