local addonName = ...

-- Candidate script handlers to probe on every animation object.
-- The interesting one is OnEvent: no schema (UI.xsd AnimScriptsType) or model
-- (wowless uiobjects) lists it for animations, so we measure whether the live
-- engine actually accepts it.
local HANDLERS = {
    "OnLoad", "OnUpdate", "OnEvent",
    "OnPlay", "OnPause", "OnStop", "OnFinished", "OnLoop",
    "OnShow", "OnHide", "OnEnter", "OnLeave",
}

-- Every Animation subtype creatable via AnimationGroup:CreateAnimation(type).
local ANIM_TYPES = {
    "Alpha", "Translation", "Scale", "Rotation",
    "LineTranslation", "LineScale", "Path", "FlipBook", "VertexColor",
}

local function probeObject(obj)
    local result = { hasScript = {}, setScript = {} }
    for _, name in ipairs(HANDLERS) do
        -- HasScript: canonical "is this handler supported on this object".
        local okHas, has = pcall(obj.HasScript, obj, name)
        result.hasScript[name] = okHas and (has and true or false) or "ERR"

        -- SetScript cross-check: does the engine accept binding it at all?
        local okSet = pcall(obj.SetScript, obj, name, function() end)
        result.setScript[name] = okSet and true or false
        if okSet then
            pcall(obj.SetScript, obj, name, nil)
        end
    end
    return result
end

local function runProbe()
    local frame = CreateFrame("Frame", nil, UIParent)
    local group = frame:CreateAnimationGroup()

    local probe = {
        build = { GetBuildInfo() },
        objects = {},
    }

    probe.objects["Frame"] = probeObject(frame)
    probe.objects["AnimationGroup"] = probeObject(group)

    for _, animType in ipairs(ANIM_TYPES) do
        local okCreate, anim = pcall(group.CreateAnimation, group, animType)
        if okCreate and anim then
            local entry = probeObject(anim)
            entry.objectType = anim:GetObjectType()
            probe.objects[animType] = entry
        else
            probe.objects[animType] = { error = "CreateAnimation failed" }
        end
    end

    AnimScriptProbeDB = probe
    return probe
end

local function summarize(probe)
    print(string.format("%s: interface %s", addonName, tostring(probe.build[4])))
    -- Focus line: does the engine accept OnEvent on a plain Animation?
    for _, animType in ipairs({ "Alpha", "AnimationGroup" }) do
        local o = probe.objects[animType]
        if o and o.hasScript then
            print(string.format(
                "  %s: HasScript(OnEvent)=%s SetScript(OnEvent)=%s | OnLoop=%s OnPause=%s",
                animType,
                tostring(o.hasScript["OnEvent"]),
                tostring(o.setScript["OnEvent"]),
                tostring(o.hasScript["OnLoop"]),
                tostring(o.hasScript["OnPause"])
            ))
        end
    end
    print("  Full matrix saved to AnimScriptProbeDB (see SavedVariables/AnimScriptProbe.lua)")
end

local loader = CreateFrame("Frame")
loader:RegisterEvent("PLAYER_LOGIN")
loader:SetScript("OnEvent", function()
    summarize(runProbe())
end)

SLASH_ANIMSCRIPTPROBE1 = "/animprobe"
SlashCmdList.ANIMSCRIPTPROBE = function()
    summarize(runProbe())
end
