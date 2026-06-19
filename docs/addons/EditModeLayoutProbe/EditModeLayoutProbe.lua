local addonName = ...

local function ensureDb()
    if type(EditModeLayoutProbeDB) ~= "table" then
        EditModeLayoutProbeDB = {}
    end
    if type(EditModeLayoutProbeDB.history) ~= "table" then
        EditModeLayoutProbeDB.history = {}
    end
    return EditModeLayoutProbeDB
end

local function call(fn, ...)
    if type(fn) ~= "function" then
        return false, "not_function"
    end
    return pcall(fn, ...)
end

local function first(fn, ...)
    local ok, value = call(fn, ...)
    if ok then
        return value
    end
    return nil
end

local function characterKey()
    local realm = first(GetRealmName) or "UnknownRealm"
    local name = first(UnitName, "player") or "Unknown"
    return realm .. "/" .. name
end

local function copyLayoutInfo(info)
    if type(info) ~= "table" then
        return info
    end
    return {
        layoutIndex = info.layoutIndex,
        layoutName = info.layoutName,
        layoutType = info.layoutType,
        systems = type(info.systems) == "table" and #info.systems or nil,
    }
end

local function loadEditMode()
    local result = {}
    result.beforeLoaded = type(C_AddOns) == "table"
        and type(C_AddOns.IsAddOnLoaded) == "function"
        and C_AddOns.IsAddOnLoaded("Blizzard_EditMode") or nil

    local okUiParent, valueUiParent = call(UIParentLoadAddOn, "Blizzard_EditMode")
    result.uiParentLoadOk = okUiParent
    result.uiParentLoadValue = valueUiParent

    local okCAddOns, valueCAddOns = false, "missing_C_AddOns_LoadAddOn"
    if type(C_AddOns) == "table" then
        okCAddOns, valueCAddOns = call(C_AddOns.LoadAddOn, "Blizzard_EditMode")
    end
    result.cAddOnsLoadOk = okCAddOns
    result.cAddOnsLoadValue = valueCAddOns

    result.afterLoaded = type(C_AddOns) == "table"
        and type(C_AddOns.IsAddOnLoaded) == "function"
        and C_AddOns.IsAddOnLoaded("Blizzard_EditMode") or nil
    return result
end

local function snapshot(reason)
    local db = ensureDb()
    local record = {
        reason = reason or "manual",
        capturedAt = first(time) or 0,
        characterKey = characterKey(),
        loadEditMode = loadEditMode(),
        globals = {
            EditModeManagerFrame = type(EditModeManagerFrame),
            C_EditMode = type(C_EditMode),
        },
    }

    local emm = EditModeManagerFrame
    record.manager = {
        type = type(emm),
        getActiveLayoutInfo = type(emm) == "table" and type(emm.GetActiveLayoutInfo) or nil,
    }
    if type(emm) == "table" and type(emm.GetActiveLayoutInfo) == "function" then
        local ok, info = call(emm.GetActiveLayoutInfo, emm)
        record.manager.getActiveLayoutInfoOk = ok
        record.manager.activeLayoutInfo = copyLayoutInfo(info)
    end

    local cEditMode = C_EditMode
    record.cEditMode = {
        type = type(cEditMode),
        getLayouts = type(cEditMode) == "table" and type(cEditMode.GetLayouts) or nil,
    }
    if type(cEditMode) == "table" and type(cEditMode.GetLayouts) == "function" then
        local ok, layouts = call(cEditMode.GetLayouts)
        record.cEditMode.getLayoutsOk = ok
        if type(layouts) == "table" then
            record.cEditMode.activeLayout = layouts.activeLayout
            record.cEditMode.layoutNames = {}
            if type(layouts.layouts) == "table" then
                for index, layout in ipairs(layouts.layouts) do
                    record.cEditMode.layoutNames[index] = type(layout) == "table" and layout.layoutName or nil
                end
            end
        else
            record.cEditMode.layoutsValue = layouts
        end
    end

    db.last = record
    table.insert(db.history, record)
    while #db.history > 20 do
        table.remove(db.history, 1)
    end
    print(addonName .. ": captured " .. record.reason)
    return record
end

local frame = CreateFrame("Frame")
for _, event in ipairs({
    "PLAYER_LOGIN",
    "PLAYER_ENTERING_WORLD",
    "EDIT_MODE_LAYOUTS_UPDATED",
    "PLAYER_LOGOUT",
}) do
    frame:RegisterEvent(event)
end

frame:SetScript("OnEvent", function(_, event)
    snapshot(event)
end)

SLASH_EDITMODELAYOUTPROBE1 = "/emprobe"
SlashCmdList.EDITMODELAYOUTPROBE = function()
    snapshot("slash")
end
