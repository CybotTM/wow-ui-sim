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

/// C_* namespace stubs that are referenced during addon loading.
fn register_missing_c_namespaces(
    lua: &Lua,
    g: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    register_item_pet_aura_namespaces(lua, g)?;
    register_utility_namespaces(lua, g)?;
    register_account_encounter_proto_namespaces(lua, g, state)?;
    register_reincarnation_table_util(lua, g)?;
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

fn register_c_arrow_callout_manager(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(ARROW_CALLOUT_MANAGER_LUA).exec()?;
    g.get::<mlua::Table>("C_ArrowCalloutManager")
        .and_then(|arrow| g.set("C_ArrowCalloutManager", arrow))
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
    lua.load(ENCOUNTER_EVENTS_LUA).exec()?;
    g.get::<mlua::Table>("C_EncounterEvents")
        .and_then(|encounter_events| g.set("C_EncounterEvents", encounter_events))
}

fn register_c_prototype_dialog(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let pd = lua.create_table()?;
    pd.set(
        "EnsureRemoved",
        lua.create_function(|_, _dialog_id: Value| Ok(()))?,
    )?;
    pd.set(
        "SelectOption",
        lua.create_function(|_, (_dialog_id, _option_id): (Value, Value)| Ok(()))?,
    )?;
    g.set("C_PrototypeDialog", pd)
}

/// C_Reincarnation and C_TableUtil stubs.
fn register_reincarnation_table_util(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let ri = lua.create_table()?;
    ri.set(
        "GetReincarnatingCharacter",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    ri.set("IsReincarnating", lua.create_function(|_, ()| Ok(false))?)?;
    ri.set("StartReincarnation", lua.create_function(|_, ()| Ok(()))?)?;
    ri.set("StopReincarnation", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_Reincarnation", ri)?;

    let tu = lua.create_table()?;
    tu.set(
        "FindIndexedMismatch",
        lua.create_function(|_, (_t1, _t2, _fn): (Value, Value, Value)| Ok(Value::Nil))?,
    )?;
    g.set("C_TableUtil", tu)?;

    Ok(())
}

/// C_ItemSocketInfo, C_PetInfo, C_UnitAurasPrivate stubs.
fn register_item_pet_aura_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_c_item_socket_info(lua, g)?;
    register_c_pet_info(lua, g)?;
    register_c_unit_auras_private(lua, g)?;
    Ok(())
}

fn register_c_item_socket_info(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let isi = lua.create_table()?;
    isi.set("GetCurrUIType", lua.create_function(|_, ()| Ok(0i32))?)?;
    isi.set(
        "GetExistingSocketInfo",
        lua.create_function(|_, _idx: i32| Ok(Value::Nil))?,
    )?;
    isi.set("AcceptSockets", lua.create_function(|_, ()| Ok(()))?)?;
    isi.set("CloseSocketInfo", lua.create_function(|_, ()| Ok(()))?)?;
    isi.set(
        "IsArtifactRelicItem",
        lua.create_function(|_, _item: Value| Ok(false))?,
    )?;
    g.set("C_ItemSocketInfo", isi)
}

fn register_c_pet_info(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let pi = lua.create_table()?;
    pi.set(
        "GetPetTamersForMap",
        lua.create_function(|lua, _map_id: Value| lua.create_table())?,
    )?;
    pi.set(
        "GetSpellForPetAction",
        lua.create_function(|_, _action: Value| Ok(Value::Nil))?,
    )?;
    pi.set(
        "IsPetActionPassive",
        lua.create_function(|_, _action: Value| Ok(false))?,
    )?;
    g.set("C_PetInfo", pi)
}

fn register_c_unit_auras_private(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let uap = lua.create_table()?;
    uap.set(
        "GetAuraDataBySlot",
        lua.create_function(|_, (_unit, _slot): (Value, Value)| Ok(Value::Nil))?,
    )?;
    uap.set(
        "SetPrivateAuraAnchorAddedCallback",
        lua.create_function(|_, _cb: Value| Ok(()))?,
    )?;
    uap.set(
        "SetPrivateAuraAnchorRemovedCallback",
        lua.create_function(|_, _cb: Value| Ok(()))?,
    )?;
    uap.set(
        "GetPrivateAuraAnchors",
        lua.create_function(|lua, _unit: Value| lua.create_table())?,
    )?;
    uap.set(
        "SetPrivateWarningTextFrame",
        lua.create_function(|_, _frame: Value| Ok(()))?,
    )?;
    uap.set(
        "SetPrivateRaidBossMessageCallback",
        lua.create_function(|_, _cb: Value| Ok(()))?,
    )?;
    uap.set(
        "SetShowDispelTypeCallback",
        lua.create_function(|_, _cb: Value| Ok(()))?,
    )?;
    g.set("C_UnitAurasPrivate", uap)
}

/// C_LevelLink, C_EventScheduler, C_RestrictedActions, C_TransmogOutfitInfo stubs.
fn register_utility_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let ll = lua.create_table()?;
    ll.set(
        "IsActionLocked",
        lua.create_function(|_, _action_id: Value| Ok(false))?,
    )?;
    ll.set(
        "IsSpellLocked",
        lua.create_function(|_, _spell_id: Value| Ok(false))?,
    )?;
    g.set("C_LevelLink", ll)?;

    let es = lua.create_table()?;
    es.set("CanShowEvents", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_EventScheduler", es)?;
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
    g.set("IsPlayerInWorld", lua.create_function(|_, ()| Ok(true))?)?;
    g.set(
        "ActionBarController_GetCurrentActionBarState",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    g.set(
        "GetMaxLevelForLatestExpansion",
        lua.create_function(|_, ()| Ok(80i32))?,
    )?;
    g.set(
        "HasAttachedGlyph",
        lua.create_function(|_, _spell_id: Value| Ok(false))?,
    )?;
    g.set(
        "IsSpellValidForPendingGlyph",
        lua.create_function(|_, _spell_id: Value| Ok(false))?,
    )?;
    install_register_ui_panel(lua)?;
    g.set(
        "GetScenariosChoiceOrder",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "SpellIsSelfBuff",
        lua.create_function(|_, _spell_id: i32| Ok(false))?,
    )?;
    g.set(
        "GetExpansionDisplayInfo",
        lua.create_function(|_, _expansion_level: Value| Ok(Value::Nil))?,
    )?;
    g.set(
        "AddSourceLocationExclude",
        lua.create_function(|_, _location: Value| Ok(()))?,
    )?;
    register_event_callback(lua)?;
    g.set(
        "UnitIsHumanPlayer",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?,
    )?;
    g.set(
        "GetFileIDFromPath",
        lua.create_function(|_, path: String| {
            let key = normalize_file_path(&path);
            Ok(get_path_to_file_id().get(&key).copied().map(|id| id as i64))
        })?,
    )?;
    Ok(())
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
    )?;
    g.set(
        "CombatLogGetRetentionTime",
        lua.create_function(|_, ()| Ok(300.0f64))?,
    )?;
    g.set(
        "CombatLogSetRetentionTime",
        lua.create_function(|_, _time: Value| Ok(()))?,
    )?;
    g.set(
        "CombatLog_Object_IsA",
        lua.create_function(|_, (unit_flags, mask): (i64, i64)| Ok(unit_flags & mask != 0))?,
    )?;
    Ok(())
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

    let mfsb = lua.create_table()?;
    mfsb.set("InitialScrollDelay", 0.4f64)?;
    mfsb.set("HeldScrollDelay", 0.04f64)?;
    g.set("MessageFrameScrollButtonConstants", mfsb)?;
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
    lua.load(
        r#"
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

        if UIFrameManager_ManagedFrameMixin == nil then
            UIFrameManager_ManagedFrameMixin = {}

            function UIFrameManager_ManagedFrameMixin:OnLoad()
                UIFrameManager:RegisterFrameForFrameType(self, self.frameType)
            end

            function UIFrameManager_ManagedFrameMixin:UpdateFrameState(show)
                self:SetShown(show)
            end
        end
        "#,
    )
    .exec()
}

/// ActionButtonSpellAlertManager stub — referenced by PetBattleUI OnLoad before ActionBar loads.
fn register_action_button_spell_alert_manager(lua: &Lua, _g: &mlua::Table) -> Result<()> {
    lua.load(
        r#"
        if ActionButtonSpellAlertManager == nil then
            ActionButtonSpellAlertManager = {
                activeAlerts = {},
                SpellAlertType = { Default = 1, AssistedCombatRotation = 2 },
            }

            local function GetAlertFrame(actionButton, create)
                local frame = actionButton.SpellActivationAlert
                if frame == nil and create then
                    frame = CreateFrame("Frame", nil, actionButton)
                    frame:SetAllPoints(actionButton)
                    frame:Hide()
                    actionButton.SpellActivationAlert = frame
                end
                return frame
            end

            function ActionButtonSpellAlertManager:ShowAlert(actionButton, skipBirth)
                local currentType = self.activeAlerts[actionButton]
                local alertType = self.SpellAlertType.Default
                if currentType == alertType then
                    local alertFrame = GetAlertFrame(actionButton, false)
                    if alertFrame then
                        alertFrame:Show()
                    end
                    return
                end

                self.activeAlerts[actionButton] = alertType
                local alertFrame = GetAlertFrame(actionButton, true)
                alertFrame:Show()
            end

            function ActionButtonSpellAlertManager:HideAlert(actionButton)
                if self.activeAlerts[actionButton] == nil then
                    return
                end

                local alertFrame = GetAlertFrame(actionButton, false)
                if alertFrame then
                    alertFrame:Hide()
                end
                self.activeAlerts[actionButton] = nil
            end

            function ActionButtonSpellAlertManager:HasAlert(actionButton)
                local alertType = self.activeAlerts[actionButton]
                return alertType ~= nil, alertType
            end
        end
        "#,
    )
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

/// Dispatcher - event dispatch system (real impl: Blizzard_Dispatcher addon).
fn build_dispatcher_stub(lua: &Lua) -> Result<mlua::Table> {
    lua.load(
        r#"
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
        "#,
    )
    .eval::<mlua::Table>()
}

/// Secure/premium/niche C_* namespaces referenced during addon loading.
fn register_secure_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    super::c_stubs_api_secure::register_auth_ping_store(lua, g)?;
    super::c_stubs_api_social::register_social_feature_stubs(lua, g)?;
    Ok(())
}
