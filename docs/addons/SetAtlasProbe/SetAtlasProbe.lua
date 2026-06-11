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

local function probeSetAtlas()
    local parent = CreateFrame("Frame", "SetAtlasProbeParent", UIParent)
    local texture = parent:CreateTexture("SetAtlasProbeTexture")

    return {
        nilArg = callProtected(function()
            return texture:SetAtlas(nil)
        end),
        noArg = callProtected(function()
            return texture:SetAtlas()
        end),
        falseArg = callProtected(function()
            return texture:SetAtlas(false)
        end),
        zeroArg = callProtected(function()
            return texture:SetAtlas(0)
        end),
        emptyString = callProtected(function()
            return texture:SetAtlas("")
        end),
        unknownString = callProtected(function()
            return texture:SetAtlas("nonexistent-atlas-name-12345")
        end),
    }
end

local function runProbe()
    SetAtlasProbeDB = {
        addonName = addonName,
        build = { GetBuildInfo() },
        setAtlas = probeSetAtlas(),
    }

    print("SetAtlasProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
