-- Supplied outer widgets only. Original TokenUI/Hybrid functions and all currency APIs stay intact.
GameFontDisable = CreateFont('GameFontDisable')
GameFontHighlight = CreateFont('GameFontHighlight')
GameFontDisable:SetFont('Fonts\\FRIZQT__.TTF', 12)
GameFontHighlight:SetFont('Fonts\\FRIZQT__.TTF', 12)
TOO_MANY_WATCHED_TOKENS = 'Only %d currencies may be watched'
BACKPACK_HEIGHT = 200
B5Backpack = CreateFrame('Frame', 'B5Backpack', UIParent)
B5Backpack:SetHeight(BACKPACK_HEIGHT)
-- Outer inventory lookup, not a currency query or a TokenUI handler.
function GetBackpackFrame() return B5Backpack end

TokenFrame = CreateFrame('Frame', 'TokenFrame', UIParent)
TokenFramePopup = CreateFrame('Frame', 'TokenFramePopup', UIParent)
TokenFramePopup:Hide()
TokenFramePopupInactiveCheckBox = CreateFrame('CheckButton', 'TokenFramePopupInactiveCheckBox', TokenFramePopup)
TokenFramePopupBackpackCheckBox = CreateFrame('CheckButton', 'TokenFramePopupBackpackCheckBox', TokenFramePopup)
CharacterFrameTab5 = CreateFrame('Button', 'CharacterFrameTab5', UIParent)
UIErrorsFrame = CreateFrame('ScrollingMessageFrame', 'UIErrorsFrame', UIParent)
TokenFrameContainer = CreateFrame('ScrollFrame', 'TokenFrameContainer', TokenFrame)
TokenFrameContainer:SetSize(313, 160)
TokenFrameContainer.scrollChild = CreateFrame('Frame', 'B5ScrollChild', TokenFrameContainer)
TokenFrameContainer:SetScrollChild(TokenFrameContainer.scrollChild)
TokenFrameContainer.buttons = {}
for i = 1, 6 do
    local name = 'B5TokenButton'..i
    local button = CreateFrame('Button', name, TokenFrameContainer.scrollChild)
    button:SetSize(295, 17)
    for _, suffix in ipairs({'Count', 'Name'}) do
        button:CreateFontString(name..suffix, 'OVERLAY'):SetFont('Fonts\\FRIZQT__.TTF', 12)
    end
    for _, suffix in ipairs({'Icon', 'Check', 'ExpandIcon', 'CategoryLeft', 'CategoryRight', 'Highlight', 'Stripe'}) do
        button:CreateTexture(name..suffix, 'ARTWORK')
    end
    button.LinkButton = CreateFrame('Button', nil, button)
    TokenButton_OnLoad(button)
    TokenFrameContainer.buttons[i] = button
end
BackpackTokenFrame = CreateFrame('Frame', 'BackpackTokenFrame', UIParent)
for i = 1, 3 do
    local name = 'BackpackTokenFrameToken'..i
    local button = CreateFrame('Button', name, BackpackTokenFrame)
    button.icon = button:CreateTexture(nil, 'ARTWORK')
    button.count = button:CreateFontString(nil, 'OVERLAY')
    button.count:SetFont('Fonts\\FRIZQT__.TTF', 12)
end
