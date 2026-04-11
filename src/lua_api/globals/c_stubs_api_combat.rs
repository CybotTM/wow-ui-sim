//! Combat, color, curve, and encounter-related C_* namespace stubs.
//!
//! Split from c_stubs_api_extra.rs to keep file sizes manageable.
//! Contains: C_ColorUtil, C_CurveUtil, C_RestrictedActions, C_TransmogOutfitInfo,
//! C_DamageMeter, C_CombatText, C_CombatAudioAlert, C_HousingPhotoSharing,
//! NamePlateConstants, C_DeathRecap.
//! C_CombatLog → c_stubs_api_combat_log.rs
//! C_EncounterTimeline + fixup_combat_log_aliases → c_stubs_api_encounter.rs

use mlua::{Lua, Result, Value};

/// Register all combat/encounter-related stubs.
pub fn register_combat_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    super::c_stubs_api_combat_curve::register_curve_support(lua)?;
    register_c_color_util(lua, &g)?;
    super::c_stubs_api_combat_log::register_c_combat_log(lua, &g)?;
    register_c_restricted_transmog(lua, &g)?;
    register_c_damage_meter(lua, &g)?;
    register_c_combat_text(lua, &g)?;
    register_c_combat_audio_alert(lua, &g)?;
    register_c_housing_photo_sharing(lua, &g)?;
    register_nameplate_constants(lua)?;
    register_c_death_recap(lua, &g)?;
    super::c_stubs_api_encounter::register_c_encounter_timeline(lua, &g)?;
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

const DAMAGE_METER_LUA: &str = r#"
    C_DamageMeter = C_DamageMeter or {}

    local api = C_DamageMeter
    local overallSessionType = Enum and Enum.DamageMeterSessionType and Enum.DamageMeterSessionType.Overall or 0
    local currentSessionType = Enum and Enum.DamageMeterSessionType and Enum.DamageMeterSessionType.Current or 1
    local expiredSessionType = Enum and Enum.DamageMeterSessionType and Enum.DamageMeterSessionType.Expired or 2
    local damageDoneType = Enum and Enum.DamageMeterType and Enum.DamageMeterType.DamageDone or 0
    local allyDisplayType = Enum and Enum.DamageMeterSourceDisplayType and Enum.DamageMeterSourceDisplayType.Ally or 1
    local enemyDisplayType = Enum and Enum.DamageMeterSourceDisplayType and Enum.DamageMeterSourceDisplayType.Enemy or 2

    local function deep_copy(value)
        if type(value) ~= "table" then
            return value
        end

        local copy = {}
        for key, nested in pairs(value) do
            copy[key] = deep_copy(nested)
        end
        return copy
    end

    local function create_spell(spellID, totalAmount, amountPerSecond, unitDetails)
        return {
            spellID = spellID,
            totalAmount = totalAmount,
            amountPerSecond = amountPerSecond,
            creatureName = "",
            overkillAmount = 0,
            isAvoidable = false,
            isDeadly = false,
            combatSpellDetails = unitDetails,
        }
    end

    local function create_source_source(combatSpells)
        local maxAmount = 0
        local totalAmount = 0
        for _, combatSpell in ipairs(combatSpells) do
            if combatSpell.totalAmount > maxAmount then
                maxAmount = combatSpell.totalAmount
            end
            totalAmount = totalAmount + combatSpell.totalAmount
        end

        return {
            combatSpells = combatSpells,
            maxAmount = maxAmount,
            totalAmount = totalAmount,
        }
    end

    local function create_combat_session(combatSources, durationSeconds)
        local maxAmount = 0
        local totalAmount = 0
        for _, combatSource in ipairs(combatSources) do
            if combatSource.totalAmount > maxAmount then
                maxAmount = combatSource.totalAmount
            end
            totalAmount = totalAmount + combatSource.totalAmount
        end

        return {
            combatSources = combatSources,
            maxAmount = maxAmount,
            totalAmount = totalAmount,
            durationSeconds = durationSeconds,
        }
    end

    local function create_default_state()
        local playerGuid = "Player-0001-00000001"
        local dummyGuid = "Creature-0-0000-00000-00000-31146-0000000001"
        local durationSeconds = 40

        local playerSpellDetails = {
            unitName = "Training Dummy",
            unitClassFilename = "",
            classification = "normal",
            isPet = false,
            isMob = true,
            amount = 52000,
            specIconID = 0,
        }
        local dummySpellDetails = {
            unitName = "Player",
            unitClassFilename = "PALADIN",
            classification = "normal",
            isPet = false,
            isMob = false,
            amount = 18000,
            specIconID = 0,
        }

        local playerSpells = {
            create_spell(19750, 52000, 1300, playerSpellDetails),
        }
        local dummySpells = {
            create_spell(6603, 18000, 450, dummySpellDetails),
        }

        local playerSource = {
            sourceGUID = playerGuid,
            sourceCreatureID = nil,
            name = "Player",
            classFilename = "PALADIN",
            specIconID = 0,
            totalAmount = 52000,
            amountPerSecond = 1300,
            isLocalPlayer = true,
            deathRecapID = 0,
            deathTimeSeconds = 0,
            classification = "normal",
            sourceDisplayType = allyDisplayType,
        }
        local dummySource = {
            sourceGUID = dummyGuid,
            sourceCreatureID = 31146,
            name = "Training Dummy",
            classFilename = "",
            specIconID = 0,
            totalAmount = 18000,
            amountPerSecond = 450,
            isLocalPlayer = false,
            deathRecapID = 0,
            deathTimeSeconds = 0,
            classification = "normal",
            sourceDisplayType = enemyDisplayType,
        }

        local overallSession = create_combat_session(
            {playerSource, dummySource},
            durationSeconds
        )
        local currentSession = create_combat_session(
            {deep_copy(playerSource), deep_copy(dummySource)},
            durationSeconds
        )

        return {
            availableSessions = {
                {
                    sessionID = 1,
                    name = "Training Dummy",
                    durationSeconds = durationSeconds,
                },
            },
            sessionsByID = {
                [1] = {
                    [damageDoneType] = overallSession,
                },
            },
            sessionsByType = {
                [overallSessionType] = {
                    [damageDoneType] = overallSession,
                },
                [currentSessionType] = {
                    [damageDoneType] = currentSession,
                },
                [expiredSessionType] = {},
            },
            sourcesByID = {
                [1] = {
                    [damageDoneType] = {
                        [playerGuid] = create_source_source(playerSpells),
                        [dummyGuid] = create_source_source(dummySpells),
                    },
                },
            },
            sourcesByType = {
                [overallSessionType] = {
                    [damageDoneType] = {
                        [playerGuid] = create_source_source(deep_copy(playerSpells)),
                        [dummyGuid] = create_source_source(deep_copy(dummySpells)),
                    },
                },
                [currentSessionType] = {
                    [damageDoneType] = {
                        [playerGuid] = create_source_source(deep_copy(playerSpells)),
                        [dummyGuid] = create_source_source(deep_copy(dummySpells)),
                    },
                },
                [expiredSessionType] = {},
            },
        }
    end

    local function current_state()
        if api._state == nil then
            api._state = create_default_state()
        end
        return api._state
    end

    local function lookup_session(bucket, sessionKey, damageMeterType)
        local sessionByType = bucket[sessionKey]
        if not sessionByType then
            return nil
        end
        return sessionByType[damageMeterType]
    end

    local function lookup_source(bucket, sessionKey, damageMeterType, sourceGUID, sourceCreatureID)
        local sourceByType = bucket[sessionKey]
        if not sourceByType then
            return nil
        end

        local sources = sourceByType[damageMeterType]
        if not sources then
            return nil
        end

        if sourceGUID and sources[sourceGUID] then
            return sources[sourceGUID]
        end

        if sourceCreatureID then
            for guid, source in pairs(sources) do
                local _ = guid
                if sourceCreatureID ~= nil and tostring(sourceCreatureID) ~= "" then
                    local session = current_state()
                    local overall = session.sessionsByType[overallSessionType]
                    local overallSession = overall and overall[damageMeterType]
                    if overallSession then
                        for _, combatSource in ipairs(overallSession.combatSources) do
                            if combatSource.sourceCreatureID == sourceCreatureID then
                                return source
                            end
                        end
                    end
                end
            end
        end

        return nil
    end

    function api.IsDamageMeterAvailable()
        return true, ""
    end

    function api.GetAvailableCombatSessions()
        return deep_copy(current_state().availableSessions)
    end

    function api.GetCombatSessionFromID(sessionID, damageMeterType)
        local session = lookup_session(current_state().sessionsByID, sessionID, damageMeterType)
        return session and deep_copy(session) or nil
    end

    function api.GetCombatSessionFromType(sessionType, damageMeterType)
        local session = lookup_session(current_state().sessionsByType, sessionType, damageMeterType)
        return session and deep_copy(session) or nil
    end

    function api.GetCombatSessionSourceFromID(sessionID, damageMeterType, sourceGUID, sourceCreatureID)
        local source = lookup_source(current_state().sourcesByID, sessionID, damageMeterType, sourceGUID, sourceCreatureID)
        return source and deep_copy(source) or nil
    end

    function api.GetCombatSessionSourceFromType(sessionType, damageMeterType, sourceGUID, sourceCreatureID)
        local source = lookup_source(current_state().sourcesByType, sessionType, damageMeterType, sourceGUID, sourceCreatureID)
        return source and deep_copy(source) or nil
    end

    function api.GetSessionDurationSeconds(sessionType)
        local session = api.GetCombatSessionFromType(sessionType, damageDoneType)
        return session and session.durationSeconds or nil
    end

    function api.ResetAllCombatSessions()
        api._state = create_default_state()
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
    toi.set("__activeOutfitID", 0i32)?;
    toi.set("__currentlyViewedOutfitID", 0i32)?;
    toi.set("__pendingSheatheCategories", lua.create_table()?)?;
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
    let transmog_state = toi.clone();
    toi.set(
        "GetActiveOutfitID",
        lua.create_function(move |_, ()| transmog_state.get::<i32>("__activeOutfitID"))?,
    )?;
    let transmog_state = toi.clone();
    toi.set(
        "GetCurrentlyViewedOutfitID",
        lua.create_function(move |_, ()| transmog_state.get::<i32>("__currentlyViewedOutfitID"))?,
    )?;
    toi.set(
        "GetAllTransmogOutfitOptionSheatheCategoryInfo",
        lua.create_function(build_sheathe_category_info)?,
    )?;
    let transmog_state = toi.clone();
    toi.set(
        "SetPendingTransmogSheatheCategory",
        lua.create_function(
            move |_, (slot, weapon_option, sheathe_category): (i32, i32, i32)| {
                set_pending_transmog_sheathe_category(
                    &transmog_state,
                    slot,
                    weapon_option,
                    sheathe_category,
                )
            },
        )?,
    )?;
    let transmog_state = toi.clone();
    toi.set(
        "ClearOutfit",
        lua.create_function(move |lua, ()| clear_transmog_outfit(lua, &transmog_state))?,
    )?;
    let transmog_state = toi.clone();
    toi.set(
        "ChangeToOutfit",
        lua.create_function(
            move |lua, (player_facing_outfit_index, allow_remove_outfit): (i32, bool)| {
                change_to_outfit(
                    lua,
                    &transmog_state,
                    player_facing_outfit_index,
                    allow_remove_outfit,
                )
            },
        )?,
    )?;
    g.set("C_TransmogOutfitInfo", toi)?;
    Ok(())
}

fn build_sheathe_category_info(lua: &Lua, ima_id: i32) -> Result<Value> {
    if ima_id <= 0 {
        return Ok(Value::Nil);
    }

    let categories = lua.create_table()?;
    for (index, (sheathe_category, category_name)) in
        [(0, "Default"), (1, "Back"), (2, "Side"), (3, "Hide")]
            .into_iter()
            .enumerate()
    {
        let category_info = lua.create_table()?;
        category_info.set("sheatheCategory", sheathe_category)?;
        category_info.set("categoryName", category_name)?;
        categories.set(index + 1, category_info)?;
    }
    Ok(Value::Table(categories))
}

fn set_pending_transmog_sheathe_category(
    transmog_state: &mlua::Table,
    slot: i32,
    weapon_option: i32,
    sheathe_category: i32,
) -> Result<()> {
    let pending_categories = transmog_state.get::<mlua::Table>("__pendingSheatheCategories")?;
    let key = pending_sheathe_category_key(slot, weapon_option);
    pending_categories.set(key, sheathe_category)?;
    Ok(())
}

fn clear_transmog_outfit(lua: &Lua, transmog_state: &mlua::Table) -> Result<()> {
    transmog_state.set("__activeOutfitID", 0i32)?;
    transmog_state.set("__currentlyViewedOutfitID", 0i32)?;
    transmog_state.set("__pendingSheatheCategories", lua.create_table()?)?;
    Ok(())
}

fn change_to_outfit(
    lua: &Lua,
    transmog_state: &mlua::Table,
    player_facing_outfit_index: i32,
    allow_remove_outfit: bool,
) -> Result<()> {
    let active_outfit_id = transmog_state.get::<i32>("__activeOutfitID")?;
    let should_clear = allow_remove_outfit && active_outfit_id == player_facing_outfit_index;
    if should_clear {
        return clear_transmog_outfit(lua, transmog_state);
    }

    transmog_state.set("__activeOutfitID", player_facing_outfit_index)?;
    transmog_state.set("__currentlyViewedOutfitID", player_facing_outfit_index)?;
    Ok(())
}

fn pending_sheathe_category_key(slot: i32, weapon_option: i32) -> String {
    format!("{slot}:{weapon_option}")
}

/// C_DamageMeter - damage/healing meter API.
fn register_c_damage_meter(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(DAMAGE_METER_LUA).exec()?;
    let damage_meter = g.get::<mlua::Table>("C_DamageMeter")?;
    g.set("C_DamageMeter", damage_meter)?;
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

