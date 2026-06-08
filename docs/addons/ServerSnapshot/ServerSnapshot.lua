local addonName = ...

local addon = {}
_G.ServerSnapshot = addon

local function ensureDatabase()
    if type(ServerSnapshotDB) ~= "table" then
        ServerSnapshotDB = {}
    end
    if type(ServerSnapshotDB.characters) ~= "table" then
        ServerSnapshotDB.characters = {}
    end
    if type(ServerSnapshotDB.history) ~= "table" then
        ServerSnapshotDB.history = {}
    end
    return ServerSnapshotDB
end

local function call(fn, ...)
    if type(fn) ~= "function" then
        return false
    end
    return pcall(fn, ...)
end

local function first(fn, ...)
    local ok, value = call(fn, ...)
    if ok then
        return value
    end
    return nil
end

local function getTime()
    return first(time) or 0
end

local function getCharacterKey()
    local name = first(UnitName, "player") or "Unknown"
    local realm = first(GetRealmName) or "UnknownRealm"
    return realm .. "/" .. name
end

local function getPlayerGuid()
    return first(UnitGUID, "player")
end

local function getBuildInfo()
    local ok, version, build, date, interface = call(GetBuildInfo)
    if not ok then
        return {}
    end
    return {
        version = version,
        build = build,
        date = date,
        interface = interface,
    }
end

local function getClassInfo()
    local ok, localized, english, classID = call(UnitClass, "player")
    if not ok then
        return {}
    end
    return {
        localized = localized,
        english = english,
        classID = classID,
    }
end

local function getSpecInfo()
    local specIndex = first(GetSpecialization)
    if not specIndex then
        return nil
    end

    local ok, id, name, description, icon, role, primaryStat = call(GetSpecializationInfo, specIndex)
    if not ok then
        return {
            index = specIndex,
        }
    end

    return {
        index = specIndex,
        id = id,
        name = name,
        description = description,
        icon = icon,
        role = role,
        primaryStat = primaryStat,
    }
end

local function actionCooldown(slot)
    local ok, start, duration, enabled, modRate = call(GetActionCooldown, slot)
    if not ok then
        return nil
    end
    return {
        start = start,
        duration = duration,
        enabled = enabled,
        modRate = modRate,
    }
end

local function actionCharges(slot)
    local ok, current, max, cooldownStart, cooldownDuration, chargeModRate = call(GetActionCharges, slot)
    if not ok then
        return nil
    end
    return {
        current = current,
        max = max,
        cooldownStart = cooldownStart,
        cooldownDuration = cooldownDuration,
        chargeModRate = chargeModRate,
    }
end

local function actionUsable(slot)
    local ok, usable, noMana = call(IsUsableAction, slot)
    if not ok then
        return nil
    end
    return {
        usable = usable,
        noMana = noMana,
    }
end

local function snapshotActionSlot(slot)
    local ok, actionType, id, subType, spellID = call(GetActionInfo, slot)
    if not ok then
        return {
            slot = slot,
            error = "GetActionInfo failed",
        }
    end

    if actionType == nil then
        return {
            slot = slot,
            empty = true,
        }
    end

    return {
        slot = slot,
        type = actionType,
        id = id,
        subType = subType,
        spellID = spellID,
        text = first(GetActionText, slot),
        texture = first(GetActionTexture, slot),
        count = first(GetActionCount, slot),
        cooldown = actionCooldown(slot),
        charges = actionCharges(slot),
        usable = actionUsable(slot),
        isAttack = first(IsAttackAction, slot),
        isCurrent = first(IsCurrentAction, slot),
        isEquipped = first(IsEquippedAction, slot),
        isPassive = first(IsActionPassive, slot),
    }
end

local function snapshotActionBars()
    local maxSlots = MAX_ACTIONBAR_ACTIONS or 180
    local slots = {}

    for slot = 1, maxSlots do
        slots[slot] = snapshotActionSlot(slot)
    end

    return {
        maxSlots = maxSlots,
        slots = slots,
    }
end

local function spellBookItem(index, bookType)
    local ok, itemType, id = call(GetSpellBookItemInfo, index, bookType)
    if not ok then
        return nil
    end

    local name, subName = first(GetSpellBookItemName, index, bookType)
    local spellID = nil
    local okSpellID, resolvedSpellID = call(GetSpellBookItemSpellID, index, bookType)
    if okSpellID then
        spellID = resolvedSpellID
    end

    return {
        index = index,
        bookType = bookType,
        type = itemType,
        id = id,
        spellID = spellID,
        name = name,
        subName = subName,
        texture = first(GetSpellBookItemTexture, index, bookType),
    }
end

local function snapshotLegacySpellBook()
    local tabs = {}
    local numTabs = first(GetNumSpellTabs)
    if type(numTabs) ~= "number" then
        return tabs
    end

    for tabIndex = 1, numTabs do
        local ok, name, texture, offset, numSlots, isGuild, offSpecID = call(GetSpellTabInfo, tabIndex)
        if ok then
            local tab = {
                index = tabIndex,
                name = name,
                texture = texture,
                offset = offset,
                numSlots = numSlots,
                isGuild = isGuild,
                offSpecID = offSpecID,
                spells = {},
            }

            for localIndex = 1, (numSlots or 0) do
                local spell = spellBookItem((offset or 0) + localIndex, BOOKTYPE_SPELL or "spell")
                if spell then
                    tab.spells[localIndex] = spell
                end
            end

            tabs[tabIndex] = tab
        end
    end

    return tabs
end

local function snapshotCSpellBook()
    local cSpellBook = C_SpellBook
    if type(cSpellBook) ~= "table" then
        return nil
    end

    local result = {}

    if type(cSpellBook.GetSpellBookSkillLineInfo) == "function" and type(cSpellBook.GetNumSpellBookSkillLines) == "function" then
        local okNum, numLines = call(cSpellBook.GetNumSpellBookSkillLines)
        if okNum and type(numLines) == "number" then
            result.skillLines = {}
            for lineIndex = 1, numLines do
                local okLine, lineInfo = call(cSpellBook.GetSpellBookSkillLineInfo, lineIndex)
                if okLine then
                    result.skillLines[lineIndex] = lineInfo
                end
            end
        end
    end

    if type(cSpellBook.GetSpellBookItemInfo) == "function" then
        result.note = "C_SpellBook exists; legacy spellbook snapshot is stored separately when available."
    end

    return result
end

local function snapshotSpellBook()
    return {
        legacy = snapshotLegacySpellBook(),
        cSpellBook = snapshotCSpellBook(),
    }
end

local function snapshotMacros()
    local ok, globalCount, characterCount = call(GetNumMacros)
    if not ok then
        return nil
    end

    local macros = {
        globalCount = globalCount,
        characterCount = characterCount,
        entries = {},
    }

    local total = (globalCount or 0) + (characterCount or 0)
    for index = 1, total do
        local okInfo, name, icon, body, isLocal = call(GetMacroInfo, index)
        if okInfo and name then
            macros.entries[index] = {
                index = index,
                name = name,
                icon = icon,
                body = body,
                isLocal = isLocal,
            }
        end
    end

    return macros
end

local function snapshotTalentConfig()
    local classTalents = C_ClassTalents
    if type(classTalents) ~= "table" then
        return nil
    end

    local talents = {}

    if type(classTalents.GetActiveConfigID) == "function" then
        talents.activeConfigID = first(classTalents.GetActiveConfigID)
    end

    if type(classTalents.GetLastSelectedSavedConfigID) == "function" then
        talents.lastSelectedSavedConfigID = first(classTalents.GetLastSelectedSavedConfigID)
    end

    if type(classTalents.GetConfigIDsBySpecID) == "function" then
        local spec = getSpecInfo()
        local specID = spec and spec.id
        if specID then
            talents.configIDsBySpecID = first(classTalents.GetConfigIDsBySpecID, specID)
        end
    end

    if type(classTalents.GetConfigInfo) == "function" and talents.activeConfigID then
        talents.activeConfigInfo = first(classTalents.GetConfigInfo, talents.activeConfigID)
    end

    return talents
end

local function snapshotPvpTalents()
    local result = {}

    if type(C_SpecializationInfo) == "table" and type(C_SpecializationInfo.GetAllSelectedPvpTalentIDs) == "function" then
        result.selectedIDs = first(C_SpecializationInfo.GetAllSelectedPvpTalentIDs)
    end

    if type(GetPvpTalentInfoByID) == "function" and type(result.selectedIDs) == "table" then
        result.selected = {}
        for index, talentID in ipairs(result.selectedIDs) do
            local ok, talentIDValue, name, icon, selected, available, spellID, unlocked, row, column, known = call(GetPvpTalentInfoByID, talentID)
            if ok then
                result.selected[index] = {
                    talentID = talentIDValue,
                    name = name,
                    icon = icon,
                    selected = selected,
                    available = available,
                    spellID = spellID,
                    unlocked = unlocked,
                    row = row,
                    column = column,
                    known = known,
                }
            end
        end
    end

    return result
end

function addon:Snapshot(reason)
    local db = ensureDatabase()
    local characterKey = getCharacterKey()

    local snapshot = {
        addon = addonName,
        reason = reason or "manual",
        capturedAt = getTime(),
        characterKey = characterKey,
        guid = getPlayerGuid(),
        build = getBuildInfo(),
        class = getClassInfo(),
        specialization = getSpecInfo(),
        actionBars = snapshotActionBars(),
        spellBook = snapshotSpellBook(),
        macros = snapshotMacros(),
        talents = snapshotTalentConfig(),
        pvpTalents = snapshotPvpTalents(),
    }

    db.characters[characterKey] = snapshot
    db.lastCharacterKey = characterKey
    db.lastCapturedAt = snapshot.capturedAt
    db.version = 1

    table.insert(db.history, {
        characterKey = characterKey,
        capturedAt = snapshot.capturedAt,
        reason = snapshot.reason,
    })

    while #db.history > 50 do
        table.remove(db.history, 1)
    end

    return snapshot
end

local function printStatus(reason)
    local snapshot = addon:Snapshot(reason)
    local actionBars = snapshot.actionBars or {}
    local slots = actionBars.slots or {}
    print(string.format("%s: saved %s with %d action slots", addonName, snapshot.characterKey, #slots))
end

local events = {
    PLAYER_LOGIN = true,
    PLAYER_LOGOUT = true,
    ACTIONBAR_SLOT_CHANGED = true,
    ACTIONBAR_PAGE_CHANGED = true,
    UPDATE_BONUS_ACTIONBAR = true,
    UPDATE_OVERRIDE_ACTIONBAR = true,
    UPDATE_POSSESS_BAR = true,
    SPELLS_CHANGED = true,
    LEARNED_SPELL_IN_TAB = true,
    PLAYER_TALENT_UPDATE = true,
    ACTIVE_TALENT_GROUP_CHANGED = true,
    TRAIT_CONFIG_UPDATED = true,
    PLAYER_SPECIALIZATION_CHANGED = true,
    UPDATE_MACROS = true,
}

local frame = CreateFrame("Frame")
for event in pairs(events) do
    frame:RegisterEvent(event)
end

frame:SetScript("OnEvent", function(_, event)
    addon:Snapshot(event)
end)

SLASH_SERVERSNAPSHOT1 = "/serversnapshot"
SLASH_SERVERSNAPSHOT2 = "/ssnap"
SlashCmdList.SERVERSNAPSHOT = function()
    printStatus("slash")
end
