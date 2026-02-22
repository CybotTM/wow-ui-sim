//! Font API functions for WoW UI simulation.
//!
//! Contains CreateFont, CreateFontFamily, GetFonts, GetFontInfo, and
//! standard WoW font object creation.

use crate::lua_api::script_helpers::lua_error;
use mlua::{Lua, Result, Value};

/// Set default font properties on a table.
fn set_font_defaults(font: &mlua::Table, name: Option<&str>) -> Result<()> {
    font.set("__fontPath", "Fonts\\FRIZQT__.TTF")?;
    font.set("__fontHeight", 12.0)?;
    font.set("__fontFlags", "")?;
    font.set("__textColorR", 1.0)?;
    font.set("__textColorG", 1.0)?;
    font.set("__textColorB", 1.0)?;
    font.set("__textColorA", 1.0)?;
    font.set("__shadowColorR", 0.0)?;
    font.set("__shadowColorG", 0.0)?;
    font.set("__shadowColorB", 0.0)?;
    font.set("__shadowColorA", 0.0)?;
    font.set("__shadowOffsetX", 0.0)?;
    font.set("__shadowOffsetY", 0.0)?;
    font.set("__justifyH", "CENTER")?;
    font.set("__justifyV", "MIDDLE")?;
    font.set("__name", name)?;
    Ok(())
}

/// Add all standard font methods to a table (SetFont, GetFont, etc.).
fn add_font_methods(lua: &Lua, font: &mlua::Table) -> Result<()> {
    add_font_path_methods(lua, font)?;
    add_font_color_methods(lua, font)?;
    add_font_shadow_methods(lua, font)?;
    add_font_justify_methods(lua, font)?;
    add_font_misc_methods(lua, font)?;
    add_font_type_methods(lua, font)?;
    Ok(())
}

/// SetFont, GetFont.
fn add_font_path_methods(lua: &Lua, font: &mlua::Table) -> Result<()> {
    font.set(
        "SetFont",
        lua.create_function(
            |_, (this, path, height, flags): (mlua::Table, String, f64, Option<String>)| {
                this.set("__fontPath", path)?;
                this.set("__fontHeight", height)?;
                this.set("__fontFlags", flags.unwrap_or_default())?;
                Ok(())
            },
        )?,
    )?;

    font.set(
        "GetFont",
        lua.create_function(|lua, this: mlua::Table| -> Result<mlua::MultiValue> {
            let path: Option<String> = this.get("__fontPath")?;
            let height: f64 = this.get::<f64>("__fontHeight").unwrap_or(0.0);
            let flags: String = this.get::<String>("__fontFlags").unwrap_or_default();
            Ok(mlua::MultiValue::from_vec(vec![
                match path {
                    Some(p) => Value::String(lua.create_string(&p)?),
                    None => Value::Nil,
                },
                Value::Number(height),
                Value::String(lua.create_string(&flags)?),
            ]))
        })?,
    )?;

    Ok(())
}

/// SetTextColor, GetTextColor.
fn add_font_color_methods(lua: &Lua, font: &mlua::Table) -> Result<()> {
    font.set(
        "SetTextColor",
        lua.create_function(
            |_, (this, r, g, b, a): (mlua::Table, f64, f64, f64, Option<f64>)| {
                this.set("__textColorR", r)?;
                this.set("__textColorG", g)?;
                this.set("__textColorB", b)?;
                this.set("__textColorA", a.unwrap_or(1.0))?;
                Ok(())
            },
        )?,
    )?;

    font.set(
        "GetTextColor",
        lua.create_function(|_, this: mlua::Table| {
            Ok((
                this.get::<f64>("__textColorR")?,
                this.get::<f64>("__textColorG")?,
                this.get::<f64>("__textColorB")?,
                this.get::<f64>("__textColorA")?,
            ))
        })?,
    )?;

    Ok(())
}

/// SetShadowColor, GetShadowColor, SetShadowOffset, GetShadowOffset.
fn add_font_shadow_methods(lua: &Lua, font: &mlua::Table) -> Result<()> {
    font.set(
        "SetShadowColor",
        lua.create_function(
            |_, (this, r, g, b, a): (mlua::Table, f64, f64, f64, Option<f64>)| {
                this.set("__shadowColorR", r)?;
                this.set("__shadowColorG", g)?;
                this.set("__shadowColorB", b)?;
                this.set("__shadowColorA", a.unwrap_or(1.0))?;
                Ok(())
            },
        )?,
    )?;

    font.set(
        "GetShadowColor",
        lua.create_function(|_, this: mlua::Table| {
            Ok((
                this.get::<f64>("__shadowColorR")?,
                this.get::<f64>("__shadowColorG")?,
                this.get::<f64>("__shadowColorB")?,
                this.get::<f64>("__shadowColorA")?,
            ))
        })?,
    )?;

    font.set(
        "SetShadowOffset",
        lua.create_function(|_, (this, x, y): (mlua::Table, f64, f64)| {
            this.set("__shadowOffsetX", x)?;
            this.set("__shadowOffsetY", y)?;
            Ok(())
        })?,
    )?;

    font.set(
        "GetShadowOffset",
        lua.create_function(|_, this: mlua::Table| {
            Ok((
                this.get::<f64>("__shadowOffsetX")?,
                this.get::<f64>("__shadowOffsetY")?,
            ))
        })?,
    )?;

    Ok(())
}

/// SetJustifyH, GetJustifyH, SetJustifyV, GetJustifyV, SetSpacing, GetSpacing.
fn add_font_justify_methods(lua: &Lua, font: &mlua::Table) -> Result<()> {
    font.set(
        "SetJustifyH",
        lua.create_function(|_, (this, justify): (mlua::Table, String)| {
            this.set("__justifyH", justify)?;
            Ok(())
        })?,
    )?;
    font.set(
        "GetJustifyH",
        lua.create_function(|_, this: mlua::Table| this.get::<String>("__justifyH"))?,
    )?;
    font.set(
        "SetJustifyV",
        lua.create_function(|_, (this, justify): (mlua::Table, String)| {
            this.set("__justifyV", justify)?;
            Ok(())
        })?,
    )?;
    font.set(
        "GetJustifyV",
        lua.create_function(|_, this: mlua::Table| this.get::<String>("__justifyV"))?,
    )?;
    font.set(
        "SetSpacing",
        lua.create_function(|_, (this, spacing): (mlua::Table, f64)| {
            this.set("__spacing", spacing)?;
            Ok(())
        })?,
    )?;
    font.set(
        "GetSpacing",
        lua.create_function(|_, this: mlua::Table| {
            Ok(this.get::<f64>("__spacing").unwrap_or(0.0))
        })?,
    )?;
    Ok(())
}

/// GetName, GetFontObjectForAlphabet, CopyFontObject.
fn add_font_misc_methods(lua: &Lua, font: &mlua::Table) -> Result<()> {
    font.set(
        "GetName",
        lua.create_function(|_, this: mlua::Table| {
            Ok(this.get::<Option<String>>("__name").ok().flatten())
        })?,
    )?;

    font.set(
        "GetFontObjectForAlphabet",
        lua.create_function(|_, this: mlua::Table| Ok(this))?,
    )?;

    font.set(
        "CopyFontObject",
        lua.create_function(|lua, (this, src): (mlua::Table, Value)| {
            let src_table: Option<mlua::Table> = match src {
                Value::String(name) => lua
                    .globals()
                    .get::<Option<mlua::Table>>(name.to_string_lossy().to_string())
                    .ok()
                    .flatten(),
                Value::Table(t) => Some(t),
                _ => None,
            };
            if let Some(src) = src_table {
                copy_font_properties(&this, &src)?;
            }
            Ok(())
        })?,
    )?;

    Ok(())
}

/// GetObjectType, IsObjectType, GetFontObject, SetFontObject (on the Font object itself).
fn add_font_type_methods(lua: &Lua, font: &mlua::Table) -> Result<()> {
    font.set(
        "GetObjectType",
        lua.create_function(|lua, _this: mlua::Table| {
            Ok(Value::String(lua.create_string("Font")?))
        })?,
    )?;

    font.set(
        "IsObjectType",
        lua.create_function(|_, (_this, type_name): (mlua::Table, String)| {
            Ok(type_name.eq_ignore_ascii_case("Font"))
        })?,
    )?;

    // Font:GetFontObject() returns the font object set via SetFontObject.
    font.set(
        "GetFontObject",
        lua.create_function(|_, this: mlua::Table| {
            let fo: Value = this.get("__fontObject")?;
            Ok(fo)
        })?,
    )?;

    // Font:SetFontObject(target) — set inherited font object with cycle detection.
    font.set(
        "SetFontObject",
        lua.create_function(|lua, (this, target): (mlua::Table, mlua::Table)| {
            let name: String = this
                .get::<String>("__name")
                .unwrap_or_else(|_| "Font".to_string());
            // Walk the target's __fontObject chain looking for this table.
            let mut current: Value = Value::Table(target.clone());
            loop {
                match current {
                    Value::Table(ref t) => {
                        if *t == this {
                            return Err(lua_error(
                                lua,
                                format!(
                                    "{}:SetFontObject(): Can't create a font object loop",
                                    name
                                ),
                            ));
                        }
                        current = t.get::<Value>("__fontObject").unwrap_or(Value::Nil);
                    }
                    _ => break,
                }
            }
            this.set("__fontObject", target)?;
            Ok(())
        })?,
    )?;

    Ok(())
}

/// Create a bare Font object with no font path set (for MessageFrame internal fonts).
///
/// Returns a Lua table with all Font methods but `GetFont()` returns `(nil, 0, "")`.
pub fn create_bare_font(lua: &Lua) -> Result<mlua::Table> {
    let font = lua.create_table()?;
    // No __fontPath set — GetFont will return (nil, 0, "")
    font.set("__fontHeight", 0.0)?;
    font.set("__fontFlags", "")?;
    font.set("__textColorR", 1.0)?;
    font.set("__textColorG", 1.0)?;
    font.set("__textColorB", 1.0)?;
    font.set("__textColorA", 1.0)?;
    font.set("__shadowColorR", 0.0)?;
    font.set("__shadowColorG", 0.0)?;
    font.set("__shadowColorB", 0.0)?;
    font.set("__shadowColorA", 0.0)?;
    font.set("__shadowOffsetX", 0.0)?;
    font.set("__shadowOffsetY", 0.0)?;
    font.set("__justifyH", "CENTER")?;
    font.set("__justifyV", "MIDDLE")?;
    add_font_methods(lua, &font)?;
    Ok(font)
}

/// Copy font properties from src table to dst table.
fn copy_font_properties(dst: &mlua::Table, src: &mlua::Table) -> Result<()> {
    if let Ok(v) = src.get::<String>("__fontPath") {
        dst.set("__fontPath", v)?;
    }
    if let Ok(v) = src.get::<f64>("__fontHeight") {
        dst.set("__fontHeight", v)?;
    }
    if let Ok(v) = src.get::<String>("__fontFlags") {
        dst.set("__fontFlags", v)?;
    }
    for key in &[
        "__textColorR",
        "__textColorG",
        "__textColorB",
        "__textColorA",
        "__shadowColorR",
        "__shadowColorG",
        "__shadowColorB",
        "__shadowColorA",
        "__shadowOffsetX",
        "__shadowOffsetY",
    ] {
        if let Ok(v) = src.get::<f64>(*key) {
            dst.set(*key, v)?;
        }
    }
    if let Ok(v) = src.get::<String>("__justifyH") {
        dst.set("__justifyH", v)?;
    }
    if let Ok(v) = src.get::<String>("__justifyV") {
        dst.set("__justifyV", v)?;
    }
    Ok(())
}

/// Register all font API functions.
pub fn register_font_api(lua: &Lua) -> Result<()> {
    register_create_font(lua)?;
    register_get_fonts(lua)?;
    register_get_font_info(lua)?;
    register_create_font_family(lua)?;
    Ok(())
}

/// Register CreateFont global function.
///
/// - Requires a name argument (nil or absent → error).
/// - Coerces numeric names to strings.
/// - Returns the same font object on repeat calls (uses an internal registry).
fn register_create_font(lua: &Lua) -> Result<()> {
    let func = lua.create_function(|lua, name_arg: Value| {
        // Coerce name to string; nil / absent → error.
        let name: String = match &name_arg {
            Value::String(s) => s.to_string_lossy().to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Number(n) => (*n as i64).to_string(),
            _ => {
                return Err(lua_error(lua, "Usage: CreateFont(\"name\")"));
            }
        };

        // Check the internal font registry first so repeat calls return the same object.
        // Always re-register in _G so `_G[name]` is valid even if the caller cleared it.
        let registry: mlua::Table = lua
            .load("_G.__font_registry = _G.__font_registry or {}; return _G.__font_registry")
            .eval()?;
        let existing: Value = registry.get(name.as_str())?;
        if let Value::Table(t) = existing {
            lua.globals().set(name.as_str(), t.clone())?;
            return Ok(t);
        }

        // Create new font, store in registry and _G.
        let font = lua.create_table()?;
        set_font_defaults(&font, Some(&name))?;
        add_font_methods(lua, &font)?;
        registry.set(name.as_str(), font.clone())?;
        lua.globals().set(name.as_str(), font.clone())?;
        Ok(font)
    })?;
    lua.globals().set("CreateFont", func)?;
    Ok(())
}

/// Register GetFonts global function.
fn register_get_fonts(lua: &Lua) -> Result<()> {
    let func = lua.create_function(|lua, ()| lua.create_table())?;
    lua.globals().set("GetFonts", func)?;
    Ok(())
}

/// Register GetFontInfo global function.
fn register_get_font_info(lua: &Lua) -> Result<()> {
    let func = lua.create_function(|lua, font_input: Value| {
        let info = lua.create_table()?;
        let font_obj = resolve_font_info_source(lua, &font_input, &info)?;
        populate_font_info(&info, font_obj.as_ref(), lua)?;
        Ok(info)
    })?;
    lua.globals().set("GetFontInfo", func)?;
    Ok(())
}

/// Resolve font input to a table and set the name on the info table.
fn resolve_font_info_source(
    lua: &Lua,
    font_input: &Value,
    info: &mlua::Table,
) -> Result<Option<mlua::Table>> {
    match font_input {
        Value::String(name) => {
            let name_str = name.to_string_lossy().to_string();
            info.set("name", name_str.clone())?;
            Ok(lua.globals().get::<mlua::Table>(name_str).ok())
        }
        Value::Table(t) => {
            info.set("name", t.get::<String>("__name").unwrap_or_default())?;
            Ok(Some(t.clone()))
        }
        _ => {
            info.set("name", "")?;
            Ok(None)
        }
    }
}

/// Populate font info table from a font object.
fn populate_font_info(info: &mlua::Table, obj: Option<&mlua::Table>, lua: &Lua) -> Result<()> {
    if let Some(obj) = obj {
        info.set("height", obj.get::<f64>("__fontHeight").unwrap_or(12.0))?;
        info.set("outline", obj.get::<String>("__fontFlags").unwrap_or_default())?;
        let color = lua.create_table()?;
        color.set("r", obj.get::<f64>("__textColorR").unwrap_or(1.0))?;
        color.set("g", obj.get::<f64>("__textColorG").unwrap_or(1.0))?;
        color.set("b", obj.get::<f64>("__textColorB").unwrap_or(1.0))?;
        color.set("a", obj.get::<f64>("__textColorA").unwrap_or(1.0))?;
        info.set("color", color)?;
    } else {
        info.set("height", 12.0)?;
        info.set("outline", "")?;
    }
    Ok(())
}

/// Register CreateFontFamily global function.
fn register_create_font_family(lua: &Lua) -> Result<()> {
    let func = lua.create_function(|lua, (name, members): (String, mlua::Table)| {
        let font = lua.create_table()?;
        set_font_defaults(&font, Some(&name))?;
        apply_font_family_member(&font, &members)?;
        add_font_methods(lua, &font)?;
        lua.globals().set(name.as_str(), font.clone())?;
        Ok(font)
    })?;
    lua.globals().set("CreateFontFamily", func)?;
    Ok(())
}

/// Override font defaults from the first member of a font family.
fn apply_font_family_member(font: &mlua::Table, members: &mlua::Table) -> Result<()> {
    if let Ok(first_member) = members.get::<mlua::Table>(1) {
        if let Ok(file) = first_member.get::<String>("file") {
            font.set("__fontPath", file)?;
        }
        if let Ok(height) = first_member.get::<f64>("height") {
            font.set("__fontHeight", height)?;
        }
        if let Ok(flags) = first_member.get::<String>("flags") {
            font.set("__fontFlags", flags)?;
        }
    }
    Ok(())
}

/// Create standard WoW font objects that addons expect to exist.
pub fn create_standard_font_objects(lua: &Lua) -> Result<()> {
    for &(name, height, flags, r, g, b) in STANDARD_FONTS {
        create_font_object(lua, name, height, flags, r, g, b)?;
    }
    Ok(())
}

/// (name, height, flags, r, g, b)
const STANDARD_FONTS: &[(&str, f64, &str, f64, f64, f64)] = &[
    // Gold text
    ("GameFontNormal", 12.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalSmall", 10.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalLarge", 16.0, "", 1.0, 0.82, 0.0),
    ("GameFontNormalHuge", 20.0, "", 1.0, 0.82, 0.0),
    // Highlighted (white)
    ("GameFontHighlight", 12.0, "", 1.0, 1.0, 1.0),
    ("GameFontHighlightSmall", 10.0, "", 1.0, 1.0, 1.0),
    ("GameFontHighlightSmallOutline", 10.0, "OUTLINE", 1.0, 1.0, 1.0),
    ("GameFontHighlightLarge", 16.0, "", 1.0, 1.0, 1.0),
    ("GameFontHighlightHuge", 20.0, "", 1.0, 1.0, 1.0),
    ("GameFontHighlightOutline", 12.0, "OUTLINE", 1.0, 1.0, 1.0),
    // Disabled (gray)
    ("GameFontDisable", 12.0, "", 0.5, 0.5, 0.5),
    ("GameFontDisableSmall", 10.0, "", 0.5, 0.5, 0.5),
    ("GameFontDisableLarge", 16.0, "", 0.5, 0.5, 0.5),
    // Red
    ("GameFontRed", 12.0, "", 1.0, 0.1, 0.1),
    ("GameFontRedSmall", 10.0, "", 1.0, 0.1, 0.1),
    ("GameFontRedLarge", 16.0, "", 1.0, 0.1, 0.1),
    // Green
    ("GameFontGreen", 12.0, "", 0.1, 1.0, 0.1),
    ("GameFontGreenSmall", 10.0, "", 0.1, 1.0, 0.1),
    ("GameFontGreenLarge", 16.0, "", 0.1, 1.0, 0.1),
    // White
    ("GameFontWhite", 12.0, "", 1.0, 1.0, 1.0),
    ("GameFontWhiteSmall", 10.0, "", 1.0, 1.0, 1.0),
    ("GameFontWhiteTiny", 9.0, "", 1.0, 1.0, 1.0),
    // Black
    ("GameFontBlack", 12.0, "", 0.0, 0.0, 0.0),
    ("GameFontBlackSmall", 10.0, "", 0.0, 0.0, 0.0),
    // Number fonts
    ("NumberFontNormal", 14.0, "OUTLINE", 1.0, 1.0, 1.0),
    ("NumberFontNormalSmall", 12.0, "OUTLINE", 1.0, 1.0, 1.0),
    ("NumberFontNormalLarge", 16.0, "OUTLINE", 1.0, 1.0, 1.0),
    ("NumberFontNormalHuge", 24.0, "OUTLINE", 1.0, 1.0, 1.0),
    ("NumberFontNormalRightRed", 14.0, "OUTLINE", 1.0, 0.1, 0.1),
    ("NumberFontNormalRightYellow", 14.0, "OUTLINE", 1.0, 1.0, 0.0),
    // Chat fonts
    ("ChatFontNormal", 14.0, "", 1.0, 1.0, 1.0),
    ("ChatFontSmall", 12.0, "", 1.0, 1.0, 1.0),
    // System fonts
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
    // Tooltip fonts
    ("GameTooltipHeader", 14.0, "", 1.0, 1.0, 1.0),
    ("GameTooltipText", 12.0, "", 1.0, 1.0, 1.0),
    ("GameTooltipTextSmall", 10.0, "", 1.0, 1.0, 1.0),
    // Subzone fonts
    ("SubZoneTextFont", 26.0, "OUTLINE", 1.0, 0.82, 0.0),
    ("PVPInfoTextFont", 20.0, "OUTLINE", 1.0, 0.1, 0.1),
    // Misc fonts
    ("FriendsFont_Normal", 12.0, "", 1.0, 1.0, 1.0),
    ("FriendsFont_Small", 10.0, "", 1.0, 1.0, 1.0),
    ("FriendsFont_Large", 14.0, "", 1.0, 1.0, 1.0),
    ("FriendsFont_UserText", 11.0, "", 1.0, 1.0, 1.0),
];

/// Create a single named font object with specific properties and register it globally.
fn create_font_object(
    lua: &Lua,
    name: &str,
    height: f64,
    flags: &str,
    r: f64,
    g: f64,
    b: f64,
) -> Result<()> {
    let font = lua.create_table()?;
    set_font_defaults(&font, Some(name))?;
    font.set("__fontHeight", height)?;
    font.set("__fontFlags", flags)?;
    font.set("__textColorR", r)?;
    font.set("__textColorG", g)?;
    font.set("__textColorB", b)?;
    add_font_methods(lua, &font)?;
    lua.globals().set(name, font)?;
    Ok(())
}
