//! Temporary Fog of War pin refresh repair.
//!
//! Fog of War pins need simulator-side nil-safe refresh behavior until the map
//! pin/data-provider state matches Blizzard's startup expectations natively.

use crate::lua_api::{LoaderEnv, WowLuaEnv};

const FOG_OF_WAR_PIN_WORKAROUND_LUA: &str = r#"
local function __wow_clear_fog_of_war_pin_assets(pin)
    if type(pin) ~= "table" then
        return
    end
    if type(pin.SetFogOfWarID) == "function" then
        pin:SetFogOfWarID(nil, true)
    end
    if type(pin.SetFogOfWarBackgroundAtlas) == "function" then
        pin:SetFogOfWarBackgroundAtlas(nil)
    end
    if type(pin.SetFogOfWarMaskAtlas) == "function" then
        pin:SetFogOfWarMaskAtlas(nil)
    end
end

local function __wow_resolve_fog_of_war_map_id(pin)
    local mapID = nil
    if type(pin) == "table" and type(pin.GetMap) == "function" then
        local map = pin:GetMap()
        if map ~= nil and type(map.GetMapID) == "function" then
            mapID = map:GetMapID()
        end
    end

    if (mapID == nil or mapID == 0) and C_Map ~= nil and type(C_Map.GetCurrentMapID) == "function" then
        mapID = C_Map.GetCurrentMapID()
    end

    return mapID or 0
end

local function __wow_refresh_fog_of_war_pin(pin, forceUpdate)
    if type(pin) ~= "table" then
        return
    end

    local mapID = __wow_resolve_fog_of_war_map_id(pin)
    if type(pin.SetUiMapID) == "function" then
        pin:SetUiMapID(mapID)
    end

    if mapID == 0 then
        __wow_clear_fog_of_war_pin_assets(pin)
        if type(pin.Hide) == "function" then
            pin:Hide()
        end
        return
    end

    local fogOfWarID = nil
    if C_FogOfWar ~= nil and type(C_FogOfWar.GetFogOfWarForMap) == "function" then
        fogOfWarID = C_FogOfWar.GetFogOfWarForMap(mapID)
    end
    if type(pin.SetFogOfWarID) == "function" then
        pin:SetFogOfWarID(fogOfWarID, forceUpdate)
    end

    local hasBackgroundAtlas =
        type(pin.GetFogOfWarBackgroundAtlas) == "function" and pin:GetFogOfWarBackgroundAtlas() ~= nil
    local hasMaskAtlas =
        type(pin.GetFogOfWarMaskAtlas) == "function" and pin:GetFogOfWarMaskAtlas() ~= nil
    if fogOfWarID == nil or (not hasBackgroundAtlas and not hasMaskAtlas) then
        __wow_clear_fog_of_war_pin_assets(pin)
        if type(pin.Hide) == "function" then
            pin:Hide()
        end
    end
end

local function __wow_apply_fog_of_war_pin_workaround(pin)
    if type(pin) ~= "table" then
        return
    end
    if type(FogOfWarPinMixin) == "table" and type(FogOfWarPinMixin.OnMapChanged) == "function" then
        pin.OnMapChanged = FogOfWarPinMixin.OnMapChanged
    end
    if type(FogOfWarFrameMixin) == "table" and type(FogOfWarFrameMixin.TryFindingBestFogOfWarID) == "function" then
        pin.TryFindingBestFogOfWarID = FogOfWarFrameMixin.TryFindingBestFogOfWarID
    end
    __wow_refresh_fog_of_war_pin(pin, true)
end

local function __wow_patch_live_fog_of_war_pins(map)
    if type(map) ~= "table" then
        return
    end

    if type(map.EnumeratePinsByTemplate) == "function" then
        for pin in map:EnumeratePinsByTemplate("FogOfWarPinTemplate") do
            __wow_apply_fog_of_war_pin_workaround(pin)
        end
    end

    if type(map.dataProviders) ~= "table" then
        return
    end

    for provider in pairs(map.dataProviders) do
        local pin = type(provider) == "table" and rawget(provider, "pin") or nil
        if type(pin) == "table" then
            __wow_apply_fog_of_war_pin_workaround(pin)
        end
    end
end

if type(FogOfWarPinMixin) == "table" and not rawget(_G, "__wow_fog_of_war_pin_methods_patched") then
    if type(FogOfWarFrameMixin) == "table" and type(FogOfWarFrameMixin.TryFindingBestFogOfWarID) == "function" then
        FogOfWarFrameMixin.TryFindingBestFogOfWarID = function(self, forceUpdate)
            __wow_refresh_fog_of_war_pin(self, forceUpdate)
        end
    end

    if type(FogOfWarPinMixin.OnMapChanged) == "function" then
        FogOfWarPinMixin.OnMapChanged = function(self)
            __wow_refresh_fog_of_war_pin(self, true)
        end
    end

    rawset(_G, "__wow_fog_of_war_pin_methods_patched", true)
end

for _, mapName in ipairs({ "WorldMapFrame", "BattlefieldMapFrame" }) do
    __wow_patch_live_fog_of_war_pins(_G[mapName])
end
"#;

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(FOG_OF_WAR_PIN_WORKAROUND_LUA);
}

pub(crate) fn patch_for_runtime_addon_load(env: &LoaderEnv<'_>) {
    let _ = env.exec(FOG_OF_WAR_PIN_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn install_fog_of_war_fixture(env: &WowLuaEnv) {
        env.exec(
            r#"
            C_Map = {
                GetCurrentMapID = function()
                    return 99
                end,
            }
            C_FogOfWar = {
                GetFogOfWarForMap = function(mapID)
                    return mapID + 1000
                end,
            }
            FogOfWarPinMixin = {
                OnMapChanged = function() end,
            }
            FogOfWarFrameMixin = {
                TryFindingBestFogOfWarID = function() end,
            }
            "#,
        )
        .expect("fog of war fixture should install");
    }

    #[test]
    fn refreshes_pin_from_own_map() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_fog_of_war_fixture(&env);
        patch(&env);

        let (ui_map_id, fog_id, force_update, hidden): (i64, i64, bool, bool) = env
            .eval(
                r#"
                local pin = {
                    hidden = false,
                    GetMap = function()
                        return {
                            GetMapID = function()
                                return 12
                            end,
                        }
                    end,
                    SetUiMapID = function(self, mapID)
                        self.uiMapID = mapID
                    end,
                    SetFogOfWarID = function(self, fogID, forceUpdate)
                        self.fogID = fogID
                        self.forceUpdate = forceUpdate
                    end,
                    GetFogOfWarBackgroundAtlas = function()
                        return "background"
                    end,
                    GetFogOfWarMaskAtlas = function()
                        return nil
                    end,
                    Hide = function(self)
                        self.hidden = true
                    end,
                }
                FogOfWarPinMixin.OnMapChanged(pin)
                return pin.uiMapID, pin.fogID, pin.forceUpdate, pin.hidden
                "#,
            )
            .expect("fog of war pin should refresh from own map");

        assert_eq!(ui_map_id, 12);
        assert_eq!(fog_id, 1012);
        assert!(force_update);
        assert!(!hidden);
    }

    #[test]
    fn hides_pin_when_no_map_id_is_available() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_fog_of_war_fixture(&env);
        env.exec("C_Map.GetCurrentMapID = function() return 0 end")
            .expect("current map fixture should update");
        patch(&env);

        let (ui_map_id, fog_cleared, background_cleared, mask_cleared, hidden): (
            i64,
            bool,
            bool,
            bool,
            bool,
        ) = env
            .eval(
                r#"
                local pin = {
                    hidden = false,
                    SetUiMapID = function(self, mapID)
                        self.uiMapID = mapID
                    end,
                    SetFogOfWarID = function(self, fogID)
                        self.fogCleared = fogID == nil
                    end,
                    SetFogOfWarBackgroundAtlas = function(self, atlas)
                        self.backgroundCleared = atlas == nil
                    end,
                    SetFogOfWarMaskAtlas = function(self, atlas)
                        self.maskCleared = atlas == nil
                    end,
                    Hide = function(self)
                        self.hidden = true
                    end,
                }
                FogOfWarPinMixin.OnMapChanged(pin)
                return pin.uiMapID,
                    pin.fogCleared,
                    pin.backgroundCleared,
                    pin.maskCleared,
                    pin.hidden
                "#,
            )
            .expect("fog of war pin should hide without map ID");

        assert_eq!(ui_map_id, 0);
        assert!(fog_cleared);
        assert!(background_cleared);
        assert!(mask_cleared);
        assert!(hidden);
    }

    #[test]
    fn applies_methods_to_live_world_map_pin() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_fog_of_war_fixture(&env);
        env.exec(
            r#"
            livePin = {
                GetMap = function()
                    return {
                        GetMapID = function()
                            return 7
                        end,
                    }
                end,
                SetUiMapID = function(self, mapID)
                    self.uiMapID = mapID
                end,
                SetFogOfWarID = function(self, fogID)
                    self.fogID = fogID
                end,
                GetFogOfWarBackgroundAtlas = function()
                    return "background"
                end,
                GetFogOfWarMaskAtlas = function()
                    return nil
                end,
            }
            WorldMapFrame = {
                EnumeratePinsByTemplate = function()
                    local yielded = false
                    return function()
                        if yielded then
                            return nil
                        end
                        yielded = true
                        return livePin
                    end
                end,
            }
            "#,
        )
        .expect("live map pin fixture should install");

        patch(&env);

        let (has_on_map_changed, has_try_finding, ui_map_id, fog_id): (bool, bool, i64, i64) = env
            .eval(
                r#"
                return type(livePin.OnMapChanged) == "function",
                    type(livePin.TryFindingBestFogOfWarID) == "function",
                    livePin.uiMapID,
                    livePin.fogID
                "#,
            )
            .expect("live fog of war pin should be patched");

        assert!(has_on_map_changed);
        assert!(has_try_finding);
        assert_eq!(ui_map_id, 7);
        assert_eq!(fog_id, 1007);
    }
}
