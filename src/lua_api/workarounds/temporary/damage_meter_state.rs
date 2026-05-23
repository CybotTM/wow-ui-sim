//! Temporary `C_DamageMeter` seeded state surface.
//!
//! Damage meter data is currently a startup fixture used by Blizzard data
//! providers. Keep it explicit until combat-session data is backed by a real
//! simulator subsystem.

const DAMAGE_METER_STATE_LUA: &str = r#"
if type(C_DamageMeter) ~= "table" then
    C_DamageMeter = {}
end

local function DamageMeterSpellDetails(unitName, unitClassFilename, amount)
    return {
        amount = amount or 0,
        classification = "normal",
        isMob = false,
        isPet = false,
        specIconID = 0,
        unitClassFilename = unitClassFilename or "PALADIN",
        unitName = unitName or "Player",
    }
end

local function DamageMeterSpell(spellID, amount, amountPerSecond, unitName, unitClassFilename)
    return {
        spellID = spellID,
        totalAmount = amount or 0,
        amountPerSecond = amountPerSecond or 0,
        creatureName = unitName or "Player",
        overkillAmount = 0,
        isAvoidable = false,
        isDeadly = false,
        combatSpellDetails = DamageMeterSpellDetails(unitName, unitClassFilename, amount),
    }
end

local seededSource = {
    name = "Player",
    isLocalPlayer = true,
    sourceGUID = "Player-1-00000001",
    sourceCreatureID = 1,
    totalAmount = 52000,
    maxAmount = 52000,
    amountPerSecond = 1300,
    classFilename = "PALADIN",
    classification = "normal",
    deathRecapID = 0,
    deathTimeSeconds = 0,
    specIconID = 0,
    combatSpells = {
        DamageMeterSpell(19750, 52000, 1300, "Player", "PALADIN"),
    },
}

local seededSession = {
    sessionID = 1,
    totalAmount = 52000,
    maxAmount = 52000,
    durationSeconds = 40,
    combatSources = {
        seededSource,
        {
            name = "Companion",
            isLocalPlayer = false,
            sourceGUID = "Creature-1-00000002",
            sourceCreatureID = 2,
            totalAmount = 3333,
            maxAmount = 3333,
            amountPerSecond = 83.325,
            classFilename = "WARRIOR",
            classification = "normal",
            deathRecapID = 0,
            deathTimeSeconds = 0,
            specIconID = 0,
            combatSpells = {
                DamageMeterSpell(1337, 3333, 83.325, "Companion", "WARRIOR"),
            },
        },
    },
}

local function EmptyDamageMeterSession(sessionID)
    return {
        sessionID = sessionID,
        totalAmount = 0,
        maxAmount = 0,
        durationSeconds = 0,
        combatSources = {},
    }
end

local function EmptyDamageMeterSource()
    return {
        totalAmount = 0,
        maxAmount = 0,
        amountPerSecond = 0,
        combatSpells = {},
    }
end

local function IsKnownDamageMeterType(damageType)
    if type(Enum) ~= "table" or type(Enum.DamageMeterType) ~= "table" then
        return false
    end
    for _, knownType in pairs(Enum.DamageMeterType) do
        if damageType == knownType then
            return true
        end
    end
    return false
end

local function IsSeededDamageMeterSessionID(sessionID)
    return sessionID == 0 or sessionID == seededSession.sessionID
end

local function HasSeededDamageMeterSessionType(sessionType)
    return sessionType == Enum.DamageMeterSessionType.Overall
        or sessionType == Enum.DamageMeterSessionType.Current
end

if rawget(C_DamageMeter, "IsDamageMeterAvailable") == nil then
    function C_DamageMeter.IsDamageMeterAvailable()
        return true, nil
    end
end

if rawget(C_DamageMeter, "GetAvailableCombatSessions") == nil then
    function C_DamageMeter.GetAvailableCombatSessions()
        return { { sessionID = 1 } }
    end
end

if rawget(C_DamageMeter, "GetCurrentCombatSessionID") == nil then
    function C_DamageMeter.GetCurrentCombatSessionID()
        return 1
    end
end

if rawget(C_DamageMeter, "GetDamageMeterEntries") == nil then
    function C_DamageMeter.GetDamageMeterEntries()
        return {}
    end
end

if rawget(C_DamageMeter, "GetCombatSessionFromType") == nil then
    function C_DamageMeter.GetCombatSessionFromType(sessionType, damageType)
        if HasSeededDamageMeterSessionType(sessionType) and damageType == Enum.DamageMeterType.DamageDone then
            return seededSession
        end
        if HasSeededDamageMeterSessionType(sessionType) and IsKnownDamageMeterType(damageType) then
            return EmptyDamageMeterSession(seededSession.sessionID)
        end
        return nil
    end
end

if rawget(C_DamageMeter, "GetCombatSessionSourceFromType") == nil then
    function C_DamageMeter.GetCombatSessionSourceFromType(sessionType, damageType, sourceGUID, sourceCreatureID)
        if not HasSeededDamageMeterSessionType(sessionType) then
            return nil
        end
        if damageType ~= Enum.DamageMeterType.DamageDone then
            if IsKnownDamageMeterType(damageType) then
                return EmptyDamageMeterSource()
            end
            return nil
        end
        if sourceGUID ~= seededSource.sourceGUID then
            return nil
        end
        if sourceCreatureID ~= nil and sourceCreatureID ~= seededSource.sourceCreatureID then
            return nil
        end
        return seededSource
    end
end

if rawget(C_DamageMeter, "GetCombatSessionFromID") == nil then
    function C_DamageMeter.GetCombatSessionFromID(sessionID, damageType)
        if not IsSeededDamageMeterSessionID(sessionID) then
            return nil
        end
        if damageType ~= Enum.DamageMeterType.DamageDone then
            if IsKnownDamageMeterType(damageType) then
                return EmptyDamageMeterSession(sessionID)
            end
            return nil
        end
        return seededSession
    end
end

if rawget(C_DamageMeter, "GetCombatSessionSourceFromID") == nil then
    function C_DamageMeter.GetCombatSessionSourceFromID(sessionID, damageType, sourceGUID, sourceCreatureID)
        if not IsSeededDamageMeterSessionID(sessionID) then
            return nil
        end
        if damageType ~= Enum.DamageMeterType.DamageDone then
            if IsKnownDamageMeterType(damageType) then
                return EmptyDamageMeterSource()
            end
            return nil
        end
        if sourceGUID ~= seededSource.sourceGUID then
            return nil
        end
        if sourceCreatureID ~= nil and sourceCreatureID ~= seededSource.sourceCreatureID then
            return nil
        end
        return seededSource
    end
end

if rawget(C_DamageMeter, "GetSessionDurationSeconds") == nil then
    function C_DamageMeter.GetSessionDurationSeconds(sessionType, sessionID)
        if HasSeededDamageMeterSessionType(sessionType) or IsSeededDamageMeterSessionID(sessionID) then
            return seededSession.durationSeconds
        end
        return 0
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(DAMAGE_METER_STATE_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_seeded_damage_meter_sessions() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local available, unavailableReason = C_DamageMeter.IsDamageMeterAvailable()
                if not available or unavailableReason ~= nil then
                    return "unavailable"
                end
                if C_DamageMeter.GetAvailableCombatSessions()[1].sessionID ~= 1 then
                    return "bad_available_session"
                end
                if C_DamageMeter.GetCurrentCombatSessionID() ~= 1 then
                    return "bad_current_session"
                end
                if #C_DamageMeter.GetDamageMeterEntries() ~= 0 then
                    return "bad_entries"
                end
                local session = C_DamageMeter.GetCombatSessionFromType(
                    Enum.DamageMeterSessionType.Current,
                    Enum.DamageMeterType.DamageDone
                )
                if session.sessionID ~= 1 or session.totalAmount ~= 52000 or #session.combatSources ~= 2 then
                    return "bad_session_from_type"
                end
                local source = C_DamageMeter.GetCombatSessionSourceFromType(
                    Enum.DamageMeterSessionType.Current,
                    Enum.DamageMeterType.DamageDone,
                    "Player-1-00000001",
                    1
                )
                if source.name ~= "Player" or source.combatSpells[1].spellID ~= 19750 then
                    return "bad_source_from_type"
                end
                if C_DamageMeter.GetCombatSessionSourceFromType(
                    Enum.DamageMeterSessionType.Current,
                    Enum.DamageMeterType.DamageDone,
                    "missing",
                    nil
                ) ~= nil then
                    return "bad_missing_source"
                end
                local emptySession = C_DamageMeter.GetCombatSessionFromID(1, Enum.DamageMeterType.HealingDone)
                if emptySession.sessionID ~= 1 or emptySession.totalAmount ~= 0 or #emptySession.combatSources ~= 0 then
                    return "bad_empty_session"
                end
                local emptySource = C_DamageMeter.GetCombatSessionSourceFromID(1, Enum.DamageMeterType.HealingDone, nil, nil)
                if emptySource.totalAmount ~= 0 or #emptySource.combatSpells ~= 0 then
                    return "bad_empty_source"
                end
                if C_DamageMeter.GetCombatSessionFromID(99, Enum.DamageMeterType.DamageDone) ~= nil then
                    return "bad_missing_session"
                end
                if C_DamageMeter.GetSessionDurationSeconds(Enum.DamageMeterSessionType.Overall, nil) ~= 40 then
                    return "bad_duration_type"
                end
                if C_DamageMeter.GetSessionDurationSeconds(nil, 1) ~= 40 then
                    return "bad_duration_id"
                end
                if C_DamageMeter.GetSessionDurationSeconds(nil, 99) ~= 0 then
                    return "bad_missing_duration"
                end
                return "ok"
                "#,
            )
            .expect("damage meter probe should run");

        assert_eq!(result, "ok");
    }
}
