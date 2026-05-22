//! Post-load workarounds that are still required on the live rilua path.

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
