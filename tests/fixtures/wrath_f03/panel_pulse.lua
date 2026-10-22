-- Native center-panel displacement; registration is test-owned table data.
local first = CreateFrame("Frame", "NativePanelPulseFirst", UIParent)
first:SetSize(300, 400)
first:Hide()
UIPanelWindows[first:GetName()] = { area = "center", pushable = 0, whileDead = 1 }
local second = CreateFrame("Frame", "NativePanelPulseSecond", UIParent)
second:SetSize(300, 400)
second:Hide()
UIPanelWindows[second:GetName()] = { area = "center", pushable = 0, whileDead = 1 }
ShowUIPanel(first)
assert(first:IsShown() and not second:IsShown(), "first native pulse")
assert(GetUIPanel("center") == first, "first pulse owns center")
ShowUIPanel(second)
assert(not first:IsShown() and second:IsShown(), "second native pulse replaces first")
assert(GetUIPanel("center") == second, "second pulse owns center")
CloseAllWindows()
assert(not first:IsShown() and not second:IsShown(), "native close drains both panels")
