-- Proves native CloseAllWindows ownership, not RegisterUIPanel shim behavior.
local panel = CreateFrame("Frame", "NativeCloseStackPanel", UIParent)
panel:SetSize(300, 400)
panel:Hide()
UIPanelWindows[panel:GetName()] = { area = "center", pushable = 0, whileDead = 1 }
assert(panel.editModeManuallyShown == nil, "native stack is not manual EditMode visibility")
ShowUIPanel(panel)
assert(panel:IsShown() and GetUIPanel("center") == panel, "native stack owns shown panel")
CloseAllWindows()
assert(not panel:IsShown(), "native registered panel closes with the escape stack")
