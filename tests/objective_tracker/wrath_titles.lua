local titles = {}
assert(#WATCHFRAME_QUESTLINES > 0, "native watch rows must populate")
for _, line in ipairs(WATCHFRAME_QUESTLINES) do
    if line:IsShown() then
        local text = line.text:GetText()
        for logIndex = 1, GetNumQuestLogEntries() do
            local title, _, _, _, isHeader = GetQuestLogTitle(logIndex)
            if not isHeader and title == text then
                table.insert(titles, text)
                break
            end
        end
    end
end
table.sort(titles)
return table.concat(titles, "|")
