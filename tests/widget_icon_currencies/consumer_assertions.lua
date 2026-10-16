-- Observe actual cached outputs/pools/anchors, never substitute consumers.
local function AssertFontColor(fontString, color)
    local expectedRed, expectedGreen, expectedBlue = color:GetRGB()
    local red, green, blue = fontString:GetTextColor()
    assert(math.abs(red-expectedRed) < .0001 and math.abs(green-expectedGreen) < .0001
        and math.abs(blue-expectedBlue) < .0001, 'cached font color')
end
local function EnabledColor(state)
    if state == Enum.WidgetEnabledState.Disabled then return DISABLED_FONT_COLOR end
    if state == Enum.WidgetEnabledState.Red then return RED_FONT_COLOR end
    if state == Enum.WidgetEnabledState.Green then return GREEN_FONT_COLOR end
    assert(state == Enum.WidgetEnabledState.White, 'fixture color state')
    return HIGHLIGHT_FONT_COLOR
end
function AssertCurrencyRows(frame, expected, enabledState)
    assert(frame.currencyPool:GetNumActive() == #expected, 'exact active currency count')
    local remaining = {}
    for row in frame.currencyPool:EnumerateActive() do remaining[row] = true end
    local previous, ordered = frame.Text, {}
    local sizes = {[0]=16, [1]=20, [2]=22, [3]=18}
    for index, info in ipairs(expected) do
        local found
        for row in pairs(remaining) do
            local point, relative, relativePoint, x, y = row:GetPoint(1)
            if relative == previous then
                assert(not found, 'unique next currency anchor')
                assert(point == 'TOPLEFT')
                assert(relativePoint == (index == 1 and 'BOTTOMLEFT' or 'TOPRIGHT'))
                assert(x == (index == 1 and 0 or 10) and y == (index == 1 and -2 or 0))
                found = row
            end
        end
        assert(found and found:IsVisible(), 'ordered acquired row ' .. index)
        assert(found:GetParent() == frame, 'actual pool parent')
        assert(found.Text:GetText() == info.text and found.Text:IsVisible())
        if info.leadingText == '' then
            assert(not found.LeadingText:IsShown(), 'empty leading text hidden')
        else
            assert(found.LeadingText:IsVisible() and found.LeadingText:GetText() == info.leadingText)
            AssertFontColor(found.LeadingText, EnabledColor(enabledState))
        end
        assert(found.Icon:IsVisible() and found.Icon:GetTexture() == info.iconFileID,
            'requested numeric texture identity only, not asset/pixel proof')
        assert(found.Icon:GetWidth() == sizes[info.iconSizeType]
            and found.Icon:GetHeight() == sizes[info.iconSizeType])
        assert(found.Icon:IsDesaturated() == (enabledState == Enum.WidgetEnabledState.Disabled))
        assert(found.tooltip == info.tooltip and found.tooltipLoc == 3, 'cached tooltip fields')
        AssertFontColor(found.Text, EnabledColor(enabledState))
        remaining[found] = nil
        ordered[index], previous = found, found
    end
    assert(next(remaining) == nil, 'no stale active rows outside ordered chain')
    return ordered
end
function AssertCurrencyConsumer(frame, expected, acquiredTag)
    assert(frame:IsVisible() and frame:GetParent() == FixtureContainer)
    assert(frame.Text:IsVisible() and frame.Text:GetText() == expected.text)
    assert(frame.Icon:GetAtlas() == expected.textureKit, 'cached Icon format is %s, no suffix')
    local enabled = expected.enabledState
    if enabled == Enum.WidgetEnabledState.Yellow then enabled = Enum.WidgetEnabledState.White end
    AssertFontColor(frame.Text, EnabledColor(enabled))
    assert(frame.Icon:IsDesaturated() == (enabled == Enum.WidgetEnabledState.Disabled))
    local rows = AssertCurrencyRows(frame, expected.currencies, enabled)
    local showDescription = expected.descriptionShownState == Enum.WidgetShownState.Shown
        and expected.description ~= ''
    assert(frame.Description:IsShown() == showDescription, 'conditional description visibility')
    if showDescription then
        assert(frame.Description:GetText() == expected.description)
        AssertFontColor(frame.Description, EnabledColor(expected.descriptionEnabledState))
        local point, relative, relativePoint, x, y = frame.Description:GetPoint(1)
        assert(point == 'TOPLEFT' and relativePoint == 'BOTTOMLEFT' and x == 0 and y == -2)
        assert(relative == (rows[1] or frame.Text), 'description follows first currency or text')
    end
    assert(frame.widgetTag == acquiredTag, 'cached acquisition tag')
    assert(frame.orderIndex == expected.orderIndex and frame.layoutDirection == expected.layoutDirection)
    local scale = expected.widgetScale == 1 and .9 or 1
    assert(math.abs(frame:GetScale()-scale) < .0001)
    assert(frame:GetHeight() >= frame.Icon:GetHeight(), 'cached content height')
    return rows
end
function AssertIconCurrenciesInitial(frame)
    return AssertCurrencyConsumer(frame, CurrencyExpected, 'iconcurrencies-contract-731')
end
function AssertIconCurrenciesUpdated(frame, acquiredTag)
    return AssertCurrencyConsumer(frame, CurrencyUpdated, acquiredTag)
end
