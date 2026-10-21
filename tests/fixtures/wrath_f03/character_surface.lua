-- INFERRED selectors from shipped 12340 CharacterFrame/PaperDollFrame.
assert(CharacterFrame:IsShown(), "character panel should open")
assert(CharacterNameText:GetText() == UnitPVPName("player"), "native character title")
for _, name in ipairs({"CharacterHeadSlot", "CharacterChestSlot", "CharacterMainHandSlot"}) do
    local slot = assert(_G[name], "missing native equipment slot " .. name)
    local expected = GetInventoryItemTexture("player", slot:GetID())
    if expected == nil then expected = slot.backgroundTextureName end
    local icon = assert(_G[name .. "IconTexture"], "missing native slot icon " .. name)
    assert(icon:GetTexture() == expected, "native slot texture mismatch " .. name)
end
