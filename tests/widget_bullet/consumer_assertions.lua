-- Observe unchanged cached pools, font strings and anchors; never replace APIs.
function AssertBulletLines(frame, expected, enabledState)
    assert(frame:IsShown())
    assert(frame.linePool:GetNumActive() == #expected, 'active line count')
    local active = {}
    for line in frame.linePool:EnumerateActive() do active[line] = true end
    local previous = frame
    local ordered = {}
    local totalHeight = 0
    local color = enabledState == Enum.WidgetEnabledState.Disabled
        and DISABLED_FONT_COLOR or NORMAL_FONT_COLOR
    local red, green, blue = color:GetRGB()
    for index, text in ipairs(expected) do
        local found
        for line in pairs(active) do
            local point, relative, relativePoint, x, y = line:GetPoint(1)
            if relative == previous then
                assert(not found, 'unique next anchor in line chain')
                assert(point == 'TOPLEFT' and x == 0)
                assert(relativePoint == (index == 1 and 'TOPLEFT' or 'BOTTOMLEFT'))
                assert(y == (index == 1 and 0 or -3))
                found = line
            end
        end
        assert(found, 'ordered line anchor ' .. index)
        assert(found:IsShown() and found.Text:GetText() == text,
            'ordered visible text ' .. index)
        for _, fontString in ipairs({found.Bullet, found.Text}) do
            assert(fontString.enabledState == enabledState, 'cached enabled-state propagation')
            local r, g, b = fontString:GetTextColor()
            assert(math.abs(r-red) < .0001 and math.abs(g-green) < .0001
                and math.abs(b-blue) < .0001, 'cached enabled-state font color')
        end
        assert(math.abs(found:GetHeight() - found.Text:GetStringHeight()) < .0001)
        totalHeight = totalHeight + found:GetHeight() + (index == 1 and 0 or 3)
        ordered[index] = found
        active[found] = nil
        previous = found
    end
    assert(next(active) == nil, 'no stale active lines outside chain')
    assert(math.abs(frame:GetHeight() - totalHeight) < .0001, 'cached total line height')
    return ordered
end

function AssertBulletCallback()
    assert(bulletConsumerEvents == bulletExpectedEvents, 'synchronous callback count')
    assert(bulletConsumerEventError == nil, bulletConsumerEventError)
    AssertBulletSnapshot(bulletConsumerSnapshot, expectedBullet)
end
