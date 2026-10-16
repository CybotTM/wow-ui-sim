-- Observations only. Texture identity is not asset or pixel proof.
local function near(actual, expected)
    assert(math.abs(actual - expected) < 0.00001, 'unexpected numeric value')
end
function AssertCompletedItem(frame, expected)
    assert(frame:IsShown() and frame.widgetID == 934227 and frame.widgetSetID == 467)
    assert(frame.widgetType == 27)
    local child, item = assert(frame.Item), expected.itemInfo
    assert(child:GetParent() == frame, 'actual XML child identity')
    assert(child.itemID == 6948 and child.quality == 1)
    assert(child.Icon:GetParent() == child)
    assert(child.Icon:GetTexture() == 134414, 'existing Hearthstone metadata texture identity')
    assert(child.ItemName:IsShown())
    assert(child.ItemName:GetText() == (item.overrideItemName or 'Hearthstone'))
    assert(child.InfoText:IsShown() == (item.infoText ~= nil))
    if item.infoText ~= nil then assert(child.InfoText:GetText() == item.infoText) end
    assert(child.tooltip == item.overrideTooltip and child.tooltipEnabled == item.tooltipEnabled)
    assert(child.EarnedCheck:IsShown() == item.showAsEarned)
    local iconSize = item.iconSizeType == 0 and 24 or 36
    near(child.Icon:GetWidth(), iconSize)
    near(child.Icon:GetHeight(), iconSize)
    near(frame:GetWidth(), child:GetWidth())
    near(frame:GetHeight(), child:GetHeight())
    assert(frame:GetWidth() > 1 and frame:GetHeight() > 1, 'synchronous continuation completed sizing')
    assert(frame.orderIndex == expected.orderIndex and frame.layoutDirection == expected.layoutDirection)
    near(frame:GetScale(), 1)
    assert(not child.Tooltip or not child.Tooltip:IsShown())
    return child
end
function AssertLiveItemColorCallback(child)
    local color = assert(ColorManager.GetColorDataForItemQuality(1))
    assert(math.abs(color.r - 0.17) > 0.00001 or math.abs(color.g - 0.29) > 0.00001)
    child.ItemName:SetTextColor(0.17, 0.29, 0.41, 1)
    EventRegistry:TriggerEvent('ColorManager.OnColorDataUpdated')
    local r, g, b = child.ItemName:GetTextColor()
    near(r, color.r); near(g, color.g); near(b, color.b)
end
function AssertReleasedItem(frame)
    assert(not frame:IsShown())
    local child = assert(frame.Item)
    assert(not child.Tooltip or not child.Tooltip:IsShown())
    -- A still-registered real callback would overwrite these colors. No mock callback.
    child.ItemName:SetTextColor(0.17, 0.29, 0.41, 1)
    EventRegistry:TriggerEvent('ColorManager.OnColorDataUpdated')
    local r, g, b = child.ItemName:GetTextColor()
    near(r, 0.17); near(g, 0.29); near(b, 0.41)
end
