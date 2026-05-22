//! Temporary action-bar button event fanout workaround.
//!
//! The real action-bar event frame fans events out to registered button frames.
//! Keep this isolated until the simulator models that ActionBarButtonEventsFrame
//! setup path directly.

use crate::lua_api::WowLuaEnv;

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

ActionBarButtonEventsFrameMixin.OnEvent = on_event
ActionBarButtonEventsFrameMixin.OnCountdownForCooldownsChanged = on_countdown_for_cooldowns_changed
ActionBarButtonEventsFrameMixin.ForEachFrame = for_each_frame

if type(ActionBarButtonEventsFrame) == "table" then
    ActionBarButtonEventsFrame.OnEvent = on_event
    ActionBarButtonEventsFrame.OnCountdownForCooldownsChanged = on_countdown_for_cooldowns_changed
    ActionBarButtonEventsFrame.ForEachFrame = for_each_frame
    if type(ActionBarButtonEventsFrame.SetScript) == "function" then
        ActionBarButtonEventsFrame:SetScript("OnEvent", on_event)
    end
end
"##;

pub(crate) fn patch(env: &WowLuaEnv) {
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

    fn install_action_bar_event_surface(env: &WowLuaEnv) {
        env.exec(ACTION_BAR_EVENT_SURFACE_LUA)
            .expect("action-bar event test surface should install");
    }
}
