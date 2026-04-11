use crate::lua_api::SimState;
use mlua::{Lua, Result};
use std::cell::RefCell;
use std::rc::Rc;

/// C_AccountServices, C_ArrowCalloutManager, C_EncounterEvents, C_PrototypeDialog stubs.
pub(super) fn register_account_encounter_proto_namespaces(
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
