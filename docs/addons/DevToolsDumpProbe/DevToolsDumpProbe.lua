local addonName = ...

local messages = {}

local function appendMessage(message)
    messages[#messages + 1] = tostring(message)
end

if DevTools_AddMessageHandler then
    DevTools_AddMessageHandler(appendMessage)
end

local function valueSummary(value)
    return {
        luaType = type(value),
        isNil = value == nil,
        tostring = tostring(value),
    }
end

local function callProtected(fn)
    local results = { pcall(fn) }
    return results
end

local function runProbe()
    wipe(messages)

    local f = CreateFrame("Frame")
    local insertResult = callProtected(function()
        return tinsert(f, "foo")
    end)
    local slotValue = f[1]
    local dumpResult = callProtected(function()
        return DevTools_Dump(f)
    end)

    DevToolsDumpProbeDB = {
        addonName = addonName,
        build = { GetBuildInfo() },
        devToolsDumpType = type(DevTools_Dump),
        addMessageHandlerType = type(DevTools_AddMessageHandler),
        frameType = type(f),
        frameString = tostring(f),
        insertOk = insertResult[1],
        insertReturn = valueSummary(insertResult[2]),
        slotOne = valueSummary(slotValue),
        slotOneEqualsFoo = slotValue == "foo",
        dumpOk = dumpResult[1],
        dumpReturn = valueSummary(dumpResult[2]),
        messageCount = #messages,
        messages = CopyTable(messages),
    }

    print("DevToolsDumpProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
