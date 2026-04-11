use mlua::{Lua, Result};

pub(super) fn register_missing_c_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    super::namespaces::register_item_pet_aura_namespaces(lua, g)?;
    register_utility_namespaces(lua, g)
}

const REINCARNATION_LUA: &str = r#"
    C_Reincarnation = C_Reincarnation or {}
    local api = C_Reincarnation
    api._isActive = api._isActive == true
    api._character = api._character
    api._history = api._history or {}
    api._nextTransitionIndex = api._nextTransitionIndex or 1

    local function copyTable(input)
        if type(input) ~= "table" then
            return nil
        end
        local copy = {}
        for key, value in pairs(input) do
            copy[key] = value
        end
        return copy
    end

    local function normalizeCharacter(character)
        if character == nil then
            return {
                guid = "Player-0000-00000001",
                name = "SimReincarnatingCharacter",
                classID = 1,
            }
        end
        if type(character) == "table" then
            local copy = copyTable(character)
            return copy or {}
        end
        if type(character) == "string" then
            return {
                guid = character,
                name = "SimReincarnatingCharacter",
            }
        end
        if type(character) == "number" then
            return {
                guid = tostring(math.floor(character)),
                name = "SimReincarnatingCharacter",
            }
        end
        return nil
    end

    local function recordTransition(transition)
        local index = api._nextTransitionIndex
        api._nextTransitionIndex = index + 1
        api._history[index] = {
            transition = transition,
            isActive = api._isActive == true,
        }
    end

    api.IsReincarnating = api.IsReincarnating or function()
        return api._isActive == true
    end

    api.GetReincarnatingCharacter = api.GetReincarnatingCharacter or function()
        return copyTable(api._character)
    end

    api.StartReincarnation = api.StartReincarnation or function(character)
        if api._isActive then
            return false
        end
        local normalizedCharacter = normalizeCharacter(character)
        if normalizedCharacter == nil then
            return false
        end

        api._isActive = true
        api._character = normalizedCharacter
        recordTransition("start")
        return true
    end

    api.StopReincarnation = api.StopReincarnation or function()
        local wasActive = api._isActive == true
        api._isActive = false
        api._character = nil
        recordTransition("stop")
        return wasActive
    end
"#;

const TABLE_UTIL_LUA: &str = r#"
    C_TableUtil = C_TableUtil or {}
    local api = C_TableUtil

    api.FindIndexedMismatch = api.FindIndexedMismatch or function(t1, t2, comparator)
        if type(t1) ~= "table" or type(t2) ~= "table" then
            return nil
        end

        local compare = nil
        if type(comparator) == "function" then
            compare = comparator
        end

        local maxLength = math.max(#t1, #t2)
        for index = 1, maxLength do
            local v1 = t1[index]
            local v2 = t2[index]
            local matches
            if compare ~= nil then
                matches = compare(v1, v2, index)
            else
                matches = (v1 == v2)
            end
            if not matches then
                return index
            end
        end
        return nil
    end
"#;

/// C_Reincarnation and C_TableUtil stubs.
pub(super) fn register_reincarnation_table_util(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(REINCARNATION_LUA).exec()?;
    g.get::<mlua::Table>("C_Reincarnation")
        .and_then(|reincarnation| g.set("C_Reincarnation", reincarnation))?;
    lua.load(TABLE_UTIL_LUA).exec()?;
    g.get::<mlua::Table>("C_TableUtil")
        .and_then(|table_util| g.set("C_TableUtil", table_util))?;

    Ok(())
}

const LEVEL_LINK_LUA: &str = r#"
    C_LevelLink = C_LevelLink or {}
    local api = C_LevelLink

    api._state = api._state or {
        lockedActions = {},
        lockedSpells = {},
        lastActionQuery = nil,
        lastSpellQuery = nil,
    }

    local function normalizeNumber(value)
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

    local function isLocked(lockMap, id)
        if type(lockMap) ~= "table" or id == nil then
            return false
        end
        if lockMap[id] == true then
            return true
        end
        local entry = lockMap[id]
        if type(entry) == "table" then
            return entry.locked == true
        end
        return false
    end

    api.IsActionLocked = api.IsActionLocked or function(actionID)
        local normalized = normalizeNumber(actionID)
        api._state.lastActionQuery = normalized
        return isLocked(api._state.lockedActions, normalized)
    end

    api.IsSpellLocked = api.IsSpellLocked or function(spellID)
        local normalized = normalizeNumber(spellID)
        api._state.lastSpellQuery = normalized
        return isLocked(api._state.lockedSpells, normalized)
    end
"#;

const EVENT_SCHEDULER_LUA: &str = r#"
    C_EventScheduler = C_EventScheduler or {}
    local api = C_EventScheduler

    local function displayInfo(overrides)
        local info = {
            hideTimeLeft = false,
            hideDescription = false,
            overrideAtlas = nil,
            overrideTooltipWidgetSetID = nil,
        }
        if type(overrides) == "table" then
            for key, value in pairs(overrides) do
                info[key] = value
            end
        end
        return info
    end

    local function seededOngoingEvents()
        return {
            {
                areaPoiID = 1001,
                rewardsClaimed = false,
                displayInfo = displayInfo({
                    overrideAtlas = "worldquest-icon-pvpbattle",
                }),
            },
            {
                areaPoiID = 1002,
                rewardsClaimed = true,
                displayInfo = displayInfo({
                    hideDescription = true,
                    overrideAtlas = "Dungeon",
                }),
            },
        }
    end

    local function seededScheduledEvents()
        local now = time()
        local hour = 60 * 60
        local day = 24 * hour
        return {
            {
                eventKey = "pvp-brawl-blitz",
                eventID = 2001,
                areaPoiID = 1003,
                startTime = now + (6 * hour),
                endTime = now + (3 * day),
                duration = (3 * day) - (6 * hour),
                hasReminder = false,
                rewardsClaimed = false,
                displayInfo = displayInfo({
                    overrideAtlas = "worldquest-icon-pvpbattle",
                }),
            },
            {
                eventKey = "darkmoon-faire-arrival",
                eventID = 2002,
                areaPoiID = 1004,
                startTime = now + (5 * day),
                endTime = now + (12 * day),
                duration = 7 * day,
                hasReminder = true,
                rewardsClaimed = false,
                displayInfo = displayInfo({
                    hideTimeLeft = true,
                    overrideTooltipWidgetSetID = 90210,
                }),
            },
        }
    end

    local function populateSeededEventState(state)
        state.ongoingEvents = seededOngoingEvents()
        state.scheduledEvents = seededScheduledEvents()
    end

    local SEEDED_EVENT_LOCATIONS = {
        [1001] = { zoneName = "Warsong Gulch", uiMapID = 8685 },
        [1002] = { zoneName = "The Cinderbrew Meadery", uiMapID = 1980 },
        [1003] = { zoneName = "Arathi Basin", uiMapID = 10440 },
        [1004] = { zoneName = "Darkmoon Island", uiMapID = 5861 },
    }

    api._state = api._state or {
        canShowEvents = nil,
        suppressDisplay = false,
        ongoingEvents = seededOngoingEvents(),
        scheduledEvents = seededScheduledEvents(),
    }

    local function normalizeBool(value)
        if value == true then
            return true
        end
        if value == false then
            return false
        end
        return nil
    end

    local function hasVisibleEvents(list)
        if type(list) ~= "table" then
            return false
        end
        for _, eventInfo in ipairs(list) do
            if eventInfo ~= nil then
                return true
            end
        end
        return false
    end

    api.CanShowEvents = api.CanShowEvents or function()
        local state = api._state
        local override = normalizeBool(state.canShowEvents)
        if override ~= nil then
            return override
        end
        if state.suppressDisplay == true then
            return false
        end
        return hasVisibleEvents(state.ongoingEvents) or hasVisibleEvents(state.scheduledEvents)
    end

    api.GetOngoingEvents = api.GetOngoingEvents or function()
        return api._state.ongoingEvents
    end

    api.GetScheduledEvents = api.GetScheduledEvents or function()
        return api._state.scheduledEvents
    end

    api.HasData = api.HasData or function()
        local state = api._state
        return hasVisibleEvents(state.ongoingEvents) or hasVisibleEvents(state.scheduledEvents)
    end

    api.GetEventZoneName = api.GetEventZoneName or function(areaPoiID)
        local location = SEEDED_EVENT_LOCATIONS[tonumber(areaPoiID)]
        if type(location) ~= "table" then
            return ""
        end
        return tostring(location.zoneName or "")
    end

    api.GetEventUiMapID = api.GetEventUiMapID or function(areaPoiID)
        local location = SEEDED_EVENT_LOCATIONS[tonumber(areaPoiID)]
        if type(location) ~= "table" then
            return 0
        end
        return tonumber(location.uiMapID) or 0
    end

    api.RequestEvents = api.RequestEvents or function()
        populateSeededEventState(api._state)
    end
"#;

/// C_LevelLink, C_EventScheduler, C_RestrictedActions, C_TransmogOutfitInfo stubs.
fn register_utility_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(LEVEL_LINK_LUA).exec()?;
    g.get::<mlua::Table>("C_LevelLink")
        .and_then(|level_link| g.set("C_LevelLink", level_link))?;
    lua.load(EVENT_SCHEDULER_LUA).exec()?;
    g.get::<mlua::Table>("C_EventScheduler")
        .and_then(|event_scheduler| g.set("C_EventScheduler", event_scheduler))?;
    Ok(())
}
