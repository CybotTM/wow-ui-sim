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

local function makeEventRecorder()
    local events = {}
    return events, function(self, name, value)
        events[#events + 1] = {
            selfName = type(self.GetName) == "function" and self:GetName() or nil,
            name = name,
            value = valueSummary(value),
            current = valueSummary(type(self.GetAttribute) == "function" and self:GetAttribute(name) or nil),
        }
    end
end

local function runRepeatedScalarProbe()
    local frame = CreateFrame("Frame", "AttributeDispatchRepeatedScalarProbe", UIParent)
    local events, handler = makeEventRecorder()
    frame:SetScript("OnAttributeChanged", handler)

    local first = callProtected(function()
        return frame:SetAttribute("showgrid", 1)
    end)
    local second = callProtected(function()
        return frame:SetAttribute("showgrid", 1)
    end)

    return {
        first = first,
        second = second,
        finalValue = valueSummary(frame:GetAttribute("showgrid")),
        eventCount = #events,
        events = events,
    }
end

local function runRepeatedFalseProbe()
    local frame = CreateFrame("Frame", "AttributeDispatchRepeatedFalseProbe", UIParent)
    local events, handler = makeEventRecorder()
    frame:SetScript("OnAttributeChanged", handler)

    local first = callProtected(function()
        return frame:SetAttribute("flag", false)
    end)
    local second = callProtected(function()
        return frame:SetAttribute("flag", false)
    end)

    return {
        first = first,
        second = second,
        finalValue = valueSummary(frame:GetAttribute("flag")),
        eventCount = #events,
        events = events,
    }
end

local function runPanelPulseProbe()
    local first = CreateFrame("Frame", "AttributeDispatchPanelPulseFirst", UIParent)
    first:SetSize(300, 400)
    first:Hide()
    RegisterUIPanel(first, { area = "center", pushable = 0, whileDead = 1 })

    local second = CreateFrame("Frame", "AttributeDispatchPanelPulseSecond", UIParent)
    second:SetSize(300, 400)
    second:Hide()
    RegisterUIPanel(second, { area = "center", pushable = 0, whileDead = 1 })

    local showFirst = callProtected(function()
        return ShowUIPanel(first)
    end)
    local showSecond = callProtected(function()
        return ShowUIPanel(second)
    end)
    local closeAll = callProtected(function()
        return CloseAllWindows()
    end)

    return {
        showFirst = showFirst,
        firstShownAfterShowFirst = first:IsShown(),
        secondShownAfterShowFirst = second:IsShown(),
        showSecond = showSecond,
        firstShownAfterShowSecond = first:IsShown(),
        secondShownAfterShowSecond = second:IsShown(),
        closeAll = closeAll,
        firstShownAfterCloseAll = first:IsShown(),
        secondShownAfterCloseAll = second:IsShown(),
    }
end

local function runProbe()
    AttributeDispatchProbeDB = {
        addonName = addonName,
        build = { GetBuildInfo() },
        capturedAt = date("%Y-%m-%dT%H:%M:%S"),
        repeatedScalar = runRepeatedScalarProbe(),
        repeatedFalse = runRepeatedFalseProbe(),
        panelPulse = runPanelPulseProbe(),
    }

    print("AttributeDispatchProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
