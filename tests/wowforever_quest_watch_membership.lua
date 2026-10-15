-- Explicit simulator policies, not native-client parity.
-- Defines cases only; Rust invokes each case in a fresh environment.
WatchMembershipTests = {}
local tests = WatchMembershipTests

local function pack(...)
    return {n = select('#', ...), ...}
end

local function assertOne(actual, expected, context)
    assert(actual.n == 1, context .. ': expected exactly one result')
    assert(actual[1] == expected, context .. ': unexpected result')
end

function tests.assertProjection(expected)
    assertOne(pack(C_QuestLog.GetNumQuestWatches()), #expected, 'CAPI count')
    assertOne(pack(GetNumQuestWatches()), #expected, 'global count')
    for index, questID in ipairs(expected) do
        assertOne(pack(C_QuestLog.GetQuestIDForQuestWatchIndex(index)), questID,
            'watch index ' .. index)
        local logIndex = C_QuestLog.GetLogIndexForQuestID(questID)
        local info = C_QuestLog.GetInfo(logIndex)
        assert(info.questID == questID and info.questLogIndex == logIndex)
        assert(not info.isHeader and not info.isWorldQuest)
        assert(IsQuestWatched(logIndex) == true, 'legacy query uses log index')
        assert(C_QuestLog.IsOnQuest(questID))
        assert(C_QuestLog.GetQuestDifficultyLevel(questID) == info.level)
        assert(info.difficultyLevel == info.level)
    end
    assert(select('#', C_QuestLog.GetQuestIDForQuestWatchIndex(#expected + 1)) == 0)
end

function tests.assertUnwatched(questID)
    assertOne(pack(C_QuestLog.GetQuestWatchType(questID)), nil, 'absent watch type')
    local logIndex = C_QuestLog.GetLogIndexForQuestID(questID)
    if logIndex then
        assert(IsQuestWatched(logIndex) == false, 'rich row is not watch membership')
    end
end

local function observeEvents()
    local events = {}
    local listener = CreateFrame('Frame')
    listener:RegisterEvent('QUEST_WATCH_LIST_CHANGED')
    listener:SetScript('OnEvent', function(self, event, ...)
        local payload = pack(...)
        -- Protected dispatch may swallow errors. Save them and assert outside it.
        local ok, snapshot = pcall(function()
            local questID = payload[1]
            local logIndex = C_QuestLog.GetLogIndexForQuestID(questID)
            local ids = {}
            for index = 1, C_QuestLog.GetNumQuestWatches() do
                ids[index] = C_QuestLog.GetQuestIDForQuestWatchIndex(index)
            end
            return {
                count = C_QuestLog.GetNumQuestWatches(),
                globalCount = GetNumQuestWatches(),
                ids = ids,
                watchType = pack(C_QuestLog.GetQuestWatchType(questID)),
                watched = IsQuestWatched(logIndex),
            }
        end)
        events[#events + 1] = {
            sameFrame = self == listener, name = event, payload = payload,
            ok = ok, snapshot = snapshot,
        }
    end)
    return events, listener
end

local function assertEvent(record, questID, added, expected)
    assert(record, 'transition must dispatch synchronously before return')
    assert(record.sameFrame and record.name == 'QUEST_WATCH_LIST_CHANGED')
    assert(record.payload.n == 2, 'simulator transition supplies ID and added flag')
    assert(record.payload[1] == questID and record.payload[2] == added)
    assert(record.ok, 'callback post-state query failed: ' .. tostring(record.snapshot))
    local snapshot = record.snapshot
    assert(snapshot.count == #expected and snapshot.globalCount == #expected)
    assert(#snapshot.ids == #expected)
    for index, expectedID in ipairs(expected) do
        assert(snapshot.ids[index] == expectedID, 'callback observed stale membership')
    end
    local expectedType
    if added then expectedType = Enum.QuestWatchType.Manual end
    assertOne(snapshot.watchType, expectedType, 'callback watch type')
    assert(snapshot.watched == added, 'callback observed stale legacy membership')
end

function tests.defaults()
    for _, name in ipairs({'AddQuestWatch', 'RemoveQuestWatch', 'GetQuestWatchType'}) do
        assert(type(rawget(C_QuestLog, name)) == 'function', name .. ' raw registration')
    end
    assert(type(rawget(_G, 'IsQuestWatched')) == 'function')
    assert(Enum.QuestWatchType.Automatic == 0)
    assert(Enum.QuestWatchType.Manual == 1)
    tests.assertProjection({80000, 80001})
    for _, questID in ipairs({80000, 80001}) do
        assertOne(pack(C_QuestLog.GetQuestWatchType(questID)), 1, 'seeded Manual watch')
    end
    for _, questID in ipairs({80002, 90001, 90101, 456789}) do
        tests.assertUnwatched(questID)
    end
    assert(C_QuestLog.GetLogIndexForQuestID(80002) == nil)
    assert(C_QuestLog.GetQuestDifficultyLevel(80000) == 80)
    assert(C_QuestLog.GetQuestDifficultyLevel(80001) == 80)
end

function tests.transitions()
    tests.assertProjection({80000, 80001})
    local events, listener = observeEvents()
    assertOne(pack(C_QuestLog.AddQuestWatch(80000)), false, 'duplicate seed')
    assert(#events == 0, 'duplicate Add must not emit')
    assertOne(pack(C_QuestLog.RemoveQuestWatch(80000)), true, 'actual Remove')
    assert(#events == 1)
    assertEvent(events[1], 80000, false, {80001})
    tests.assertUnwatched(80000)
    tests.assertProjection({80001})
    assertOne(pack(C_QuestLog.RemoveQuestWatch(80000)), false, 'duplicate Remove')
    assert(#events == 1, 'absent Remove must not emit')
    assertOne(pack(C_QuestLog.AddQuestWatch(80000)), true, 'actual Add')
    assert(#events == 2)
    assertEvent(events[2], 80000, true, {80000, 80001})
    tests.assertProjection({80000, 80001})
    assertOne(pack(C_QuestLog.AddQuestWatch(80000)), false, 'duplicate Add')
    assertOne(pack(C_QuestLog.GetQuestWatchType(80000)), 1, 'duplicate preserves Manual')
    assert(#events == 2)
    listener:UnregisterEvent('QUEST_WATCH_LIST_CHANGED')
end

function tests.invalidAndWorld()
    local events, listener = observeEvents()
    local invalid = {'80000', false, {}, function() end, 0/0, math.huge,
        -math.huge, 80000.5, 2147483648, -2147483649}
    for _, name in ipairs({'AddQuestWatch', 'RemoveQuestWatch', 'GetQuestWatchType'}) do
        local method = C_QuestLog[name]
        for _, value in ipairs(invalid) do
            local ok, message = pcall(method, value)
            assert(not ok and type(message) == 'string', name .. ' must reject invalid ID')
            assert(string.find(message, name, 1, true), name .. ' error needs API context')
        end
        for _, args in ipairs({{n = 0}, {n = 1}}) do
            local ok, message = pcall(function() return method(unpack(args, 1, args.n)) end)
            assert(not ok and type(message) == 'string', name .. ' missing/nil ID')
            assert(string.find(message, name, 1, true), name .. ' missing/nil error context')
        end
    end
    -- Signed-i32 boundary values are valid IDs, but have no current rich row.
    for _, questID in ipairs({-2147483648, 2147483647, 0, -1, 80002, 456789}) do
        assertOne(pack(C_QuestLog.AddQuestWatch(questID)), false, 'unknown Add')
        assertOne(pack(C_QuestLog.RemoveQuestWatch(questID)), false, 'absent Remove')
        tests.assertUnwatched(questID)
    end
    for _, questID in ipairs({90001, 90101}) do
        local info = C_QuestLog.GetInfo(C_QuestLog.GetLogIndexForQuestID(questID))
        assert(info.isWorldQuest, 'world exclusion control must be a rich world row')
        assertOne(pack(C_QuestLog.AddQuestWatch(questID)), false, 'reject world Add')
        assertOne(pack(C_QuestLog.RemoveQuestWatch(questID)), false, 'world has no ordinary metadata')
        tests.assertUnwatched(questID)
        assert(C_QuestLog.IsOnQuest(questID), 'world row remains in rich log')
        assert(C_QuestLog.GetQuestDifficultyLevel(questID) == info.level)
    end
    assert(#events == 0, 'invalid/unknown/world calls must not dispatch watch changes')
    tests.assertProjection({80000, 80001})
    listener:UnregisterEvent('QUEST_WATCH_LIST_CHANGED')
end

function tests.automaticDuplicate()
    tests.assertProjection({80000, 80001})
    assertOne(pack(C_QuestLog.GetQuestWatchType(80000)), 0, 'supplied Automatic metadata')
    local events, listener = observeEvents()
    assertOne(pack(C_QuestLog.AddQuestWatch(80000)), false, 'duplicate Automatic Add')
    assertOne(pack(C_QuestLog.GetQuestWatchType(80000)), 0, 'must not demote Automatic')
    assert(#events == 0)
    assertOne(pack(C_QuestLog.RemoveQuestWatch(80000)), true, 'remove Automatic entry')
    assert(#events == 1)
    assertEvent(events[1], 80000, false, {80001})
    assertOne(pack(C_QuestLog.AddQuestWatch(80000)), true, 're-add uses Manual policy')
    assert(#events == 2)
    assertEvent(events[2], 80000, true, {80000, 80001})
    listener:UnregisterEvent('QUEST_WATCH_LIST_CHANGED')
end
