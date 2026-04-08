//! Missing C_* namespace stubs and game-state globals referenced during startup.
//!
//! Split from c_stubs_api.rs — contains social/system namespaces, video options,
//! perks activities, game-state stubs, and incoming summon.

use mlua::{Lua, Result, Value};

const PERKS_ACTIVITIES_LUA: &str = r#"
    C_PerksActivities = C_PerksActivities or {}
    local api = C_PerksActivities

    api._state = api._state or {
        trackedIDs = {},
        activityInfoByID = {},
        chatLinkByID = {},
        removeCount = 0,
        lastRemovedID = nil,
        allTags = { tagName = {} },
        activitiesInfo = {
            activePerksMonth = 1,
            displayMonthName = "",
            secondsRemaining = 0,
            activities = {},
            thresholds = {},
        },
        pendingCompletion = { pendingIDs = {} },
    }

    local function normalizeID(id)
        local n = tonumber(id)
        if n == nil then
            return nil
        end
        return math.floor(n)
    end

    local function ensureState()
        local state = api._state
        if type(state.trackedIDs) ~= "table" then
            state.trackedIDs = {}
        end
        if type(state.activityInfoByID) ~= "table" then
            state.activityInfoByID = {}
        end
        if type(state.chatLinkByID) ~= "table" then
            state.chatLinkByID = {}
        end
        if type(state.allTags) ~= "table" then
            state.allTags = { tagName = {} }
        end
        if type(state.allTags.tagName) ~= "table" then
            state.allTags.tagName = {}
        end
        if type(state.activitiesInfo) ~= "table" then
            state.activitiesInfo = {}
        end
        local activitiesInfo = state.activitiesInfo
        if type(activitiesInfo.activities) ~= "table" then
            activitiesInfo.activities = {}
        end
        if type(activitiesInfo.thresholds) ~= "table" then
            activitiesInfo.thresholds = {}
        end
        if tonumber(activitiesInfo.activePerksMonth) == nil then
            activitiesInfo.activePerksMonth = 1
        end
        if type(activitiesInfo.displayMonthName) ~= "string" then
            activitiesInfo.displayMonthName = ""
        end
        if tonumber(activitiesInfo.secondsRemaining) == nil then
            activitiesInfo.secondsRemaining = 0
        end
        if type(state.pendingCompletion) ~= "table" then
            state.pendingCompletion = { pendingIDs = {} }
        end
        if type(state.pendingCompletion.pendingIDs) ~= "table" then
            state.pendingCompletion.pendingIDs = {}
        end
        return state
    end

    local function copyArray(input)
        local out = {}
        if type(input) ~= "table" then
            return out
        end
        for index, value in ipairs(input) do
            out[index] = value
        end
        return out
    end

    local function copyTableShallow(input)
        if type(input) ~= "table" then
            return {}
        end
        local out = {}
        for key, value in pairs(input) do
            out[key] = value
        end
        return out
    end

    api.AddTrackedPerksActivity = api.AddTrackedPerksActivity or function(id)
        local state = ensureState()
        local activityID = normalizeID(id)
        if activityID == nil then
            return false
        end
        for _, trackedID in ipairs(state.trackedIDs) do
            if normalizeID(trackedID) == activityID then
                return false
            end
        end
        table.insert(state.trackedIDs, activityID)
        return true
    end

    api.GetTrackedPerksActivities = api.GetTrackedPerksActivities or function()
        local state = ensureState()
        local tracked = {}
        for index, id in ipairs(state.trackedIDs) do
            tracked[index] = id
        end
        return { trackedIDs = tracked }
    end

    api.GetPerksActivityInfo = api.GetPerksActivityInfo or function(id)
        local state = ensureState()
        local activityID = normalizeID(id)
        if activityID == nil then
            return nil
        end
        return state.activityInfoByID[activityID]
    end

    api.GetPerksActivityChatLink = api.GetPerksActivityChatLink or function(id)
        local state = ensureState()
        local activityID = normalizeID(id)
        if activityID == nil then
            return nil
        end
        return state.chatLinkByID[activityID]
    end

    api.RemoveTrackedPerksActivity = api.RemoveTrackedPerksActivity or function(id)
        local state = ensureState()
        local activityID = normalizeID(id)
        local removed = false
        if activityID ~= nil then
            for index = #state.trackedIDs, 1, -1 do
                if normalizeID(state.trackedIDs[index]) == activityID then
                    table.remove(state.trackedIDs, index)
                    removed = true
                end
            end
        end
        state.removeCount = (tonumber(state.removeCount) or 0) + 1
        state.lastRemovedID = activityID
        return removed
    end

    api.GetAllPerksActivityTags = api.GetAllPerksActivityTags or function()
        local state = ensureState()
        return {
            tagName = copyArray(state.allTags.tagName),
        }
    end

    api.GetPerksActivitiesInfo = api.GetPerksActivitiesInfo or function()
        local state = ensureState()
        local activitiesInfo = state.activitiesInfo
        local info = copyTableShallow(activitiesInfo)
        info.activities = copyArray(activitiesInfo.activities)
        info.thresholds = copyArray(activitiesInfo.thresholds)
        info.activePerksMonth = normalizeID(activitiesInfo.activePerksMonth) or 1
        info.displayMonthName = tostring(activitiesInfo.displayMonthName or "")
        info.secondsRemaining = normalizeID(activitiesInfo.secondsRemaining) or 0
        return info
    end

    api.GetPerksActivitiesPendingCompletion = api.GetPerksActivitiesPendingCompletion or function()
        local state = ensureState()
        return { pendingIDs = copyArray(state.pendingCompletion.pendingIDs) }
    end

    api.ClearPerksActivitiesPendingCompletion = api.ClearPerksActivitiesPendingCompletion or function()
        local state = ensureState()
        state.pendingCompletion.pendingIDs = {}
    end
"#;

const STORE_GLUE_LUA: &str = r#"
    C_StoreGlue = C_StoreGlue or {}
    local api = C_StoreGlue

    api._state = api._state or {
        disconnectOnLogout = false,
        vasProductReady = false,
        purchaseStateByGuid = {},
        requestedQueueGuids = {},
        requestCharacterQueueTimeCount = 0,
        updateVASPurchaseStatesCount = 0,
        lastRequestedQueueGuid = nil,
    }

    local function ensureState()
        local state = api._state
        if type(state.purchaseStateByGuid) ~= "table" then
            state.purchaseStateByGuid = {}
        end
        if type(state.requestedQueueGuids) ~= "table" then
            state.requestedQueueGuids = {}
        end
        return state
    end

    api.GetDisconnectOnLogout = api.GetDisconnectOnLogout or function()
        return api._state.disconnectOnLogout == true
    end

    api.GetVASProductReady = api.GetVASProductReady or function()
        return api._state.vasProductReady == true
    end

    api.GetVASPurchaseStateInfo = api.GetVASPurchaseStateInfo or function(guid)
        local state = ensureState()
        local key = tostring(guid)
        local entry = state.purchaseStateByGuid[key]
        if type(entry) == "table" then
            local purchaseState = tonumber(entry.purchaseState or entry.state) or 0
            return math.floor(purchaseState), entry.productID, entry.result
        end
        return 0, nil, nil
    end

    api.RequestCharacterQueueTime = api.RequestCharacterQueueTime or function(guid)
        local state = ensureState()
        state.requestCharacterQueueTimeCount = (tonumber(state.requestCharacterQueueTimeCount) or 0) + 1
        state.lastRequestedQueueGuid = guid
        table.insert(state.requestedQueueGuids, guid)
        return true
    end

    api.UpdateVASPurchaseStates = api.UpdateVASPurchaseStates or function()
        local state = ensureState()
        state.updateVASPurchaseStatesCount = (tonumber(state.updateVASPurchaseStatesCount) or 0) + 1
        return true
    end
"#;

const VIDEO_OPTIONS_LUA: &str = r#"
    C_VideoOptions = C_VideoOptions or {}
    local api = C_VideoOptions

    api._state = api._state or {
        defaultGameWindowSize = { x = 1920, y = 1080 },
        currentGameWindowSize = { x = 1920, y = 1080 },
        availableGameWindowSizes = {},
        setGameWindowSizeCount = 0,
        lastSetWindowSize = nil,
        gxAdapterInfo = {},
    }

    local function normalizeAxis(value, fallback)
        local number = tonumber(value)
        if number == nil then
            return fallback
        end
        return math.max(1, math.floor(number))
    end

    local function normalizeSize(size, fallbackX, fallbackY)
        if type(size) ~= "table" then
            return { x = fallbackX, y = fallbackY }
        end
        return {
            x = normalizeAxis(size.x, fallbackX),
            y = normalizeAxis(size.y, fallbackY),
        }
    end

    local function normalizeSizesList(list)
        local normalized = {}
        if type(list) == "table" then
            for _, size in ipairs(list) do
                table.insert(normalized, normalizeSize(size, 1920, 1080))
            end
        end
        return normalized
    end

    local function ensureState()
        local state = api._state
        state.defaultGameWindowSize = normalizeSize(state.defaultGameWindowSize, 1920, 1080)
        state.currentGameWindowSize = normalizeSize(
            state.currentGameWindowSize,
            state.defaultGameWindowSize.x,
            state.defaultGameWindowSize.y
        )
        state.availableGameWindowSizes = normalizeSizesList(state.availableGameWindowSizes)
        if type(state.gxAdapterInfo) ~= "table" then
            state.gxAdapterInfo = {}
        end
        return state
    end

    local function cloneSize(size)
        return { x = size.x, y = size.y }
    end

    api.GetDefaultGameWindowSize = api.GetDefaultGameWindowSize or function(_monitor)
        local state = ensureState()
        return cloneSize(state.defaultGameWindowSize)
    end

    api.GetCurrentGameWindowSize = api.GetCurrentGameWindowSize or function(...)
        local state = ensureState()
        return cloneSize(state.currentGameWindowSize)
    end

    api.GetGameWindowSizes = api.GetGameWindowSizes or function(...)
        local state = ensureState()
        local sizes = {}
        for index, size in ipairs(state.availableGameWindowSizes) do
            sizes[index] = cloneSize(size)
        end
        return sizes
    end

    api.GetGxAdapterInfo = api.GetGxAdapterInfo or function()
        local state = ensureState()
        return state.gxAdapterInfo
    end

    api.IsSpellVisualDensitySystemSupported = api.IsSpellVisualDensitySystemSupported or function()
        return false
    end

    api.SetGameWindowSize = api.SetGameWindowSize or function(x, y)
        local state = ensureState()
        local nextSize = {
            x = normalizeAxis(x, state.currentGameWindowSize.x),
            y = normalizeAxis(y, state.currentGameWindowSize.y),
        }
        state.currentGameWindowSize = nextSize
        state.setGameWindowSizeCount = (tonumber(state.setGameWindowSizeCount) or 0) + 1
        state.lastSetWindowSize = cloneSize(nextSize)
        return true
    end
"#;

/// C_PerksActivities - Monthly activities / Trading Post tracking.
pub(crate) fn register_c_perks_activities(lua: &Lua) -> Result<()> {
    lua.load(PERKS_ACTIVITIES_LUA).exec()?;
    let g = lua.globals();
    g.get::<mlua::Table>("C_PerksActivities")
        .and_then(|namespace| g.set("C_PerksActivities", namespace))
}

/// Missing C_* namespaces and globals referenced during startup events.
pub(crate) fn register_missing_namespaces(lua: &Lua) -> Result<()> {
    register_social_namespaces(lua)?;
    register_system_namespaces(lua)?;
    Ok(())
}

/// Social, friends, and matchmaking namespace stubs.
fn register_social_namespaces(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    register_social_status_namespaces(lua, &g)?;
    register_social_queue_namespace(lua, &g)?;
    Ok(())
}

fn register_social_status_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let spectating = lua.create_table()?;
    spectating.set("IsSpectating", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_SpectatingUI", spectating)?;

    let social = lua.create_table()?;
    social.set("IsMuted", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("IsSilenced", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("IsSquelched", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("IsChatDisabled", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("CanReceiveChat", lua.create_function(|_, ()| Ok(true))?)?;
    g.set("C_SocialRestrictions", social)?;

    let lobby = lua.create_table()?;
    lobby.set("IsParticipating", lua.create_function(|_, ()| Ok(false))?)?;
    lobby.set("IsInQueue", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_LobbyMatchmakerInfo", lobby)?;

    let mentorship = lua.create_table()?;
    mentorship.set(
        "GetMentorshipStatus",
        lua.create_function(|_, _unit: Value| Ok(0i32))?,
    )?;
    mentorship.set(
        "IsActivePlayerConsideredNewcomer",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set("C_PlayerMentorship", mentorship)?;

    let recent_allies = lua.create_table()?;
    recent_allies.set("IsSystemEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_RecentAllies", recent_allies)?;
    Ok(())
}

fn register_social_queue_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let social_queue = lua.create_table()?;
    social_queue.set(
        "GetAllGroups",
        lua.create_function(|lua, _local_only: Option<bool>| lua.create_table())?,
    )?;
    social_queue.set(
        "GetConfig",
        lua.create_function(|lua, ()| {
            let config = lua.create_table()?;
            config.set("toastDuration", 60.0f64)?;
            config.set("enableToasts", false)?;
            Ok(config)
        })?,
    )?;
    g.set("C_SocialQueue", social_queue)?;
    Ok(())
}

/// System, service, and utility namespace stubs.
fn register_system_namespaces(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    super::c_stubs_api_glue::register_system_namespaces(lua, &g)?;
    super::c_stubs_api_store::register_c_account_store(lua)?;
    register_c_video_options(lua)?;
    Ok(())
}

fn register_shared_character_services_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let shared_character_services = lua.create_table()?;
    shared_character_services.set(
        "GetUpgradeDistributions",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set("C_SharedCharacterServices", shared_character_services)?;
    Ok(())
}

fn register_configuration_warnings_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let configuration_warnings = lua.create_table()?;
    configuration_warnings.set(
        "GetConfigurationWarnings",
        lua.create_function(|lua, _include_seen_warnings: Option<bool>| lua.create_table())?,
    )?;
    configuration_warnings.set(
        "GetConfigurationWarningString",
        lua.create_function(|_, _warning: Value| Ok(Value::Nil))?,
    )?;
    g.set("C_ConfigurationWarnings", configuration_warnings)?;
    Ok(())
}

fn register_store_glue_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(STORE_GLUE_LUA).exec()?;
    g.get::<mlua::Table>("C_StoreGlue")
        .and_then(|store_glue| g.set("C_StoreGlue", store_glue))
}

/// C_VideoOptions — screen resolution and graphics queries.
fn register_c_video_options(lua: &Lua) -> Result<()> {
    lua.load(VIDEO_OPTIONS_LUA).exec()?;
    let g = lua.globals();
    g.get::<mlua::Table>("C_VideoOptions")
        .and_then(|video_options| g.set("C_VideoOptions", video_options))
}

/// Game-state global stubs for functions referenced during startup events.
pub(crate) fn register_game_state_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    super::c_stubs_api_glue::register_game_state_namespaces(lua, &g)?;
    register_shared_character_services_namespace(lua, &g)?;
    register_configuration_warnings_namespace(lua, &g)?;
    register_store_glue_namespace(lua, &g)?;
    g.set("IsTargetLoose", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("IsPartyLFG", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("IsPartyWorldPVP", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "PlayerGetTimerunningSeasonID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "UnitDistanceSquared",
        lua.create_function(|_, _unit: Value| Ok((0.0f64, true)))?,
    )?;
    g.set(
        "UnitInOtherParty",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "UnitHasIncomingResurrection",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetLFGRoles",
        lua.create_function(|_, ()| Ok((false, false, false)))?,
    )?;
    g.set(
        "GetLFGReadyCheckUpdate",
        lua.create_function(|_, ()| Ok(mlua::MultiValue::new()))?,
    )?;
    g.set(
        "CanPartyLFGBackfill",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetNumArenaOpponentSpecs",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetArenaOpponentSpec",
        lua.create_function(|_, _index: Value| Ok((0i32, 0i32)))?,
    )?;
    g.set(
        "UnitTreatAsPlayerForDisplay",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetLFGDeserterExpiration",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "UnitHasLFGDeserter",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetWorldPVPQueueStatus",
        lua.create_function(|_, _index: Value| Ok(("none", 0i32, 0i32, 0i32)))?,
    )?;
    g.set(
        "CanHearthAndResurrectFromArea",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetChannelList",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "CanBeRaidTarget",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetRaidTargetIndex",
        lua.create_function(|_, _unit: Value| Ok(Value::Nil))?,
    )?;
    Ok(())
}

/// C_IncomingSummon namespace stubs.
pub(crate) fn register_c_incoming_summon(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "HasIncomingSummon",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    t.set(
        "IncomingSummonStatus",
        lua.create_function(|_, _unit: Value| Ok(0i32))?,
    )?;
    lua.globals().set("C_IncomingSummon", t)?;
    Ok(())
}
