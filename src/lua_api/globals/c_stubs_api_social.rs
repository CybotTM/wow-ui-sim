//! Social, trial, and feature namespace stubs.
//!
//! Split from c_stubs_api_extra.rs. Contains:
//! - C_ClassTrial, C_RecruitAFriend, C_WowTokenPublic, C_FriendList
//! - C_CatalogShop, C_Who, C_PrivateAuras, C_GuildBank, C_PetBattles
//! - C_DelvesUI, C_ZoneAbility, C_AutoComplete, C_PhotoSharing
//! - Misc global stubs (totems, parental controls, etc.)

use mlua::{Lua, MultiValue, Result, Value};

const CATALOG_SHOP_CATEGORY_ID: i64 = 1;
const CATALOG_SHOP_SECTION_ID: i64 = 101;
const CATALOG_SHOP_PRODUCT_ID: i64 = 1001;
const CATALOG_SHOP_SESSION_ID: &str = "catalog-shop-session";

struct CatalogShopCategorySeed {
    id: i64,
    display_name: &'static str,
    icon_texture: &'static str,
    link_tag: &'static str,
    is_disabled: bool,
    show_persistent_refund_button: bool,
}

struct CatalogShopSectionSeed {
    id: i64,
    category_id: i64,
    display_name: &'static str,
    card_type: &'static str,
    scroll_grid_size: i64,
    should_show_recommendation_opt_out_disclaimer: bool,
}

struct CatalogShopProductSeed {
    id: i64,
    name: &'static str,
    product_type: &'static str,
    description: &'static str,
    price: &'static str,
}

const CATALOG_SHOP_CATEGORY: CatalogShopCategorySeed = CatalogShopCategorySeed {
    id: CATALOG_SHOP_CATEGORY_ID,
    display_name: "Featured",
    icon_texture: "",
    link_tag: "featured",
    is_disabled: false,
    show_persistent_refund_button: false,
};

const CATALOG_SHOP_SECTION: CatalogShopSectionSeed = CatalogShopSectionSeed {
    id: CATALOG_SHOP_SECTION_ID,
    category_id: CATALOG_SHOP_CATEGORY_ID,
    display_name: "Starter Bundles",
    card_type: "",
    scroll_grid_size: 3,
    should_show_recommendation_opt_out_disclaimer: false,
};

const CATALOG_SHOP_PRODUCT: CatalogShopProductSeed = CatalogShopProductSeed {
    id: CATALOG_SHOP_PRODUCT_ID,
    name: "Apprentice Rider Bundle",
    product_type: "Services",
    description: "A starter riding bundle for new characters.",
    price: "$9.99",
};

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
    let t: mlua::Table = match g.get::<Value>("C_FriendList")? {
        Value::Table(existing) => existing,
        _ => lua.create_table()?,
    };
    t.set("SetWhoToUi", lua.create_function(|_, _flag: bool| Ok(()))?)?;
    t.set("SendWho", lua.create_function(|_, _msg: String| Ok(()))?)?;
    t.set("GetNumWhoResults", lua.create_function(|_, ()| Ok(0i32))?)?;
    if t.get::<Value>("GetNumFriends")?.is_nil() {
        t.set("GetNumFriends", lua.create_function(|_, ()| Ok(0i32))?)?;
    }
    if t.get::<Value>("GetNumOnlineFriends")?.is_nil() {
        t.set(
            "GetNumOnlineFriends",
            lua.create_function(|_, ()| Ok(0i32))?,
        )?;
    }
    if t.get::<Value>("GetFriendInfoByIndex")?.is_nil() {
        t.set(
            "GetFriendInfoByIndex",
            lua.create_function(|_, _idx: i32| Ok(Value::Nil))?,
        )?;
    }
    t.set("ShowFriends", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_FriendList", t)
}

/// C_CatalogShop, C_Who, C_PrivateAuras stubs.
fn register_shop_who_auras(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let catalog_shop: mlua::Table = match g.get::<Value>("C_CatalogShop")? {
        Value::Table(existing) => existing,
        _ => lua.create_table()?,
    };
    register_catalog_shop_queries(lua, &catalog_shop)?;
    register_catalog_shop_interaction(lua, &catalog_shop)?;
    if catalog_shop.get::<Value>("IsShop2Enabled")?.is_nil() {
        catalog_shop.set("IsShop2Enabled", lua.create_function(|_, ()| Ok(false))?)?;
    }
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

fn register_catalog_shop_queries(lua: &Lua, catalog_shop: &mlua::Table) -> Result<()> {
    register_catalog_shop_category_queries(lua, catalog_shop)?;
    register_catalog_shop_product_queries(lua, catalog_shop)?;
    register_catalog_shop_misc_queries(lua, catalog_shop)?;
    Ok(())
}

fn register_catalog_shop_category_queries(lua: &Lua, catalog_shop: &mlua::Table) -> Result<()> {
    catalog_shop.set(
        "GetAvailableCategoryIDs",
        lua.create_function(|lua, ()| lua.create_sequence_from([CATALOG_SHOP_CATEGORY_ID]))?,
    )?;
    catalog_shop.set(
        "GetCategoryInfo",
        lua.create_function(|lua, category_id: i64| catalog_shop_category_info(lua, category_id))?,
    )?;
    catalog_shop.set(
        "GetSectionIDsForCategory",
        lua.create_function(|lua, category_id: i64| {
            if category_id == CATALOG_SHOP_CATEGORY_ID {
                return Ok(Value::Table(
                    lua.create_sequence_from([CATALOG_SHOP_SECTION_ID])?,
                ));
            }
            Ok(Value::Table(lua.create_table()?))
        })?,
    )?;
    catalog_shop.set(
        "GetCategorySectionInfo",
        lua.create_function(|lua, (category_id, section_id): (i64, i64)| {
            catalog_shop_section_info(lua, category_id, section_id)
        })?,
    )?;
    catalog_shop.set(
        "GetProductIDsForCategory",
        lua.create_function(|lua, category_id: i64| {
            catalog_shop_product_ids_for_category(lua, category_id)
        })?,
    )?;
    catalog_shop.set(
        "GetProductIDsForCategorySection",
        lua.create_function(|lua, (category_id, section_id): (i64, i64)| {
            catalog_shop_product_ids_for_section(lua, category_id, section_id)
        })?,
    )?;
    Ok(())
}

fn register_catalog_shop_product_queries(lua: &Lua, catalog_shop: &mlua::Table) -> Result<()> {
    catalog_shop.set(
        "GetProductInfo",
        lua.create_function(|lua, product_id: i64| catalog_shop_product_info(lua, product_id))?,
    )?;
    catalog_shop.set(
        "GetCatalogShopProductDisplayInfo",
        lua.create_function(|lua, product_id: i64| {
            catalog_shop_product_display_info(lua, product_id)
        })?,
    )?;
    catalog_shop.set(
        "GetProductSortOrder",
        lua.create_function(
            |_, (category_id, section_id, product_id): (i64, i64, i64)| {
                if category_id == CATALOG_SHOP_CATEGORY_ID
                    && section_id == CATALOG_SHOP_SECTION_ID
                    && product_id == CATALOG_SHOP_PRODUCT_ID
                {
                    return Ok(Value::Integer(1));
                }
                Ok(Value::Nil)
            },
        )?,
    )?;
    catalog_shop.set(
        "GetFirstCategoryByProductID",
        lua.create_function(|lua, product_id: i64| {
            if product_id == CATALOG_SHOP_PRODUCT_ID {
                return catalog_shop_category_info(lua, CATALOG_SHOP_CATEGORY_ID);
            }
            Ok(Value::Nil)
        })?,
    )?;
    catalog_shop.set(
        "GetProductAvailabilityTimeRemainingSecs",
        lua.create_function(|_, _product_id: i64| Ok(Value::Nil))?,
    )?;
    Ok(())
}

fn register_catalog_shop_misc_queries(lua: &Lua, catalog_shop: &mlua::Table) -> Result<()> {
    catalog_shop.set(
        "GetProductIDsForBundle",
        lua.create_function(|lua, _product_id: i64| Ok(Value::Table(lua.create_table()?)))?,
    )?;
    catalog_shop.set(
        "GetAvailableTransmogRaceInfos",
        lua.create_function(|lua, ()| Ok(Value::Table(lua.create_table()?)))?,
    )?;
    catalog_shop.set(
        "GetNewProducts",
        lua.create_function(|lua, ()| Ok(Value::Table(lua.create_table()?)))?,
    )?;
    catalog_shop.set(
        "GetVCProductInfos",
        lua.create_function(|lua, ()| Ok(Value::Table(lua.create_table()?)))?,
    )?;
    catalog_shop.set(
        "GetVirtualCurrencyBalance",
        lua.create_function(|lua, _currency_code: String| {
            Ok(Value::String(lua.create_string("0")?))
        })?,
    )?;
    catalog_shop.set(
        "GetRefundableDecors",
        lua.create_function(|lua, _: Value| {
            Ok(MultiValue::from_vec(vec![
                Value::Table(lua.create_table()?),
                Value::Integer(0),
            ]))
        })?,
    )?;
    catalog_shop.set(
        "GetFailureInfo",
        lua.create_function(|_, ()| Ok(MultiValue::from_vec(vec![Value::Nil, Value::Nil])))?,
    )?;
    catalog_shop.set(
        "GetSpellVisualInfoForMount",
        lua.create_function(|lua, _spell_visual_id: i64| Ok(Value::Table(lua.create_table()?)))?,
    )?;
    catalog_shop.set(
        "IsProductIncludedInAnyBundle",
        lua.create_function(|_, _product_id: i64| Ok(false))?,
    )?;
    Ok(())
}

fn register_catalog_shop_interaction(lua: &Lua, catalog_shop: &mlua::Table) -> Result<()> {
    register_catalog_shop_session_actions(lua, catalog_shop)?;
    register_catalog_shop_purchase_actions(lua, catalog_shop)?;
    register_catalog_shop_telemetry_actions(lua, catalog_shop)?;
    Ok(())
}

fn register_catalog_shop_session_actions(lua: &Lua, catalog_shop: &mlua::Table) -> Result<()> {
    catalog_shop.set(
        "OpenCatalogShopInteractionFromShop",
        lua.create_function(|lua, ()| open_catalog_shop_interaction(lua))?,
    )?;
    catalog_shop.set(
        "OpenCatalogShopInteractionFromHouse",
        lua.create_function(|lua, ()| open_catalog_shop_interaction(lua))?,
    )?;
    catalog_shop.set(
        "CloseCatalogShopInteraction",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    Ok(())
}

fn register_catalog_shop_purchase_actions(lua: &Lua, catalog_shop: &mlua::Table) -> Result<()> {
    catalog_shop.set(
        "PurchaseProduct",
        lua.create_function(|_, _product_id: i64| Ok(false))?,
    )?;
    catalog_shop.set(
        "BulkPurchaseProducts",
        lua.create_function(|_, _product_ids: Value| Ok(false))?,
    )?;
    catalog_shop.set(
        "ConfirmHousingPurchase",
        lua.create_function(|_, _product_ids: Value| Ok(()))?,
    )?;
    catalog_shop.set(
        "FindBestCurrencyProductForNeededAmount",
        lua.create_function(|_, (_currency_code, _amount): (String, i64)| Ok(Value::Nil))?,
    )?;
    catalog_shop.set(
        "RefreshRefundableDecors",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    catalog_shop.set(
        "RefreshVirtualCurrencyBalance",
        lua.create_function(|lua, currency_code: String| {
            fire_event(
                lua,
                "CATALOG_SHOP_VIRTUAL_CURRENCY_BALANCE_UPDATE",
                &[
                    Value::String(lua.create_string(currency_code)?),
                    Value::String(lua.create_string("0")?),
                ],
            )
        })?,
    )?;
    Ok(())
}

fn register_catalog_shop_telemetry_actions(lua: &Lua, catalog_shop: &mlua::Table) -> Result<()> {
    catalog_shop.set(
        "ProductDisplayedTelemetry",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    catalog_shop.set(
        "ProductSelectedTelemetry",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    catalog_shop.set(
        "OnLegalDisclaimerClicked",
        lua.create_function(|_, _product_id: i64| Ok(()))?,
    )?;
    catalog_shop.set(
        "OnLegalPersonalizedOptOutClicked",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    catalog_shop.set(
        "ShouldShowHousingWarning",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    catalog_shop.set(
        "StartHousingVCPurchaseConfirmation",
        lua.create_function(|_, _product_id: i64| Ok(()))?,
    )?;
    catalog_shop.set(
        "BulkRefundDecors",
        lua.create_function(|_, _decor_guids: Value| Ok(()))?,
    )?;
    Ok(())
}

fn catalog_shop_category_info(lua: &Lua, category_id: i64) -> Result<Value> {
    if category_id != CATALOG_SHOP_CATEGORY.id {
        return Ok(Value::Nil);
    }

    let info = lua.create_table()?;
    info.set("ID", CATALOG_SHOP_CATEGORY.id)?;
    info.set("displayName", CATALOG_SHOP_CATEGORY.display_name)?;
    info.set("iconTexture", CATALOG_SHOP_CATEGORY.icon_texture)?;
    info.set("linkTag", CATALOG_SHOP_CATEGORY.link_tag)?;
    info.set("isDisabled", CATALOG_SHOP_CATEGORY.is_disabled)?;
    info.set(
        "showPersistentRefundButton",
        CATALOG_SHOP_CATEGORY.show_persistent_refund_button,
    )?;
    Ok(Value::Table(info))
}

fn catalog_shop_section_info(lua: &Lua, category_id: i64, section_id: i64) -> Result<Value> {
    if category_id != CATALOG_SHOP_SECTION.category_id || section_id != CATALOG_SHOP_SECTION.id {
        return Ok(Value::Nil);
    }

    let info = lua.create_table()?;
    info.set("ID", CATALOG_SHOP_SECTION.id)?;
    info.set("displayName", CATALOG_SHOP_SECTION.display_name)?;
    info.set(
        "parentCatalogShopCategoryInfoID",
        CATALOG_SHOP_SECTION.category_id,
    )?;
    info.set("cardType", CATALOG_SHOP_SECTION.card_type)?;
    info.set("scrollGridSize", CATALOG_SHOP_SECTION.scroll_grid_size)?;
    info.set(
        "shouldShowRecommendationOptOutDisclaimer",
        CATALOG_SHOP_SECTION.should_show_recommendation_opt_out_disclaimer,
    )?;
    Ok(Value::Table(info))
}

fn catalog_shop_product_ids_for_category(lua: &Lua, category_id: i64) -> Result<Value> {
    if category_id == CATALOG_SHOP_CATEGORY_ID {
        return Ok(Value::Table(
            lua.create_sequence_from([CATALOG_SHOP_PRODUCT_ID])?,
        ));
    }
    Ok(Value::Table(lua.create_table()?))
}

fn catalog_shop_product_ids_for_section(
    lua: &Lua,
    category_id: i64,
    section_id: i64,
) -> Result<Value> {
    if category_id == CATALOG_SHOP_CATEGORY_ID && section_id == CATALOG_SHOP_SECTION_ID {
        return Ok(Value::Table(
            lua.create_sequence_from([CATALOG_SHOP_PRODUCT_ID])?,
        ));
    }
    Ok(Value::Table(lua.create_table()?))
}

fn catalog_shop_product_info(lua: &Lua, product_id: i64) -> Result<Value> {
    if product_id != CATALOG_SHOP_PRODUCT.id {
        return Ok(Value::Nil);
    }

    let info = lua.create_table()?;
    set_catalog_shop_product_identity(&info)?;
    set_catalog_shop_product_collection_fields(lua, &info)?;
    set_catalog_shop_product_card_fields(&info)?;
    set_catalog_shop_product_purchase_fields(lua, &info)?;
    Ok(Value::Table(info))
}

fn catalog_shop_product_display_info(lua: &Lua, product_id: i64) -> Result<Value> {
    if product_id != CATALOG_SHOP_PRODUCT.id {
        return Ok(Value::Nil);
    }

    let info = lua.create_table()?;
    set_catalog_shop_display_scene_fields(lua, &info)?;
    set_catalog_shop_display_icon_fields(lua, &info)?;
    set_catalog_shop_display_media_fields(&info)?;
    Ok(Value::Table(info))
}

fn set_catalog_shop_product_identity(info: &mlua::Table) -> Result<()> {
    info.set("catalogShopProductID", CATALOG_SHOP_PRODUCT.id)?;
    info.set("name", CATALOG_SHOP_PRODUCT.name)?;
    info.set("type", CATALOG_SHOP_PRODUCT.product_type)?;
    info.set("description", CATALOG_SHOP_PRODUCT.description)?;
    info.set("iconTexture", "")?;
    Ok(())
}

fn set_catalog_shop_product_collection_fields(lua: &Lua, info: &mlua::Table) -> Result<()> {
    info.set("itemID", 0)?;
    info.set("mountID", 0)?;
    info.set("mountTypeName", "")?;
    info.set("speciesID", 0)?;
    info.set("transmogSetID", 0)?;
    info.set("itemModifiedAppearanceID", 0)?;
    info.set("subItems", lua.create_table()?)?;
    info.set("subItemsLoaded", true)?;
    Ok(())
}

fn set_catalog_shop_product_card_fields(info: &mlua::Table) -> Result<()> {
    info.set("backgroundTexture", "")?;
    info.set("foregroundTexture", Value::Nil)?;
    info.set("smallCardBGTexture", Value::Nil)?;
    info.set("smallCardFGTexture", Value::Nil)?;
    info.set("wideCardBGTexture", Value::Nil)?;
    info.set("wideCardFGTexture", Value::Nil)?;
    info.set("previewIconTexture", Value::Nil)?;
    info.set("optionalWideCardBackgroundTexture", Value::Nil)?;
    info.set("isBundle", false)?;
    info.set("bundleChildrenSize", 1)?;
    info.set("numBundleDetailCards", 0)?;
    Ok(())
}

fn set_catalog_shop_product_purchase_fields(lua: &Lua, info: &mlua::Table) -> Result<()> {
    info.set("isFullyOwned", false)?;
    info.set("isPurchasePending", false)?;
    info.set("refundable", false)?;
    info.set("price", CATALOG_SHOP_PRODUCT.price)?;
    info.set("originalPrice", "")?;
    info.set("discountPercentage", 0)?;
    info.set("licenseTermType", 0)?;
    info.set("licenseTermDuration", 0)?;
    info.set("virtualCurrencies", lua.create_table()?)?;
    info.set("isHidden", false)?;
    info.set("hasPendingOrders", false)?;
    info.set("isDynamicallyDiscounted", false)?;
    info.set("shouldShowOriginalPrice", false)?;
    info.set("wideCardBGOverrideProductURL", Value::Nil)?;
    info.set("previewBGOverrideProductURL", Value::Nil)?;
    info.set("previewSmallBGOverrideProductURL", Value::Nil)?;
    info.set("decorQuantity", Value::Nil)?;
    info.set("isVCProduct", false)?;
    info.set("containsHousingItem", false)?;
    Ok(())
}

fn set_catalog_shop_display_scene_fields(lua: &Lua, info: &mlua::Table) -> Result<()> {
    info.set("defaultPreviewModelSceneID", 0)?;
    info.set("defaultCardModelSceneID", 0)?;
    info.set("defaultWideCardModelSceneID", 0)?;
    info.set("itemID", 0)?;
    info.set("overridePreviewModelSceneID", Value::Nil)?;
    info.set("overrideCardModelSceneID", Value::Nil)?;
    info.set("overrideWideCardModelSceneID", Value::Nil)?;
    info.set("creatureDisplayInfoIDs", lua.create_table()?)?;
    info.set("spellVisualIDs", lua.create_table()?)?;
    info.set("mainHandItemModifiedAppearanceID", Value::Nil)?;
    info.set("offHandItemModifiedAppearanceID", Value::Nil)?;
    info.set("itemModifiedAppearanceIDs", lua.create_table()?)?;
    Ok(())
}

fn set_catalog_shop_display_icon_fields(lua: &Lua, info: &mlua::Table) -> Result<()> {
    info.set("iconFileDataID", 132261)?;
    info.set("iconTextureKit", Value::Nil)?;
    info.set("productType", CATALOG_SHOP_PRODUCT.product_type)?;
    info.set("itemDescription", CATALOG_SHOP_PRODUCT.description)?;
    info.set("hasUnknownLicense", false)?;
    info.set("productPMTURL", Value::Nil)?;
    info.set("additionalProductPMTURLs", lua.create_table()?)?;
    Ok(())
}

fn set_catalog_shop_display_media_fields(info: &mlua::Table) -> Result<()> {
    info.set("otherProductImageAtlasName", Value::Nil)?;
    info.set("otherProductGameTitleBaseTag", Value::Nil)?;
    info.set("otherProductGameType", Value::Nil)?;
    info.set("customLoopingSoundStart", Value::Nil)?;
    info.set("customLoopingSoundMiddle", Value::Nil)?;
    info.set("customLoopingSoundEnd", Value::Nil)?;
    info.set("specialActorID_1", Value::Nil)?;
    info.set("specialActorID_2", Value::Nil)?;
    info.set("specialActorID_3", Value::Nil)?;
    info.set("specialActorID_4", Value::Nil)?;
    info.set("specialActorID_5", Value::Nil)?;
    info.set("gameFlavorID", Value::Nil)?;
    info.set("decorFileDataID", Value::Nil)?;
    info.set("quantity", Value::Nil)?;
    info.set("houseTextureAtlas", Value::Nil)?;
    Ok(())
}

fn open_catalog_shop_interaction(lua: &Lua) -> Result<Value> {
    let session = Value::String(lua.create_string(CATALOG_SHOP_SESSION_ID)?);
    fire_event(
        lua,
        "CATALOG_SHOP_DATA_REFRESH",
        std::slice::from_ref(&session),
    )?;
    fire_event(
        lua,
        "CATALOG_SHOP_FETCH_SUCCESS",
        std::slice::from_ref(&session),
    )?;
    Ok(session)
}

fn fire_event(lua: &Lua, event_name: &str, args: &[Value]) -> Result<()> {
    let fire: mlua::Function = lua.globals().get("FireEvent")?;
    let mut call_args = vec![Value::String(lua.create_string(event_name)?)];
    call_args.extend(args.iter().cloned());
    fire.call(MultiValue::from_vec(call_args))
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
