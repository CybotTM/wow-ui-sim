-- Authored manual membership requests, not native defaults.
-- Whole original UI is loaded by Rust. No public APIs or consumers are replaced.
N3 = {}

function N3.prepare()
    AUTO_QUEST_WATCH = "0"
    AUTO_QUEST_PROGRESS = "0"
    -- Session globals only: no persisted CVar setters.
    WATCHFRAME_SORT_TYPE = WATCHFRAME_SORT_MANUAL
    WATCHFRAME_FILTER_TYPE = WATCHFRAME_FILTER_COMPLETED_QUESTS + WATCHFRAME_FILTER_REMOTE_ZONES
    if ChatEdit_GetActiveWindow() then
        ChatEdit_DeactivateChat(ChatEdit_GetActiveWindow())
    end
    assert(not ChatEdit_GetActiveWindow(), 'CHATLINK branch must be inactive')
end

function N3.request_membership(indices)
    -- Explicit supplied manual membership via public legacy mutators, NOT a
    -- guessed private field or Forever C_QuestLog quest-ID seeding policy.
    for i = 1, GetNumQuestLogEntries() do RemoveQuestWatch(i) end
    for _, i in ipairs(indices) do AddQuestWatch(i) end
end

function N3.open()
    ShowUIPanel(QuestLogFrame)
    WatchFrame:Show()
    WatchFrame_Expand(WatchFrame)
    QuestLog_Update()
    WatchFrame_Update()
    assert(QuestLogFrame:IsVisible() and WatchFrameLines:IsVisible())
end

function N3.log_button(index)
    for _, button in ipairs(QuestLogScrollFrame.buttons) do
        if button:IsShown() and button:GetID() == index then return button end
    end
    error('missing original log button at index '..index)
end

function N3.click_log(index)
    local button = N3.log_button(index)
    A_Admin.SetShiftKeyDown(true)
    assert(IsModifiedClick('QUESTWATCHTOGGLE'))
    button:Click('LeftButton') -- original XML OnClick, not a substituted handler
    A_Admin.SetShiftKeyDown(false)
end

function N3.expect_membership(expected)
    local wanted = {}
    local count = 0
    for index, title in pairs(expected) do wanted[index] = true; count = count + 1 end
    assert(GetNumQuestWatches() == count, 'supplied manual watch cardinality')
    local seen = {}
    for watch = 1, GetNumQuestWatches() do
        local index = GetQuestIndexForWatch(watch)
        assert(wanted[index] and not seen[index], 'watch maps to expected current log row')
        assert(GetQuestLogTitle(index) == expected[index], 'mapping reads supplied title')
        seen[index] = true
    end
    for index = 1, GetNumQuestLogEntries() do
        assert(not not IsQuestWatched(index) == not not wanted[index], 'membership agrees')
        assert(N3.log_button(index).check:IsShown() == not not wanted[index], 'original log check')
    end
    local visible = {}
    for _, index in ipairs(VISIBLE_WATCHES) do visible[index] = true end
    for index in pairs(wanted) do assert(visible[index], 'original watch list contains supplied row') end
    for index in pairs(visible) do assert(wanted[index], 'no unrelated visible watch row') end
    for _, title in pairs(expected) do
        local found = false
        for _, line in ipairs(WATCHFRAME_QUESTLINES) do
            if line:IsVisible() and line.text:GetText() == title then found = true end
        end
        assert(found, 'visible original tracker FontString: '..title)
    end
end

function N3.expect_no_removed_identity(removedID)
    for watch = 1, GetNumQuestWatches() do
        local index = GetQuestIndexForWatch(watch)
        assert(type(index) == 'number' and index >= 1 and index <= GetNumQuestLogEntries(),
               'no stale watch index after committed removal')
        local title, _, _, _, _, _, _, _, id = GetQuestLogTitle(index)
        assert(title and id ~= removedID, 'removed identity cannot appear in current watch projection')
    end
    for _, line in ipairs(WATCHFRAME_QUESTLINES) do
        assert(not line:IsVisible() or line.text:GetText() ~= 'N3 supplied B', 'removed title hidden')
    end
end
-- PENDING authority: watch ordering/retention across row permutation, stale-index
-- return sentinel, native event trace, cap enforcement, automatic/timed policy.
