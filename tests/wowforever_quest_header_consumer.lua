-- UNEXECUTED candidate. Only the unrelated refresh tail is replaced.
-- QuestLogMixin and QuestUtil/QuestUtils functions come from entire cached files.
WatchHeaderConsumerTests = {}
local tests = WatchHeaderConsumerTests

local function assertIDs(actual, expected)
    assert(#actual == #expected, 'unexpected watch projection length')
    for index, id in ipairs(expected) do
        assert(actual[index] == id, 'unexpected watch ID at index ' .. index)
    end
end

local function readWatchIDs()
    local ids = {}
    for index = 1, C_QuestLog.GetNumQuestWatches() do
        ids[index] = C_QuestLog.GetQuestIDForQuestWatchIndex(index)
    end
    return ids
end

local function assertWatches(expected)
    assertIDs(readWatchIDs(), expected)
    assert(GetNumQuestWatches() == #expected)
    for _, id in ipairs(expected) do
        assert(QuestUtils_IsQuestWatched(id))
        assert(IsQuestWatched(C_QuestLog.GetLogIndexForQuestID(id)))
    end
    for _, id in ipairs({90001, 90101}) do
        local index = C_QuestLog.GetLogIndexForQuestID(id)
        local info = C_QuestLog.GetInfo(index)
        assert(info.questID == id and info.isWorldQuest and not info.isHeader)
        assert(C_QuestLog.GetQuestWatchType(id) == nil)
        assert(not QuestUtils_IsQuestWatched(id) and not IsQuestWatched(index))
    end
end

local function assertRows(expected)
    local shown, quests = C_QuestLog.GetNumQuestLogEntries()
    assert(shown == #expected + 1 and quests == #expected)
    local header = C_QuestLog.GetInfo(1)
    assert(header.isHeader and header.questLogIndex == 1)
    for index, id in ipairs(expected) do
        local row = C_QuestLog.GetInfo(index + 1)
        assert(not row.isHeader and row.questID == id)
        assert(row.questLogIndex == index + 1)
        assert(C_QuestLog.GetLogIndexForQuestID(id) == index + 1)
    end
    assert(C_QuestLog.GetInfo(shown + 1) == nil)
end

local function observeConsumer()
    assert(type(QuestLogMixin.SetHeaderQuestsTracked) == 'function')
    assert(type(QuestUtils_IsQuestWatched) == 'function')
    assert(type(QuestUtil.CanRemoveQuestWatch) == 'function')
    local context = {
        receiver = {ignoreQuestWatchListChanged = false},
        events = {}, refreshes = {}, inCall = false,
    }
    local listener = CreateFrame('Frame')
    context.listener = listener
    listener:RegisterEvent('QUEST_WATCH_LIST_CHANGED')
    listener:SetScript('OnEvent', function(self, event, ...)
        local record = {
            sameFrame = self == listener, name = event,
            arity = select('#', ...), id = select(1, ...), added = select(2, ...),
            ignored = context.receiver.ignoreQuestWatchListChanged,
            inCall = context.inCall, refreshCount = #context.refreshes,
        }
        -- Dispatch can protect callback errors. Retain errors for outer assertions.
        record.ok, record.snapshot = pcall(function()
            local index = C_QuestLog.GetLogIndexForQuestID(record.id)
            return {
                ids = readWatchIDs(), count = GetNumQuestWatches(),
                watchType = C_QuestLog.GetQuestWatchType(record.id),
                watched = QuestUtils_IsQuestWatched(record.id),
                legacyWatched = IsQuestWatched(index),
                worldOne = QuestUtils_IsQuestWatched(90001),
                worldTwo = QuestUtils_IsQuestWatched(90101),
            }
        end)
        context.events[#context.events + 1] = record
    end)
    -- Permitted seam: observe the unrelated unconditional UI-refresh tail.
    -- Does not exercise QuestMapFrame_UpdateAll or the actual frame's OnEvent.
    QuestMapFrame_UpdateAll = function(...)
        context.refreshes[#context.refreshes + 1] = {
            ignored = context.receiver.ignoreQuestWatchListChanged,
            eventCount = #context.events, inCall = context.inCall,
            arity = select('#', ...), ids = readWatchIDs(),
        }
    end
    return context
end

local function invoke(context, tracked)
    local previousRefreshes = #context.refreshes
    context.inCall = true
    -- Dot invocation passes exactly one receiver to the actual colon-defined method.
    QuestLogMixin.SetHeaderQuestsTracked(context.receiver, 1, tracked)
    context.inCall = false
    assert(context.receiver.ignoreQuestWatchListChanged == false)
    assert(#context.refreshes == previousRefreshes + 1, 'exactly one refresh per call')
    local refresh = context.refreshes[#context.refreshes]
    assert(refresh.ignored == false, 'flag must be reset before refresh')
    assert(refresh.inCall and refresh.arity == 0)
    assert(refresh.eventCount == #context.events, 'refresh follows all synchronous events')
    assertIDs(refresh.ids, readWatchIDs())
end

local function assertEvent(context, index, id, added, expected, refreshCount)
    local record = context.events[index]
    assert(record, 'missing synchronous event')
    assert(record.sameFrame and record.name == 'QUEST_WATCH_LIST_CHANGED')
    assert(record.arity == 2 and record.id == id and record.added == added)
    assert(record.inCall and record.ignored == true, 'event must see receiver ignore flag set')
    assert(record.refreshCount == refreshCount, 'event must precede this call refresh')
    assert(record.ok, 'event post-state query failed: ' .. tostring(record.snapshot))
    local snapshot = record.snapshot
    assertIDs(snapshot.ids, expected)
    assert(snapshot.count == #expected)
    assert(snapshot.watched == added and snapshot.legacyWatched == added)
    local watchType
    if added then watchType = Enum.QuestWatchType.Manual end
    assert(snapshot.watchType == watchType)
    assert(not snapshot.worldOne and not snapshot.worldTwo)
end

function tests.seededRoundTrip()
    assertRows({80000, 80001, 90001, 90101})
    assertWatches({80000, 80001})
    assert(not C_PlayerInfo.IsPlayerNPERestricted() and QuestUtil.CanRemoveQuestWatch())
    local context = observeConsumer()
    invoke(context, false)
    assert(#context.events == 2)
    assertEvent(context, 1, 80000, false, {80001}, 0)
    assertEvent(context, 2, 80001, false, {}, 0)
    assertWatches({})
    invoke(context, false)
    assert(#context.events == 2, 'duplicate untrack emits nothing')
    invoke(context, true)
    assert(#context.events == 4)
    assertEvent(context, 3, 80000, true, {80000}, 2)
    assertEvent(context, 4, 80001, true, {80000, 80001}, 2)
    assertWatches({80000, 80001})
    invoke(context, true)
    assert(#context.events == 4, 'duplicate track and world rows emit nothing')
    assert(#context.refreshes == 4)
    context.listener:UnregisterEvent('QUEST_WATCH_LIST_CHANGED')
end

function tests.mixedLiveRows()
    assertRows({80000, 90001, 80001, 90101, 456792})
    local supplied = C_QuestLog.GetInfo(6)
    assert(supplied.title == 'Supplied ordinary row after world rows')
    assert(supplied.level == 29 and not supplied.isWorldQuest)
    assert(C_QuestLog.GetQuestWatchType(80000) == Enum.QuestWatchType.Automatic)
    assert(C_QuestLog.RemoveQuestWatch(80001) == true)
    assertWatches({80000})
    assert(not QuestUtils_IsQuestWatched(80001) and not QuestUtils_IsQuestWatched(456792))
    local context = observeConsumer()
    invoke(context, true)
    assert(#context.events == 2)
    assertEvent(context, 1, 80001, true, {80000, 80001}, 0)
    assertEvent(context, 2, 456792, true, {80000, 80001, 456792}, 0)
    assertWatches({80000, 80001, 456792})
    assert(C_QuestLog.GetQuestWatchType(80000) == Enum.QuestWatchType.Automatic)
    invoke(context, true)
    assert(#context.events == 2, 'existing and world rows must not emit changes')
    assert(C_QuestLog.GetQuestWatchType(80000) == Enum.QuestWatchType.Automatic)
    invoke(context, false)
    assert(#context.events == 5)
    assertEvent(context, 3, 80000, false, {80001, 456792}, 2)
    assertEvent(context, 4, 80001, false, {456792}, 2)
    assertEvent(context, 5, 456792, false, {}, 2)
    assertWatches({})
    assert(#context.refreshes == 3)
    context.listener:UnregisterEvent('QUEST_WATCH_LIST_CHANGED')
end

function tests.restrictedUntrack()
    assertRows({80000, 80001, 90001, 90101})
    assert(C_PlayerInfo.IsPlayerNPERestricted() and not QuestUtil.CanRemoveQuestWatch())
    local context = observeConsumer()
    tests.restrictedContext = context
    invoke(context, false)
    assertWatches({80000, 80001})
    assert(#context.events == 0 and #context.refreshes == 1)
end

function tests.unrestrictedUntrack()
    assert(not C_PlayerInfo.IsPlayerNPERestricted() and QuestUtil.CanRemoveQuestWatch())
    local context = tests.restrictedContext
    invoke(context, false)
    assert(#context.events == 2 and #context.refreshes == 2)
    assertEvent(context, 1, 80000, false, {80001}, 1)
    assertEvent(context, 2, 80001, false, {}, 1)
    assertWatches({})
    context.listener:UnregisterEvent('QUEST_WATCH_LIST_CHANGED')
end
