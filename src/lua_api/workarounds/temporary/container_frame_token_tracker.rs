//! Temporary ContainerFrame token tracker bootstrap repair.
//!
//! Startup emits a consolidated `ADDON_LOADED("WoWUISim")` event after
//! bootstrap. Bag setup expects Blizzard_TokenUI's per-addon callback to have
//! initialized `ContainerFrameSettingsManager.TokenTracker`.

use crate::lua_api::WowLuaEnv;

const CONTAINER_FRAME_TOKEN_TRACKER_LUA: &str = r#"
if type(ContainerFrameSettingsManager) ~= "table" then
    return
end
if ContainerFrameSettingsManager.TokenTracker ~= nil then
    return
end
if type(ContainerFrameSettingsManager.OnAddonLoaded) ~= "function" then
    return
end

local tokenUiLoaded = false
if type(C_AddOns) == "table" and type(C_AddOns.IsAddOnLoaded) == "function" then
    tokenUiLoaded = C_AddOns.IsAddOnLoaded("Blizzard_TokenUI")
end

if tokenUiLoaded then
    pcall(
        ContainerFrameSettingsManager.OnAddonLoaded,
        ContainerFrameSettingsManager,
        "Blizzard_TokenUI"
    )
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
}
