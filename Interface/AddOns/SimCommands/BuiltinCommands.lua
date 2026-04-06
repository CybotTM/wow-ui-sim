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
