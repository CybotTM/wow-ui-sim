//! rilua RustFn equivalents of font_api, strings/mod, and c_collection_api globals.
//!
//! Each section mirrors the mlua original but targets the rilua VM:
//! - Plain `fn(&mut LuaState) -> LuaResult<u32>` (or non-capturing closures that
//!   coerce to that) for state-free helpers.
//! - `borrow_state` / `borrow_state_mut` for SimState access inside RustFns.
//! - `TableBuilder` for namespace tables (C_*).
//! - `LuaApiMut::register_function` for top-level globals.
//!
//! # TODOs
//! - `CreateFont`: return cached object on repeat calls (registry lookup).
//! - `CopyFontObject`: cycle-safe property copy.
//! - `SetFontObject`: cycle detection.
//! - `GetFontInfo`: resolve font object from name or table arg.
//! - `CreateFontFamily`: extract first member's file/height/flags.
//! - Keybinding functions: wire up to `keybindings` module on SimState.
//! - `register_rilua_item_quality_colors`: iterate `ITEM_QUALITY_COLORS_DATA`.
//! - `register_rilua_class_name_tables`: iterate `CLASS_NAMES_DATA`.
//! - `register_rilua_icon_list`: iterate `ICON_LIST_DATA`.
//! - `C_PetJournal::GetPetInfoByPetID / GetPetInfoBySpeciesID`: lookup by ID.

use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_get, table_set,
    val_to_string,
};
use crate::lua_bridge::table_set_rust_fn;
use crate::lua_bridge::{FromStack, IntoStack, TableBuilder};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

// ── Global-table helpers (mirrors rilua_create_frame.rs) ─────────────────────

fn set_global_val(state: &mut LuaState, name: &str, value: Val) {
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    if let Some(g) = state.gc.tables.get_mut(global) {
        let _ = g.raw_set(Val::Str(key), value, &state.gc.string_arena);
    }
}

const NAMED_COLOR_GLOBALS: &[(&str, (f64, f64, f64, f64))] = &[
    (
        "PLAYER_FACTION_COLOR_HORDE",
        (0.90196, 0.05098, 0.07059, 1.0),
    ),
    (
        "PLAYER_FACTION_COLOR_ALLIANCE",
        (0.29412, 0.33333, 0.91373, 1.0),
    ),
    ("NORMAL_FONT_COLOR", (1.0, 0.82, 0.0, 1.0)),
    ("HIGHLIGHT_FONT_COLOR", (1.0, 1.0, 1.0, 1.0)),
    ("RED_FONT_COLOR", (1.0, 0.1, 0.1, 1.0)),
    ("GREEN_FONT_COLOR", (0.1, 1.0, 0.1, 1.0)),
    ("GRAY_FONT_COLOR", (0.5, 0.5, 0.5, 1.0)),
    ("PASSIVE_SPELL_FONT_COLOR", (0.5, 0.5, 0.5, 1.0)),
    ("BLACK_FONT_COLOR", (0.0, 0.0, 0.0, 1.0)),
    ("YELLOW_FONT_COLOR", (1.0, 1.0, 0.0, 1.0)),
    ("LIGHTYELLOW_FONT_COLOR", (1.0, 1.0, 0.6, 1.0)),
    ("ORANGE_FONT_COLOR", (1.0, 0.5, 0.25, 1.0)),
    ("WHITE_FONT_COLOR", (1.0, 1.0, 1.0, 1.0)),
    ("DISABLED_FONT_COLOR", (0.5, 0.5, 0.5, 1.0)),
    ("DIM_RED_FONT_COLOR", (0.8, 0.1, 0.1, 1.0)),
    ("LIGHTBLUE_FONT_COLOR", (0.51176, 0.77255, 1.0, 1.0)),
    ("ACTIONBAR_HOTKEY_FONT_COLOR", (0.6, 0.6, 0.6, 1.0)),
    ("FACTION_RED_COLOR", (0.8, 0.13, 0.13, 1.0)),
    ("FACTION_ORANGE_COLOR", (0.93, 0.53, 0.13, 1.0)),
    ("FACTION_YELLOW_COLOR", (0.8, 0.73, 0.13, 1.0)),
    ("FACTION_GREEN_COLOR", (0.13, 0.8, 0.13, 1.0)),
    (
        "OBJECTIVE_TRACKER_BLOCK_HEADER_COLOR",
        (1.0, 0.82, 0.0, 1.0),
    ),
    ("PANEL_BACKGROUND_COLOR", (0.15, 0.15, 0.15, 1.0)),
    ("EDIT_MODE_GRID_LINE_COLOR", (1.0, 1.0, 1.0, 0.3)),
    ("EDIT_MODE_GRID_CENTER_LINE_COLOR", (0.0, 0.8, 1.0, 0.6)),
];

const RAID_CLASS_COLORS_DATA: &[(&str, (f64, f64, f64, f64))] = &[
    ("WARRIOR", (0.78, 0.61, 0.43, 1.0)),
    ("PALADIN", (0.96, 0.55, 0.73, 1.0)),
    ("HUNTER", (0.67, 0.83, 0.45, 1.0)),
    ("ROGUE", (1.00, 0.96, 0.41, 1.0)),
    ("PRIEST", (1.00, 1.00, 1.00, 1.0)),
    ("DEATHKNIGHT", (0.77, 0.12, 0.23, 1.0)),
    ("SHAMAN", (0.00, 0.44, 0.87, 1.0)),
    ("MAGE", (0.25, 0.78, 0.92, 1.0)),
    ("WARLOCK", (0.53, 0.53, 0.93, 1.0)),
    ("MONK", (0.00, 1.00, 0.60, 1.0)),
    ("DRUID", (1.00, 0.49, 0.04, 1.0)),
    ("DEMONHUNTER", (0.64, 0.19, 0.79, 1.0)),
    ("EVOKER", (0.20, 0.58, 0.50, 1.0)),
    ("ADVENTURER", (1.00, 1.00, 1.00, 1.0)),
    ("TRAVELER", (1.00, 1.00, 1.00, 1.0)),
];

// ============================================================================
// Section 1: font_api — CreateFont, GetFonts, GetFontInfo, CreateFontFamily,
//             create_standard_font_objects
// ============================================================================

// ── Font table field helpers ─────────────────────────────────────────────────

fn font_set_defaults(state: &mut LuaState, font: Val, name: Option<&str>) {
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
    let center = create_string(state, "CENTER");
    table_set(state, font, "__justifyH", center);
    let middle = create_string(state, "MIDDLE");
    table_set(state, font, "__justifyV", middle);
    let name_val = match name {
        Some(n) => create_string(state, n),
        None => Val::Nil,
    };
    table_set(state, font, "__name", name_val);
}

fn font_f64(state: &mut LuaState, font: Val, key: &str) -> f64 {
    match table_get(state, font, key) {
        Val::Num(n) => n,
        _ => 0.0,
    }
}

fn font_str(state: &mut LuaState, font: Val, key: &str) -> String {
    let value = table_get(state, font, key);
    val_to_string(state, value).unwrap_or_default()
}

// ── Font method registration ─────────────────────────────────────────────────

/// Register all font methods (SetFont, GetFont, colors, shadow, justify, etc.)
/// on the given font table Val.
fn add_font_methods(state: &mut LuaState, font: Val) -> LuaResult<()> {
    let Val::Table(font_ref) = font else {
        return Err(runtime_error("add_font_methods: expected table"));
    };

    table_set_rust_fn(state, font_ref, "SetFontHeight", |state| {
        let font = Val::from_stack(state, 1)?;
        let height = f64::from_stack(state, 2)?;
        table_set(state, font, "__fontHeight", Val::Num(height));
        Ok(0)
    })?;

    table_set_rust_fn(state, font_ref, "GetFontHeight", |state| {
        let font = Val::from_stack(state, 1)?;
        font_f64(state, font, "__fontHeight").into_stack(state)
    })?;

    table_set_rust_fn(state, font_ref, "SetFont", |state| {
        let font = Val::from_stack(state, 1)?;
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
    })?;

    table_set_rust_fn(state, font_ref, "GetFont", |state| {
        let font = Val::from_stack(state, 1)?;
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
    })?;

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

    table_set_rust_fn(state, font_ref, "GetTextColor", |state| {
        let font = Val::from_stack(state, 1)?;
        let r = font_f64(state, font, "__textColorR");
        let g = font_f64(state, font, "__textColorG");
        let b = font_f64(state, font, "__textColorB");
        let a = font_f64(state, font, "__textColorA");
        (r, g, b, a).into_stack(state)
    })?;

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

    table_set_rust_fn(state, font_ref, "GetShadowColor", |state| {
        let font = Val::from_stack(state, 1)?;
        let r = font_f64(state, font, "__shadowColorR");
        let g = font_f64(state, font, "__shadowColorG");
        let b = font_f64(state, font, "__shadowColorB");
        let a = font_f64(state, font, "__shadowColorA");
        (r, g, b, a).into_stack(state)
    })?;

    table_set_rust_fn(state, font_ref, "SetShadowOffset", |state| {
        let font = Val::from_stack(state, 1)?;
        let x = f64::from_stack(state, 2)?;
        let y = f64::from_stack(state, 3)?;
        table_set(state, font, "__shadowOffsetX", Val::Num(x));
        table_set(state, font, "__shadowOffsetY", Val::Num(y));
        Ok(0)
    })?;

    table_set_rust_fn(state, font_ref, "GetShadowOffset", |state| {
        let font = Val::from_stack(state, 1)?;
        let x = font_f64(state, font, "__shadowOffsetX");
        let y = font_f64(state, font, "__shadowOffsetY");
        (x, y).into_stack(state)
    })?;

    table_set_rust_fn(state, font_ref, "SetJustifyH", |state| {
        let font = Val::from_stack(state, 1)?;
        let j = String::from_stack(state, 2)?;
        let jv = create_string(state, &j);
        table_set(state, font, "__justifyH", jv);
        Ok(0)
    })?;

    table_set_rust_fn(state, font_ref, "GetJustifyH", |state| {
        let font = Val::from_stack(state, 1)?;
        let j = font_str(state, font, "__justifyH");
        create_string(state, &j).into_stack(state)
    })?;

    table_set_rust_fn(state, font_ref, "SetJustifyV", |state| {
        let font = Val::from_stack(state, 1)?;
        let j = String::from_stack(state, 2)?;
        let jv = create_string(state, &j);
        table_set(state, font, "__justifyV", jv);
        Ok(0)
    })?;

    table_set_rust_fn(state, font_ref, "GetJustifyV", |state| {
        let font = Val::from_stack(state, 1)?;
        let j = font_str(state, font, "__justifyV");
        create_string(state, &j).into_stack(state)
    })?;

    table_set_rust_fn(state, font_ref, "SetSpacing", |state| {
        let font = Val::from_stack(state, 1)?;
        let spacing = f64::from_stack(state, 2)?;
        table_set(state, font, "__spacing", Val::Num(spacing));
        Ok(0)
    })?;

    table_set_rust_fn(state, font_ref, "GetSpacing", |state| {
        let font = Val::from_stack(state, 1)?;
        font_f64(state, font, "__spacing").into_stack(state)
    })?;

    table_set_rust_fn(state, font_ref, "SetIndentedWordWrap", |state| {
        let font = Val::from_stack(state, 1)?;
        let v = bool::from_stack(state, 2)?;
        table_set(state, font, "__indentedWordWrap", Val::Bool(v));
        Ok(0)
    })?;

    table_set_rust_fn(state, font_ref, "GetIndentedWordWrap", |state| {
        let font = Val::from_stack(state, 1)?;
        let v = matches!(
            table_get(state, font, "__indentedWordWrap"),
            Val::Bool(true)
        );
        v.into_stack(state)
    })?;

    table_set_rust_fn(state, font_ref, "SetMaxLines", |state| {
        let font = Val::from_stack(state, 1)?;
        let v = i32::from_stack(state, 2)?;
        table_set(state, font, "__maxLines", Val::Num(v as f64));
        Ok(0)
    })?;

    table_set_rust_fn(state, font_ref, "GetMaxLines", |state| {
        let font = Val::from_stack(state, 1)?;
        let v = match table_get(state, font, "__maxLines") {
            Val::Num(n) => n as i32,
            _ => 0,
        };
        v.into_stack(state)
    })?;

    table_set_rust_fn(state, font_ref, "GetName", |state| {
        let font = Val::from_stack(state, 1)?;
        let name = table_get(state, font, "__name");
        match name {
            Val::Str(_) => name.into_stack(state),
            _ => Val::Nil.into_stack(state),
        }
    })?;

    table_set_rust_fn(state, font_ref, "GetFontObjectForAlphabet", |state| {
        Val::from_stack(state, 1)?.into_stack(state)
    })?;

    // TODO: full cycle-safe property copy (mlua: copy_font_properties)
    table_set_rust_fn(state, font_ref, "CopyFontObject", |state| {
        let _font = Val::from_stack(state, 1)?;
        let _src = Val::from_stack(state, 2)?;
        Ok(0)
    })?;

    table_set_rust_fn(state, font_ref, "GetObjectType", |state| {
        create_string(state, "Font").into_stack(state)
    })?;

    table_set_rust_fn(state, font_ref, "IsObjectType", |state| {
        let name = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
        name.eq_ignore_ascii_case("Font").into_stack(state)
    })?;

    table_set_rust_fn(state, font_ref, "GetFontObject", |state| {
        let font = Val::from_stack(state, 1)?;
        table_get(state, font, "__fontObject").into_stack(state)
    })?;

    // TODO: cycle detection (mlua: detect_font_object_cycle)
    table_set_rust_fn(state, font_ref, "SetFontObject", |state| {
        let font = Val::from_stack(state, 1)?;
        let target = Val::from_stack(state, 2)?;
        if matches!(target, Val::Table(_)) {
            table_set(state, font, "__fontObject", target);
        }
        Ok(0)
    })?;

    Ok(())
}

// ── CreateFont ───────────────────────────────────────────────────────────────

pub fn create_font(state: &mut LuaState) -> LuaResult<u32> {
    // Coerce name arg (string or number); nil/other → error
    let name_val = Val::from_stack(state, 1)?;
    let name = match name_val {
        Val::Str(_) => val_to_string(state, name_val)
            .ok_or_else(|| runtime_error("CreateFont: invalid string"))?,
        Val::Num(n) => (n as i64).to_string(),
        _ => return Err(runtime_error("Usage: CreateFont(\"name\")")),
    };

    // TODO: check __font_registry for existing object (return same instance on repeat calls)
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
    // TODO: resolve font object from arg (string name or table), populate fields
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
    let _members = Val::from_stack(state, 2)?;
    // TODO: extract first member's file/height/flags from members table

    let font = create_table(state);
    font_set_defaults(state, font, Some(&name));
    add_font_methods(state, font)?;
    set_global_val(state, &name, font);
    font.into_stack(state)
}

// ── Standard font objects ─────────────────────────────────────────────────────

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

/// Create and register all standard WoW font globals on the rilua state.
pub fn register_standard_font_objects(lua: &mut rilua::Lua) -> LuaResult<()> {
    for &(name, height, flags, r, g, b) in STANDARD_FONTS {
        let state = lua.state_mut();
        let font = create_table(state);
        font_set_defaults(state, font, Some(name));
        table_set(state, font, "__fontHeight", Val::Num(height));
        let flags_val = create_string(state, flags);
        table_set(state, font, "__fontFlags", flags_val);
        table_set(state, font, "__textColorR", Val::Num(r));
        table_set(state, font, "__textColorG", Val::Num(g));
        table_set(state, font, "__textColorB", Val::Num(b));
        add_font_methods(state, font)?;
        set_global_val(state, name, font);
    }
    Ok(())
}

// ============================================================================
// Section 2: strings/mod — color tables, keybinding functions
//
// String constant registration (register_all_ui_strings) passes raw &str
// data arrays and has no rilua-specific logic — it stays mlua.
// Only the Lua-table-building helpers that create method-bearing tables need
// rilua equivalents.
// ============================================================================

// ── Color table helper ────────────────────────────────────────────────────────

/// Build a rilua color table {r, g, b, a} with GetRGB/GetRGBA/GenerateHexColor/WrapTextInColorCode.
pub fn make_rilua_color_table(
    state: &mut LuaState,
    r: f64,
    g: f64,
    b: f64,
    a: f64,
) -> LuaResult<Val> {
    let t = create_table(state);
    table_set(state, t, "r", Val::Num(r));
    table_set(state, t, "g", Val::Num(g));
    table_set(state, t, "b", Val::Num(b));
    table_set(state, t, "a", Val::Num(a));

    let Val::Table(t_ref) = t else { unreachable!() };

    table_set_rust_fn(state, t_ref, "GetRGB", |state| {
        let this = Val::from_stack(state, 1)?;
        let r = match table_get(state, this, "r") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        let g = match table_get(state, this, "g") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        let b = match table_get(state, this, "b") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        (r, g, b).into_stack(state)
    })?;

    table_set_rust_fn(state, t_ref, "GetRGBA", |state| {
        let this = Val::from_stack(state, 1)?;
        let r = match table_get(state, this, "r") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        let g = match table_get(state, this, "g") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        let b = match table_get(state, this, "b") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        let a = match table_get(state, this, "a") {
            Val::Num(n) => n,
            _ => 1.0,
        };
        (r, g, b, a).into_stack(state)
    })?;

    table_set_rust_fn(state, t_ref, "GenerateHexColor", |state| {
        let this = Val::from_stack(state, 1)?;
        let r = match table_get(state, this, "r") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        let g = match table_get(state, this, "g") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        let b = match table_get(state, this, "b") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        let hex = format!(
            "{:02x}{:02x}{:02x}",
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8
        );
        create_string(state, &hex).into_stack(state)
    })?;

    table_set_rust_fn(state, t_ref, "WrapTextInColorCode", |state| {
        let this = Val::from_stack(state, 1)?;
        let text = String::from_stack(state, 2)?;
        let r = match table_get(state, this, "r") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        let g = match table_get(state, this, "g") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        let b = match table_get(state, this, "b") {
            Val::Num(n) => n,
            _ => 0.0,
        };
        let wrapped = format!(
            "|cff{:02x}{:02x}{:02x}{}|r",
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            text
        );
        create_string(state, &wrapped).into_stack(state)
    })?;

    Ok(t)
}

pub fn register_rilua_tooltip_colors(lua: &mut rilua::Lua) -> LuaResult<()> {
    // TODO: pull exact constants from string_data::TOOLTIP_DEFAULT_COLOR
    let state = lua.state_mut();
    let color = make_rilua_color_table(state, 1.0, 0.9, 0.0, 1.0)?;
    set_global_val(state, "TOOLTIP_DEFAULT_COLOR", color);
    let bg = make_rilua_color_table(state, 0.09, 0.09, 0.19, 1.0)?;
    set_global_val(state, "TOOLTIP_DEFAULT_BACKGROUND_COLOR", bg);
    Ok(())
}

pub fn register_rilua_item_quality_colors(lua: &mut rilua::Lua) -> LuaResult<()> {
    // TODO: iterate ITEM_QUALITY_COLORS_DATA from string_data
    let state = lua.state_mut();
    let t = create_table(state);
    set_global_val(state, "ITEM_QUALITY_COLORS", t);
    Ok(())
}

pub fn register_rilua_class_name_tables(lua: &mut rilua::Lua) -> LuaResult<()> {
    // TODO: iterate CLASS_NAMES_DATA from string_data
    let state = lua.state_mut();
    let male = create_table(state);
    let female = create_table(state);
    set_global_val(state, "LOCALIZED_CLASS_NAMES_MALE", male);
    set_global_val(state, "LOCALIZED_CLASS_NAMES_FEMALE", female);
    Ok(())
}

pub fn register_rilua_icon_list(lua: &mut rilua::Lua) -> LuaResult<()> {
    // TODO: iterate ICON_LIST_DATA from string_data
    let state = lua.state_mut();
    let t = create_table(state);
    set_global_val(state, "ICON_LIST", t);
    Ok(())
}

pub fn register_rilua_color_globals(lua: &mut rilua::Lua) -> LuaResult<()> {
    let state = lua.state_mut();
    for &(name, (r, g, b, a)) in NAMED_COLOR_GLOBALS {
        let color = make_rilua_color_table(state, r, g, b, a)?;
        set_global_val(state, name, color);
    }

    let raid_class_colors = create_table(state);
    for &(class_name, (r, g, b, a)) in RAID_CLASS_COLORS_DATA {
        let color = make_rilua_color_table(state, r, g, b, a)?;
        table_set(state, raid_class_colors, class_name, color);
    }
    set_global_val(state, "RAID_CLASS_COLORS", raid_class_colors);

    let class_color_namespace = create_table(state);
    let Val::Table(class_color_ref) = class_color_namespace else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(state, class_color_ref, "GetClassColor", |state| {
        let class_name = val_to_string(state, Val::from_stack(state, 1)?)
            .unwrap_or_default()
            .to_ascii_uppercase();
        let (r, g, b, a) = RAID_CLASS_COLORS_DATA
            .iter()
            .find(|(name, _)| *name == class_name)
            .map(|(_, color)| *color)
            .unwrap_or((1.0, 1.0, 1.0, 1.0));
        let color = make_rilua_color_table(state, r, g, b, a)?;
        state.push(color);
        Ok(1)
    })?;
    set_global_val(state, "C_ClassColor", class_color_namespace);

    Ok(())
}

// ── Keybinding functions ──────────────────────────────────────────────────────
//
// Keybinding state lives in SimState. Full wiring is a TODO; stubs return
// no-op / empty values so addons don't hard-error on load.

// Keybinding globals moved to `rilua_keybindings.rs`.

// ============================================================================
// Section 3: c_collection_api — C_PetJournal, C_MountJournal, C_ToyBox
//
// These are namespace tables. RustFns access SimState via borrow_state /
// borrow_state_mut (app_data pattern — no captured state in fn pointers).
// ============================================================================

// ── C_PetJournal ─────────────────────────────────────────────────────────────

pub fn register_rilua_pet_journal(lua: &mut rilua::Lua) -> LuaResult<()> {
    let t = TableBuilder::new(lua.state_mut())
        .set_function("GetNumPets", |state| {
            let st = borrow_state(state)?;
            let total = st.world.pets.len() as i32;
            let owned = st.world.pets.iter().filter(|p| p.is_collected).count() as i32;
            drop(st);
            (total, owned).into_stack(state)
        })?
        .set_function("GetNumCollectedInfo", |state| {
            let st = borrow_state(state)?;
            let owned = st.world.pets.iter().filter(|p| p.is_collected).count() as i32;
            let total = st.world.pets.len() as i32;
            drop(st);
            (owned, total).into_stack(state)
        })?
        .set_function("GetNumPetsNeedingFanfare", |state| (0i32).into_stack(state))?
        .set_function("GetPetInfoByIndex", |state| {
            let index = i32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let i = (index - 1) as usize;
            let Some(p) = st.world.pets.get(i) else {
                drop(st);
                return Ok(0);
            };
            let species = p.species_id as f64;
            let level = p.level as f64;
            let icon = p.icon as f64;
            let pet_type = p.pet_type as f64;
            let name_str = p.name.clone();
            drop(st);
            // speciesId, nil, level, 0, 0, 0, false, name, icon, petType, 0, "", "", false, true, false, false
            state.push(Val::Num(species));
            state.push(Val::Nil);
            state.push(Val::Num(level));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            let name_val = create_string(state, &name_str);
            let empty_1 = create_string(state, "");
            let empty_2 = create_string(state, "");
            state.push(Val::Bool(false));
            state.push(name_val);
            state.push(Val::Num(icon));
            state.push(Val::Num(pet_type));
            state.push(Val::Num(0.0));
            state.push(empty_1);
            state.push(empty_2);
            state.push(Val::Bool(false));
            state.push(Val::Bool(true));
            state.push(Val::Bool(false));
            state.push(Val::Bool(false));
            Ok(17)
        })?
        .set_function("GetPetInfoByPetID", |_state| {
            // TODO: lookup by pet_id string
            Ok(0)
        })?
        .set_function("GetPetInfoBySpeciesID", |_state| {
            // TODO: lookup by species_id
            Ok(0)
        })?
        .set_function("PetIsSummonable", |state| false.into_stack(state))?
        .build();

    set_global_val(lua.state_mut(), "C_PetJournal", t);
    Ok(())
}

// ── C_MountJournal ───────────────────────────────────────────────────────────

pub fn register_rilua_mount_journal(lua: &mut rilua::Lua) -> LuaResult<()> {
    let t = TableBuilder::new(lua.state_mut())
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
            let name_val = create_string(state, &name);
            state.push(name_val);
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
            let name_val = create_string(state, &name);
            state.push(name_val);
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
            let empty = create_string(state, "");
            let source = create_string(state, "Drop");
            state.push(Val::Num(0.0));
            state.push(empty);
            state.push(source);
            state.push(Val::Bool(false));
            state.push(Val::Num(mount_type));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Bool(false));
            Ok(9)
        })?
        .set_function("GetMountIDs", |state| create_table(state).into_stack(state))?
        .set_function("GetNumMountsNeedingFanfare", |state| {
            (0i32).into_stack(state)
        })?
        .set_function("GetCollectedFilterSetting", |state| true.into_stack(state))?
        .set_function("SetCollectedFilterSetting", |_state| Ok(0))?
        .set_function("GetIsFavorite", |state| (false, false).into_stack(state))?
        .set_function("SetIsFavorite", |_state| Ok(0))?
        .set_function("Summon", |_state| Ok(0))?
        .set_function("Dismiss", |_state| Ok(0))?
        .build();

    set_global_val(lua.state_mut(), "C_MountJournal", t);
    Ok(())
}

// ── C_ToyBox ─────────────────────────────────────────────────────────────────

pub fn register_rilua_toy_box(lua: &mut rilua::Lua) -> LuaResult<()> {
    let t = TableBuilder::new(lua.state_mut())
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
            let name_val = create_string(state, &name);
            state.push(Val::Num(tid));
            state.push(name_val);
            state.push(Val::Num(icon));
            state.push(Val::Bool(false));
            state.push(Val::Bool(false));
            state.push(Val::Num(1.0));
            Ok(6)
        })?
        .set_function("IsToyUsable", |state| {
            let item_id = i32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let usable = st
                .world
                .toys
                .iter()
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
            let link = st
                .world
                .toys
                .iter()
                .find(|t| t.item_id == item_id as u32)
                .map(|toy| {
                    format!(
                        "|cff0070dd|Hitem:{}::::::::1:0|h[{}]|h|r",
                        toy.item_id, toy.name
                    )
                });
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
        .set_function("GetCollectedShown", |state| true.into_stack(state))?
        .set_function("GetUncollectedShown", |state| true.into_stack(state))?
        .set_function("GetUnusableShown", |state| true.into_stack(state))?
        .set_function("SetCollectedShown", |_state| Ok(0))?
        .set_function("SetUncollectedShown", |_state| Ok(0))?
        .set_function("SetUnusableShown", |_state| Ok(0))?
        .set_function("ForceToyRefilter", |_state| Ok(0))?
        .build();

    set_global_val(lua.state_mut(), "C_ToyBox", t);
    Ok(())
}

// ============================================================================
// register_all — main entry point
// ============================================================================

/// Register all font, string-table, and collection API globals on the rilua VM.
pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    // Font API
    LuaApiMut::register_function(lua, "CreateFont", create_font)?;
    LuaApiMut::register_function(lua, "GetFonts", get_fonts)?;
    LuaApiMut::register_function(lua, "GetFontInfo", get_font_info)?;
    LuaApiMut::register_function(lua, "CreateFontFamily", create_font_family)?;
    register_standard_font_objects(lua)?;

    // String/UI table globals (Lua-table-backed; pure string constants stay mlua)
    register_rilua_tooltip_colors(lua)?;
    register_rilua_item_quality_colors(lua)?;
    register_rilua_class_name_tables(lua)?;
    register_rilua_icon_list(lua)?;
    register_rilua_color_globals(lua)?;
    super::rilua_keybindings::register_all(lua)?;

    // Collection namespaces
    register_rilua_pet_journal(lua)?;
    register_rilua_mount_journal(lua)?;
    register_rilua_toy_box(lua)?;

    Ok(())
}
