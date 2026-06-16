local addonName = ...

-- Settles three things wow-ui-sim cannot answer about itself, against the live
-- client:
--   1. Does C_Timer.NewTicker accept a C_FunctionContainers callback object
--      (not a raw function) as its callback?
--   2. Does NewTicker return the *same* object you passed in (so it can be fed
--      back into another NewTicker), or an opaque handle?
--   3. If one callback object backs two tickers, is the per-ticker iteration
--      count independent ("state not shared")?
--
-- The async scenarios increment counters that live inside the DB table, so the
-- final tallies are whatever they are when SavedVariables flush (logout/reload).

local DELAY_LONG = 3600 -- park-and-cancel timers used only for synchronous probes

local function safeCall(fn, ...)
    local ok, res = pcall(fn, ...)
    if ok then
        return true, res
    end
    return false, tostring(res)
end

local function cancelTicker(obj)
    if type(obj) == "table" and type(obj.Cancel) == "function" then
        pcall(obj.Cancel, obj)
    end
end

-- ── Synchronous facts ────────────────────────────────────────────────────────

local function probeStatic()
    local s = {}
    s.hasFunctionContainers = (C_FunctionContainers ~= nil)
    s.hasCreateCallback = s.hasFunctionContainers
        and type(C_FunctionContainers.CreateCallback) == "function"

    if s.hasCreateCallback then
        local cb = C_FunctionContainers.CreateCallback(function() end)
        s.createCallbackType = type(cb)
        -- Is the container directly callable, or must you :Invoke()?
        s.containerCallableViaParen = (safeCall(function() cb() end))
        s.containerHasInvoke = (type(cb) == "table" and type(cb.Invoke) == "function")

        -- Does NewTicker accept the container as its callback?
        local okNew, obj1 = safeCall(C_Timer.NewTicker, DELAY_LONG, cb, 1)
        s.newTickerAcceptsContainer = okNew
        if okNew then
            s.newTickerReturnType = type(obj1)
            s.tickerEqualsContainer = (obj1 == cb)
            s.tickerCallableViaParen = (safeCall(function() obj1() end))
            cancelTicker(obj1)
        else
            s.newTickerContainerError = obj1
        end
    end

    -- Baseline: plain function callback (what virtually every addon passes).
    local f = function() end
    local okP, objP = safeCall(C_Timer.NewTicker, DELAY_LONG, f, 1)
    s.plainAccepted = okP
    if okP then
        s.plainReturnType = type(objP)
        s.plainTickerEqualsFn = (objP == f)
        cancelTicker(objP)
    else
        s.plainError = objP
    end

    return s
end

-- ── Async scenario A: one FunctionContainer backs two tickers ──────────────────
-- This is the exact "C_Timer state not shared" case: feed the object returned by
-- the first NewTicker straight back in as the second ticker's callback.

local function startSharedContainerScenario(run)
    local s = run.sharedContainer
    if not (C_FunctionContainers and type(C_FunctionContainers.CreateCallback) == "function") then
        s.skipped = "no C_FunctionContainers.CreateCallback"
        return
    end

    local cb = C_FunctionContainers.CreateCallback(function(self)
        s.total = s.total + 1
        if s.firstArgType == nil then
            s.firstArgType = type(self)
        end
    end)

    local okA, obj1 = safeCall(C_Timer.NewTicker, 0.05, cb, 5)
    s.tickerAStarted = okA
    if not okA then
        s.tickerAError = obj1
        return
    end
    s.obj1Type = type(obj1)
    s.obj1EqualsCb = (obj1 == cb)

    -- The crux: pass the returned ticker object back in as a callback.
    local okB, obj2 = safeCall(C_Timer.NewTicker, 0.08, obj1, 3)
    s.tickerBStarted = okB
    if not okB then
        s.tickerBError = obj2
    end
end

-- ── Async scenario B: the same plain function backs two tickers ────────────────
-- Definitely-valid WoW usage. Confirms per-ticker iteration state is independent
-- for the normal case, regardless of the exotic container path above.

local function startPlainFunctionScenario(run)
    local s = run.plainFunction
    local fn = function() s.total = s.total + 1 end
    local okA = safeCall(C_Timer.NewTicker, 0.05, fn, 5)
    local okB = safeCall(C_Timer.NewTicker, 0.08, fn, 3)
    s.tickerAStarted = okA
    s.tickerBStarted = okB
end

-- ── Run + report ───────────────────────────────────────────────────────────────

local function runProbe()
    local run = {
        build = { GetBuildInfo() },
        capturedAt = time(),
        static = probeStatic(),
        sharedContainer = { total = 0, expected = 8 },
        plainFunction = { total = 0, expected = 8 },
    }
    startSharedContainerScenario(run)
    startPlainFunctionScenario(run)
    TimerCallbackProbeDB = run
    return run
end

local function verdict(s)
    if s.skipped then
        return "skipped (" .. s.skipped .. ")"
    end
    if s.tickerAStarted == false then
        return "ticker A rejected: " .. tostring(s.tickerAError)
    end
    if s.tickerBStarted == false then
        return "ticker B rejected: " .. tostring(s.tickerBError)
    end
    if s.total == s.expected then
        return "PASS (" .. s.total .. "/" .. s.expected .. " — state not shared)"
    end
    return "DIVERGES (" .. tostring(s.total) .. "/" .. s.expected .. ")"
end

local function report()
    local run = TimerCallbackProbeDB
    if not run then
        print(addonName .. ": no data yet — run /timerprobe, wait ~1s, then /timerprobe report")
        return
    end
    local st = run.static
    print(string.format("%s: interface %s", addonName, tostring(run.build[4])))
    print(string.format("  CreateCallback -> %s | callable() = %s | :Invoke = %s",
        tostring(st.createCallbackType),
        tostring(st.containerCallableViaParen),
        tostring(st.containerHasInvoke)))
    print(string.format("  NewTicker accepts container = %s%s",
        tostring(st.newTickerAcceptsContainer),
        st.newTickerAcceptsContainer
            and string.format(" | returns %s | == container = %s | callable() = %s",
                tostring(st.newTickerReturnType),
                tostring(st.tickerEqualsContainer),
                tostring(st.tickerCallableViaParen))
            or (" | error: " .. tostring(st.newTickerContainerError))))
    print(string.format("  NewTicker(plain fn) -> %s | == fn = %s",
        tostring(st.plainReturnType), tostring(st.plainTickerEqualsFn)))
    print("  shared-container scenario: " .. verdict(run.sharedContainer))
    print("  plain-function scenario:   " .. verdict(run.plainFunction))
    print("  Full capture in TimerCallbackProbeDB (SavedVariables/TimerCallbackProbe.lua)")
end

local loader = CreateFrame("Frame")
loader:RegisterEvent("PLAYER_LOGIN")
loader:SetScript("OnEvent", function()
    runProbe()
    -- Tickers need ~0.4s to finish; report once they have had time to fire.
    C_Timer.After(1.0, report)
end)

SLASH_TIMERCALLBACKPROBE1 = "/timerprobe"
SlashCmdList.TIMERCALLBACKPROBE = function(msg)
    if msg and msg:lower():match("report") then
        report()
    else
        runProbe()
        print(addonName .. ": started — run '/timerprobe report' in ~1s (or just wait)")
        C_Timer.After(1.0, report)
    end
end
