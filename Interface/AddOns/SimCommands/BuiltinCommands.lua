-- Built-in commands for the SimCommands palette.

SimCommands:Register("Open Mailbox", "Open simulated mailbox interaction", function()
    A_Admin.OpenMailbox()
end, "UI Panels")

SimCommands:Register("Open Bank", "Fire BANKFRAME_OPENED event", function()
    FireEvent("BANKFRAME_OPENED")
end, "UI Panels")

SimCommands:Register("Open Merchant", "Fire MERCHANT_SHOW event", function()
    FireEvent("MERCHANT_SHOW")
end, "UI Panels")

SimCommands:Register("Talk to Quest NPC", "Open old single-quest gossip dialog", function()
    if A_Admin and A_Admin.OpenQuestNpc then
        A_Admin.OpenQuestNpc()
    else
        print("[SimCommands] Quest NPC admin API unavailable.")
    end
end, "UI Panels")

SimCommands:Register("Talk to Multi-Quest NPC", "Open quest gossip with completed, incomplete, and reward quests", function()
    if A_Admin and A_Admin.OpenMultiQuestNpc then
        A_Admin.OpenMultiQuestNpc()
    else
        print("[SimCommands] Multi-quest NPC admin API unavailable.")
    end
end, "UI Panels")

SimCommands:Register("Open Guild Bank", "Fire GUILDBANKFRAME_OPENED event", function()
    FireEvent("GUILDBANKFRAME_OPENED")
end, "UI Panels")

local function ResolveProfessionSkillLineID(professionEnumID, fallbackSkillLineID)
    if C_TradeSkillUI and C_TradeSkillUI.GetProfessionSkillLineID and professionEnumID then
        local skillLineID = C_TradeSkillUI.GetProfessionSkillLineID(professionEnumID)
        if type(skillLineID) == "number" and skillLineID > 0 then
            return skillLineID
        end
    end
    return fallbackSkillLineID
end

local function OpenProfessionPanel(professionName, professionEnumID, fallbackSkillLineID)
    local skillLineID = ResolveProfessionSkillLineID(professionEnumID, fallbackSkillLineID)
    if not skillLineID then
        print("[SimCommands] Could not resolve " .. professionName .. " skill line ID.")
        return
    end

    if type(OpenProfessionUIToSkillLine) == "function" then
        OpenProfessionUIToSkillLine(skillLineID)
        return
    end

    if C_TradeSkillUI and C_TradeSkillUI.OpenTradeSkill then
        C_TradeSkillUI.OpenTradeSkill(skillLineID)
        return
    end

    print("[SimCommands] Profession UI API unavailable.")
end

local BLACKSMITHING_PROFESSION_ENUM_ID = (Enum and Enum.Profession and Enum.Profession.Blacksmithing) or 1

SimCommands:Register("Open Blacksmithing", "Open the Blacksmithing profession panel", function()
    OpenProfessionPanel("Blacksmithing", BLACKSMITHING_PROFESSION_ENUM_ID, 164)
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

SimCommands:Register("Add Battle Pet", "Collect a battle pet by species ID", function()
    SimCommands:Prompt("Enter pet species ID:", function(text)
        local id = tonumber(text)
        if id and id > 0 then
            A_Admin.SetPetCollected(id, true)
        end
    end)
end, "Collections")

SimCommands:Register("Add Toy", "Collect a toy by item ID", function()
    SimCommands:Prompt("Enter toy item ID:", function(text)
        local id = tonumber(text)
        if id and id > 0 then
            A_Admin.SetToyCollected(id, true)
        end
    end)
end, "Collections")

SimCommands:Register("Add Campsite", "Collect a campsite by warband scene ID", function()
    SimCommands:Prompt("Enter campsite ID:", function(text)
        local id = tonumber(text)
        if id and id > 0 then
            A_Admin.SetCampsiteCollected(id, true)
        end
    end)
end, "Collections")

SimCommands:Register("Earn Achievement", "Earn an achievement by ID", function()
    SimCommands:Prompt("Enter achievement ID:", function(text)
        local id = tonumber(text)
        if id and id > 0 then
            A_Admin.EarnAchievement(id)
        end
    end)
end, "Collections")

local RANDOM_ACHIEVEMENT_IDS = { 6, 7, 8, 9, 10, 11, 776 }

SimCommands:Register("Earn Random Achievement", "Earn a random seeded achievement", function()
    local id = RANDOM_ACHIEVEMENT_IDS[math.random(#RANDOM_ACHIEVEMENT_IDS)]
    if id then
        A_Admin.EarnAchievement(id)
    end
end, "Collections")

SimCommands:Register("Toggle Debug Borders", "Red borders around elements", function()
    A_Admin.ToggleDebugBorders()
end, "Debug")

SimCommands:Register("Toggle Debug Anchors", "Green dots at anchor points", function()
    A_Admin.ToggleDebugAnchors()
end, "Debug")

SimCommands:Register("Reload UI", "Reload the interface (ReloadUI)", function()
    ReloadUI()
end, "Debug")

SimCommands:Register("Set LFG Queue Pop Delay", "Seconds before an LFG queue proposal appears", function()
    SimCommands:Prompt("Enter LFG queue pop delay in seconds:", function(text)
        local delay = tonumber(text)
        if delay and delay >= 0 and A_Admin and A_Admin.SetLfgQueuePopDelay then
            A_Admin.SetLfgQueuePopDelay(delay)
        end
    end)
end, "Group Finder")

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

---------------------------------------------------------------------------
-- Premade Group Finder live simulation
---------------------------------------------------------------------------

local premadeSimRunning = false

local function TickPremadeSimulation()
    if not premadeSimRunning then return end
    local _, results = C_LFGList.GetSearchResults()
    if not results then return end
    for _, resultID in ipairs(results) do
        local info = C_LFGList.GetSearchResultInfo(resultID)
        if info and not info.isDelisted then
            -- Randomly increment member count
            if info.numMembers < info.maxMembers and math.random() < 0.3 then
                A_Admin.UpdatePremadeListing(resultID, "numMembers", info.numMembers + 1)
            end
            -- Mark full groups as delisted
            if info.numMembers >= info.maxMembers and math.random() < 0.5 then
                A_Admin.UpdatePremadeListing(resultID, "isDelisted", true)
            end
        end
    end
    if C_Timer and C_Timer.After then
        C_Timer.After(5, TickPremadeSimulation)
    end
end

function SimCommands.StartPremadeSimulation()
    if premadeSimRunning then return end
    premadeSimRunning = true
    if C_Timer and C_Timer.After then
        C_Timer.After(5, TickPremadeSimulation)
    end
end

function SimCommands.StopPremadeSimulation()
    premadeSimRunning = false
end

function SimCommands.IsPremadeSimulationRunning()
    return premadeSimRunning
end

SimCommands:Register("Toggle Premade Simulation", "Start/stop live group fill simulation", function()
    if premadeSimRunning then
        SimCommands.StopPremadeSimulation()
    else
        SimCommands.StartPremadeSimulation()
    end
end, "Debug")
