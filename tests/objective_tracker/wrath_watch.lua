-- Seed watches through the native selected-log-index API, not modern blocks.
for logIndex = 1, GetNumQuestLogEntries() do
    local title, _, _, _, isHeader = GetQuestLogTitle(logIndex)
    if title and not isHeader then
        AddQuestWatch(logIndex)
    end
end
WatchFrame_Expand(WatchFrame)
WatchFrame_Update(WatchFrame)
