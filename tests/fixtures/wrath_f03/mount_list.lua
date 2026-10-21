-- INFERRED native mount selector from PetPaperDollFrame.lua.
-- Rust supplies two collected, explicitly eligible catalog entries and dispatches
-- COMPANION_LEARNED. This is a single-page population test, not paging coverage.
local expected = GetNumCompanions("MOUNT")
assert(expected == 2, "two supplied eligible owned mounts reach the native count API")
assert(expected <= NUM_COMPANIONS_PER_PAGE, "complete supplied catalog fits one native page")
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
assert(displayed == expected, "complete native mount list count")
