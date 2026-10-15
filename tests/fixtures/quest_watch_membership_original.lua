-- UNEXECUTED. Private main-owned regression, not a producer patch.
-- Run only after current query-only projection is GREEN and execution is authorized.
-- Fresh Forever normal environment; ordinary seeded IDs 80000 and 80001.
assert(rawget(_G, "WatchMutatorRedNext0700") == nil, "preserve previous raw observations")
local report = {events = {}, failures = {}, phase = "setup"}
_G.WatchMutatorRedNext0700 = report

local function pack(...)
    return {n = select("#", ...), ...}
end

local function observe(fn)
    local result = {}
    local ok, err = pcall(function() result.values = pack(fn()) end)
    result.ok = ok
    if not ok then result.error = err end
    return result
end

report.rawPublication = {}
for _, name in ipairs({"AddQuestWatch", "RemoveQuestWatch", "GetQuestWatchType"}) do
    report.rawPublication[name] = rawget(C_QuestLog, name)
end

local function snapshot()
    local state = {ids = {}}
    state.read = observe(function()
        state.count = C_QuestLog.GetNumQuestWatches()
        -- Harness bound only, not a modeled watch-capacity requirement.
        assert(type(state.count) == "number" and state.count >= 0
            and state.count <= 64 and state.count == math.floor(state.count),
            "watch enumeration exceeds bounded harness")
        for index = 1, state.count do
            state.ids[index] = C_QuestLog.GetQuestIDForQuestWatchIndex(index)
        end
        state.logIndex = C_QuestLog.GetLogIndexForQuestID(80000)
        if state.logIndex then state.row = C_QuestLog.GetInfo(state.logIndex) end
        state.onQuest = C_QuestLog.IsOnQuest(80000)
        state.difficulty = C_QuestLog.GetQuestDifficultyLevel(80000)
    end)
    state.watchType = observe(function() return C_QuestLog.GetQuestWatchType(80000) end)
    return state
end

local function contains(ids, id)
    for _, value in pairs(ids) do
        if value == id then return true end
    end
    return false
end

report.before = snapshot()
local listener = CreateFrame("Frame")
listener:RegisterEvent("QUEST_WATCH_LIST_CHANGED")
listener:SetScript("OnEvent", function(_, event, ...)
    -- Protected dispatch can swallow assertions: capture raw data ONLY here.
    report.events[#report.events + 1] = {
        event = event, phase = report.phase, payload = pack(...),
    }
end)

report.phase = "remove"
report.remove = observe(function() return C_QuestLog.RemoveQuestWatch(80000) end)
report.eventsAtRemoveReturn = #report.events
report.afterRemove = snapshot() -- count/state queries OUTSIDE protected callback
report.phase = "add"
report.add = observe(function() return C_QuestLog.AddQuestWatch(80000) end)
report.eventsAtAddReturn = #report.events
report.afterAdd = snapshot() -- count/state queries OUTSIDE protected callback
report.phase = "finished"
listener:UnregisterEvent("QUEST_WATCH_LIST_CHANGED")
listener:SetScript("OnEvent", nil)

-- All raw observations above survive even when acceptance assertions fail below.
local function check(condition, message)
    if not condition then report.failures[#report.failures + 1] = message end
end
local function singleTrue(result)
    return result.ok and result.values.n == 1 and result.values[1] == true
end
local function hasPayload(first, last, added)
    for index = first, last do
        local event = report.events[index]
        if event.event == "QUEST_WATCH_LIST_CHANGED" and event.payload.n == 2
            and event.payload[1] == 80000 and event.payload[2] == added then
            return true
        end
    end
    return false
end

local before, removed, added = report.before, report.afterRemove, report.afterAdd
check(before.read.ok, "initial normal API snapshot must succeed")
check(before.count == 2 and contains(before.ids, 80000) and contains(before.ids, 80001),
    "fresh ordinary seed must contain 80000 and 80001")
check(singleTrue(report.remove), "removing existing watched 80000 must return one boolean true")
check(removed.read.ok, "post-remove normal API snapshot must succeed")
check(type(before.count) == "number" and removed.count == before.count - 1,
    "remove must decrement watch count")
check(not contains(removed.ids, 80000), "removed ID must be absent from watch enumeration")
check(contains(removed.ids, 80001), "other ordinary watched ID must remain")
check(removed.onQuest == true and removed.row and removed.row.questID == 80000
    and removed.row.title == "The Lost Expedition", "remove must retain rich quest row")
check(before.difficulty == 80 and removed.difficulty == before.difficulty,
    "remove must retain supplied quest difficulty")
check(removed.watchType.ok and removed.watchType.values[1] == nil,
    "removed quest watch type must be nil")
check(singleTrue(report.add), "adding back 80000 must return one boolean true")
check(added.read.ok and added.count == before.count, "add back must restore watch count")
check(contains(added.ids, 80000) and contains(added.ids, 80001),
    "add back must restore membership without imposing index order")
check(added.watchType.ok and added.watchType.values[1] == 1,
    "explicit bounded simulator policy: manual AddQuestWatch yields Manual=1")
check(added.onQuest == true and added.row and added.row.questID == 80000
    and added.difficulty == before.difficulty, "add back must preserve rich quest and difficulty")
check(hasPayload(1, report.eventsAtRemoveReturn, false),
    "remove must synchronously capture two-argument payload (80000, false)")
check(hasPayload(report.eventsAtRemoveReturn + 1, report.eventsAtAddReturn, true),
    "add must synchronously capture two-argument payload (80000, true)")
report.passed = #report.failures == 0
assert(report.passed, table.concat(report.failures, "\n"))
