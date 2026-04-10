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
    lua.load(c_store_secure_lua_src()).exec()
}

fn c_store_secure_lua_src() -> &'static str {
    r#"
        local TOKEN_RESULT_FAILURE_CAP = (Enum.CanRedeemTokenForBalanceResult and Enum.CanRedeemTokenForBalanceResult.FailureCap) or 1
        local TOKEN_REDEEM_TYPE_GAME_TIME = LE_TOKEN_REDEEM_TYPE_GAME_TIME or 1
        local TOKEN_REDEEM_TYPE_BALANCE = LE_TOKEN_REDEEM_TYPE_BALANCE or 2
        local WOW_TOKEN_GAME_TIME_MINUTES = 30 * 24 * 60

        local WOW_TOKEN_STATE = {
            tokenCount = 2,
            currentBalance = 2500,
            balanceRedeemAmount = 1500,
            balanceAmountString = "$15.00",
            cannotRedeemReason = 0,
            isSubscribed = false,
            remainingGameTime = 1440,
            pendingRedeemType = nil,
            priceLockDuration = 900,
            willKickFromWorld = false,
            lastBuyConfirmed = false,
            lastSellConfirmed = false,
        }

        local function fire_wowtoken_event(eventName, ...)
            if FireEvent then
                FireEvent(eventName, ...)
            end
        end

        local function parse_balance_amount(value)
            local text = tostring(value or "")
            local dollars, cents = text:match("(%d+)[^%d]+(%d%d)")
            if dollars then
                return (tonumber(dollars) or 0) * 100 + (tonumber(cents) or 0)
            end

            local wholeUnits = text:match("(%d+)")
            if wholeUnits then
                return (tonumber(wholeUnits) or 0) * 100
            end

            return nil
        end

        C_WowTokenSecure = setmetatable({
            CanRedeemForBalance = function()
                local canRedeem = WOW_TOKEN_STATE.tokenCount > 0
                fire_wowtoken_event("TOKEN_REDEEM_BALANCE_UPDATED")
                if canRedeem then
                    return Enum.CanRedeemTokenForBalanceResult.Ok
                end
                return TOKEN_RESULT_FAILURE_CAP
            end,
            CancelRedeem = function()
                WOW_TOKEN_STATE.pendingRedeemType = nil
                return true
            end,
            ConfirmBuyToken = function(accepted)
                WOW_TOKEN_STATE.lastBuyConfirmed = accepted and true or false
                if not accepted then
                    return false
                end

                WOW_TOKEN_STATE.tokenCount = WOW_TOKEN_STATE.tokenCount + 1
                fire_wowtoken_event("TOKEN_STATUS_CHANGED")
                return true
            end,
            ConfirmSellToken = function(accepted)
                WOW_TOKEN_STATE.lastSellConfirmed = accepted and true or false
                if not accepted then
                    return false
                end

                if WOW_TOKEN_STATE.tokenCount > 0 then
                    WOW_TOKEN_STATE.tokenCount = WOW_TOKEN_STATE.tokenCount - 1
                end
                fire_wowtoken_event("TOKEN_STATUS_CHANGED")
                return true
            end,
            GetBalanceRedeemAmount = function()
                return WOW_TOKEN_STATE.balanceRedeemAmount
            end,
            GetBalanceRedemptionInfo = function()
                return WOW_TOKEN_STATE.currentBalance,
                    WOW_TOKEN_STATE.balanceRedeemAmount,
                    WOW_TOKEN_STATE.tokenCount > 0,
                    WOW_TOKEN_STATE.cannotRedeemReason
            end,
            GetGameTimeRedemptionInfo = function()
                return WOW_TOKEN_STATE.isSubscribed, WOW_TOKEN_STATE.remainingGameTime
            end,
            GetPriceLockDuration = function()
                return WOW_TOKEN_STATE.priceLockDuration
            end,
            GetRemainingGameTime = function()
                fire_wowtoken_event("TOKEN_REDEEM_GAME_TIME_UPDATED")
                return WOW_TOKEN_STATE.remainingGameTime
            end,
            GetTokenCount = function()
                return WOW_TOKEN_STATE.tokenCount
            end,
            IsRedemptionStillValid = function()
                return WOW_TOKEN_STATE.pendingRedeemType ~= nil and WOW_TOKEN_STATE.tokenCount > 0
            end,
            RedeemToken = function(redeemType)
                if WOW_TOKEN_STATE.tokenCount <= 0 then
                    return false
                end

                WOW_TOKEN_STATE.pendingRedeemType = redeemType
                return true
            end,
            RedeemTokenConfirm = function(redeemType)
                if WOW_TOKEN_STATE.pendingRedeemType ~= redeemType or WOW_TOKEN_STATE.tokenCount <= 0 then
                    return false
                end

                WOW_TOKEN_STATE.pendingRedeemType = nil
                WOW_TOKEN_STATE.tokenCount = WOW_TOKEN_STATE.tokenCount - 1

                if redeemType == TOKEN_REDEEM_TYPE_BALANCE then
                    WOW_TOKEN_STATE.currentBalance = WOW_TOKEN_STATE.currentBalance + WOW_TOKEN_STATE.balanceRedeemAmount
                    fire_wowtoken_event("TOKEN_STATUS_CHANGED")
                    fire_wowtoken_event("TOKEN_REDEEM_BALANCE_UPDATED")
                    return true
                end

                if redeemType == TOKEN_REDEEM_TYPE_GAME_TIME then
                    WOW_TOKEN_STATE.isSubscribed = true
                    WOW_TOKEN_STATE.remainingGameTime = WOW_TOKEN_STATE.remainingGameTime + WOW_TOKEN_GAME_TIME_MINUTES
                    fire_wowtoken_event("TOKEN_STATUS_CHANGED")
                    fire_wowtoken_event("TOKEN_REDEEM_GAME_TIME_UPDATED")
                    return true
                end

                return false
            end,
            SetBalanceAmountString = function(value)
                WOW_TOKEN_STATE.balanceAmountString = tostring(value or "")
                local parsedAmount = parse_balance_amount(value)
                if parsedAmount then
                    WOW_TOKEN_STATE.balanceRedeemAmount = parsedAmount
                end
            end,
            WillKickFromWorld = function()
                return WOW_TOKEN_STATE.willKickFromWorld
            end,
        }, { __index = function() return function() end end })

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
                    productDecorator = Enum.BattlepayProductDecorator.VasService,
                    vasServiceType = Enum.VasServiceType.NameChange,
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

        local STORE_REALMS = {
            { realmName = "Azeroth", virtualRealmAddress = 101 },
            { realmName = "Kalimdor", virtualRealmAddress = 202 },
        }

        local STORE_CHARACTERS_BY_REALM = {
            [101] = {
                {
                    guid = 501001,
                    name = "Simhero",
                    level = 70,
                    classID = 2,
                    classFileName = "PALADIN",
                    className = "Paladin",
                    raceID = 1,
                    raceName = "Human",
                    faction = 1,
                    realmName = "Azeroth",
                    wowAccount = 1001,
                    isGuildMaster = true,
                },
                {
                    guid = 501002,
                    name = "Simalt",
                    level = 60,
                    classID = 3,
                    classFileName = "HUNTER",
                    className = "Hunter",
                    raceID = 4,
                    raceName = "Night Elf",
                    faction = 1,
                    realmName = "Azeroth",
                    wowAccount = 1001,
                    isGuildMaster = false,
                },
            },
            [202] = {
                {
                    guid = 602001,
                    name = "Hordesim",
                    level = 70,
                    classID = 7,
                    classFileName = "SHAMAN",
                    className = "Shaman",
                    raceID = 2,
                    raceName = "Orc",
                    faction = 0,
                    realmName = "Kalimdor",
                    wowAccount = 2002,
                    isGuildMaster = true,
                },
            },
        }

        local STORE_CHARACTERS_BY_GUID = {}
        for _, characters in pairs(STORE_CHARACTERS_BY_REALM) do
            for _, character in ipairs(characters) do
                STORE_CHARACTERS_BY_GUID[character.guid] = character
            end
        end

        local STORE_GUILD_MASTER_INFO = {
            [501001] = { guildName = "Simulator Guild", guildMasterName = "Simleader" },
            [602001] = { guildName = "Horde Sim Guild", guildMasterName = "Hordeleader" },
        }

        local STORE_GUILD_FOLLOW_INFO = {
            [501001] = { transferredRealm = "Kalimdor", factionChanged = false },
            [602001] = { transferredRealm = nil, factionChanged = true },
        }

        local STORE_ELIGIBLE_RACES = {
            [501001] = {
                { raceID = 1, raceName = "Human", isAlliedRace = false, isHeritageArmorUnlocked = true },
                { raceID = 29, raceName = "Void Elf", isAlliedRace = true, isHeritageArmorUnlocked = false },
            },
            [501002] = {
                { raceID = 4, raceName = "Night Elf", isAlliedRace = false, isHeritageArmorUnlocked = true },
            },
            [602001] = {
                { raceID = 2, raceName = "Orc", isAlliedRace = false, isHeritageArmorUnlocked = true },
                { raceID = 36, raceName = "Mag'har Orc", isAlliedRace = true, isHeritageArmorUnlocked = false },
            },
        }

        local LOCAL_WOW_ACCOUNT_GUIDS = {
            WoW1 = 1001,
            WoW2 = 1002,
        }

        local REMOTE_WOW_ACCOUNT_GUIDS = {
            WoW2 = 2002,
            WoW3 = 2003,
        }

        local DEFAULT_BNET_TRANSFER_INFO = {
            guid = 3001,
            gameAccounts = { "WoW2", "WoW3" },
        }

        local STORE_BNET_TRANSFER_INFO = {
            guid = DEFAULT_BNET_TRANSFER_INFO.guid,
            gameAccounts = { unpack(DEFAULT_BNET_TRANSFER_INFO.gameAccounts) },
        }

        local STORE_VAS_ERRORS = {}
        local STORE_FAILURE_INFO = nil
        local STORE_PREGENERATED_EXTERNAL_TRANSACTION_ID = nil
        local STORE_DISCONNECT_ON_LOGOUT = false
        local STORE_VAS_PRODUCT_READY = false
        local STORE_VAS_COMPLETION_INFO = nil
        local STORE_LAST_PRODUCT_LIST_RESPONSE_ERROR = 0
        local STORE_CONFIRMATION_INFO = {
            productID = 2003,
            walletName = "Blizzard Balance",
            currentDollars = 10,
            currentCents = 0,
        }
        local STORE_UNREVOKED_BOOST_INFO = {
            productName = "Level 70 Character Boost",
            characterName = "Simhero",
            realmName = "Azeroth",
        }

        local function fire_store_event(eventName, ...)
            if FireEvent then
                FireEvent(eventName, ...)
            end
        end

        local function find_realm_name_by_address(realmAddress)
            for _, realm in ipairs(STORE_REALMS) do
                if realm.virtualRealmAddress == realmAddress then
                    return realm.realmName
                end
            end
            return nil
        end

        local function build_confirmation_info(productID)
            local productInfo = STORE_PRODUCT_INFO[productID]
            if not productInfo then
                return nil
            end

            return {
                productID = productID,
                walletName = "Blizzard Balance",
                currentDollars = productInfo.sharedData.currentDollars,
                currentCents = productInfo.sharedData.currentCents,
            }
        end

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
                STORE_LAST_PRODUCT_LIST_RESPONSE_ERROR = 0
                FireEvent("STORE_PRODUCTS_UPDATED")
                FireEvent("PRODUCT_DISTRIBUTIONS_UPDATED")
                return STORE_PRODUCTS_BY_GROUP
            end,
            GetFailureInfo = function()
                if not STORE_FAILURE_INFO then
                    return nil, nil
                end
                return unpack(STORE_FAILURE_INFO)
            end,
            GetWoWAccountGUIDFromName = function(accountName, isLocalAccount)
                local accountMap = isLocalAccount and LOCAL_WOW_ACCOUNT_GUIDS or REMOTE_WOW_ACCOUNT_GUIDS
                return accountMap[accountName] or 0
            end,
            GetCharacterInfoByGUID = function(characterGUID)
                return STORE_CHARACTERS_BY_GUID[characterGUID]
            end,
            GetVASErrors = function()
                return STORE_VAS_ERRORS
            end,
            GetBnetTransferInfo = function()
                return STORE_BNET_TRANSFER_INFO.guid, STORE_BNET_TRANSFER_INFO.gameAccounts
            end,
            GetUnrevokedBoostInfo = function()
                return STORE_UNREVOKED_BOOST_INFO.productName, STORE_UNREVOKED_BOOST_INFO.characterName, STORE_UNREVOKED_BOOST_INFO.realmName
            end,
            GetVASCompletionInfo = function()
                if not STORE_VAS_COMPLETION_INFO then
                    return nil
                end
                return STORE_VAS_COMPLETION_INFO.productID,
                    STORE_VAS_COMPLETION_INFO.guid,
                    STORE_VAS_COMPLETION_INFO.realmName,
                    STORE_VAS_COMPLETION_INFO.shouldHandle
            end,
            GetVASRealmList = function()
                return STORE_REALMS
            end,
            AckFailure = function()
                STORE_FAILURE_INFO = nil
                STORE_VAS_ERRORS = {}
            end,
            ClearPreGeneratedExternalTransactionID = function()
                STORE_PREGENERATED_EXTERNAL_TRANSACTION_ID = nil
            end,
            GetCharactersForRealm = function(realmAddress, isGuildVAS)
                local characters = STORE_CHARACTERS_BY_REALM[realmAddress] or {}
                if not isGuildVAS then
                    return characters
                end

                local guildCharacters = {}
                for _, character in ipairs(characters) do
                    if character.isGuildMaster then
                        table.insert(guildCharacters, character)
                    end
                end
                return guildCharacters
            end,
            GetEligibleRacesForVASService = function(characterGUID, _serviceType)
                return STORE_ELIGIBLE_RACES[characterGUID] or {}
            end,
            GetRealmList = function()
                return STORE_REALMS
            end,
            SetDisconnectOnLogout = function(flag)
                STORE_DISCONNECT_ON_LOGOUT = flag and true or false
                if STORE_VAS_COMPLETION_INFO then
                    STORE_VAS_COMPLETION_INFO.shouldHandle = STORE_DISCONNECT_ON_LOGOUT
                end
            end,
            ValidateBnetTransfer = function(email)
                local hasError = email == nil or email == "" or email == "invalid@example.com"
                if hasError then
                    STORE_BNET_TRANSFER_INFO = { guid = 0, gameAccounts = {} }
                else
                    STORE_BNET_TRANSFER_INFO = {
                        guid = DEFAULT_BNET_TRANSFER_INFO.guid,
                        gameAccounts = { unpack(DEFAULT_BNET_TRANSFER_INFO.gameAccounts) },
                    }
                end
                fire_store_event("VAS_TRANSFER_VALIDATION_UPDATE", hasError)
                return not hasError
            end,
            GetConfirmationInfo = function()
                if not STORE_CONFIRMATION_INFO then
                    return nil
                end
                return STORE_CONFIRMATION_INFO.productID,
                    STORE_CONFIRMATION_INFO.walletName,
                    nil,
                    nil,
                    STORE_CONFIRMATION_INFO.currentDollars,
                    STORE_CONFIRMATION_INFO.currentCents
            end,
            GetLastProductListResponseError = function()
                return STORE_LAST_PRODUCT_LIST_RESPONSE_ERROR
            end,
            GetVASGuildMasterInfoForCharacterByGUID = function(characterGUID)
                return STORE_GUILD_MASTER_INFO[characterGUID]
            end,
            GetVasServiceType = function(productID)
                local productInfo = STORE_PRODUCT_INFO[productID]
                return productInfo and productInfo.sharedData and productInfo.sharedData.vasServiceType or nil
            end,
            IsRegionLocked = function()
                return false
            end,
            OpenNydusLink = function(entryID)
                local entryInfo = STORE_ENTRIES[entryID]
                if entryInfo then
                    STORE_CONFIRMATION_INFO = build_confirmation_info(entryInfo.productID)
                end
                return true
            end,
            PurchaseVASProduct = function(productID, characterGUID, _newCharacterName, _oldGuildNewName, _newGuildMaster, destinationRealmAddress, _wowAccountGUID, _bnetAccountGUID, _transferFactionChangeBundle, _isGuildFollow)
                if not STORE_PRODUCT_INFO[productID] or not STORE_CHARACTERS_BY_GUID[characterGUID] then
                    STORE_FAILURE_INFO = { Enum.StoreError.Other, "InvalidVASPurchase" }
                    STORE_VAS_ERRORS = { Enum.StoreError.Other }
                    return false
                end

                if STORE_PREGENERATED_EXTERNAL_TRANSACTION_ID then
                    STORE_FAILURE_INFO = { Enum.StoreError.Other, "DuplicateVASPurchase" }
                    STORE_VAS_ERRORS = { Enum.StoreError.Other }
                    return false
                end

                STORE_PREGENERATED_EXTERNAL_TRANSACTION_ID = string.format("vas-%d-%d", productID, characterGUID)
                STORE_CONFIRMATION_INFO = build_confirmation_info(productID)
                STORE_FAILURE_INFO = nil
                STORE_VAS_ERRORS = {}
                STORE_VAS_COMPLETION_INFO = {
                    productID = productID,
                    guid = characterGUID,
                    realmName = find_realm_name_by_address(destinationRealmAddress) or STORE_CHARACTERS_BY_GUID[characterGUID].realmName,
                    shouldHandle = STORE_DISCONNECT_ON_LOGOUT,
                }
                return true
            end,
            RequestCharacterGuildFollowInfo = function(characterGUID, _realmAddress)
                local guildFollowInfo = STORE_GUILD_FOLLOW_INFO[characterGUID] or { transferredRealm = nil, factionChanged = false }
                fire_store_event("STORE_GUILD_FOLLOW_INFO_RECEIVED", characterGUID, guildFollowInfo)
                return guildFollowInfo
            end,
            RequestRealmGuildMasterInfo = function(realmAddress)
                fire_store_event("STORE_GUILD_MASTER_INFO_RECEIVED", realmAddress)
                return true
            end,
            SetVASProductReady = function(flag)
                STORE_VAS_PRODUCT_READY = flag and true or false
                if STORE_VAS_PRODUCT_READY and STORE_VAS_COMPLETION_INFO then
                    fire_store_event("STORE_VAS_PURCHASE_COMPLETE")
                end
            end,
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
