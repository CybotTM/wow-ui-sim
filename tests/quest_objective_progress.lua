-- Test-owned inputs/observers only. No public API replacement or synthetic dispatch.
QL1 = { id = 990001 }

-- Proposed simulator-only boundary; not currently published or approved.
-- Missing publication leaves baseline unchanged, then Rust asserts real count/value RED.
function QL1.submit(rows, percentage)
    local publish = rawget(A_Admin, "SetQuestObjectives")
    if type(publish) == "function" then
        return publish(QL1.id, rows, percentage)
    end
end

function QL1.supply(fulfilled)
    QL1.expectedKind, QL1.expectedFulfilled = "monster", fulfilled
    return QL1.submit({
        { text = "Wolves: " .. fulfilled .. "/2", type = "monster",
          finished = fulfilled == 2, numFulfilled = fulfilled, numRequired = 2 },
        { text = "Relic: 0/1", type = "item", finished = false,
          numFulfilled = 0, numRequired = 1 },
    })
end

function QL1.assertRows(fulfilled)
    local index = C_QuestLog.GetLogIndexForQuestID(QL1.id)
    assert(C_QuestLog.GetInfo(index).questID == QL1.id, "live rich index")
    assert(C_QuestLog.GetNumQuestObjectives(QL1.id) == 2)
    assert(GetNumQuestLeaderBoards(index) == 2, "legacy count shares rich identity")
    local rows = C_QuestLog.GetQuestObjectives(QL1.id)
    assert(type(rows) == "table" and #rows == 2)
    local expectedText = "Wolves: " .. fulfilled .. "/2"
    local first = rows[1]
    assert(first.text == expectedText and first.type == "monster")
    assert(first.numFulfilled == fulfilled and first.numRequired == 2)
    assert(first.finished == (fulfilled == 2))
    assert(rows[2].text == "Relic: 0/1" and rows[2].type == "item")
    assert(rows[2].numFulfilled == 0 and rows[2].numRequired == 1 and not rows[2].finished)
    for n, row in ipairs(rows) do
        local text, kind, finished = GetQuestLogLeaderBoard(n, index, true)
        assert(text == row.text and kind == row.type and finished == row.finished)
    end
end

function QL1.supplyBar(percentage)
    QL1.expectedKind, QL1.expectedFulfilled = "progressbar", percentage
    return QL1.submit({
        { text = "Supplies", type = "progressbar", finished = false,
          numFulfilled = percentage, numRequired = 100 },
    }, percentage)
end

function QL1.assertBar(percentage)
    local rows = C_QuestLog.GetQuestObjectives(QL1.id)
    assert(#rows == 1 and rows[1].type == "progressbar")
    assert(rows[1].text == "Supplies" and not rows[1].finished)
    assert(rows[1].numFulfilled == percentage and rows[1].numRequired == 100)
    local index = C_QuestLog.GetLogIndexForQuestID(QL1.id)
    assert(GetNumQuestLeaderBoards(index) == 1)
    local text, kind, finished = GetQuestLogLeaderBoard(1, index, true)
    assert(text == "Supplies" and kind == "progressbar" and finished == false)
end

function QL1.installTracker()
    -- Whole cached addon closure, including actual block, line, animation and bar code.
    -- Only explicit presentation geometry/backing inputs; no fake block or sink methods.
    local info = debug.getinfo(QuestObjectiveTracker.DoQuestObjectives, "S")
    assert(info.source:find("Blizzard_QuestObjectiveTracker.lua", 1, true), "original consumer provenance")
    QL1.block = CreateFrame("Frame", nil, UIParent, "ObjectiveTrackerQuestPOIBlockTemplate")
    QL1.block.parentModule = QuestObjectiveTracker
    QL1.block.id = QL1.id
    QL1.block:SetSize(300, 200)
    QL1.block:Init()
    QuestObjectiveTracker.ContentsFrame:SetSize(300, 600)
    print("QL1 original tracker source: " .. info.source .. ":" .. info.linedefined)
end

function QL1.render()
    QL1.block:Reset()
    -- Explicitly invoke the original consumer, not the full automatic layout scheduler.
    -- Quest itself stays incomplete: relic objective remains unfulfilled (QL-2 excluded).
    QuestObjectiveTracker:DoQuestObjectives(QL1.block, false, false, true, true)
end

function QL1.observeTracker()
    QL1.updates, QL1.watches, QL1.criteriaCount = {}, {}, 0
    QL1.observer = CreateFrame("Frame")
    QL1.observer:RegisterEvent("QUEST_LOG_UPDATE")
    QL1.observer:RegisterEvent("QUEST_WATCH_UPDATE")
    QL1.observer:RegisterEvent("QUEST_LOG_CRITERIA_UPDATE")
    QL1.observer:SetScript("OnEvent", function(_, event, ...)
        if event == "QUEST_LOG_CRITERIA_UPDATE" then
            QL1.criteriaCount = QL1.criteriaCount + 1
            return
        end
        if QL1.expectedKind == "progressbar" then
            QL1.assertBar(QL1.expectedFulfilled)
            assert(GetQuestProgressBarPercent(QL1.id) == QL1.expectedFulfilled)
        else
            QL1.assertRows(QL1.expectedFulfilled)
        end
        if event == "QUEST_WATCH_UPDATE" then
            table.insert(QL1.watches, { arity = select("#", ...), id = (...), committed = true })
            return
        end
        local committed = C_QuestLog.IsOnQuest(QL1.id)
            and not C_QuestLog.IsQuestFlaggedCompleted(QL1.id)
        QL1.render()
        local first = QL1.block:GetExistingLine(1)
        local relic = QL1.block:GetExistingLine(2)
        table.insert(QL1.updates, {
            text = first.Text:GetText(), relic = relic and relic.Text:GetText() or "",
            committed = committed,
        })
    end)
end

function QL1.assertWatchPayloads()
    assert(#QL1.watches == #QL1.updates)
    for _, payload in ipairs(QL1.watches) do
        assert(payload.arity == 1 and payload.id == QL1.id and payload.committed, "watch payload is quest ID, not log index")
    end
end

function QL1.supplyCriteria(fulfilled)
    return QL1.submit({
        { text = "Runes", type = "event", finished = false,
          numFulfilled = fulfilled, numRequired = 3, criteriaID = 71 },
    })
end

function QL1.observeCriteria()
    QL1.criteria = {}
    QL1.criteriaObserver = CreateFrame("Frame")
    QL1.criteriaObserver:RegisterEvent("QUEST_LOG_CRITERIA_UPDATE")
    QL1.criteriaObserver:SetScript("OnEvent", function(_, _, ...)
        local id, tree, description, fulfilled, required = ...
        local row = C_QuestLog.GetQuestObjectives(id)[1]
        local index = C_QuestLog.GetLogIndexForQuestID(id)
        local text, kind, finished = GetQuestLogLeaderBoard(1, index, true)
        local committed = row.text == description and row.numFulfilled == fulfilled
            and row.numRequired == required and row.finished == false
            and C_QuestLog.GetNumQuestObjectives(id) == 1
            and GetNumQuestLeaderBoards(index) == 1
            and text == description and kind == "event" and finished == false
        table.insert(QL1.criteria, {
            payload = select("#", ...) .. ":" .. id .. ":" .. tree .. ":" .. description .. ":" .. fulfilled .. ":" .. required,
            committed = committed,
        })
    end)
end
