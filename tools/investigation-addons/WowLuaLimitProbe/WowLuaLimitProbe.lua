local ADDON_NAME = ...

WowLuaLimitProbeDB = WowLuaLimitProbeDB or {}

local DEFAULT_LIMIT = 420
local PREFIX = "|cff66ccffLuaLimitProbe|r"

local function say(message)
    if DEFAULT_CHAT_FRAME then
        DEFAULT_CHAT_FRAME:AddMessage(PREFIX .. " " .. tostring(message))
    else
        print(PREFIX, message)
    end
end

local function join(parts, sep)
    return table.concat(parts, sep or "")
end

local function localNames(prefix, count)
    local names = {}
    for i = 1, count do
        names[i] = prefix .. i
    end
    return join(names, ",")
end

local function chunkLocals(count)
    return "local " .. localNames("v", count) .. "\nreturn true"
end

local function localFunctions(count)
    local lines = {}
    for i = 1, count do
        lines[i] = "local function f" .. i .. "() return " .. i .. " end"
    end
    lines[count + 1] = "return true"
    return join(lines, "\n")
end

local function doBlockLocals(count)
    return "do local " .. localNames("b", count) .. " end\nreturn true"
end

local function repeatedDoBlocks(count)
    local lines = {}
    for i = 1, count do
        lines[i] = "do local b" .. i .. " = " .. i .. " end"
    end
    lines[count + 1] = "return true"
    return join(lines, "\n")
end

local function chunkLocalsPlusFunctions(count)
    local half = math.floor(count / 2)
    local lines = { "local " .. localNames("v", half) }
    for i = half + 1, count do
        lines[#lines + 1] = "local function f" .. i .. "() return " .. i .. " end"
    end
    lines[#lines + 1] = "return true"
    return join(lines, "\n")
end

local PROBES = {
    { key = "chunk_locals", label = "top-level locals", make = chunkLocals },
    { key = "local_functions", label = "top-level local functions", make = localFunctions },
    { key = "do_block_locals", label = "single do-block locals", make = doBlockLocals },
    { key = "repeated_do_blocks", label = "repeated scoped do-block locals", make = repeatedDoBlocks },
    { key = "mixed_chunk", label = "mixed locals plus local functions", make = chunkLocalsPlusFunctions },
}

local function compile(source)
    if loadstring then
        return loadstring(source, ADDON_NAME .. " generated probe")
    end
    return load(source, ADDON_NAME .. " generated probe")
end

local function canCompile(makeSource, count)
    local fn, err = compile(makeSource(count))
    if fn then
        return true
    end
    return false, err
end

local function findLimit(makeSource, maxCount)
    local lo, hi = 0, maxCount
    local lastErr

    while lo < hi do
        local mid = math.floor((lo + hi + 1) / 2)
        local ok, err = canCompile(makeSource, mid)
        if ok then
            lo = mid
        else
            hi = mid - 1
            lastErr = err
        end
    end

    local _, nextErr = canCompile(makeSource, lo + 1)
    return lo, nextErr or lastErr
end

local function run(limit)
    limit = tonumber(limit) or DEFAULT_LIMIT
    local results = {
        addon = ADDON_NAME,
        date = date and date("%Y-%m-%d %H:%M:%S") or nil,
        build = { GetBuildInfo() },
        limit = limit,
        probes = {},
    }

    say("running with max " .. limit)
    for _, probe in ipairs(PROBES) do
        local maxOk, err = findLimit(probe.make, limit)
        results.probes[probe.key] = {
            label = probe.label,
            maxOk = maxOk,
            nextError = err,
        }
        say(probe.label .. ": " .. maxOk .. (err and ("; next error: " .. err) or ""))
    end

    WowLuaLimitProbeDB.last = results
    WowLuaLimitProbeDB.history = WowLuaLimitProbeDB.history or {}
    table.insert(WowLuaLimitProbeDB.history, results)
    say("saved results to WowLuaLimitProbeDB.last")
end

local function dumpLast()
    local last = WowLuaLimitProbeDB and WowLuaLimitProbeDB.last
    if not last then
        say("no results yet; run /llimit")
        return
    end
    say("build: " .. table.concat(last.build or {}, " / "))
    for _, probe in ipairs(PROBES) do
        local result = last.probes and last.probes[probe.key]
        if result then
            say(result.label .. ": " .. tostring(result.maxOk))
        end
    end
end

SLASH_WOWLUALIMITPROBE1 = "/llimit"
SlashCmdList.WOWLUALIMITPROBE = function(msg)
    msg = msg and msg:match("^%s*(.-)%s*$") or ""
    if msg == "dump" then
        dumpLast()
    elseif msg == "clear" then
        WowLuaLimitProbeDB = {}
        say("cleared saved results")
    else
        run(tonumber(msg) or DEFAULT_LIMIT)
    end
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", function()
    say("ready: /llimit [max], /llimit dump, /llimit clear")
end)
