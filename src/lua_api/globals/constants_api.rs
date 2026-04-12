//! WoW Constants table containing game constant namespaces.
//!
//! This module registers the global `Constants` table. Unlike `Enum`, which
//! contains enumerations, `Constants` contains named constant values grouped
//! into namespaces (e.g., `Constants.LFG_ROLEConstants.LFG_ROLE_NO_ROLE`).
//!
//! The table uses auto-vivifying metatables so that accessing undefined
//! subtables returns an empty table instead of nil, preventing crashes
//! when Blizzard code references constants we haven't stubbed yet.

use mlua::{Lua, Result};

/// Auto-generated Lua code that registers missing WoW client constants.
const MISSING_CONSTANTS_LUA: &str = include_str!("enum_data/missing_constants.lua");

/// Auto-generated Constants.* values from wowless globals.yaml.
const CONSTANTS_VALUES_LUA: &str = include_str!("enum_data/constants_values.lua");

/// Register the Constants table with auto-vivifying metatables.
pub fn register_constants_api(lua: &Lua) -> Result<()> {
    register_constants_table(lua)?;
    register_color_globals(lua)?;
    register_raid_class_colors(lua)?;
    // Load auto-generated missing constants from the WoW client diff snapshot.
    lua.load(MISSING_CONSTANTS_LUA)
        .set_name("missing_constants")
        .exec()?;
    Ok(())
}

/// Create the auto-vivifying Constants table and populate constant namespaces.
fn register_constants_table(lua: &Lua) -> Result<()> {
    create_autovivify_constants(lua)?;
    lua.load(CONSTANTS_VALUES_LUA)
        .set_name("constants_values")
        .exec()
}

/// Create the auto-vivifying Constants global table.
fn create_autovivify_constants(lua: &Lua) -> Result<()> {
    lua.load(
        r#"
        local function make_autovivify()
            local mt = {
                __index = function(t, k)
                    local sub = setmetatable({}, getmetatable(t))
                    rawset(t, k, sub)
                    return sub
                end
            }
            return setmetatable({}, mt)
        end
        Constants = make_autovivify()
    "#,
    )
    .exec()
}

/// Register global color objects (faction colors, font colors, PvP colors).
fn register_color_globals(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    register_color_group(lua, &globals, FACTION_COLOR_GLOBALS)?;
    register_color_group(lua, &globals, FONT_COLOR_GLOBALS)?;
    register_color_group(lua, &globals, PVP_COLOR_GLOBALS)?;
    register_color_group(lua, &globals, MISC_COLOR_GLOBALS)?;
    Ok(())
}

fn register_color_group(
    lua: &Lua,
    globals: &mlua::Table,
    colors: &[(&str, (f64, f64, f64, f64))],
) -> Result<()> {
    for (name, (r, g, b, a)) in colors {
        globals.set(
            *name,
            super::strings::make_color_table(lua, *r, *g, *b, *a)?,
        )?;
    }
    Ok(())
}

const FACTION_COLOR_GLOBALS: &[(&str, (f64, f64, f64, f64))] = &[
    (
        "PLAYER_FACTION_COLOR_HORDE",
        (0.90196, 0.05098, 0.07059, 1.0),
    ),
    (
        "PLAYER_FACTION_COLOR_ALLIANCE",
        (0.29412, 0.33333, 0.91373, 1.0),
    ),
];

const FONT_COLOR_GLOBALS: &[(&str, (f64, f64, f64, f64))] = &[
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
];

const PVP_COLOR_GLOBALS: &[(&str, (f64, f64, f64, f64))] = &[
    ("PVP_SCOREBOARD_HORDE_CELL_COLOR", (1.0, 0.18, 0.18, 1.0)),
    ("PVP_SCOREBOARD_ALLIANCE_CELL_COLOR", (0.36, 0.45, 1.0, 1.0)),
];

const MISC_COLOR_GLOBALS: &[(&str, (f64, f64, f64, f64))] = &[
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

/// Register `RAID_CLASS_COLORS` - maps class file names to color objects.
fn register_raid_class_colors(lua: &Lua) -> Result<()> {
    lua.load(
        r#"
        local function makeClassColor(r, g, b)
            local c = { r = r, g = g, b = b }
            function c:GetRGB() return self.r, self.g, self.b end
            function c:GetRGBA() return self.r, self.g, self.b, 1.0 end
            function c:GenerateHexColor()
                return string.format("%02x%02x%02x",
                    math.floor(self.r * 255), math.floor(self.g * 255), math.floor(self.b * 255))
            end
            function c:WrapTextInColorCode(text) return "|cff" .. self:GenerateHexColor() .. text .. "|r" end
            return c
        end

        RAID_CLASS_COLORS = {
            WARRIOR     = makeClassColor(0.78, 0.61, 0.43),
            PALADIN     = makeClassColor(0.96, 0.55, 0.73),
            HUNTER      = makeClassColor(0.67, 0.83, 0.45),
            ROGUE       = makeClassColor(1.00, 0.96, 0.41),
            PRIEST      = makeClassColor(1.00, 1.00, 1.00),
            DEATHKNIGHT = makeClassColor(0.77, 0.12, 0.23),
            SHAMAN      = makeClassColor(0.00, 0.44, 0.87),
            MAGE        = makeClassColor(0.25, 0.78, 0.92),
            WARLOCK     = makeClassColor(0.53, 0.53, 0.93),
            MONK        = makeClassColor(0.00, 1.00, 0.60),
            DRUID       = makeClassColor(1.00, 0.49, 0.04),
            DEMONHUNTER = makeClassColor(0.64, 0.19, 0.79),
            EVOKER      = makeClassColor(0.20, 0.58, 0.50),
        }
    "#,
    )
    .exec()
}
