-- INFERRED native PaperDollItemSlotButton hover entry and public line oracle.
-- Modern GetPrimaryTooltipInfo/Data reflection is not a native requirement.
ToggleCharacter("PaperDollFrame")
assert(CharacterFrame:IsShown(), "native character panel opens")
local slot = CharacterHeadSlot
local onEnter = slot:GetScript("OnEnter")
assert(type(onEnter) == "function", "native equipment slot hover handler exists")
onEnter(slot)
assert(GameTooltip:NumLines() > 0, "native inventory tooltip has populated lines")
ToggleCharacter("PaperDollFrame")
assert(not CharacterFrame:IsShown(), "native character panel closes after tooltip use")
