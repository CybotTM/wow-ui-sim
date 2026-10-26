-- Native 12340 MainMenuBarMicroButtons.xml: 28x58 buttons, -3px joins.
UpdateMicroButtons()
local buttons = {
    CharacterMicroButton, SpellbookMicroButton, TalentMicroButton,
    AchievementMicroButton, QuestLogMicroButton, SocialsMicroButton,
    PVPMicroButton, LFDMicroButton, MainMenuMicroButton,
}
assert(#buttons == 9, "native micro buttons missing")
for index, button in ipairs(buttons) do
    assert(button:GetParent() == MainMenuBarArtFrame, "native micro-button parent")
    assert(button:GetWidth() == 28 and button:GetHeight() == 58, "native micro-button size")
    local left, bottom, width = button:GetRect()
    assert(left and bottom and width, "native micro-button layout unresolved")
    if index > 1 then
        local previousLeft, previousBottom, previousWidth = buttons[index - 1]:GetRect()
        assert(math.abs(left - (previousLeft + previousWidth - 3)) < 0.75, "native micro-button horizontal join")
        assert(math.abs(bottom - previousBottom) < 0.75, "native micro-button baseline")
    end
end
