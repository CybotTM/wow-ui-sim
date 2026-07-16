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

local function snapshotCascadeGroup()
    return {
        parent = snapshot(FrameStrataProbeCascadeParent),
        parentChild = snapshot(FrameStrataProbeCascadeParentChild),
        defaultChild = snapshot(FrameStrataProbeCascadeDefaultChild),
        fixedChild = snapshot(FrameStrataProbeCascadeFixedChild),
        parentGrandchild = snapshot(FrameStrataProbeCascadeParentGrandchild),
        fixedGrandchild = snapshot(FrameStrataProbeCascadeFixedGrandchild),
    }
end

local function snapshotReparentGroup()
    return {
        highParent = snapshot(FrameStrataProbeReparentHigh),
        lowParent = snapshot(FrameStrataProbeReparentLow),
        parentChild = snapshot(FrameStrataProbeReparentParentChild),
        defaultChild = snapshot(FrameStrataProbeReparentDefaultChild),
        fixedChild = snapshot(FrameStrataProbeReparentFixedChild),
        parentGrandchild = snapshot(FrameStrataProbeReparentParentGrandchild),
        fixedGrandchild = snapshot(FrameStrataProbeReparentFixedGrandchild),
    }
end

local function runProbe()
    local db = {
        addonName = addonName,
        build = { GetBuildInfo() },
        capturedAt = date("%Y-%m-%dT%H:%M:%S"),
        templateBase = snapshot(FrameStrataProbeTemplateBase),
        templateParent = snapshot(FrameStrataProbeTemplateParent),
        cascadeBefore = snapshotCascadeGroup(),
        reparentBefore = snapshotReparentGroup(),
    }

    FrameStrataProbeCascadeParent:SetFrameStrata("LOW")
    db.cascadeAfterParentSetLow = snapshotCascadeGroup()

    FrameStrataProbeReparentParentChild:SetParent(FrameStrataProbeReparentLow)
    FrameStrataProbeReparentDefaultChild:SetParent(FrameStrataProbeReparentLow)
    FrameStrataProbeReparentFixedChild:SetParent(FrameStrataProbeReparentLow)
    db.reparentAfterSetParentLow = snapshotReparentGroup()

    FrameStrataProbeDB = db
    print("FrameStrataProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
