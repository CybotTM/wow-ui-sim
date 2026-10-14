-- Assertions only: do not replace providers, templates, handlers, or font APIs.
function AssertSubtextLine(line, text, justify, enabled, font, color)
    assert(line:IsVisible() and line:GetText() == text, 'line visible/text: ' .. tostring(line:GetText()))
    assert(line:GetJustifyH() == justify and line.enabledState == enabled,
        'line justify/state: ' .. tostring(line:GetJustifyH()) .. '/' .. tostring(line.enabledState))
    local path, size, flags = line:GetFont()
    local expectedPath, expectedSize, expectedFlags = font:GetFont()
    assert(path == expectedPath and size == expectedSize and flags == expectedFlags,
        'line font: ' .. tostring(path) .. '/' .. tostring(size) .. '/' .. tostring(flags)
        .. ' expected ' .. tostring(expectedPath) .. '/' .. tostring(expectedSize) .. '/' .. tostring(expectedFlags))
    local r, g, b = line:GetTextColor()
    local er, eg, eb = color:GetRGB()
    assert(math.abs(r-er) < .0001 and math.abs(g-eg) < .0001 and math.abs(b-eb) < .0001,
        'line color: ' .. tostring(r) .. '/' .. tostring(g) .. '/' .. tostring(b))
    assert(line:GetStringHeight() > 1 and line:GetStringWidth() > 1, 'line measured dimensions')
end

function AssertSubtextGeometry(frame, width, spacing)
    assert(frame:IsVisible(), 'frame visibility')
    assert(math.abs(frame:GetWidth() - width) < .0001,
        'frame width: ' .. tostring(frame:GetWidth()) .. ' expected ' .. tostring(width))
    assert(frame.Text:GetWidth() == width and frame.SubText:GetWidth() == width,
        'child widths: ' .. tostring(frame.Text:GetWidth()) .. '/' .. tostring(frame.SubText:GetWidth())
        .. ' expected ' .. tostring(width))
    assert(frame.spacing == spacing, 'frame spacing: ' .. tostring(frame.spacing))
    -- Cached VerticalLayoutMixin rounds child sizes to 0.1 before placement.
    local mainHeight = RoundToNearestMultiple(frame.Text:GetHeight(), .1)
    local subHeight = RoundToNearestMultiple(frame.SubText:GetHeight(), .1)
    assert(mainHeight > 1 and subHeight > 1, 'rounded child heights')
    assert(math.abs(frame:GetHeight() - mainHeight - spacing - subHeight) < .0001,
        'frame height: ' .. tostring(frame:GetHeight()) .. ' expected '
        .. tostring(mainHeight + spacing + subHeight))
    local mainPoint, mainRelative, mainRelativePoint, mainX, mainY = frame.Text:GetPoint(1)
    local subPoint, subRelative, subRelativePoint, subX, subY = frame.SubText:GetPoint(1)
    assert(mainPoint == 'TOPLEFT' and subPoint == 'TOPLEFT', 'child anchor points')
    assert(mainRelative == frame and subRelative == frame, 'child anchor relatives')
    assert(mainRelativePoint == 'TOPLEFT' and subRelativePoint == 'TOPLEFT', 'child relative anchor points')
    assert(mainX == 0 and subX == 0 and mainY == 0, 'initial child offsets')
    assert(math.abs(subY + mainHeight + spacing) < .0001, 'second line follows first plus spacing')
end

function AssertSubtextInitial(frame)
    AssertSubtextLine(frame.Text, 'Subtext heading 731', 'LEFT', 3, SystemFont_Large, HIGHLIGHT_FONT_COLOR)
    AssertSubtextLine(frame.SubText, 'Secondary line 731', 'RIGHT', 4, SystemFont_Shadow_Med3, GREEN_FONT_COLOR)
    AssertSubtextGeometry(frame, 180.5, 3.5)
    assert(frame.tooltip == 'Subtext tooltip 731')
    assert(frame.tooltipLoc == 3 and frame.tooltipAnchor == 'ANCHOR_LEFT')
    assert(frame:IsMouseEnabled() and not frame:IsMouseClickEnabled())
    assert(math.abs(frame:GetScale() - .9) < .0001)
    assert(frame.orderIndex == 37.5 and frame.widgetTag == 'subtext-contract-731')
    assert(frame.layoutDirection == 1)
end

function AssertSubtextUpdated(frame, acquiredTag)
    AssertSubtextLine(frame.Text, 'Updated heading 947', 'RIGHT', 4, SystemFont_Shadow_Med3, GREEN_FONT_COLOR)
    AssertSubtextLine(frame.SubText, 'Updated secondary 947', 'CENTER', 3, SystemFont_Large, HIGHLIGHT_FONT_COLOR)
    AssertSubtextGeometry(frame, 206.25, 7.25)
    assert(frame.tooltip == 'Updated tooltip 947', 'updated tooltip')
    assert(frame.tooltipLoc == 5 and frame.tooltipAnchor == 'ANCHOR_RIGHT', 'updated tooltip location')
    assert(frame:IsMouseEnabled() and not frame:IsMouseClickEnabled(), 'updated mouse policy')
    assert(frame:GetScale() == 1 and frame.orderIndex == -12.25,
        'updated scale/order: ' .. tostring(frame:GetScale()) .. '/' .. tostring(frame.orderIndex))
    -- The cached manager initializes widgetTag on acquisition, not ordinary Setup.
    assert(frame.widgetTag == acquiredTag and frame.layoutDirection == 1,
        'acquired tag/layout: ' .. tostring(frame.widgetTag) .. '/' .. tostring(frame.layoutDirection))
end
