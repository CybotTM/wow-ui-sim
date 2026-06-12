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

local function packResults(ok, ...)
    local results = {}
    for index = 1, select("#", ...) do
        results[index] = valueSummary(select(index, ...))
    end
    return {
        ok = ok,
        results = results,
        error = ok and nil or tostring((...)),
    }
end

local function callProtected(fn)
    return packResults(pcall(fn))
end

local function protectedTuple(region)
    return callProtected(function()
        return region:IsProtected()
    end)
end

local function setterProbe(region)
    local result = {
        protectType = valueSummary(region and region.Protect),
        setProtectedType = valueSummary(region and region.SetProtected),
        before = protectedTuple(region),
    }

    result.protectCall = callProtected(function()
        return region:Protect()
    end)
    result.afterProtect = protectedTuple(region)

    result.setProtectedTrueCall = callProtected(function()
        return region:SetProtected(true)
    end)
    result.afterSetProtectedTrue = protectedTuple(region)

    result.setProtectedFalseCall = callProtected(function()
        return region:SetProtected(false)
    end)
    result.afterSetProtectedFalse = protectedTuple(region)

    return result
end

local function probeIsProtected()
    local plain = CreateFrame("Frame", "IsProtectedProbePlain", UIParent)
    local secureButtonResult = callProtected(function()
        return CreateFrame("Button", "IsProtectedProbeSecureButton", UIParent, "SecureActionButtonTemplate")
    end)

    local secureButton = _G.IsProtectedProbeSecureButton
    local result = {
        plainSetters = setterProbe(plain),
        secureButtonCreate = secureButtonResult,
        secureButtonExists = valueSummary(secureButton ~= nil),
    }

    if not secureButton then
        return result
    end

    local child = CreateFrame("Frame", "IsProtectedProbeChild", secureButton)
    local grandchild = CreateFrame("Frame", "IsProtectedProbeGrandchild", child)

    local anchoredToProtected = CreateFrame("Frame", "IsProtectedProbeAnchoredToProtected", UIParent)
    anchoredToProtected:SetPoint("CENTER", secureButton, "CENTER", 0, 0)

    local anchoredToChild = CreateFrame("Frame", "IsProtectedProbeAnchoredToChild", UIParent)
    anchoredToChild:SetPoint("CENTER", child, "CENTER", 0, 0)

    local childAnchoredToProtected = CreateFrame("Frame", "IsProtectedProbeChildAnchoredToProtected", UIParent)
    childAnchoredToProtected:SetPoint("CENTER", secureButton, "CENTER", 0, 0)
    secureButton.childAnchoredToProtected = childAnchoredToProtected

    result.secureButtonProtected = protectedTuple(secureButton)
    result.secureButtonSetters = setterProbe(secureButton)
    result.childProtected = protectedTuple(child)
    result.grandchildProtected = protectedTuple(grandchild)
    result.anchoredToProtectedProtected = protectedTuple(anchoredToProtected)
    result.anchoredToChildProtected = protectedTuple(anchoredToChild)
    result.childAnchoredToProtectedProtected = protectedTuple(childAnchoredToProtected)

    return result
end

local function runProbe()
    IsProtectedProbeDB = {
        addonName = addonName,
        build = { GetBuildInfo() },
        isProtected = probeIsProtected(),
    }

    print("IsProtectedProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
