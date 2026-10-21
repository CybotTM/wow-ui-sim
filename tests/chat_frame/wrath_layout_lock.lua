-- Native FloatingChatFrameTemplate: 430x120, separate scroll buttons.
local chat = assert(ChatFrame1, "native chat frame")
assert(chat:IsShown(), "native chat hidden")
assert(chat:GetWidth() == 430 and chat:GetHeight() == 120, "native chat dimensions")
assert(ChatFrame1Background, "native chat background")
assert(chat.editBox == ChatFrame1EditBox, "native editbox identity")
assert(chat.resizeButton == ChatFrame1ResizeButton, "native resize button identity")
local controls = assert(chat.buttonFrame, "native scroll controls")
assert(controls:GetWidth() == 29, "native scrolling button frame width")
assert(controls:GetHeight() == chat:GetHeight(), "native scroll controls height")
assert(controls.upButton and controls.downButton and controls.bottomButton, "native scrolling buttons missing")
local left, bottom, width = chat:GetRect()
local controlsLeft, controlsBottom, controlsWidth = controls:GetRect()
assert(left and bottom and controlsLeft and controlsBottom, "native chat rect unresolved")
assert(math.abs(controlsBottom - bottom) < 0.75, "native scroll controls bottom")
if chat.buttonSide == "left" then
    assert(math.abs(controlsLeft + controlsWidth - (left - 4)) < 0.75, "native left controls anchor")
else
    assert(chat.buttonSide == "right", "native button side missing")
    assert(math.abs(controlsLeft - (left + width + 4)) < 0.75, "native right controls anchor")
end
return "ok"
