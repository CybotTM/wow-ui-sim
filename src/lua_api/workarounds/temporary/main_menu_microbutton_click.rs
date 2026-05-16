//! Temporary main-menu microbutton click workaround.
//!
//! The real UI wires the main menu button through the loaded micro-button
//! mixin and panel manager state. Keep this click bridge isolated until the
//! simulator models that full startup path.

use crate::lua_api::WowLuaEnv;

const MAIN_MENU_MICROBUTTON_CLICK_WORKAROUND_LUA: &str = r#"
local function __wow_show_game_menu(frame)
    if type(ShowUIPanel) == "function" then
        ShowUIPanel(frame)
    end
    if type(frame.IsShown) == "function" and not frame:IsShown() and type(frame.Show) == "function" then
        frame:Show()
    end
end

local function __wow_hide_game_menu(frame)
    if type(HideUIPanel) == "function" then
        HideUIPanel(frame)
    end
    if type(frame.IsShown) == "function" and frame:IsShown() and type(frame.Hide) == "function" then
        frame:Hide()
    end
end

local function __wow_toggle_main_menu()
    local gameMenuFrame = rawget(_G, "GameMenuFrame")
    if not gameMenuFrame then
        return
    end
    if type(AreAllPanelsDisallowed) == "function" and AreAllPanelsDisallowed() then
        return
    end
    if gameMenuFrame:IsShown() then
        if type(PlaySound) == "function" and SOUNDKIT and SOUNDKIT.IG_MAINMENU_QUIT then
            PlaySound(SOUNDKIT.IG_MAINMENU_QUIT)
        end
        __wow_hide_game_menu(gameMenuFrame)
    else
        if type(SettingsPanel) == "table" and type(SettingsPanel.IsShown) == "function" and SettingsPanel:IsShown() and type(SettingsPanel.Close) == "function" then
            SettingsPanel:Close()
        end
        if type(CloseMenus) == "function" then
            CloseMenus()
        end
        if type(CloseAllWindows) == "function" then
            CloseAllWindows()
        end
        if type(PlaySound) == "function" and SOUNDKIT and SOUNDKIT.IG_MAINMENU_OPEN then
            PlaySound(SOUNDKIT.IG_MAINMENU_OPEN)
        end
        __wow_show_game_menu(gameMenuFrame)
    end
end

local function __wow_wrap_main_menu_mouse_scripts(button)
    if type(button) ~= "table" or button.__wow_uisim_mouse_scripts_wrapped or type(button.GetScript) ~= "function" or type(button.SetScript) ~= "function" then
        return
    end
    button.__wow_uisim_mouse_scripts_wrapped = true

    local onMouseDown = button:GetScript("OnMouseDown")
    if type(onMouseDown) == "function" then
        button:SetScript("OnMouseDown", function(self, ...)
            if self then
                self.__wow_uisim_onclick_seen_this_press = nil
                self.__wow_uisim_suppress_next_click_toggle = nil
            end
            return onMouseDown(self, ...)
        end)
    end

    local onMouseUp = button:GetScript("OnMouseUp")
    if type(onMouseUp) == "function" then
        button:SetScript("OnMouseUp", function(self, ...)
            local gameMenuFrame = rawget(_G, "GameMenuFrame")
            local wasShown = gameMenuFrame and gameMenuFrame:IsShown()
            local clickAlreadyRan = self and self.__wow_uisim_onclick_seen_this_press
            local result = onMouseUp(self, ...)
            if self then
                self.__wow_uisim_onclick_seen_this_press = nil
                local isShown = gameMenuFrame and gameMenuFrame:IsShown()
                if wasShown ~= isShown and not clickAlreadyRan then
                    self.__wow_uisim_suppress_next_click_toggle = true
                end
            end
            return result
        end)
    end
end

if type(MainMenuMicroButtonMixin) == "table" and not MainMenuMicroButtonMixin.__wow_uisim_click_patched then
    MainMenuMicroButtonMixin.__wow_uisim_click_patched = true
    MainMenuMicroButtonMixin.OnClick = function(self, button, down)
        if self and self.__wow_uisim_suppress_next_click_toggle then
            self.__wow_uisim_suppress_next_click_toggle = nil
            return
        end
        if self and self.down then
            self.__wow_uisim_onclick_seen_this_press = true
            return
        end
        if self then
            self.__wow_uisim_onclick_seen_this_press = true
        end
        local result = __wow_toggle_main_menu()
        if self then
            self.__wow_uisim_onclick_seen_this_press = nil
        end
        return result
    end
end

if type(MainMenuMicroButton) == "table" and type(MainMenuMicroButton.SetScript) == "function" then
    __wow_wrap_main_menu_mouse_scripts(MainMenuMicroButton)
    MainMenuMicroButton:SetScript("OnClick", function(self, button, down)
        if self and self.__wow_uisim_suppress_next_click_toggle then
            self.__wow_uisim_suppress_next_click_toggle = nil
            return
        end
        if self and self.down then
            self.__wow_uisim_onclick_seen_this_press = true
            return
        end
        if self then
            self.__wow_uisim_onclick_seen_this_press = true
        end
        local result = __wow_toggle_main_menu()
        if self then
            self.__wow_uisim_onclick_seen_this_press = nil
        end
        return result
    end)
end
"#;

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(MAIN_MENU_MICROBUTTON_CLICK_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn microbutton_click_toggles_game_menu() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            calls = {}
            SOUNDKIT = {
                IG_MAINMENU_OPEN = "open",
                IG_MAINMENU_QUIT = "quit",
            }
            GameMenuFrame = {
                shown = false,
                IsShown = function(self)
                    return self.shown
                end,
                Show = function(self)
                    self.shown = true
                    table.insert(calls, "show")
                end,
                Hide = function(self)
                    self.shown = false
                    table.insert(calls, "hide")
                end,
            }
            SettingsPanel = {
                shown = true,
                IsShown = function(self)
                    return self.shown
                end,
                Close = function(self)
                    self.shown = false
                    table.insert(calls, "settings-close")
                end,
            }
            MainMenuMicroButtonMixin = {}
            MainMenuMicroButton = {
                scripts = {},
                SetScript = function(self, event, script)
                    self.scripts[event] = script
                end,
            }
            function CloseMenus()
                table.insert(calls, "close-menus")
            end
            function CloseAllWindows()
                table.insert(calls, "close-all")
            end
            function ShowUIPanel(frame)
                table.insert(calls, "show-ui")
            end
            function HideUIPanel(frame)
                table.insert(calls, "hide-ui")
            end
            function PlaySound(sound)
                table.insert(calls, "sound:" .. tostring(sound))
            end
            "#,
        )
        .expect("main-menu test surface should install");

        patch(&env);

        let (patched, script_installed, opened, closed, calls): (bool, bool, bool, bool, String) =
            env.eval(
                r#"
                MainMenuMicroButton.scripts.OnClick(MainMenuMicroButton)
                local opened = GameMenuFrame:IsShown()
                MainMenuMicroButtonMixin.OnClick(MainMenuMicroButton)
                local closed = not GameMenuFrame:IsShown()

                return MainMenuMicroButtonMixin.__wow_uisim_click_patched == true,
                    type(MainMenuMicroButton.scripts.OnClick) == "function",
                    opened,
                    closed,
                    table.concat(calls, ",")
                "#,
            )
            .expect("patched main-menu state should be readable");

        assert!(patched);
        assert!(script_installed);
        assert!(opened);
        assert!(closed);
        assert_eq!(
            calls,
            "settings-close,close-menus,close-all,sound:open,show-ui,show,sound:quit,hide-ui,hide"
        );
    }

    #[test]
    fn panel_disallow_prevents_toggle() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            GameMenuFrame = {
                shown = false,
                IsShown = function(self)
                    return self.shown
                end,
                Show = function(self)
                    self.shown = true
                end,
            }
            MainMenuMicroButton = {
                scripts = {},
                SetScript = function(self, event, script)
                    self.scripts[event] = script
                end,
            }
            function AreAllPanelsDisallowed()
                return true
            end
            "#,
        )
        .expect("disallowed panel test surface should install");

        patch(&env);

        let shown: bool = env
            .eval(
                r#"
                MainMenuMicroButton.scripts.OnClick(MainMenuMicroButton)
                return GameMenuFrame:IsShown()
                "#,
            )
            .expect("disallowed panel state should be readable");

        assert!(!shown);
    }
}
