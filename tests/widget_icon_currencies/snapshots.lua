-- Independent concrete expected values, not derived from returned state.
CurrencyExpected = {
    shownState=1, enabledState=1, descriptionShownState=1, descriptionEnabledState=2,
    text='Currency contract 731', description='Description 731',
    currencies={
        {iconFileID=134400, leadingText='Earned', text='17 / 80',
         tooltip='First currency tooltip 731', isCurrencyMaxed=false,
         textFontType=0, textSizeType=0, textEnabledState=4, iconSizeType=0, updateAnimType=0},
        {iconFileID=134414, leadingText='', text='43',
         tooltip='Second currency tooltip 947', isCurrencyMaxed=true,
         textFontType=1, textSizeType=4, textEnabledState=2, iconSizeType=2, updateAnimType=0},
    },
    tooltipLoc=3, widgetSizeSetting=180.5, textureKit='iconcurrencies-fixture-731',
    frameTextureKit='iconcurrencies-frame-731', hasTimer=false, orderIndex=37.5,
    widgetTag='iconcurrencies-contract-731', inAnimType=0, outAnimType=0,
    widgetScale=1, layoutDirection=1, modelSceneLayer=0, scriptedAnimationEffectID=0,
}
CurrencyReplacement = {
    shownState=0, enabledState=0, descriptionShownState=0, descriptionEnabledState=4,
    text='Replaced currency contract 947', description='Replaced description 947',
    currencies={
        {iconFileID=134415, leadingText='Spent', text='26 / 90',
         tooltip='Replacement first tooltip', isCurrencyMaxed=true,
         textFontType=2, textSizeType=1, textEnabledState=0, iconSizeType=1, updateAnimType=1},
        {iconFileID=134416, leadingText='Remaining', text='65',
         tooltip='Replacement second tooltip', isCurrencyMaxed=false,
         textFontType=0, textSizeType=2, textEnabledState=3, iconSizeType=3, updateAnimType=2},
    },
    tooltipLoc=6, widgetSizeSetting=206.25, textureKit='replacement-currency-icon',
    frameTextureKit='replacement-currency-frame', hasTimer=true, orderIndex=-12.25,
    widgetTag='replacement-iconcurrencies-tag', inAnimType=1, outAnimType=1,
    widgetScale=0, layoutDirection=2, modelSceneLayer=1, scriptedAnimationEffectID=947.5,
}
function CopyCurrencyExpected(value)
    if type(value) ~= 'table' then return value end
    local copy = {}
    for key, child in pairs(value) do copy[key] = CopyCurrencyExpected(child) end
    return copy
end
CurrencyUpdated = CopyCurrencyExpected(CurrencyExpected)
CurrencyUpdated.enabledState = 0
CurrencyUpdated.descriptionEnabledState = 4
CurrencyUpdated.text = 'Updated currencies 947'
CurrencyUpdated.description = 'Updated description 947'
CurrencyUpdated.textureKit = 'iconcurrencies-fixture-947'
CurrencyUpdated.widgetTag = 'updated-iconcurrencies-tag'
CurrencyUpdated.orderIndex = -12.25
CurrencyUpdated.widgetScale = 0
CurrencyUpdated.currencies[1], CurrencyUpdated.currencies[2] =
    CurrencyUpdated.currencies[2], CurrencyUpdated.currencies[1]
CurrencyUpdated.currencies[1].text = 'Updated 43'

local function AssertDeepSnapshot(actual, expected, path)
    assert(type(actual) == type(expected), 'wrong type: ' .. path)
    assert(not issecretvalue(actual), 'snapshot must be public: ' .. path)
    if type(expected) ~= 'table' then
        assert(actual == expected, 'wrong value: ' .. path)
        return
    end
    local expectedCount, actualCount = 0, 0
    for key, value in pairs(expected) do
        expectedCount = expectedCount + 1
        AssertDeepSnapshot(actual[key], value, path .. '.' .. tostring(key))
    end
    for key in pairs(actual) do
        actualCount = actualCount + 1
        assert(not issecretvalue(key), 'snapshot key must be public')
        assert(expected[key] ~= nil, 'unexpected key: ' .. path .. '.' .. tostring(key))
    end
    assert(actualCount == expectedCount, 'field count: ' .. path)
end
-- Test assertion only, not a production getter/provider/Setup replacement.
function AssertSuppliedSnapshot(...)
    assert(select('#', ...) == 1, 'exactly one result')
    local actual = ...
    AssertDeepSnapshot(actual, FixtureExpected, 'widget')
    return actual
end
function AssertCurrencyAt(id, expected)
    local previous = FixtureExpected
    FixtureExpected = expected
    local actual = AssertSuppliedSnapshot(FixtureGetter(id))
    FixtureExpected = previous
    return actual
end
FixtureExpected = CurrencyExpected
