local addonName = ...

-- Events (CVAR_UPDATE, DISPLAY_SIZE_CHANGED) can fire before ADDON_LOADED /
-- PLAYER_LOGIN, before SavedVariables exist. Buffer entries until the DB is
-- bound, then flush.
local pendingEntries = {}
local dbReady = false

local function snapshot()
    local physW, physH = GetPhysicalScreenSize()
    return {
        screenWidth = GetScreenWidth(),
        screenHeight = GetScreenHeight(),
        physicalWidth = physW,
        physicalHeight = physH,
        uiScaleCVar = GetCVar("uiScale"),
        useUiScaleCVar = GetCVar("useUiScale"),
        uiParentScale = UIParent and UIParent:GetScale(),
        uiParentEffectiveScale = UIParent and UIParent:GetEffectiveScale(),
    }
end

local function appendEntry(kind, extra)
    local entry = {
        kind = kind,
        time = GetTime(),
        date = date("%Y-%m-%d %H:%M:%S"),
        state = snapshot(),
    }
    if extra then
        for key, value in pairs(extra) do
            entry[key] = value
        end
    end
    if dbReady then
        table.insert(ScaleEventProbeDB.log, entry)
    else
        table.insert(pendingEntries, entry)
    end
    return entry
end

local function bindSavedVariables()
    if dbReady then
        return
    end
    ScaleEventProbeDB = ScaleEventProbeDB or {}
    ScaleEventProbeDB.addonName = addonName
    ScaleEventProbeDB.build = { GetBuildInfo() }
    ScaleEventProbeDB.log = ScaleEventProbeDB.log or {}
    dbReady = true
    for _, entry in ipairs(pendingEntries) do
        table.insert(ScaleEventProbeDB.log, entry)
    end
    pendingEntries = {}
end

local counts = {}

local function onEvent(_, event, ...)
    if event == "ADDON_LOADED" then
        local loadedName = ...
        if loadedName == addonName then
            bindSavedVariables()
        end
        return
    end

    if event == "PLAYER_LOGIN" then
        bindSavedVariables()
        appendEntry("login")
        print("ScaleEventProbe: logging DISPLAY_SIZE_CHANGED / UI_SCALE_CHANGED / CVAR_UPDATE(uiScale,useUiScale)")
        print("ScaleEventProbe: use /scaleprobe mark <label> before each scenario, /scaleprobe counts for totals")
        return
    end

    if event == "CVAR_UPDATE" then
        local cvarName, value = ...
        if cvarName ~= "uiScale" and cvarName ~= "useUiScale" then
            return
        end
        appendEntry("cvar", { cvar = cvarName, value = tostring(value) })
        print(("ScaleEventProbe: CVAR_UPDATE %s=%s"):format(tostring(cvarName), tostring(value)))
        return
    end

    counts[event] = (counts[event] or 0) + 1
    local entry = appendEntry("event", { event = event })
    local state = entry.state
    print(("ScaleEventProbe: %s #%d screen=%.1fx%.1f phys=%dx%d uiScale=%s useUiScale=%s eff=%.4f"):format(
        event,
        counts[event],
        state.screenWidth or -1,
        state.screenHeight or -1,
        state.physicalWidth or -1,
        state.physicalHeight or -1,
        tostring(state.uiScaleCVar),
        tostring(state.useUiScaleCVar),
        state.uiParentEffectiveScale or -1
    ))
end

SLASH_SCALEEVENTPROBE1 = "/scaleprobe"
SlashCmdList.SCALEEVENTPROBE = function(command)
    bindSavedVariables()
    local action, rest = command:match("^(%S*)%s*(.-)$")
    if action == "mark" and rest ~= "" then
        appendEntry("mark", { label = rest })
        print("ScaleEventProbe: marked '" .. rest .. "'")
    elseif action == "counts" then
        for event, count in pairs(counts) do
            print(("ScaleEventProbe: %s fired %d times this session"):format(event, count))
        end
        print(("ScaleEventProbe: %d log entries total; /reload or logout to flush SavedVariables"):format(#ScaleEventProbeDB.log))
    elseif action == "reset" then
        ScaleEventProbeDB.log = {}
        counts = {}
        appendEntry("reset")
        print("ScaleEventProbe: log cleared")
    else
        print("ScaleEventProbe commands: /scaleprobe mark <label> | counts | reset")
    end
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("ADDON_LOADED")
frame:RegisterEvent("PLAYER_LOGIN")
frame:RegisterEvent("DISPLAY_SIZE_CHANGED")
frame:RegisterEvent("UI_SCALE_CHANGED")
frame:RegisterEvent("CVAR_UPDATE")
frame:SetScript("OnEvent", onEvent)
