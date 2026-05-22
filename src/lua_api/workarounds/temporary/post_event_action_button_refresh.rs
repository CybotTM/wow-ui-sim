//! Temporary post-event ActionButton visual refresh.
//!
//! Startup events can update action state before every button has refreshed its
//! art and hotkey text on the simulator path. Re-run the lightweight visual
//! refresh after startup events until the action-bar event flow matches WoW.

use crate::lua_api::WowLuaEnv;

const REFRESH_ACTION_BUTTONS_LUA: &str = r###"
local function __wow_refresh_action_button(button)
    if type(button) ~= "table" then
        return
    end
    if type(button.UpdateButtonArt) == "function" then
        pcall(button.UpdateButtonArt, button)
    end
    if type(button.UpdateHotkeys) == "function" then
        pcall(button.UpdateHotkeys, button, button.buttonType)
    end
end

for i = 1, 12 do
    __wow_refresh_action_button(_G["ActionButton" .. i])
end
"###;

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(REFRESH_ACTION_BUTTONS_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refreshes_action_button_art_and_hotkeys() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            for index = 1, 12 do
                _G["ActionButton" .. index] = {
                    buttonType = "ACTIONBUTTON" .. tostring(index),
                    artUpdates = 0,
                    hotkeyType = nil,
                    UpdateButtonArt = function(self)
                        self.artUpdates = self.artUpdates + 1
                    end,
                    UpdateHotkeys = function(self, buttonType)
                        self.hotkeyType = buttonType
                    end,
                }
            end
            "#,
        )
        .expect("action-button fixture should install");

        patch(&env);

        let (updated_count, first_hotkey, last_hotkey): (i64, String, String) = env
            .eval(
                r#"
                local updated = 0
                for index = 1, 12 do
                    local button = _G["ActionButton" .. index]
                    if button.artUpdates == 1 and button.hotkeyType == button.buttonType then
                        updated = updated + 1
                    end
                end
                return updated, ActionButton1.hotkeyType, ActionButton12.hotkeyType
                "#,
            )
            .expect("action-button refresh state should be readable");

        assert_eq!(updated_count, 12);
        assert_eq!(first_hotkey, "ACTIONBUTTON1");
        assert_eq!(last_hotkey, "ACTIONBUTTON12");
    }

    #[test]
    fn skips_missing_or_partial_action_buttons() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            ActionButton1 = nil
            ActionButton2 = {
                artUpdates = 0,
                UpdateButtonArt = function(self)
                    self.artUpdates = self.artUpdates + 1
                end,
            }
            "#,
        )
        .expect("partial action-button fixture should install");

        patch(&env);

        let art_updates: i64 = env
            .eval("return ActionButton2.artUpdates")
            .expect("partial action-button refresh state should be readable");

        assert_eq!(art_updates, 1);
    }
}
