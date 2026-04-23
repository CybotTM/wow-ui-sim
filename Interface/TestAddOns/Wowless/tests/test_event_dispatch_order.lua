test("individual event fires before all-events", function()
    local log = {}
    local f1 = CreateFrame("Frame")
    local f2 = CreateFrame("Frame")
    local f3 = CreateFrame("Frame")
    local f4 = CreateFrame("Frame")

    f1:SetScript("OnEvent", function() table.insert(log, "individual1") end)
    f2:SetScript("OnEvent", function() table.insert(log, "all1") end)
    f3:SetScript("OnEvent", function() table.insert(log, "all2") end)
    f4:SetScript("OnEvent", function() table.insert(log, "individual2") end)

    f1:RegisterEvent("CHAT_MSG_SYSTEM")
    f2:RegisterAllEvents()
    f3:RegisterAllEvents()
    f4:RegisterEvent("CHAT_MSG_SYSTEM")

    SendSystemMessage("test")

    assertEquals("individual1,individual2,all1,all2", table.concat(log, ","))
end)
