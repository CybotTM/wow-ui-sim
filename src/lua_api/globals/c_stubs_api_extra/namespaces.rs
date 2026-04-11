use mlua::{Lua, Result};

/// C_ItemSocketInfo, C_PetInfo, C_UnitAurasPrivate stubs.
pub(super) fn register_item_pet_aura_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
