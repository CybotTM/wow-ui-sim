//! Extra C_* namespace stubs and global tables split from c_stubs_api.rs.
//!
//! Contains:
//! - C_DelvesUI - Delves companion data
//! - C_ZoneAbility - Zone ability data
//! - C_ItemSocketInfo, C_PetInfo, C_UnitAurasPrivate, C_Sound
//! - Missing global functions, constants, and utility tables

use crate::lua_api::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::OnceLock;

/// Convert a Lua value to f64 for abbreviation functions, returning None for non-numeric.
fn to_abbrev_number(value: &Value) -> Option<f64> {
    match value {
        Value::Nil => None,
        Value::Number(n) => Some(*n),
        Value::Integer(n) => Some(*n as f64),
        Value::String(s) => s.to_str().ok()?.parse::<f64>().ok(),
        _ => None,
    }
}

/// Format a number with B/M/K suffixes. threshold_k controls K cutoff (10000 or 1000).
fn format_abbreviated(n: f64, threshold_k: f64) -> String {
    if n >= 1_000_000_000.0 {
        format!("{:.1}B", n / 1_000_000_000.0)
    } else if n >= 1_000_000.0 {
        format!("{:.1}M", n / 1_000_000.0)
    } else if n >= threshold_k {
        format!("{:.1}K", n / 1_000.0)
    } else {
        format!("{}", n.floor() as i64)
    }
}

/// Register all extra stubs (called from c_stubs_api::register_c_stubs_api).
pub fn register_extra_stubs(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let g = lua.globals();
    register_missing_c_namespaces(lua, &g, state)?;
    register_secure_namespaces(lua, &g)?;
    register_missing_global_functions(lua, &g)?;
    register_missing_constants(lua, &g)?;
    register_missing_global_tables(lua, &g)?;
    super::c_stubs_achievement::register_simulate_ping(lua)?;
    // Re-apply CombatLog global aliases so they share the same function pointer as C_CombatLog.
    super::c_stubs_api_combat::fixup_combat_log_aliases(lua, &g)?;
    Ok(())
}

pub(crate) fn register_diff_missing_namespaces(
    lua: &Lua,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    let g = lua.globals();
    register_account_encounter_proto_namespaces(lua, &g, state)?;
    register_reincarnation_table_util(lua, &g)
}

/// C_* namespace stubs that are referenced during addon loading.
fn register_missing_c_namespaces(
    lua: &Lua,
    g: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    register_item_pet_aura_namespaces(lua, g)?;
    register_utility_namespaces(lua, g)?;
    let _ = state;
    Ok(())
}

/// C_AccountServices, C_ArrowCalloutManager, C_EncounterEvents, C_PrototypeDialog stubs.
fn register_account_encounter_proto_namespaces(
    lua: &Lua,
    g: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    register_c_account_services(lua, g, state)?;
    register_c_arrow_callout_manager(lua, g)?;
    register_c_encounter_events(lua, g)?;
    register_c_prototype_dialog(lua, g)?;
    Ok(())
}

const ACCOUNT_EXPORT_RESULT_SUCCESS: i32 = 0;
const ACCOUNT_EXPORT_RESULT_UNAVAILABLE: i32 = 10;
const ACCOUNT_EXPORT_RESULT_ALREADY_IN_PROGRESS: i32 = 11;

fn save_account_data(state: &mut SimState) -> (bool, i32) {
    if !state.account_save_enabled {
        return (false, ACCOUNT_EXPORT_RESULT_UNAVAILABLE);
    }
    if state.account_save_in_progress {
        return (false, ACCOUNT_EXPORT_RESULT_ALREADY_IN_PROGRESS);
    }

    state.account_save_in_progress = true;
    state.account_locked_post_save = true;
    state.account_save_in_progress = false;
    (true, ACCOUNT_EXPORT_RESULT_SUCCESS)
}

fn register_c_account_services(
    lua: &Lua,
    g: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    let acct = lua.create_table()?;
    let locked = state.clone();
    acct.set(
        "IsAccountLockedPostSave",
        lua.create_function(move |_, ()| Ok(locked.borrow().account_locked_post_save))?,
    )?;
    let enabled = state.clone();
    acct.set(
        "IsAccountSaveEnabled",
        lua.create_function(move |_, ()| Ok(enabled.borrow().account_save_enabled))?,
    )?;
    let in_progress = state.clone();
    acct.set(
        "IsAccountSaveInProgress",
        lua.create_function(move |_, ()| Ok(in_progress.borrow().account_save_in_progress))?,
    )?;
    acct.set(
        "SaveAccountData",
        lua.create_function(move |_, ()| {
            let mut state = state.borrow_mut();
            Ok(save_account_data(&mut state))
        })?,
    )?;
    g.set("C_AccountServices", acct)
}

const ARROW_CALLOUT_MANAGER_LUA: &str = r#"
    C_ArrowCalloutManager = C_ArrowCalloutManager or {}
    local manager = C_ArrowCalloutManager
    manager.activeCallouts = manager.activeCallouts or {}
    manager.acknowledgedCallouts = manager.acknowledgedCallouts or {}

    local function resolveCalloutID(value)
        if type(value) == "table" then
            return value.calloutID
        end
        return value
    end

    local function dispatchCalloutEvent(eventName, payload)
        local frameManager = ArrowCalloutFrameManager
        if frameManager and type(frameManager.OnEvent) == "function" then
            frameManager:OnEvent(eventName, payload)
        end
    end

    manager.ShowCallout = manager.ShowCallout or function(calloutInfo)
        if type(calloutInfo) ~= "table" then
            return false
        end

        local calloutID = calloutInfo.calloutID
        if calloutID == nil then
            return false
        end

        manager.activeCallouts[calloutID] = calloutInfo
        manager.acknowledgedCallouts[calloutID] = nil
        dispatchCalloutEvent("SHOW_ARROW_CALLOUT", calloutInfo)
        return true
    end

    manager.HideCallout = manager.HideCallout or function(value)
        local calloutID = resolveCalloutID(value)
        if calloutID == nil then
            return false
        end

        local hadActiveCallout = manager.activeCallouts[calloutID] ~= nil
        manager.activeCallouts[calloutID] = nil
        if hadActiveCallout then
            dispatchCalloutEvent("HIDE_ARROW_CALLOUT", calloutID)
        end
        return hadActiveCallout
    end

    manager.AcknowledgeCallout = manager.AcknowledgeCallout or function(value)
        local calloutID = resolveCalloutID(value)
        if calloutID == nil then
            return false
        end

        manager.acknowledgedCallouts[calloutID] = true
        manager.HideCallout(calloutID)
        return true
    end

    manager.IsCalloutActive = manager.IsCalloutActive or function(calloutID)
        return manager.activeCallouts[calloutID] ~= nil
    end

    manager.IsCalloutAcknowledged = manager.IsCalloutAcknowledged or function(calloutID)
        return manager.acknowledgedCallouts[calloutID] == true
    end
"#;

fn register_namespace_from_lua_chunk(
    lua: &Lua,
    g: &mlua::Table,
    lua_chunk: &str,
    namespace: &str,
) -> Result<()> {
    lua.load(lua_chunk).exec()?;
    let table: mlua::Table = g.get(namespace)?;
    g.set(namespace, table)
}

fn register_c_arrow_callout_manager(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_namespace_from_lua_chunk(lua, g, ARROW_CALLOUT_MANAGER_LUA, "C_ArrowCalloutManager")
}

const ENCOUNTER_EVENTS_LUA: &str = r#"
    C_EncounterEvents = C_EncounterEvents or {}
    local api = C_EncounterEvents
    api._eventInfoByID = api._eventInfoByID or {
        [1001] = {
            encounterEventID = 1001,
            enabled = true,
            spellID = 853,
            iconFileID = 135963,
            severity = 1,
            icons = 0,
        },
        [1002] = {
            encounterEventID = 1002,
            enabled = true,
            spellID = 31935,
            iconFileID = 135920,
            severity = 2,
            icons = 0,
        },
    }
    api._eventColorOverrides = api._eventColorOverrides or {}
    api._eventSoundOverrides = api._eventSoundOverrides or {}
    api._nextSoundHandle = api._nextSoundHandle or 1

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

    local function soundKey(eventID, trigger)
        return tostring(eventID) .. ":" .. tostring(trigger)
    end

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

    local function copyColor(color)
        if type(color) ~= "table" then
            return nil
        end
        local out = {}
        out.r = color.r or color.red
        out.g = color.g or color.green
        out.b = color.b or color.blue
        if color.a ~= nil or color.alpha ~= nil then
            out.a = color.a or color.alpha
        end
        if out.r == nil or out.g == nil or out.b == nil then
            return nil
        end
        return out
    end

    local function copyEventInfo(eventID)
        local info = api._eventInfoByID[eventID]
        if info == nil then
            return nil
        end
        local out = copyTable(info)
        out.color = copyColor(api._eventColorOverrides[eventID])
        return out
    end

    api.GetEventList = api.GetEventList or function()
        local ids = {}
        for eventID in pairs(api._eventInfoByID) do
            ids[#ids + 1] = eventID
        end
        table.sort(ids)
        return ids
    end

    api.HasEventInfo = api.HasEventInfo or function(encounterEventID)
        local eventID = normalizeNumber(encounterEventID)
        return eventID ~= nil and api._eventInfoByID[eventID] ~= nil
    end

    api.GetEventInfo = api.GetEventInfo or function(encounterEventID)
        local eventID = normalizeNumber(encounterEventID)
        if eventID == nil then
            return nil
        end
        return copyEventInfo(eventID)
    end

    api.GetEventColor = api.GetEventColor or function(encounterEventID)
        local eventID = normalizeNumber(encounterEventID)
        if eventID == nil then
            return nil
        end
        return copyColor(api._eventColorOverrides[eventID])
    end

    api.SetEventColor = api.SetEventColor or function(encounterEventID, color)
        local eventID = normalizeNumber(encounterEventID)
        if eventID == nil or api._eventInfoByID[eventID] == nil then
            return
        end
        if color == nil then
            api._eventColorOverrides[eventID] = nil
            return
        end
        local normalizedColor = copyColor(color)
        if normalizedColor ~= nil then
            api._eventColorOverrides[eventID] = normalizedColor
        end
    end

    api.GetEventSound = api.GetEventSound or function(encounterEventID, trigger)
        local eventID = normalizeNumber(encounterEventID)
        local triggerID = normalizeNumber(trigger)
        if eventID == nil or triggerID == nil then
            return nil
        end
        return copyTable(api._eventSoundOverrides[soundKey(eventID, triggerID)])
    end

    api.SetEventSound = api.SetEventSound or function(encounterEventID, trigger, sound)
        local eventID = normalizeNumber(encounterEventID)
        local triggerID = normalizeNumber(trigger)
        if eventID == nil or triggerID == nil or api._eventInfoByID[eventID] == nil then
            return
        end
        local key = soundKey(eventID, triggerID)
        if sound == nil then
            api._eventSoundOverrides[key] = nil
            return
        end
        local copy = copyTable(sound)
        if copy ~= nil then
            api._eventSoundOverrides[key] = copy
        end
    end

    api.PlayEventSound = api.PlayEventSound or function(encounterEventID, trigger)
        local sound = api.GetEventSound(encounterEventID, trigger)
        if sound == nil then
            return nil
        end
        local handle = api._nextSoundHandle
        api._nextSoundHandle = api._nextSoundHandle + 1
        return handle
    end
"#;

fn register_c_encounter_events(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_namespace_from_lua_chunk(lua, g, ENCOUNTER_EVENTS_LUA, "C_EncounterEvents")
}

const PROTOTYPE_DIALOG_LUA: &str = r#"
    C_PrototypeDialog = C_PrototypeDialog or {}
    local api = C_PrototypeDialog
    api._activeDialogs = api._activeDialogs or {}
    api._removedDialogs = api._removedDialogs or {}
    api._transitionHistory = api._transitionHistory or {}
    api._nextTransitionIndex = api._nextTransitionIndex or 1

    local function normalizeDialogID(value)
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

    local function normalizeOptionID(value)
        if value == nil or type(value) == "table" then
            return nil
        end
        if type(value) == "number" then
            return math.floor(value)
        end
        return value
    end

    local function recordTransition(dialogID, transition, optionID)
        local index = api._nextTransitionIndex
        api._nextTransitionIndex = index + 1
        api._transitionHistory[index] = {
            dialogID = dialogID,
            transition = transition,
            optionID = optionID,
        }
    end

    api.EnsureRemoved = api.EnsureRemoved or function(dialogID)
        local normalizedDialogID = normalizeDialogID(dialogID)
        if normalizedDialogID == nil then
            return false
        end

        local hadActiveDialog = api._activeDialogs[normalizedDialogID] ~= nil
        api._activeDialogs[normalizedDialogID] = nil
        api._removedDialogs[normalizedDialogID] = true
        recordTransition(normalizedDialogID, "removed", nil)
        return hadActiveDialog
    end

    api.SelectOption = api.SelectOption or function(dialogID, optionID)
        local normalizedDialogID = normalizeDialogID(dialogID)
        local normalizedOptionID = normalizeOptionID(optionID)
        if normalizedDialogID == nil or normalizedOptionID == nil then
            return false
        end

        local dialogState = api._activeDialogs[normalizedDialogID]
        if type(dialogState) ~= "table" then
            dialogState = {
                dialogID = normalizedDialogID,
                selectionCount = 0,
            }
            api._activeDialogs[normalizedDialogID] = dialogState
        end

        dialogState.selectionCount = (dialogState.selectionCount or 0) + 1
        dialogState.selectedOptionID = normalizedOptionID
        dialogState.lastTransition = "selected"
        api._removedDialogs[normalizedDialogID] = nil
        recordTransition(normalizedDialogID, "selected", normalizedOptionID)
        return true
    end
"#;

fn register_c_prototype_dialog(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_namespace_from_lua_chunk(lua, g, PROTOTYPE_DIALOG_LUA, "C_PrototypeDialog")
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
fn register_reincarnation_table_util(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(REINCARNATION_LUA).exec()?;
    g.get::<mlua::Table>("C_Reincarnation")
        .and_then(|reincarnation| g.set("C_Reincarnation", reincarnation))?;
    lua.load(TABLE_UTIL_LUA).exec()?;
    g.get::<mlua::Table>("C_TableUtil")
        .and_then(|table_util| g.set("C_TableUtil", table_util))?;

    Ok(())
}

/// C_ItemSocketInfo, C_PetInfo, C_UnitAurasPrivate stubs.
fn register_item_pet_aura_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_c_item_socket_info(lua, g)?;
    register_c_pet_info(lua, g)?;
    register_c_unit_auras_private(lua, g)?;
    Ok(())
}

const ITEM_SOCKET_INFO_LUA: &str = r#"
    C_ItemSocketInfo = C_ItemSocketInfo or {}
    local api = C_ItemSocketInfo

    api._state = api._state or {
        uiType = 0,
        isOpen = true,
        numSockets = 0,
        itemInfo = {
            name = nil,
            icon = nil,
            quality = 0,
            isRefundable = false,
            isBoundTradeable = false,
        },
        socketTypes = {},
        existingSockets = {},
        newSockets = {},
        clickProposals = {},
        artifactRelicItemIDs = {},
        selectedSocketIndex = nil,
        hasBoundGemProposed = false,
        acceptCount = 0,
        closeCount = 0,
        lastAction = nil,
    }

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

    local function normalizedSocketInfo(info)
        if type(info) ~= "table" then
            return nil
        end
        local out = {}
        out.name = info.name
        out.icon = info.icon
        out.link = info.link
        out.gemMatchesSocket = info.gemMatchesSocket == true
        out.isBound = info.isBound == true or info.bound == true
        return out
    end

    local function getNumSockets()
        local state = api._state
        local highest = normalizeIndex(state.numSockets) or 0
        for idx in pairs(state.socketTypes or {}) do
            if type(idx) == "number" and idx > highest then
                highest = idx
            end
        end
        for idx in pairs(state.existingSockets or {}) do
            if type(idx) == "number" and idx > highest then
                highest = idx
            end
        end
        for idx in pairs(state.newSockets or {}) do
            if type(idx) == "number" and idx > highest then
                highest = idx
            end
        end
        return math.max(0, highest)
    end

    local function readSocketInfo(source, index)
        local entry = source[index]
        if type(entry) ~= "table" then
            return nil, nil, false
        end
        return entry.name, entry.icon, entry.gemMatchesSocket == true
    end

    local function recalculateBoundGemProposed()
        local state = api._state
        state.hasBoundGemProposed = false
        for _, info in pairs(state.newSockets or {}) do
            if type(info) == "table" and (info.isBound == true or info.bound == true) then
                state.hasBoundGemProposed = true
                return
            end
        end
    end

    local function itemIDFromInfo(info)
        if type(info) == "number" then
            return math.floor(info)
        end
        if type(info) == "string" then
            local direct = tonumber(info)
            if direct ~= nil then
                return math.floor(direct)
            end
            local fromLink = string.match(info, "item:(%d+)")
            if fromLink ~= nil then
                return tonumber(fromLink)
            end
        end
        if type(info) == "table" then
            local candidate = info.itemID or info.itemId or info.id
            if type(candidate) == "number" then
                return math.floor(candidate)
            end
        end
        return nil
    end

    api.GetCurrUIType = api.GetCurrUIType or function()
        return api._state.uiType or 0
    end

    api.GetNumSockets = api.GetNumSockets or function()
        return getNumSockets()
    end

    api.GetSocketTypes = api.GetSocketTypes or function(index)
        local socketIndex = normalizeIndex(index)
        if socketIndex == nil then
            return ""
        end
        local socketType = api._state.socketTypes[socketIndex]
        if socketType == nil then
            return ""
        end
        return socketType
    end

    api.GetExistingSocketInfo = api.GetExistingSocketInfo or function(index)
        local socketIndex = normalizeIndex(index)
        if socketIndex == nil then
            return nil, nil, false
        end
        return readSocketInfo(api._state.existingSockets or {}, socketIndex)
    end

    api.GetNewSocketInfo = api.GetNewSocketInfo or function(index)
        local socketIndex = normalizeIndex(index)
        if socketIndex == nil then
            return nil, nil, false
        end
        return readSocketInfo(api._state.newSockets or {}, socketIndex)
    end

    api.GetExistingSocketLink = api.GetExistingSocketLink or function(index)
        local socketIndex = normalizeIndex(index)
        if socketIndex == nil then
            return nil
        end
        local info = (api._state.existingSockets or {})[socketIndex]
        if type(info) ~= "table" then
            return nil
        end
        return info.link
    end

    api.GetNewSocketLink = api.GetNewSocketLink or function(index)
        local socketIndex = normalizeIndex(index)
        if socketIndex == nil then
            return nil
        end
        local info = (api._state.newSockets or {})[socketIndex]
        if type(info) ~= "table" then
            return nil
        end
        return info.link
    end

    api.GetSocketItemInfo = api.GetSocketItemInfo or function()
        local itemInfo = api._state.itemInfo or {}
        return itemInfo.name, itemInfo.icon, itemInfo.quality or 0
    end

    api.GetSocketItemRefundable = api.GetSocketItemRefundable or function()
        local itemInfo = api._state.itemInfo or {}
        return itemInfo.isRefundable == true
    end

    api.GetSocketItemBoundTradeable = api.GetSocketItemBoundTradeable or function()
        local itemInfo = api._state.itemInfo or {}
        return itemInfo.isBoundTradeable == true
    end

    api.HasBoundGemProposed = api.HasBoundGemProposed or function()
        return api._state.hasBoundGemProposed == true
    end

    api.ClickSocketButton = api.ClickSocketButton or function(index)
        local socketIndex = normalizeIndex(index)
        if socketIndex == nil or socketIndex < 1 or socketIndex > getNumSockets() then
            return false
        end

        local state = api._state
        state.selectedSocketIndex = socketIndex
        state.lastAction = "click"

        local proposal = (state.clickProposals or {})[socketIndex]
        if type(proposal) == "table" then
            state.newSockets[socketIndex] = normalizedSocketInfo(proposal)
            recalculateBoundGemProposed()
        end
        return true
    end

    api.AcceptSockets = api.AcceptSockets or function()
        local state = api._state
        state.lastAction = "accept"
        state.acceptCount = (state.acceptCount or 0) + 1
        state.isOpen = true

        for index, info in pairs(state.newSockets or {}) do
            if type(info) == "table" then
                state.existingSockets[index] = normalizedSocketInfo(info)
            end
        end

        state.newSockets = {}
        recalculateBoundGemProposed()
        return true
    end

    api.CompleteSocketing = api.CompleteSocketing or function()
        return api.AcceptSockets()
    end

    api.CloseSocketInfo = api.CloseSocketInfo or function()
        local state = api._state
        local wasOpen = state.isOpen ~= false
        state.isOpen = false
        state.closeCount = (state.closeCount or 0) + 1
        state.selectedSocketIndex = nil
        state.lastAction = "close"
        state.newSockets = {}
        recalculateBoundGemProposed()
        return wasOpen
    end

    api.IsArtifactRelicItem = api.IsArtifactRelicItem or function(info)
        local itemID = itemIDFromInfo(info)
        if itemID == nil then
            return false
        end
        return api._state.artifactRelicItemIDs[itemID] == true
    end
"#;

fn register_c_item_socket_info(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(ITEM_SOCKET_INFO_LUA).exec()?;
    g.get::<mlua::Table>("C_ItemSocketInfo")
        .and_then(|item_socket_info| g.set("C_ItemSocketInfo", item_socket_info))
}

const PET_INFO_LUA: &str = r#"
    C_PetInfo = C_PetInfo or {}
    local api = C_PetInfo

    api._state = api._state or {
        petTamersByMapID = {},
        spellByPetActionID = {},
        passivePetActionIDs = {},
        petActionsByID = {},
        lastQueriedMapID = nil,
        lastQueriedPetActionID = nil,
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

    local function copyPosition(position)
        if type(position) ~= "table" then
            return nil
        end
        local x = position.x
        if x == nil then
            x = position[1]
        end
        local y = position.y
        if y == nil then
            y = position[2]
        end
        if type(x) ~= "number" or type(y) ~= "number" then
            return nil
        end
        return { x = x, y = y }
    end

    local function copyTamerInfo(info)
        if type(info) ~= "table" then
            return nil
        end
        local out = {}
        out.areaPoiID = normalizeNumber(info.areaPoiID) or 0
        out.position = copyPosition(info.position)
        out.name = tostring(info.name or "")
        out.atlasName = info.atlasName
        out.textureIndex = normalizeNumber(info.textureIndex)
        return out
    end

    local function copyTamerList(list)
        if type(list) ~= "table" then
            return {}
        end
        local copy = {}
        for index, tamerInfo in ipairs(list) do
            local normalized = copyTamerInfo(tamerInfo)
            if normalized ~= nil then
                copy[index] = normalized
            end
        end
        return copy
    end

    local function readSpellFromActionInfo(actionInfo)
        if type(actionInfo) ~= "table" then
            return nil
        end
        return normalizeNumber(actionInfo.spellID)
    end

    api.GetPetTamersForMap = api.GetPetTamersForMap or function(uiMapID)
        local mapID = normalizeNumber(uiMapID)
        api._state.lastQueriedMapID = mapID
        if mapID == nil then
            return {}
        end
        local list = (api._state.petTamersByMapID or {})[mapID]
        return copyTamerList(list)
    end

    api.GetSpellForPetAction = api.GetSpellForPetAction or function(actionID)
        local normalizedActionID = normalizeNumber(actionID)
        api._state.lastQueriedPetActionID = normalizedActionID
        if normalizedActionID == nil then
            return nil
        end

        local byAction = api._state.spellByPetActionID or {}
        local spellID = normalizeNumber(byAction[normalizedActionID])
        if spellID ~= nil then
            return spellID
        end

        local actionInfo = (api._state.petActionsByID or {})[normalizedActionID]
        return readSpellFromActionInfo(actionInfo)
    end

    api.IsPetActionPassive = api.IsPetActionPassive or function(actionID)
        local normalizedActionID = normalizeNumber(actionID)
        if normalizedActionID == nil then
            return false
        end

        local passiveSet = api._state.passivePetActionIDs or {}
        if passiveSet[normalizedActionID] == true then
            return true
        end

        local actionInfo = (api._state.petActionsByID or {})[normalizedActionID]
        return type(actionInfo) == "table" and actionInfo.isPassive == true
    end
"#;

fn register_c_pet_info(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(PET_INFO_LUA).exec()?;
    g.get::<mlua::Table>("C_PetInfo")
        .and_then(|pet_info| g.set("C_PetInfo", pet_info))
}

const UNIT_AURAS_PRIVATE_LUA: &str = r#"
    C_UnitAurasPrivate = C_UnitAurasPrivate or {}
    C_UnitAuras = C_UnitAuras or {}
    local api = C_UnitAurasPrivate

    api._state = api._state or {
        anchorsByID = {},
        anchorOrder = {},
        nextAnchorID = 1,
        anchorAddedCallback = nil,
        anchorRemovedCallback = nil,
        updateCallbacksByUnit = {},
        warningTextFrame = nil,
        raidBossMessageCallback = nil,
        showDispelTypeCallback = nil,
        lastShowDispelType = nil,
        privateAurasByUnit = {},
        auraDataByUnit = {},
        auraAppliedSoundsByUnitSpell = {},
        anchoredFramesByAnchorID = {},
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

    local function copyTable(value)
        if type(value) ~= "table" then
            return nil
        end
        local copy = {}
        for key, child in pairs(value) do
            copy[key] = child
        end
        return copy
    end

    local function asUnitKey(unit)
        if unit == nil then
            return ""
        end
        return tostring(unit)
    end

    local function removeFromOrder(anchorID)
        local order = api._state.anchorOrder
        for index, id in ipairs(order) do
            if id == anchorID then
                table.remove(order, index)
                return
            end
        end
    end

    local function addAnchorInternal(anchorArgs)
        if type(anchorArgs) ~= "table" then
            return 0
        end

        local state = api._state
        local anchorID = normalizeNumber(anchorArgs.anchorID)
        if anchorID == nil or anchorID <= 0 then
            anchorID = state.nextAnchorID
            state.nextAnchorID = anchorID + 1
        elseif anchorID >= state.nextAnchorID then
            state.nextAnchorID = anchorID + 1
        end

        local anchorInfo = copyTable(anchorArgs) or {}
        anchorInfo.anchorID = anchorID
        state.anchorsByID[anchorID] = anchorInfo

        removeFromOrder(anchorID)
        table.insert(state.anchorOrder, anchorID)

        local callback = state.anchorAddedCallback
        if type(callback) == "function" then
            pcall(callback, copyTable(anchorInfo))
        end
        return anchorID
    end

    local function removeAnchorInternal(anchorID)
        local normalizedAnchorID = normalizeNumber(anchorID)
        if normalizedAnchorID == nil then
            return false
        end

        local state = api._state
        if state.anchorsByID[normalizedAnchorID] == nil then
            return false
        end

        state.anchorsByID[normalizedAnchorID] = nil
        state.anchoredFramesByAnchorID[normalizedAnchorID] = nil
        removeFromOrder(normalizedAnchorID)

        local callback = state.anchorRemovedCallback
        if type(callback) == "function" then
            pcall(callback, normalizedAnchorID)
        end
        return true
    end

    local function getAnchors(unitFilter)
        local state = api._state
        local result = {}
        local unitKey = nil
        if unitFilter ~= nil then
            unitKey = tostring(unitFilter)
        end
        for _, anchorID in ipairs(state.anchorOrder) do
            local anchorInfo = state.anchorsByID[anchorID]
            if type(anchorInfo) == "table" then
                if unitKey == nil or tostring(anchorInfo.unitToken) == unitKey then
                    table.insert(result, copyTable(anchorInfo))
                end
            end
        end
        return result
    end

    local function clonePrivateAuras(unit)
        local state = api._state
        local key = asUnitKey(unit)
        local list = state.privateAurasByUnit[key]
        if type(list) ~= "table" then
            return {}
        end
        local copy = {}
        for index, auraInfo in ipairs(list) do
            if type(auraInfo) == "table" then
                copy[index] = copyTable(auraInfo)
            else
                copy[index] = auraInfo
            end
        end
        return copy
    end

    api.GetAuraDataBySlot = api.GetAuraDataBySlot or function(unit, slot)
        local slotIndex = normalizeNumber(slot)
        if slotIndex == nil or slotIndex < 1 then
            return nil
        end
        local list = clonePrivateAuras(unit)
        return list[slotIndex]
    end

    api.SetPrivateAuraAnchorAddedCallback = api.SetPrivateAuraAnchorAddedCallback or function(callback)
        if type(callback) == "function" then
            api._state.anchorAddedCallback = callback
        else
            api._state.anchorAddedCallback = nil
        end
    end

    api.SetPrivateAuraAnchorRemovedCallback = api.SetPrivateAuraAnchorRemovedCallback or function(callback)
        if type(callback) == "function" then
            api._state.anchorRemovedCallback = callback
        else
            api._state.anchorRemovedCallback = nil
        end
    end

    api.GetPrivateAuraAnchors = api.GetPrivateAuraAnchors or function(unit)
        return getAnchors(unit)
    end

    api.SetPrivateWarningTextFrame = api.SetPrivateWarningTextFrame or function(frame)
        api._state.warningTextFrame = frame
    end

    api.SetPrivateRaidBossMessageCallback = api.SetPrivateRaidBossMessageCallback or function(callback)
        if type(callback) == "function" then
            api._state.raidBossMessageCallback = callback
        else
            api._state.raidBossMessageCallback = nil
        end
    end

    api.SetShowDispelTypeCallback = api.SetShowDispelTypeCallback or function(callback)
        if type(callback) == "function" then
            api._state.showDispelTypeCallback = callback
        else
            api._state.showDispelTypeCallback = nil
        end
    end

    api.AddPrivateAuraUpdateCallback = api.AddPrivateAuraUpdateCallback or function(unit, callback)
        local key = asUnitKey(unit)
        local callbacks = api._state.updateCallbacksByUnit[key]
        if type(callbacks) ~= "table" then
            callbacks = {}
            api._state.updateCallbacksByUnit[key] = callbacks
        end
        if type(callback) ~= "function" then
            return
        end
        for _, existing in ipairs(callbacks) do
            if existing == callback then
                return
            end
        end
        table.insert(callbacks, callback)
    end

    api.GetAllPrivateAuras = api.GetAllPrivateAuras or function(unit)
        return clonePrivateAuras(unit)
    end

    api.GetAuraDataByAuraInstanceIDPrivate = api.GetAuraDataByAuraInstanceIDPrivate or function(unit, auraInstanceID)
        local key = asUnitKey(unit)
        local id = normalizeNumber(auraInstanceID)
        if id == nil then
            return nil
        end
        local byInstance = api._state.auraDataByUnit[key]
        if type(byInstance) ~= "table" then
            return nil
        end
        return copyTable(byInstance[id])
    end

    api.GetAuraAppliedSoundsForSpell = api.GetAuraAppliedSoundsForSpell or function(unit, spellID)
        local key = asUnitKey(unit)
        local normalizedSpellID = normalizeNumber(spellID)
        if normalizedSpellID == nil then
            return {}
        end
        local byUnit = api._state.auraAppliedSoundsByUnitSpell[key]
        if type(byUnit) ~= "table" then
            return {}
        end
        local sounds = byUnit[normalizedSpellID]
        if type(sounds) ~= "table" then
            return {}
        end
        local copy = {}
        for index, sound in ipairs(sounds) do
            if type(sound) == "table" then
                copy[index] = copyTable(sound)
            else
                copy[index] = sound
            end
        end
        return copy
    end

    api.AnchorPrivateAura = api.AnchorPrivateAura or function(frame, icon, duration, anchorID)
        local normalizedAnchorID = normalizeNumber(anchorID)
        if normalizedAnchorID == nil then
            return false
        end
        if api._state.anchorsByID[normalizedAnchorID] == nil then
            return false
        end
        api._state.anchoredFramesByAnchorID[normalizedAnchorID] = {
            frame = frame,
            icon = icon,
            duration = duration,
        }
        return true
    end

    api._TriggerPrivateAuraUpdate = api._TriggerPrivateAuraUpdate or function(unit, privateSource, updateInfo)
        local key = asUnitKey(unit)
        local callbacks = api._state.updateCallbacksByUnit[key]
        if type(callbacks) ~= "table" then
            return 0
        end
        local fired = 0
        for _, callback in ipairs(callbacks) do
            if type(callback) == "function" then
                pcall(callback, privateSource, updateInfo)
                fired = fired + 1
            end
        end
        return fired
    end

    api._TriggerPrivateRaidBossMessage = api._TriggerPrivateRaidBossMessage or function(...)
        local callback = api._state.raidBossMessageCallback
        if type(callback) ~= "function" then
            return false
        end
        pcall(callback, ...)
        return true
    end

    api._AddPrivateAuraAnchorForTest = api._AddPrivateAuraAnchorForTest or function(anchorArgs)
        return addAnchorInternal(anchorArgs)
    end

    api._RemovePrivateAuraAnchorForTest = api._RemovePrivateAuraAnchorForTest or function(anchorID)
        return removeAnchorInternal(anchorID)
    end

    C_UnitAuras.TriggerPrivateAuraShowDispelType = function(showDispelType)
        local showFlag = showDispelType == true
        local state = api._state
        state.lastShowDispelType = showFlag
        if type(state.showDispelTypeCallback) == "function" then
            pcall(state.showDispelTypeCallback, showFlag)
        end
    end

    C_UnitAuras.SetPrivateWarningTextAnchor = function(...)
        api._state.warningTextAnchorArgs = { ... }
        return true
    end
"#;

fn register_c_unit_auras_private(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(UNIT_AURAS_PRIVATE_LUA).exec()?;
    g.get::<mlua::Table>("C_UnitAurasPrivate")
        .and_then(|unit_auras_private| g.set("C_UnitAurasPrivate", unit_auras_private))
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
        [1002] = { zoneName = "The Cinderbrew Meadery", uiMapID = 0 },
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

/// Global functions referenced during addon loading.
fn register_missing_global_functions(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_gameplay_globals(lua, g)?;
    register_legacy_constants(g)?;
    register_combat_log_globals(lua, g)?;
    register_taint_and_env_globals(lua, g)?;
    Ok(())
}

/// Lazy reverse map: normalized path (uppercase, no Interface/ prefix, no extension) → file ID.
static PATH_TO_FILE_ID: OnceLock<HashMap<String, u32>> = OnceLock::new();

fn get_path_to_file_id() -> &'static HashMap<String, u32> {
    PATH_TO_FILE_ID.get_or_init(|| {
        crate::manifest_interface_data::MANIFEST
            .entries()
            .map(|(&id, &path)| (path.to_uppercase(), id))
            .collect()
    })
}

/// Normalize a WoW file path for manifest lookup.
///
/// Converts backslashes to `/`, strips `interface/` prefix (case-insensitive),
/// strips file extension, and uppercases — matching manifest key format.
fn normalize_file_path(raw: &str) -> String {
    let normalized = raw.replace('\\', "/");
    let stripped = normalized
        .strip_prefix("interface/")
        .or_else(|| normalized.strip_prefix("Interface/"))
        .unwrap_or_else(|| {
            let lower = normalized.to_lowercase();
            if lower.starts_with("interface/") {
                &normalized[10..]
            } else {
                &normalized
            }
        });
    let no_ext = stripped
        .rfind('.')
        .map(|i| &stripped[..i])
        .unwrap_or(stripped);
    no_ext.to_uppercase()
}

/// Gameplay-related global function stubs.
fn register_gameplay_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_core_gameplay_globals(lua, g)?;
    register_spell_and_glyph_globals(lua, g)?;
    install_register_ui_panel(lua)?;
    install_encounter_journal_globals(lua)?;
    register_ui_and_progression_globals(lua, g)?;
    register_event_callback(lua)?;
    register_file_path_lookup_global(lua, g)?;
    Ok(())
}

fn register_core_gameplay_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set("IsPlayerInWorld", lua.create_function(|_, ()| Ok(true))?)?;
    g.set(
        "ActionBarController_GetCurrentActionBarState",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    g.set(
        "GetMaxLevelForLatestExpansion",
        lua.create_function(|_, ()| Ok(80i32))?,
    )?;
    Ok(())
}

fn register_spell_and_glyph_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "HasAttachedGlyph",
        lua.create_function(|_, _spell_id: Value| Ok(false))?,
    )?;
    g.set(
        "IsSpellValidForPendingGlyph",
        lua.create_function(|_, _spell_id: Value| Ok(false))?,
    )?;
    g.set(
        "SpellIsSelfBuff",
        lua.create_function(|_, _spell_id: i32| Ok(false))?,
    )?;
    Ok(())
}

fn register_ui_and_progression_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "GetScenariosChoiceOrder",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "GetExpansionDisplayInfo",
        lua.create_function(|_, _expansion_level: Value| Ok(Value::Nil))?,
    )?;
    g.set(
        "AddSourceLocationExclude",
        lua.create_function(|_, _location: Value| Ok(()))?,
    )?;
    g.set(
        "UnitIsHumanPlayer",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?,
    )?;
    Ok(())
}

fn register_file_path_lookup_global(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "GetFileIDFromPath",
        lua.create_function(|_, path: String| {
            let key = normalize_file_path(&path);
            Ok(get_path_to_file_id().get(&key).copied().map(|id| id as i64))
        })?,
    )?;
    Ok(())
}

const ENCOUNTER_JOURNAL_GLOBALS_LUA: &str = r#"
    __wow_sim_ej_state = __wow_sim_ej_state or {}
    local state = __wow_sim_ej_state

    local function normalize_int(value, fallback)
        local number = tonumber(value)
        if number == nil then
            return fallback
        end
        return math.floor(number)
    end

    if normalize_int(state.currentTier, nil) == nil then
        local serverTier = normalize_int((GetServerExpansionLevel and GetServerExpansionLevel() or 0), 0) + 1
        state.currentTier = math.max(1, serverTier)
    end
    state.numTiers = normalize_int(state.numTiers, 12) or 12
    state.currentDifficulty = normalize_int(state.currentDifficulty, 0) or 0
    state.lootClassID = normalize_int(state.lootClassID, 0) or 0
    state.lootSpecID = normalize_int(state.lootSpecID, 0) or 0

    local function get_default_tier()
        local serverTier = normalize_int((GetServerExpansionLevel and GetServerExpansionLevel() or 0), 0) + 1
        return math.max(1, serverTier)
    end

    EJ_GetCurrentTier = EJ_GetCurrentTier or function()
        state.currentTier = math.max(1, normalize_int(state.currentTier, get_default_tier()) or 1)
        return state.currentTier
    end

    EJ_SelectTier = EJ_SelectTier or function(tier)
        state.currentTier = math.max(1, normalize_int(tier, state.currentTier) or 1)
        return state.currentTier
    end

    EJ_GetNumTiers = EJ_GetNumTiers or function()
        return state.numTiers
    end

    EJ_GetTierInfo = EJ_GetTierInfo or function(tier)
        local tierID = normalize_int(tier, state.currentTier) or 1
        return string.format("Tier %d", tierID)
    end

    EJ_GetLootFilter = EJ_GetLootFilter or function()
        state.lootClassID = normalize_int(state.lootClassID, 0) or 0
        state.lootSpecID = normalize_int(state.lootSpecID, 0) or 0
        return state.lootClassID, state.lootSpecID
    end

    EJ_SetLootFilter = EJ_SetLootFilter or function(classID, specID)
        state.lootClassID = normalize_int(classID, 0) or 0
        state.lootSpecID = normalize_int(specID, 0) or 0
    end

    EJ_ResetLootFilter = EJ_ResetLootFilter or function()
        state.lootClassID = 0
        state.lootSpecID = 0
    end

    EJ_GetDifficulty = EJ_GetDifficulty or function()
        state.currentDifficulty = normalize_int(state.currentDifficulty, 0) or 0
        return state.currentDifficulty
    end

    EJ_SetDifficulty = EJ_SetDifficulty or function(difficultyID)
        state.currentDifficulty = normalize_int(difficultyID, state.currentDifficulty) or 0
    end

    EJ_IsValidInstanceDifficulty = EJ_IsValidInstanceDifficulty or function(difficultyID)
        return normalize_int(difficultyID, nil) ~= nil
    end

    C_EncounterJournal = C_EncounterJournal or {}
    C_EncounterJournal.InitalizeSelectedTier = C_EncounterJournal.InitalizeSelectedTier or function()
        local tier = EJ_GetCurrentTier()
        if EJ_SelectTier then
            EJ_SelectTier(tier)
        end
        return tier
    end
"#;

fn install_encounter_journal_globals(lua: &Lua) -> Result<()> {
    lua.load(ENCOUNTER_JOURNAL_GLOBALS_LUA).exec()
}

fn install_register_ui_panel(lua: &Lua) -> Result<()> {
    lua.load(
        r#"
        if RegisterUIPanel == nil then
            function RegisterUIPanel(frame, attributes)
                if frame == nil then
                    return
                end

                local name = frame:GetName()
                if name == nil then
                    return
                end

                if UIPanelWindows[name] == nil then
                    UIPanelWindows[name] = attributes
                end
            end
        end
        "#,
    )
    .exec()
}

/// RegisterEventCallback - validates event names with taint detection.
///
/// Wrapped in Lua to avoid mlua::Error::RuntimeError overhead (12000x slower than
/// Lua error() due to Elune taint bookkeeping on Rust→Lua error boundary).
fn register_event_callback(lua: &Lua) -> Result<()> {
    let callback_tbl = lua.create_table()?;
    let restricted_tbl = lua.create_table()?;
    for &e in crate::event::callback_events() {
        callback_tbl.set(e, true)?;
    }
    for &e in crate::event::restricted_events() {
        restricted_tbl.set(e, true)?;
    }
    lua.load(event_callback_lua_src())
        .call::<()>((callback_tbl, restricted_tbl))
}

fn event_callback_lua_src() -> &'static str {
    concat!(
        "local callback_events, restricted_events = ...\n",
        "local getinfo, getstacktaint = debug.getinfo, debug.getstacktaint\n",
        "local match = string.match\n",
        "local function get_taint()\n",
        "    for level = 2, 30 do\n",
        "        local info = getinfo(level, 'S')\n",
        "        if not info then break end\n",
        "        if info.source then\n",
        "            local addon = match(info.source, 'AddOns/([^/]+)')\n",
        "            if addon then return addon end\n",
        "        end\n",
        "    end\n",
        "    return getstacktaint()\n",
        "end\n",
        "RegisterEventCallback = function(event, callback)\n",
        "    if not callback_events[event] then\n",
        "        local taint = get_taint()\n",
        "        local suffix = taint and ('\\nLua Taint: ' .. taint) or ''\n",
        "        error('RegisterEventCallback Attempt to register unknown event \"' .. event .. '\"' .. suffix, 0)\n",
        "    end\n",
        "    return not restricted_events[event]\n",
        "end\n",
        "RegisterEventCallback = debug.newsecurefunction(RegisterEventCallback)\n",
    )
}

/// Legacy LE_* constants and label strings.
fn register_legacy_constants(g: &mlua::Table) -> Result<()> {
    g.set("NUM_LE_LFG_CATEGORYS", 7i32)?;
    for (i, name) in [
        "OTHER",
        "INTERACTED",
        "IN_GROUP",
        "GUILD",
        "FRIEND",
        "ACCOUNT_CHARACTER",
        "ACCOUNT_CHARACTER_SAME_REALM",
    ]
    .iter()
    .enumerate()
    {
        g.set(format!("LE_AUTOCOMPLETE_PRIORITY_{name}"), (i + 1) as i32)?;
    }
    g.set("AUTOCOMPLETE_LABEL_INTERACTED", "Interacted")?;
    g.set("AUTOCOMPLETE_LABEL_GROUP", "Group")?;
    g.set("AUTOCOMPLETE_LABEL_GUILD", "Guild")?;
    g.set("AUTOCOMPLETE_LABEL_FRIEND", "Friend")?;
    g.set("LE_PARTY_CATEGORY_HOME", 1i32)?;
    g.set("LE_PARTY_CATEGORY_INSTANCE", 2i32)?;
    Ok(())
}

/// CombatLog C++ API functions used by Blizzard_CombatLog.
fn register_combat_log_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_combat_log_entry_stubs(lua, g)?;
    register_combat_log_retention_stubs(lua, g)?;
    register_combat_log_object_stubs(lua, g)?;
    Ok(())
}

fn register_combat_log_entry_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set("CombatLogResetFilter", lua.create_function(|_, ()| Ok(()))?)?;
    g.set(
        "CombatLogAddFilter",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?,
    )?;
    g.set(
        "CombatLogSetCurrentEntry",
        lua.create_function(|_, _index: Value| Ok(()))?,
    )?;
    g.set(
        "CombatLogGetCurrentEntry",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "CombatLogGetNumEntries",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "CombatLogShowCurrentEntry",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "CombatLogAdvanceEntry",
        lua.create_function(|_, _delta: Value| Ok(false))?,
    )?;
    g.set(
        "CombatLogClearEntries",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "CombatLogGetCurrentEventInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )
}

fn register_combat_log_retention_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "CombatLogGetRetentionTime",
        lua.create_function(|_, ()| Ok(300.0f64))?,
    )?;
    g.set(
        "CombatLogSetRetentionTime",
        lua.create_function(|_, _time: Value| Ok(()))?,
    )
}

fn register_combat_log_object_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "CombatLog_Object_IsA",
        lua.create_function(|_, (unit_flags, mask): (i64, i64)| Ok(unit_flags & mask != 0))?,
    )
}

/// Taint system and restricted environment globals.
fn register_taint_and_env_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "GetGlobalEnvironment",
        lua.create_function(|lua, ()| Ok(lua.globals()))?,
    )?;
    // secretwrap(...) -> pass-through, returns all args unchanged.
    g.set(
        "secretwrap",
        lua.create_function(|_, args: mlua::MultiValue| Ok(args))?,
    )?;
    // AbbreviateNumbers: K threshold at 10000.
    g.set(
        "AbbreviateNumbers",
        lua.create_function(|_, (value, _): (Value, Value)| {
            Ok(match to_abbrev_number(&value) {
                None => value.to_string().unwrap_or_else(|_| "0".into()),
                Some(n) => format_abbreviated(n, 10_000.0),
            })
        })?,
    )?;
    // AbbreviateLargeNumbers: K threshold at 1000.
    g.set(
        "AbbreviateLargeNumbers",
        lua.create_function(|_, (value, _): (Value, Value)| {
            Ok(match to_abbrev_number(&value) {
                None => value.to_string().unwrap_or_else(|_| "0".into()),
                Some(n) => format_abbreviated(n, 1_000.0),
            })
        })?,
    )?;
    Ok(())
}

/// Constants tables referenced during addon loading.
fn register_missing_constants(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_bag_constants(lua, g)?;
    register_chat_constants(lua, g)?;
    register_deprecated_garrison_constants(g)?;
    register_deprecated_item_quality_constants(g)?;
    register_deprecated_wow_token_constants(g)?;
    register_deprecated_world_elapsed_timer_constants(g)?;
    // Defined in Blizzard_UIParent/Mainline/UIParent.lua but needed earlier
    // by Blizzard_GameTooltip which loads before Blizzard_UIParent.
    g.set("TOOLTIP_UPDATE_TIME", 0.2f64)?;
    Ok(())
}

fn register_bag_constants(_lua: &Lua, g: &mlua::Table) -> Result<()> {
    // BACKPACK_CONTAINER = Enum.BagIndex.Backpack = 0
    g.set("BACKPACK_CONTAINER", 0i32)?;
    // NUM_BAG_SLOTS + NUM_REAGENTBAG_SLOTS
    g.set("NUM_BAG_SLOTS", 4i32)?;
    g.set("NUM_REAGENTBAG_SLOTS", 1i32)?;
    g.set("NUM_TOTAL_EQUIPPED_BAG_SLOTS", 5i32)?;
    Ok(())
}

fn register_chat_constants(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let cfc = lua.create_table()?;
    cfc.set("MaxCharacterNameBytes", 305i32)?;
    cfc.set("MaxChatChannels", 20i32)?;
    cfc.set("MaxChatWindows", 10i32)?;
    cfc.set("ScrollToBottomFlashInterval", 0.5f64)?;
    cfc.set("WhisperSoundAlertCooldown", 3.0f64)?;
    cfc.set("TruncatedCommunityNameLength", 12i32)?;
    cfc.set("TruncatedCommunityNameWithoutChannelLength", 24i32)?;
    cfc.set("MaxRememberedWhisperTargets", 10i32)?;
    g.set("ChatFrameConstants", cfc)?;
    g.set("MAX_CHARACTER_NAME_BYTES", 305i32)?;
    g.set("MAX_COMMUNITY_NAME_LENGTH", 12i32)?;
    g.set("MAX_COMMUNITY_NAME_LENGTH_NO_CHANNEL", 24i32)?;

    let mfsb = lua.create_table()?;
    mfsb.set("InitialScrollDelay", 0.4f64)?;
    mfsb.set("HeldScrollDelay", 0.04f64)?;
    g.set("MessageFrameScrollButtonConstants", mfsb)?;
    Ok(())
}

fn register_deprecated_garrison_constants(g: &mlua::Table) -> Result<()> {
    g.set("LE_FOLLOWER_MISSION_COMPLETE_STATE_ALIVE", 1i32)?;
    g.set("LE_FOLLOWER_MISSION_COMPLETE_STATE_SAVED", 3i32)?;
    g.set("LE_FOLLOWER_TYPE_GARRISON_7_0", 4i32)?;
    Ok(())
}

fn register_deprecated_item_quality_constants(g: &mlua::Table) -> Result<()> {
    g.set("LE_ITEM_QUALITY_COMMON", 1i32)?;
    g.set("LE_ITEM_QUALITY_UNCOMMON", 2i32)?;
    g.set("LE_ITEM_QUALITY_RARE", 3i32)?;
    g.set("LE_ITEM_QUALITY_EPIC", 4i32)?;
    g.set("LE_ITEM_QUALITY_LEGENDARY", 5i32)?;
    g.set("LE_ITEM_QUALITY_ARTIFACT", 6i32)?;
    g.set("LE_ITEM_QUALITY_HEIRLOOM", 7i32)?;
    g.set("LE_ITEM_QUALITY_WOW_TOKEN", 8i32)?;
    Ok(())
}

fn register_deprecated_wow_token_constants(g: &mlua::Table) -> Result<()> {
    g.set("LE_TOKEN_REDEEM_TYPE_GAME_TIME", 1i32)?;
    g.set("LE_TOKEN_REDEEM_TYPE_BALANCE", 2i32)?;
    g.set("LE_TOKEN_RESULT_ERROR_BALANCE_NEAR_CAP", 10i32)?;
    Ok(())
}

fn register_deprecated_world_elapsed_timer_constants(g: &mlua::Table) -> Result<()> {
    g.set("LE_WORLD_ELAPSED_TIMER_TYPE_NONE", 0i32)?;
    g.set("LE_WORLD_ELAPSED_TIMER_TYPE_CHALLENGE_MODE", 1i32)?;
    g.set("LE_WORLD_ELAPSED_TIMER_TYPE_PROVING_GROUND", 2i32)?;
    Ok(())
}

/// Global Lua tables that are referenced by addon code.
fn register_missing_global_tables(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_simple_stub_tables(lua, g)?;
    register_ui_frame_manager_stub(lua, g)?;
    register_action_button_spell_alert_manager(lua, g)?;
    Ok(())
}

/// QuestUtil, ChatFrameMixin, TalentButtonUtil, SpellSearchUtil, Dispatcher stubs.
fn register_simple_stub_tables(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if g.get::<Value>("QuestUtil")?.is_nil() {
        g.set("QuestUtil", lua.create_table()?)?;
    }
    if g.get::<Value>("ChatFrameMixin")?.is_nil() {
        g.set("ChatFrameMixin", lua.create_table()?)?;
    }
    if g.get::<Value>("ChatFrameEditBoxMixin")?.is_nil() {
        g.set("ChatFrameEditBoxMixin", lua.create_table()?)?;
    }
    if g.get::<Value>("TalentButtonUtil")?.is_nil() {
        g.set("TalentButtonUtil", build_talent_button_util(lua)?)?;
    }
    if g.get::<Value>("SpellSearchUtil")?.is_nil() {
        g.set("SpellSearchUtil", build_spell_search_util(lua)?)?;
    }
    if g.get::<Value>("Dispatcher")?.is_nil() {
        g.set("Dispatcher", build_dispatcher_stub(lua)?)?;
    }
    Ok(())
}

/// UIFrameManager_ManagedFrameMixin stub — needed before Blizzard_UIFrameManager loads.
/// (Blizzard_UIFrameManager loads after Blizzard_Tutorials alphabetically.)
fn register_ui_frame_manager_stub(lua: &Lua, _g: &mlua::Table) -> Result<()> {
    install_ui_frame_manager_namespace(lua)?;
    install_ui_frame_manager_managed_mixin(lua)
}

const UI_FRAME_MANAGER_NAMESPACE_LUA: &str = r#"
    if UIFrameManager == nil then
        UIFrameManager = {
            registeredFrames = {},
            registeredFrameTypeToFrames = {},
        }

        function UIFrameManager:RegisterFrameForFrameType(frame, frameType)
            if self.registeredFrames[frame] then
                return
            end

            if self.registeredFrameTypeToFrames[frameType] == nil then
                self.registeredFrameTypeToFrames[frameType] = {}
            end

            self.registeredFrameTypeToFrames[frameType][frame] = true
            self.registeredFrames[frame] = true

            frame:UpdateFrameState(C_FrameManager.GetFrameVisibilityState(frameType))
        end

        function UIFrameManager:OnEvent(event, ...)
            if event == "FRAME_MANAGER_UPDATE_ALL" then
                for frameType, frames in pairs(self.registeredFrameTypeToFrames) do
                    for frame in pairs(frames) do
                        frame:UpdateFrameState(C_FrameManager.GetFrameVisibilityState(frameType))
                    end
                end
            else
                local frameType, show = ...
                local frames = self.registeredFrameTypeToFrames[frameType]
                if frames then
                    for frame in pairs(frames) do
                        frame:UpdateFrameState(show)
                    end
                end
            end
        end
    end
"#;

fn install_ui_frame_manager_namespace(lua: &Lua) -> Result<()> {
    lua.load(UI_FRAME_MANAGER_NAMESPACE_LUA).exec()
}

const UI_FRAME_MANAGER_MANAGED_MIXIN_LUA: &str = r#"
    if UIFrameManager_ManagedFrameMixin == nil then
        UIFrameManager_ManagedFrameMixin = {}

        function UIFrameManager_ManagedFrameMixin:OnLoad()
            UIFrameManager:RegisterFrameForFrameType(self, self.frameType)
        end

        function UIFrameManager_ManagedFrameMixin:UpdateFrameState(show)
            self:SetShown(show)
        end
    end
"#;

fn install_ui_frame_manager_managed_mixin(lua: &Lua) -> Result<()> {
    lua.load(UI_FRAME_MANAGER_MANAGED_MIXIN_LUA).exec()
}

/// ActionButtonSpellAlertManager stub — referenced by PetBattleUI OnLoad before ActionBar loads.
fn register_action_button_spell_alert_manager(lua: &Lua, _g: &mlua::Table) -> Result<()> {
    install_action_button_spell_alert_manager_namespace(lua)?;
    install_action_button_spell_alert_manager_methods(lua)
}

const ACTION_BUTTON_SPELL_ALERT_MANAGER_NAMESPACE_LUA: &str = r#"
    if ActionButtonSpellAlertManager == nil then
        ActionButtonSpellAlertManager = {
            activeAlerts = {},
            SpellAlertType = { Default = 1, AssistedCombatRotation = 2 },
        }
    end

    if ActionButtonSpellAlertManager.GetAlertFrame == nil then
        function ActionButtonSpellAlertManager:GetAlertFrame(actionButton, create)
            local frame = actionButton.SpellActivationAlert
            if frame == nil and create then
                frame = CreateFrame("Frame", nil, actionButton)
                frame:SetAllPoints(actionButton)
                frame:Hide()
                actionButton.SpellActivationAlert = frame
            end
            return frame
        end
    end
"#;

fn install_action_button_spell_alert_manager_namespace(lua: &Lua) -> Result<()> {
    lua.load(ACTION_BUTTON_SPELL_ALERT_MANAGER_NAMESPACE_LUA)
        .exec()
}

const ACTION_BUTTON_SPELL_ALERT_MANAGER_METHODS_LUA: &str = r#"
    if ActionButtonSpellAlertManager and ActionButtonSpellAlertManager.ShowAlert == nil then
        function ActionButtonSpellAlertManager:ShowAlert(actionButton, skipBirth)
            local currentType = self.activeAlerts[actionButton]
            local alertType = self.SpellAlertType.Default
            if currentType == alertType then
                local alertFrame = self:GetAlertFrame(actionButton, false)
                if alertFrame then
                    alertFrame:Show()
                end
                return
            end

            self.activeAlerts[actionButton] = alertType
            local alertFrame = self:GetAlertFrame(actionButton, true)
            alertFrame:Show()
        end
    end

    if ActionButtonSpellAlertManager and ActionButtonSpellAlertManager.HideAlert == nil then
        function ActionButtonSpellAlertManager:HideAlert(actionButton)
            if self.activeAlerts[actionButton] == nil then
                return
            end

            local alertFrame = self:GetAlertFrame(actionButton, false)
            if alertFrame then
                alertFrame:Hide()
            end
            self.activeAlerts[actionButton] = nil
        end
    end

    if ActionButtonSpellAlertManager and ActionButtonSpellAlertManager.HasAlert == nil then
        function ActionButtonSpellAlertManager:HasAlert(actionButton)
            local alertType = self.activeAlerts[actionButton]
            return alertType ~= nil, alertType
        end
    end
"#;

fn install_action_button_spell_alert_manager_methods(lua: &Lua) -> Result<()> {
    lua.load(ACTION_BUTTON_SPELL_ALERT_MANAGER_METHODS_LUA)
        .exec()
}

/// TalentButtonUtil - utility table for talent button rendering.
fn build_talent_button_util(lua: &Lua) -> Result<mlua::Table> {
    let tbu = lua.create_table()?;
    tbu.set("CircleEdgeDiameterOffset", 1.2f64)?;
    tbu.set("SquareEdgeMinDiameterOffset", 1.2f64)?;
    tbu.set("SquareEdgeMaxDiameterOffset", 1.5f64)?;
    tbu.set("ChoiceEdgeMinDiameterOffset", 1.2f64)?;
    tbu.set("ChoiceEdgeMaxDiameterOffset", 1.5f64)?;
    let bvs = lua.create_table()?;
    for (i, name) in [
        "Normal",
        "Gated",
        "Disabled",
        "Locked",
        "Selectable",
        "Maxed",
        "Invisible",
        "RefundInvalid",
        "DisplayError",
    ]
    .iter()
    .enumerate()
    {
        bvs.set(*name, (i + 1) as i32)?;
    }
    tbu.set("BaseVisualState", bvs)?;
    Ok(tbu)
}

/// SpellSearchUtil - spell search utility tables.
fn build_spell_search_util(lua: &Lua) -> Result<mlua::Table> {
    let ssu = lua.create_table()?;
    let mt = lua.create_table()?;
    for (i, name) in [
        "DescriptionMatch",
        "NameMatch",
        "RelatedMatch",
        "ExactMatch",
        "NotOnActionBar",
        "OnInactiveBonusBar",
        "OnDisabledActionBar",
        "AssistedCombat",
    ]
    .iter()
    .enumerate()
    {
        mt.set(*name, (i + 1) as i32)?;
    }
    ssu.set("MatchType", mt)?;
    let st = lua.create_table()?;
    for (i, name) in ["Trait", "PvPTalent", "SpellBookItem"].iter().enumerate() {
        st.set(*name, (i + 1) as i32)?;
    }
    ssu.set("SourceType", st)?;
    let ft = lua.create_table()?;
    for (i, name) in ["Text", "ActionBar", "Name", "AssistedCombat"]
        .iter()
        .enumerate()
    {
        ft.set(*name, (i + 1) as i32)?;
    }
    ssu.set("FilterType", ft)?;
    ssu.set("ActionBarStatusTooltips", lua.create_table()?)?;
    Ok(ssu)
}

const DISPATCHER_STUB_LUA: &str = r#"
        local dispatcherFrame = CreateFrame("Frame")
        local nextID = 1
        local eventEntries = {}
        local functionHooks = {}
        local scriptHooks = {}

        local function nextToken()
            local id = nextID
            nextID = nextID + 1
            return id
        end

        local function resolveCallback(kind, key, callback)
            if type(callback) == "function" then
                return callback, callback
            end

            if type(callback) ~= "table" then
                return nil, callback
            end

            local method = callback[key]
            if type(method) == "function" then
                return function(...)
                    return method(callback, ...)
                end, callback
            end

            if kind == "event" then
                local onEvent = callback.OnEvent
                if type(onEvent) == "function" then
                    return function(...)
                        return onEvent(callback, key, ...)
                    end, callback
                end
            end

            return nil, callback
        end

        local function removeListEntry(list, match)
            for i = #list, 1, -1 do
                if match(list[i]) then
                    table.remove(list, i)
                end
            end
        end

        local function trimEvent(eventName)
            local entries = eventEntries[eventName]
            if not entries or #entries == 0 then
                eventEntries[eventName] = nil
                if eventName ~= "OnUpdate" then
                    dispatcherFrame:UnregisterEvent(eventName)
                else
                    dispatcherFrame:SetScript("OnUpdate", nil)
                end
            end
        end

        local function dispatchEntries(entries, ...)
            if not entries then
                return
            end

            local removals = {}
            for _, entry in ipairs(entries) do
                entry.callback(...)
                if entry.once then
                    table.insert(removals, entry.id)
                end
            end
            for _, id in ipairs(removals) do
                removeListEntry(entries, function(entry) return entry.id == id end)
            end
        end

        dispatcherFrame:SetScript("OnEvent", function(_, event, ...)
            local entries = eventEntries[event]
            dispatchEntries(entries, ...)
            trimEvent(event)
        end)

        local Dispatcher = {}

        function Dispatcher:RegisterEvent(eventName, callback, once)
            local cb, owner = resolveCallback("event", eventName, callback)
            if not cb then
                return nil
            end

            local entry = {
                id = nextToken(),
                owner = owner,
                callback = cb,
                once = once == true,
            }

            if not eventEntries[eventName] then
                eventEntries[eventName] = {}
                if eventName ~= "OnUpdate" then
                    dispatcherFrame:RegisterEvent(eventName)
                else
                    dispatcherFrame:SetScript("OnUpdate", function(_, elapsed)
                        local entries = eventEntries.OnUpdate
                        dispatchEntries(entries, elapsed)
                        trimEvent("OnUpdate")
                    end)
                end
            end

            table.insert(eventEntries[eventName], entry)
            return entry.id
        end

        function Dispatcher:UnregisterEvent(eventName, ownerOrToken)
            local entries = eventEntries[eventName]
            if not entries then
                return
            end
            removeListEntry(entries, function(entry)
                return entry.id == ownerOrToken or entry.owner == ownerOrToken
            end)
            trimEvent(eventName)
        end

        function Dispatcher:UnregisterAllEvents(ownerOrToken)
            for eventName, entries in pairs(eventEntries) do
                removeListEntry(entries, function(entry)
                    return entry.id == ownerOrToken or entry.owner == ownerOrToken
                end)
                trimEvent(eventName)
            end
        end

        local function functionHookKey(target, method)
            return tostring(target) .. "\31" .. method
        end

        local function trimFunctionHook(hookKey)
            local hook = functionHooks[hookKey]
            if not hook or #hook.entries > 0 then
                return
            end
            hook.target[hook.method] = hook.original
            functionHooks[hookKey] = nil
        end

        local function ensureFunctionHook(target, method)
            local hookKey = functionHookKey(target, method)
            local hook = functionHooks[hookKey]
            if hook then
                return hookKey, hook
            end

            local original = target[method]
            hook = {
                target = target,
                method = method,
                original = original,
                entries = {},
            }
            functionHooks[hookKey] = hook
            target[method] = function(...)
                if type(hook.original) == "function" then
                    hook.original(...)
                end
                local removals = {}
                for _, entry in ipairs(hook.entries) do
                    entry.callback(...)
                    if entry.once then
                        table.insert(removals, entry.id)
                    end
                end
                for _, id in ipairs(removals) do
                    removeListEntry(hook.entries, function(entry) return entry.id == id end)
                end
                trimFunctionHook(hookKey)
            end
            return hookKey, hook
        end

        function Dispatcher:RegisterFunction(targetOrName, methodOrCallback, callbackOrOnce, once)
            local target, method, callback, fireOnce
            if type(targetOrName) == "string" then
                target = _G
                method = targetOrName
                callback = methodOrCallback
                fireOnce = callbackOrOnce
            else
                target = targetOrName
                method = methodOrCallback
                callback = callbackOrOnce
                fireOnce = once
            end

            local cb, owner = resolveCallback("function", method, callback)
            if not cb then
                return nil
            end

            local _, hook = ensureFunctionHook(target, method)
            local entry = {
                id = nextToken(),
                owner = owner,
                callback = cb,
                once = fireOnce == true,
            }
            table.insert(hook.entries, entry)
            return entry.id
        end

        function Dispatcher:UnregisterFunction(targetOrName, methodOrOwner, ownerOrToken)
            local target, method, owner = nil, nil, nil
            if type(targetOrName) == "string" then
                target = _G
                method = targetOrName
                owner = methodOrOwner
            else
                target = targetOrName
                method = methodOrOwner
                owner = ownerOrToken
            end

            if type(method) ~= "string" then
                return
            end

            local hookKey = functionHookKey(target, method)
            local hook = functionHooks[hookKey]
            if not hook then
                return
            end

            removeListEntry(hook.entries, function(entry)
                return entry.id == owner or entry.owner == owner
            end)
            trimFunctionHook(hookKey)
        end

        function Dispatcher:UnregisterAllFunctions(ownerOrToken)
            for hookKey, hook in pairs(functionHooks) do
                removeListEntry(hook.entries, function(entry)
                    return entry.id == ownerOrToken or entry.owner == ownerOrToken
                end)
                trimFunctionHook(hookKey)
            end
        end

        local function scriptHookKey(frame, script)
            return tostring(frame) .. "\31" .. script
        end

        local function trimScriptHook(hookKey)
            local hook = scriptHooks[hookKey]
            if not hook or #hook.entries > 0 then
                return
            end
            hook.frame:SetScript(hook.script, hook.original)
            scriptHooks[hookKey] = nil
        end

        local function ensureScriptHook(frame, script)
            local hookKey = scriptHookKey(frame, script)
            local hook = scriptHooks[hookKey]
            if hook then
                return hookKey, hook
            end

            local original = frame:GetScript(script)
            hook = {
                frame = frame,
                script = script,
                original = original,
                entries = {},
            }
            scriptHooks[hookKey] = hook
            frame:SetScript(script, function(...)
                if type(hook.original) == "function" then
                    hook.original(...)
                end
                local removals = {}
                for _, entry in ipairs(hook.entries) do
                    entry.callback(...)
                    if entry.once then
                        table.insert(removals, entry.id)
                    end
                end
                for _, id in ipairs(removals) do
                    removeListEntry(hook.entries, function(entry) return entry.id == id end)
                end
                trimScriptHook(hookKey)
            end)
            return hookKey, hook
        end

        function Dispatcher:RegisterScript(frame, script, callback, once)
            local cb, owner = resolveCallback("script", script, callback)
            if not cb then
                return nil
            end

            local _, hook = ensureScriptHook(frame, script)
            local entry = {
                id = nextToken(),
                owner = owner,
                callback = cb,
                once = once == true,
            }
            table.insert(hook.entries, entry)
            return entry.id
        end

        function Dispatcher:UnregisterScript(frame, script, ownerOrToken)
            local hookKey = scriptHookKey(frame, script)
            local hook = scriptHooks[hookKey]
            if not hook then
                return
            end

            removeListEntry(hook.entries, function(entry)
                return entry.id == ownerOrToken or entry.owner == ownerOrToken
            end)
            trimScriptHook(hookKey)
        end

        function Dispatcher:UnregisterAllScripts(ownerOrToken)
            for hookKey, hook in pairs(scriptHooks) do
                removeListEntry(hook.entries, function(entry)
                    return entry.id == ownerOrToken or entry.owner == ownerOrToken
                end)
                trimScriptHook(hookKey)
            end
        end

        function Dispatcher:UnregisterAll(ownerOrToken)
            self:UnregisterAllEvents(ownerOrToken)
            self:UnregisterAllFunctions(ownerOrToken)
            self:UnregisterAllScripts(ownerOrToken)
        end

        return Dispatcher
        "#;

fn evaluate_dispatcher_stub(lua: &Lua) -> Result<mlua::Table> {
    lua.load(DISPATCHER_STUB_LUA).eval::<mlua::Table>()
}

/// Dispatcher - event dispatch system (real impl: Blizzard_Dispatcher addon).
fn build_dispatcher_stub(lua: &Lua) -> Result<mlua::Table> {
    evaluate_dispatcher_stub(lua)
}

/// Secure/premium/niche C_* namespaces referenced during addon loading.
fn register_secure_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    super::c_stubs_api_secure::register_auth_ping_store(lua, g)?;
    super::c_stubs_api_social::register_social_feature_stubs(lua, g)?;
    Ok(())
}
