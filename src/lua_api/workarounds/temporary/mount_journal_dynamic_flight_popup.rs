//! Temporary mount journal dynamic-flight popup workaround.
//!
//! The mount journal's dynamic-flight flyout animation assumes popup children
//! created by the full Collections journal flow. Keep the nil-safe animation
//! bridge isolated until that flow is modeled.

use crate::lua_api::LoaderEnv;

const MOUNT_JOURNAL_DYNAMIC_FLIGHT_POPUP_WORKAROUND_LUA: &str = r#"
local function __wow_patch_mount_journal_dynamic_flight_animation()
    if type(MountJournalToggleDynamicFlightFlyoutButtonMixin) ~= "table" then
        return
    end
    if rawget(_G, "__wow_mount_journal_dynamic_flight_popup_patched") then
        return
    end
    if type(MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation) ~= "function" then
        return
    end

    MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation = function(self)
        local isPopupOpen = type(self.IsPopupOpen) == "function" and self:IsPopupOpen() or false
        if self.UnspentGlyphsAnim and type(self.UnspentGlyphsAnim.SetPlaying) == "function" then
            self.UnspentGlyphsAnim:SetPlaying(self.canSpendDragonridingGlyphs and not isPopupOpen)
        end

        local popup = rawget(self, "popup")
        local popupButton = type(popup) == "table" and rawget(popup, "OpenDynamicFlightSkillTreeButton") or nil
        local popupAnim = popupButton and popupButton.UnspentGlyphsAnim or nil
        if popupAnim and type(popupAnim.SetPlaying) == "function" then
            popupAnim:SetPlaying(self.canSpendDragonridingGlyphs and isPopupOpen)
        end
    end

    rawset(_G, "__wow_mount_journal_dynamic_flight_popup_patched", true)
end

__wow_patch_mount_journal_dynamic_flight_animation()
"#;

pub(crate) fn patch(env: &LoaderEnv<'_>) {
    let _ = env.exec(MOUNT_JOURNAL_DYNAMIC_FLIGHT_POPUP_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn updates_button_and_popup_animation_states() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            local function anim()
                return {
                    calls = {},
                    SetPlaying = function(self, playing)
                        table.insert(self.calls, playing and "true" or "false")
                    end,
                }
            end

            buttonAnim = anim()
            popupAnim = anim()
            MountJournalToggleDynamicFlightFlyoutButtonMixin = {
                UpdateUnspentGlyphsAnimation = function(self)
                    error("original should be replaced")
                end,
            }
            flyoutButton = {
                canSpendDragonridingGlyphs = true,
                popupOpen = false,
                UnspentGlyphsAnim = buttonAnim,
                popup = {
                    OpenDynamicFlightSkillTreeButton = {
                        UnspentGlyphsAnim = popupAnim,
                    },
                },
                IsPopupOpen = function(self)
                    return self.popupOpen
                end,
            }
            "#,
        )
        .expect("mount journal test surface should install");

        let loader_env = LoaderEnv::new(&env);
        patch(&loader_env);

        let (patched, button_closed, popup_closed, button_open, popup_open): (
            bool,
            String,
            String,
            String,
            String,
        ) = env
            .eval(
                r#"
                MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation(flyoutButton)
                flyoutButton.popupOpen = true
                MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation(flyoutButton)

                return __wow_mount_journal_dynamic_flight_popup_patched == true,
                    buttonAnim.calls[1],
                    popupAnim.calls[1],
                    buttonAnim.calls[2],
                    popupAnim.calls[2]
                "#,
            )
            .expect("patched mount journal state should be readable");

        assert!(patched);
        assert_eq!(button_closed, "true");
        assert_eq!(popup_closed, "false");
        assert_eq!(button_open, "false");
        assert_eq!(popup_open, "true");
    }

    #[test]
    fn tolerates_missing_popup() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            buttonAnim = {
                playing = nil,
                SetPlaying = function(self, playing)
                    self.playing = playing
                end,
            }
            MountJournalToggleDynamicFlightFlyoutButtonMixin = {
                UpdateUnspentGlyphsAnimation = function(self)
                    error("original should be replaced")
                end,
            }
            flyoutButton = {
                canSpendDragonridingGlyphs = true,
                UnspentGlyphsAnim = buttonAnim,
            }
            "#,
        )
        .expect("missing popup test surface should install");

        let loader_env = LoaderEnv::new(&env);
        patch(&loader_env);

        let button_playing: bool = env
            .eval(
                r#"
                MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation(flyoutButton)
                return buttonAnim.playing
                "#,
            )
            .expect("missing popup path should not error");

        assert!(button_playing);
    }
}
