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

-- Gear set presets: { name, description, items = { [slot] = itemId, ... } }
local GEAR_PRESETS = {
    {
        name = "Entombed Seraph (ilvl 571)",
        description = "Plate DPS set — default Ret Paladin gear",
        items = {
            [1]=211993, [2]=230637, [3]=211991, [5]=211996,
            [6]=211990, [7]=211992, [8]=211995, [9]=211989,
            [10]=211994, [11]=225748, [12]=215135, [13]=218715,
            [14]=236914, [15]=211988, [16]=229181,
        },
    },
    {
        name = "Naked (no gear)",
        description = "Unequip all slots",
        items = {},
    },
}

for _, preset in ipairs(GEAR_PRESETS) do
    SimCommands:Register("Equip: " .. preset.name, preset.description, function()
        -- Clear all slots first
        for slot = 1, 19 do
            pcall(A_Admin.UnequipItem, slot)
        end
        -- Equip preset items
        for slot, itemId in pairs(preset.items) do
            A_Admin.EquipItem(slot, itemId)
        end
    end, "Player State")
end

SimCommands:Register("Join Guild", "Join a guild (prompts for name)", function()
    SimCommands:Prompt("Enter guild name:", function(text)
        if text and text ~= "" then
            A_Admin.JoinGuild(text, "Member", 150)
        end
    end)
end, "Player State")

SimCommands:Register("Leave Guild", "Leave current guild", function()
    A_Admin.LeaveGuild()
end, "Player State")

SimCommands:Register("Add Mount", "Collect a mount by ID", function()
    SimCommands:Prompt("Enter mount ID:", function(text)
        local id = tonumber(text)
        if id and id > 0 then
            A_Admin.SetMountCollected(id, true)
        end
    end)
end, "Collections")

SimCommands:Register("Set Honor Level", "Change PvP honor level (1-500)", function()
    SimCommands:Prompt("Enter honor level (1-500):", function(text)
        local level = tonumber(text)
        if level and level >= 1 and level <= 500 then
            A_Admin.SetHonorLevel(level)
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
