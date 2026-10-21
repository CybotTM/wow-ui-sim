-- INFERRED native mount selector from PetPaperDollFrame.lua.
-- Keep the seeded-data invariant: an empty legacy default is not a valid pass.
local expected = GetNumCompanions("MOUNT")
assert(expected > 0, "supplied mount catalog must reach the native companion API")
ToggleCharacter("PetPaperDollFrame")
PetPaperDollFrame_SetTab(3)
PetPaperDollFrame_SetCompanionPage(0)
assert(CharacterFrame:IsShown() and PetPaperDollFrameCompanionFrame:IsShown())
assert(PetPaperDollFrameCompanionFrame.mode == "MOUNT")
local displayed = 0
for index = 1, NUM_COMPANIONS_PER_PAGE do
    local button = _G["CompanionButton" .. index]
    if button:IsEnabled() then
        displayed = displayed + 1
        assert(button.creatureID == GetCompanionInfo("MOUNT", index), "mount button identity")
    end
end
assert(displayed == math.min(expected, NUM_COMPANIONS_PER_PAGE), "mount page count")
