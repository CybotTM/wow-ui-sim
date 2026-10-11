local function assertQuery(frame, registered, firstUnit)
    assert(select('#', frame:IsEventRegistered('UNIT_HEALTH')) == 2,
        'UNIT_HEALTH query must return exactly two values')
    local actualRegistered, actualFirstUnit = frame:IsEventRegistered('UNIT_HEALTH')
    assert(actualRegistered == registered, 'unexpected registration state')
    assert(actualFirstUnit == firstUnit, 'unexpected first unit')
end

local frame = CreateFrame('Frame')
assertQuery(frame, false, nil)
frame:RegisterUnitEvent('UNIT_HEALTH', 'player', 'target')
assertQuery(frame, true, 'player')
frame:RegisterUnitEvent('UNIT_HEALTH', 'target')
assertQuery(frame, true, 'target')
frame:UnregisterEvent('UNIT_HEALTH')
assertQuery(frame, false, nil)

-- Invalid-first compatibility checks the current simulator model, not native WoW.
local invalidFirstFrame = CreateFrame('Frame')
invalidFirstFrame:RegisterUnitEvent('UNIT_HEALTH', 'not_a_unit')
assertQuery(invalidFirstFrame, true, nil)

print('UNIT_QUERY_ARITY_PASS')
