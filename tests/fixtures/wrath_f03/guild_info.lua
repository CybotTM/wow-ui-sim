-- INFERRED legacy selectors from FriendsFrame.lua/xml. Guild challenges are
-- modern-only; preserve the shared MOTD/details behavior on the native panels.
ToggleFriendsFrame(3)
assert(FriendsFrame:IsShown() and GuildFrame:IsShown())
assert(GuildFrameNotesText:GetText() == "Raid invites tonight at 20:00 server.", "native MOTD")
GuildInfoFrame:Show()
assert(GuildInfoEditBox:GetText():find("Mythic%-focused guild recruiting"), "native guild details")
