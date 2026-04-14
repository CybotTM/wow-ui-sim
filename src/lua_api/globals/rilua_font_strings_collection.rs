//! rilua RustFn equivalents of font_api, strings/mod, and c_collection_api globals.
//!
//! Each section mirrors the mlua original but targets the rilua VM:
//! - Plain `fn(&mut LuaState) -> LuaResult<u32>` for state-free helpers.
//! - `borrow_state` / `borrow_state_mut` for SimState access.
//! - `TableBuilder` / `define_functions!` for namespace tables.
//! - `LuaApiMut::register_function` (via `WowLuaEnv::register_rilua_function`) for globals.

use crate::lua_api::rilua_methods::{borrow_state, borrow_state_mut, create_string, create_table, create_table_with_fields, table_get, table_set, val_to_string};
use crate::lua_bridge::{FromStack, IntoStack, TableBuilder};
use crate::lua_bridge::table_set_rust_fn;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

// ============================================================================
// Section 1: font_api — CreateFont, GetFonts, GetFontInfo, CreateFontFamily,
//             create_standard_font_objects
// ============================================================================

// ── Font table field helpers ─────────────────────────────────────────────────

fn font_set_defaults(state: &mut LuaState, font: Val, name: Option<&str>) {
    table_set(state, font, "__fontHeight", Val::Num(0.0));
    table_set(state, font, "__fontFlags", create_string(state, ""));
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
    table_set(state, font, "__justifyH", create_string(state, "CENTER"));
    table_set(state, font, "__justifyV", create_string(state, "MIDDLE"));
    match name {
        Some(n) => { table_set(state, font, "__name", create_string(state, n)); }
        None    => { table_set(state, font, "__name", Val::Nil); }
    }
}

fn font_table_get_f64(state: &mut LuaState, font: Val, key: &str) -> f64 {
    match table_get(state, font, key) {
        Val::Num(n) => n,
        _ => 0.0,
    }
}

fn font_table_get_string(state: &mut LuaState, font: Val, key: &str) -> String {
    val_to_string(state, table_get(state, font, key)).unwrap_or_default()
}

// ── Font method registration ─────────────────────────────────────────────────

fn add_font_methods(state: &mut LuaState, font: Val) -> LuaResult<()> {
    let Val::Table(font_ref) = font else {
        return Err(runtime_error("expected font table"));
    };

    // SetFontHeight
    table_set_rust_fn(state, font_ref, "SetFontHeight", |state| {
        // args: self, height
        let font = Val::from_stack(state, 1)?;
        let height = f64::from_stack(state, 2)?;
        table_set(state, font, "__fontHeight", Val::Num(height));
        Ok(0)
    })?;

    // GetFontHeight
    table_set_rust_fn(state, font_ref, "GetFontHeight", |state| {
        let font = Val::from_stack(state, 1)?;
        let height = font_table_get_f64(state, font, "__fontHeight");
        height.into_stack(state)
    })?;

    // SetFont
    table_set_rust_fn(state, font_ref, "SetFont", |state| {
        let font = Val::from_stack(state, 1)?;
        let path = Option::<String>::from_stack(state, 2)?;
        let height = Option::<f64>::from_stack(state, 3)?;
        let flags = Option::<String>::from_stack(state, 4)?;
        let Some(path) = path else { return Ok(0) };
        table_set(state, font, "__fontPath", create_string(state, &path));
        if let Some(h) = height {
            table_set(state, font, "__fontHeight", Val::Num(h));
        }
        let flags_val = create_string(state, flags.as_deref().unwrap_or(""));
        table_set(state, font, "__fontFlags", flags_val);
        Ok(0)
    })?;

    // GetFont → (path|nil, height, flags)
    table_set_rust_fn(state, font_ref, "GetFont", |state| {
        let font = Val::from_stack(state, 1)?;
        let path = table_get(state, font, "__fontPath");
        let path_val = match path {
            Val::Str(_) => path,
            _ => Val::Nil,
        };
        let height = font_table_get_f64(state, font, "__fontHeight");
        let flags = font_table_get_string(state, font, "__fontFlags");
        let flags_val = create_string(state, &flags);
        state.push(path_val);
        state.push(Val::Num(height));
        state.push(flags_val);
        Ok(3)
    })?;

    // SetTextColor
    table_set_rust_fn(state, font_ref, "SetTextColor", |state| {
        let font = Val::from_stack(state, 1)?;
        let r = f64::from_stack(state, 2)?;
        let g = f64::from_stack(state, 3)?;
        let b = f64::from_stack(state, 4)?;
        let a = Option::<f64>::from_stack(state, 5)?.unwrap_or(1.0);
        table_set(state, font, "__textColorR", Val::Num(r));
        table_set(state, font, "__textColorG", Val::Num(g));
        table_set(state, font, "__textColorB", Val::Num(b));
        table_set(state, font, "__textColorA", Val::Num(a));
        Ok(0)
    })?;

    // GetTextColor → (r, g, b, a)
    table_set_rust_fn(state, font_ref, "GetTextColor", |state| {
        let font = Val::from_stack(state, 1)?;
        let r = font_table_get_f64(state, font, "__textColorR");
        let g = font_table_get_f64(state, font, "__textColorG");
        let b = font_table_get_f64(state, font, "__textColorB");
        let a = font_table_get_f64(state, font, "__textColorA");
        (r, g, b, a).into_stack(state)
    })?;

    // SetShadowColor
    table_set_rust_fn(state, font_ref, "SetShadowColor", |state| {
        let font = Val::from_stack(state, 1)?;
        let r = f64::from_stack(state, 2)?;
        let g = f64::from_stack(state, 3)?;
        let b = f64::from_stack(state, 4)?;
        let a = Option::<f64>::from_stack(state, 5)?.unwrap_or(1.0);
        table_set(state, font, "__shadowColorR", Val::Num(r));
        table_set(state, font, "__shadowColorG", Val::Num(g));
        table_set(state, font, "__shadowColorB", Val::Num(b));
        table_set(state, font, "__shadowColorA", Val::Num(a));
        Ok(0)
    })?;

    // GetShadowColor → (r, g, b, a)
    table_set_rust_fn(state, font_ref, "GetShadowColor", |state| {
        let font = Val::from_stack(state, 1)?;
        let r = font_table_get_f64(state, font, "__shadowColorR");
        let g = font_table_get_f64(state, font, "__shadowColorG");
        let b = font_table_get_f64(state, font, "__shadowColorB");
        let a = font_table_get_f64(state, font, "__shadowColorA");
        (r, g, b, a).into_stack(state)
    })?;

    // SetShadowOffset
    table_set_rust_fn(state, font_ref, "SetShadowOffset", |state| {
        let font = Val::from_stack(state, 1)?;
        let x = f64::from_stack(state, 2)?;
        let y = f64::from_stack(state, 3)?;
        table_set(state, font, "__shadowOffsetX", Val::Num(x));
        table_set(state, font, "__shadowOffsetY", Val::Num(y));
        Ok(0)
    })?;

    // GetShadowOffset → (x, y)
    table_set_rust_fn(state, font_ref, "GetShadowOffset", |state| {
        let font = Val::from_stack(state, 1)?;
        let x = font_table_get_f64(state, font, "__shadowOffsetX");
        let y = font_table_get_f64(state, font, "__shadowOffsetY");
        (x, y).into_stack(state)
    })?;

    // SetJustifyH
    table_set_rust_fn(state, font_ref, "SetJustifyH", |state| {
        let font = Val::from_stack(state, 1)?;
        let j = String::from_stack(state, 2)?;
        table_set(state, font, "__justifyH", create_string(state, &j));
        Ok(0)
    })?;

    // GetJustifyH
    table_set_rust_fn(state, font_ref, "GetJustifyH", |state| {
        let font = Val::from_stack(state, 1)?;
        let j = font_table_get_string(state, font, "__justifyH");
        create_string(state, &j).into_stack(state)
    })?;

    // SetJustifyV
    table_set_rust_fn(state, font_ref, "SetJustifyV", |state| {
        let font = Val::from_stack(state, 1)?;
        let j = String::from_stack(state, 2)?;
        table_set(state, font, "__justifyV", create_string(state, &j));
        Ok(0)
    })?;

    // GetJustifyV
    table_set_rust_fn(state, font_ref, "GetJustifyV", |state| {
        let font = Val::from_stack(state, 1)?;
        let j = font_table_get_string(state, font, "__justifyV");
        create_string(state, &j).into_stack(state)
    })?;

    // SetSpacing
    table_set_rust_fn(state, font_ref, "SetSpacing", |state| {
        let font = Val::from_stack(state, 1)?;
        let spacing = f64::from_stack(state, 2)?;
        table_set(state, font, "__spacing", Val::Num(spacing));
        Ok(0)
    })?;

    // GetSpacing
    table_set_rust_fn(state, font_ref, "GetSpacing", |state| {
        let font = Val::from_stack(state, 1)?;
        let s = font_table_get_f64(state, font, "__spacing");
        s.into_stack(state)
    })?;

    // SetIndentedWordWrap
    table_set_rust_fn(state, font_ref, "SetIndentedWordWrap", |state| {
        let font = Val::from_stack(state, 1)?;
        let v = bool::from_stack(state, 2)?;
        table_set(state, font, "__indentedWordWrap", Val::Bool(v));
        Ok(0)
    })?;

    // GetIndentedWordWrap
    table_set_rust_fn(state, font_ref, "GetIndentedWordWrap", |state| {
        let font = Val::from_stack(state, 1)?;
        let v = matches!(table_get(state, font, "__indentedWordWrap"), Val::Bool(true));
        v.into_stack(state)
    })?;

    // SetMaxLines
    table_set_rust_fn(state, font_ref, "SetMaxLines", |state| {
        let font = Val::from_stack(state, 1)?;
        let v = i32::from_stack(state, 2)?;
        table_set(state, font, "__maxLines", Val::Num(v as f64));
        Ok(0)
    })?;

    // GetMaxLines
    table_set_rust_fn(state, font_ref, "GetMaxLines", |state| {
        let font = Val::from_stack(state, 1)?;
        let v = match table_get(state, font, "__maxLines") {
            Val::Num(n) => n as i32,
            _ => 0,
        };
        v.into_stack(state)
    })?;

    // GetName
    table_set_rust_fn(state, font_ref, "GetName", |state| {
        let font = Val::from_stack(state, 1)?;
        let name = table_get(state, font, "__name");
        match name {
            Val::Str(_) => name.into_stack(state),
            _ => Val::Nil.into_stack(state),
        }
    })?;

    // GetFontObjectForAlphabet — returns self
    table_set_rust_fn(state, font_ref, "GetFontObjectForAlphabet", |state| {
        let font = Val::from_stack(state, 1)?;
        font.into_stack(state)
    })?;

    // CopyFontObject — TODO: full cycle-safe copy; stubs field copy for now
    table_set_rust_fn(state, font_ref, "CopyFontObject", |state| {
        // TODO: implement cycle-safe font property copy
        let _font = Val::from_stack(state, 1)?;
        let _src = Val::from_stack(state, 2)?;
        Ok(0)
    })?;

    // GetObjectType → "Font"
    table_set_rust_fn(state, font_ref, "GetObjectType", |state| {
        create_string(state, "Font").into_stack(state)
    })?;

    // IsObjectType(name) → bool
    table_set_rust_fn(state, font_ref, "IsObjectType", |state| {
        let type_name = Option::<String>::from_stack(state, 2)?
            .unwrap_or_default();
        type_name.eq_ignore_ascii_case("Font").into_stack(state)
    })?;

    // GetFontObject → __fontObject
    table_set_rust_fn(state, font_ref, "GetFontObject", |state| {
        let font = Val::from_stack(state, 1)?;
        let fo = table_get(state, font, "__fontObject");
        fo.into_stack(state)
    })?;

    // SetFontObject — TODO: cycle detection; stubs assignment for now
    table_set_rust_fn(state, font_ref, "SetFontObject", |state| {
        // TODO: implement cycle detection (detect_font_object_cycle equivalent)
        let font = Val::from_stack(state, 1)?;
        let target = Val::from_stack(state, 2)?;
        match target {
            Val::Table(_) | Val::Str(_) => {
                // Only assign table targets directly; string lookup not implemented yet
                table_set(state, font, "__fontObject", target);
            }
            _ => {}
        }
        Ok(0)
    })?;

    Ok(())
}

// ── CreateFont global ────────────────────────────────────────────────────────

fn register_create_font(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "CreateFont", |state| {
        // Coerce name arg (string or number), nil → error
        let name_val = Val::from_stack(state, 1)?;
        let name = match name_val {
            Val::Str(_) => val_to_string(state, name_val)
                .ok_or_else(|| runtime_error("CreateFont: invalid string"))?,
            Val::Num(n) => (n as i64).to_string(),
            _ => return Err(runtime_error("Usage: CreateFont(\"name\")")),
        };

        // Check/create __font_registry in globals
        // TODO: check registry for existing font object to return same instance
        let font = create_table(state);
        font_set_defaults(state, font, Some(&name));
        add_font_methods(state, font)?;

        // Set global _G[name] = font
        let name_val = create_string(state, &name);
        let Val::Str(name_ref) = name_val else { unreachable!() };
        LuaApiMut::set_global(state, name_ref, font)?;

        font.into_stack(state)
    })
}

// ── GetFonts global ──────────────────────────────────────────────────────────

fn register_get_fonts(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "GetFonts", |state| {
        let t = create_table(state);
        t.into_stack(state)
    })
}

// ── GetFontInfo global ───────────────────────────────────────────────────────

fn register_get_font_info(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "GetFontInfo", |state| {
        // TODO: resolve font object from arg (string name or table), populate info
        let info = create_table(state);
        table_set(state, info, "name", create_string(state, ""));
        table_set(state, info, "height", Val::Num(12.0));
        table_set(state, info, "outline", create_string(state, ""));
        info.into_stack(state)
    })
}

// ── CreateFontFamily global ──────────────────────────────────────────────────

fn register_create_font_family(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "CreateFontFamily", |state| {
        let name = String::from_stack(state, 1)?;
        let members = Val::from_stack(state, 2)?;

        let font = create_table(state);
        font_set_defaults(state, font, Some(&name));

        // Apply first member's file/height/flags if present
        // TODO: extract first member from the members table
        let _ = members;

        add_font_methods(state, font)?;
        let name_val = create_string(state, &name);
        let Val::Str(name_ref) = name_val else { unreachable!() };
        LuaApiMut::set_global(state, name_ref, font)?;
        font.into_stack(state)
    })
}

// ── create_standard_font_objects ─────────────────────────────────────────────

/// (name, height, flags, r, g, b)
const STANDARD_FONTS: &[(&str, f64, &str, f64, f64, f64)] = &[
    ("GameFontNormal", 12.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalSmall", 10.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalLarge", 16.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalHuge", 20.0, "", 1.0, 0.82, 0.0),
    ("GameFontHighlight", 12.0, "", 1.0, 1.0, 1.0),
    ("GameFontHighlightSmall", 10.0, "", 1.0, 1.0, 1.0),
    ("GameFontHighlightSmallOutline", 10.0, "OUTLINE", 1.0, 1.0, 1.0),
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
    ("NumberFontNormalRightYellow", 14.0, "OUTLINE", 1.0, 1.0, 0.0),
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
    ("SystemFont_OutlineThick_Huge2", 24.0, "OUTLINE, THICKOUTLINE", 1.0, 1.0, 1.0),
    ("SystemFont_OutlineThick_Huge4", 32.0, "OUTLINE, THICKOUTLINE", 1.0, 1.0, 1.0),
    ("SystemFont_OutlineThick_WTF", 64.0, "OUTLINE, THICKOUTLINE", 1.0, 1.0, 1.0),
    ("SystemFont_Shadow_Small", 10.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Shadow_Med1", 12.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Shadow_Med2", 13.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Shadow_Med3", 14.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Shadow_Large", 16.0, "", 1.0, 1.0, 1.0),
    ("SystemFont_Shadow_Large_Outline", 16.0, "OUTLINE", 1.0, 1.0, 1.0),
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

fn create_standard_font_object(
    lua: &mut rilua::Lua,
    name: &str,
    height: f64,
    flags: &str,
    r: f64,
    g: f64,
    b: f64,
) -> LuaResult<()> {
    let state = lua.state_mut();
    let font = create_table(state);
    font_set_defaults(state, font, Some(name));
    table_set(state, font, "__fontHeight", Val::Num(height));
    table_set(state, font, "__fontFlags", create_string(state, flags));
    table_set(state, font, "__textColorR", Val::Num(r));
    table_set(state, font, "__textColorG", Val::Num(g));
    table_set(state, font, "__textColorB", Val::Num(b));
    add_font_methods(state, font)?;
    let name_val = create_string(state, name);
    let Val::Str(name_ref) = name_val else { unreachable!() };
    LuaApiMut::set_global(state, name_ref, font)
}

pub fn register_standard_font_objects(lua: &mut rilua::Lua) -> LuaResult<()> {
    for &(name, height, flags, r, g, b) in STANDARD_FONTS {
        create_standard_font_object(lua, name, height, flags, r, g, b)?;
    }
    Ok(())
}

// ============================================================================
// Section 2: strings/mod — keybinding functions, tooltip colors, item quality
//             colors, class name tables, icon list
//
// NOTE: String constant registration (register_all_ui_strings) passes raw
// &str data arrays and has no rilua-specific logic — it continues to use mlua.
// Only the Lua-table-building helpers are converted here.
// ============================================================================

// ── Color table helpers ──────────────────────────────────────────────────────

/// Build a rilua color table {r, g, b, a} with GetRGB/GetRGBA/GenerateHexColor/WrapTextInColorCode.
pub fn make_rilua_color_table(state: &mut LuaState, r: f64, g: f64, b: f64, a: f64) -> LuaResult<Val> {
    let t = create_table(state);
    table_set(state, t, "r", Val::Num(r));
    table_set(state, t, "g", Val::Num(g));
    table_set(state, t, "b", Val::Num(b));
    table_set(state, t, "a", Val::Num(a));

    let Val::Table(t_ref) = t else { unreachable!() };

    table_set_rust_fn(state, t_ref, "GetRGB", |state| {
        let this = Val::from_stack(state, 1)?;
        let r = match table_get(state, this, "r") { Val::Num(n) => n, _ => 0.0 };
        let g = match table_get(state, this, "g") { Val::Num(n) => n, _ => 0.0 };
        let b = match table_get(state, this, "b") { Val::Num(n) => n, _ => 0.0 };
        (r, g, b).into_stack(state)
    })?;

    table_set_rust_fn(state, t_ref, "GetRGBA", |state| {
        let this = Val::from_stack(state, 1)?;
        let r = match table_get(state, this, "r") { Val::Num(n) => n, _ => 0.0 };
        let g = match table_get(state, this, "g") { Val::Num(n) => n, _ => 0.0 };
        let b = match table_get(state, this, "b") { Val::Num(n) => n, _ => 0.0 };
        let a = match table_get(state, this, "a") { Val::Num(n) => n, _ => 1.0 };
        (r, g, b, a).into_stack(state)
    })?;

    table_set_rust_fn(state, t_ref, "GenerateHexColor", |state| {
        let this = Val::from_stack(state, 1)?;
        let r = match table_get(state, this, "r") { Val::Num(n) => n, _ => 0.0 };
        let g = match table_get(state, this, "g") { Val::Num(n) => n, _ => 0.0 };
        let b = match table_get(state, this, "b") { Val::Num(n) => n, _ => 0.0 };
        let hex = format!("{:02x}{:02x}{:02x}", (r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8);
        create_string(state, &hex).into_stack(state)
    })?;

    table_set_rust_fn(state, t_ref, "WrapTextInColorCode", |state| {
        let this = Val::from_stack(state, 1)?;
        let text = String::from_stack(state, 2)?;
        let r = match table_get(state, this, "r") { Val::Num(n) => n, _ => 0.0 };
        let g = match table_get(state, this, "g") { Val::Num(n) => n, _ => 0.0 };
        let b = match table_get(state, this, "b") { Val::Num(n) => n, _ => 0.0 };
        let wrapped = format!(
            "|cff{:02x}{:02x}{:02x}{}|r",
            (r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, text
        );
        create_string(state, &wrapped).into_stack(state)
    })?;

    Ok(t)
}

/// Register TOOLTIP_DEFAULT_COLOR and TOOLTIP_DEFAULT_BACKGROUND_COLOR globals.
pub fn register_rilua_tooltip_colors(lua: &mut rilua::Lua) -> LuaResult<()> {
    // TODO: pull constants from string_data when accessible
    let state = lua.state_mut();
    let color = make_rilua_color_table(state, 1.0, 0.9, 0.0, 1.0)?;
    let Val::Str(k1) = create_string(state, "TOOLTIP_DEFAULT_COLOR") else { unreachable!() };
    LuaApiMut::set_global(state, k1, color)?;

    let bg = make_rilua_color_table(state, 0.09, 0.09, 0.19, 1.0)?;
    let Val::Str(k2) = create_string(state, "TOOLTIP_DEFAULT_BACKGROUND_COLOR") else { unreachable!() };
    LuaApiMut::set_global(state, k2, bg)
}

/// Register ITEM_QUALITY_COLORS global.
pub fn register_rilua_item_quality_colors(lua: &mut rilua::Lua) -> LuaResult<()> {
    // TODO: iterate ITEM_QUALITY_COLORS_DATA from string_data and build table
    let state = lua.state_mut();
    let t = create_table(state);
    let Val::Str(k) = create_string(state, "ITEM_QUALITY_COLORS") else { unreachable!() };
    LuaApiMut::set_global(state, k, t)
}

/// Register LOCALIZED_CLASS_NAMES_MALE and LOCALIZED_CLASS_NAMES_FEMALE globals.
pub fn register_rilua_class_name_tables(lua: &mut rilua::Lua) -> LuaResult<()> {
    // TODO: iterate CLASS_NAMES_DATA from string_data
    let state = lua.state_mut();
    let male = create_table(state);
    let female = create_table(state);
    let Val::Str(km) = create_string(state, "LOCALIZED_CLASS_NAMES_MALE") else { unreachable!() };
    LuaApiMut::set_global(state, km, male)?;
    let Val::Str(kf) = create_string(state, "LOCALIZED_CLASS_NAMES_FEMALE") else { unreachable!() };
    LuaApiMut::set_global(state, kf, female)
}

/// Register ICON_LIST global.
pub fn register_rilua_icon_list(lua: &mut rilua::Lua) -> LuaResult<()> {
    // TODO: iterate ICON_LIST_DATA from string_data
    let state = lua.state_mut();
    let t = create_table(state);
    let Val::Str(k) = create_string(state, "ICON_LIST") else { unreachable!() };
    LuaApiMut::set_global(state, k, t)
}

// ── Keybinding function registration ────────────────────────────────────────
//
// Keybindings store their data in SimState, so these RustFns access it via
// borrow_state / borrow_state_mut.

pub fn register_rilua_keybinding_functions(lua: &mut rilua::Lua) -> LuaResult<()> {
    register_rilua_binding_getters(lua)?;
    register_rilua_binding_setters(lua)?;
    register_rilua_binding_persistence(lua)?;
    Ok(())
}

fn register_rilua_binding_getters(lua: &mut rilua::Lua) -> LuaResult<()> {
    // GetBindingKey(action) → key1, key2
    LuaApiMut::register_function(lua, "GetBindingKey", |state| {
        // TODO: implement via keybindings::get_binding_key equivalent on SimState
        state.push(Val::Nil);
        state.push(Val::Nil);
        Ok(2)
    })?;

    // GetBindingKeyForAction(action) → key|nil
    LuaApiMut::register_function(lua, "GetBindingKeyForAction", |state| {
        // TODO: implement via keybindings lookup on SimState
        Val::Nil.into_stack(state)
    })?;

    // GetBindingAction(key, checkOverride?) → action
    LuaApiMut::register_function(lua, "GetBindingAction", |state| {
        // TODO: reverse keybinding lookup on SimState
        create_string(state, "").into_stack(state)
    })?;

    // GetBinding(index) → action, header, key1?, key2?
    LuaApiMut::register_function(lua, "GetBinding", |state| {
        // TODO: keybindings::get_binding_at on SimState
        state.push(Val::Nil);
        state.push(Val::Nil);
        Ok(2)
    })?;

    // GetNumBindings() → count
    LuaApiMut::register_function(lua, "GetNumBindings", |state| {
        // TODO: keybindings::get_num_bindings on SimState
        (0i32).into_stack(state)
    })?;

    // GetCurrentBindingSet() → 1
    LuaApiMut::register_function(lua, "GetCurrentBindingSet", |state| {
        (1i32).into_stack(state)
    })?;

    // GetBindingText(key, prefix?, abbrev?) → key|""
    LuaApiMut::register_function(lua, "GetBindingText", |state| {
        let key = Option::<String>::from_stack(state, 1)?;
        match key {
            Some(k) => create_string(state, &k).into_stack(state),
            None => create_string(state, "").into_stack(state),
        }
    })?;

    // IsBindingForGamePad → false
    LuaApiMut::register_function(lua, "IsBindingForGamePad", |state| {
        false.into_stack(state)
    })?;

    Ok(())
}

fn register_rilua_binding_setters(lua: &mut rilua::Lua) -> LuaResult<()> {
    // SetBinding(key, action?) → bool
    LuaApiMut::register_function(lua, "SetBinding", |state| {
        // TODO: keybindings::set_binding on SimState
        true.into_stack(state)
    })?;

    LuaApiMut::register_function(lua, "SetBindingClick", |state| {
        true.into_stack(state)
    })?;
    LuaApiMut::register_function(lua, "SetBindingSpell", |state| {
        true.into_stack(state)
    })?;
    LuaApiMut::register_function(lua, "SetBindingItem", |state| {
        true.into_stack(state)
    })?;
    LuaApiMut::register_function(lua, "SetBindingMacro", |state| {
        true.into_stack(state)
    })?;
    Ok(())
}

fn register_rilua_binding_persistence(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "SaveBindings", |state| {
        let _which = Option::<i32>::from_stack(state, 1)?;
        Ok(0)
    })?;
    LuaApiMut::register_function(lua, "LoadBindings", |state| {
        let _which = Option::<i32>::from_stack(state, 1)?;
        Ok(0)
    })?;
    Ok(())
}

// ============================================================================
// Section 3: c_collection_api — C_PetJournal, C_MountJournal, C_ToyBox
//
// These are namespace tables (C_*) that live in globals. They need SimState
// access via borrow_state / borrow_state_mut inside each RustFn.
//
// NOTE: rilua RustFn cannot capture Rc<RefCell<SimState>> (plain fn pointer),
// so SimState is always accessed via state.app_data::<WowLuaAppData>().
// ============================================================================

// ── C_PetJournal ─────────────────────────────────────────────────────────────

fn register_rilua_pet_journal(lua: &mut rilua::Lua) -> LuaResult<()> {
    let state = lua.state_mut();
    let t = TableBuilder::new(state)
        // GetNumPets() → total, owned
        .set_function("GetNumPets", |state| {
            let st = borrow_state(state)?;
            let total = st.world.pets.len() as i32;
            let owned = st.world.pets.iter().filter(|p| p.is_collected).count() as i32;
            drop(st);
            (total, owned).into_stack(state)
        })?
        // GetNumCollectedInfo(speciesId) → collected, total
        .set_function("GetNumCollectedInfo", |state| {
            let st = borrow_state(state)?;
            let owned = st.world.pets.iter().filter(|p| p.is_collected).count() as i32;
            let total = st.world.pets.len() as i32;
            drop(st);
            (owned, total).into_stack(state)
        })?
        // GetNumPetsNeedingFanfare() → 0
        .set_function("GetNumPetsNeedingFanfare", |state| {
            (0i32).into_stack(state)
        })?
        // GetPetInfoByIndex(index) → speciesId, ...
        .set_function("GetPetInfoByIndex", |state| {
            let index = i32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let i = (index - 1) as usize;
            let Some(p) = st.world.pets.get(i) else {
                drop(st);
                return Ok(0);
            };
            // Push: speciesId, nil, level, 0, 0, 0, false, name, icon, petType, 0, "", "", false, true, false, false
            let species = p.species_id as f64;
            let level = p.level as f64;
            let icon = p.icon as f64;
            let pet_type = p.pet_type as f64;
            let name_str = p.name.clone();
            drop(st);
            state.push(Val::Num(species));
            state.push(Val::Nil);
            state.push(Val::Num(level));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Bool(false));
            state.push(create_string(state, &name_str));
            state.push(Val::Num(icon));
            state.push(Val::Num(pet_type));
            state.push(Val::Num(0.0));
            state.push(create_string(state, ""));
            state.push(create_string(state, ""));
            state.push(Val::Bool(false));
            state.push(Val::Bool(true));
            state.push(Val::Bool(false));
            state.push(Val::Bool(false));
            Ok(17)
        })?
        // GetPetInfoByPetID(petId) → speciesId, ...
        .set_function("GetPetInfoByPetID", |state| {
            // TODO: lookup by pet_id string
            Ok(0)
        })?
        // GetPetInfoBySpeciesID(speciesId) → speciesId, ...
        .set_function("GetPetInfoBySpeciesID", |state| {
            // TODO: lookup by species_id
            Ok(0)
        })?
        // PetIsSummonable(petId) → false
        .set_function("PetIsSummonable", |state| {
            false.into_stack(state)
        })?
        .build();

    let Val::Str(k) = create_string(lua.state_mut(), "C_PetJournal") else { unreachable!() };
    LuaApiMut::set_global(lua.state_mut(), k, t)
}

// ── C_MountJournal ───────────────────────────────────────────────────────────

fn register_rilua_mount_journal(lua: &mut rilua::Lua) -> LuaResult<()> {
    let state = lua.state_mut();
    let t = TableBuilder::new(state)
        .set_function("GetNumMounts", |state| {
            let st = borrow_state(state)?;
            let n = st.world.mounts.len() as i32;
            drop(st);
            n.into_stack(state)
        })?
        .set_function("GetNumDisplayedMounts", |state| {
            let st = borrow_state(state)?;
            let n = st.world.mounts.len() as i32;
            drop(st);
            n.into_stack(state)
        })?
        .set_function("GetDisplayedMountInfo", |state| {
            let index = i32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let i = (index - 1) as usize;
            let Some(m) = st.world.mounts.get(i) else {
                drop(st);
                return Ok(0);
            };
            let name = m.name.clone();
            let spell_id = m.spell_id as f64;
            let icon = m.icon as f64;
            let is_usable = m.is_usable;
            let is_collected = m.is_collected;
            let mount_id = m.mount_id as f64;
            drop(st);
            state.push(create_string(state, &name));
            state.push(Val::Num(spell_id));
            state.push(Val::Num(icon));
            state.push(Val::Bool(false));
            state.push(Val::Bool(is_usable));
            state.push(Val::Num(0.0));
            state.push(Val::Bool(false));
            state.push(Val::Bool(false));
            state.push(Val::Nil);
            state.push(Val::Bool(false));
            state.push(Val::Bool(is_collected));
            state.push(Val::Num(mount_id));
            Ok(12)
        })?
        .set_function("GetMountInfoByID", |state| {
            let mount_id = u32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let Some(m) = st.world.mounts.iter().find(|m| m.mount_id == mount_id) else {
                drop(st);
                return Ok(0);
            };
            let name = m.name.clone();
            let spell_id = m.spell_id as f64;
            let icon = m.icon as f64;
            let is_usable = m.is_usable;
            let is_collected = m.is_collected;
            let mid = m.mount_id as f64;
            drop(st);
            state.push(create_string(state, &name));
            state.push(Val::Num(spell_id));
            state.push(Val::Num(icon));
            state.push(Val::Bool(false));
            state.push(Val::Bool(is_usable));
            state.push(Val::Num(0.0));
            state.push(Val::Bool(false));
            state.push(Val::Bool(false));
            state.push(Val::Nil);
            state.push(Val::Bool(false));
            state.push(Val::Bool(is_collected));
            state.push(Val::Num(mid));
            Ok(12)
        })?
        .set_function("GetMountInfoExtraByID", |state| {
            let mount_id = u32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let Some(m) = st.world.mounts.iter().find(|m| m.mount_id == mount_id) else {
                drop(st);
                return Ok(0);
            };
            let mount_type = m.mount_type as f64;
            drop(st);
            state.push(Val::Num(0.0));
            state.push(create_string(state, ""));
            state.push(create_string(state, "Drop"));
            state.push(Val::Bool(false));
            state.push(Val::Num(mount_type));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Bool(false));
            Ok(9)
        })?
        .set_function("GetMountIDs", |state| {
            create_table(state).into_stack(state)
        })?
        .set_function("GetNumMountsNeedingFanfare", |state| {
            (0i32).into_stack(state)
        })?
        .set_function("GetCollectedFilterSetting", |state| {
            true.into_stack(state)
        })?
        .set_function("SetCollectedFilterSetting", |state| {
            Ok(0)
        })?
        .set_function("GetIsFavorite", |state| {
            (false, false).into_stack(state)
        })?
        .set_function("SetIsFavorite", |state| {
            Ok(0)
        })?
        .set_function("Summon", |state| {
            Ok(0)
        })?
        .set_function("Dismiss", |state| {
            Ok(0)
        })?
        .build();

    let Val::Str(k) = create_string(lua.state_mut(), "C_MountJournal") else { unreachable!() };
    LuaApiMut::set_global(lua.state_mut(), k, t)
}

// ── C_ToyBox ─────────────────────────────────────────────────────────────────

fn register_rilua_toy_box(lua: &mut rilua::Lua) -> LuaResult<()> {
    let state = lua.state_mut();
    let t = TableBuilder::new(state)
        .set_function("GetNumTotalDisplayedToys", |state| {
            let st = borrow_state(state)?;
            let n = st.world.toys.len() as i32;
            drop(st);
            n.into_stack(state)
        })?
        .set_function("GetNumLearnedDisplayedToys", |state| {
            let st = borrow_state(state)?;
            let n = st.world.toys.iter().filter(|t| t.is_collected).count() as i32;
            drop(st);
            n.into_stack(state)
        })?
        .set_function("GetNumToys", |state| {
            let st = borrow_state(state)?;
            let n = st.world.toys.len() as i32;
            drop(st);
            n.into_stack(state)
        })?
        .set_function("GetNumFilteredToys", |state| {
            let st = borrow_state(state)?;
            let n = st.world.toys.len() as i32;
            drop(st);
            n.into_stack(state)
        })?
        .set_function("GetToyFromIndex", |state| {
            let index = i32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let i = (index - 1) as usize;
            let id = st.world.toys.get(i).map_or(0i32, |t| t.item_id as i32);
            drop(st);
            id.into_stack(state)
        })?
        .set_function("GetToyInfo", |state| {
            let item_id = u32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let Some(toy) = st.world.toys.iter().find(|t| t.item_id == item_id) else {
                drop(st);
                return Ok(0);
            };
            let tid = toy.item_id as f64;
            let name = toy.name.clone();
            let icon = toy.icon as f64;
            drop(st);
            state.push(Val::Num(tid));
            state.push(create_string(state, &name));
            state.push(Val::Num(icon));
            state.push(Val::Bool(false));
            state.push(Val::Bool(false));
            state.push(Val::Num(1.0));
            Ok(6)
        })?
        .set_function("IsToyUsable", |state| {
            let item_id = i32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let usable = st.world.toys.iter()
                .find(|t| t.item_id == item_id as u32)
                .map(|t| t.is_usable)
                .unwrap_or(false);
            drop(st);
            usable.into_stack(state)
        })?
        .set_function("GetIsFavorite", |state| {
            let item_id = i32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let fav = st.world.favorite_toys.contains(&(item_id as u32));
            drop(st);
            fav.into_stack(state)
        })?
        .set_function("HasFavorites", |state| {
            let st = borrow_state(state)?;
            let has = !st.world.favorite_toys.is_empty();
            drop(st);
            has.into_stack(state)
        })?
        .set_function("GetToyLink", |state| {
            let item_id = i32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let link = st.world.toys.iter()
                .find(|t| t.item_id == item_id as u32)
                .map(|toy| format!("|cff0070dd|Hitem:{}::::::::1:0|h[{}]|h|r", toy.item_id, toy.name));
            drop(st);
            match link {
                Some(s) => create_string(state, &s).into_stack(state),
                None => Val::Nil.into_stack(state),
            }
        })?
        .set_function("SetIsFavorite", |state| {
            let item_id = i32::from_stack(state, 1)?;
            let is_fav = bool::from_stack(state, 2)?;
            let mut st = borrow_state_mut(state)?;
            if is_fav {
                st.world.favorite_toys.insert(item_id as u32);
            } else {
                st.world.favorite_toys.remove(&(item_id as u32));
            }
            Ok(0)
        })?
        // Filter stubs
        .set_function("GetCollectedShown", |state| { true.into_stack(state) })?
        .set_function("GetUncollectedShown", |state| { true.into_stack(state) })?
        .set_function("GetUnusableShown", |state| { true.into_stack(state) })?
        .set_function("SetCollectedShown", |state| { Ok(0) })?
        .set_function("SetUncollectedShown", |state| { Ok(0) })?
        .set_function("SetUnusableShown", |state| { Ok(0) })?
        .set_function("ForceToyRefilter", |state| { Ok(0) })?
        .build();

    let Val::Str(k) = create_string(lua.state_mut(), "C_ToyBox") else { unreachable!() };
    LuaApiMut::set_global(lua.state_mut(), k, t)
}

// ============================================================================
// register_all — main entry point
// ============================================================================

/// Register all font, string-table, and collection API globals on the rilua VM.
pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    // Font API
    register_create_font(lua)?;
    register_get_fonts(lua)?;
    register_get_font_info(lua)?;
    register_create_font_family(lua)?;
    register_standard_font_objects(lua)?;

    // String/UI table globals (Lua-table-backed only; pure constants stay mlua)
    register_rilua_tooltip_colors(lua)?;
    register_rilua_item_quality_colors(lua)?;
    register_rilua_class_name_tables(lua)?;
    register_rilua_icon_list(lua)?;
    register_rilua_keybinding_functions(lua)?;

    // Collection namespaces
    register_rilua_pet_journal(lua)?;
    register_rilua_mount_journal(lua)?;
    register_rilua_toy_box(lua)?;

    Ok(())
}
