local addonName = ...

local function snapshot(frame)
    if not frame then
        return { missing = true }
    end
    return {
        frameLevel = frame:GetFrameLevel(),
        hasFixedFrameLevel = type(frame.HasFixedFrameLevel) == "function" and frame:HasFixedFrameLevel() or nil,
        isUsingParentLevel = type(frame.IsUsingParentLevel) == "function" and frame:IsUsingParentLevel() or nil,
        frameStrata = frame:GetFrameStrata(),
    }
end

local function runProbe()
    local parent = XmlFrameLevelProbeParent

    local db = {
        addonName = addonName,
        build = { GetBuildInfo() },
        capturedAt = date("%Y-%m-%dT%H:%M:%S"),
        load = {
            parent = snapshot(parent),
            childPlain = snapshot(XmlFrameLevelProbeChildPlain),
            childFixed = snapshot(XmlFrameLevelProbeChildFixed),
            childUPL = snapshot(XmlFrameLevelProbeChildUPL),
            childBare = snapshot(XmlFrameLevelProbeChildBare),
            childTemplated = snapshot(XmlFrameLevelProbeChildTemplated),
        },
    }

    -- Does raising the parent's level shift unfixed XML children (offset
    -- semantics) or leave them at their absolute level?
    parent:SetFrameLevel(60)
    db.afterParentSetLevel60 = {
        parent = snapshot(parent),
        childPlain = snapshot(XmlFrameLevelProbeChildPlain),
        childFixed = snapshot(XmlFrameLevelProbeChildFixed),
        childUPL = snapshot(XmlFrameLevelProbeChildUPL),
        childBare = snapshot(XmlFrameLevelProbeChildBare),
        childTemplated = snapshot(XmlFrameLevelProbeChildTemplated),
    }

    -- Does Lua SetFrameLevel implicitly fix the level across SetParent?
    local f = CreateFrame("Frame", nil, UIParent)
    local h = CreateFrame("Frame", nil, UIParent)
    h:SetFrameLevel(50)
    f:SetFrameLevel(5)
    local levelAfterSet = f:GetFrameLevel()
    local fixedAfterSet = type(f.HasFixedFrameLevel) == "function" and f:HasFixedFrameLevel() or nil
    f:SetParent(h)
    db.luaSetFrameLevel = {
        levelAfterSet = levelAfterSet,
        hasFixedAfterSet = fixedAfterSet,
        levelAfterReparentToLevel50Parent = f:GetFrameLevel(),
    }

    XmlFrameLevelProbeDB = db
    print("XmlFrameLevelProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
