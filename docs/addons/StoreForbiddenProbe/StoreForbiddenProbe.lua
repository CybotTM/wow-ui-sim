local addonName = ...

local function valueSummary(value)
    local luaType = type(value)
    local scalarValue
    if luaType == "nil" or luaType == "boolean" or luaType == "number" or luaType == "string" then
        scalarValue = value
    end

    return {
        luaType = luaType,
        isNil = value == nil,
        value = scalarValue,
        tostring = tostring(value),
    }
end

local function callProtected(fn)
    local results = { pcall(fn) }
    local ok = table.remove(results, 1)
    local summaries = {}
    for index, value in ipairs(results) do
        summaries[index] = valueSummary(value)
    end
    return {
        ok = ok,
        results = summaries,
        error = ok and nil or tostring(results[1]),
    }
end

local function protectedTuple(region)
    if not region then
        return valueSummary(nil)
    end
    return callProtected(function()
        return region:IsForbidden(), region:IsProtected()
    end)
end

local function populateDropdown(dropdown, result, key)
    return callProtected(function()
        StoreDropdown_SetDropdown(dropdown, {
            { text = "first", checked = true },
            { text = "second", checked = false },
        }, function() end)

        result[key .. "ButtonCount"] = valueSummary(dropdown.List and dropdown.List.Buttons and #dropdown.List.Buttons)
        result[key .. "FirstButton"] = protectedTuple(dropdown.List and dropdown.List.Buttons and dropdown.List.Buttons[1])
        result[key .. "SecondButton"] = protectedTuple(dropdown.List and dropdown.List.Buttons and dropdown.List.Buttons[2])
        return true
    end)
end

local function probeStoreDropdownForbidden()
    local result = {
        addonVisibleCreateForbiddenFrame = valueSummary(type(CreateForbiddenFrame)),
        loadStoreUI = callProtected(function()
            return C_AddOns.LoadAddOn("Blizzard_StoreUI")
        end),
        storeDropdownFunction = valueSummary(type(StoreDropdown_SetDropdown)),
        storeVASValidationFrame = valueSummary(type(StoreVASValidationFrame)),
    }

    if type(StoreDropdown_SetDropdown) ~= "function" then
        return result
    end

    result.realBlizzardDropdowns = {}
    local realDropdowns = {
        RealmSelector = StoreVASValidationFrame and StoreVASValidationFrame.RealmSelector,
        CharacterSelector = StoreVASValidationFrame and StoreVASValidationFrame.CharacterSelector,
        TransferAccountDropdown = StoreVASValidationFrame and StoreVASValidationFrame.TransferAccountDropdown,
        TransferBnetWoWAccountDropdown = StoreVASValidationFrame and StoreVASValidationFrame.TransferBnetWoWAccountDropdown,
    }
    for key, dropdown in pairs(realDropdowns) do
        local dropdownResult = {
            exists = valueSummary(dropdown ~= nil),
        }
        dropdownResult.populate = dropdown and populateDropdown(dropdown, dropdownResult, "real") or nil
        result.realBlizzardDropdowns[key] = dropdownResult
    end

    result.syntheticDropdown = callProtected(function()
        local dropdown = CreateFrame("Frame", "StoreForbiddenProbeDropdown", UIParent, "StoreDropdownMenuTemplate")
        dropdown:SetWidth(280)
        return populateDropdown(dropdown, result, "synthetic")
    end)

    return result
end

local function runProbe()
    StoreForbiddenProbeDB = {
        addonName = addonName,
        build = { GetBuildInfo() },
        storeForbidden = probeStoreDropdownForbidden(),
    }

    print("StoreForbiddenProbe captured; /sfp scans open Store UI; /reload or logout to flush SavedVariables")
end

local function scanFrameTree(root)
    local rows = {}
    local function getChildren(frame)
        local results = { pcall(function()
            return frame:GetChildren()
        end) }
        local ok = table.remove(results, 1)
        if not ok then
            return nil, tostring(results[1])
        end
        return results
    end

    local function scan(frame, depth)
        if not frame or depth > 8 then
            return
        end
        local okForbidden, forbidden = pcall(function()
            return frame:IsForbidden()
        end)
        local okProtected, protected = pcall(function()
            return frame:IsProtected()
        end)
        if okForbidden and okProtected and (forbidden or protected) then
            local okName, name = pcall(function()
                return frame:GetName()
            end)
            local okObjectType, objectType = pcall(function()
                return frame:GetObjectType()
            end)
            rows[#rows + 1] = {
                name = okName and name or nil,
                objectType = okObjectType and objectType or nil,
                forbidden = forbidden,
                protected = protected,
                depth = depth,
            }
        end
        local children = getChildren(frame)
        if not children then
            return
        end
        for index = 1, #children do
            scan(children[index], depth + 1)
        end
    end
    scan(root, 0)
    return rows
end

local function forbiddenProtectedTuple(frame)
    if not frame then
        return nil
    end
    local okForbidden, forbidden = pcall(function()
        return frame:IsForbidden()
    end)
    local okProtected, protected = pcall(function()
        return frame:IsProtected()
    end)
    return {
        forbidden = okForbidden and forbidden or nil,
        forbiddenError = okForbidden and nil or tostring(forbidden),
        protected = okProtected and protected or nil,
        protectedError = okProtected and nil or tostring(protected),
    }
end

SLASH_STOREFORBIDDENPROBE1 = "/sfp"
SlashCmdList.STOREFORBIDDENPROBE = function()
    StoreForbiddenProbeDB = StoreForbiddenProbeDB or {}
    StoreForbiddenProbeDB.manual = {
        build = { GetBuildInfo() },
        storeFrameForbidden = forbiddenProtectedTuple(StoreFrame),
        vasForbidden = forbiddenProtectedTuple(StoreVASValidationFrame),
        forbiddenDescendants = StoreVASValidationFrame and scanFrameTree(StoreVASValidationFrame) or nil,
    }
    print("StoreForbiddenProbe /sfp captured", StoreForbiddenProbeDB.manual.forbiddenDescendants and #StoreForbiddenProbeDB.manual.forbiddenDescendants or 0, "forbidden/protected descendants")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
