//! Store, auth, and ping secure namespace stubs.
//!
//! Split from c_stubs_api_extra.rs. Contains:
//! - C_AuthChallenge, C_WowTokenSecure, C_StoreSecure
//! - C_PingSecure, C_Ping

use mlua::{Lua, Result, Value};

/// Register all store/auth/ping secure namespaces.
pub fn register_auth_ping_store(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_c_auth_challenge(lua, g)?;
    register_c_ping_secure(lua)?;
    register_c_ping(lua)?;
    register_c_store_secure(lua)?;
    Ok(())
}

/// Register deferred store hooks that require C_Timer (call after register_timer_api).
pub fn register_store_hooks_deferred(lua: &Lua) -> Result<()> {
    lua.load(r#"
        C_Timer.After(0, function()
            if StoreCardMixin and StoreCardMixin.ShowIcon then
                hooksecurefunc(StoreCardMixin, "ShowIcon", function(self, displayData)
                    if not displayData then return end
                    local useSquare = bit.band(displayData.flags or 0, Enum.BattlepayDisplayFlags.UseSquareIconBorder) == Enum.BattlepayDisplayFlags.UseSquareIconBorder
                    if not useSquare and self.Icon and self.Icon:IsShown() then
                        SetPortraitToTexture(self.Icon, self.Icon:GetTexture())
                    end
                end)
            end
        end)
    "#).exec()
}

fn register_c_auth_challenge(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let auth_challenge = lua.create_table()?;
    auth_challenge.set("SetFrame", lua.create_function(|_, _frame: Value| Ok(()))?)?;
    g.set("C_AuthChallenge", auth_challenge)
}

/// C_WowTokenSecure + C_StoreSecure — fake catalog for Blizzard store UI.
fn register_c_store_secure(lua: &Lua) -> Result<()> {
    lua.load(
        r#"
        C_WowTokenSecure = setmetatable({}, {
            __index = function() return function() end end,
        })
    "#,
    )
    .exec()?;
    lua.load(c_store_secure_lua_src()).exec()
}

fn c_store_secure_lua_src() -> &'static str {
    r#"
        local STORE_GROUPS = {
            {
                groupID = WOW_GAMES_CATEGORY_ID or 33,
                parentGroupID = 0,
                name = "Featured",
                groupName = "Featured",
                description = "Simulator featured products",
                displayedOrder = 1,
                texture = "Interface\\Icons\\INV_Box_02",
                flags = Enum.BattlepayProductGroupFlags.EnabledForTrial + Enum.BattlepayProductGroupFlags.EnabledForVeteran,
                disabledTooltip = "",
            },
            {
                groupID = WOW_SERVICES_CATEGORY_ID or 22,
                parentGroupID = 0,
                name = "Services",
                groupName = "Services",
                description = "Simulator account services",
                displayedOrder = 2,
                texture = "Interface\\Icons\\INV_Misc_Note_02",
                flags = Enum.BattlepayProductGroupFlags.EnabledForTrial + Enum.BattlepayProductGroupFlags.EnabledForVeteran,
                disabledTooltip = "",
            },
        }

        local STORE_ENTRIES = {
            [1001] = {
                entryID = 1001,
                productID = 2001,
                browseBuyButtonText = "Buy Now",
                sharedData = {
                    name = "Apprentice Rider Bundle",
                    description = "A starter bundle for simulator storefront testing.",
                    tooltip = "Includes a mock mount token and a tabard for store card rendering.",
                    currentDollars = 14,
                    currentCents = 99,
                    normalDollars = 19,
                    normalCents = 99,
                    cardType = Enum.BattlepayCardType.MediumCard,
                    flags = 0,
                    buyableHere = true,
                    eligibility = Enum.PurchaseEligibility.Ok,
                    productDecorator = nil,
                    texture = "Interface\\Icons\\Ability_Mount_RidingHorse",
                    overrideTexture = nil,
                    overrideBackground = nil,
                    cards = {},
                    deliverables = {},
                    itemQuantity = 1,
                },
            },
            [1002] = {
                entryID = 1002,
                productID = 2002,
                browseBuyButtonText = "Buy Now",
                sharedData = {
                    name = "Griffon Skycharger",
                    description = "A mock flying mount for the fake simulator store catalog.",
                    tooltip = "Built to exercise icon rendering and price display in the store UI.",
                    currentDollars = 24,
                    currentCents = 99,
                    normalDollars = 24,
                    normalCents = 99,
                    cardType = Enum.BattlepayCardType.MediumCardWithBuyButton,
                    flags = Enum.BattlepayDisplayFlags.UseSquareIconBorder,
                    buyableHere = true,
                    eligibility = Enum.PurchaseEligibility.Ok,
                    productDecorator = nil,
                    texture = "Interface\\Icons\\Ability_Mount_Gryphon_01",
                    overrideTexture = nil,
                    overrideBackground = nil,
                    cards = {},
                    deliverables = {},
                    itemQuantity = 1,
                },
            },
            [1003] = {
                entryID = 1003,
                productID = 2003,
                browseBuyButtonText = "Purchase",
                sharedData = {
                    name = "Name Change Service",
                    description = "A mock account service entry for the simulator.",
                    tooltip = "Useful for validating multiple top-level groups in the fake store catalog.",
                    currentDollars = 10,
                    currentCents = 0,
                    normalDollars = 10,
                    normalCents = 0,
                    cardType = Enum.BattlepayCardType.MediumCard,
                    flags = 0,
                    buyableHere = true,
                    eligibility = Enum.PurchaseEligibility.Ok,
                    productDecorator = nil,
                    texture = "Interface\\Icons\\INV_Misc_Note_02",
                    overrideTexture = nil,
                    overrideBackground = nil,
                    cards = {},
                    deliverables = {},
                    itemQuantity = 0,
                },
            },
        }

        local STORE_PRODUCTS_BY_GROUP = {
            [(WOW_GAMES_CATEGORY_ID or 33)] = { 1001, 1002 },
            [(WOW_SERVICES_CATEGORY_ID or 22)] = { 1003 },
        }

        local STORE_PRODUCT_INFO = {
            [2001] = { productID = 2001, sharedData = STORE_ENTRIES[1001].sharedData },
            [2002] = { productID = 2002, sharedData = STORE_ENTRIES[1002].sharedData },
            [2003] = { productID = 2003, sharedData = STORE_ENTRIES[1003].sharedData },
        }

        local STORE_CURRENCY_INFO = {
            sharedData = {
                regionID = 1,
                formatShort = "$%s",
                formatLong = "$%s",
                licenseAcceptText = "",
                requireLicenseAccept = false,
                browseHasStar = false,
                hideBrowseNotice = false,
                hideConfirmationBrowseNotice = false,
            },
        }

        C_StoreSecure = setmetatable({
            IsStoreAvailable = function() return true end,
            IsAvailable = function() return true end,
            HasPurchaseInProgress = function() return false end,
            HasPurchaseList = function() return true end,
            HasProductList = function() return true end,
            HasDistributionList = function() return true end,
            GetCurrencyInfo = function() return STORE_CURRENCY_INFO end,
            GetCurrencyID = function() return 1 end,
            GetProductGroups = function() return STORE_GROUPS end,
            GetProducts = function(groupID) return STORE_PRODUCTS_BY_GROUP[groupID] or {} end,
            GetProductGroupInfo = function(groupID)
                for _, groupInfo in ipairs(STORE_GROUPS) do
                    if groupInfo.groupID == groupID then
                        return groupInfo
                    end
                end
                return nil
            end,
            GetEntryInfo = function(entryID) return STORE_ENTRIES[entryID] end,
            GetProductInfo = function(productID) return STORE_PRODUCT_INFO[productID] end,
            GetPurchaseList = function() return {} end,
            GetProductList = function()
                FireEvent("STORE_PRODUCTS_UPDATED")
                FireEvent("PRODUCT_DISTRIBUTIONS_UPDATED")
                return STORE_PRODUCTS_BY_GROUP
            end,
            GetFailureInfo = function() return nil, nil end,
            IsDynamicBundle = function() return false end,
            HasDynamicPriceData = function() return true end,
            RequestAllDynamicPriceInfo = function() end,
            PurchaseProduct = function() return true end,
            PurchaseProductConfirm = function() return true end,
        }, { __index = function() return function() end end })
    "#
}

/// C_PingSecure - stores callbacks for Blizzard PingUI, implements action methods.
fn register_c_ping_secure(lua: &Lua) -> Result<()> {
    lua.load(r#"
        _G.__PingSecureCallbacks = _G.__PingSecureCallbacks or {}
        local cbs = _G.__PingSecureCallbacks
        C_PingSecure = {
            SetPingRadialWheelCreatedCallback = function(cb) cbs.RadialWheelCreated = cb end,
            SetPingPinFrameAddedCallback = function(cb) cbs.PingPinFrameAdded = cb end,
            SetPingPinFrameRemovedCallback = function(cb) cbs.PingPinFrameRemoved = cb end,
            SetPingPinFrameScreenClampStateUpdatedCallback = function(cb) cbs.ScreenClampStateUpdated = cb end,
            SetSendMacroPingCallback = function(cb) cbs.SendMacroPing = cb end,
            SetTogglePingListenerCallback = function(cb) cbs.TogglePingListener = cb end,
            SetPendingPingOffScreenCallback = function(cb) cbs.PendingPingOffScreen = cb end,
            SetPingCooldownStartedCallback = function(cb) cbs.PingCooldownStarted = cb end,
            CreateFrame = function()
                local f = CreateFrame("Frame", nil, UIParent)
                if cbs.RadialWheelCreated then cbs.RadialWheelCreated(f) end
            end,
            SendPing = function(pingType, guid) return Enum.PingResult.Success end,
            GetTargetPingReceiver = function(x, y) return nil end,
            GetTargetWorldPing = function(x, y) return true end,
            GetTargetWorldPingAndSend = function()
                return { result = Enum.PingResult.Success }
            end,
            DisplayError = function(err) end,
            ClearPendingPingInfo = function() end,
        }
    "#).exec()
}

fn ping_get_default_options(lua: &Lua, (): ()) -> Result<mlua::Table> {
    let result = lua.create_table()?;
    let entries: &[(i32, &str)] = &[(0, "Attack"), (1, "Warning"), (2, "Assist"), (3, "OnMyWay")];
    for (i, (order_index, texture_kit)) in entries.iter().enumerate() {
        let entry = lua.create_table()?;
        entry.set("orderIndex", *order_index)?;
        entry.set("type", *order_index)?;
        entry.set("uiTextureKitID", *texture_kit)?;
        result.set(i + 1, entry)?;
    }
    Ok(result)
}

fn ping_get_texture_kit(lua: &Lua, ping_type: Value) -> Result<Value> {
    let n = match ping_type {
        Value::Integer(n) => n,
        Value::Number(n) => n as i64,
        _ => return Ok(Value::Nil),
    };
    let kit: Option<&str> = match n {
        0 => Some("Attack"),
        1 => Some("Warning"),
        2 => Some("Assist"),
        3 => Some("OnMyWay"),
        4 => Some("Threat"),
        5 => Some("NonThreat"),
        _ => None,
    };
    match kit {
        Some(s) => Ok(Value::String(lua.create_string(s)?)),
        None => Ok(Value::Nil),
    }
}

/// C_Ping - non-secure ping API with real data for PingManager:SetupDefaultPingOptions.
fn register_c_ping(lua: &Lua) -> Result<()> {
    let ping = lua.create_table()?;
    ping.set(
        "GetCooldownInfo",
        lua.create_function(|_, _: mlua::MultiValue| Ok(Value::Nil))?,
    )?;
    ping.set(
        "GetDefaultPingOptions",
        lua.create_function(ping_get_default_options)?,
    )?;
    ping.set(
        "GetTextureKitForType",
        lua.create_function(ping_get_texture_kit)?,
    )?;
    ping.set(
        "IsPingSystemEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    lua.globals().set("C_Ping", ping)?;
    Ok(())
}
