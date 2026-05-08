//! Post-load workarounds that are still required on the live rilua path.

pub(super) const CATALOG_SHOP_PRODUCT_CARD_DEFAULTS_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_catalog_shop_product_card_defaults_wrapped") then
        return
    end

    if type(CatalogShopDefaultProductCardMixin) ~= "table"
        or type(CatalogShopDefaultProductCardMixin.Layout) ~= "function" then
        return
    end

    local original_layout = CatalogShopDefaultProductCardMixin.Layout

    local function resolve_product_id(card)
        if type(card.productInfo) == "table"
            and type(card.productInfo.catalogShopProductID) == "number" then
            return card.productInfo.catalogShopProductID
        end

        if type(card.GetElementData) == "function" then
            local elementData = card:GetElementData()
            if type(elementData) == "table" then
                local productID = elementData.catalogShopProductID or elementData.productID
                if type(productID) == "number" then
                    if type(card.productInfo) == "table" then
                        card.productInfo.catalogShopProductID = productID
                    end
                    return productID
                end
            end
        end

        return nil
    end

    CatalogShopDefaultProductCardMixin.Layout = function(self, ...)
        if resolve_product_id(self) == nil then
            return
        end
        return original_layout(self, ...)
    end

    rawset(_G, "__wow_catalog_shop_product_card_defaults_wrapped", true)
"#;

pub(super) const ITEM_QUALITY_COLOR_DATA_METHODS_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_item_quality_color_data_methods_wrapped") then
        return
    end

    local function ensureColorDataMethods(colorData)
        if type(colorData) ~= "table" then
            return
        end

        if type(colorData.GetRGB) ~= "function" then
            function colorData:GetRGB()
                return self.r, self.g, self.b
            end
        end

        if type(colorData.GetRGBA) ~= "function" then
            function colorData:GetRGBA()
                return self.r, self.g, self.b, self.a or 1
            end
        end
    end

    local function ensureAllItemQualityColorMethods()
        if type(ITEM_QUALITY_COLORS) ~= "table" then
            return
        end

        for _, colorData in pairs(ITEM_QUALITY_COLORS) do
            ensureColorDataMethods(colorData)
        end
    end

    ensureAllItemQualityColorMethods()

    if type(ColorManager) == "table" and type(ColorManager.UpdateColorsForItemQuality) == "function" then
        local originalUpdateColorsForItemQuality = ColorManager.UpdateColorsForItemQuality
        function ColorManager.UpdateColorsForItemQuality(...)
            originalUpdateColorsForItemQuality(...)
            ensureAllItemQualityColorMethods()
        end
    end

    rawset(_G, "__wow_item_quality_color_data_methods_wrapped", true)
"#;

pub(super) const ARTIFACT_UI_SHOW_PANEL_GUARD_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_artifact_ui_show_panel_guard_wrapped") then
        return
    end

    if type(ShowUIPanel) ~= "function" then
        return
    end

    local originalShowUIPanel = ShowUIPanel

    local function shouldBlockArtifactPanel(frame)
        return frame == ArtifactFrame
            and type(ArtifactUI_CanViewArtifact) == "function"
            and not ArtifactUI_CanViewArtifact()
    end

    local function callArtifactShowFailedFunc()
        local entry = type(UIPanelWindows) == "table" and UIPanelWindows["ArtifactFrame"] or nil
        local showFailedFunc = type(entry) == "table" and entry.showFailedFunc or nil
        if type(showFailedFunc) == "function" then
            showFailedFunc()
        end
    end

    ShowUIPanel = function(frame, ...)
        if frame and frame:IsShown() then
            return originalShowUIPanel(frame, ...)
        end

        if shouldBlockArtifactPanel(frame) then
            callArtifactShowFailedFunc()
            return
        end

        return originalShowUIPanel(frame, ...)
    end

    rawset(_G, "__wow_artifact_ui_show_panel_guard_wrapped", true)
"#;

pub(super) const AUCTION_HOUSE_CATEGORIES_REFRESH_COUNT_WORKAROUND_LUA: &str = r#"
    local function getNumElementsForRefresh()
        return type(AuctionCategories) == "table" and #AuctionCategories or 0
    end

    local categoriesList = AuctionHouseFrame and AuctionHouseFrame.CategoriesList or nil
    if type(categoriesList) == "table"
        and type(categoriesList.GetNumElementsForRefresh) ~= "function" then
        categoriesList.GetNumElementsForRefresh = getNumElementsForRefresh
    end

    if type(AuctionHouseCategoriesListMixin) ~= "table" then
        return
    end

    if type(AuctionHouseCategoriesListMixin.GetNumElementsForRefresh) == "function" then
        return
    end

    AuctionHouseCategoriesListMixin.GetNumElementsForRefresh = getNumElementsForRefresh
"#;

pub(super) const AUCTION_HOUSE_BROWSE_RESULTS_EVENT_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_auction_house_browse_results_event_wrapped") then
        return
    end

    if type(AuctionHouseFrameMixin) ~= "table" then
        return
    end

    local browseResultsEvent = "AUCTION_HOUSE_BROWSE_RESULTS_UPDATED"

    local function registerBrowseResultsEvent(frame)
        if type(frame) == "table" and type(frame.RegisterEvent) == "function" then
            frame:RegisterEvent(browseResultsEvent)
        end
    end

    local function unregisterBrowseResultsEvent(frame)
        if type(frame) == "table" and type(frame.UnregisterEvent) == "function" then
            frame:UnregisterEvent(browseResultsEvent)
        end
    end

    local originalOnShow = AuctionHouseFrameMixin.OnShow
    local originalOnHide = AuctionHouseFrameMixin.OnHide

    AuctionHouseFrameMixin.OnShow = function(self, ...)
        if type(originalOnShow) == "function" then
            originalOnShow(self, ...)
        end
        registerBrowseResultsEvent(self)
    end

    AuctionHouseFrameMixin.OnHide = function(self, ...)
        unregisterBrowseResultsEvent(self)
        if type(originalOnHide) == "function" then
            originalOnHide(self, ...)
        end
    end

    local frame = AuctionHouseFrame
    if type(frame) == "table" then
        local frameOnShow = frame:GetScript("OnShow")
        frame:SetScript("OnShow", function(self, ...)
            if type(frameOnShow) == "function" then
                frameOnShow(self, ...)
            end
            registerBrowseResultsEvent(self)
        end)

        local frameOnHide = frame:GetScript("OnHide")
        frame:SetScript("OnHide", function(self, ...)
            unregisterBrowseResultsEvent(self)
            if type(frameOnHide) == "function" then
                frameOnHide(self, ...)
            end
        end)

        registerBrowseResultsEvent(frame)
    end

    rawset(_G, "__wow_auction_house_browse_results_event_wrapped", true)
"#;

pub(super) const AUCTION_HOUSE_SEARCH_CONTEXT_ALIASES_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_auction_house_search_context_aliases_patched") then
        return
    end

    if type(AuctionHouseSearchContext) ~= "table" then
        return
    end

    if AuctionHouseSearchContext.Auctions == nil then
        AuctionHouseSearchContext.Auctions = AuctionHouseSearchContext.AllAuctions
    end

    if AuctionHouseSearchContext.BrowseFavorites == nil then
        AuctionHouseSearchContext.BrowseFavorites = AuctionHouseSearchContext.AllFavorites
    end

    rawset(_G, "__wow_auction_house_search_context_aliases_patched", true)
"#;

pub(super) const AUTH_CHALLENGE_FRAME_PARENT_WORKAROUND_LUA: &str = r#"
    local authChallengeFunctions = {
        "AuthChallengeUI_OnLoad",
        "AuthChallengeUI_Submit",
        "AuthChallengeUI_Cancel",
        "AuthChallengeUI_OnTabPressed",
        "AuthChallengeUI_OnKeyDown",
    }

    for _, functionName in ipairs(authChallengeFunctions) do
        if rawget(_G, functionName) == nil
            and type(__secureenv) == "table"
            and type(rawget(__secureenv, functionName)) == "function" then
            rawset(_G, functionName, rawget(__secureenv, functionName))
        end
    end

    if type(AuthChallengeFrame) ~= "table" or type(UIParent) ~= "table" then
        return
    end

    if AuthChallengeFrame:GetParent() ~= UIParent then
        AuthChallengeFrame:SetParent(UIParent)
    end

    local inputFrame = AuthChallengeFrame.InputFrame
    if inputFrame and inputFrame.Submit == nil and type(inputFrame.GetChildren) == "function" then
        for _, child in ipairs({ inputFrame:GetChildren() }) do
            if type(child.GetObjectType) == "function"
                and child:GetObjectType() == "Button"
                and type(child.GetText) == "function"
                and child:GetText() == BLIZZARD_CHALLENGE_SUBMIT then
                inputFrame.Submit = child
                break
            end
        end
    end
"#;
