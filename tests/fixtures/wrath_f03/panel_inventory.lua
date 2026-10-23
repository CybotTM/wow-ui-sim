-- INFERRED from native UIParent.lua; modern Communities/PVE/professions names
-- are not the legacy guild, dungeon-finder, and spellbook window identities.
for _, entry in ipairs({
    {"CharacterFrame", "left"}, {"FriendsFrame", "left"},
    {"GameMenuFrame", "center"}, {"HelpFrame", "center"},
    {"SpellBookFrame", "left"}, {"LFDParentFrame", "left"},
    {"MailFrame", "left"}, {"MerchantFrame", "left"},
    {"BankFrame", "left"}, {"GossipFrame", "left"},
    {"DressUpFrame", "left"}, {"QuestFrame", "left"},
}) do
    local info = assert(UIPanelWindows[entry[1]], "unregistered " .. entry[1])
    assert(info.area == entry[2], "wrong native area for " .. entry[1])
end
