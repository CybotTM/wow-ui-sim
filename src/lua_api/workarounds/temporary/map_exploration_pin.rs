//! Temporary MapExploration pin refresh repair.
//!
//! Map exploration pins need nil-safe sizing, load-finalization, and retry
//! behavior until map texture/provider state matches Blizzard startup data.

use crate::lua_api::{LoaderEnv, WowLuaEnv};

const MAP_EXPLORATION_PIN_WORKAROUND_LUA: &str = r#"
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

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(MAP_EXPLORATION_PIN_WORKAROUND_LUA);
}

pub(crate) fn patch_for_runtime_addon_load(env: &LoaderEnv<'_>) {
    let _ = env.exec(MAP_EXPLORATION_PIN_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn install_mixin_fixture(env: &WowLuaEnv) {
        env.exec(
            r#"
            acquired_calls = 0
            refresh_calls = 0
            update_calls = 0
            canvas_size_calls = 0
            alpha_calls = 0
            show_calls = 0
            reset_calls = 0
            MapExplorationPinMixin = {
                OnAcquired = function(self, dataProvider)
                    acquired_calls = acquired_calls + 1
                    self.dataProvider = dataProvider
                end,
                RefreshOverlays = function(self, fullUpdate)
                    refresh_calls = refresh_calls + 1
                    self.lastFullUpdate = fullUpdate
                    return "refreshed"
                end,
                OnUpdate = function(self, elapsed)
                    update_calls = update_calls + 1
                    self.elapsed = elapsed
                    return "updated"
                end,
            }
            "#,
        )
        .expect("map exploration mixin fixture should install");
    }

    fn waiting_pin_fixture() -> &'static str {
        r#"
        {
            isWaitingForLoad = true,
            shown = false,
            OnCanvasSizeChanged = function(self)
                canvas_size_calls = canvas_size_calls + 1
            end,
            GetMap = function()
                return {
                    GetMapID = function()
                        return 42
                    end,
                    AreDetailLayersLoaded = function()
                        return true
                    end,
                    GetCanvasContainer = function()
                        return {}
                    end,
                }
            end,
            textureLoadGroup = {
                IsFullyLoaded = function()
                    return true
                end,
                Reset = function()
                    reset_calls = reset_calls + 1
                end,
            },
            overlayTexturePool = {
                GetNumActive = function()
                    return 0
                end,
            },
            RefreshAlpha = function(self)
                alpha_calls = alpha_calls + 1
            end,
            IsShown = function(self)
                return self.shown
            end,
            Show = function(self)
                show_calls = show_calls + 1
                self.shown = true
            end,
        }
        "#
    }

    #[test]
    fn acquired_pin_is_sized_and_finalized() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_mixin_fixture(&env);
        patch(&env);

        let script = r#"
                local pin = __PIN_FIXTURE__
                MapExplorationPinMixin.OnAcquired(pin, "provider")
                return acquired_calls,
                    canvas_size_calls,
                    alpha_calls,
                    pin.isWaitingForLoad == true,
                    reset_calls
                "#
        .replace("__PIN_FIXTURE__", waiting_pin_fixture());

        let (acquired, canvas, alpha, waiting, reset): (i64, i64, i64, bool, i64) = env
            .eval(&script)
            .expect("wrapped OnAcquired should size and finalize");

        assert_eq!(acquired, 1);
        assert_eq!(canvas, 1);
        assert_eq!(alpha, 1);
        assert!(!waiting);
        assert_eq!(reset, 1);
    }

    #[test]
    fn refresh_overlays_ensures_zoom_level_and_returns_original_result() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_mixin_fixture(&env);
        patch(&env);

        let (result, refreshes, zoom_scale, canvas_calls): (String, i64, f64, i64) = env
            .eval(
                r#"
                local container = {}
                local pin = {
                    OnCanvasSizeChanged = function()
                        canvas_size_calls = canvas_size_calls + 1
                    end,
                    GetMap = function()
                        return {
                            GetCanvasContainer = function()
                                return container
                            end,
                        }
                    end,
                }
                local result = MapExplorationPinMixin.RefreshOverlays(pin, true)
                return result,
                    refresh_calls,
                    container.zoomLevels[1].scale,
                    canvas_size_calls
                "#,
            )
            .expect("wrapped RefreshOverlays should ensure zoom levels");

        assert_eq!(result, "refreshed");
        assert_eq!(refreshes, 1);
        assert_eq!(zoom_scale, 1.0);
        assert_eq!(canvas_calls, 1);
    }

    #[test]
    fn update_shows_waiting_hidden_pin_before_original_update() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_mixin_fixture(&env);
        patch(&env);

        let (result, updates, shows, shown): (String, i64, i64, bool) = env
            .eval(
                r#"
                local pin = {
                    isWaitingForLoad = true,
                    shown = false,
                    IsShown = function(self)
                        return self.shown
                    end,
                    Show = function(self)
                        show_calls = show_calls + 1
                        self.shown = true
                    end,
                }
                local result = MapExplorationPinMixin.OnUpdate(pin, 0.25)
                return result, update_calls, show_calls, pin.shown
                "#,
            )
            .expect("wrapped OnUpdate should show waiting hidden pin");

        assert_eq!(result, "updated");
        assert_eq!(updates, 1);
        assert_eq!(shows, 1);
        assert!(shown);
    }
}
