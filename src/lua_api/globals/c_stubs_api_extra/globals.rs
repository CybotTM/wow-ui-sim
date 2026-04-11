use mlua::{Lua, Result, Value};
use std::collections::HashMap;
use std::sync::OnceLock;

pub(super) fn register_missing_global_functions(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
            Ok(match super::to_abbrev_number(&value) {
                None => value.to_string().unwrap_or_else(|_| "0".into()),
                Some(n) => super::format_abbreviated(n, 10_000.0),
            })
        })?,
    )?;
    // AbbreviateLargeNumbers: K threshold at 1000.
    g.set(
        "AbbreviateLargeNumbers",
        lua.create_function(|_, (value, _): (Value, Value)| {
            Ok(match super::to_abbrev_number(&value) {
                None => value.to_string().unwrap_or_else(|_| "0".into()),
                Some(n) => super::format_abbreviated(n, 1_000.0),
            })
        })?,
    )?;
    Ok(())
}
