//! Game menu and global game stub APIs.
//!
//! Contains game-system stubs and menu-related functions:
//! - C_ExternalEventURL, C_StorePublic, Kiosk, GameRulesUtil
//! - Game menu stubs (Logout, Quit, StaticPopup, etc.)
//! - C_CatalogShop, C_SplashScreen, C_ArtifactUI/C_AzeriteItem
//! - C_Commentator, C_ChallengeMode, C_Club, C_ClubFinder
//! - Global game stubs (combat, action, account, unit stats, store)
//! - C_Garrison, MinimapUtil, C_CraftingOrders, ExpansionLandingPage

use mlua::{Lua, Result, Value};

const ITEM_UPGRADE_LOCATION_REGISTRY_KEY: &str = "__wow_item_upgrade_location";

pub(super) fn register_all(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_c_external_event_url(lua)?;
    register_c_store_public(lua)?;
    register_kiosk(lua)?;
    register_game_rules_util(lua)?;
    register_game_menu_stubs(lua)?;
    register_c_catalog_shop(lua)?;
    register_c_commentator(lua)?;
    register_c_artifact_and_azerite(lua)?;
    super::c_misc_api_game_systems::register_game_system_support(lua, state)?;
    register_c_garrison(lua)?;
    register_minimap_util(lua)?;
    register_c_crafting_orders(lua)?;
    register_expansion_landing_page(lua)?;
    register_minimap_globals(lua)?;
    register_c_equipment_set(lua)?;
    register_c_adventure_journal(lua)?;
    register_c_summon_info(lua)?;
    register_c_ui(lua)?;
    register_c_item_upgrade(lua)?;
    Ok(())
}

fn register_c_item_upgrade(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    register_c_item_upgrade_queries(lua, &t)?;
    register_c_item_upgrade_selection(lua, &t)?;
    lua.globals().set("C_ItemUpgrade", t)?;
    Ok(())
}

fn register_c_item_upgrade_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "CanUpgradeItem",
        lua.create_function(|lua, loc: Value| {
            Ok(item_upgrade_item_id_from_location(lua, &loc).is_some())
        })?,
    )?;
    t.set(
        "GetItemHyperlink",
        lua.create_function(|lua, ()| Ok(item_upgrade_item_hyperlink(lua)?.unwrap_or(Value::Nil)))?,
    )?;
    Ok(())
}

fn register_c_item_upgrade_selection(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "SetItemUpgradeFromLocation",
        lua.create_function(|lua, loc: Value| set_item_upgrade_location(lua, loc))?,
    )?;
    t.set(
        "ClearItemUpgrade",
        lua.create_function(|lua, ()| clear_item_upgrade_location(lua))?,
    )?;
    Ok(())
}

pub(crate) fn selected_item_upgrade_item_id(lua: &Lua) -> Option<u32> {
    let location = lua
        .named_registry_value::<Value>(ITEM_UPGRADE_LOCATION_REGISTRY_KEY)
        .ok()?;
    item_upgrade_item_id_from_location(lua, &location)
}

fn set_item_upgrade_location(lua: &Lua, loc: Value) -> Result<()> {
    lua.set_named_registry_value(ITEM_UPGRADE_LOCATION_REGISTRY_KEY, loc)
}

fn clear_item_upgrade_location(lua: &Lua) -> Result<()> {
    lua.set_named_registry_value(ITEM_UPGRADE_LOCATION_REGISTRY_KEY, Value::Nil)
}

fn item_upgrade_item_hyperlink(lua: &Lua) -> Result<Option<Value>> {
    let Some(item_id) = selected_item_upgrade_item_id(lua) else {
        return Ok(None);
    };
    let Some(item) = crate::items::get_item(item_id) else {
        return Ok(None);
    };
    let color = super::c_item_api::quality_color(item.quality);
    let link = format!(
        "|cff{}|Hitem:{}::::::::80:::::|h[{}]|h|r",
        color, item_id, item.name
    );
    Ok(Some(Value::String(lua.create_string(&link)?)))
}

fn item_upgrade_item_id_from_location(lua: &Lua, loc: &Value) -> Option<u32> {
    bag_item_id_from_location(lua, loc).or_else(|| equipped_item_id_from_location(lua, loc))
}

fn bag_item_id_from_location(lua: &Lua, loc: &Value) -> Option<u32> {
    let Value::Table(table) = loc else {
        return None;
    };
    let bag_id = table.get("bagID").ok()?;
    let slot_index = table.get("slotIndex").ok()?;
    let state_rc = crate::lua_api::frame::get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .get_bag_item(bag_id, slot_index)
        .map(|(item_id, _)| item_id)
}

fn equipped_item_id_from_location(lua: &Lua, loc: &Value) -> Option<u32> {
    let Value::Table(table) = loc else {
        return None;
    };
    let slot = table.get("equipmentSlotIndex").ok()?;
    super::c_item_api_globals::get_equipped_item_id(lua, slot)
}

fn register_c_external_event_url(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("HasURL", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsNew", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("LaunchURL", lua.create_function(|_, ()| Ok(()))?)?;
    lua.globals().set("C_ExternalEventURL", t)?;
    Ok(())
}

fn register_c_store_public(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(true))?)?;
    t.set(
        "IsDisabledByParentalControls",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "EventStoreUISetShown",
        lua.create_function(|_, (_shown, _context): (bool, Option<String>)| Ok(()))?,
    )?;
    t.set(
        "DoesGroupHavePurchaseableProducts",
        lua.create_function(|lua, group_id: i32| {
            let globals = lua.globals();
            let store_secure: mlua::Table = globals.get("C_StoreSecure")?;
            let get_products: mlua::Function = store_secure.get("GetProducts")?;
            let products: mlua::Table = get_products.call(group_id)?;
            Ok(products.raw_len() > 0)
        })?,
    )?;
    lua.globals().set("C_StorePublic", t)?;
    Ok(())
}

fn register_kiosk(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("Kiosk", t)?;
    Ok(())
}

fn register_game_rules_util(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetActiveAccountStore",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set("ShouldShowAddOns", lua.create_function(|_, ()| Ok(true))?)?;
    t.set(
        "ShouldShowSplashScreen",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "ShouldShowExpansionLandingPageButton",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "ShouldShowPlayerCastBar",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    t.set(
        "IsTimerunningSeasonActive",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    lua.globals().set("GameRulesUtil", t)?;
    Ok(())
}

fn register_game_menu_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();

    g.set(
        "CurrentVersionHasNewUnseenSettings",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "StaticPopup_Visible",
        lua.create_function(|_, _w: String| Ok(Value::Nil))?,
    )?;
    g.set(
        "IsRestrictedAccount",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    for name in [
        "Logout",
        "Quit",
        "ForceLogout",
        "ForceQuit",
        "ShowMacroFrame",
        "ToggleHelpFrame",
        "ToggleStoreUI",
        "UpdateMicroButtons",
        "SetGamePadCursorControl",
    ] {
        g.set(name, lua.create_function(|_, _: mlua::MultiValue| Ok(()))?)?;
    }
    g.set(
        "CanAutoSetGamePadCursorControl",
        lua.create_function(|_, _e: bool| Ok(false))?,
    )?;
    g.set(
        "SetPortraitTexture",
        lua.create_function(|lua, (tex, unit): (Value, Value)| {
            let texture_path = class_icon_path_for_unit(lua, &unit);
            if let Some(id) = crate::lua_api::frame::extract_frame_id(&tex) {
                let state_rc = crate::lua_api::frame::get_sim_state(lua);
                let mut state = state_rc.borrow_mut();
                if let Some(frame) = state.widgets.get_mut_visual(id) {
                    frame.texture = Some(texture_path);
                }
            }
            Ok(())
        })?,
    )?;
    for name in [
        "ChangeActionBarPage",
        "StaticPopup_UpdateAll",
        "StaticPopup_Show",
        "StaticPopup_Hide",
    ] {
        g.set(name, lua.create_function(|_, _: mlua::MultiValue| Ok(()))?)?;
    }
    g.set(
        "IsTutorialFlagged",
        lua.create_function(|_, _f: i32| Ok(false))?,
    )?;

    register_c_splash_screen(lua)?;
    Ok(())
}

fn register_c_catalog_shop(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsShop2Enabled", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_CatalogShop", t)?;
    Ok(())
}

fn register_c_splash_screen(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "RequestLatestSplashScreen",
        lua.create_function(|_, _f: Option<bool>| Ok(()))?,
    )?;
    t.set(
        "AcknowledgeSplashScreen",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    t.set(
        "CanViewSplashScreen",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "SendSplashScreenCloseTelem",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    lua.globals().set("C_SplashScreen", t)?;
    lua.globals().set(
        "IsCharacterNewlyBoosted",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    Ok(())
}

fn register_c_artifact_and_azerite(lua: &Lua) -> Result<()> {
    let art = lua.create_table()?;
    art.set(
        "IsEquippedArtifactMaxed",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    art.set(
        "IsEquippedArtifactDisabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    art.set(
        "GetEquippedArtifactInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    art.set("GetArtifactItemID", lua.create_function(|_, ()| Ok(0i32))?)?;
    art.set("GetArtifactTier", lua.create_function(|_, ()| Ok(0i32))?)?;
    art.set("IsAtForge", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_ArtifactUI", art)?;

    let az = lua.create_table()?;
    az.set(
        "FindActiveAzeriteItem",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    az.set(
        "IsAzeriteItemAtMaxLevel",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    az.set(
        "IsAzeriteItemEnabled",
        lua.create_function(|_, _i: Value| Ok(false))?,
    )?;
    lua.globals().set("C_AzeriteItem", az)?;

    let aze = lua.create_table()?;
    aze.set(
        "IsAzeriteEmpoweredItem",
        lua.create_function(|_, _loc: Value| Ok(false))?,
    )?;
    aze.set(
        "IsAzeriteEmpoweredItemByID",
        lua.create_function(|_, _id: Value| Ok(false))?,
    )?;
    lua.globals().set("C_AzeriteEmpoweredItem", aze)?;
    Ok(())
}

fn register_c_commentator(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetMode", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("IsSpectating", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_Commentator", t)?;
    Ok(())
}

fn register_c_garrison(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetLandingPageGarrisonType",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "IsLandingPageMinimapButtonVisible",
        lua.create_function(|_, _gt: i32| Ok(false))?,
    )?;
    t.set(
        "GetFollowerShipments",
        lua.create_function(|lua, _id: Value| lua.create_table())?,
    )?;
    lua.globals().set("C_Garrison", t)?;
    Ok(())
}

fn register_minimap_util(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "SetTrackingFilterByFilterIndex",
        lua.create_function(|_, (_i, _v): (i32, bool)| Ok(()))?,
    )?;
    t.set(
        "GetFilterIndexForFilterID",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    lua.globals().set("MinimapUtil", t)?;
    Ok(())
}

fn register_c_crafting_orders(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetPersonalOrdersInfo",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    lua.globals().set("C_CraftingOrders", t)?;
    Ok(())
}

fn register_expansion_landing_page(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsOverlayApplied", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetLandingPageType",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetOverlayMinimapDisplayInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    lua.globals().set("ExpansionLandingPage", t)?;
    Ok(())
}

fn register_minimap_globals(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("HasNewMail", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "GetLatestThreeSenders",
        lua.create_function(|_, ()| Ok(mlua::MultiValue::new()))?,
    )?;
    g.set(
        "DoesFollowerMatchCurrentGarrisonType",
        lua.create_function(|_, _ft: Value| Ok(false))?,
    )?;
    g.set(
        "ShowGarrisonLandingPage",
        lua.create_function(|_, _gt: Value| Ok(()))?,
    )?;
    g.set(
        "ToggleExpansionLandingPage",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "CovenantCalling_CheckCallings",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "ToggleMajorFactionRenown",
        lua.create_function(|_, _fid: Value| Ok(()))?,
    )?;
    g.set(
        "GetGameTime",
        lua.create_function(|lua, ()| {
            // Return local (hour, minute) via Lua's os.date to match the system clock.
            let hour: i32 = lua.load("tonumber(os.date('%H'))").eval()?;
            let min: i32 = lua.load("tonumber(os.date('%M'))").eval()?;
            Ok((hour, min))
        })?,
    )?;
    Ok(())
}

fn register_c_equipment_set(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetEquipmentSetIDs",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetNumEquipmentSets",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetEquipmentSetInfo",
        lua.create_function(|_, _id: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetEquipmentSetID",
        lua.create_function(|_, _name: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetIgnoredSlots",
        lua.create_function(|lua, _id: Value| lua.create_table())?,
    )?;
    t.set(
        "GetEquipmentSetAssignedSpec",
        lua.create_function(|_, _id: Value| Ok(0i32))?,
    )?;
    lua.globals().set("C_EquipmentSet", t)?;
    Ok(())
}

fn register_c_adventure_journal(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("CanBeShown", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("UpdateSuggestions", lua.create_function(|_, ()| Ok(()))?)?;
    t.set(
        "GetNumAvailableSuggestions",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set("GetPrimaryOffset", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set(
        "SetPrimaryOffset",
        lua.create_function(|_, _off: i32| Ok(()))?,
    )?;
    lua.globals().set("C_AdventureJournal", t)?;
    Ok(())
}

fn register_c_ui(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "ShouldUIParentAvoidNotch",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetTopLeftNotchSafeRegion",
        lua.create_function(|_, ()| Ok((0.0f64, 0.0f64, 0.0f64, 0.0f64)))?,
    )?;
    lua.globals().set("C_UI", t)?;
    Ok(())
}

fn register_c_summon_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetSummonConfirmTimeLeft",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set("GetSummonReason", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set(
        "IsSummonSkippingStartExperience",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    lua.globals().set("C_SummonInfo", t)?;
    Ok(())
}

/// Look up the unit's class via `UnitClass` and return the class icon texture path.
fn class_icon_path_for_unit(lua: &Lua, unit: &Value) -> String {
    let class_file = lua
        .globals()
        .get::<mlua::Function>("UnitClass")
        .ok()
        .and_then(|f| f.call::<mlua::MultiValue>(unit.clone()).ok())
        .and_then(|mv| mv.into_iter().nth(1))
        .and_then(|v| match v {
            Value::String(s) => s.to_str().ok().map(|s| s.to_owned()),
            _ => None,
        });
    match class_file.as_deref() {
        Some("WARRIOR") => r"Interface\Icons\ClassIcon_Warrior",
        Some("PALADIN") => r"Interface\Icons\ClassIcon_Paladin",
        Some("HUNTER") => r"Interface\Icons\ClassIcon_Hunter",
        Some("ROGUE") => r"Interface\Icons\ClassIcon_Rogue",
        Some("PRIEST") => r"Interface\Icons\ClassIcon_Priest",
        Some("DEATHKNIGHT") => r"Interface\Icons\ClassIcon_DeathKnight",
        Some("SHAMAN") => r"Interface\Icons\ClassIcon_Shaman",
        Some("MAGE") => r"Interface\Icons\ClassIcon_Mage",
        Some("WARLOCK") => r"Interface\Icons\ClassIcon_Warlock",
        Some("MONK") => r"Interface\Icons\ClassIcon_Monk",
        Some("DRUID") => r"Interface\Icons\ClassIcon_Druid",
        Some("DEMONHUNTER") => r"Interface\Icons\ClassIcon_DemonHunter",
        Some("EVOKER") => r"Interface\Icons\ClassIcon_Evoker",
        _ => r"Interface\CharacterFrame\TempPortrait",
    }
    .to_string()
}
