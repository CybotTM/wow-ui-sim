//! Temporary Auction House browse-results event registration repair.
//!
//! The browse results surface dispatches updates, but some Blizzard paths still
//! need the frame/mixin event registration patched during startup.

use crate::lua_api::{LoaderEnv, WowLuaEnv};

const AUCTION_HOUSE_BROWSE_RESULTS_EVENT_WORKAROUND_LUA: &str = r#"
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

pub(crate) fn patch(env: &LoaderEnv<'_>) {
    let _ = env.exec(AUCTION_HOUSE_BROWSE_RESULTS_EVENT_WORKAROUND_LUA);
}

pub(crate) fn patch_env(env: &WowLuaEnv) {
    let _ = env.exec(AUCTION_HOUSE_BROWSE_RESULTS_EVENT_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn install_auction_house_fixture(env: &WowLuaEnv) {
        env.exec(
            r#"
            original_show_calls = 0
            original_hide_calls = 0
            frame_show_script_calls = 0
            frame_hide_script_calls = 0
            AuctionHouseFrameMixin = {
                OnShow = function(self)
                    original_show_calls = original_show_calls + 1
                end,
                OnHide = function(self)
                    original_hide_calls = original_hide_calls + 1
                end,
            }
            AuctionHouseFrame = {
                events = {},
                scripts = {
                    OnShow = function()
                        frame_show_script_calls = frame_show_script_calls + 1
                    end,
                    OnHide = function()
                        frame_hide_script_calls = frame_hide_script_calls + 1
                    end,
                },
                RegisterEvent = function(self, event)
                    self.events[event] = true
                end,
                UnregisterEvent = function(self, event)
                    self.events[event] = false
                end,
                GetScript = function(self, name)
                    return self.scripts[name]
                end,
                SetScript = function(self, name, script)
                    self.scripts[name] = script
                end,
            }
            "#,
        )
        .expect("auction house fixture should install");
    }

    #[test]
    fn registers_browse_results_event_on_existing_frame() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_auction_house_fixture(&env);

        patch_env(&env);

        let registered: bool = env
            .eval(r#"return AuctionHouseFrame.events["AUCTION_HOUSE_BROWSE_RESULTS_UPDATED"]"#)
            .expect("auction house event registration should be readable");

        assert!(registered);
    }

    #[test]
    fn wraps_frame_show_and_hide_scripts() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_auction_house_fixture(&env);
        patch_env(&env);

        let (registered_after_hide, registered_after_show, show_calls, hide_calls): (
            bool,
            bool,
            i64,
            i64,
        ) = env
            .eval(
                r#"
                AuctionHouseFrame.scripts.OnHide(AuctionHouseFrame)
                local registeredAfterHide =
                    AuctionHouseFrame.events["AUCTION_HOUSE_BROWSE_RESULTS_UPDATED"]
                AuctionHouseFrame.scripts.OnShow(AuctionHouseFrame)
                local registeredAfterShow =
                    AuctionHouseFrame.events["AUCTION_HOUSE_BROWSE_RESULTS_UPDATED"]
                return registeredAfterHide,
                    registeredAfterShow,
                    frame_show_script_calls,
                    frame_hide_script_calls
                "#,
            )
            .expect("wrapped auction house frame scripts should run");

        assert!(!registered_after_hide);
        assert!(registered_after_show);
        assert_eq!(show_calls, 1);
        assert_eq!(hide_calls, 1);
    }

    #[test]
    fn wraps_mixin_show_and_hide_methods() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_auction_house_fixture(&env);
        patch_env(&env);

        let (registered_after_hide, registered_after_show, show_calls, hide_calls): (
            bool,
            bool,
            i64,
            i64,
        ) = env
            .eval(
                r#"
                AuctionHouseFrameMixin.OnHide(AuctionHouseFrame)
                local registeredAfterHide =
                    AuctionHouseFrame.events["AUCTION_HOUSE_BROWSE_RESULTS_UPDATED"]
                AuctionHouseFrameMixin.OnShow(AuctionHouseFrame)
                local registeredAfterShow =
                    AuctionHouseFrame.events["AUCTION_HOUSE_BROWSE_RESULTS_UPDATED"]
                return registeredAfterHide,
                    registeredAfterShow,
                    original_show_calls,
                    original_hide_calls
                "#,
            )
            .expect("wrapped auction house mixin methods should run");

        assert!(!registered_after_hide);
        assert!(registered_after_show);
        assert_eq!(show_calls, 1);
        assert_eq!(hide_calls, 1);
    }
}
