use mlua::{Lua, Result};

pub(super) fn register_missing_constants(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_bag_constants(lua, g)?;
    register_chat_constants(lua, g)?;
    register_deprecated_garrison_constants(g)?;
    register_deprecated_item_quality_constants(g)?;
    register_deprecated_wow_token_constants(g)?;
    register_deprecated_world_elapsed_timer_constants(g)?;
    register_material_color_constants(lua)?;
    // Defined in Blizzard_UIParent/Mainline/UIParent.lua but needed earlier
    // by Blizzard_GameTooltip which loads before Blizzard_UIParent.
    g.set("TOOLTIP_UPDATE_TIME", 0.2f64)?;
    Ok(())
}

/// Engine-defined material text/title color globals used by Blizzard_Colors.
/// Built as raw tables with GetRGB/GetRGBA methods since CreateColor isn't
/// available yet (it's defined in Blizzard_SharedXMLBase which loads later).
fn register_material_color_constants(lua: &Lua) -> Result<()> {
    lua.load(COLOR_HELPER_LUA).exec()?;
    lua.load(MATERIAL_COLOR_LUA).exec()?;
    lua.load(QUALITY_COLOR_LUA).exec()
}

/// Temporary color builder used before CreateColor is available.
const COLOR_HELPER_LUA: &str = r#"
    local function _EngineColor(r, g, b, a)
        return {
            r = r, g = g, b = b, a = a or 1,
            GetRGB = function(self) return self.r, self.g, self.b end,
            GetRGBA = function(self) return self.r, self.g, self.b, self.a end,
            GetRGBAsBytes = function(self)
                return math.floor(self.r * 255 + 0.5),
                       math.floor(self.g * 255 + 0.5),
                       math.floor(self.b * 255 + 0.5)
            end,
            GenerateHexColor = function(self)
                return string.format("ff%02x%02x%02x",
                    math.floor(self.r * 255 + 0.5),
                    math.floor(self.g * 255 + 0.5),
                    math.floor(self.b * 255 + 0.5))
            end,
            GenerateHexColorMarkup = function(self)
                return "|c" .. self:GenerateHexColor()
            end,
            WrapTextInColorCode = function(self, text)
                return self:GenerateHexColorMarkup() .. text .. "|r"
            end,
        }
    end
    _G._EngineColor = _EngineColor
"#;

const MATERIAL_COLOR_LUA: &str = r#"
    local C = _EngineColor
    DEFAULT_MATERIAL_TEXT_COLOR = C(0.18, 0.12, 0.06)
    STONE_MATERIAL_TEXT_COLOR = C(0.18, 0.12, 0.06)
    PARCHMENT_MATERIAL_TEXT_COLOR = C(0.18, 0.12, 0.06)
    MARBLE_MATERIAL_TEXT_COLOR = C(0.18, 0.12, 0.06)
    SILVER_MATERIAL_TEXT_COLOR = C(0.18, 0.12, 0.06)
    BRONZE_MATERIAL_TEXT_COLOR = C(0.18, 0.12, 0.06)
    PARCHMENTLARGE_MATERIAL_TEXT_COLOR = C(0.18, 0.12, 0.06)
    PROGENITOR_MATERIAL_TEXT_COLOR = C(0.18, 0.12, 0.06)

    DEFAULT_MATERIAL_TITLETEXT_COLOR = C(0.00, 0.00, 0.00)
    STONE_MATERIAL_TITLETEXT_COLOR = C(0.00, 0.00, 0.00)
    PARCHMENT_MATERIAL_TITLETEXT_COLOR = C(0.00, 0.00, 0.00)
    MARBLE_MATERIAL_TITLETEXT_COLOR = C(0.00, 0.00, 0.00)
    SILVER_MATERIAL_TITLETEXT_COLOR = C(0.00, 0.00, 0.00)
    BRONZE_MATERIAL_TITLETEXT_COLOR = C(0.00, 0.00, 0.00)
    PARCHMENTLARGE_MATERIAL_TITLETEXT_COLOR = C(0.00, 0.00, 0.00)
    PROGENITOR_MATERIAL_TITLETEXT_COLOR = C(0.00, 0.00, 0.00)
"#;

const QUALITY_COLOR_LUA: &str = r#"
    local C = _EngineColor
    COMMON_GRAY_COLOR = C(0.62, 0.62, 0.62)
    UNCOMMON_GREEN_COLOR = C(0.12, 1.00, 0.00)
    RARE_BLUE_COLOR = C(0.00, 0.44, 0.87)
    EPIC_PURPLE_COLOR = C(0.64, 0.21, 0.93)
    LEGENDARY_ORANGE_COLOR = C(1.00, 0.50, 0.00)
    ARTIFACT_GOLD_COLOR = C(0.90, 0.80, 0.50)
    HEIRLOOM_BLUE_COLOR = C(0.00, 0.80, 1.00)
"#;

fn register_bag_constants(_lua: &Lua, g: &mlua::Table) -> Result<()> {
    // BACKPACK_CONTAINER = Enum.BagIndex.Backpack = 0
    g.set("BACKPACK_CONTAINER", 0i32)?;
    // NUM_BAG_SLOTS + NUM_REAGENTBAG_SLOTS
    g.set("NUM_BAG_SLOTS", 4i32)?;
    g.set("NUM_REAGENTBAG_SLOTS", 1i32)?;
    g.set("NUM_TOTAL_EQUIPPED_BAG_SLOTS", 5i32)?;
    Ok(())
}

fn register_chat_constants(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let cfc = lua.create_table()?;
    cfc.set("MaxCharacterNameBytes", 305i32)?;
    cfc.set("MaxChatChannels", 20i32)?;
    cfc.set("MaxChatWindows", 10i32)?;
    cfc.set("ScrollToBottomFlashInterval", 0.5f64)?;
    cfc.set("WhisperSoundAlertCooldown", 3.0f64)?;
    cfc.set("TruncatedCommunityNameLength", 12i32)?;
    cfc.set("TruncatedCommunityNameWithoutChannelLength", 24i32)?;
    cfc.set("MaxRememberedWhisperTargets", 10i32)?;
    g.set("ChatFrameConstants", cfc)?;
    g.set("MAX_CHARACTER_NAME_BYTES", 305i32)?;
    g.set("MAX_COMMUNITY_NAME_LENGTH", 12i32)?;
    g.set("MAX_COMMUNITY_NAME_LENGTH_NO_CHANNEL", 24i32)?;

    let mfsb = lua.create_table()?;
    mfsb.set("InitialScrollDelay", 0.4f64)?;
    mfsb.set("HeldScrollDelay", 0.04f64)?;
    g.set("MessageFrameScrollButtonConstants", mfsb)?;
    Ok(())
}

fn register_deprecated_garrison_constants(g: &mlua::Table) -> Result<()> {
    g.set("LE_FOLLOWER_MISSION_COMPLETE_STATE_ALIVE", 1i32)?;
    g.set("LE_FOLLOWER_MISSION_COMPLETE_STATE_SAVED", 3i32)?;
    g.set("LE_FOLLOWER_TYPE_GARRISON_7_0", 4i32)?;
    Ok(())
}

fn register_deprecated_item_quality_constants(g: &mlua::Table) -> Result<()> {
    g.set("LE_ITEM_QUALITY_COMMON", 1i32)?;
    g.set("LE_ITEM_QUALITY_UNCOMMON", 2i32)?;
    g.set("LE_ITEM_QUALITY_RARE", 3i32)?;
    g.set("LE_ITEM_QUALITY_EPIC", 4i32)?;
    g.set("LE_ITEM_QUALITY_LEGENDARY", 5i32)?;
    g.set("LE_ITEM_QUALITY_ARTIFACT", 6i32)?;
    g.set("LE_ITEM_QUALITY_HEIRLOOM", 7i32)?;
    g.set("LE_ITEM_QUALITY_WOW_TOKEN", 8i32)?;
    Ok(())
}

fn register_deprecated_wow_token_constants(g: &mlua::Table) -> Result<()> {
    g.set("LE_TOKEN_REDEEM_TYPE_GAME_TIME", 1i32)?;
    g.set("LE_TOKEN_REDEEM_TYPE_BALANCE", 2i32)?;
    g.set("LE_TOKEN_RESULT_ERROR_BALANCE_NEAR_CAP", 10i32)?;
    Ok(())
}

fn register_deprecated_world_elapsed_timer_constants(g: &mlua::Table) -> Result<()> {
    g.set("LE_WORLD_ELAPSED_TIMER_TYPE_NONE", 0i32)?;
    g.set("LE_WORLD_ELAPSED_TIMER_TYPE_CHALLENGE_MODE", 1i32)?;
    g.set("LE_WORLD_ELAPSED_TIMER_TYPE_PROVING_GROUND", 2i32)?;
    Ok(())
}

