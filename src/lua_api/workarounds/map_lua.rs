pub(super) const MAP_EXPLORATION_PIN_WORKAROUND_LUA: &str = r#"
local function __wow_size_map_exploration_pin(pin)
    if type(pin) ~= "table" then
        return
    end
    if type(pin.OnCanvasSizeChanged) == "function" then
        pin:OnCanvasSizeChanged()
    end
end

local function __wow_finalize_map_exploration_pin_waiting(pin)
    if type(pin) ~= "table" then
        return
    end

    if not rawget(pin, "isWaitingForLoad") then
        return
    end

    local map = type(pin.GetMap) == "function" and pin:GetMap() or nil
    local detailLayersLoaded = type(map) == "table"
        and type(map.AreDetailLayersLoaded) == "function"
        and map:AreDetailLayersLoaded()
    local textureLoadGroup = rawget(pin, "textureLoadGroup")
    local texturesLoaded = type(textureLoadGroup) == "table"
        and type(textureLoadGroup.IsFullyLoaded) == "function"
        and textureLoadGroup:IsFullyLoaded()
    local overlayPool = rawget(pin, "overlayTexturePool")
    local hasOverlayTextures = type(overlayPool) == "table"
        and type(overlayPool.GetNumActive) == "function"
        and overlayPool:GetNumActive() > 0

    if detailLayersLoaded and (texturesLoaded or hasOverlayTextures) then
        if type(pin.RefreshAlpha) == "function" then
            pin:RefreshAlpha()
        end
        pin.isWaitingForLoad = nil
        if type(textureLoadGroup) == "table" and type(textureLoadGroup.Reset) == "function" then
            textureLoadGroup:Reset()
        end
        return
    end

    if type(pin.Show) == "function" and type(pin.IsShown) == "function" and not pin:IsShown() then
        pin:Show()
    end
end

local function __wow_map_exploration_pin_overlay_count(pin)
    if type(pin) ~= "table" then
        return 0
    end
    local overlayPool = rawget(pin, "overlayTexturePool")
    if type(overlayPool) ~= "table" or type(overlayPool.GetNumActive) ~= "function" then
        return 0
    end
    return overlayPool:GetNumActive()
end

local function __wow_should_retry_map_exploration_pin_overlay_refresh(pin)
    if type(pin) ~= "table" then
        return false
    end

    local map = type(pin.GetMap) == "function" and pin:GetMap() or nil
    local mapID = type(map) == "table" and type(map.GetMapID) == "function" and map:GetMapID() or nil
    if type(mapID) ~= "number" or mapID == 0 then
        return false
    end

    if type(C_MapExplorationInfo) ~= "table" or type(C_MapExplorationInfo.GetExploredMapTextures) ~= "function" then
        return false
    end

    local exploredMapTextures = C_MapExplorationInfo.GetExploredMapTextures(mapID)
    if type(exploredMapTextures) ~= "table" or #exploredMapTextures == 0 then
        return false
    end

    return __wow_map_exploration_pin_overlay_count(pin) == 0
end

local function __wow_schedule_map_exploration_pin_finalize_retry(pin)
    if type(pin) ~= "table" or not rawget(pin, "isWaitingForLoad") then
        return
    end

    if rawget(pin, "__wow_finalize_retry_pending") then
        return
    end
    if type(C_Timer) ~= "table" or type(C_Timer.After) ~= "function" then
        return
    end

    rawset(pin, "__wow_finalize_retry_pending", true)
    C_Timer.After(0, function()
        if type(pin) ~= "table" then
            return
        end
        rawset(pin, "__wow_finalize_retry_pending", nil)
        __wow_finalize_map_exploration_pin_waiting(pin)
    end)
end

local function __wow_schedule_map_exploration_pin_overlay_retry(pin)
    if type(pin) ~= "table" then
        return
    end

    if not __wow_should_retry_map_exploration_pin_overlay_refresh(pin) then
        return
    end

    if rawget(pin, "__wow_overlay_retry_pending") then
        return
    end
    if type(C_Timer) ~= "table" or type(C_Timer.After) ~= "function" then
        return
    end

    rawset(pin, "__wow_overlay_retry_pending", true)
    C_Timer.After(0, function()
        if type(pin) ~= "table" then
            return
        end

        rawset(pin, "__wow_overlay_retry_pending", nil)
        if not __wow_should_retry_map_exploration_pin_overlay_refresh(pin) then
            return
        end

        if type(pin.RefreshOverlays) == "function" then
            pin:RefreshOverlays(true)
        end

        __wow_finalize_map_exploration_pin_waiting(pin)
        __wow_schedule_map_exploration_pin_finalize_retry(pin)
    end)
end

local function __wow_patch_live_map_exploration_pins(map)
    if type(map) ~= "table" then
        return
    end

    if type(map.EnumeratePinsByTemplate) == "function" then
        for pin in map:EnumeratePinsByTemplate("MapExplorationPinTemplate") do
            __wow_size_map_exploration_pin(pin)
            __wow_schedule_map_exploration_pin_overlay_retry(pin)
        end
    end

    if type(map.dataProviders) ~= "table" then
        return
    end

    for provider in pairs(map.dataProviders) do
        local pin = type(provider) == "table" and rawget(provider, "pin") or nil
        if type(pin) == "table"
            and type(pin.RefreshOverlays) == "function"
            and type(pin.OnCanvasSizeChanged) == "function"
        then
            if type(MapExplorationPinMixin) == "table" and type(MapExplorationPinMixin.RefreshOverlays) == "function" then
                pin.RefreshOverlays = MapExplorationPinMixin.RefreshOverlays
            end
            __wow_size_map_exploration_pin(pin)
            __wow_schedule_map_exploration_pin_overlay_retry(pin)
        end
    end
end

if type(MapExplorationPinMixin) == "table" and not rawget(_G, "__wow_map_exploration_pin_patched") then
    if type(MapExplorationPinMixin.OnAcquired) == "function" then
        local originalOnAcquired = MapExplorationPinMixin.OnAcquired
        MapExplorationPinMixin.OnAcquired = function(self, dataProvider)
            originalOnAcquired(self, dataProvider)
            __wow_size_map_exploration_pin(self)
            __wow_finalize_map_exploration_pin_waiting(self)
            __wow_schedule_map_exploration_pin_finalize_retry(self)
            __wow_schedule_map_exploration_pin_overlay_retry(self)
        end
    end

    if type(MapExplorationPinMixin.RefreshOverlays) == "function" then
        local originalRefreshOverlays = MapExplorationPinMixin.RefreshOverlays
        MapExplorationPinMixin.RefreshOverlays = function(self, fullUpdate)
            __wow_size_map_exploration_pin(self)
            local map = type(self.GetMap) == "function" and self:GetMap() or nil
            local container = type(map) == "table" and type(map.GetCanvasContainer) == "function" and map:GetCanvasContainer() or nil
            if type(container) == "table" then
                local ensureZoomLevels = rawget(_G, "__wow_ensure_map_canvas_zoom_levels")
                if type(ensureZoomLevels) == "function" then
                    ensureZoomLevels(container)
                end
                local zoomLevels = rawget(container, "zoomLevels")
                if (type(zoomLevels) ~= "table" or type(zoomLevels[1]) ~= "table")
                    and type(container.CreateZoomLevels) == "function"
                then
                    pcall(container.CreateZoomLevels, container)
                    zoomLevels = rawget(container, "zoomLevels")
                end
                if type(zoomLevels) ~= "table" or type(zoomLevels[1]) ~= "table" then
                    rawset(container, "zoomLevels", { { scale = 1.0, layerIndex = 1 } })
                end
            end
            local result = originalRefreshOverlays(self, fullUpdate)
            __wow_finalize_map_exploration_pin_waiting(self)
            __wow_schedule_map_exploration_pin_finalize_retry(self)
            __wow_schedule_map_exploration_pin_overlay_retry(self)
            return result
        end
    end

    if type(MapExplorationPinMixin.OnUpdate) == "function" then
        local originalOnUpdate = MapExplorationPinMixin.OnUpdate
        MapExplorationPinMixin.OnUpdate = function(self, elapsed)
            if rawget(self, "isWaitingForLoad")
                and type(self.Show) == "function"
                and type(self.IsShown) == "function"
                and not self:IsShown()
            then
                self:Show()
            end
            local result = originalOnUpdate(self, elapsed)
            __wow_finalize_map_exploration_pin_waiting(self)
            __wow_schedule_map_exploration_pin_overlay_retry(self)
            return result
        end
    end

    rawset(_G, "__wow_map_exploration_pin_patched", true)
end

for _, mapName in ipairs({ "WorldMapFrame", "BattlefieldMapFrame" }) do
    __wow_patch_live_map_exploration_pins(_G[mapName])
end
"#;

pub(super) const MAP_CANVAS_DATA_PROVIDER_WORKAROUND_LUA: &str = r#"
local function __wow_fix_provider_pin(provider)
    if type(provider) ~= "table" then
        return
    end

    local pin = provider.pin
    if pin ~= nil then
        pin.dataProvider = provider
    end
end

if type(MapCanvasMixin) == "table" and not rawget(_G, "__wow_map_canvas_add_data_provider_patched") then
    if type(MapCanvasMixin.AddDataProvider) == "function" then
        local originalAddDataProvider = MapCanvasMixin.AddDataProvider
        MapCanvasMixin.AddDataProvider = function(self, dataProvider, ...)
            local result = originalAddDataProvider(self, dataProvider, ...)
            __wow_fix_provider_pin(dataProvider)
            return result
        end
    end

    rawset(_G, "__wow_map_canvas_add_data_provider_patched", true)
end

for _, mapName in ipairs({ "WorldMapFrame", "BattlefieldMapFrame" }) do
    local map = rawget(_G, mapName)
    if type(map) == "table" then
        if type(map.AddDataProvider) == "function" and rawget(map, "__wow_add_data_provider_patched") ~= true then
            local originalAddDataProvider = map.AddDataProvider
            map.AddDataProvider = function(self, dataProvider, ...)
                local result = originalAddDataProvider(self, dataProvider, ...)
                __wow_fix_provider_pin(dataProvider)
                return result
            end
            rawset(map, "__wow_add_data_provider_patched", true)
        end

        if type(map.dataProviders) == "table" then
            for provider in pairs(map.dataProviders) do
                __wow_fix_provider_pin(provider)
            end
        end
    end
end
"#;
