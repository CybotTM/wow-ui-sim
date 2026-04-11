//! C_CombatLog and C_CombatLogSecure namespace stubs.
//!
//! Split from c_stubs_api_combat.rs to keep file sizes manageable.

use mlua::{Lua, Result};

/// C_CombatLog - combat log API (relocated from global functions in modern WoW).
pub(super) fn register_c_combat_log(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(COMBAT_LOG_LUA).exec()?;
    g.get::<mlua::Table>("C_CombatLog")
        .and_then(|combat_log| g.set("C_CombatLog", combat_log))?;
    g.get::<mlua::Table>("C_CombatLogSecure")
        .and_then(|combat_log_secure| g.set("C_CombatLogSecure", combat_log_secure))?;
    Ok(())
}

const COMBAT_LOG_LUA: &str = r#"
    C_CombatLog = C_CombatLog or {}
    C_CombatLogSecure = C_CombatLogSecure or {}

    local api = C_CombatLog
    local secure = C_CombatLogSecure

    local function defaultEntries()
        return {
            {
                1234567889,
                "SPELL_HEAL",
                false,
                "Player-1",
                "Player",
                0x511,
                0,
                "Player-2",
                "Target",
                0x512,
                0,
                82326,
                "Holy Light",
                2,
                275,
                0,
                0,
                false,
            },
            {
                1234567890,
                "SPELL_DAMAGE",
                false,
                "Player-1",
                "Player",
                0x511,
                0,
                "Creature-1",
                "Training Dummy",
                0x10a48,
                0,
                19750,
                "Flash of Light",
                2,
                150,
                0,
                2,
                0,
                0,
                0,
                false,
                false,
                false,
                false,
            },
        }
    end

    api._state = api._state or {}
    secure._state = secure._state or {}

    api._state.retentionTime = tonumber(api._state.retentionTime) or 300
    api._state.filteredEventsEnabled = api._state.filteredEventsEnabled == true
    api._state.messageLimit = math.max(1, math.floor(tonumber(api._state.messageLimit) or 300))
    api._state.restricted = api._state.restricted == true
    api._state.filterSettings = api._state.filterSettings

    if type(secure._state.entries) ~= "table" then
        secure._state.entries = defaultEntries()
    end
    if type(secure._state.createdMessages) ~= "table" then
        secure._state.createdMessages = {}
    end
    if type(secure._state.eventFilters) ~= "table" then
        secure._state.eventFilters = {}
    end

    local function fireFrameEvent(eventName, ...)
        if type(FireEvent) == "function" then
            pcall(FireEvent, eventName, ...)
        end
    end

    local function normalizeIndex(value)
        if type(value) == "number" then
            return math.floor(value)
        end
        if type(value) == "string" then
            local parsed = tonumber(value)
            if parsed ~= nil then
                return math.floor(parsed)
            end
        end
        return nil
    end

    local function entryCount()
        return #secure._state.entries
    end

    local function newestEntryIndex()
        local count = entryCount()
        if count < 1 then
            return nil
        end
        return count
    end

    local function clampedCurrentIndex()
        local index = normalizeIndex(secure._state.currentIndex)
        local count = entryCount()
        if index == nil then
            return newestEntryIndex()
        end
        if index < 1 or index > count then
            return nil
        end
        return index
    end

    local function currentEntry()
        local index = clampedCurrentIndex()
        if index == nil then
            return nil
        end
        return secure._state.entries[index]
    end

    local function currentEntryValues()
        local entry = currentEntry()
        if type(entry) ~= "table" then
            return nil
        end
        return unpack(entry)
    end

    local function setCurrentEntryIndex(index)
        local normalized = normalizeIndex(index)
        if normalized == nil then
            secure._state.currentIndex = nil
            return nil
        end

        local count = entryCount()
        if normalized < 1 or normalized > count then
            secure._state.currentIndex = nil
            return nil
        end

        secure._state.currentIndex = normalized
        return normalized
    end

    secure.AddEventFilter = secure.AddEventFilter or function(eventList, sourceFlags, destFlags)
        table.insert(secure._state.eventFilters, {
            eventList = eventList,
            sourceFlags = sourceFlags,
            destFlags = destFlags,
        })
    end

    secure.ClearEventFilters = secure.ClearEventFilters or function()
        secure._state.eventFilters = {}
    end

    secure.CreateCombatLogMessage = secure.CreateCombatLogMessage or function(message, colorR, colorG, colorB, order)
        table.insert(secure._state.createdMessages, {
            message = message,
            colorR = tonumber(colorR) or 1,
            colorG = tonumber(colorG) or 1,
            colorB = tonumber(colorB) or 1,
            order = order,
        })
        fireFrameEvent(
            "COMBAT_LOG_MESSAGE",
            message,
            tonumber(colorR) or 1,
            tonumber(colorG) or 1,
            tonumber(colorB) or 1,
            order
        )
    end

    secure.GetCurrentEntryInfo = secure.GetCurrentEntryInfo or function()
        return currentEntryValues()
    end

    secure.GetCurrentEventInfo = secure.GetCurrentEventInfo or function()
        return currentEntryValues()
    end

    secure.GetEntryCount = secure.GetEntryCount or function()
        return entryCount()
    end

    secure.SeekToNewestEntry = secure.SeekToNewestEntry or function()
        local newestIndex = newestEntryIndex()
        if newestIndex == nil then
            secure._state.currentIndex = nil
            return nil
        end
        secure._state.currentIndex = newestIndex
        return true
    end

    secure.SeekToPreviousEntry = secure.SeekToPreviousEntry or function()
        local index = clampedCurrentIndex()
        if index == nil then
            return nil
        end
        return setCurrentEntryIndex(index - 1) and true or nil
    end

    secure.ShouldShowCurrentEntry = secure.ShouldShowCurrentEntry or function()
        return currentEntry() ~= nil
    end

    api.DoesObjectMatchFilter = api.DoesObjectMatchFilter or function(unitFlags, mask)
        local flags = tonumber(unitFlags) or 0
        local normalizedMask = tonumber(mask) or 0
        return bit.band(flags, normalizedMask) ~= 0
    end

    api.AddEventFilter = api.AddEventFilter or function(eventList, sourceFlags, destFlags)
        return secure.AddEventFilter(eventList, sourceFlags, destFlags)
    end

    api.ClearEntries = api.ClearEntries or function()
        secure._state.entries = {}
        secure._state.currentIndex = nil
        fireFrameEvent("COMBAT_LOG_ENTRIES_CLEARED")
    end

    api.GetCurrentEntryInfo = api.GetCurrentEntryInfo or function()
        return secure.GetCurrentEntryInfo()
    end

    api.GetCurrentEventInfo = api.GetCurrentEventInfo or function()
        return secure.GetCurrentEventInfo()
    end

    api.GetEntryCount = api.GetEntryCount or function(ignoreFilter)
        return secure.GetEntryCount(ignoreFilter)
    end

    api.ShowCurrentEntry = api.ShowCurrentEntry or function()
        return secure.ShouldShowCurrentEntry()
    end

    api.ShouldShowCurrentEntry = api.ShouldShowCurrentEntry or function()
        return secure.ShouldShowCurrentEntry()
    end

    api.AdvanceEntry = api.AdvanceEntry or function(delta)
        local offset = normalizeIndex(delta)
        if offset == nil then
            return nil
        end

        local index = clampedCurrentIndex()
        if index == nil then
            return nil
        end

        return setCurrentEntryIndex(index - offset) and true or nil
    end

    api.GetEntryRetentionTime = api.GetEntryRetentionTime or function()
        return api._state.retentionTime
    end

    api.GetRetentionTime = api.GetRetentionTime or function()
        return api.GetEntryRetentionTime()
    end

    api.SetEntryRetentionTime = api.SetEntryRetentionTime or function(retentionTime)
        api._state.retentionTime = tonumber(retentionTime) or api._state.retentionTime
    end

    api.SetRetentionTime = api.SetRetentionTime or function(retentionTime)
        api.SetEntryRetentionTime(retentionTime)
    end

    api.GetMessageLimit = api.GetMessageLimit or function()
        return api._state.messageLimit
    end

    api.SetMessageLimit = api.SetMessageLimit or function(messageLimit)
        local normalized = normalizeIndex(messageLimit)
        if normalized == nil or normalized < 1 then
            return
        end
        api._state.messageLimit = normalized
        fireFrameEvent("COMBAT_LOG_MESSAGE_LIMIT_CHANGED", normalized)
    end

    api.AreFilteredEventsEnabled = api.AreFilteredEventsEnabled or function()
        return api._state.filteredEventsEnabled == true
    end

    api.SetFilteredEventsEnabled = api.SetFilteredEventsEnabled or function(enabled)
        api._state.filteredEventsEnabled = enabled == true
    end

    api.IsCombatLogRestricted = api.IsCombatLogRestricted or function()
        return api._state.restricted == true
    end

    api.ClearEventFilters = api.ClearEventFilters or function()
        secure.ClearEventFilters()
    end

    api.ResetFilter = api.ResetFilter or function()
        api.ClearEventFilters()
    end

    api.SetCurrentEntry = api.SetCurrentEntry or function(index)
        setCurrentEntryIndex(index)
    end

    api.ApplyFilterSettings = api.ApplyFilterSettings or function(filterSettings)
        api._state.filterSettings = filterSettings
        fireFrameEvent("COMBAT_LOG_APPLY_FILTER_SETTINGS", filterSettings)
    end

    api.RefilterEntries = api.RefilterEntries or function()
        fireFrameEvent("COMBAT_LOG_REFILTER_ENTRIES")
    end
"#;
