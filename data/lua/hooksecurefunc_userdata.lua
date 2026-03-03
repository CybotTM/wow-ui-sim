-- Wrap Elune's hooksecurefunc to accept userdata (FrameRef) as the table arg.
--
-- Elune's C implementation uses lua_istable() which rejects userdata, then
-- lua_rawset() which also fails on userdata. Our frames are userdata, so when
-- the 3-arg form fails we proxy through the frame's per-instance env table.
local _elune_hooksecurefunc = hooksecurefunc

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
