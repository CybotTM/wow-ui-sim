//! Font API — CreateFont, GetFonts, GetFontInfo, CreateFontFamily,
//! register_standard_font_objects, and the per-font-object method table.

use crate::lua_api::methods::{
    create_string, create_string_static, create_table, table_get, table_get_static, table_set,
    table_set_static, val_to_string,
};
use crate::lua_bridge::table_set_rust_fn_static;
use crate::lua_bridge::{FromStack, IntoStack, stack_val};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table as RiluaTable;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

use super::set_global_val;

type FontTableRef = GcRef<RiluaTable>;

// ── Data ─────────────────────────────────────────────────────────────────────

const DEFAULT_FONT_PATH: &str = "Fonts\\FRIZQT__.TTF";

/// (name, height, flags, r, g, b)
const STANDARD_FONTS: &[(&'static str, f64, &str, f64, f64, f64)] = &[
    ("GameFontNormal", 12.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalSmall", 10.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalLarge", 16.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalHuge", 20.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalMed1", 13.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalMed2", 13.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalMed3", 14.0, "", 1.0, 0.82, 0.0),
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
    ("GameFontHighlightMedium", 14.0, "", 1.0, 1.0, 1.0),
    ("GameFontHighlightMed2", 13.0, "", 1.0, 1.0, 1.0),
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
    ("GameFontBlackMedium", 14.0, "", 0.0, 0.0, 0.0),
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
    ("GameTooltipHeaderText", 14.0, "", 1.0, 1.0, 1.0),
    ("GameTooltipText", 12.0, "", 1.0, 1.0, 1.0),
    ("GameTooltipTextSmall", 10.0, "", 1.0, 1.0, 1.0),
    ("ObjectiveFont", 12.0, "", 0.8, 0.8, 0.8),
    ("SubZoneTextFont", 26.0, "OUTLINE", 1.0, 0.82, 0.0),
    ("PVPInfoTextFont", 20.0, "OUTLINE", 1.0, 0.1, 0.1),
    ("FriendsFont_Normal", 12.0, "", 1.0, 1.0, 1.0),
    ("FriendsFont_Small", 10.0, "", 1.0, 1.0, 1.0),
    ("FriendsFont_Large", 14.0, "", 1.0, 1.0, 1.0),
    ("FriendsFont_UserText", 11.0, "", 1.0, 1.0, 1.0),
];

// ── Low-level field helpers ───────────────────────────────────────────────────

const TEXT_COLOR_KEYS: [&str; 4] = [
    "__textColorR",
    "__textColorG",
    "__textColorB",
    "__textColorA",
];
const SHADOW_COLOR_KEYS: [&str; 4] = [
    "__shadowColorR",
    "__shadowColorG",
    "__shadowColorB",
    "__shadowColorA",
];

/// Fetch a numeric font field by compile-time-literal key. Routes
/// through `table_get_static` → `intern_string_static` so the key
/// is never content-hashed past its first insertion in the static
/// intern cache.
pub(super) fn font_f64_static(state: &mut LuaState, font: Val, key: &'static str) -> f64 {
    match table_get_static(state, font, key) {
        Val::Num(n) => n,
        _ => 0.0,
    }
}

/// Fetch a string font field by compile-time-literal key. Same
/// static-cache fast path as [`font_f64_static`].
pub(super) fn font_str_static(state: &mut LuaState, font: Val, key: &'static str) -> String {
    let value = table_get_static(state, font, key);
    val_to_string(state, value).unwrap_or_default()
}

pub(super) fn font_set_defaults(state: &mut LuaState, font: Val, name: Option<&str>) {
    table_set_static(state, font, "__fontHeight", Val::Num(0.0));
    let empty = create_string_static(state, "");
    table_set_static(state, font, "__fontFlags", empty);
    table_set_static(state, font, "__textColorR", Val::Num(1.0));
    table_set_static(state, font, "__textColorG", Val::Num(1.0));
    table_set_static(state, font, "__textColorB", Val::Num(1.0));
    table_set_static(state, font, "__textColorA", Val::Num(1.0));
    table_set_static(state, font, "__shadowColorR", Val::Num(0.0));
    table_set_static(state, font, "__shadowColorG", Val::Num(0.0));
    table_set_static(state, font, "__shadowColorB", Val::Num(0.0));
    table_set_static(state, font, "__shadowColorA", Val::Num(0.0));
    table_set_static(state, font, "__shadowOffsetX", Val::Num(0.0));
    table_set_static(state, font, "__shadowOffsetY", Val::Num(0.0));
    let center = create_string_static(state, "CENTER");
    table_set_static(state, font, "__justifyH", center);
    let middle = create_string_static(state, "MIDDLE");
    table_set_static(state, font, "__justifyV", middle);
    let name_val = match name {
        Some(n) => create_string(state, n),
        None => Val::Nil,
    };
    table_set_static(state, font, "__name", name_val);
}

// ── Method group registrations ────────────────────────────────────────────────

fn add_font_field_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(state, font_ref, "SetFontHeight", font_set_font_height)?;
    table_set_rust_fn_static(state, font_ref, "GetFontHeight", font_get_font_height)?;
    table_set_rust_fn_static(state, font_ref, "SetFont", font_set_font)?;
    table_set_rust_fn_static(state, font_ref, "GetFont", font_get_font)?;
    Ok(())
}

fn font_set_font_height(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let height = f64::from_stack(state, 2)?;
    table_set_static(state, font, "__fontHeight", Val::Num(height));
    Ok(0)
}

fn font_get_font_height(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    font_f64_static(state, font, "__fontHeight").into_stack(state)
}

fn font_set_font(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let path = Option::<String>::from_stack(state, 2)?;
    let height = Option::<f64>::from_stack(state, 3)?;
    let flags = Option::<String>::from_stack(state, 4)?;
    let Some(path) = path else { return Ok(0) };
    let path_val = create_string(state, &path);
    table_set_static(state, font, "__fontPath", path_val);
    if let Some(h) = height {
        table_set_static(state, font, "__fontHeight", Val::Num(h));
    }
    let flags_val = create_string(state, flags.as_deref().unwrap_or(""));
    table_set_static(state, font, "__fontFlags", flags_val);
    Ok(0)
}

fn font_get_font(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let path = table_get_static(state, font, "__fontPath");
    let path_val = match path {
        Val::Str(_) => path,
        _ => Val::Nil,
    };
    let height = font_f64_static(state, font, "__fontHeight");
    let flags = font_str_static(state, font, "__fontFlags");
    let flags_val = create_string(state, &flags);
    state.push(path_val);
    state.push(Val::Num(height));
    state.push(flags_val);
    Ok(3)
}

fn add_font_text_color_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(state, font_ref, "SetTextColor", font_set_text_color)?;
    table_set_rust_fn_static(state, font_ref, "GetTextColor", font_get_text_color)?;
    Ok(())
}

fn font_set_text_color(state: &mut LuaState) -> LuaResult<u32> {
    set_rgba_component_fields(state, TEXT_COLOR_KEYS)
}

fn font_get_text_color(state: &mut LuaState) -> LuaResult<u32> {
    get_rgba_component_fields(state, TEXT_COLOR_KEYS)
}

fn add_font_shadow_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(state, font_ref, "SetShadowColor", font_set_shadow_color)?;
    table_set_rust_fn_static(state, font_ref, "GetShadowColor", font_get_shadow_color)?;
    table_set_rust_fn_static(state, font_ref, "SetShadowOffset", font_set_shadow_offset)?;
    table_set_rust_fn_static(state, font_ref, "GetShadowOffset", font_get_shadow_offset)?;
    Ok(())
}

fn font_set_shadow_color(state: &mut LuaState) -> LuaResult<u32> {
    set_rgba_component_fields(state, SHADOW_COLOR_KEYS)
}

fn font_get_shadow_color(state: &mut LuaState) -> LuaResult<u32> {
    get_rgba_component_fields(state, SHADOW_COLOR_KEYS)
}

fn font_set_shadow_offset(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let x = f64::from_stack(state, 2)?;
    let y = f64::from_stack(state, 3)?;
    table_set_static(state, font, "__shadowOffsetX", Val::Num(x));
    table_set_static(state, font, "__shadowOffsetY", Val::Num(y));
    Ok(0)
}

fn font_get_shadow_offset(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let x = font_f64_static(state, font, "__shadowOffsetX");
    let y = font_f64_static(state, font, "__shadowOffsetY");
    (x, y).into_stack(state)
}

fn set_rgba_component_fields(state: &mut LuaState, keys: [&'static str; 4]) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let r = f64::from_stack(state, 2)?;
    let g = f64::from_stack(state, 3)?;
    let b = f64::from_stack(state, 4)?;
    let a = Option::<f64>::from_stack(state, 5)?.unwrap_or(1.0);
    for (key, value) in keys.into_iter().zip([r, g, b, a]) {
        table_set_static(state, font, key, Val::Num(value));
    }
    Ok(0)
}

fn get_rgba_component_fields(state: &mut LuaState, keys: [&'static str; 4]) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let [r_key, g_key, b_key, a_key] = keys;
    let r = font_f64_static(state, font, r_key);
    let g = font_f64_static(state, font, g_key);
    let b = font_f64_static(state, font, b_key);
    let a = font_f64_static(state, font, a_key);
    (r, g, b, a).into_stack(state)
}

fn font_set_justify_h(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let j = String::from_stack(state, 2)?;
    let jv = create_string(state, &j);
    table_set_static(state, font, "__justifyH", jv);
    Ok(0)
}

fn font_get_justify_h(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let j = font_str_static(state, font, "__justifyH");
    create_string(state, &j).into_stack(state)
}

fn font_set_justify_v(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let j = String::from_stack(state, 2)?;
    let jv = create_string(state, &j);
    table_set_static(state, font, "__justifyV", jv);
    Ok(0)
}

fn font_get_justify_v(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let j = font_str_static(state, font, "__justifyV");
    create_string(state, &j).into_stack(state)
}

fn font_set_spacing(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let spacing = f64::from_stack(state, 2)?;
    table_set_static(state, font, "__spacing", Val::Num(spacing));
    Ok(0)
}

fn font_get_spacing(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    font_f64_static(state, font, "__spacing").into_stack(state)
}

fn add_font_justify_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(state, font_ref, "SetJustifyH", font_set_justify_h)?;
    table_set_rust_fn_static(state, font_ref, "GetJustifyH", font_get_justify_h)?;
    table_set_rust_fn_static(state, font_ref, "SetJustifyV", font_set_justify_v)?;
    table_set_rust_fn_static(state, font_ref, "GetJustifyV", font_get_justify_v)?;
    table_set_rust_fn_static(state, font_ref, "SetSpacing", font_set_spacing)?;
    table_set_rust_fn_static(state, font_ref, "GetSpacing", font_get_spacing)?;
    Ok(())
}

fn add_font_wrap_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(state, font_ref, "SetIndentedWordWrap", |state| {
        let font = stack_val(state, 1);
        let v = bool::from_stack(state, 2)?;
        table_set_static(state, font, "__indentedWordWrap", Val::Bool(v));
        Ok(0)
    })?;
    table_set_rust_fn_static(state, font_ref, "GetIndentedWordWrap", |state| {
        let font = stack_val(state, 1);
        let v = matches!(
            table_get_static(state, font, "__indentedWordWrap"),
            Val::Bool(true)
        );
        v.into_stack(state)
    })?;
    table_set_rust_fn_static(state, font_ref, "SetMaxLines", |state| {
        let font = stack_val(state, 1);
        let v = i32::from_stack(state, 2)?;
        table_set_static(state, font, "__maxLines", Val::Num(v as f64));
        Ok(0)
    })?;
    table_set_rust_fn_static(state, font_ref, "GetMaxLines", |state| {
        let font = stack_val(state, 1);
        let v = match table_get_static(state, font, "__maxLines") {
            Val::Num(n) => n as i32,
            _ => 0,
        };
        v.into_stack(state)
    })?;
    Ok(())
}

fn font_get_name(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let name = table_get_static(state, font, "__name");
    match name {
        Val::Str(_) => name.into_stack(state),
        _ => Val::Nil.into_stack(state),
    }
}

fn font_get_font_object_for_alphabet(state: &mut LuaState) -> LuaResult<u32> {
    stack_val(state, 1).into_stack(state)
}

fn font_copy_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let dest = stack_val(state, 1);
    let Some(src) = resolve_font_object(state, stack_val(state, 2)) else {
        return Ok(0);
    };

    copy_font_string_alias(state, dest, src, "__fontPath", "__font");
    copy_font_number_alias(state, dest, src, "__fontHeight", "__height");
    copy_font_string_alias(state, dest, src, "__fontFlags", "__outline");
    copy_color_components(state, dest, src, TEXT_COLOR_KEYS);
    copy_xml_text_color_components(state, dest, src);
    copy_color_components(state, dest, src, SHADOW_COLOR_KEYS);
    copy_font_number(state, dest, src, "__shadowOffsetX");
    copy_font_number(state, dest, src, "__shadowOffsetY");
    copy_font_string(state, dest, src, "__justifyH");
    copy_font_string(state, dest, src, "__justifyV");
    copy_font_number(state, dest, src, "__spacing");
    copy_font_bool(state, dest, src, "__indentedWordWrap");
    copy_font_number(state, dest, src, "__maxLines");
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
    let existing = table_get_static(state, font, "__fontObject");
    if !matches!(existing, Val::Nil) {
        return existing.into_stack(state);
    }
    let auto_font = create_font_object(state, None);
    table_set_static(state, font, "__fontObject", auto_font);
    auto_font.into_stack(state)
}

// TODO: cycle detection (mlua: detect_font_object_cycle)
fn font_set_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let font = stack_val(state, 1);
    let target = stack_val(state, 2);
    let resolved = match target {
        Val::Table(_) => Some(target),
        Val::Str(_) => resolve_font_object(state, target),
        Val::Nil => None,
        _ => None,
    }
    .ok_or_else(|| runtime_error("SetFontObject requires a font object"))?;
    table_set_static(state, font, "__fontObject", resolved);
    Ok(0)
}

fn add_font_meta_methods(state: &mut LuaState, font_ref: FontTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(state, font_ref, "GetName", font_get_name)?;
    table_set_rust_fn_static(
        state,
        font_ref,
        "GetFontObjectForAlphabet",
        font_get_font_object_for_alphabet,
    )?;
    table_set_rust_fn_static(state, font_ref, "CopyFontObject", font_copy_font_object)?;
    table_set_rust_fn_static(state, font_ref, "GetObjectType", font_get_object_type)?;
    table_set_rust_fn_static(state, font_ref, "IsObjectType", font_is_object_type)?;
    table_set_rust_fn_static(state, font_ref, "GetFontObject", font_get_font_object)?;
    table_set_rust_fn_static(state, font_ref, "SetFontObject", font_set_font_object)?;
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

pub(crate) fn create_font_object(state: &mut LuaState, name: Option<&str>) -> Val {
    let font = create_table(state);
    font_set_defaults(state, font, name);
    let _ = add_font_methods(state, font);
    if let Some(name) = name {
        set_global_val(state, name, font);
    }
    font
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
    let font = create_font_object(state, Some(&name));
    font.into_stack(state)
}

pub fn get_fonts(state: &mut LuaState) -> LuaResult<u32> {
    create_table(state).into_stack(state)
}

pub fn get_font_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(font) = resolve_font_object(state, stack_val(state, 1)) else {
        let info = create_table(state);
        let empty = create_string_static(state, "");
        table_set(state, info, "name", empty);
        table_set(state, info, "height", Val::Num(0.0));
        let empty = create_string_static(state, "");
        table_set(state, info, "outline", empty);
        let empty = create_string_static(state, "");
        table_set(state, info, "flags", empty);
        let color = font_color_info(state, Val::Nil);
        table_set(state, info, "color", color);
        return info.into_stack(state);
    };
    let info = create_table(state);
    let name = match table_get_static(state, font, "__name") {
        Val::Str(_) => table_get_static(state, font, "__name"),
        _ => create_string_static(state, ""),
    };
    table_set(state, info, "name", name);
    let height = font_f64_static(state, font, "__fontHeight");
    table_set(state, info, "height", Val::Num(height));
    let flags = font_str_static(state, font, "__fontFlags");
    let flags_val = create_string(state, &flags);
    table_set(state, info, "outline", flags_val);
    let flags_val = create_string(state, &flags);
    table_set(state, info, "flags", flags_val);
    let font_path = match table_get_static(state, font, "__fontPath") {
        Val::Str(_) => table_get_static(state, font, "__fontPath"),
        _ => Val::Nil,
    };
    table_set(state, info, "font", font_path);
    table_set(state, info, "fontHeight", Val::Num(height));
    let font_flags = create_string(state, &flags);
    table_set(state, info, "fontFlags", font_flags);
    let color = font_color_info(state, font);
    table_set(state, info, "color", color);
    info.into_stack(state)
}

fn font_color_info(state: &mut LuaState, font: Val) -> Val {
    let color = create_table(state);
    let r = font_f64_static(state, font, "__textColorR");
    let g = font_f64_static(state, font, "__textColorG");
    let b = font_f64_static(state, font, "__textColorB");
    let a = font_f64_static(state, font, "__textColorA");
    table_set_static(state, color, "r", Val::Num(r));
    table_set_static(state, color, "g", Val::Num(g));
    table_set_static(state, color, "b", Val::Num(b));
    table_set_static(state, color, "a", Val::Num(a));
    color
}

fn resolve_font_object(state: &mut LuaState, value: Val) -> Option<Val> {
    match value {
        Val::Table(_) => Some(value),
        Val::Str(_) => {
            let name = val_to_string(state, value)?;
            let font = table_get(state, Val::Table(state.global), &name);
            matches!(font, Val::Table(_)).then_some(font)
        }
        _ => None,
    }
}

fn first_family_member_snapshot(
    state: &mut LuaState,
    members: Val,
) -> (Option<String>, f64, String) {
    let Val::Table(members_ref) = members else {
        return (None, 0.0, String::new());
    };
    let first = state
        .gc
        .tables
        .get(members_ref)
        .map(|table| table.get_int(1))
        .unwrap_or(Val::Nil);
    let Val::Table(member_ref) = first else {
        return (None, 0.0, String::new());
    };

    let file = table_get(state, Val::Table(member_ref), "file");
    let path = match file {
        Val::Str(_) => val_to_string(state, file),
        _ => None,
    };
    let height = match table_get(state, Val::Table(member_ref), "height") {
        Val::Num(n) => n,
        _ => 0.0,
    };
    let flags_val = table_get(state, Val::Table(member_ref), "flags");
    let flags = match flags_val {
        Val::Str(_) => val_to_string(state, flags_val).unwrap_or_default(),
        _ => String::new(),
    };
    (path, height, flags)
}

fn copy_font_string(state: &mut LuaState, dest: Val, src: Val, key: &'static str) {
    let value = table_get_static(state, src, key);
    if matches!(value, Val::Str(_)) {
        table_set_static(state, dest, key, value);
    }
}

fn copy_font_string_alias(
    state: &mut LuaState,
    dest: Val,
    src: Val,
    dest_key: &'static str,
    source_alias: &'static str,
) {
    let canonical_value = table_get_static(state, src, dest_key);
    let value = match canonical_value {
        Val::Str(_) => canonical_value,
        _ => table_get_static(state, src, source_alias),
    };
    if matches!(value, Val::Str(_)) {
        table_set_static(state, dest, dest_key, value);
    }
}

fn copy_font_number(state: &mut LuaState, dest: Val, src: Val, key: &'static str) {
    let value = table_get_static(state, src, key);
    if matches!(value, Val::Num(_)) {
        table_set_static(state, dest, key, value);
    }
}

fn copy_font_number_alias(
    state: &mut LuaState,
    dest: Val,
    src: Val,
    dest_key: &'static str,
    source_alias: &'static str,
) {
    let canonical_value = table_get_static(state, src, dest_key);
    let value = match canonical_value {
        Val::Num(_) => canonical_value,
        _ => table_get_static(state, src, source_alias),
    };
    if matches!(value, Val::Num(_)) {
        table_set_static(state, dest, dest_key, value);
    }
}

fn copy_font_bool(state: &mut LuaState, dest: Val, src: Val, key: &'static str) {
    let value = table_get_static(state, src, key);
    if matches!(value, Val::Bool(_)) {
        table_set_static(state, dest, key, value);
    }
}

fn copy_color_components(state: &mut LuaState, dest: Val, src: Val, keys: [&'static str; 4]) {
    for key in keys {
        let value = table_get_static(state, src, key);
        if matches!(value, Val::Num(_)) {
            table_set_static(state, dest, key, value);
        }
    }
}

fn copy_xml_text_color_components(state: &mut LuaState, dest: Val, src: Val) {
    for (dest_key, source_key) in [
        ("__textColorR", "__r"),
        ("__textColorG", "__g"),
        ("__textColorB", "__b"),
    ] {
        let already_copied = table_get_static(state, dest, dest_key);
        if matches!(already_copied, Val::Num(_)) {
            continue;
        }
        let value = table_get_static(state, src, source_key);
        if matches!(value, Val::Num(_)) {
            table_set_static(state, dest, dest_key, value);
        }
    }
}

pub fn create_font_family(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let members = stack_val(state, 2);
    let (path, height, flags) = first_family_member_snapshot(state, members);
    let font = create_font_object(state, Some(&name));
    if let Some(path) = path {
        let path_val = create_string(state, &path);
        table_set_static(state, font, "__fontPath", path_val);
    }
    table_set_static(state, font, "__fontHeight", Val::Num(height));
    let flags_val = create_string(state, &flags);
    table_set_static(state, font, "__fontFlags", flags_val);
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
    let path = create_string_static(state, DEFAULT_FONT_PATH);
    table_set_static(state, font, "__fontPath", path);
    table_set_static(state, font, "__fontHeight", Val::Num(height));
    let flags_val = create_string(state, flags);
    table_set_static(state, font, "__fontFlags", flags_val);
    table_set_static(state, font, "__textColorR", Val::Num(r));
    table_set_static(state, font, "__textColorG", Val::Num(g));
    table_set_static(state, font, "__textColorB", Val::Num(b));
}

/// Create and register all standard WoW font globals on the rilua state.
pub fn register_standard_font_objects(lua: &mut rilua::Lua) -> LuaResult<()> {
    for &(name, height, flags, r, g, b) in STANDARD_FONTS {
        let state = lua.state_mut();
        let font = create_font_object(state, Some(name));
        apply_standard_font_colors(state, font, height, flags, r, g, b);
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
