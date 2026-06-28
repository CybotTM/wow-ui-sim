local ADDON_NAME = ...

local function safeCall(fn)
    local ok, a, b, c = pcall(fn)
    if ok then
        return true, a, b, c
    end
    return false, tostring(a)
end

local function describeGlobal(name)
    local value = _G[name]
    local info = {
        exists = value ~= nil,
        luaType = type(value),
        tostringValue = tostring(value),
    }

    if type(value) == "table" or type(value) == "userdata" then
        local ok, objectType = safeCall(function()
            return value.GetObjectType and value:GetObjectType() or nil
        end)
        info.objectTypeOk = ok
        info.objectType = objectType

        local nameOk, objectName = safeCall(function()
            return value.GetName and value:GetName() or nil
        end)
        info.getNameOk = nameOk
        info.objectName = objectName

        local fontOk, fontPath, fontHeight, fontFlags = safeCall(function()
            return value.GetFont and value:GetFont() or nil
        end)
        info.getFontOk = fontOk
        info.fontPath = fontPath
        info.fontHeight = fontHeight
        info.fontFlags = fontFlags
    end

    return info
end

local function describeFontString(name, expectedFontGlobalName)
    local value = _G[name]
    local info = describeGlobal(name)

    if value then
        local fontObjectOk, fontObject = safeCall(function()
            return value:GetFontObject()
        end)
        info.getFontObjectOk = fontObjectOk
        info.fontObjectString = tostring(fontObject)
        info.fontObjectSameAsExpectedGlobal = fontObject == _G[expectedFontGlobalName]

        if fontObject then
            local nameOk, fontObjectName = safeCall(function()
                return fontObject.GetName and fontObject:GetName() or nil
            end)
            info.fontObjectNameOk = nameOk
            info.fontObjectName = fontObjectName
        end

        local fontOk, fontPath, fontHeight, fontFlags = safeCall(function()
            return value:GetFont()
        end)
        info.getFontOk = fontOk
        info.fontPath = fontPath
        info.fontHeight = fontHeight
        info.fontFlags = fontFlags
    end

    return info
end

local function captureResults()
    FontVirtualProbeDB = {
        addonName = ADDON_NAME,
        buildInfo = { GetBuildInfo() },
        virtualFont = describeGlobal("FontVirtualProbeVirtualFont"),
        concreteFont = describeGlobal("FontVirtualProbeConcreteFont"),
        virtualString = describeFontString("FontVirtualProbeVirtualString", "FontVirtualProbeVirtualFont"),
        concreteString = describeFontString("FontVirtualProbeConcreteString", "FontVirtualProbeConcreteFont"),
    }

    print("FontVirtualProbe virtual global:", FontVirtualProbeDB.virtualFont.luaType, FontVirtualProbeDB.virtualFont.objectName)
    print("FontVirtualProbe concrete global:", FontVirtualProbeDB.concreteFont.luaType, FontVirtualProbeDB.concreteFont.objectName)
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", captureResults)
