-- Independent concrete values, never expectations derived from getter output.
ItemExpected = {
    shownState=1, tooltipLoc=3,
    itemInfo={itemID=6948, stackCount=17, overrideItemName='Supplied hearth 731',
        infoText='Info 731', overrideTooltip='Tooltip 731', textDisplayStyle=1,
        tooltipEnabled=true, iconSizeType=0, infoTextEnabledState=4, showAsEarned=false,
        itemNameTextFontType=0, itemNameTextSizeType=0, infoTextFontType=0,
        infoTextSizeType=0, itemNameCustomColor=3, itemNameCustomColorOverrideState=0},
    widgetSizeSetting=180.5, textureKit='', frameTextureKit='', hasTimer=false,
    orderIndex=37.5, widgetTag='item-contract-731', inAnimType=0, outAnimType=0,
    widgetScale=0, layoutDirection=1, modelSceneLayer=0, scriptedAnimationEffectID=0,
}
ItemReplacement = {
    shownState=0, tooltipLoc=6,
    itemInfo={itemID=19019.5, stackCount=0.5, overrideItemName='Replaced item 947',
        infoText='Replaced info 947', overrideTooltip='Replaced tooltip 947', textDisplayStyle=2,
        tooltipEnabled=false, iconSizeType=2, infoTextEnabledState=2, showAsEarned=true,
        itemNameTextFontType=1, itemNameTextSizeType=4, infoTextFontType=2,
        infoTextSizeType=3, itemNameCustomColor=4, itemNameCustomColorOverrideState=1},
    widgetSizeSetting=206.25, textureKit='projection-item-947', frameTextureKit='projection-frame-947',
    hasTimer=true, orderIndex=-12.25, widgetTag='item-contract-947', inAnimType=1, outAnimType=1,
    widgetScale=1, layoutDirection=2, modelSceneLayer=1, scriptedAnimationEffectID=947.5,
}
function CopyItemExpected(value)
    if type(value) ~= 'table' then return value end
    local copy = {}
    for key, child in pairs(value) do copy[key] = CopyItemExpected(child) end
    return copy
end
ItemNoOptionals = CopyItemExpected(ItemExpected)
ItemNoOptionals.itemInfo.stackCount = nil
ItemNoOptionals.itemInfo.overrideItemName = nil
ItemNoOptionals.itemInfo.infoText = nil
ItemNoOptionals.itemInfo.overrideTooltip = nil
ItemEmptyStrings = CopyItemExpected(ItemExpected)
ItemEmptyStrings.itemInfo.stackCount = 0
ItemEmptyStrings.itemInfo.overrideItemName = ''
ItemEmptyStrings.itemInfo.infoText = ''
ItemEmptyStrings.itemInfo.overrideTooltip = ''
ItemUpdated = CopyItemExpected(ItemExpected)
ItemUpdated.itemInfo.overrideItemName = 'Updated hearth 947'
ItemUpdated.itemInfo.infoText = 'Updated info 947'
ItemUpdated.itemInfo.overrideTooltip = 'Updated tooltip 947'
ItemUpdated.itemInfo.stackCount = 23
ItemUpdated.itemInfo.iconSizeType = 2
ItemUpdated.itemInfo.showAsEarned = true
ItemUpdated.orderIndex = -12.25
ItemUpdated.widgetTag = 'updated-item-tag'

-- Nested assertion extension only, not a getter/provider/Setup replacement.
local function AssertDeepItem(actual, expected, path)
    assert(type(actual) == type(expected), 'wrong type: ' .. path)
    assert(not issecretvalue(actual), 'public snapshot: ' .. path)
    if type(expected) ~= 'table' then
        assert(actual == expected, 'wrong value: ' .. path)
        return
    end
    local actualCount, expectedCount = 0, 0
    for key, value in pairs(expected) do
        expectedCount = expectedCount + 1
        AssertDeepItem(actual[key], value, path .. '.' .. key)
    end
    for key in pairs(actual) do
        actualCount = actualCount + 1
        assert(not issecretvalue(key), 'public key: ' .. path)
        assert(expected[key] ~= nil, 'unexpected field: ' .. path .. '.' .. tostring(key))
    end
    assert(actualCount == expectedCount, 'missing or extra fields: ' .. path)
end
function AssertSuppliedSnapshot(...)
    assert(select('#', ...) == 1, 'getter must return exactly one result')
    local actual = ...
    assert(type(actual) == 'table', 'populated ItemDisplay getter returned nil/non-table')
    AssertDeepItem(actual, FixtureExpected, 'widget')
    return actual
end
function AssertItemAt(id, expected)
    local previous = FixtureExpected
    FixtureExpected = expected
    local actual = AssertSuppliedSnapshot(FixtureGetter(id))
    FixtureExpected = previous
    return actual
end
FixtureExpected = ItemExpected
