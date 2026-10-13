-- Selected simulator removal policy, not established native WoW behavior.
local frame = CreateFrame("Frame")
local ok, result = pcall(function()
    return frame:UnregisterEvent('PLAYER_EQUIPED_SPELLS_CHANGED')
end)
assert(ok, "cached Base.lua:697 UnregisterEvent('PLAYER_EQUIPED_SPELLS_CHANGED') must not throw: " .. tostring(result))
assert(result == false, "absent cached event removal must return false")

local function pack(...)
    return { n = select('#', ...), ... }
end

local function assert_absent_removal(event)
    local values = pack(frame:UnregisterEvent(event))
    assert(values.n == 1, "absent removal must return exactly one value: " .. event)
    assert(values[1] == false, "absent removal must return false: " .. event)
    assert(not frame:IsEventRegistered(event), "absent removal must not register: " .. event)
end

local unknown = "WOW_UI_SIM_UNREGISTER_UNKNOWN_POLICY_TEST"
assert_absent_removal('PLAYER_EQUIPED_SPELLS_CHANGED')
assert_absent_removal(unknown)

local delivered = 0
frame:SetScript("OnEvent", function(self, event)
    assert(self == frame, "event must reach the registered frame")
    assert(event == "PLAYER_LOGIN", "known event payload must be preserved")
    delivered = delivered + 1
end)
assert(frame:RegisterEvent("PLAYER_LOGIN") == true, "known event registration control")
assert(frame:IsEventRegistered("PLAYER_LOGIN"), "known event must be registered")
A_Admin.FireEvent("PLAYER_LOGIN")
assert(delivered == 1, "known registration must deliver before unknown cleanup")

assert_absent_removal(unknown)
assert_absent_removal('PLAYER_EQUIPED_SPELLS_CHANGED')
assert(frame:IsEventRegistered("PLAYER_LOGIN"), "unknown cleanup must preserve known registration")

local register_ok = pcall(function()
    frame:RegisterEvent(unknown)
end)
assert(not register_ok, "Retail/PTR RegisterEvent must still reject unknown names")
local unit_register_ok = pcall(function()
    frame:RegisterUnitEvent(unknown, "player")
end)
assert(not unit_register_ok, "Retail/PTR RegisterUnitEvent must still reject unknown names")
assert(not frame:IsEventRegistered(unknown), "rejected registration must not install unknown event")
assert(frame:IsEventRegistered("PLAYER_LOGIN"), "rejected registrations must preserve known registration")
A_Admin.FireEvent("PLAYER_LOGIN")
assert(delivered == 2, "known registration must deliver after unknown cleanup and rejected registrations")

local removed = pack(frame:UnregisterEvent("PLAYER_LOGIN"))
assert(removed.n == 1 and removed[1] == true, "known removal must return exactly one true value")
assert(not frame:IsEventRegistered("PLAYER_LOGIN"), "known removal must clear registration")
A_Admin.FireEvent("PLAYER_LOGIN")
assert(delivered == 2, "removed known event must no longer deliver")
assert_absent_removal("PLAYER_LOGIN")
frame:SetScript("OnEvent", nil)
print("FRAME_UNREGISTER_UNKNOWN_POLICY_DONE")
