-- Wrap Elune's hooksecurefunc and issecurevariable to accept userdata
-- (FrameRef) as the table argument.
--
-- Elune's C implementations use lua_istable() which rejects userdata.
-- Our frames are userdata, so when the table-arg form fails we proxy
-- through the frame's per-instance env table.
local _elune_hooksecurefunc = hooksecurefunc
local _elune_issecurevariable = issecurevariable

local function hooksecurefunc_wrapper(tblOrName, nameOrHook, hookOrNil)
    if hookOrNil == nil then
        -- 2-arg form: hooksecurefunc("name", hookfn) on global table
        return _elune_hooksecurefunc(tblOrName, nameOrHook)
    end
    -- 3-arg form: hooksecurefunc(obj, "method", hookfn)
    local ok = pcall(_elune_hooksecurefunc, tblOrName, nameOrHook, hookOrNil)
    if ok then return end
    -- Elune rejected the object (userdata) — proxy through env table
    local obj, name, hook = tblOrName, nameOrHook, hookOrNil
    local orig = obj[name]
    if type(orig) ~= "function" then
        error("hooksecurefunc(): " .. tostring(name) .. " is not a function")
    end
    local env = debug.getfenv(obj)
    local envt = env and env[1]
    if not envt then
        envt = {}
        if env then env[1] = envt else debug.setfenv(obj, { envt }) end
    end
    envt[name] = function(...)
        local results = { orig(...) }
        hook(...)
        return unpack(results)
    end
end

hooksecurefunc = hooksecurefunc_wrapper

local function issecurevariable_wrapper(tblOrName, nameOrNil)
    if nameOrNil == nil then
        -- 1-arg form: issecurevariable("globalName") on global table
        return _elune_issecurevariable(tblOrName)
    end
    -- 2-arg form: issecurevariable(obj, "key")
    local ok, secure, taintSource = pcall(_elune_issecurevariable, tblOrName, nameOrNil)
    if ok then return secure, taintSource end
    -- Elune rejected the object (userdata) — check env table for taint
    local env = debug.getfenv(tblOrName)
    local envt = env and env[1]
    if envt and rawget(envt, nameOrNil) ~= nil then
        -- Key exists in per-instance env table (set by addon code = tainted)
        local taint = debug.getobjecttaint(rawget(envt, nameOrNil))
        if taint then return false, taint end
    end
    -- Native method (registered in Rust) or not overridden = secure
    return true, nil
end

issecurevariable = issecurevariable_wrapper
