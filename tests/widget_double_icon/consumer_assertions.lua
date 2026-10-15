-- Observations only. No replacement Setup, provider, pools, fonts or atlas metadata.
local function AssertClose(actual, expected, label)
    assert(math.abs(actual - expected) < .0001,
        label .. ': ' .. tostring(actual) .. ' expected ' .. tostring(expected))
end

local function AssertSide(side, text, tooltip)
    assert(side:IsVisible() and side.Text:IsVisible(), 'side/text visibility')
    assert(side.Text:GetText() == text, 'side text')
    assert(side.tooltip == tooltip, 'side tooltip')
    assert(side:IsMouseEnabled() and not side:IsMouseClickEnabled(), 'side tooltip mouse policy')
    assert(side.Text:GetStringWidth() > 1, 'nonempty side text measurement')
    AssertClose(side:GetWidth(), side.Icon:GetWidth() + side.Text:GetWidth() + 5, 'side width')
end

function AssertDoubleIcon(frame, label, leftText, leftTooltip, rightText, rightTooltip)
    assert(frame:IsVisible() and frame.Label:IsVisible(), 'frame/label visibility')
    assert(frame.Label:GetText() == label, 'label text')
    assert(frame.Label:GetStringWidth() > 1, 'nonempty label measurement')
    AssertSide(frame.Left, leftText, leftTooltip)
    AssertSide(frame.Right, rightText, rightTooltip)
    AssertClose(frame:GetWidth(),
        frame.Label:GetWidth() + 15 + frame.Left:GetWidth() + 25 + frame.Right:GetWidth(),
        'total width')
end

function AssertDoubleIconInitial(frame)
    AssertDoubleIcon(frame, 'Double label 731', 'Left text 731', 'Left tooltip 731',
        'Right text 731', 'Right tooltip 731')
    assert(frame.tooltipLoc == 3 and frame.tooltipAnchor == 'ANCHOR_LEFT')
    AssertClose(frame:GetScale(), .9, 'initial scale')
    assert(frame.orderIndex == 37.5 and frame.layoutDirection == 1)
    assert(frame.widgetTag == 'doubleicon-contract-731')
end

function AssertDoubleIconUpdated(frame, acquiredTag)
    AssertDoubleIcon(frame, 'Updated label 947', 'Updated left 947', 'Updated left tooltip 947',
        'Updated right 947', 'Updated right tooltip 947')
    assert(frame.tooltipLoc == 5 and frame.tooltipAnchor == 'ANCHOR_RIGHT')
    assert(frame:GetScale() == 1 and frame.orderIndex == -12.25 and frame.layoutDirection == 1)
    -- Cached manager assigns tag on acquisition, not ordinary in-place Setup.
    assert(frame.widgetTag == acquiredTag, 'acquisition tag')
end
