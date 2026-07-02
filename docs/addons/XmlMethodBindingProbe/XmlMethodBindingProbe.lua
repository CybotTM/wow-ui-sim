local addonName = ...

local function copyArray(values)
    local copy = {}
    if type(values) ~= "table" then
        return copy
    end

    for index, value in ipairs(values) do
        copy[index] = value
    end
    return copy
end

local function summarize()
    local log = XmlMethodBindingProbeLog or {}
    local siblingMutation = copyArray(log.siblingMutation)

    return {
        addonName = addonName,
        build = { GetBuildInfo() },
        siblingMutation = {
            order = siblingMutation,
            joined = table.concat(siblingMutation, ","),
            expected = "load,hide2,hide3",
        },
        setScriptObjectTable = log.setScriptObjectTable,
        inheritedMethod = {
            value = log.inheritedMethod,
            expected = "override",
        },
        inheritedScriptText = {
            value = log.inheritedScriptText,
            expected = "override",
            notes = "Inline script text control: body calls self:Foo() when OnLoad runs.",
        },
    }
end

local function runProbe()
    XmlMethodBindingProbeDB = summarize()
    print("XmlMethodBindingProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
