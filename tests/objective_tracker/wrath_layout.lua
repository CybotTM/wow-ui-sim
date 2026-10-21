-- Native WatchFrame.xml uses named header/lines, not modern objective blocks.
assert(WatchFrame:IsVisible(), "native watch frame hidden")
assert(WatchFrameTitle:GetText() == OBJECTIVES_TRACKER_LABEL .. " (3)", "native watch title")
assert(WatchFrame:GetFrameStrata() == "LOW", "native watch strata")
assert(WatchFrame:GetWidth() == WATCHFRAME_EXPANDEDWIDTH, "native watch width")
local point, relative, relativePoint, x, y = WatchFrameLines:GetPoint(1)
assert(point == "TOPLEFT" and relative == WatchFrame and relativePoint == "TOPLEFT", "native lines top anchor")
assert(x == 0 and y == -30, "native lines top offset")
local rows = 0
local previousTop
for _, line in ipairs(WATCHFRAME_QUESTLINES) do
    if line:IsShown() then
        rows = rows + 1
        assert(line:GetParent() == WatchFrameLines, "native row parent")
        local top = assert(line:GetTop(), "native row top unresolved")
        -- Completed quests may allocate a blank completion-text row; its text
        -- height is not a layout invariant. Nonempty text measurement has its own case.
        if previousTop then
            assert(top < previousTop, "native rows overlap or reorder")
        end
        previousTop = top
    end
end
assert(rows > 3, "native watch quest and objective rows missing")
