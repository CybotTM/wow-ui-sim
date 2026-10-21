-- Explicit test-owned native settings include completed and remote quests.
SetCVar("trackerFilter", WATCHFRAME_FILTER_COMPLETED_QUESTS + WATCHFRAME_FILTER_REMOTE_ZONES)
SetCVar("trackerSorting", WATCHFRAME_SORT_MANUAL)
WatchFrame_OnEvent(WatchFrame, "VARIABLES_LOADED")
-- Reserve enough native viewport for all three supplied quests and their rows.
WatchFrame:SetHeight(400)
-- Seed watches through the native selected-log-index API, not modern blocks.
for logIndex = 1, GetNumQuestLogEntries() do
    local title, _, _, _, isHeader = GetQuestLogTitle(logIndex)
    if title and not isHeader then
        AddQuestWatch(logIndex)
    end
end
WatchFrame_Expand(WatchFrame)
WatchFrame_Update(WatchFrame)
