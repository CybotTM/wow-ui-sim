-- Reserve enough native viewport for all three supplied quests and their rows.
WatchFrame:SetHeight(400)
for _, filter in ipairs({ WATCHFRAME_FILTER_COMPLETED_QUESTS, WATCHFRAME_FILTER_REMOTE_ZONES }) do
    if bit.band(WATCHFRAME_FILTER_TYPE, filter) == 0 then
        WatchFrame_SetFilter(nil, filter)
    end
end
-- Seed watches through the native selected-log-index API, not modern blocks.
for logIndex = 1, GetNumQuestLogEntries() do
    local title, _, _, _, isHeader = GetQuestLogTitle(logIndex)
    if title and not isHeader then
        AddQuestWatch(logIndex)
    end
end
WatchFrame_Expand(WatchFrame)
WatchFrame_Update(WatchFrame)
