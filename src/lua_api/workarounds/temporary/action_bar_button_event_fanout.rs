//! Temporary action-bar button event fanout workaround.
//!
//! The real action-bar event frame fans events out to registered button frames.
//! Keep this isolated until the simulator models that ActionBarButtonEventsFrame
//! setup path directly.

use crate::lua_api::{LoaderEnv, WowLuaEnv};

const ACTION_BAR_BUTTON_EVENT_FANOUT_WORKAROUND_LUA: &str = r##"
if type(ActionBarButtonEventsFrameMixin) ~= "table" then
    return
end

local traceFanout = {trace_fanout}

local function button_label(frame, index)
    if type(frame) ~= "table" then
        return "#" .. tostring(index)
    end
    if type(frame.GetName) == "function" then
        local name = frame:GetName()
        if name ~= nil then
            return name
        end
    end
    if frame.action ~= nil then
        return "action:" .. tostring(frame.action)
    end
    return "#" .. tostring(index)
end

local function for_each_button_frame(self, func)
    local frames = self.frames
    if type(frames) ~= "table" then
        return
    end
    for i = 1, #frames do
        local frame = rawget(frames, i)
        if frame ~= nil then
            if traceFanout then
                print("[ActionBarFanout] begin " .. button_label(frame, i))
            end
            func(frame)
            if traceFanout then
                print("[ActionBarFanout] end " .. button_label(frame, i))
            end
        end
    end
end

local function on_event(self, event, ...)
    local args = { ... }
    for_each_button_frame(self, function(frame)
        frame:OnEvent(event, unpack(args))
    end)
    if event == "ACTIONBAR_SLOT_CHANGED" or event == "ACTIONBAR_UPDATE_STATE" then
        for_each_button_frame(self, function(frame)
            if type(frame.UpdateButtonArt) == "function" then
                pcall(frame.UpdateButtonArt, frame)
            end
        end)
    end
end

local function on_countdown_for_cooldowns_changed(self)
    for_each_button_frame(self, function(frame)
        ActionButton_UpdateCooldownNumberHidden(frame)
    end)
end

local function for_each_frame(self, func)
    for_each_button_frame(self, func)
end

local function ensure_action_bar_onload(frame)
    if type(frame) ~= "table" or type(frame.actionButtons) == "table" then
        return
    end
    if type(frame.GetScript) ~= "function" then
        return
    end

    local onLoad = frame:GetScript("OnLoad")
    if type(onLoad) == "function" then
        onLoad(frame)
    end
end

ActionBarButtonEventsFrameMixin.OnEvent = on_event
ActionBarButtonEventsFrameMixin.OnCountdownForCooldownsChanged = on_countdown_for_cooldowns_changed
ActionBarButtonEventsFrameMixin.ForEachFrame = for_each_frame

if type(ActionBarButtonEventsFrame) == "table" then
    if type(ActionBarButtonEventsFrame.frames) ~= "table" then
        ActionBarButtonEventsFrame.frames = {}
    end
    ActionBarButtonEventsFrame.OnEvent = on_event
    ActionBarButtonEventsFrame.OnCountdownForCooldownsChanged = on_countdown_for_cooldowns_changed
    ActionBarButtonEventsFrame.ForEachFrame = for_each_frame
    if type(ActionBarButtonEventsFrame.SetScript) == "function" then
        ActionBarButtonEventsFrame:SetScript("OnEvent", on_event)
    end
end

ensure_action_bar_onload(StanceBar)
"##;

pub(crate) fn patch(env: &WowLuaEnv) {
    let trace_fanout = std::env::var_os("WOW_SIM_TRACE_ACTIONBAR_BUTTON_FANOUT").is_some();
    let script = script(trace_fanout);
    let _ = env.exec(&script);
}

pub(crate) fn patch_loader(env: &LoaderEnv<'_>) {
    let trace_fanout = std::env::var_os("WOW_SIM_TRACE_ACTIONBAR_BUTTON_FANOUT").is_some();
    let script = script(trace_fanout);
    let _ = env.exec(&script);
}

fn script(trace_fanout: bool) -> String {
    let trace_fanout = if trace_fanout { "true" } else { "false" };
    ACTION_BAR_BUTTON_EVENT_FANOUT_WORKAROUND_LUA.replace("{trace_fanout}", trace_fanout)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTION_BAR_EVENT_SURFACE_LUA: &str = r#"
        events = {}
        artUpdates = {}
        cooldowns = {}
        ActionBarButtonEventsFrameMixin = {}
        ActionBarButtonEventsFrame = {
            scripts = {},
            frames = {
                {
                    label = "button1",
                    OnEvent = function(self, event, value)
                        table.insert(events, self.label .. ":" .. event .. ":" .. tostring(value))
                    end,
                    UpdateButtonArt = function(self)
                        table.insert(artUpdates, self.label)
                    end,
                },
                {
                    label = "button2",
                    OnEvent = function(self, event, value)
                        table.insert(events, self.label .. ":" .. event .. ":" .. tostring(value))
                    end,
                    UpdateButtonArt = function(self)
                        table.insert(artUpdates, self.label)
                    end,
                },
            },
            SetScript = function(self, event, handler)
                self.scripts[event] = handler
            end,
        }
        ActionButton_UpdateCooldownNumberHidden = function(frame)
            table.insert(cooldowns, frame.label)
        end
    "#;

    #[test]
    fn fans_events_out_to_button_frames_and_updates_art_for_actionbar_events() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_action_bar_event_surface(&env);

        env.exec(&script(false))
            .expect("action-bar fanout patch should install");

        let (event_count, art_count, first_event, second_event): (i64, i64, String, String) = env
            .eval(
                r#"
                ActionBarButtonEventsFrame:OnEvent("ACTIONBAR_SLOT_CHANGED", 7)
                return #events, #artUpdates, events[1], events[2]
                "#,
            )
            .expect("action-bar event fanout should be readable");

        assert_eq!(event_count, 2);
        assert_eq!(art_count, 2);
        assert_eq!(first_event, "button1:ACTIONBAR_SLOT_CHANGED:7");
        assert_eq!(second_event, "button2:ACTIONBAR_SLOT_CHANGED:7");
    }

    #[test]
    fn countdown_change_fans_out_cooldown_number_updates() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_action_bar_event_surface(&env);

        env.exec(&script(false))
            .expect("action-bar fanout patch should install");

        let cooldown_count: i64 = env
            .eval(
                r#"
                ActionBarButtonEventsFrame:OnCountdownForCooldownsChanged()
                return #cooldowns
                "#,
            )
            .expect("cooldown fanout should be readable");

        assert_eq!(cooldown_count, 2);
    }

    #[test]
    fn installs_on_event_script_when_frame_supports_set_script() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_action_bar_event_surface(&env);

        env.exec(&script(false))
            .expect("action-bar fanout patch should install");

        let script_installed: bool = env
            .eval("return ActionBarButtonEventsFrame.scripts.OnEvent ~= nil")
            .expect("installed action-bar event script should be readable");

        assert!(script_installed);
    }

    #[test]
    fn initializes_missing_frame_registry_on_existing_event_frame() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            ActionBarButtonEventsFrameMixin = {}
            ActionBarButtonEventsFrame = {}
            "#,
        )
        .expect("action-bar event frame fixture should install");

        env.exec(&script(false))
            .expect("action-bar fanout patch should install");

        let frames_type: String = env
            .eval("return type(ActionBarButtonEventsFrame.frames)")
            .expect("frame registry type should be readable");

        assert_eq!(frames_type, "table");
    }

    #[test]
    fn runs_stance_bar_onload_when_action_buttons_are_missing() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            ActionBarButtonEventsFrameMixin = {}
            StanceBar = {
                loaded = 0,
                GetScript = function(self, name)
                    if name == "OnLoad" then
                        return function(frame)
                            frame.loaded = frame.loaded + 1
                            frame.actionButtons = {}
                        end
                    end
                end,
            }
            "#,
        )
        .expect("stance bar fixture should install");

        env.exec(&script(false))
            .expect("action-bar fanout patch should install");

        let (loaded, action_buttons_type): (i32, String) = env
            .eval("return StanceBar.loaded, type(StanceBar.actionButtons)")
            .expect("stance bar load state should be readable");

        assert_eq!(loaded, 1);
        assert_eq!(action_buttons_type, "table");
    }

    #[test]
    fn does_not_rerun_stance_bar_onload_when_action_buttons_exist() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            ActionBarButtonEventsFrameMixin = {}
            StanceBar = {
                loaded = 0,
                actionButtons = {},
                GetScript = function(self, name)
                    if name == "OnLoad" then
                        return function(frame)
                            frame.loaded = frame.loaded + 1
                        end
                    end
                end,
            }
            "#,
        )
        .expect("stance bar fixture should install");

        env.exec(&script(false))
            .expect("action-bar fanout patch should install");

        let loaded: i32 = env
            .eval("return StanceBar.loaded")
            .expect("stance bar load count should be readable");

        assert_eq!(loaded, 0);
    }

    fn install_action_bar_event_surface(env: &WowLuaEnv) {
        env.exec(ACTION_BAR_EVENT_SURFACE_LUA)
            .expect("action-bar event test surface should install");
    }
}
