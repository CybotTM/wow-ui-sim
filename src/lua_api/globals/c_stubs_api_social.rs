//! Social, trial, and feature namespace stubs.
//!
//! Split from c_stubs_api_extra.rs. Contains:
//! - C_ClassTrial, C_RecruitAFriend, C_WowTokenPublic, C_FriendList
//! - C_CatalogShop, C_Who, C_PrivateAuras, C_GuildBank, C_PetBattles
//! - C_DelvesUI, C_ZoneAbility, C_AutoComplete, C_PhotoSharing
//! - Misc global stubs (totems, parental controls, etc.)

use mlua::{Lua, Result, Value};

/// Register trial/social/feature namespaces and misc globals.
pub fn register_social_feature_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_trial_raf_token(lua, g)?;
    register_shop_who_auras(lua, g)?;
    register_guild_bank_pet_battles(lua, g)?;
    register_c_delves_ui(lua)?;
    register_c_zone_ability(lua)?;
    register_c_auto_complete(lua, g)?;
    register_c_photo_sharing(lua, g)?;
    register_auto_complete_globals(lua, g)?;
    register_misc_global_stubs(lua)?;
    Ok(())
}

/// C_ClassTrial, C_RecruitAFriend, C_WowTokenPublic, C_FriendList stubs.
fn register_trial_raf_token(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_c_class_trial(lua, g)?;
    register_c_recruit_a_friend(lua, g)?;
    register_c_wow_token_public(lua, g)?;
    register_c_friend_list(lua, g)?;
    Ok(())
}

/// C_ClassTrial stubs.
fn register_c_class_trial(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "IsClassTrialCharacter",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetClassTrialLogoutTimeSeconds",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set("C_ClassTrial", t)
}

/// C_RecruitAFriend stubs.
fn register_c_recruit_a_friend(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetRecruitInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "IsRecruitingEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set("GetRAFInfo", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set(
        "GetRAFSystemInfo",
        lua.create_function(|lua, ()| {
            let info = lua.create_table()?;
            info.set("maxRecruits", 0i32)?;
            info.set("maxRecruitMonths", 0i32)?;
            info.set("maxRewardMonths", 0i32)?;
            info.set("daysInCycle", 30i32)?;
            Ok(info)
        })?,
    )?;
    g.set("C_RecruitAFriend", t)
}

/// C_WowTokenPublic stubs.
fn register_c_wow_token_public(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetCurrentMarketPrice",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set("GetGuaranteedPrice", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("UpdateTokenCount", lua.create_function(|_, ()| Ok(()))?)?;
    t.set(
        "GetCommerceSystemStatus",
        lua.create_function(|_, ()| Ok((false, false, false)))?,
    )?;
    t.set("UpdateMarketPrice", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_WowTokenPublic", t)
}

/// C_FriendList stubs.
fn register_c_friend_list(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("SetWhoToUi", lua.create_function(|_, _flag: bool| Ok(()))?)?;
    t.set("SendWho", lua.create_function(|_, _msg: String| Ok(()))?)?;
    t.set("GetNumWhoResults", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("GetNumFriends", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set(
        "GetNumOnlineFriends",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetFriendInfoByIndex",
        lua.create_function(|_, _idx: i32| Ok(Value::Nil))?,
    )?;
    t.set("ShowFriends", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_FriendList", t)
}

/// C_CatalogShop, C_Who, C_PrivateAuras stubs.
fn register_shop_who_auras(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let catalog_shop = lua.create_table()?;
    catalog_shop.set(
        "GetAvailableCategoryIDs",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    catalog_shop.set("IsShop2Enabled", lua.create_function(|_, ()| Ok(false))?)?;
    catalog_shop.set("HasNewProducts", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_CatalogShop", catalog_shop)?;

    let who = lua.create_table()?;
    who.set("SetWhoToUi", lua.create_function(|_, _flag: bool| Ok(()))?)?;
    who.set("SendWho", lua.create_function(|_, _msg: String| Ok(()))?)?;
    who.set(
        "GetWhoInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    g.set("C_Who", who)?;

    let private_auras = lua.create_table()?;
    private_auras.set(
        "SetPrivateRaidBossMessageCallback",
        lua.create_function(|_, _cb: Value| Ok(()))?,
    )?;
    g.set("C_PrivateAuras", private_auras)?;
    Ok(())
}

/// C_GuildBank, C_PetBattles stubs.
fn register_guild_bank_pet_battles(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let guild_bank = lua.create_table()?;
    guild_bank.set(
        "IsGuildBankEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    guild_bank.set("GetCurrentBankTab", lua.create_function(|_, ()| Ok(1i32))?)?;
    guild_bank.set("FetchNumTabs", lua.create_function(|_, ()| Ok(0i32))?)?;
    g.set("C_GuildBank", guild_bank)?;

    // C_PetBattles - plain table, no metatable (Wowless expects getmetatable == nil).
    let pet = lua.create_table()?;
    pet.set("IsInBattle", lua.create_function(|_, ()| Ok(false))?)?;
    pet.set("IsWildBattle", lua.create_function(|_, ()| Ok(false))?)?;
    pet.set("IsPlayerNPC", lua.create_function(|_, ()| Ok(false))?)?;
    pet.set("GetAllEffectNames", lua.create_function(|_, ()| Ok(()))?)?;
    pet.set(
        "GetAllStates",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    pet.set(
        "GetBattleState",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    pet.set(
        "GetPVPMatchmakingInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set("C_PetBattles", pet)?;
    Ok(())
}

/// C_DelvesUI namespace - Delves companion data.
fn register_c_delves_ui(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetTraitTreeForCompanion",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetRoleNodeForCompanion",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetRoleSubtreeForCompanion",
        lua.create_function(|_, _role_type: Value| Ok(0i32))?,
    )?;
    t.set(
        "GetCreatureDisplayInfoForCompanion",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetCurioNodeForCompanion",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetCurrentDelvesSeasonNumber",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    t.set(
        "GetDelvesMinRequiredLevel",
        lua.create_function(|_, ()| Ok(80i32))?,
    )?;
    t.set(
        "GetFactionForCompanion",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set("HasActiveDelve", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetUnseenCuriosBySlotType",
        lua.create_function(|lua, _slot_type: Value| lua.create_table())?,
    )?;
    t.set(
        "GetDelvesFactionForSeason",
        lua.create_function(|_, _season: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "RequestPartyEligibilityForDelveTiers",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    t.set(
        "SaveSeenCuriosBySlotType",
        lua.create_function(|_, (_slot_type, _table): (Value, Value)| Ok(()))?,
    )?;
    lua.globals().set("C_DelvesUI", t)?;
    Ok(())
}

/// C_ZoneAbility namespace - zone ability data.
fn register_c_zone_ability(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetActiveAbilities",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetZoneAbilityIcon",
        lua.create_function(|_, _spell_id: Value| Ok(Value::Nil))?,
    )?;
    lua.globals().set("C_ZoneAbility", t)?;
    Ok(())
}

/// C_AutoComplete namespace - player name autocomplete results.
fn register_c_auto_complete(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
fn register_c_photo_sharing(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
fn register_auto_complete_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
fn register_misc_global_stubs(lua: &Lua) -> Result<()> {
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
