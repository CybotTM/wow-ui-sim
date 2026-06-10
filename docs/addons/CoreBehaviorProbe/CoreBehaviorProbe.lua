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

local function enumerateContains(target)
    local current = EnumerateFrames()
    while current do
        if current == target then
            return true
        end
        current = EnumerateFrames(current)
    end
    return false
end

local function probeSetForbidden()
    local frame = CreateFrame("Frame", "CoreBehaviorSetForbiddenProbe", UIParent)
    local before = frame:IsForbidden()

    local setTrue = callProtected(function()
        return frame:SetForbidden(true)
    end)
    local afterTrue = frame:IsForbidden()

    local setFalse = callProtected(function()
        return frame:SetForbidden(false)
    end)
    local afterFalse = frame:IsForbidden()

    return {
        before = before,
        setTrue = setTrue,
        afterTrue = afterTrue,
        setFalse = setFalse,
        afterFalse = afterFalse,
    }
end

local function probeCreateForbiddenFrame()
    local result = {
        createForbiddenFrameType = type(CreateForbiddenFrame),
    }

    if type(CreateForbiddenFrame) ~= "function" then
        return result
    end

    local created
    result.create = callProtected(function()
        created = CreateForbiddenFrame("Frame", "CoreBehaviorCreateForbiddenProbe", UIParent)
        return created
    end)

    if created then
        result.objectType = created:GetObjectType()
        result.name = created:GetName()
        result.isForbidden = created:IsForbidden()
        result.seenByEnumerateFrames = enumerateContains(created)
    end

    return result
end

local function probeRegisterUnitEvent()
    local frame = CreateFrame("Frame", "CoreBehaviorRegisterUnitEventProbe", UIParent)
    local invalid = CreateFrame("Frame", "CoreBehaviorRegisterUnitEventInvalidProbe", UIParent)

    local register = callProtected(function()
        return frame:RegisterUnitEvent("UNIT_HEALTH", "player")
    end)
    local registered, unit = frame:IsEventRegistered("UNIT_HEALTH")

    local invalidRegister = callProtected(function()
        return invalid:RegisterUnitEvent("UNIT_HEALTH", "not_a_unit")
    end)
    local invalidRegistered, invalidUnit = invalid:IsEventRegistered("UNIT_HEALTH")

    return {
        register = register,
        registered = registered,
        unit = valueSummary(unit),
        invalidRegister = invalidRegister,
        invalidRegistered = invalidRegistered,
        invalidUnit = valueSummary(invalidUnit),
    }
end

local function probeAttributeFalseWildcard()
    local frame = CreateFrame("Frame", "CoreBehaviorAttributeFalseProbe", UIParent)
    frame:SetAttribute("*type1", false)
    frame:SetAttribute("*type2", true)
    frame:SetAttribute("*type3", "spell")

    local directFalse = frame:GetAttribute("*type1")
    local wildcardFalse = frame:GetAttribute("help", "type", "1")
    local directTrue = frame:GetAttribute("*type2")
    local wildcardTrue = frame:GetAttribute("help", "type", "2")
    local directString = frame:GetAttribute("*type3")
    local wildcardString = frame:GetAttribute("help", "type", "3")

    return {
        directFalse = valueSummary(directFalse),
        wildcardFalse = valueSummary(wildcardFalse),
        directTrue = valueSummary(directTrue),
        wildcardTrue = valueSummary(wildcardTrue),
        directString = valueSummary(directString),
        wildcardString = valueSummary(wildcardString),
    }
end

local function frameLevelSnapshot(frame)
    return {
        frameLevel = frame:GetFrameLevel(),
        raisedFrameLevel = type(frame.GetRaisedFrameLevel) == "function" and frame:GetRaisedFrameLevel() or nil,
        frameStrata = frame:GetFrameStrata(),
        isShown = frame:IsShown(),
        isVisible = frame:IsVisible(),
    }
end

local function makeRaiseFrame(name, parent, level)
    local frame = CreateFrame("Frame", name, parent)
    frame:SetSize(20, 20)
    frame:SetPoint("CENTER")
    frame:SetFrameLevel(level)
    frame:Show()
    return frame
end

local function runRaiseCase(prefix, parent)
    local low = makeRaiseFrame(prefix .. "Low", parent, 1)
    local high = makeRaiseFrame(prefix .. "High", parent, 10)

    local before = {
        low = frameLevelSnapshot(low),
        high = frameLevelSnapshot(high),
    }

    local raise = callProtected(function()
        return low:Raise()
    end)

    local afterRaise = {
        low = frameLevelSnapshot(low),
        high = frameLevelSnapshot(high),
    }

    local lower = callProtected(function()
        return high:Lower()
    end)

    local afterLower = {
        low = frameLevelSnapshot(low),
        high = frameLevelSnapshot(high),
    }

    return {
        getRaisedFrameLevelType = type(low.GetRaisedFrameLevel),
        before = before,
        raise = raise,
        afterRaise = afterRaise,
        lower = lower,
        afterLower = afterLower,
        lowRaisedAboveHigh = afterRaise.low.raisedFrameLevel ~= nil
            and afterRaise.high.raisedFrameLevel ~= nil
            and afterRaise.low.raisedFrameLevel > afterRaise.high.raisedFrameLevel,
        highLoweredBelowLow = afterLower.low.raisedFrameLevel ~= nil
            and afterLower.high.raisedFrameLevel ~= nil
            and afterLower.high.raisedFrameLevel < afterLower.low.raisedFrameLevel,
    }
end

local function probeRaise()
    local parent = CreateFrame("Frame", "CoreBehaviorRaiseProbeParent", UIParent)
    parent:SetSize(100, 100)
    parent:SetPoint("CENTER")
    parent:Show()

    return {
        parented = runRaiseCase("CoreBehaviorRaiseProbeParented", parent),
        uiParent = runRaiseCase("CoreBehaviorRaiseProbeUIParent", UIParent),
    }
end

local function currentMouseFocus()
    if type(GetMouseFoci) == "function" then
        local values = { GetMouseFoci() }
        if #values == 1 and type(values[1]) == "table" then
            return values[1][1]
        end
        return values[1]
    end

    if type(GetMouseFocus) == "function" then
        return GetMouseFocus()
    end
end

local function focusSnapshot(low, high)
    local focus = currentMouseFocus()
    local which = "other"
    if focus == low then
        which = "low"
    elseif focus == high then
        which = "high"
    elseif focus == nil then
        which = "nil"
    end

    return {
        which = which,
        focus = valueSummary(focus),
        getMouseFocusType = type(GetMouseFocus),
        getMouseFociType = type(GetMouseFoci),
        low = frameLevelSnapshot(low),
        high = frameLevelSnapshot(high),
    }
end

local function makeRaiseHitFrame(level)
    local frame = CreateFrame("Frame", nil, UIParent)
    frame:SetAllPoints(UIParent)
    frame:SetFrameStrata("DIALOG")
    frame:SetFrameLevel(level)
    frame:EnableMouse(true)
    frame:Show()
    return frame
end

local function startRaiseHitProbe()
    CoreBehaviorProbeDB.raiseHit = {
        status = "pending",
    }

    if type(C_Timer) ~= "table" or type(C_Timer.After) ~= "function" then
        CoreBehaviorProbeDB.raiseHit = {
            status = "skipped",
            reason = "C_Timer.After missing",
        }
        return
    end

    local low = makeRaiseHitFrame(1)
    local high = makeRaiseHitFrame(10)

    C_Timer.After(0, function()
        local before = focusSnapshot(low, high)
        local raise = callProtected(function()
            return low:Raise()
        end)

        C_Timer.After(0, function()
            local afterRaise = focusSnapshot(low, high)
            local lower = callProtected(function()
                return high:Lower()
            end)

            C_Timer.After(0, function()
                local afterLower = focusSnapshot(low, high)
                low:Hide()
                high:Hide()

                CoreBehaviorProbeDB.raiseHit = {
                    status = "captured",
                    before = before,
                    raise = raise,
                    afterRaise = afterRaise,
                    lower = lower,
                    afterLower = afterLower,
                }

                print("CoreBehaviorProbe Raise/Lower hit capture complete; /reload or logout to flush SavedVariables")
            end)
        end)
    end)
end

local function runProbe()
    CoreBehaviorProbeDB = {
        addonName = addonName,
        build = { GetBuildInfo() },
        setForbidden = probeSetForbidden(),
        createForbiddenFrame = probeCreateForbiddenFrame(),
        registerUnitEvent = probeRegisterUnitEvent(),
        attributeFalseWildcard = probeAttributeFalseWildcard(),
        raise = probeRaise(),
    }
    startRaiseHitProbe()

    print("CoreBehaviorProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
