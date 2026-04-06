-- Built-in commands for the SimCommands palette.

SimCommands:Register("Open Mailbox", "Fire MAIL_SHOW event", function()
    FireEvent("MAIL_SHOW")
end, "UI Panels")

SimCommands:Register("Open Bank", "Fire BANKFRAME_OPENED event", function()
    FireEvent("BANKFRAME_OPENED")
end, "UI Panels")

SimCommands:Register("Open Merchant", "Fire MERCHANT_SHOW event", function()
    FireEvent("MERCHANT_SHOW")
end, "UI Panels")

SimCommands:Register("Open Guild Bank", "Fire GUILDBANKFRAME_OPENED event", function()
    FireEvent("GUILDBANKFRAME_OPENED")
end, "UI Panels")

SimCommands:Register("Add Gold", "Add gold to player (enter amount in gold)", function()
    SimCommands:Prompt("Enter gold amount:", function(text)
        local gold = tonumber(text)
        if gold and gold > 0 then
            local current = GetMoney() or 0
            A_Admin.SetMoney(current + gold * 10000)
        end
    end)
end, "Player State")

SimCommands:Register("Set Player Level", "Change player level (1-80)", function()
    SimCommands:Prompt("Enter level (1-80):", function(text)
        local level = tonumber(text)
        if level and level >= 1 and level <= 80 then
            A_Admin.SetPlayerLevel(level)
        end
    end)
end, "Player State")
