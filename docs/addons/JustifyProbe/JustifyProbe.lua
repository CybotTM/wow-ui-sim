local addonName = ...

local firstFontStringRegion
local getOrCreateMessageFrame
local getOrCreateScrollingMessageFrame

local probes = {
    {
        name = "frame-font-left-top",
        kind = "FrameLayerFontString",
        object = function()
            return JustifyProbeFrameFontLeftTop
        end,
    },
    {
        name = "frame-font-center-middle",
        kind = "FrameLayerFontString",
        object = function()
            return JustifyProbeFrameFontCenterMiddle
        end,
    },
    {
        name = "frame-font-right-bottom",
        kind = "FrameLayerFontString",
        object = function()
            return JustifyProbeFrameFontRightBottom
        end,
    },
    {
        name = "frame-font-width-only",
        kind = "FrameLayerFontString",
        object = function()
            return JustifyProbeFrameFontWidthOnly
        end,
    },
    {
        name = "frame-font-height-only",
        kind = "FrameLayerFontString",
        object = function()
            return JustifyProbeFrameFontHeightOnly
        end,
    },
    {
        name = "frame-font-width-height",
        kind = "FrameLayerFontString",
        object = function()
            return JustifyProbeFrameFontWidthHeight
        end,
    },
    {
        name = "frame-font-anchor-top-only",
        kind = "FrameLayerFontString",
        object = function()
            return JustifyProbeFrameFontAnchorTopOnly
        end,
    },
    {
        name = "frame-font-anchor-bottom-only",
        kind = "FrameLayerFontString",
        object = function()
            return JustifyProbeFrameFontAnchorBottomOnly
        end,
    },
    {
        name = "frame-font-anchor-left-only",
        kind = "FrameLayerFontString",
        object = function()
            return JustifyProbeFrameFontAnchorLeftOnly
        end,
    },
    {
        name = "frame-font-anchor-right-only",
        kind = "FrameLayerFontString",
        object = function()
            return JustifyProbeFrameFontAnchorRightOnly
        end,
    },
    {
        name = "frame-font-anchor-top-left",
        kind = "FrameLayerFontString",
        object = function()
            return JustifyProbeFrameFontAnchorTopLeft
        end,
    },
    {
        name = "button-text-left-top",
        kind = "ButtonText",
        object = function()
            return JustifyProbeFrameButtonLeftTop and JustifyProbeFrameButtonLeftTop.ButtonText
        end,
    },
    {
        name = "button-text-center-middle",
        kind = "ButtonText",
        object = function()
            return JustifyProbeFrameButtonCenterMiddle and JustifyProbeFrameButtonCenterMiddle.ButtonText
        end,
    },
    {
        name = "button-text-right-bottom",
        kind = "ButtonText",
        object = function()
            return JustifyProbeFrameButtonRightBottom and JustifyProbeFrameButtonRightBottom.ButtonText
        end,
    },
    {
        name = "button-text-sized",
        kind = "ButtonText",
        object = function()
            return JustifyProbeFrameButtonSizedText and JustifyProbeFrameButtonSizedText.ButtonText
        end,
    },
    {
        name = "button-text-anchor-top-only",
        kind = "ButtonText",
        object = function()
            return JustifyProbeFrameButtonTextAnchorTopOnly and JustifyProbeFrameButtonTextAnchorTopOnly.ButtonText
        end,
    },
    {
        name = "button-text-anchor-bottom-only",
        kind = "ButtonText",
        object = function()
            return JustifyProbeFrameButtonTextAnchorBottomOnly and JustifyProbeFrameButtonTextAnchorBottomOnly.ButtonText
        end,
    },
    {
        name = "button-text-anchor-left-only",
        kind = "ButtonText",
        object = function()
            return JustifyProbeFrameButtonTextAnchorLeftOnly and JustifyProbeFrameButtonTextAnchorLeftOnly.ButtonText
        end,
    },
    {
        name = "editbox-font",
        kind = "EditBoxFontString",
        object = function()
            return firstFontStringRegion(JustifyProbeFrameEditBox)
        end,
        owner = function()
            return JustifyProbeFrameEditBox
        end,
    },
    {
        name = "editbox-font-sized",
        kind = "EditBoxFontString",
        object = function()
            return firstFontStringRegion(JustifyProbeFrameEditBoxSized)
        end,
        owner = function()
            return JustifyProbeFrameEditBoxSized
        end,
    },
    {
        name = "editbox-font-inset",
        kind = "EditBoxFontString",
        object = function()
            return firstFontStringRegion(JustifyProbeFrameEditBoxInset)
        end,
        owner = function()
            return JustifyProbeFrameEditBoxInset
        end,
    },
    {
        name = "messageframe-font",
        kind = "MessageFrameFontString",
        object = function()
            return firstFontStringRegion(getOrCreateMessageFrame(false))
        end,
        owner = function()
            return getOrCreateMessageFrame(false)
        end,
    },
    {
        name = "messageframe-font-inset",
        kind = "MessageFrameFontString",
        object = function()
            return firstFontStringRegion(getOrCreateMessageFrame(true))
        end,
        owner = function()
            return getOrCreateMessageFrame(true)
        end,
    },
    {
        name = "messageframe-owner",
        kind = "MessageFrameOwner",
        object = function()
            return getOrCreateMessageFrame(false)
        end,
        owner = function()
            return getOrCreateMessageFrame(false)
        end,
    },
    {
        name = "messageframe-owner-inset",
        kind = "MessageFrameOwner",
        object = function()
            return getOrCreateMessageFrame(true)
        end,
        owner = function()
            return getOrCreateMessageFrame(true)
        end,
    },
    {
        name = "scrolling-messageframe-font",
        kind = "ScrollingMessageFrameFontString",
        object = function()
            return firstFontStringRegion(getOrCreateScrollingMessageFrame(false))
        end,
        owner = function()
            return getOrCreateScrollingMessageFrame(false)
        end,
    },
    {
        name = "scrolling-messageframe-font-inset",
        kind = "ScrollingMessageFrameFontString",
        object = function()
            return firstFontStringRegion(getOrCreateScrollingMessageFrame(true))
        end,
        owner = function()
            return getOrCreateScrollingMessageFrame(true)
        end,
    },
    {
        name = "scrolling-messageframe-owner",
        kind = "ScrollingMessageFrameOwner",
        object = function()
            return getOrCreateScrollingMessageFrame(false)
        end,
        owner = function()
            return getOrCreateScrollingMessageFrame(false)
        end,
    },
    {
        name = "scrolling-messageframe-owner-inset",
        kind = "ScrollingMessageFrameOwner",
        object = function()
            return getOrCreateScrollingMessageFrame(true)
        end,
        owner = function()
            return getOrCreateScrollingMessageFrame(true)
        end,
    },
}

local function ensureDB()
    if type(JustifyProbeDB) ~= "table" then
        JustifyProbeDB = {}
    end
    JustifyProbeDB.runs = JustifyProbeDB.runs or {}
    return JustifyProbeDB
end

local function call(method, object, ...)
    if object == nil or type(object[method]) ~= "function" then
        return false
    end
    return pcall(object[method], object, ...)
end

local function captureTextInsets(object)
    local ok, left, right, top, bottom = call("GetTextInsets", object)
    if not ok then
        return nil
    end
    return {
        left = left,
        right = right,
        top = top,
        bottom = bottom,
    }
end

local function captureOwner(object)
    if not object then
        return nil
    end

    local okName, objectName = call("GetName", object)
    local okType, objectType = call("GetObjectType", object)
    local okRegions, regions = pcall(function()
        return { object:GetRegions() }
    end)

    local regionTypes = {}
    if okRegions and type(regions) == "table" then
        for index, region in ipairs(regions) do
            local okRegionType, regionType = call("GetObjectType", region)
            regionTypes[index] = okRegionType and regionType or type(region)
        end
    end

    return {
        objectName = okName and objectName or nil,
        objectType = okType and objectType or nil,
        textInsets = captureTextInsets(object),
        regionTypes = regionTypes,
    }
end

firstFontStringRegion = function(owner)
    if not owner then
        return nil
    end

    local ok, regions = pcall(function()
        return { owner:GetRegions() }
    end)
    if not ok or type(regions) ~= "table" then
        return nil
    end

    for _, region in ipairs(regions) do
        local okType, objectType = call("GetObjectType", region)
        if okType and objectType == "FontString" then
            return region
        end
        if type(region) == "table" and type(region.GetText) == "function" and type(region.GetNumPoints) == "function" then
            return region
        end
    end

    return nil
end

getOrCreateMessageFrame = function(withInsets)
    local name = withInsets and "JustifyProbeLuaMessageFrameInset" or "JustifyProbeLuaMessageFrame"
    local frame = _G[name]
    if not frame then
        frame = CreateFrame("MessageFrame", name, JustifyProbeFrame)
        frame:SetSize(180, 32)
        if frame.SetFontObject then
            frame:SetFontObject(GameFontNormal)
        end
        if frame.SetJustifyH then
            frame:SetJustifyH("RIGHT")
        end
        if frame.SetJustifyV then
            frame:SetJustifyV("BOTTOM")
        end
        if withInsets and frame.SetTextInsets then
            frame:SetTextInsets(7, 11, 13, 17)
        end
    end
    if frame.Clear then
        frame:Clear()
    end
    if frame.AddMessage then
        frame:AddMessage(withInsets and "MessageInset" or "MessageText")
    end
    return frame
end

getOrCreateScrollingMessageFrame = function(withInsets)
    local name = withInsets and "JustifyProbeLuaScrollingMessageFrameInset" or "JustifyProbeLuaScrollingMessageFrame"
    local frame = _G[name]
    if not frame then
        frame = CreateFrame("ScrollingMessageFrame", name, JustifyProbeFrame)
        frame:SetSize(180, 32)
        if frame.SetFontObject then
            frame:SetFontObject(GameFontNormal)
        end
        if frame.SetJustifyH then
            frame:SetJustifyH("RIGHT")
        end
        if frame.SetJustifyV then
            frame:SetJustifyV("BOTTOM")
        end
        if withInsets and frame.SetTextInsets then
            frame:SetTextInsets(7, 11, 13, 17)
        end
    end
    if frame.Clear then
        frame:Clear()
    end
    if frame.AddMessage then
        frame:AddMessage(withInsets and "ScrollingInset" or "ScrollingText")
    end
    return frame
end

local function capturePoint(object, index)
    local ok, point, relativeTo, relativePoint, xOfs, yOfs = call("GetPoint", object, index)
    if not ok or point == nil then
        return nil
    end

    local relativeName = nil
    if relativeTo and type(relativeTo.GetName) == "function" then
        relativeName = relativeTo:GetName()
    end

    return {
        point = point,
        relativeTo = relativeName,
        relativePoint = relativePoint,
        xOfs = xOfs,
        yOfs = yOfs,
    }
end

local function captureObject(probe)
    local object = probe.object()
    local owner = probe.owner and probe.owner() or nil
    if not object then
        return {
            name = probe.name,
            kind = probe.kind,
            missing = true,
            owner = captureOwner(owner),
        }
    end

    local okName, objectName = call("GetName", object)
    local okNumPoints, numPoints = call("GetNumPoints", object)
    local okWidth, width = call("GetWidth", object)
    local okHeight, height = call("GetHeight", object)
    local okJustifyH, justifyH = call("GetJustifyH", object)
    local okJustifyV, justifyV = call("GetJustifyV", object)
    local okText, text = call("GetText", object)
    local okObjectType, objectType = call("GetObjectType", object)

    local points = {}
    if okNumPoints and type(numPoints) == "number" then
        for index = 1, numPoints do
            points[index] = capturePoint(object, index)
        end
    end

    return {
        name = probe.name,
        kind = probe.kind,
        objectName = okName and objectName or nil,
        objectType = okObjectType and objectType or nil,
        numPoints = okNumPoints and numPoints or nil,
        points = points,
        width = okWidth and width or nil,
        height = okHeight and height or nil,
        justifyH = okJustifyH and justifyH or nil,
        justifyV = okJustifyV and justifyV or nil,
        text = okText and text or nil,
        textInsets = captureTextInsets(object),
        owner = captureOwner(owner),
    }
end

local function capture()
    local db = ensureDB()
    local run = {
        addon = addonName,
        capturedAt = time(),
        build = { GetBuildInfo() },
        parent = {
            width = JustifyProbeFrame:GetWidth(),
            height = JustifyProbeFrame:GetHeight(),
        },
        probes = {},
    }

    for index, probe in ipairs(probes) do
        local ok, result = pcall(captureObject, probe)
        if ok then
            run.probes[index] = result
        else
            run.probes[index] = {
                name = probe.name,
                kind = probe.kind,
                error = tostring(result),
            }
        end
    end

    db.latest = run
    table.insert(db.runs, run)
    while #db.runs > 20 do
        table.remove(db.runs, 1)
    end

    print(string.format("%s captured %d justify probes", addonName, #run.probes))
end

SLASH_JUSTIFYPROBE1 = "/justifyprobe"
SLASH_JUSTIFYPROBE2 = "/jprobe"
SlashCmdList.JUSTIFYPROBE = capture

local frame = CreateFrame("Frame")
frame:RegisterEvent("ADDON_LOADED")
frame:RegisterEvent("PLAYER_LOGIN")
frame:RegisterEvent("PLAYER_LOGOUT")
frame:SetScript("OnEvent", function(_, event, loadedAddonName)
    if event == "ADDON_LOADED" and loadedAddonName == addonName then
        ensureDB()
    elseif event == "PLAYER_LOGIN" then
        C_Timer.After(0, capture)
    elseif event == "PLAYER_LOGOUT" then
        capture()
    end
end)
