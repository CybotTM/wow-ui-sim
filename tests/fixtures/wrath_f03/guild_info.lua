-- INFERRED legacy selectors from FriendsFrame.lua/xml. Guild challenges are
-- modern-only; preserve the shared MOTD/details behavior on the native panels.
ToggleFriendsFrame(3)
assert(FriendsFrame:IsShown() and GuildFrame:IsShown())
assert(GuildFrameNotesText:GetText():find("Raid invites tonight at 20:00 server.", 1, true) == 1,
    "native MOTD includes the supplied raid notice")
GuildInfoFrame:Show()
assert(GuildInfoEditBox:GetText():find("Mythic%-focused guild recruiting"), "native guild details")
