-- Built-in commands for the SimCommands palette.

SimCommands:Register("Open Mailbox", "Fire MAIL_SHOW event", function()
    FireEvent("MAIL_SHOW")
end, "UI Panels")
