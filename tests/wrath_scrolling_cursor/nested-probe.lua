-- Runs inside the real cursor hook through another active runtime LoadAddOn.
-- Exercise state/widget mutation and actual font measurement while it is active.
local label = C2NestedScroll:CreateFontString(nil, 'ARTWORK')
assert(label:SetFont('Fonts/FRIZQT__.TTF', 20))
label:SetText('nested font access')
assert(label:GetStringWidth() > 0)
label:Hide()
table.insert(C2NestedTrace, 'probe')
