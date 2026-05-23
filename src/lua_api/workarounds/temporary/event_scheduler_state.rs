//! Temporary `C_EventScheduler` state surface.
//!
//! Event scheduler data is currently a small simulator seed used by Quest Log
//! and event-tab startup paths. Keep it explicit as temporary compatibility
//! state rather than hiding it in the generic runtime bootstrap.

const EVENT_SCHEDULER_STATE_LUA: &str = r#"
if type(C_EventScheduler) ~= "table" then
    C_EventScheduler = {}
end

local function EventSchedulerNamespaceFallback(t, key)
    if type(__wow_record_nil_symbol_access) == "function" then
        __wow_record_nil_symbol_access("C_EventScheduler", key, nil, nil)
    end
    local fn = function()
        return nil
    end
    rawset(t, key, fn)
    return fn
end

local mt = getmetatable(C_EventScheduler)
if mt == nil then
    setmetatable(C_EventScheduler, { __index = EventSchedulerNamespaceFallback })
elseif mt.__index == nil then
    mt.__index = EventSchedulerNamespaceFallback
end

local function EventSchedulerSeedState()
    local now = (os and type(os.time) == "function") and os.time() or 0
    return {
        canShowEvents = nil,
        suppressDisplay = false,
        ongoingEvents = {
            {
                areaPoiID = 1001,
                eventID = 1001,
                eventKey = "warsong-gulch",
                displayInfo = {},
                rewardsClaimed = false,
            },
            {
                areaPoiID = 1002,
                eventID = 1002,
                eventKey = "cinderbrew-meadery",
                displayInfo = {},
                rewardsClaimed = false,
            },
        },
        scheduledEvents = {
            {
                areaPoiID = 1001,
                eventID = 2001,
                eventKey = "pvp-brawl-blitz",
                startTime = now + 3600,
                endTime = now + 7200,
                duration = 3600,
                hasReminder = false,
                rewardsClaimed = false,
                displayInfo = {},
            },
            {
                areaPoiID = 1004,
                eventID = 2002,
                eventKey = "darkmoon-island",
                startTime = now + 7200,
                endTime = now + 10800,
                duration = 3600,
                hasReminder = true,
                rewardsClaimed = false,
                displayInfo = {},
            },
        },
        reminders = {},
    }
end

if type(rawget(C_EventScheduler, "_state")) ~= "table" then
    C_EventScheduler._state = EventSchedulerSeedState()
end

if rawget(C_EventScheduler, "CanShowEvents") == nil then
    function C_EventScheduler.CanShowEvents()
        local state = C_EventScheduler._state
        if type(state) ~= "table" then
            return false
        end
        if state.canShowEvents ~= nil then
            return state.canShowEvents == true
        end
        if state.suppressDisplay == true then
            return false
        end
        return #(state.ongoingEvents or {}) > 0 or #(state.scheduledEvents or {}) > 0
    end
end

if rawget(C_EventScheduler, "RequestEvents") == nil then
    function C_EventScheduler.RequestEvents()
        C_EventScheduler._state = EventSchedulerSeedState()
    end
end

if rawget(C_EventScheduler, "GetOngoingEvents") == nil then
    function C_EventScheduler.GetOngoingEvents()
        return C_EventScheduler._state.ongoingEvents
    end
end

if rawget(C_EventScheduler, "GetScheduledEvents") == nil then
    function C_EventScheduler.GetScheduledEvents()
        return C_EventScheduler._state.scheduledEvents
    end
end

if rawget(C_EventScheduler, "HasData") == nil then
    function C_EventScheduler.HasData()
        local state = C_EventScheduler._state
        return #(state.ongoingEvents or {}) > 0 or #(state.scheduledEvents or {}) > 0
    end
end

if rawget(C_EventScheduler, "GetEventZoneName") == nil then
    function C_EventScheduler.GetEventZoneName(areaPoiID)
        local poi = C_AreaPoiInfo.GetAreaPOIInfo(nil, areaPoiID)
        return poi and poi.name or ""
    end
end

if rawget(C_EventScheduler, "GetEventUiMapID") == nil then
    function C_EventScheduler.GetEventUiMapID(areaPoiID)
        local poi = C_AreaPoiInfo.GetAreaPOIInfo(nil, areaPoiID)
        return (poi and poi.uiMapID) or 0
    end
end

if rawget(C_EventScheduler, "HasSavedReminders") == nil then
    function C_EventScheduler.HasSavedReminders()
        local reminders = C_EventScheduler._state.reminders or {}
        return next(reminders) ~= nil
    end
end

if rawget(C_EventScheduler, "SetReminder") == nil then
    function C_EventScheduler.SetReminder(eventKey)
        if eventKey ~= nil then
            C_EventScheduler._state.reminders[tostring(eventKey)] = true
        end
    end
end

if rawget(C_EventScheduler, "ClearReminder") == nil then
    function C_EventScheduler.ClearReminder(eventKey)
        if eventKey ~= nil then
            C_EventScheduler._state.reminders[tostring(eventKey)] = nil
        end
    end
end

if rawget(C_EventScheduler, "GetActiveContinentName") == nil then
    function C_EventScheduler.GetActiveContinentName()
        return nil
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(EVENT_SCHEDULER_STATE_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_seeded_events_reminders_and_namespace_fallback() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if #C_EventScheduler.GetOngoingEvents() ~= 2 then
                    return "bad_ongoing"
                end
                if #C_EventScheduler.GetScheduledEvents() ~= 2 then
                    return "bad_scheduled"
                end
                if not C_EventScheduler.CanShowEvents() then
                    return "not_visible"
                end
                C_EventScheduler.SetReminder("warsong-gulch")
                if not C_EventScheduler.HasSavedReminders() then
                    return "missing_reminder"
                end
                C_EventScheduler.ClearReminder("warsong-gulch")
                if C_EventScheduler.HasSavedReminders() then
                    return "stale_reminder"
                end
                if C_EventScheduler.GetEventZoneName(1001) ~= "Warsong Gulch" then
                    return "bad_zone_name"
                end
                if C_EventScheduler.GetEventUiMapID(1004) == 0 then
                    return "bad_map_id"
                end
                if type(C_EventScheduler.SomeUnimplementedMember) ~= "function" then
                    return "missing_fallback"
                end
                if C_EventScheduler.SomeUnimplementedMember() ~= nil then
                    return "fallback_returned_value"
                end
                return "ok"
                "#,
            )
            .expect("event scheduler probe should run");

        assert_eq!(result, "ok");
    }
}
