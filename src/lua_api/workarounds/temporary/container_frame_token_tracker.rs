//! Temporary ContainerFrame token tracker bootstrap repair.
//!
//! Startup emits a consolidated `ADDON_LOADED("WoWUISim")` event after
//! bootstrap. Bag setup expects Blizzard_TokenUI's per-addon callback to have
//! initialized `ContainerFrameSettingsManager.TokenTracker`.

use crate::lua_api::WowLuaEnv;

const CONTAINER_FRAME_TOKEN_TRACKER_LUA: &str = r#"
local tokenUiLoaded = false
if type(C_AddOns) == "table" and type(C_AddOns.IsAddOnLoaded) == "function" then
    tokenUiLoaded = C_AddOns.IsAddOnLoaded("Blizzard_TokenUI")
end

local function ensureBackpackTokenFrame()
    if not tokenUiLoaded or type(BackpackTokenFrame) == "table" then
        return
    end

    if type(BackpackTokenFrameTemplate) == "table" then
        pcall(
            CreateFrame,
            "FRAME",
            "BackpackTokenFrame",
            UIParent,
            "BackpackTokenFrameTemplate"
        )
    end
    pcall(
        function()
            if type(BackpackTokenFrame) ~= "table" then
                BackpackTokenFrame = CreateFrame(
                    "Frame",
                    "BackpackTokenFrame",
                    UIParent
                )
            end
        end
    )
end

if type(ContainerFrameSettingsManager) ~= "table" then
    ensureBackpackTokenFrame()
    return
end
if ContainerFrameSettingsManager.TokenTracker ~= nil and type(BackpackTokenFrame) == "table" then
    return
end

if tokenUiLoaded then
    if type(ContainerFrameSettingsManager.OnAddonLoaded) == "function" then
        pcall(
            ContainerFrameSettingsManager.OnAddonLoaded,
            ContainerFrameSettingsManager,
            "Blizzard_TokenUI"
        )
    end
    ensureBackpackTokenFrame()
    if type(BackpackTokenFrame) == "table" and ContainerFrameSettingsManager.TokenTracker == nil then
        ContainerFrameSettingsManager.TokenTracker = BackpackTokenFrame
    end
end
"#;

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(CONTAINER_FRAME_TOKEN_TRACKER_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_token_ui_addon_loaded_when_tracker_is_missing() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            addon_loaded_calls = {}
            C_AddOns = {
                IsAddOnLoaded = function(addonName)
                    return addonName == "Blizzard_TokenUI"
                end,
            }
            ContainerFrameSettingsManager = {
                OnAddonLoaded = function(self, addonName)
                    table.insert(addon_loaded_calls, addonName)
                    self.TokenTracker = { addonName = addonName }
                end,
            }
            "#,
        )
        .expect("container frame fixture should install");

        patch(&env);

        let (call_count, tracker_addon): (i64, String) = env
            .eval(
                r#"
                return #addon_loaded_calls,
                    ContainerFrameSettingsManager.TokenTracker.addonName
                "#,
            )
            .expect("token tracker state should be readable");

        assert_eq!(call_count, 1);
        assert_eq!(tracker_addon, "Blizzard_TokenUI");
    }

    #[test]
    fn preserves_existing_token_tracker() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            addon_loaded_calls = 0
            existing = { preserved = true }
            C_AddOns = {
                IsAddOnLoaded = function()
                    return true
                end,
            }
            ContainerFrameSettingsManager = {
                TokenTracker = existing,
                OnAddonLoaded = function()
                    addon_loaded_calls = addon_loaded_calls + 1
                end,
            }
            "#,
        )
        .expect("container frame fixture should install");

        patch(&env);

        let (same_tracker, call_count): (bool, i64) = env
            .eval(
                r#"
                return ContainerFrameSettingsManager.TokenTracker == existing,
                    addon_loaded_calls
                "#,
            )
            .expect("preserved token tracker state should be readable");

        assert!(same_tracker);
        assert_eq!(call_count, 0);
    }

    #[test]
    fn skips_callback_until_token_ui_is_loaded() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            addon_loaded_calls = 0
            C_AddOns = {
                IsAddOnLoaded = function()
                    return false
                end,
            }
            ContainerFrameSettingsManager = {
                OnAddonLoaded = function()
                    addon_loaded_calls = addon_loaded_calls + 1
                end,
            }
            "#,
        )
        .expect("container frame fixture should install");

        patch(&env);

        let (has_tracker, call_count): (bool, i64) = env
            .eval(
                r#"
                return ContainerFrameSettingsManager.TokenTracker ~= nil,
                    addon_loaded_calls
                "#,
            )
            .expect("token tracker state should be readable");

        assert!(!has_tracker);
        assert_eq!(call_count, 0);
    }

    #[test]
    fn creates_backpack_token_frame_without_settings_manager() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_AddOns = {
                IsAddOnLoaded = function(addonName)
                    return addonName == "Blizzard_TokenUI"
                end,
            }
            "#,
        )
        .expect("container frame fixture should install");

        patch(&env);

        let has_backpack_token_frame: bool = env
            .eval("return type(BackpackTokenFrame) == 'table'")
            .expect("backpack token frame state should be readable");

        assert!(has_backpack_token_frame);
    }
}
