-- Observable pooled frame state only; no private handler invocation or replacements.
function ColumnActiveEntries(frame)
    local entries = {}
    for entry in frame.entryPool:EnumerateActive() do entries[#entries + 1] = entry end
    table.sort(entries, function(a, b) return a.layoutIndex < b.layoutIndex end)
    return entries
end

local function assertColumnText(line, info, font)
    assert(line:IsVisible() and line:GetText() == info.text, 'visible nested text')
    assert(math.abs(line:GetWidth() - info.columnWidth) < .0001, 'columnWidth applied')
    local aligns = {[0] = 'LEFT', [1] = 'CENTER', [2] = 'RIGHT'}
    assert(line:GetJustifyH() == aligns[info.hAlign], 'entry hAlign applied')
    assert(line.enabledState == info.enabledState, 'entry enabled state applied')
    local path, size, flags = line:GetFont()
    local expectedPath, expectedSize, expectedFlags = font:GetFont()
    assert(path == expectedPath and size == expectedSize and flags == expectedFlags, 'font/size applied')
    local colors = {[0] = DISABLED_FONT_COLOR, [2] = RED_FONT_COLOR,
        [3] = HIGHLIGHT_FONT_COLOR, [4] = GREEN_FONT_COLOR}
    local er, eg, eb = colors[info.enabledState]:GetRGB()
    local r, g, b = line:GetTextColor()
    assert(math.abs(r-er) < .0001 and math.abs(g-eg) < .0001 and math.abs(b-eb) < .0001,
        'entry enabled color applied')
end

function ColumnAssertFrame(frame, expected, font, acquiredTag)
    assert(frame:IsVisible() and frame.widgetType == 21 and frame.widgetID == 763621)
    assert(frame.widgetSetID == 436 and frame.tooltip == expected.tooltip)
    assert(frame.tooltipLoc == expected.tooltipLoc)
    assert(frame.bottomPadding == expected.bottomPadding)
    assert(frame.fixedHeight == expected.widgetSizeSetting + expected.bottomPadding)
    assert(math.abs(frame:GetHeight() - frame.fixedHeight) < .0001, 'fixed row height')
    assert(frame.orderIndex == expected.orderIndex)
    -- Cached manager stamps widgetTag only on acquisition, not incremental Setup.
    assert(frame.widgetTag == acquiredTag)
    assert(frame.layoutDirection == expected.layoutDirection)
    local scale = expected.widgetScale == 1 and .9 or 1
    assert(math.abs(frame:GetScale() - scale) < .0001)
    local entries = ColumnActiveEntries(frame)
    assert(#entries == #expected.entries and frame.entryPool:GetNumActive() == #entries)
    for index, entry in ipairs(entries) do
        assert(entry.layoutIndex == index and entry:GetParent() == frame and entry:IsShown())
        assertColumnText(entry.Text, expected.entries[index], font)
    end
    return entries
end

ColumnConsumerExpected = {}
for key, value in pairs(ColumnReplacement) do ColumnConsumerExpected[key] = value end
ColumnConsumerExpected.shownState = 1
ColumnConsumerExpected.textureKit = ''
ColumnConsumerExpected.frameTextureKit = ''
ColumnConsumerExpected.hasTimer = false
ColumnConsumerExpected.inAnimType = 0
ColumnConsumerExpected.outAnimType = 0
ColumnConsumerExpected.layoutDirection = 1
ColumnConsumerExpected.modelSceneLayer = 0
ColumnConsumerExpected.scriptedAnimationEffectID = 0
