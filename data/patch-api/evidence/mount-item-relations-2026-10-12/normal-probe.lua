local query = C_MountJournal.GetMountFromItem
assert(type(query) == 'function', 'actual normal-session mount-item provider')
local function miss(...)
    assert(select('#', ...) == 1 and (...) == nil, 'empty normal relation and exact arity')
end
miss(query(6948))
miss(query(901001))
local ok, message = pcall(query, '6948')
assert(not ok and type(message) == 'string' and message:find('number', 1, true), 'normal numeric input policy')
assert(type(C_MountJournal.GetMountFromSpell) == 'function', 'existing journal spell provider preserved')
assert(C_MountJournal.GetMountFromSpell(458) == 6, 'existing Brown Horse relation preserved')
assert(UIParent ~= nil and PlayerFrame ~= nil and IsLoggedIn(), 'normal game startup readiness')
print('MOUNT_ITEM_NORMAL_PROBE_PASS')
