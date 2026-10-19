-- Authored supplied-state assertions through unchanged original consumers.
-- No native boolean/arity/enum, invalid-index, event-order, or readiness claim.
N2 = {}

local function assertColor(font, expected)
    local r, g, b = font:GetTextColor()
    assert(math.abs(r - expected) < 0.00001, "objective red channel")
    assert(math.abs(g - expected) < 0.00001, "objective green channel")
    assert(math.abs(b - expected) < 0.00001, "objective blue channel")
end

function N2.assertA(done)
    assert(QuestInfoObjectivesFrame:IsShown(), "original supplied objectives frame must show")
    assert(QuestInfoObjective1:IsShown() and QuestInfoObjective2:IsShown())
    assert(QuestInfoObjective1:GetText() ==
        (done and "Wolves slain: 2/2 (" .. COMPLETE .. ")" or "Wolves slain: 1/2"),
        "original objective one text/finished branch")
    assertColor(QuestInfoObjective1, done and 0.2 or 0)
    assert(QuestInfoObjective2:GetText() == "Sealed relic recovered: 1/1 (" .. COMPLETE .. ")")
    assertColor(QuestInfoObjective2, 0.2)
    for i = 3, MAX_OBJECTIVES do
        assert(not _G["QuestInfoObjective" .. i]:IsShown(), "unused objective must hide")
    end
end

function N2.directA(done)
    local frame, last = QuestInfo_ShowObjectives()
    assert(frame == QuestInfoObjectivesFrame, "original consumer returned no supplied objective frame")
    assert(last == QuestInfoObjective2, "original consumer last objective")
    N2.assertA(done)
end

function N2.detailA(done)
    QuestLog_UpdateQuestDetails(false)
    assert(QuestLogDetailScrollFrame:IsShown(), "original detail entry must show scroll")
    assert(QuestInfoTitleHeader:GetText() == "Wrath supplied wolves")
    N2.assertA(done)
    -- Authored summary projection, not native narrative-summary equivalence.
    assert(QuestInfoObjectivesText:GetText() ==
        (done and "Wolves slain: 2/2" or "Wolves slain: 1/2") .. "\nSealed relic recovered: 1/1")
end

function N2.detailB()
    QuestLog_UpdateQuestDetails(false)
    assert(QuestInfoTitleHeader:GetText() == "Wrath supplied delivery")
    assert(QuestInfoObjectivesFrame:IsShown() and QuestInfoObjective1:IsShown())
    assert(QuestInfoObjective1:GetText() == "Crates gathered: 0/3")
    assertColor(QuestInfoObjective1, 0)
    for i = 2, MAX_OBJECTIVES do
        assert(not _G["QuestInfoObjective" .. i]:IsShown(), "shorter quest must hide stale rows")
    end
end

function N2.empty()
    local frame, last = QuestInfo_ShowObjectives()
    assert(frame == nil and last == nil, "empty objectives return no widget")
    assert(not QuestInfoObjectivesFrame:IsShown())
    for i = 1, MAX_OBJECTIVES do
        assert(not _G["QuestInfoObjective" .. i]:IsShown(), "empty snapshot hides stale rows")
    end
    QuestLog_UpdateQuestDetails(false)
    assert(not QuestInfoObjectivesFrame:IsShown())
    assert(QuestInfoObjectivesText:GetText() == nil or QuestInfoObjectivesText:GetText() == "")
end

function N2.assertExplicitRows()
    -- Authored flat-index projection; native header/invalid selector cases PENDING Q4/Q5.
    assert(GetNumQuestLeaderBoards(2) == 2)
    assert(GetNumQuestLeaderBoards(1) == 1)
    local a = GetQuestLogLeaderBoard(1, 2)
    local b = GetQuestLogLeaderBoard(1, 1)
    assert(a == "Wolves slain: 1/2", "reordered explicit rich index must read A")
    assert(b == "Crates gathered: 0/3", "reordered explicit rich index must read B")
    -- No exact arity, serialized finished value, or native type-string assertion here.
end
