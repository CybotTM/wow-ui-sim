local addonName = ...

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
            return JustifyProbeFrameEditBoxText
        end,
    },
    {
        name = "editbox-font-sized",
        kind = "EditBoxFontString",
        object = function()
            return JustifyProbeFrameEditBoxSizedText
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
    if not object then
        return {
            name = probe.name,
            kind = probe.kind,
            missing = true,
        }
    end

    local okName, objectName = call("GetName", object)
    local okNumPoints, numPoints = call("GetNumPoints", object)
    local okWidth, width = call("GetWidth", object)
    local okHeight, height = call("GetHeight", object)
    local okJustifyH, justifyH = call("GetJustifyH", object)
    local okJustifyV, justifyV = call("GetJustifyV", object)
    local okText, text = call("GetText", object)

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
        numPoints = okNumPoints and numPoints or nil,
        points = points,
        width = okWidth and width or nil,
        height = okHeight and height or nil,
        justifyH = okJustifyH and justifyH or nil,
        justifyV = okJustifyV and justifyV or nil,
        text = okText and text or nil,
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
        run.probes[index] = captureObject(probe)
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
