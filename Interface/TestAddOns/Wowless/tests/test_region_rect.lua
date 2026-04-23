test("Region rect init state", function()
    local f = CreateFrame("Frame")
    assertEquals(false, f:IsRectValid())
    assertEquals(0, select('#', f:GetBottom()))
    assertEquals(0, select('#', f:GetCenter()))
    assertEquals(0, f:GetHeight())
    assertEquals(0, f:GetHeight(true))
    assertEquals(0, select('#', f:GetLeft()))
    assertEquals(0, f:GetNumPoints())
    assertEquals(0, select('#', f:GetRect()))
    assertEquals(0, select('#', f:GetRight()))
    assertEquals(0, f:GetWidth())
    assertEquals(0, f:GetWidth(true))
    assertEquals(0, select('#', f:GetTop()))
    assertEquals(false, f:IsRectValid())
end)

test("Region rect SetAllPoints dirty then resolve", function()
    local f = CreateFrame("Frame")
    -- After SetAllPoints, rect should be dirty
    f:SetAllPoints()
    assertEquals(false, f:IsRectValid())
    assertEquals(2, f:GetNumPoints())

    -- GetHeight should resolve the dirty flag
    local h = f:GetHeight()
    print("After GetHeight - height:", h)
    assertEquals(true, f:IsRectValid())

    -- Now check GetBottom
    local bottom = f:GetBottom()
    print("GetBottom:", bottom)
    assertEquals(0, bottom)

    -- Check other values
    local left = f:GetLeft()
    print("GetLeft:", left)
    assertEquals(0, left)
end)

test("Region rect fiveten state", function()
    local f = CreateFrame("Frame")
    f:SetSize(5, 10)
    assertEquals(false, f:IsRectValid())
    assertEquals(10, f:GetHeight())
    assertEquals(10, f:GetHeight(true))
    -- Actually check what the explicit height is:
    print("After SetSize(5,10):")
    print("  GetHeight():", f:GetHeight())
    print("  GetHeight(true):", f:GetHeight(true))
    print("  GetWidth():", f:GetWidth())
    print("  GetWidth(true):", f:GetWidth(true))
    print("  GetSize():", f:GetSize())
    print("  GetSize(true):", f:GetSize(true))
end)

test("Region rect SetAllPoints + SetSize dirty tracking", function()
    local f = CreateFrame("Frame")
    -- Set size first
    f:SetSize(5, 10)
    -- Now set all points
    f:SetAllPoints()
    -- screenfiveten0: should be dirty
    assertEquals(false, f:IsRectValid())
    -- Now resolve via GetBottom
    local bottom = f:GetBottom()
    print("After SetAllPoints+SetSize, GetBottom:", bottom)
    -- screenfiveten1: should be resolved
    assertEquals(true, f:IsRectValid())

    local _, _, w, h = WorldFrame:GetRect()
    print("WorldFrame rect:", w, h)
    print("GetHeight():", f:GetHeight())
    print("GetHeight(true):", f:GetHeight(true))
    print("GetWidth():", f:GetWidth())
    print("GetWidth(true):", f:GetWidth(true))
end)
