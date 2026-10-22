-- Native UIParent data registration; no RegisterUIPanel shim capability claim.
local panel = CreateFrame("Frame", "NativeShowHidePanel", UIParent)
panel:SetSize(300, 400)
panel:Hide()
UIPanelWindows[panel:GetName()] = { area = "center", pushable = 0, whileDead = 1 }
assert(not panel:IsShown(), "native panel starts hidden")
ShowUIPanel(panel)
assert(panel:IsShown(), "native panel opens")
assert(GetUIPanel("center") == panel, "native panel enters the center stack")
HideUIPanel(panel)
assert(not panel:IsShown(), "native panel hides")
assert(GetUIPanel("center") ~= panel, "native panel leaves the center stack")
