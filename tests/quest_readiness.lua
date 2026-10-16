-- Fixture inputs and observations only; original consumers/APIs remain untouched.
QL2 = { id = 990002, autoCount = 0, completeCount = 0, turnInCount = 0 }

function QL2.objectives(finished)
    local submit = rawget(A_Admin, "SetQuestObjectives")
    if type(submit) == "function" then
        submit(QL2.id, {
            { text = "Wolves: 2/2", type = "monster", finished = true, numFulfilled = 2, numRequired = 2 },
            { text = finished and "Relic: 1/1" or "Relic: 0/1", type = "item", finished = finished,
              numFulfilled = finished and 1 or 0, numRequired = 1 },
        })
    end
end

function QL2.configure(ready, autoComplete)
    local supply = rawget(A_Admin, "SetQuestReadiness")
    if type(supply) == "function" then supply(QL2.id, ready, autoComplete) end
end

function QL2.assertReady()
    assert(C_QuestLog.ReadyForTurnIn(QL2.id))
    assert(C_QuestLog.IsComplete(QL2.id))
    assert(IsQuestComplete(QL2.id))
    assert(not C_QuestLog.IsFailed(QL2.id))
    assert(C_QuestLog.IsOnQuest(QL2.id))
    assert(not C_QuestLog.IsQuestFlaggedCompleted(QL2.id))
end

function QL2.observe()
    QL2.observer = CreateFrame("Frame")
    for _, event in ipairs({ "QUEST_LOG_UPDATE", "QUEST_AUTOCOMPLETE", "QUEST_COMPLETE", "QUEST_TURNED_IN" }) do
        QL2.observer:RegisterEvent(event)
    end
    QL2.observer:SetScript("OnEvent", function(_, event, ...)
        if event == "QUEST_LOG_UPDATE" then
            QL2.committed = C_QuestLog.ReadyForTurnIn(QL2.id)
                and C_QuestLog.IsComplete(QL2.id)
                and C_QuestLog.IsOnQuest(QL2.id)
                and not C_QuestLog.IsQuestFlaggedCompleted(QL2.id)
                and C_GossipInfo.GetActiveQuests()[1].isComplete == true
        elseif event == "QUEST_AUTOCOMPLETE" then
            QL2.autoCount = QL2.autoCount + 1
            QL2.autoPayload = select("#", ...) .. ":" .. (...)
        elseif event == "QUEST_COMPLETE" then
            QL2.completeCount = QL2.completeCount + 1
        elseif event == "QUEST_TURNED_IN" then
            QL2.turnInCount = QL2.turnInCount + 1
        end
    end)
end

function QL2.gossipReady()
    local provider = GossipFrame.GreetingPanel.ScrollBox:GetDataProvider()
    for _, element in provider:Enumerate() do
        if element.info and element.info.questID == QL2.id then
            return element.info.isComplete == true
        end
    end
    return false
end

function QL2.clickGossip()
    -- Real button template/Setup/OnClick inherited from cached shared mixin.
    QL2.button = QL2.button or CreateFrame("Button", nil, GossipFrame, "GossipTitleActiveQuestButtonTemplate")
    QL2.button:Setup(C_GossipInfo.GetActiveQuests()[1])
    QL2.button:Click()
end

function QL2.assertPopup()
    local matches = 0
    for i = 1, GetNumAutoQuestPopUps() do
        local id, kind = GetAutoQuestPopUp(i)
        if id == QL2.id then
            assert(kind == "COMPLETE")
            matches = matches + 1
        end
    end
    assert(matches == 1, "original tracker event must add one COMPLETE popup")
end
