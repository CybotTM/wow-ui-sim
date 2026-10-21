C_UnitAurasPrivate = C_UnitAurasPrivate or {}

-- Strong, environment-local ownership: registered callbacks survive GC until
-- removed/replaced.
local callbacksByUnit = {}
local raidBossCallback

local function CopyPayload(value, seen)
    if type(value) ~= "table" then return value end
    seen = seen or {}
    if seen[value] then return seen[value] end
    local copy = {}
    seen[value] = copy
    for key, nested in pairs(value) do
        copy[CopyPayload(key, seen)] = CopyPayload(nested, seen)
    end
    local mt = getmetatable(value)
    if type(mt) == "table" then setmetatable(copy, mt) end
    return copy
end

local function Invoke(callback, ...)
    if type(callback) == "function" then
        callback(...)
    else
        callback:Invoke(...)
    end
end

function C_UnitAurasPrivate.AddPrivateAuraUpdateCallback(unitToken, callback)
    local key = tostring(unitToken or "")
    local callbacks = callbacksByUnit[key]
    if callbacks == nil then
        callbacks = {}
        callbacksByUnit[key] = callbacks
    end
    callbacks[#callbacks + 1] = callback
end

-- Simulator-authored injection seam; retain the existing source/info arguments.
function C_UnitAurasPrivate._TriggerPrivateAuraUpdate(unitToken, privateSource, updateInfo)
    local fired = 0
    for _, callback in ipairs(callbacksByUnit[tostring(unitToken or "")] or {}) do
        local info = CopyPayload(updateInfo)
        if type(callback) == "function" then
            -- Retain the existing authored plain-function injection contract.
            callback(privateSource, info)
        else
            -- Original AuraContainer and PrivateAuraUnitWatcher containers
            -- consume updateInfo as their first argument, not privateSource.
            callback:Invoke(info)
        end
        fired = fired + 1
    end
    return fired
end

if modern then
    function C_UnitAurasPrivate.RemovePrivateAuraUpdateCallback(unitToken, callback)
        local key = tostring(unitToken or "")
        local callbacks = callbacksByUnit[key]
        if callbacks == nil then return end
        for index = #callbacks, 1, -1 do
            if callbacks[index] == callback then table.remove(callbacks, index) end
        end
        if #callbacks == 0 then callbacksByUnit[key] = nil end
    end

    function C_UnitAurasPrivate.SetPrivateRaidBossMessageCallback(callback)
        raidBossCallback = callback
    end

    -- Simulator-authored dispatch, not a native public API claim.
    function C_UnitAurasPrivate._TriggerPrivateRaidBossMessage(...)
        if raidBossCallback ~= nil then Invoke(raidBossCallback, ...) end
    end
end
