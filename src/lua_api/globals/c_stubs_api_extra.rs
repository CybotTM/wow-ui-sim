//! Extra C_* namespace stubs and global tables split from c_stubs_api.rs.
//!
//! Contains:
//! - C_DelvesUI - Delves companion data
//! - C_ZoneAbility - Zone ability data
//! - C_ItemSocketInfo, C_PetInfo, C_UnitAurasPrivate, C_Sound
//! - Missing global functions, constants, and utility tables

use mlua::{Lua, Result, Value};
use std::collections::HashMap;
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
    if n >= 1_000_000_000.0 { format!("{:.1}B", n / 1_000_000_000.0) }
    else if n >= 1_000_000.0 { format!("{:.1}M", n / 1_000_000.0) }
    else if n >= threshold_k { format!("{:.1}K", n / 1_000.0) }
    else { format!("{}", n.floor() as i64) }
}

/// Register all extra stubs (called from c_stubs_api::register_c_stubs_api).
pub fn register_extra_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    register_missing_c_namespaces(lua, &g)?;
    register_secure_namespaces(lua, &g)?;
    register_missing_global_functions(lua, &g)?;
    register_missing_constants(lua, &g)?;
    register_missing_global_tables(lua, &g)?;
    register_c_delves_ui(lua)?;
    register_c_zone_ability(lua)?;
    super::c_stubs_achievement::register_simulate_ping(lua)?;
    // Re-apply CombatLog global aliases so they share the same function pointer as C_CombatLog.
    super::c_stubs_api_combat::fixup_combat_log_aliases(lua, &g)?;
    Ok(())
}

/// C_* namespace stubs that are referenced during addon loading.
fn register_missing_c_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_item_pet_aura_namespaces(lua, g)?;
    register_utility_namespaces(lua, g)?;
    register_account_encounter_proto_namespaces(lua, g)?;
    register_reincarnation_table_util(lua, g)?;
    Ok(())
}

/// C_AccountServices, C_ArrowCalloutManager, C_EncounterEvents, C_PrototypeDialog stubs.
fn register_account_encounter_proto_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let acct = lua.create_table()?;
    acct.set("IsAccountLockedPostSave", lua.create_function(|_, ()| Ok(false))?)?;
    acct.set("IsAccountSaveEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    acct.set("IsAccountSaveInProgress", lua.create_function(|_, ()| Ok(false))?)?;
    acct.set("SaveAccountData", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_AccountServices", acct)?;

    let arrow = lua.create_table()?;
    arrow.set("AcknowledgeCallout", lua.create_function(|_, _id: Value| Ok(()))?)?;
    arrow.set("HideCallout", lua.create_function(|_, _id: Value| Ok(()))?)?;
    g.set("C_ArrowCalloutManager", arrow)?;

    let ee = lua.create_table()?;
    ee.set("GetEventColor", lua.create_function(|_, _event_id: Value| Ok(Value::Nil))?)?;
    ee.set("GetEventInfo", lua.create_function(|_, _event_id: Value| Ok(Value::Nil))?)?;
    ee.set("GetEventList", lua.create_function(|lua, ()| lua.create_table())?)?;
    ee.set("GetEventSound", lua.create_function(|_, _event_id: Value| Ok(Value::Nil))?)?;
    ee.set("HasEventInfo", lua.create_function(|_, _event_id: Value| Ok(false))?)?;
    ee.set("PlayEventSound", lua.create_function(|_, _event_id: Value| Ok(()))?)?;
    ee.set("SetEventColor", lua.create_function(|_, (_event_id, _color): (Value, Value)| Ok(()))?)?;
    ee.set("SetEventSound", lua.create_function(|_, (_event_id, _sound): (Value, Value)| Ok(()))?)?;
    g.set("C_EncounterEvents", ee)?;

    let pd = lua.create_table()?;
    pd.set("EnsureRemoved", lua.create_function(|_, _dialog_id: Value| Ok(()))?)?;
    pd.set("SelectOption", lua.create_function(|_, (_dialog_id, _option_id): (Value, Value)| Ok(()))?)?;
    g.set("C_PrototypeDialog", pd)?;

    Ok(())
}

/// C_Reincarnation and C_TableUtil stubs.
fn register_reincarnation_table_util(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let ri = lua.create_table()?;
    ri.set("GetReincarnatingCharacter", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    ri.set("IsReincarnating", lua.create_function(|_, ()| Ok(false))?)?;
    ri.set("StartReincarnation", lua.create_function(|_, ()| Ok(()))?)?;
    ri.set("StopReincarnation", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_Reincarnation", ri)?;

    let tu = lua.create_table()?;
    tu.set("FindIndexedMismatch", lua.create_function(|_, (_t1, _t2, _fn): (Value, Value, Value)| Ok(Value::Nil))?)?;
    g.set("C_TableUtil", tu)?;

    Ok(())
}

/// C_ItemSocketInfo, C_PetInfo, C_UnitAurasPrivate stubs.
fn register_item_pet_aura_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let isi = lua.create_table()?;
    isi.set("GetCurrUIType", lua.create_function(|_, ()| Ok(0i32))?)?;
    isi.set("GetExistingSocketInfo", lua.create_function(|_, _idx: i32| Ok(Value::Nil))?)?;
    isi.set("AcceptSockets", lua.create_function(|_, ()| Ok(()))?)?;
    isi.set("CloseSocketInfo", lua.create_function(|_, ()| Ok(()))?)?;
    isi.set("IsArtifactRelicItem", lua.create_function(|_, _item: Value| Ok(false))?)?;
    g.set("C_ItemSocketInfo", isi)?;

    let pi = lua.create_table()?;
    pi.set("GetPetTamersForMap", lua.create_function(|lua, _map_id: Value| lua.create_table())?)?;
    pi.set("GetSpellForPetAction", lua.create_function(|_, _action: Value| Ok(Value::Nil))?)?;
    pi.set("IsPetActionPassive", lua.create_function(|_, _action: Value| Ok(false))?)?;
    g.set("C_PetInfo", pi)?;

    let uap = lua.create_table()?;
    uap.set("GetAuraDataBySlot", lua.create_function(|_, (_unit, _slot): (Value, Value)| Ok(Value::Nil))?)?;
    uap.set("SetPrivateAuraAnchorAddedCallback", lua.create_function(|_, _cb: Value| Ok(()))?)?;
    uap.set("SetPrivateAuraAnchorRemovedCallback", lua.create_function(|_, _cb: Value| Ok(()))?)?;
    uap.set("GetPrivateAuraAnchors", lua.create_function(|lua, _unit: Value| lua.create_table())?)?;
    uap.set("SetPrivateWarningTextFrame", lua.create_function(|_, _frame: Value| Ok(()))?)?;
    uap.set("SetPrivateRaidBossMessageCallback", lua.create_function(|_, _cb: Value| Ok(()))?)?;
    uap.set("SetShowDispelTypeCallback", lua.create_function(|_, _cb: Value| Ok(()))?)?;
    g.set("C_UnitAurasPrivate", uap)?;
    Ok(())
}

/// C_LevelLink, C_EventScheduler, C_RestrictedActions, C_TransmogOutfitInfo stubs.
fn register_utility_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let ll = lua.create_table()?;
    ll.set("IsActionLocked", lua.create_function(|_, _action_id: Value| Ok(false))?)?;
    ll.set("IsSpellLocked", lua.create_function(|_, _spell_id: Value| Ok(false))?)?;
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
            if lower.starts_with("interface/") { &normalized[10..] } else { &normalized }
        });
    let no_ext = stripped.rfind('.').map(|i| &stripped[..i]).unwrap_or(stripped);
    no_ext.to_uppercase()
}

/// Gameplay-related global function stubs.
fn register_gameplay_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set("IsPlayerInWorld", lua.create_function(|_, ()| Ok(true))?)?;
    g.set("ActionBarController_GetCurrentActionBarState", lua.create_function(|_, ()| Ok(1i32))?)?;
    g.set("GetMaxLevelForLatestExpansion", lua.create_function(|_, ()| Ok(80i32))?)?;
    g.set("HasAttachedGlyph", lua.create_function(|_, _spell_id: Value| Ok(false))?)?;
    g.set("IsSpellValidForPendingGlyph", lua.create_function(|_, _spell_id: Value| Ok(false))?)?;
    g.set("RegisterUIPanel", lua.create_function(|_, (_frame, _attrs): (Value, Option<Value>)| Ok(()))?)?;
    g.set("GetScenariosChoiceOrder", lua.create_function(|lua, ()| lua.create_table())?)?;
    g.set("SpellIsSelfBuff", lua.create_function(|_, _spell_id: i32| Ok(false))?)?;
    g.set("GetExpansionDisplayInfo", lua.create_function(|_, _expansion_level: Value| Ok(Value::Nil))?)?;
    g.set("AddSourceLocationExclude", lua.create_function(|_, _location: Value| Ok(()))?)?;
    register_event_callback(lua)?;
    g.set("UnitIsHumanPlayer", lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?)?;
    g.set("GetFileIDFromPath", lua.create_function(|_, path: String| {
        let key = normalize_file_path(&path);
        Ok(get_path_to_file_id().get(&key).copied().map(|id| id as i64))
    })?)?;
    Ok(())
}

/// RegisterEventCallback - validates event names with taint detection.
///
/// Wrapped in Lua to avoid mlua::Error::RuntimeError overhead (12000x slower than
/// Lua error() due to Elune taint bookkeeping on Rust→Lua error boundary).
fn register_event_callback(lua: &Lua) -> Result<()> {
    let callback_tbl = lua.create_table()?;
    let restricted_tbl = lua.create_table()?;
    for &e in crate::event::callback_events() { callback_tbl.set(e, true)?; }
    for &e in crate::event::restricted_events() { restricted_tbl.set(e, true)?; }
    lua.load(event_callback_lua_src()).call::<()>((callback_tbl, restricted_tbl))
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
    for (i, name) in ["OTHER", "INTERACTED", "IN_GROUP", "GUILD",
                       "FRIEND", "ACCOUNT_CHARACTER", "ACCOUNT_CHARACTER_SAME_REALM"]
        .iter().enumerate()
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
    g.set("CombatLogAddFilter", lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?)?;
    g.set("CombatLogSetCurrentEntry", lua.create_function(|_, _index: Value| Ok(()))?)?;
    g.set("CombatLogGetCurrentEntry", lua.create_function(|_, ()| Ok(0i32))?)?;
    g.set("CombatLogGetNumEntries", lua.create_function(|_, ()| Ok(0i32))?)?;
    g.set("CombatLogShowCurrentEntry", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("CombatLogAdvanceEntry", lua.create_function(|_, _delta: Value| Ok(false))?)?;
    g.set("CombatLogClearEntries", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("CombatLogGetCurrentEventInfo", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    g.set("CombatLogGetRetentionTime", lua.create_function(|_, ()| Ok(300.0f64))?)?;
    g.set("CombatLogSetRetentionTime", lua.create_function(|_, _time: Value| Ok(()))?)?;
    g.set("CombatLog_Object_IsA", lua.create_function(|_, (unit_flags, mask): (i64, i64)| {
        Ok(unit_flags & mask != 0)
    })?)?;
    Ok(())
}

/// Taint system and restricted environment globals.
fn register_taint_and_env_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set("GetGlobalEnvironment", lua.create_function(|lua, ()| Ok(lua.globals()))?)?;
    // secretwrap(...) -> pass-through, returns all args unchanged.
    g.set("secretwrap", lua.create_function(|_, args: mlua::MultiValue| Ok(args))?)?;
    // AbbreviateNumbers: K threshold at 10000.
    g.set("AbbreviateNumbers", lua.create_function(|_, (value, _): (Value, Value)| {
        Ok(match to_abbrev_number(&value) {
            None => value.to_string().unwrap_or_else(|_| "0".into()),
            Some(n) => format_abbreviated(n, 10_000.0),
        })
    })?)?;
    // AbbreviateLargeNumbers: K threshold at 1000.
    g.set("AbbreviateLargeNumbers", lua.create_function(|_, (value, _): (Value, Value)| {
        Ok(match to_abbrev_number(&value) {
            None => value.to_string().unwrap_or_else(|_| "0".into()),
            Some(n) => format_abbreviated(n, 1_000.0),
        })
    })?)?;
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
fn register_ui_frame_manager_stub(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if g.get::<Value>("UIFrameManager_ManagedFrameMixin")?.is_nil() {
        let mixin = lua.create_table()?;
        let on_load = lua.load(
            "return function(self) if UIFrameManager and UIFrameManager.RegisterFrameForFrameType then UIFrameManager:RegisterFrameForFrameType(self, self.frameType) end end"
        ).eval::<mlua::Function>()?;
        mixin.set("OnLoad", on_load)?;
        let update_state = lua.load("return function(self, show) self:SetShown(show) end").eval::<mlua::Function>()?;
        mixin.set("UpdateFrameState", update_state)?;
        g.set("UIFrameManager_ManagedFrameMixin", mixin)?;
    }
    Ok(())
}

/// ActionButtonSpellAlertManager stub — referenced by PetBattleUI OnLoad before ActionBar loads.
fn register_action_button_spell_alert_manager(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if g.get::<Value>("ActionButtonSpellAlertManager")?.is_nil() {
        let mgr = lua.create_table()?;
        let noop = lua.create_function(|_, _: mlua::MultiValue| Ok(()))?;
        mgr.set("ShowAlert", noop.clone())?;
        mgr.set("HideAlert", noop)?;
        g.set("ActionButtonSpellAlertManager", mgr)?;
    }
    Ok(())
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
    for (i, name) in ["Normal", "Gated", "Disabled", "Locked", "Selectable",
                       "Maxed", "Invisible", "RefundInvalid", "DisplayError"]
        .iter().enumerate()
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
    for (i, name) in ["DescriptionMatch", "NameMatch", "RelatedMatch", "ExactMatch",
                       "NotOnActionBar", "OnInactiveBonusBar", "OnDisabledActionBar",
                       "AssistedCombat"].iter().enumerate()
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
    for (i, name) in ["Text", "ActionBar", "Name", "AssistedCombat"].iter().enumerate() {
        ft.set(*name, (i + 1) as i32)?;
    }
    ssu.set("FilterType", ft)?;
    ssu.set("ActionBarStatusTooltips", lua.create_table()?)?;
    Ok(ssu)
}

/// Dispatcher - event dispatch system (real impl: Blizzard_Dispatcher addon).
fn build_dispatcher_stub(lua: &Lua) -> Result<mlua::Table> {
    let d = lua.create_table()?;
    d.set("Events", lua.create_table()?)?;
    d.set("Functions", lua.create_table()?)?;
    d.set("Scripts", lua.create_table()?)?;
    d.set("NextEventID", 1i32)?;
    d.set("NextFunctionID", 1i32)?;
    d.set("NextScriptID", 1i32)?;
    let noop = lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?;
    for name in ["Initialize", "OnEvent", "RegisterEvent", "UnregisterEvent",
                  "UnregisterAllEvents", "UnregisterAll", "RegisterFunction",
                  "UnregisterFunction", "UnregisterAllFunctions",
                  "RegisterScript", "UnregisterScript", "UnregisterAllScripts"]
    {
        d.set(name, noop.clone())?;
    }
    Ok(d)
}

/// Secure/premium/niche C_* namespaces referenced during addon loading.
fn register_secure_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_auth_ping_store(lua, g)?;
    register_trial_raf_token(lua, g)?;
    register_shop_who_auras(lua, g)?;
    register_guild_bank_pet_battles(lua, g)?;
    Ok(())
}

/// C_AuthChallenge, C_PingSecure, C_Ping, C_StoreSecure stubs.
fn register_auth_ping_store(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let auth_challenge = lua.create_table()?;
    auth_challenge.set("SetFrame", lua.create_function(|_, _frame: Value| Ok(()))?)?;
    g.set("C_AuthChallenge", auth_challenge)?;

    register_c_ping_secure(lua)?;
    register_c_ping(lua)?;

    // C_WowTokenSecure - secure token operations (noop metatable)
    lua.load(r#"
        C_WowTokenSecure = setmetatable({}, {
            __index = function() return function() end end,
        })
    "#).exec()?;

    // C_StoreSecure - uses noop metatable for ~40 methods
    lua.load(r#"
        C_StoreSecure = setmetatable({
            IsStoreAvailable = function() return false end,
            IsAvailable = function() return false end,
            HasPurchaseInProgress = function() return false end,
            HasPurchaseList = function() return false end,
            HasProductList = function() return false end,
        }, { __index = function() return function() end end })
    "#).exec()?;
    Ok(())
}

/// C_PingSecure - stores callbacks for Blizzard PingUI, implements action methods.
fn register_c_ping_secure(lua: &Lua) -> Result<()> {
    lua.load(r#"
        _G.__PingSecureCallbacks = _G.__PingSecureCallbacks or {}
        local cbs = _G.__PingSecureCallbacks
        C_PingSecure = {
            SetPingRadialWheelCreatedCallback = function(cb) cbs.RadialWheelCreated = cb end,
            SetPingPinFrameAddedCallback = function(cb) cbs.PingPinFrameAdded = cb end,
            SetPingPinFrameRemovedCallback = function(cb) cbs.PingPinFrameRemoved = cb end,
            SetPingPinFrameScreenClampStateUpdatedCallback = function(cb) cbs.ScreenClampStateUpdated = cb end,
            SetSendMacroPingCallback = function(cb) cbs.SendMacroPing = cb end,
            SetTogglePingListenerCallback = function(cb) cbs.TogglePingListener = cb end,
            SetPendingPingOffScreenCallback = function(cb) cbs.PendingPingOffScreen = cb end,
            SetPingCooldownStartedCallback = function(cb) cbs.PingCooldownStarted = cb end,
            CreateFrame = function()
                local f = CreateFrame("Frame", nil, UIParent)
                if cbs.RadialWheelCreated then cbs.RadialWheelCreated(f) end
            end,
            SendPing = function(pingType, guid) return Enum.PingResult.Success end,
            GetTargetPingReceiver = function(x, y) return nil end,
            GetTargetWorldPing = function(x, y) return true end,
            GetTargetWorldPingAndSend = function()
                return { result = Enum.PingResult.Success }
            end,
            DisplayError = function(err) end,
            ClearPendingPingInfo = function() end,
        }
    "#).exec()
}

fn ping_get_default_options(lua: &Lua, (): ()) -> Result<mlua::Table> {
    let result = lua.create_table()?;
    let entries: &[(i32, &str)] =
        &[(0, "Attack"), (1, "Warning"), (2, "Assist"), (3, "OnMyWay")];
    for (i, (order_index, texture_kit)) in entries.iter().enumerate() {
        let entry = lua.create_table()?;
        entry.set("orderIndex", *order_index)?;
        entry.set("type", *order_index)?;
        entry.set("uiTextureKitID", *texture_kit)?;
        result.set(i + 1, entry)?;
    }
    Ok(result)
}

fn ping_get_texture_kit(lua: &Lua, ping_type: Value) -> Result<Value> {
    let n = match ping_type {
        Value::Integer(n) => n,
        Value::Number(n) => n as i64,
        _ => return Ok(Value::Nil),
    };
    let kit: Option<&str> = match n {
        0 => Some("Attack"),
        1 => Some("Warning"),
        2 => Some("Assist"),
        3 => Some("OnMyWay"),
        4 => Some("Threat"),
        5 => Some("NonThreat"),
        _ => None,
    };
    match kit {
        Some(s) => Ok(Value::String(lua.create_string(s)?)),
        None => Ok(Value::Nil),
    }
}

/// C_Ping - non-secure ping API with real data for PingManager:SetupDefaultPingOptions.
fn register_c_ping(lua: &Lua) -> Result<()> {
    let ping = lua.create_table()?;
    ping.set("GetCooldownInfo", lua.create_function(|_, _: mlua::MultiValue| Ok(Value::Nil))?)?;
    ping.set("GetDefaultPingOptions", lua.create_function(ping_get_default_options)?)?;
    ping.set("GetTextureKitForType", lua.create_function(ping_get_texture_kit)?)?;
    ping.set("IsPingSystemEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_Ping", ping)?;
    Ok(())
}

/// C_ClassTrial, C_RecruitAFriend, C_WowTokenPublic, C_FriendList stubs.
fn register_trial_raf_token(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_c_class_trial(lua, g)?;
    register_c_recruit_a_friend(lua, g)?;
    register_c_wow_token_public(lua, g)?;
    register_c_friend_list(lua, g)?;
    Ok(())
}

/// C_ClassTrial stubs.
fn register_c_class_trial(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsClassTrialCharacter", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("GetClassTrialLogoutTimeSeconds", lua.create_function(|_, ()| Ok(0i32))?)?;
    g.set("C_ClassTrial", t)
}

/// C_RecruitAFriend stubs.
fn register_c_recruit_a_friend(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetRecruitInfo", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsRecruitingEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("GetRAFInfo", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set("GetRAFSystemInfo", lua.create_function(|lua, ()| {
        let info = lua.create_table()?;
        info.set("maxRecruits", 0i32)?;
        info.set("maxRecruitMonths", 0i32)?;
        info.set("maxRewardMonths", 0i32)?;
        info.set("daysInCycle", 30i32)?;
        Ok(info)
    })?)?;
    g.set("C_RecruitAFriend", t)
}

/// C_WowTokenPublic stubs.
fn register_c_wow_token_public(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetCurrentMarketPrice", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("GetGuaranteedPrice", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("UpdateTokenCount", lua.create_function(|_, ()| Ok(()))?)?;
    t.set("GetCommerceSystemStatus", lua.create_function(|_, ()| Ok((false, false, false)))?)?;
    t.set("UpdateMarketPrice", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_WowTokenPublic", t)
}

/// C_FriendList stubs.
fn register_c_friend_list(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("SetWhoToUi", lua.create_function(|_, _flag: bool| Ok(()))?)?;
    t.set("SendWho", lua.create_function(|_, _msg: String| Ok(()))?)?;
    t.set("GetNumWhoResults", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("GetNumFriends", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("GetNumOnlineFriends", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("GetFriendInfoByIndex", lua.create_function(|_, _idx: i32| Ok(Value::Nil))?)?;
    t.set("ShowFriends", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_FriendList", t)
}

/// C_CatalogShop, C_Who, C_PrivateAuras stubs.
fn register_shop_who_auras(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let catalog_shop = lua.create_table()?;
    catalog_shop.set("GetAvailableCategoryIDs", lua.create_function(|lua, ()| lua.create_table())?)?;
    catalog_shop.set("IsShop2Enabled", lua.create_function(|_, ()| Ok(false))?)?;
    catalog_shop.set("HasNewProducts", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_CatalogShop", catalog_shop)?;

    let who = lua.create_table()?;
    who.set("SetWhoToUi", lua.create_function(|_, _flag: bool| Ok(()))?)?;
    who.set("SendWho", lua.create_function(|_, _msg: String| Ok(()))?)?;
    who.set("GetWhoInfo", lua.create_function(|_, _index: i32| Ok(Value::Nil))?)?;
    g.set("C_Who", who)?;

    let private_auras = lua.create_table()?;
    private_auras.set("SetPrivateRaidBossMessageCallback", lua.create_function(|_, _cb: Value| Ok(()))?)?;
    g.set("C_PrivateAuras", private_auras)?;
    Ok(())
}

/// C_GuildBank, C_PetBattles stubs.
fn register_guild_bank_pet_battles(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let guild_bank = lua.create_table()?;
    guild_bank.set("IsGuildBankEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    guild_bank.set("GetCurrentBankTab", lua.create_function(|_, ()| Ok(1i32))?)?;
    guild_bank.set("FetchNumTabs", lua.create_function(|_, ()| Ok(0i32))?)?;
    g.set("C_GuildBank", guild_bank)?;

    // C_PetBattles - plain table, no metatable (Wowless expects getmetatable == nil).
    let pet = lua.create_table()?;
    pet.set("IsInBattle", lua.create_function(|_, ()| Ok(false))?)?;
    pet.set("IsWildBattle", lua.create_function(|_, ()| Ok(false))?)?;
    pet.set("IsPlayerNPC", lua.create_function(|_, ()| Ok(false))?)?;
    pet.set("GetAllEffectNames", lua.create_function(|_, ()| Ok(()))?)?;
    pet.set("GetAllStates", lua.create_function(|lua, ()| lua.create_table())?)?;
    pet.set("GetBattleState", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    pet.set("GetPVPMatchmakingInfo", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    g.set("C_PetBattles", pet)?;
    Ok(())
}

/// C_DelvesUI namespace - Delves companion data.
fn register_c_delves_ui(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetTraitTreeForCompanion", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("GetRoleNodeForCompanion", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set("GetRoleSubtreeForCompanion", lua.create_function(|_, _role_type: Value| Ok(0i32))?)?;
    t.set("GetCreatureDisplayInfoForCompanion", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("GetCurioNodeForCompanion", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set("GetCurrentDelvesSeasonNumber", lua.create_function(|_, ()| Ok(1i32))?)?;
    t.set("GetDelvesMinRequiredLevel", lua.create_function(|_, ()| Ok(80i32))?)?;
    t.set("GetFactionForCompanion", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("HasActiveDelve", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("GetUnseenCuriosBySlotType", lua.create_function(|lua, _slot_type: Value| lua.create_table())?)?;
    t.set("GetDelvesFactionForSeason", lua.create_function(|_, _season: Value| Ok(Value::Nil))?)?;
    t.set("RequestPartyEligibilityForDelveTiers", lua.create_function(|_, ()| Ok(()))?)?;
    t.set("SaveSeenCuriosBySlotType", lua.create_function(|_, (_slot_type, _table): (Value, Value)| Ok(()))?)?;
    lua.globals().set("C_DelvesUI", t)?;
    Ok(())
}

/// C_ZoneAbility namespace - zone ability data.
fn register_c_zone_ability(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetActiveAbilities", lua.create_function(|lua, ()| lua.create_table())?)?;
    t.set("GetZoneAbilityIcon", lua.create_function(|_, _spell_id: Value| Ok(Value::Nil))?)?;
    lua.globals().set("C_ZoneAbility", t)?;
    Ok(())
}

