-- Supplied socket state is unchanged; INFERRED selectors from native
-- Blizzard_ItemSocketingUI.lua/xml exercise the original free-function consumer.
C_ItemSocketInfo._state.isOpen = true
C_ItemSocketInfo._state.numSockets = 2
C_ItemSocketInfo._state.itemInfo = {
    name = "Socketed Helm", icon = 901, itemID = 6948, link = "item:6948",
    quality = 4, isRefundable = false, isBoundTradeable = true,
}
C_ItemSocketInfo._state.socketTypes = { [1] = "Red", [2] = "Blue" }
C_ItemSocketInfo._state.existingSockets = {
    [1] = { name = "Ruby", icon = 111, itemID = 6948, gemMatchesSocket = true, link = "item:6948" },
}
C_ItemSocketInfo._state.newSockets = {
    [2] = { name = "Bound Sapphire", icon = 222, itemID = 6948, gemMatchesSocket = false,
        link = "item:6948", isBound = true },
}
C_ItemSocketInfo._state.hasBoundGemProposed = true
assert(LoadAddOn("Blizzard_ItemSocketingUI"))
ItemSocketingFrame_OnEvent(ItemSocketingFrame, "SOCKET_INFO_UPDATE")
assert(ItemSocketingFrame:IsShown())
assert(ItemSocketingSocket1:IsShown() and ItemSocketingSocket2:IsShown())
assert(not ItemSocketingSocket3:IsShown())
assert(ItemSocketingSocket1IconTexture:GetTexture() == 111)
assert(ItemSocketingSocket2IconTexture:GetTexture() == 222)
assert(ItemSocketingSocket1Shine:IsShown())
assert(ItemSocketingFrame.itemIsBoundTradeable)
assert(ItemSocketingSocketButton:IsEnabled())
ItemSocketingSocket1:GetScript("OnEnter")(ItemSocketingSocket1)
assert(GameTooltip:IsShown() and GameTooltip:IsOwned(ItemSocketingSocket1))
assert(GameTooltip:NumLines() > 0)
