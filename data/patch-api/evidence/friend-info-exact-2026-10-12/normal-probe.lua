-- Bounded normal Retail/PTR loaded-publication probe for FriendInfo exact lookup.
-- Run only against normal Retail and PTR wow-sim binaries after build; do not seed state.
local ns = rawget(_G, 'C_FriendList')
assert(type(ns) == 'table', 'C_FriendList namespace absent')
local exact = rawget(ns, 'GetFriendInfo')
assert(type(exact) == 'function', 'exact C_FriendList.GetFriendInfo raw publication absent')

local alternates = { 'GetFriendInfoByName' }
for _, name in ipairs(alternates) do
    assert(rawget(ns, name) == nil, name .. ' raw alternate publication must be absent')
    assert(ns[name] == nil, name .. ' ordinary alternate lookup must be absent')
end

local function one_result(...)
    assert(select('#', ...) == 1, 'expected exactly one return value')
    local value = ...
    assert(value == nil or type(value) == 'table', 'result must be table or nil')
    return value
end

-- Existing state names are evidenced by src/lua_api/state/defaults/friends.rs.
-- Arthax is seeded with an empty note; Durotan has the supplied note below.
local arthax = one_result(exact('Arthax'))
assert(type(arthax) == 'table', 'existing default friend Arthax should resolve')
assert(arthax.name == 'Arthax' and arthax.notes == '', 'Arthax default identity/note')
local durotan = one_result(exact('Durotan'))
assert(type(durotan) == 'table', 'existing default friend Durotan should resolve')
assert(durotan.name == 'Durotan' and durotan.notes == 'old guildie', 'Durotan supplied default state')
assert(one_result(exact('FriendInfo-NoSuchFriend')) == nil, 'unknown name should return nil')
