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

local function makeRecorder(log, tag)
    return function()
        log[#log + 1] = tag
    end
end

local function probeHookScriptBindings()
    local frame = CreateFrame("Frame", "HookScriptBindingProbeFrame", UIParent)
    local log = {}
    frame:Hide()

    local hook0Empty = callProtected(function()
        return frame:HookScript("OnShow", makeRecorder(log, "hook0Empty"), 0)
    end)
    local get0AfterEmpty = valueSummary(frame:GetScript("OnShow", 0))

    local hook2Empty = callProtected(function()
        return frame:HookScript("OnShow", makeRecorder(log, "hook2Empty"), 2)
    end)
    local get2AfterEmpty = valueSummary(frame:GetScript("OnShow", 2))

    local hook1Empty = callProtected(function()
        return frame:HookScript("OnShow", makeRecorder(log, "hook1Empty"))
    end)
    local get1AfterEmpty = valueSummary(frame:GetScript("OnShow"))

    local showAfterEmptyHooks = callProtected(function()
        return frame:Show()
    end)
    local orderAfterEmptyHooks = CopyTable(log)

    frame:Hide()
    wipe(log)
    frame:SetScript("OnShow", makeRecorder(log, "normalSet"))

    local hook0AfterNormal = callProtected(function()
        return frame:HookScript("OnShow", makeRecorder(log, "hook0AfterNormal"), 0)
    end)
    local hook2AfterNormal = callProtected(function()
        return frame:HookScript("OnShow", makeRecorder(log, "hook2AfterNormal"), 2)
    end)
    local showAfterNormalHooks = callProtected(function()
        return frame:Show()
    end)
    local orderAfterNormalHooks = CopyTable(log)

    return {
        hook0Empty = hook0Empty,
        get0AfterEmpty = get0AfterEmpty,
        hook2Empty = hook2Empty,
        get2AfterEmpty = get2AfterEmpty,
        hook1Empty = hook1Empty,
        get1AfterEmpty = get1AfterEmpty,
        showAfterEmptyHooks = showAfterEmptyHooks,
        orderAfterEmptyHooks = orderAfterEmptyHooks,
        hook0AfterNormal = hook0AfterNormal,
        hook2AfterNormal = hook2AfterNormal,
        showAfterNormalHooks = showAfterNormalHooks,
        orderAfterNormalHooks = orderAfterNormalHooks,
    }
end

local function runProbe()
    HookScriptBindingProbeDB = {
        addonName = addonName,
        build = { GetBuildInfo() },
        hookScriptBindings = probeHookScriptBindings(),
    }

    print("HookScriptBindingProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
