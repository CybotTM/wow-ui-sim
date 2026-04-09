//! Combat, color, curve, and encounter-related C_* namespace stubs.
//!
//! Split from c_stubs_api_extra.rs to keep file sizes manageable.
//! Contains: C_ColorUtil, C_CombatLog, C_CurveUtil, C_EncounterTimeline,
//! C_RestrictedActions, C_TransmogOutfitInfo, Constants.EncounterTimelineIconMasks.

use mlua::{Lua, MultiValue, Result, Value};

/// Register all combat/encounter-related stubs.
pub fn register_combat_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    super::c_stubs_api_combat_curve::register_curve_support(lua)?;
    register_c_color_util(lua, &g)?;
    register_c_combat_log(lua, &g)?;
    register_c_restricted_transmog(lua, &g)?;
    register_c_damage_meter(lua, &g)?;
    register_c_combat_text(lua, &g)?;
    register_c_combat_audio_alert(lua, &g)?;
    register_c_housing_photo_sharing(lua, &g)?;
    register_nameplate_constants(lua)?;
    register_c_death_recap(lua, &g)?;
    register_c_encounter_timeline(lua, &g)?;
    Ok(())
}

/// C_ColorUtil - hex color formatting for ColorMixin.
fn register_c_color_util(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let cu = lua.create_table()?;
    cu.set(
        "GenerateTextColorCode",
        lua.create_function(|_, color: mlua::Table| {
            let r: f64 = color.get("r").unwrap_or(1.0);
            let g: f64 = color.get("g").unwrap_or(1.0);
            let b: f64 = color.get("b").unwrap_or(1.0);
            let a: f64 = color.get("a").unwrap_or(1.0);
            Ok(format!(
                "{:02X}{:02X}{:02X}{:02X}",
                (a * 255.0) as u8,
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8
            ))
        })?,
    )?;
    cu.set(
        "WrapTextInColor",
        lua.create_function(|_, (text, color): (String, mlua::Table)| {
            let r: f64 = color.get("r").unwrap_or(1.0);
            let g: f64 = color.get("g").unwrap_or(1.0);
            let b: f64 = color.get("b").unwrap_or(1.0);
            let a: f64 = color.get("a").unwrap_or(1.0);
            let hex = format!(
                "{:02X}{:02X}{:02X}{:02X}",
                (a * 255.0) as u8,
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8
            );
            Ok(format!("|c{hex}{text}|r"))
        })?,
    )?;
    g.set("C_ColorUtil", cu)?;
    Ok(())
}

/// C_CombatLog - combat log API (relocated from global functions in modern WoW).
fn register_c_combat_log(lua: &Lua, g: &mlua::Table) -> Result<()> {
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

/// C_RestrictedActions, C_TransmogOutfitInfo stubs.
fn register_c_restricted_transmog(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let ra = lua.create_table()?;
    ra.set(
        "CheckAllowProtectedFunctions",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    g.set("C_RestrictedActions", ra)?;

    let toi = lua.create_table()?;
    toi.set(
        "GetOutfitInfoList",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    toi.set(
        "GetSlotSourceID",
        lua.create_function(|_, (_id, _slot): (Value, Value)| Ok(0i32))?,
    )?;
    toi.set(
        "GetAllSlotLocationInfo",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set("C_TransmogOutfitInfo", toi)?;
    Ok(())
}

/// C_DamageMeter - damage/healing meter API.
fn register_c_damage_meter(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "IsDamageMeterAvailable",
        lua.create_function(|_, ()| Ok((false, Value::Nil)))?,
    )?;
    t.set(
        "GetAvailableCombatSessions",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetCombatSessionFromID",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetCombatSessionFromType",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetCombatSessionSourceFromID",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetCombatSessionSourceFromType",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetSessionDurationSeconds",
        lua.create_function(|_, _st: Value| Ok(0.0f64))?,
    )?;
    t.set(
        "ResetAllCombatSessions",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set("C_DamageMeter", t)?;
    Ok(())
}

/// C_CombatText - combat floating text API.
fn register_c_combat_text(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetCurrentEventInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "SetActiveUnit",
        lua.create_function(|_, _unit: Value| Ok(()))?,
    )?;
    g.set("C_CombatText", t)?;
    Ok(())
}

/// C_CombatAudioAlert - combat audio alert system.
fn register_c_combat_audio_alert(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetCategoryVoice",
        lua.create_function(|_, _cat: Value| Ok(0i32))?,
    )?;
    t.set(
        "GetCategoryVolume",
        lua.create_function(|_, _cat: Value| Ok(1.0f64))?,
    )?;
    t.set(
        "GetFormatSetting",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(0i32))?,
    )?;
    t.set("GetSpeakerSpeed", lua.create_function(|_, ()| Ok(1.0f64))?)?;
    t.set(
        "GetSpecSetting",
        lua.create_function(|_, _s: Value| Ok(0i32))?,
    )?;
    t.set(
        "GetThrottle",
        lua.create_function(|_, _t: Value| Ok(0.0f64))?,
    )?;
    t.set(
        "SetCategoryVoice",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?,
    )?;
    t.set(
        "SetCategoryVolume",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?,
    )?;
    t.set(
        "SetFormatSetting",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?,
    )?;
    t.set(
        "SetSpeakerSpeed",
        lua.create_function(|_, _s: Value| Ok(()))?,
    )?;
    t.set(
        "SetSpecSetting",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?,
    )?;
    t.set(
        "SetThrottle",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?,
    )?;
    t.set("SpeakText", lua.create_function(|_, _text: Value| Ok(()))?)?;
    g.set("C_CombatAudioAlert", t)?;
    Ok(())
}

/// C_HousingPhotoSharing - housing screenshot sharing.
fn register_c_housing_photo_sharing(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsAuthorized", lua.create_function(|_, ()| Ok(true))?)?;
    t.set(
        "BeginAuthorizationFlow",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    t.set("ClearAuthorization", lua.create_function(|_, ()| Ok(()))?)?;
    t.set(
        "CompleteAuthorizationFlow",
        lua.create_function(|_, _url: Value| Ok(()))?,
    )?;
    t.set("GetCropRatio", lua.create_function(|_, ()| Ok(1.0f64))?)?;
    t.set(
        "GetPhotoSharingAuthURL",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "SetScreenshotPreviewTexture",
        lua.create_function(|_, _tex: Value| Ok(()))?,
    )?;
    t.set(
        "UploadPhotoToService",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?,
    )?;
    g.set("C_HousingPhotoSharing", t)?;
    Ok(())
}

/// Build the NamePlateConstants string cvar fields sub-table.
fn nameplate_cvar_fields(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.raw_set("INFO_DISPLAY_CVAR", "nameplateInfoDisplay")?;
    t.raw_set("CAST_BAR_DISPLAY_CVAR", "nameplateCastBarDisplay")?;
    t.raw_set("THREAT_DISPLAY_CVAR", "nameplateThreatDisplay")?;
    t.raw_set(
        "ENEMY_NPC_AURA_DISPLAY_CVAR",
        "nameplateEnemyNpcAuraDisplay",
    )?;
    t.raw_set(
        "ENEMY_PLAYER_AURA_DISPLAY_CVAR",
        "nameplateEnemyPlayerAuraDisplay",
    )?;
    t.raw_set(
        "FRIENDLY_PLAYER_AURA_DISPLAY_CVAR",
        "nameplateFriendlyPlayerAuraDisplay",
    )?;
    t.raw_set(
        "SHOW_DEBUFFS_ON_FRIENDLY_CVAR",
        "nameplateShowDebuffsOnFriendly",
    )?;
    t.raw_set("DEBUFF_PADDING_CVAR", "nameplateDebuffPadding")?;
    t.raw_set("AURA_SCALE_CVAR", "nameplateAuraScale")?;
    t.raw_set("SIZE_CVAR", "nameplateSize")?;
    t.raw_set("STYLE_CVAR", "nameplateStyle")?;
    t.raw_set("SIMPLIFIED_TYPES_CVAR", "nameplateSimplifiedTypes")?;
    t.raw_set("SOFT_TARGET_NAMEPLATE_SIZE_CVAR", "SoftTargetNameplateSize")?;
    t.raw_set("SOFT_TARGET_ICON_ENEMY_CVAR", "SoftTargetIconEnemy")?;
    t.raw_set("SOFT_TARGET_ICON_FRIEND_CVAR", "SoftTargetIconFriend")?;
    t.raw_set("SOFT_TARGET_ICON_INTERACT_CVAR", "SoftTargetIconInteract")?;
    t.raw_set("SHOW_FRIENDLY_NPCS_CVAR", "nameplateShowFriendlyNpcs")?;
    t.raw_set(
        "SHOW_ONLY_NAME_FOR_FRIENDLY_PLAYER_UNITS_CVAR",
        "nameplateShowOnlyNameForFriendlyPlayerUnits",
    )?;
    t.raw_set(
        "USE_CLASS_COLOR_FOR_FRIENDLY_PLAYER_UNIT_NAMES_CVAR",
        "nameplateUseClassColorForFriendlyPlayerUnitNames",
    )?;
    t.raw_set("PREVIEW_UNIT_TOKEN", "preview")?;
    Ok(t)
}

/// Build the NamePlateConstants numeric fields sub-table.
fn nameplate_numeric_fields(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.raw_set("AURA_ITEM_HEIGHT", 25_i32)?;
    t.raw_set("LARGE_HEALTH_BAR_HEIGHT", 20_i32)?;
    t.raw_set("SMALL_HEALTH_BAR_HEIGHT", 10_i32)?;
    t.raw_set("HEALTH_BAR_FONT_HEIGHT", 12_i32)?;
    t.raw_set("LARGE_CAST_BAR_HEIGHT", 16_i32)?;
    t.raw_set("SMALL_CAST_BAR_HEIGHT", 10_i32)?;
    t.raw_set("CAST_BAR_FONT_HEIGHT", 10_i32)?;
    t.raw_set("CAST_BAR_ICON_HEIGHT", 12_i32)?;
    let scales = lua.create_table()?;
    for (i, v) in [0.75f64, 1.0, 1.25, 1.5, 2.0].iter().enumerate() {
        scales.raw_set(i as i32 + 1, *v)?;
    }
    t.raw_set("NAME_PLATE_SCALES", scales)?;
    Ok(t)
}

/// NamePlateConstants - global constant table for nameplate system.
fn register_nameplate_constants(lua: &Lua) -> Result<()> {
    let t = nameplate_cvar_fields(lua)?;
    for pair in nameplate_numeric_fields(lua)?.pairs::<String, Value>() {
        let (k, v) = pair?;
        t.raw_set(k, v)?;
    }
    lua.globals().set("NamePlateConstants", t)?;
    Ok(())
}

/// C_DeathRecap - death recap data.
fn register_c_death_recap(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("HasRecapEvents", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetRecapEvents",
        lua.create_function(|lua, _id: Value| lua.create_table())?,
    )?;
    t.set(
        "GetRecapMaxHealth",
        lua.create_function(|_, _id: Value| Ok(0i32))?,
    )?;
    t.set(
        "GetRecapLink",
        lua.create_function(|_, _id: Value| Ok(Value::Nil))?,
    )?;
    g.set("C_DeathRecap", t)?;
    Ok(())
}

/// C_EncounterTimeline - encounter timeline UI data (boss ability timers).
fn register_c_encounter_timeline(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(ENCOUNTER_TIMELINE_LUA).exec()?;
    g.get::<mlua::Table>("C_EncounterTimeline")
        .and_then(|timeline| g.set("C_EncounterTimeline", timeline))?;
    Ok(())
}

const ENCOUNTER_TIMELINE_LUA: &str = r#"
    C_EncounterTimeline = C_EncounterTimeline or {}
    local api = C_EncounterTimeline

    local function make_color(r, g, b, a)
        local color = { r = r, g = g, b = b, a = a or 1.0 }
        function color:GetRGB()
            return self.r, self.g, self.b
        end
        function color:GetRGBA()
            return self.r, self.g, self.b, self.a
        end
        return color
    end

    local function create_duration(total_duration, elapsed_duration)
        local duration = {
            totalDuration = total_duration,
            elapsedDuration = elapsed_duration,
        }

        function duration:GetElapsedDuration()
            return self.elapsedDuration
        end

        function duration:GetRemainingDuration()
            return math.max(0, self.totalDuration - self.elapsedDuration)
        end

        function duration:GetTotalDuration()
            return self.totalDuration
        end

        function duration:GetElapsedPercent()
            if self.totalDuration <= 0 then
                return 0
            end
            return self:GetElapsedDuration() / self.totalDuration
        end

        function duration:GetRemainingPercent()
            if self.totalDuration <= 0 then
                return 0
            end
            return self:GetRemainingDuration() / self.totalDuration
        end

        function duration:GetStartTime()
            return 0
        end

        function duration:GetEndTime()
            return self.totalDuration
        end

        return duration
    end

    local function default_track_list()
        return {
            {
                id = Enum.EncounterTimelineTrack.Queued,
                type = Enum.EncounterTimelineTrackType.Sorted,
                minimumDuration = 0,
                maximumDuration = 0,
                maximumEventCount = 3,
            },
            {
                id = Enum.EncounterTimelineTrack.Short,
                type = Enum.EncounterTimelineTrackType.Linear,
                minimumDuration = 0,
                maximumDuration = 10,
            },
            {
                id = Enum.EncounterTimelineTrack.Medium,
                type = Enum.EncounterTimelineTrackType.Linear,
                minimumDuration = 10,
                maximumDuration = 30,
            },
            {
                id = Enum.EncounterTimelineTrack.Long,
                type = Enum.EncounterTimelineTrackType.Sorted,
                minimumDuration = 30,
                maximumDuration = 120,
                maximumEventCount = 3,
            },
        }
    end

    local function copy_track_list(track_list)
        local copy = {}
        for index, track in ipairs(track_list) do
            local track_copy = {}
            for key, value in pairs(track) do
                track_copy[key] = value
            end
            copy[index] = track_copy
        end
        return copy
    end

    local function fire_event(event_name, ...)
        if type(FireEvent) == "function" then
            pcall(FireEvent, event_name, ...)
        end
    end

    api._state = api._state or {}
    local state = api._state

    state.featureAvailable = state.featureAvailable ~= false
    state.featureEnabled = state.featureEnabled ~= false
    state.currentTime = tonumber(state.currentTime) or 2.0
    state.highlightTime = tonumber(state.highlightTime) or 5.0
    state.viewType = tonumber(state.viewType) or Enum.EncounterTimelineViewType.Timeline
    state.trackList = type(state.trackList) == "table" and state.trackList or default_track_list()
    state.eventList = type(state.eventList) == "table" and state.eventList or { 1 }
    state.eventInfo = type(state.eventInfo) == "table" and state.eventInfo or {
        [1] = {
            id = 1,
            spellID = 19750,
            spellName = "Flash of Light",
            iconFileID = "Interface\\Icons\\Spell_Holy_FlashHeal",
            icons = 0,
            severity = Enum.EncounterEventSeverity.Medium,
            color = make_color(1.0, 0.82, 0.0, 1.0),
        },
    }
    state.eventState = type(state.eventState) == "table" and state.eventState or {
        [1] = Enum.EncounterTimelineEventState.Active,
    }
    state.eventTrack = type(state.eventTrack) == "table" and state.eventTrack or {
        [1] = { Enum.EncounterTimelineTrack.Short, 1 },
    }
    state.eventBlocked = type(state.eventBlocked) == "table" and state.eventBlocked or {
        [1] = false,
    }
    state.eventTimers = type(state.eventTimers) == "table" and state.eventTimers or {
        [1] = create_duration(12.0, state.currentTime),
    }

    local function get_track_info(track_id)
        for _, track in ipairs(state.trackList) do
            if track.id == track_id then
                return track
            end
        end
        return nil
    end

    local function is_terminal_state(event_state)
        return event_state == Enum.EncounterTimelineEventState.Finished
            or event_state == Enum.EncounterTimelineEventState.Canceled
    end

    local function has_visible_event()
        for _, event_id in ipairs(state.eventList) do
            local event_state = state.eventState[event_id]
            local track_data = get_track_info((state.eventTrack[event_id] or {})[1])
            if not is_terminal_state(event_state)
                and track_data
                and track_data.type ~= Enum.EncounterTimelineTrackType.Hidden
            then
                return true
            end
        end
        return false
    end

    function api.IsFeatureAvailable()
        return state.featureAvailable
    end

    function api.IsFeatureEnabled()
        return state.featureEnabled
    end

    function api.GetEventList()
        local event_list = {}
        for index, event_id in ipairs(state.eventList) do
            event_list[index] = event_id
        end
        return event_list
    end

    function api.GetEventInfo(event_id)
        return state.eventInfo[event_id]
    end

    function api.GetEventState(event_id)
        return state.eventState[event_id]
    end

    function api.GetEventTimer(event_id)
        return state.eventTimers[event_id]
    end

    function api.GetEventTrack(event_id)
        local track = state.eventTrack[event_id]
        if not track then
            return Enum.EncounterTimelineTrack.Indeterminate, 0
        end
        return track[1], track[2]
    end

    function api.GetEventHighlightTime()
        return state.highlightTime
    end

    function api.GetEventTimeRemaining(event_id)
        local timer = api.GetEventTimer(event_id)
        return timer and timer:GetRemainingDuration() or nil
    end

    function api.IsEventBlocked(event_id)
        return state.eventBlocked[event_id] == true
    end

    function api.HasActiveEvents()
        for _, event_id in ipairs(state.eventList) do
            if state.eventState[event_id] == Enum.EncounterTimelineEventState.Active then
                return true
            end
        end
        return false
    end

    function api.HasPausedEvents()
        for _, event_id in ipairs(state.eventList) do
            if state.eventState[event_id] == Enum.EncounterTimelineEventState.Paused then
                return true
            end
        end
        return false
    end

    function api.HasVisibleEvents()
        return has_visible_event()
    end

    function api.GetTrackList()
        return copy_track_list(state.trackList)
    end

    function api.GetTrackType(track_id)
        local track = get_track_info(track_id)
        if not track then
            return Enum.EncounterTimelineTrackType.Hidden
        end
        return track.type
    end

    function api.GetViewType()
        return state.viewType
    end

    function api.SetViewType(view_type)
        local next_view_type = tonumber(view_type) or Enum.EncounterTimelineViewType.None
        local previous_view_type = state.viewType
        if previous_view_type == next_view_type then
            return
        end

        if previous_view_type and previous_view_type ~= Enum.EncounterTimelineViewType.None then
            fire_event("ENCOUNTER_TIMELINE_VIEW_DEACTIVATED", previous_view_type)
        end

        state.viewType = next_view_type

        if next_view_type ~= Enum.EncounterTimelineViewType.None then
            fire_event("ENCOUNTER_TIMELINE_VIEW_ACTIVATED", next_view_type)
        end
    end

    function api.GetCurrentTime()
        return state.currentTime
    end

    function api.AddEditModeEvents()
        return 30.0
    end

    function api.CancelEditModeEvents()
    end

    function api.SetEventIconTextures(event_id, icon_mask, textures)
        local event_info = api.GetEventInfo(event_id)
        local should_show = type(event_info) == "table"
            and bit.band(event_info.icons or 0, icon_mask or 0) ~= 0

        if type(textures) ~= "table" then
            return
        end

        for _, texture in ipairs(textures) do
            if texture and texture.SetShown then
                texture:SetShown(should_show)
            end
            if should_show and texture and texture.SetTexture then
                texture:SetTexture("Interface\\RaidFrame\\ReadyCheck-Ready")
            end
        end
    end
"#;

/// Global-to-namespace alias pairs: (global_name, C_CombatLog method name).
const COMBAT_LOG_ALIASES: &[(&str, &str)] = &[
    ("CombatLogAddFilter", "AddEventFilter"),
    ("CombatLogGetCurrentEntry", "GetCurrentEntryInfo"),
    ("CombatLogGetCurrentEventInfo", "GetCurrentEventInfo"),
    ("CombatLogGetNumEntries", "GetEntryCount"),
    ("CombatLogAdvanceEntry", "AdvanceEntry"),
    ("CombatLogSetCurrentEntry", "SetCurrentEntry"),
    ("CombatLogShowCurrentEntry", "ShowCurrentEntry"),
    ("CombatLogResetFilter", "ResetFilter"),
    ("CombatLogClearEntries", "ClearEntries"),
    ("CombatLogGetRetentionTime", "GetRetentionTime"),
    ("CombatLogSetRetentionTime", "SetRetentionTime"),
];

/// Re-alias CombatLog* globals to the same function objects stored in C_CombatLog.
///
/// Wowless's cfuncs uniqueChecker requires that alias pairs (e.g. "C_CombatLog.AddEventFilter"
/// and "CombatLogAddFilter") share the same underlying C function pointer. This function
/// overwrites separately-created global stubs with direct references to the namespace functions.
/// Must be called after all CombatLog registrations complete.
pub fn fixup_combat_log_aliases(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let Ok(cl) = g.get::<mlua::Table>("C_CombatLog") else {
        return Ok(());
    };
    apply_aliases(lua, g, &cl, COMBAT_LOG_ALIASES)
}

/// Apply alias pairs from a namespace table to globals, with fallback no-op on missing methods.
fn apply_aliases(
    lua: &Lua,
    g: &mlua::Table,
    ns: &mlua::Table,
    pairs: &[(&str, &str)],
) -> Result<()> {
    for &(global_name, method_name) in pairs {
        let f = match ns.get::<mlua::Function>(method_name) {
            Ok(f) => f,
            Err(_) => lua.create_function(|_, _: MultiValue| Ok(()))?,
        };
        g.set(global_name, f)?;
    }
    Ok(())
}
