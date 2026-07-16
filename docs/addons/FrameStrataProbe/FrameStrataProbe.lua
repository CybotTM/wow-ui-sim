local addonName = ...

local function snapshot(frame)
    if not frame then
        return { missing = true }
    end

    local hasFixedFrameStrata
    if type(frame.HasFixedFrameStrata) == "function" then
        hasFixedFrameStrata = frame:HasFixedFrameStrata()
    end

    return {
        name = frame:GetName(),
        parentName = frame:GetParent() and frame:GetParent():GetName() or nil,
        frameStrata = frame:GetFrameStrata(),
        hasFixedFrameStrata = hasFixedFrameStrata,
    }
end

local function snapshotCreationGroup()
    return {
        parent = snapshot(FrameStrataProbeCreationParent),
        parentChild = snapshot(FrameStrataProbeCreationParentChild),
        literalChild = snapshot(FrameStrataProbeCreationLiteralChild),
    }
end

local function snapshotParentSetGroup()
    return {
        parent = snapshot(FrameStrataProbeParentSetParent),
        parentChild = snapshot(FrameStrataProbeParentSetParentChild),
        defaultChild = snapshot(FrameStrataProbeParentSetDefaultChild),
        explicitChild = snapshot(FrameStrataProbeParentSetExplicitChild),
        parentGrandchild = snapshot(FrameStrataProbeParentSetParentGrandchild),
        explicitGrandchild = snapshot(FrameStrataProbeParentSetExplicitGrandchild),
    }
end

local function snapshotReparentGroup()
    return {
        highParent = snapshot(FrameStrataProbeReparentHigh),
        lowParent = snapshot(FrameStrataProbeReparentLow),
        parentChild = snapshot(FrameStrataProbeReparentParentChild),
        defaultChild = snapshot(FrameStrataProbeReparentDefaultChild),
        explicitChild = snapshot(FrameStrataProbeReparentExplicitChild),
        parentGrandchild = snapshot(FrameStrataProbeReparentParentGrandchild),
        explicitGrandchild = snapshot(FrameStrataProbeReparentExplicitGrandchild),
    }
end

local function runProbe()
    local db = {
        addonName = addonName,
        build = { GetBuildInfo() },
        capturedAt = date("%Y-%m-%dT%H:%M:%S"),
        creationOnLoad = {
            parentChild = FrameStrataProbeParentChildOnLoad,
            literalChild = FrameStrataProbeLiteralChildOnLoad,
        },
        creationAtPlayerLogin = snapshotCreationGroup(),
        templateActualParent = snapshot(FrameStrataProbeTemplateActualParent),
        templateBase = snapshot(FrameStrataProbeTemplateBase),
        templateParent = snapshot(FrameStrataProbeTemplateParent),
        templateDerivedLow = snapshot(FrameStrataProbeTemplateDerivedLow),
        parentSetBefore = snapshotParentSetGroup(),
        reparentBefore = snapshotReparentGroup(),
    }

    FrameStrataProbeParentSetParent:SetFrameStrata("LOW")
    db.parentSetAfterLow = snapshotParentSetGroup()

    FrameStrataProbeReparentParentChild:SetParent(FrameStrataProbeReparentLow)
    FrameStrataProbeReparentDefaultChild:SetParent(FrameStrataProbeReparentLow)
    FrameStrataProbeReparentExplicitChild:SetParent(FrameStrataProbeReparentLow)
    db.reparentAfterSetParentLow = snapshotReparentGroup()

    FrameStrataProbeDB = db
    print("FrameStrataProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
