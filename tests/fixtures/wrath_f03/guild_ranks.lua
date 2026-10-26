-- INFERRED from FriendsFrame.lua: native control lists every rank, including
-- the leader; modern assignment-only dropdown exclusions are a separate target.
assert(UIDropDownMenu_GetSelectedID(GuildControlPopupFrameDropDown) == 1)
assert(GuildControlPopupFrameDropDownText:GetText() == "Guild Leader")
ToggleDropDownMenu(1, nil, GuildControlPopupFrameDropDown)
assert(DropDownList1.numButtons == 3)
for index, expected in ipairs({"Guild Leader", "Officer", "Member"}) do
    local button = _G["DropDownList1Button" .. index]
    assert(button:IsShown() and button:GetText() == expected, "native rank menu entry")
end
local officer = DropDownList1Button2
officer:GetScript("OnClick")(officer, "LeftButton")
assert(UIDropDownMenu_GetSelectedID(GuildControlPopupFrameDropDown) == 2)
assert(GuildControlPopupFrameDropDownText:GetText() == "Officer")
