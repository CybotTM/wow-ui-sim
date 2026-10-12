local query = rawget(C_PlayerInfo, 'GUIDIsPlayer')
assert(type(query) == 'function', 'raw modeled GUID player query')
local function check(expected, ...)
    assert(select('#', ...) == 1, 'normal GUID result arity')
    local result = ...
    assert(type(result) == 'boolean' and result == expected, 'normal GUID result')
end
check(true, query(UnitGUID('player')))
check(false, query('Player-990001-00000099'))
assert(type(rawget(C_PlayerInfo, 'GetAlternateFormInfo')) == 'function', 'sibling registration retained')
print('GUID_PLAYER_NORMAL_PROBE_PASS')
