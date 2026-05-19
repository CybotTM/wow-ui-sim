
        if not MainActionBar then return end
        local w, h = MainActionBar:GetSize()
        if w == 562 and h == 45 then return end
        -- Compute the bar bounds from the actual button grid. Border art and
        -- end caps are anchored outside the frame; baking them into the frame
        -- size shifts the whole bar off-center.
        local lastOx = 0
        local buttonWidth = 45
        local buttonHeight = 45
        for i = 1, 12 do
            local c = _G["MainActionBarButtonContainer" .. i]
            local isShown = not c or not c.IsShown or c:IsShown()
            if c and isShown then
                local cw, ch = c:GetSize()
                if cw and cw == cw and cw > 0 then
                    buttonWidth = cw
                end
                if ch and ch == ch and ch > 0 then
                    buttonHeight = ch
                end
            end
            if c and isShown and c:GetNumPoints() > 0 then
                local point, _, _, ox, _ = c:GetPoint(1)
                if point == "BOTTOMLEFT" and ox and ox == ox and ox > lastOx then
                    lastOx = ox
                end
            end
        end
        MainActionBar:SetSize(lastOx + buttonWidth, buttonHeight)
    