-- Assertions only. No replacement getter/provider/handler/template.
ColumnGet = C_UIWidgetManager.GetTextColumnRowVisualizationInfo
ColumnExpected = {
    shownState = 1,
    entries = {
        {text = 'Alpha 731', enabledState = 3, hAlign = 0, columnWidth = 111.25},
        {text = '', enabledState = 0, hAlign = 1, columnWidth = 87.5},
        {text = 'Gamma 731', enabledState = 4, hAlign = 2, columnWidth = 146.75},
    },
    textSizeType = 1, fontType = 0, tooltip = 'Column tooltip 731', tooltipLoc = 3,
    bottomPadding = 6.5, widgetSizeSetting = 40.5, textureKit = '', frameTextureKit = '',
    hasTimer = false, orderIndex = 37.5, widgetTag = 'column-731', inAnimType = 0,
    outAnimType = 0, widgetScale = 1, layoutDirection = 1, modelSceneLayer = 0,
    scriptedAnimationEffectID = 0,
}
ColumnReplacement = {
    shownState = 0,
    entries = {
        {text = 'Delta 947', enabledState = 2, hAlign = 2, columnWidth = 91.5},
        {text = 'Epsilon 947', enabledState = 4, hAlign = 0, columnWidth = 201.25},
    },
    textSizeType = 4, fontType = 1, tooltip = 'Column tooltip 947', tooltipLoc = 5,
    bottomPadding = 8.25, widgetSizeSetting = 52.25, textureKit = 'query-only-column-kit',
    frameTextureKit = 'query-only-frame-kit', hasTimer = true, orderIndex = -12.25,
    widgetTag = 'column-947', inAnimType = 1, outAnimType = 1, widgetScale = 0,
    layoutDirection = 2, modelSceneLayer = 1, scriptedAnimationEffectID = 947.5,
}

local function sameRecord(actual, expected, label)
    assert(type(actual) == 'table', label .. ' must be a table')
    for key, value in pairs(expected) do
        assert(type(actual[key]) == type(value), label .. '.' .. tostring(key) .. ' type')
        if type(value) == 'table' then
            sameRecord(actual[key], value, label .. '.' .. tostring(key))
        else
            assert(actual[key] == value, label .. '.' .. tostring(key) .. ' value')
        end
    end
    for key in pairs(actual) do
        assert(expected[key] ~= nil, label .. ' extra field ' .. tostring(key))
    end
end

function ColumnSnapshot(actual, expected)
    sameRecord(actual, expected, 'TextColumnRow')
    return actual
end

function ColumnOneSnapshot(expected, ...)
    assert(select('#', ...) == 1, 'exactly one result')
    return ColumnSnapshot(..., expected)
end

function ColumnOneNil(...)
    assert(select('#', ...) == 1 and select(1, ...) == nil, 'exactly one nil')
end
