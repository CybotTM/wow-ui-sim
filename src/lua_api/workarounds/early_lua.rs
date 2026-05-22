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
