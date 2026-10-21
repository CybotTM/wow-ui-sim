local collapsed = {}
local measured = 0
for _, line in ipairs(WATCHFRAME_QUESTLINES) do
    if line:IsShown() then
        local text = line.text:GetText()
        if text and text ~= "" then
            measured = measured + 1
            if line.text:GetHeight() <= 0 then
                table.insert(collapsed, text)
            end
        end
    end
end
assert(measured > 3, "native watch titles and objective rows must be measured")
table.sort(collapsed)
return table.concat(collapsed, "|")
