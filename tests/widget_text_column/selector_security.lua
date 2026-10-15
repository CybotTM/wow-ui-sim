-- Assertion-only local replacement; never replace a C_* API or return a fixture.
-- ColumnGet is the existing fixture alias of the actual TextColumnRow getter.
FixtureGetter = ColumnGet
FixtureExpected = ColumnExpected

local function assertCompletePublicTree(actual, expected, path)
    assert(not issecretvalue(actual), path .. ' must be public')
    assert(type(actual) == type(expected), path .. ' type')
    if type(expected) ~= 'table' then
        assert(actual == expected, path .. ' exact supplied value')
        return
    end
    for key, value in pairs(expected) do
        assertCompletePublicTree(actual[key], value, path .. '.' .. tostring(key))
    end
    for key in pairs(actual) do
        assert(not issecretvalue(key), path .. ' public key')
        assert(expected[key] ~= nil, path .. ' unexpected key ' .. tostring(key))
    end
end

-- Complete fixture comparison, including the entries array and every entry field.
-- Recursion checks outer/array/entry tables, all their keys, and every leaf.
function AssertSuppliedSnapshot(...)
    assert(select('#', ...) == 1, 'getter must return exactly one result')
    local actual = ...
    assertCompletePublicTree(actual, FixtureExpected, 'TextColumnRow')
    return actual
end

local function assertDistinctTables(tree, seen)
    if type(tree) ~= 'table' then return end
    assert(not seen[tree], 'outer/array/entry table alias across or within snapshots')
    seen[tree] = true
    for _, value in pairs(tree) do
        assertDistinctTables(value, seen)
    end
end

ColumnIntegerRetained = AssertSuppliedSnapshot(FixtureGetter(763621))
FixtureExpected = ColumnReplacement
ColumnFractionRetained = AssertSuppliedSnapshot(FixtureGetter(763621.25))
FixtureExpected = ColumnExpected

local function assertBothRetained()
    assertCompletePublicTree(ColumnIntegerRetained, ColumnExpected, 'retained integer')
    assertCompletePublicTree(ColumnFractionRetained, ColumnReplacement, 'retained fraction')
end

local function mutateEveryReturnedEntry(snapshot)
    for _, entry in ipairs(snapshot.entries) do
        entry.text = 'caller mutation'
        entry.enabledState = 7
        entry.hAlign = 1
        entry.columnWidth = -999.25
        entry.extra = true
    end
    snapshot.entries[1] = snapshot.entries[2]
    snapshot.entries[2] = nil
    snapshot.entries[9] = {text = 'sparse caller mutation'}
    snapshot.tooltip = 'caller mutation'
    snapshot.entries.extra = true
    snapshot.extra = true
end

function AssertColumnMutationIsolation()
    FixtureExpected = ColumnExpected
    local integer = AssertSuppliedSnapshot(FixtureGetter(763621))
    FixtureExpected = ColumnReplacement
    local fraction = AssertSuppliedSnapshot(FixtureGetter(763621.25))
    local seen = {}
    assertDistinctTables(ColumnIntegerRetained, seen)
    assertDistinctTables(ColumnFractionRetained, seen)
    assertDistinctTables(integer, seen)
    assertDistinctTables(fraction, seen)
    mutateEveryReturnedEntry(integer)
    assertCompletePublicTree(fraction, ColumnReplacement, 'unmodified fraction sibling')
    assertBothRetained()
    mutateEveryReturnedEntry(fraction)
    assertBothRetained()
    FixtureExpected = ColumnExpected
    local integerAgain = AssertSuppliedSnapshot(FixtureGetter(763621))
    FixtureExpected = ColumnReplacement
    local fractionAgain = AssertSuppliedSnapshot(FixtureGetter(763621.25))
    assertDistinctTables(integerAgain, seen)
    assertDistinctTables(fractionAgain, seen)
    -- Explicit truncation probes complement canonical invalid/non-supplied policy.
    ColumnOneNil(FixtureGetter(763620.75))
    ColumnOneNil(FixtureGetter(763622))
    ColumnOneNil(FixtureGetter(763622.25))
    assertBothRetained()
    FixtureExpected = ColumnExpected
end
