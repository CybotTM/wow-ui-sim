//! Temporary `C_CombatLog` / `C_CombatLogSecure` state surface.
//!
//! Combat log history is not modeled yet. This keeps the small shared Lua state
//! fixture explicit rather than presenting it as complete C API behavior.

const COMBAT_LOG_STATE_LUA: &str = r#"
CombatLogInbound = CombatLogInbound or {
    GenerateMessage = function()
        return "", 1, 1, 1
    end,
}

if type(C_CombatLog) ~= "table" then
    C_CombatLog = {}
end
if type(C_CombatLogSecure) ~= "table" then
    C_CombatLogSecure = {}
end

local state = rawget(_G, "__wow_combat_log_state")
if type(state) ~= "table" then
    state = {
        currentEntry = 0,
        numEntries = 0,
        retentionTime = 300,
        filteredEventsEnabled = false,
        messageLimit = 300,
        entries = {},
        currentIndex = nil,
        createdMessages = {},
    }
    rawset(_G, "__wow_combat_log_state", state)
end

local function CombatLogState()
    if type(state.entries) ~= "table" then
        state.entries = {}
    end
    if type(state.createdMessages) ~= "table" then
        state.createdMessages = {}
    end
    return state
end

local function CountEntries(currentState)
    local entries = currentState.entries
    if type(entries) ~= "table" then
        return tonumber(currentState.numEntries) or 0
    end
    return #entries
end

local function CurrentEntry(currentState)
    local entries = currentState.entries
    if type(entries) ~= "table" or #entries == 0 then
        return nil
    end
    local index = currentState.currentIndex
    if type(index) ~= "number" or index < 1 or index > #entries then
        index = #entries
    end
    return entries[index], index
end

local function ClearEntries(currentState)
    currentState.entries = {}
    currentState.currentIndex = nil
    currentState.currentEntry = 0
    currentState.numEntries = 0
end

local function StoreMessage(currentState, message, red, green, blue, order)
    local entry = {
        message = tostring(message or ""),
        red = tonumber(red) or 0,
        green = tonumber(green) or 0,
        blue = tonumber(blue) or 0,
        order = order,
    }
    local newest = Enum.CombatLogMessageOrder and Enum.CombatLogMessageOrder.Newest
    if order == newest then
        table.insert(currentState.createdMessages, 1, entry)
    else
        table.insert(currentState.createdMessages, entry)
    end
end

local function ObjectMatchesFilter(objectType, mask)
    local object = math.max(0, tonumber(objectType) or 0)
    local filter = math.max(0, tonumber(mask) or 0)
    while object > 0 and filter > 0 do
        if object % 2 == 1 and filter % 2 == 1 then
            return true
        end
        object = math.floor(object / 2)
        filter = math.floor(filter / 2)
    end
    return false
end

C_CombatLog._state = CombatLogState()
C_CombatLogSecure._state = C_CombatLog._state

if rawget(C_CombatLog, "AddEventFilter") == nil then
    function C_CombatLog.AddEventFilter(_filter)
        return true
    end
end

if rawget(C_CombatLog, "ClearEventFilters") == nil then
    function C_CombatLog.ClearEventFilters()
        CombatLogState().filteredEventsEnabled = false
        return true
    end
end

if rawget(C_CombatLog, "DoesObjectMatchFilter") == nil then
    function C_CombatLog.DoesObjectMatchFilter(objectType, mask)
        return ObjectMatchesFilter(objectType, mask)
    end
end

if rawget(C_CombatLog, "GetEntryCount") == nil then
    function C_CombatLog.GetEntryCount()
        return CountEntries(CombatLogState())
    end
end

if rawget(C_CombatLog, "GetCurrentEntryInfo") == nil then
    function C_CombatLog.GetCurrentEntryInfo()
        return CombatLogState().currentEntry
    end
end

if rawget(C_CombatLog, "GetCurrentEventInfo") == nil then
    function C_CombatLog.GetCurrentEventInfo()
        local entry = CurrentEntry(CombatLogState())
        if entry == nil then
            return nil
        end
        return unpack(entry)
    end
end

if rawget(C_CombatLog, "ShouldShowCurrentEntry") == nil then
    function C_CombatLog.ShouldShowCurrentEntry()
        return CountEntries(CombatLogState()) > 0
    end
end

if rawget(C_CombatLog, "GetEntryRetentionTime") == nil then
    function C_CombatLog.GetEntryRetentionTime()
        return CombatLogState().retentionTime
    end
end

if rawget(C_CombatLog, "SetEntryRetentionTime") == nil then
    function C_CombatLog.SetEntryRetentionTime(retentionTime)
        CombatLogState().retentionTime = tonumber(retentionTime) or 0
    end
end

if rawget(C_CombatLog, "AreFilteredEventsEnabled") == nil then
    function C_CombatLog.AreFilteredEventsEnabled()
        return CombatLogState().filteredEventsEnabled == true
    end
end

if rawget(C_CombatLog, "SetFilteredEventsEnabled") == nil then
    function C_CombatLog.SetFilteredEventsEnabled(enabled)
        CombatLogState().filteredEventsEnabled = enabled == true
    end
end

if rawget(C_CombatLog, "GetMessageLimit") == nil then
    function C_CombatLog.GetMessageLimit()
        return CombatLogState().messageLimit or 300
    end
end

if rawget(C_CombatLog, "SetMessageLimit") == nil then
    function C_CombatLog.SetMessageLimit(limit)
        CombatLogState().messageLimit = tonumber(limit) or 0
    end
end

if rawget(C_CombatLog, "ClearEntries") == nil then
    function C_CombatLog.ClearEntries()
        ClearEntries(CombatLogState())
    end
end

if rawget(C_CombatLog, "ApplyFilterSettings") == nil then
    function C_CombatLog.ApplyFilterSettings(_settings)
    end
end

if rawget(C_CombatLog, "RefilterEntries") == nil then
    function C_CombatLog.RefilterEntries()
    end
end

if rawget(C_CombatLogSecure, "GetEntryCount") == nil then
    function C_CombatLogSecure.GetEntryCount()
        return CountEntries(CombatLogState())
    end
end

if rawget(C_CombatLogSecure, "GetCurrentEntryInfo") == nil then
    function C_CombatLogSecure.GetCurrentEntryInfo()
        local entry = CurrentEntry(CombatLogState())
        if entry == nil then
            return nil
        end
        return unpack(entry)
    end
end

if rawget(C_CombatLogSecure, "SeekToNewestEntry") == nil then
    function C_CombatLogSecure.SeekToNewestEntry()
        local currentState = CombatLogState()
        local count = CountEntries(currentState)
        if count == 0 then
            return false
        end
        currentState.currentIndex = count
        return true
    end
end

if rawget(C_CombatLogSecure, "SeekToPreviousEntry") == nil then
    function C_CombatLogSecure.SeekToPreviousEntry()
        local currentState = CombatLogState()
        local count = CountEntries(currentState)
        if count == 0 then
            return false
        end
        local index = currentState.currentIndex or count
        if index <= 1 then
            return false
        end
        currentState.currentIndex = index - 1
        return true
    end
end

if rawget(C_CombatLogSecure, "CreateCombatLogMessage") == nil then
    function C_CombatLogSecure.CreateCombatLogMessage(message, red, green, blue, order)
        StoreMessage(CombatLogState(), message, red, green, blue, order)
        return true
    end
end

if rawget(_G, "CombatLog_Object_IsA") == nil then
    CombatLog_Object_IsA = C_CombatLog.DoesObjectMatchFilter
end

if rawget(_G, "CombatLogAddFilter") == nil then
    CombatLogAddFilter = C_CombatLog.AddEventFilter
end

if rawget(_G, "CombatLogResetFilter") == nil then
    CombatLogResetFilter = C_CombatLog.ClearEventFilters
end

if rawget(_G, "CombatLogClearEntries") == nil then
    CombatLogClearEntries = C_CombatLog.ClearEntries
end

if rawget(_G, "CombatLogGetCurrentEntry") == nil then
    CombatLogGetCurrentEntry = C_CombatLog.GetCurrentEntryInfo
end

if rawget(_G, "CombatLogGetCurrentEventInfo") == nil then
    CombatLogGetCurrentEventInfo = C_CombatLog.GetCurrentEventInfo
end

if rawget(_G, "CombatLogGetNumEntries") == nil then
    CombatLogGetNumEntries = C_CombatLog.GetEntryCount
end

if rawget(_G, "CombatLogGetRetentionTime") == nil then
    CombatLogGetRetentionTime = C_CombatLog.GetEntryRetentionTime
end

if rawget(_G, "CombatLogSetRetentionTime") == nil then
    CombatLogSetRetentionTime = C_CombatLog.SetEntryRetentionTime
end

if rawget(_G, "CombatLogShowCurrentEntry") == nil then
    CombatLogShowCurrentEntry = C_CombatLog.ShouldShowCurrentEntry
end

if rawget(_G, "CombatLogAdvanceEntry") == nil then
    function CombatLogAdvanceEntry(step)
        local currentState = CombatLogState()
        local amount = tonumber(step) or 0
        currentState.currentEntry = math.max(0, currentState.currentEntry + amount)
        return true
    end
end

if rawget(_G, "CombatLogSetCurrentEntry") == nil then
    function CombatLogSetCurrentEntry(entry)
        CombatLogState().currentEntry = math.max(0, tonumber(entry) or 0)
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(COMBAT_LOG_STATE_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_shared_combat_log_state_and_navigation() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if C_CombatLog.GetEntryCount() ~= 0 or C_CombatLogSecure.GetEntryCount() ~= 0 then
                    return "bad_default_count"
                end
                local message, red, green, blue = CombatLogInbound.GenerateMessage()
                if message ~= "" or red ~= 1 or green ~= 1 or blue ~= 1 then
                    return "bad_inbound_default"
                end
                if CombatLog_Object_IsA ~= C_CombatLog.DoesObjectMatchFilter then
                    return "bad_object_alias"
                end
                if CombatLogAddFilter ~= C_CombatLog.AddEventFilter then
                    return "bad_add_filter_alias"
                end
                if CombatLogClearEntries ~= C_CombatLog.ClearEntries then
                    return "bad_clear_alias"
                end
                if CombatLogGetCurrentEntry ~= C_CombatLog.GetCurrentEntryInfo then
                    return "bad_current_alias"
                end
                if CombatLogGetCurrentEventInfo ~= C_CombatLog.GetCurrentEventInfo then
                    return "bad_event_alias"
                end
                if CombatLogGetNumEntries ~= C_CombatLog.GetEntryCount then
                    return "bad_count_alias"
                end
                if CombatLogGetRetentionTime ~= C_CombatLog.GetEntryRetentionTime then
                    return "bad_retention_get_alias"
                end
                if CombatLogResetFilter ~= C_CombatLog.ClearEventFilters then
                    return "bad_reset_alias"
                end
                if CombatLogSetRetentionTime ~= C_CombatLog.SetEntryRetentionTime then
                    return "bad_retention_set_alias"
                end
                if CombatLogShowCurrentEntry ~= C_CombatLog.ShouldShowCurrentEntry then
                    return "bad_show_alias"
                end
                C_CombatLog._state.entries = {
                    { "first", 1 },
                    { "second", 2 },
                }
                if C_CombatLog.GetEntryCount() ~= 2 or C_CombatLogSecure.GetEntryCount() ~= 2 then
                    return "bad_count"
                end
                local message, value = C_CombatLog.GetCurrentEventInfo()
                if message ~= "second" or value ~= 2 then
                    return "bad_current_event"
                end
                if not C_CombatLogSecure.SeekToNewestEntry() then
                    return "seek_newest_failed"
                end
                if not C_CombatLogSecure.SeekToPreviousEntry() then
                    return "seek_previous_failed"
                end
                message, value = C_CombatLogSecure.GetCurrentEntryInfo()
                if message ~= "first" or value ~= 1 then
                    return "bad_secure_current"
                end
                C_CombatLog.SetEntryRetentionTime(45)
                C_CombatLog.SetFilteredEventsEnabled(true)
                C_CombatLog.SetMessageLimit(10)
                if C_CombatLog.GetEntryRetentionTime() ~= 45 then
                    return "bad_retention"
                end
                if not C_CombatLog.AreFilteredEventsEnabled() then
                    return "bad_filter_flag"
                end
                if C_CombatLog.GetMessageLimit() ~= 10 then
                    return "bad_limit"
                end
                if not C_CombatLogSecure.CreateCombatLogMessage("created", 0.1, 0.2, 0.3, nil) then
                    return "create_failed"
                end
                if C_CombatLog._state.createdMessages[1].message ~= "created" then
                    return "bad_created_message"
                end
                CombatLogSetCurrentEntry(5)
                if CombatLogGetCurrentEntry() ~= 5 then
                    return "bad_legacy_current_entry"
                end
                if not CombatLogAdvanceEntry(2) or CombatLogGetCurrentEntry() ~= 7 then
                    return "bad_legacy_advance"
                end
                if not CombatLogAddFilter("anything") then
                    return "bad_legacy_filter"
                end
                if not CombatLogResetFilter() then
                    return "bad_legacy_reset"
                end
                if not CombatLog_Object_IsA(0x21, 0x01) or CombatLog_Object_IsA(0x20, 0x01) then
                    return "bad_legacy_object_match"
                end
                C_CombatLog.ClearEntries()
                if C_CombatLog.GetEntryCount() ~= 0 or C_CombatLog.ShouldShowCurrentEntry() then
                    return "bad_clear"
                end
                if C_CombatLogSecure.SeekToNewestEntry() or C_CombatLogSecure.SeekToPreviousEntry() then
                    return "bad_empty_seek"
                end
                return "ok"
                "#,
            )
            .expect("combat log probe should run");

        assert_eq!(result, "ok");
    }
}
