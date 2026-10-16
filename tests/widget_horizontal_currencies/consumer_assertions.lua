-- Grounding (Retail/PTR AddOns/Blizzard_UIWidgets/):
-- Blizzard_UIWidgetTemplateHorizontalCurrencies.lua:12-63 (rows/dimensions/reset);
-- Blizzard_UIWidgetTemplateBase.lua:117-156,270-310,325-403 (colors/scale/row Setup).
-- Actual cached Setup output only; no provider/Setup/pool substitutions.
local function AssertColor(red, green, blue, expected)
    local expectedRed, expectedGreen, expectedBlue = expected:GetRGB()
    assert(math.abs(red-expectedRed) < .0001)
    assert(math.abs(green-expectedGreen) < .0001)
    assert(math.abs(blue-expectedBlue) < .0001)
end
local function AssertNormalFont(fontString)
    local red, green, blue = fontString:GetTextColor()
    AssertColor(red, green, blue, NORMAL_FONT_COLOR)
end
function AssertHorizontalConsumer(frame, expected, acquiredTag)
    assert(frame:IsVisible() and frame:GetParent() == FixtureContainer)
    assert(frame.currencyPool:GetNumActive() == #expected.currencies)
    local remaining, ordered = {}, {}
    for row in frame.currencyPool:EnumerateActive() do remaining[row] = true end
    local previous, totalWidth, biggestHeight = frame, 0, 0
    local iconSizes = {[0]=16, [1]=20, [2]=22, [3]=18}
    for index, info in ipairs(expected.currencies) do
        local found
        for row in pairs(remaining) do
            local point, relative, relativePoint, x, y = row:GetPoint(1)
            if relative == previous then
                assert(not found, 'unique next pooled currency')
                assert(point == 'TOPLEFT')
                assert(relativePoint == (index == 1 and 'TOPLEFT' or 'TOPRIGHT'))
                assert(x == (index == 1 and 0 or 10) and y == 0)
                found = row
            end
        end
        assert(found and found:IsVisible(), 'ordered cached row ' .. index)
        assert(found:GetParent() == frame)
        assert(found.Text:IsVisible() and found.Text:GetText() == info.text)
        AssertNormalFont(found.Text)
        assert(found.LeadingText:IsShown() == (info.leadingText ~= ''))
        if info.leadingText ~= '' then
            assert(found.LeadingText:GetText() == info.leadingText)
            AssertNormalFont(found.LeadingText)
        end
        assert(found.Icon:IsVisible() and found.Icon:GetTexture() == info.iconFileID,
            'requested texture identity only, not asset/pixel proof')
        assert(found.Icon:GetWidth() == iconSizes[info.iconSizeType])
        assert(found.Icon:GetHeight() == iconSizes[info.iconSizeType])
        assert(not found.Icon:IsDesaturated())
        -- Cached HorizontalCurrencies uses Yellow regardless of textEnabledState.
        assert(found.enabledState == Enum.WidgetEnabledState.Yellow)
        assert(found.tooltip == info.tooltip and found.tooltipLoc == expected.tooltipLoc)
        local color = info.isCurrencyMaxed and RED_FONT_COLOR or HIGHLIGHT_FONT_COLOR
        local red, green, blue = found.tooltipColor:GetRGB()
        AssertColor(red, green, blue, color)
        totalWidth = totalWidth + found:GetWidth() + (index == 1 and 0 or 10)
        biggestHeight = math.max(biggestHeight, found:GetHeight())
        remaining[found] = nil
        ordered[index], previous = found, found
    end
    assert(next(remaining) == nil, 'no stale active rows outside ordered chain')
    local width = totalWidth
    if expected.widgetSizeSetting > totalWidth then
        width = totalWidth + (expected.widgetSizeSetting - totalWidth) / 2
    end
    assert(math.abs(frame:GetWidth()-width) < .0001, 'cached width formula, not pixels')
    assert(math.abs(frame:GetHeight()-biggestHeight) < .0001, 'cached maximum row height')
    assert(frame.widgetTag == acquiredTag, 'cached acquisition tag')
    assert(frame.orderIndex == expected.orderIndex and frame.layoutDirection == expected.layoutDirection)
    local scale = expected.widgetScale == 1 and .9 or 1
    assert(math.abs(frame:GetScale()-scale) < .0001)
    return ordered
end
function AssertHorizontalInitial(frame)
    return AssertHorizontalConsumer(frame, CurrencyExpected, 'horizontalcurrencies-contract-731')
end
function AssertHorizontalUpdated(frame, acquiredTag)
    return AssertHorizontalConsumer(frame, CurrencyUpdated, acquiredTag)
end
