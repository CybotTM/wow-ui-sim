//! Temporary MapCanvas data-provider pin back-reference repair.
//!
//! Some map pins expect `pin.dataProvider` after provider attachment. Keep this
//! startup repair isolated until provider/pin ownership is modeled natively.

use crate::lua_api::{LoaderEnv, WowLuaEnv};

const MAP_CANVAS_DATA_PROVIDER_WORKAROUND_LUA: &str = r#"
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

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(MAP_CANVAS_DATA_PROVIDER_WORKAROUND_LUA);
}

pub(crate) fn patch_for_runtime_addon_load(env: &LoaderEnv<'_>) {
    let _ = env.exec(MAP_CANVAS_DATA_PROVIDER_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_existing_world_map_provider_pin_backrefs() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            provider = { pin = {} }
            WorldMapFrame = {
                dataProviders = {
                    [provider] = true,
                },
            }
            "#,
        )
        .expect("map provider fixture should install");

        patch(&env);

        let has_backref: bool = env
            .eval("return provider.pin.dataProvider == provider")
            .expect("provider pin backref should be readable");

        assert!(has_backref);
    }

    #[test]
    fn wraps_live_map_add_data_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            add_calls = 0
            provider = { pin = {} }
            WorldMapFrame = {
                AddDataProvider = function(self, dataProvider, marker)
                    add_calls = add_calls + 1
                    self.lastProvider = dataProvider
                    self.lastMarker = marker
                    return "added"
                end,
            }
            "#,
        )
        .expect("live map add provider fixture should install");

        patch(&env);

        let (result, add_calls, marker, has_backref): (String, i64, String, bool) = env
            .eval(
                r#"
                local result = WorldMapFrame:AddDataProvider(provider, "marker")
                return result,
                    add_calls,
                    WorldMapFrame.lastMarker,
                    provider.pin.dataProvider == provider
                "#,
            )
            .expect("wrapped map AddDataProvider should run");

        assert_eq!(result, "added");
        assert_eq!(add_calls, 1);
        assert_eq!(marker, "marker");
        assert!(has_backref);
    }

    #[test]
    fn wraps_map_canvas_mixin_add_data_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            mixin_add_calls = 0
            provider = { pin = {} }
            MapCanvasMixin = {
                AddDataProvider = function(self, dataProvider)
                    mixin_add_calls = mixin_add_calls + 1
                    self.lastProvider = dataProvider
                    return "mixin-added"
                end,
            }
            "#,
        )
        .expect("map canvas mixin fixture should install");

        patch(&env);

        let (result, add_calls, has_backref): (String, i64, bool) = env
            .eval(
                r#"
                local map = {}
                local result = MapCanvasMixin.AddDataProvider(map, provider)
                return result,
                    mixin_add_calls,
                    provider.pin.dataProvider == provider
                "#,
            )
            .expect("wrapped mixin AddDataProvider should run");

        assert_eq!(result, "mixin-added");
        assert_eq!(add_calls, 1);
        assert!(has_backref);
    }
}
