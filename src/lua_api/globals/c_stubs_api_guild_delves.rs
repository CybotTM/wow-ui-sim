//! Zone ability, autocomplete, photo sharing, and misc global stubs.
//!
//! Split from c_stubs_api_social.rs. Contains:
//! - C_GuildBank, C_PetBattles (via c_stubs_api_pet_battles)
//! - C_DelvesUI (via c_stubs_api_delves)
//! - C_ZoneAbility, C_AutoComplete, C_PhotoSharing
//! - Misc global stubs (totems, parental controls, etc.)

pub(super) use super::c_stubs_api_delves::register_c_delves_ui;
pub(super) use super::c_stubs_api_pet_battles::register_guild_bank_pet_battles;

use mlua::{Lua, Result, Value};


/// C_ZoneAbility namespace - zone ability data.
pub(super) fn register_c_zone_ability(lua: &Lua) -> Result<()> {
    lua.load(ZONE_ABILITY_LUA).exec()?;
    let zone_ability = lua.globals().get::<mlua::Table>("C_ZoneAbility")?;
    lua.globals().set("C_ZoneAbility", zone_ability)?;
    Ok(())
}

const ZONE_ABILITY_LUA: &str = r#"
    C_ZoneAbility = C_ZoneAbility or {}
    local api = C_ZoneAbility

    api._state = api._state or {
        activeAbilities = {
            {
                zoneAbilityID = 1,
                uiPriority = 1,
                spellID = 372610,
                textureKit = nil,
                tutorialText = "Skyward Ascent",
            },
        },
        iconsBySpellID = {},
        defaultIcon = "Interface\\Icons\\INV_Misc_QuestionMark",
    }

    local function copyAbility(ability)
        if type(ability) ~= "table" then
            return nil
        end

        local copy = {}
        for key, value in pairs(ability) do
            copy[key] = value
        end
        return copy
    end

    local function resolveSpellTexture(spellID)
        if type(C_Spell) ~= "table" or type(C_Spell.GetSpellTexture) ~= "function" then
            return nil
        end

        local ok, texture = pcall(C_Spell.GetSpellTexture, spellID)
        if not ok or texture == nil or texture == "" then
            return nil
        end
        return texture
    end

    api.GetActiveAbilities = api.GetActiveAbilities or function()
        local abilities = api._state.activeAbilities or {}
        local copy = {}
        for index, ability in ipairs(abilities) do
            copy[index] = copyAbility(ability)
        end
        return copy
    end

    api.GetZoneAbilityIcon = api.GetZoneAbilityIcon or function(spellID)
        local iconsBySpellID = api._state.iconsBySpellID or {}
        local seededIcon = iconsBySpellID[spellID]
        if seededIcon ~= nil and seededIcon ~= "" then
            return seededIcon
        end

        local spellTexture = resolveSpellTexture(spellID)
        if spellTexture ~= nil then
            return spellTexture
        end

        return api._state.defaultIcon
    end
"#;

/// C_AutoComplete namespace - player name autocomplete results.
pub(super) fn register_c_auto_complete(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetAutoCompleteResults",
        lua.create_function(|lua, _args: mlua::MultiValue| lua.create_table())?,
    )?;
    t.set(
        "GetAutoCompletePresenceID",
        lua.create_function(|_, _name: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetAutoCompleteRealms",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "IsRecognizedName",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?,
    )?;
    g.set("C_AutoComplete", t)
}

/// C_PhotoSharing namespace - social photo sharing feature.
pub(super) fn register_c_photo_sharing(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsAuthorized", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "BeginAuthorizationFlow",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    t.set(
        "CompleteAuthorizationFlow",
        lua.create_function(|_, _url: Value| Ok(()))?,
    )?;
    t.set("ClearAuthorization", lua.create_function(|_, ()| Ok(()))?)?;
    t.set(
        "GetPhotoSharingAuthURL",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set("GetCropRatio", lua.create_function(|_, ()| Ok(1.0f64))?)?;
    t.set(
        "SetScreenshotPreviewTexture",
        lua.create_function(|_, _frame: Value| Ok(()))?,
    )?;
    t.set(
        "UploadPhotoToService",
        lua.create_function(|_, (_title, _desc): (Value, Value)| Ok(()))?,
    )?;
    t.set("GetStatus", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    g.set("C_PhotoSharing", t)
}

/// AutoComplete-related global function stubs.
pub(super) fn register_auto_complete_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "AutoCompleteEditBox_SetCustomAutoCompleteFunction",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    g.set(
        "AutoCompleteEditBox_SetAutoCompleteSource",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    Ok(())
}

/// Misc global stubs: guild info, totems, parental controls, item text.
pub(super) fn register_misc_global_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("CanEditGuildInfo", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("IsCpuBound", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "GetTotemCannotDismiss",
        lua.create_function(|_, _slot: i32| Ok(false))?,
    )?;
    g.set(
        "GetTotemTimeLeft",
        lua.create_function(|_, _slot: i32| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetSecondsUntilParentalControlsKick",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "ItemTextHasNextPage",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    Ok(())
}
