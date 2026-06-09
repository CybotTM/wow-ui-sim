local addonName = ...

local function readFrameSlot(frame, key)
    return pcall(function()
        return frame[key]
    end)
end

local function writeFrameSlot(frame, key, value)
    return pcall(function()
        frame[key] = value
    end)
end

local function callMethod(object, methodName)
    return pcall(function()
        return object[methodName](object)
    end)
end

local function valueSummary(value)
    return {
        luaType = type(value),
        isNil = value == nil,
        tostring = tostring(value),
    }
end

local function runProbe()
    local plain = CreateFrame("Frame", "FrameIdentityProbePlain", UIParent)
    local protected = CreateFrame("Button", "FrameIdentityProbeProtected", UIParent, "SecureActionButtonTemplate")

    local plainSlotOk, plainSlot = readFrameSlot(plain, 0)
    local protectedSlotOk, protectedSlot = readFrameSlot(protected, 0)
    local plainBeforeOk, plainBefore = callMethod(plain, "IsProtected")
    local protectedOk, protectedValue = callMethod(protected, "IsProtected")
    local assignOk, assignError = writeFrameSlot(plain, 0, protectedSlot)
    local plainAfterOk, plainAfter = callMethod(plain, "IsProtected")
    local plainNameOk, plainName = callMethod(plain, "GetName")

    FrameIdentityProbeDB = {
        addonName = addonName,
        build = { GetBuildInfo() },
        plainSlotReadOk = plainSlotOk,
        protectedSlotReadOk = protectedSlotOk,
        plainSlotBefore = valueSummary(plainSlot),
        protectedSlot = valueSummary(protectedSlot),
        slotsEqualBefore = plainSlot == protectedSlot,
        plainBeforeOk = plainBeforeOk,
        plainBefore = plainBefore,
        protectedOk = protectedOk,
        protectedValue = protectedValue,
        assignOk = assignOk,
        assignError = assignError,
        plainAfterOk = plainAfterOk,
        plainAfter = plainAfter,
        plainNameOk = plainNameOk,
        plainName = plainName,
        plainSlotEqualsProtectedAfter = plain[0] == protectedSlot,
    }

    print("FrameIdentityProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
