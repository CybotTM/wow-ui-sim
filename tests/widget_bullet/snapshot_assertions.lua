-- Assertions only: no getter, namespace, pool, event, or vendor replacement.
expectedBullet = {
    shownState = 1, enabledState = 1, lines = {'first', '', 'third'},
    widgetSizeSetting = 237.5, textureKit = 'supplied-kit',
    frameTextureKit = 'supplied-frame-kit', hasTimer = true, orderIndex = -12.5,
    widgetTag = 'bullet-supplied', inAnimType = 1, outAnimType = 1,
    widgetScale = 6, layoutDirection = 1, modelSceneLayer = 2,
    scriptedAnimationEffectID = 987.25,
}
function AssertBulletSnapshot(actual, expected)
    assert(type(actual) == 'table', 'complete supplied table')
    local fieldCount = 0
    for key, value in pairs(expected) do
        assert(type(actual[key]) == type(value), 'field type: ' .. key)
        if key == 'lines' then
            assert(#actual.lines == #value, 'ordered line count')
            for index, text in ipairs(value) do
                assert(type(actual.lines[index]) == 'string', 'line type')
                assert(actual.lines[index] == text, 'ordered line ' .. index)
            end
            local count = 0
            for index in pairs(actual.lines) do
                assert(type(index) == 'number' and index % 1 == 0
                    and index >= 1 and index <= #value, 'dense one-based lines')
                count = count + 1
            end
            assert(count == #value)
        else
            assert(actual[key] == value, 'field value: ' .. key)
        end
    end
    for key in pairs(actual) do
        assert(expected[key] ~= nil, 'unexpected field: ' .. tostring(key))
        fieldCount = fieldCount + 1
    end
    assert(fieldCount == 15, 'exact fifteen-field snapshot')
end
