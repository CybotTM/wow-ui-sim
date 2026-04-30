-- Admin.lua
-- Slash command interface for the A_Admin simulator API.
-- Usage: /admin <subcommand> [args...]  or  /aa <subcommand> [args...]

local function ParseBool(str)
    if str == "on" or str == "true" or str == "1" then return true end
    if str == "off" or str == "false" or str == "0" then return false end
    return nil
end

local function Val(v)
    return "|cff00ff00" .. tostring(v) .. "|r"
end

local function Confirm(msg)
    print("[Admin] " .. msg)
end

local function Err(msg)
    print("[Admin] |cffff4444Error:|r " .. msg)
end

SlashCmdList = SlashCmdList or {}

local function NormalizeTrackedRecipeBucket(bucket)
    local cleaned = {}
    local seen = {}
    if type(bucket) ~= "table" then
        return cleaned
    end

    for i = 1, #bucket do
        local recipeID = tonumber(bucket[i])
        if recipeID ~= nil then
            recipeID = math.floor(recipeID)
        end
        if recipeID ~= nil and recipeID > 0 and not seen[recipeID] then
            seen[recipeID] = true
            cleaned[#cleaned + 1] = recipeID
        end
    end

    return cleaned
end

local function EnsureTrackedRecipeDB()
    local db = WowSimTrackedRecipesDB
    if type(db) ~= "table" then
        db = {}
    end
    db.normal = NormalizeTrackedRecipeBucket(db.normal)
    db.recrafting = NormalizeTrackedRecipeBucket(db.recrafting)
    WowSimTrackedRecipesDB = db
    return db
end

local function TrackedRecipeBucket(db, isRecrafting)
    return isRecrafting and db.recrafting or db.normal
end

local function UpdateTrackedRecipeDB(recipeID, tracked, isRecrafting)
    recipeID = tonumber(recipeID)
    if recipeID == nil then
        return
    end
    recipeID = math.floor(recipeID)
    if recipeID <= 0 then
        return
    end

    local bucket = TrackedRecipeBucket(EnsureTrackedRecipeDB(), not not isRecrafting)
    local existingIndex = nil
    for i = 1, #bucket do
        if bucket[i] == recipeID then
            existingIndex = i
            break
        end
    end

    if tracked then
        if existingIndex == nil then
            bucket[#bucket + 1] = recipeID
        end
    elseif existingIndex ~= nil then
        table.remove(bucket, existingIndex)
    end
end

local function ReplayTrackedRecipeBucket(bucket, isRecrafting)
    if not (C_TradeSkillUI and C_TradeSkillUI.SetRecipeTracked) then
        return
    end
    for i = 1, #bucket do
        C_TradeSkillUI.SetRecipeTracked(bucket[i], true, isRecrafting)
    end
end

local function InstallTrackedRecipePersistence()
    EnsureTrackedRecipeDB()

    if not __wow_sim_admin_tracked_recipe_hook_installed and C_TradeSkillUI and C_TradeSkillUI.SetRecipeTracked then
        __wow_sim_admin_tracked_recipe_hook_installed = true
        hooksecurefunc(C_TradeSkillUI, "SetRecipeTracked", function(recipeID, tracked, isRecrafting)
            UpdateTrackedRecipeDB(recipeID, tracked, isRecrafting)
        end)
    end

    if not __wow_sim_admin_tracked_recipe_frame then
        local frame = CreateFrame("Frame")
        frame:RegisterEvent("PLAYER_LOGIN")
        frame:SetScript("OnEvent", function()
            local db = EnsureTrackedRecipeDB()
            ReplayTrackedRecipeBucket(db.normal, false)
            ReplayTrackedRecipeBucket(db.recrafting, true)
        end)
        __wow_sim_admin_tracked_recipe_frame = frame
    end
end

InstallTrackedRecipePersistence()

-- ---------------------------------------------------------------------------
-- Help text
-- ---------------------------------------------------------------------------

local HELP = {
    { "--- Identity ---" },
    { "/aa name <name>",                    "Set player name" },
    { "/aa class <index>",                  "Set player class (1-12)" },
    { "/aa race <index>",                   "Set player race" },
    { "/aa level <level>",                  "Set player level" },
    { "--- Combat ---" },
    { "/aa combat [on|off]",                "Toggle combat state" },
    { "/aa rest [on|off]",                  "Toggle resting (inn/city)" },
    { "/aa cast <spellId> <name> <icon> <duration>", "Start casting" },
    { "/aa stopcast",                       "Stop casting" },
    { "/aa gcd <duration>",                 "Trigger GCD" },
    { "/aa cd <spellId> <duration>",        "Set spell cooldown" },
    { "--- Health / Power ---" },
    { "/aa health <cur> <max>",             "Set player health" },
    { "/aa power <cur> <max> [powerType]",  "Set player power" },
    { "/aa thealth <cur> <max>",            "Set target health" },
    { "--- Targeting ---" },
    { "/aa target <name> <level> <class> [enemy]", "Set target (enemy defaults off)" },
    { "/aa cleartarget",                    "Clear target" },
    { "/aa tpower <cur> <max> [powerType]", "Set target power" },
    { "/aa ttype <class> <creature> <reaction>", "Set target type (e.g. elite Beast 2)" },
    { "/aa focus <name> <level> <class> [enemy]",  "Set focus" },
    { "/aa clearfocus",                     "Clear focus" },
    { "/aa fhealth <cur> <max>",            "Set focus health" },
    { "/aa fpower <cur> <max> [powerType]", "Set focus power" },
    { "/aa ftype <class> <creature> <reaction>", "Set focus type" },
    { "--- Party ---" },
    { "/aa party <size>",                   "Set party size (0-4)" },
    { "/aa partyleader <player|idx>",       "Set party leader (player or party index)" },
    { "/aa partymember <idx> <name> <class> <level>", "Set party member info" },
    { "/aa kill <idx>",                     "Kill party member" },
    { "/aa res <idx>",                      "Resurrect party member" },
    { "/aa rotdmg <level>",                 "Set rot damage level" },
    { "--- Movement ---" },
    { "/aa moving [on|off]",                "Toggle moving" },
    { "/aa mounted [on|off]",               "Toggle mounted" },
    { "/aa flying [on|off]",                "Toggle flying" },
    { "/aa falling [on|off]",               "Toggle falling" },
    { "/aa swimming [on|off]",              "Toggle swimming" },
    { "--- Spec / Talents ---" },
    { "/aa spec <index>",                   "Set active spec" },
    { "/aa talents reset",                  "Reset all talents" },
    { "--- Buffs ---" },
    { "/aa buff <spellId> <name> <icon> <duration> <stacks>", "Add buff" },
    { "/aa debuff <spellId>",               "Remove buff by spell ID" },
    { "/aa clearbuffs",                     "Clear all buffs" },
    { "--- Zone / Instance ---" },
    { "/aa zone <name> <id>",               "Set zone" },
    { "/aa subzone <name>",                 "Set sub-zone" },
    { "/aa instance <name> <type> <difficulty> <maxPlayers>", "Set instance info" },
    { "/aa ininstance [on|off]",            "Toggle in-instance flag" },
    { "--- Economy ---" },
    { "/aa money <copper>",                 "Set money (in copper)" },
    { "/aa ilvl <level>",                   "Set item level" },
    { "--- PvP / Guild ---" },
    { "/aa pvp [on|off]",                   "Toggle PvP" },
    { "/aa honor <level>",                  "Set honor level" },
    { "/aa guild <name> <rank> <members>",  "Set guild info" },
    { "/aa noguild",                        "Clear guild" },
    { "--- Action Bars ---" },
    { "/aa actionslot <slot> <spellId>",    "Set action bar slot (1-120)" },
    { "/aa clearslot <slot>",               "Clear action bar slot" },
    { "/aa clearactions",                   "Clear all action bar slots" },
    { "--- Great Vault ---" },
    { "/aa vault activity <type> <idx> <threshold> <progress> <level>", "Set vault slot" },
    { "/aa vault rewards [on|off] [canClaim]", "Toggle vault rewards" },
    { "/aa vault clear",                    "Clear all vault data" },
    { "--- Events ---" },
    { "/aa event <name> [args...]",         "Fire a WoW event" },
}

local function ShowHelp()
    print("|cffffd700[Admin] Available commands:|r")
    for _, entry in ipairs(HELP) do
        if #entry == 1 then
            print("|cffa0a0a0" .. entry[1] .. "|r")
        else
            print(string.format("  %-52s %s", entry[1], entry[2]))
        end
    end
end

-- ---------------------------------------------------------------------------
-- Subcommand dispatch
-- ---------------------------------------------------------------------------

local handlers = {}

-- Identity

handlers["name"] = function(args)
    local name = args[1]
    if not name then return Err("Usage: /aa name <name>") end
    A_Admin.SetPlayerName(name)
    Confirm("Player name set to " .. Val(name))
end

handlers["class"] = function(args)
    local idx = tonumber(args[1])
    if not idx then return Err("Usage: /aa class <index>") end
    A_Admin.SetPlayerClass(idx)
    Confirm("Player class set to index " .. Val(idx))
end

handlers["race"] = function(args)
    local idx = tonumber(args[1])
    if not idx then return Err("Usage: /aa race <index>") end
    A_Admin.SetPlayerRace(idx)
    Confirm("Player race set to index " .. Val(idx))
end

handlers["level"] = function(args)
    local lvl = tonumber(args[1])
    if not lvl then return Err("Usage: /aa level <level>") end
    A_Admin.SetPlayerLevel(lvl)
    Confirm("Player level set to " .. Val(lvl))
end

-- Combat

handlers["combat"] = function(args)
    local b = ParseBool(args[1])
    if b == nil then return Err("Usage: /aa combat [on|off]") end
    A_Admin.SetInCombat(b)
    Confirm("Combat: " .. Val(b and "on" or "off"))
end

handlers["rest"] = function(args)
    local b = ParseBool(args[1])
    if b == nil then return Err("Usage: /aa rest [on|off]") end
    A_Admin.SetResting(b)
    Confirm("Resting: " .. Val(b and "on" or "off"))
end

handlers["cast"] = function(args)
    local spellId  = tonumber(args[1])
    local name     = args[2]
    local icon     = args[3]
    local duration = tonumber(args[4])
    if not spellId or not name or not icon or not duration then
        return Err("Usage: /aa cast <spellId> <name> <icon> <duration>")
    end
    A_Admin.SetCasting(spellId, name, icon, duration)
    Confirm("Casting " .. Val(name) .. " (spell " .. Val(spellId) .. ", " .. Val(duration) .. "s)")
end

handlers["stopcast"] = function(_args)
    A_Admin.StopCasting()
    Confirm("Casting stopped")
end

handlers["gcd"] = function(args)
    local dur = tonumber(args[1])
    if not dur then return Err("Usage: /aa gcd <duration>") end
    A_Admin.SetGCD(dur)
    Confirm("GCD triggered: " .. Val(dur) .. "s")
end

handlers["cd"] = function(args)
    local spellId = tonumber(args[1])
    local dur     = tonumber(args[2])
    if not spellId or not dur then return Err("Usage: /aa cd <spellId> <duration>") end
    A_Admin.SetSpellCooldown(spellId, dur)
    Confirm("Cooldown for spell " .. Val(spellId) .. " set to " .. Val(dur) .. "s")
end

-- Health / Power

handlers["health"] = function(args)
    local cur = tonumber(args[1])
    local max = tonumber(args[2])
    if not cur or not max then return Err("Usage: /aa health <cur> <max>") end
    A_Admin.SetPlayerHealth(cur, max)
    Confirm("Player health: " .. Val(cur) .. "/" .. Val(max))
end

handlers["power"] = function(args)
    local cur       = tonumber(args[1])
    local max       = tonumber(args[2])
    local powerType = args[3] and tonumber(args[3])
    if not cur or not max then return Err("Usage: /aa power <cur> <max> [powerType]") end
    if powerType then
        A_Admin.SetPlayerPower(cur, max, powerType)
        Confirm("Player power: " .. Val(cur) .. "/" .. Val(max) .. " (type " .. Val(powerType) .. ")")
    else
        A_Admin.SetPlayerPower(cur, max)
        Confirm("Player power: " .. Val(cur) .. "/" .. Val(max))
    end
end

handlers["thealth"] = function(args)
    local cur = tonumber(args[1])
    local max = tonumber(args[2])
    if not cur or not max then return Err("Usage: /aa thealth <cur> <max>") end
    A_Admin.SetTargetHealth(cur, max)
    Confirm("Target health: " .. Val(cur) .. "/" .. Val(max))
end

-- Targeting

handlers["target"] = function(args)
    local name  = args[1]
    local level = tonumber(args[2])
    local class = tonumber(args[3])
    local enemy = ParseBool(args[4] or "false")
    if not name or not level or not class then
        return Err("Usage: /aa target <name> <level> <class> [enemy]")
    end
    A_Admin.SetTarget(name, level, class, enemy)
    Confirm("Target: " .. Val(name) .. " lv" .. Val(level) .. " class " .. Val(class) .. (enemy and " (enemy)" or ""))
end

handlers["ttype"] = function(args)
    local classification = args[1]
    local creatureType   = args[2]
    local reaction       = args[3] and tonumber(args[3])
    if not classification then return Err("Usage: /aa ttype <classification> <creatureType> <reaction>") end
    A_Admin.SetTargetType(classification, creatureType, reaction)
    Confirm("Target type: " .. Val(classification) .. " " .. Val(creatureType or "nil") .. " reaction=" .. Val(reaction or "nil"))
end

handlers["tpower"] = function(args)
    local cur       = tonumber(args[1])
    local max       = tonumber(args[2])
    local powerType = args[3] and tonumber(args[3])
    if not cur or not max then return Err("Usage: /aa tpower <cur> <max> [powerType]") end
    A_Admin.SetTargetPower(cur, max, powerType)
    Confirm("Target power: " .. Val(cur) .. "/" .. Val(max))
end

handlers["cleartarget"] = function(_args)
    A_Admin.ClearTarget()
    Confirm("Target cleared")
end

handlers["focus"] = function(args)
    local name  = args[1]
    local level = tonumber(args[2])
    local class = tonumber(args[3])
    local enemy = ParseBool(args[4] or "false")
    if not name or not level or not class then
        return Err("Usage: /aa focus <name> <level> <class> [enemy]")
    end
    A_Admin.SetFocus(name, level, class, enemy)
    Confirm("Focus: " .. Val(name) .. " lv" .. Val(level) .. " class " .. Val(class) .. (enemy and " (enemy)" or ""))
end

handlers["clearfocus"] = function(_args)
    A_Admin.ClearFocus()
    Confirm("Focus cleared")
end

handlers["fhealth"] = function(args)
    local cur = tonumber(args[1])
    local max = tonumber(args[2])
    if not cur or not max then return Err("Usage: /aa fhealth <cur> <max>") end
    A_Admin.SetFocusHealth(cur, max)
    Confirm("Focus health: " .. Val(cur) .. "/" .. Val(max))
end

handlers["ftype"] = function(args)
    local classification = args[1]
    local creatureType   = args[2]
    local reaction       = args[3] and tonumber(args[3])
    if not classification then return Err("Usage: /aa ftype <classification> <creatureType> <reaction>") end
    A_Admin.SetFocusType(classification, creatureType, reaction)
    Confirm("Focus type: " .. Val(classification) .. " " .. Val(creatureType or "nil") .. " reaction=" .. Val(reaction or "nil"))
end

handlers["fpower"] = function(args)
    local cur       = tonumber(args[1])
    local max       = tonumber(args[2])
    local powerType = args[3] and tonumber(args[3])
    if not cur or not max then return Err("Usage: /aa fpower <cur> <max> [powerType]") end
    A_Admin.SetFocusPower(cur, max, powerType)
    Confirm("Focus power: " .. Val(cur) .. "/" .. Val(max))
end

-- Party

handlers["party"] = function(args)
    local n = tonumber(args[1])
    if not n then return Err("Usage: /aa party <size>") end
    A_Admin.SetPartySize(n)
    Confirm("Party size: " .. Val(n))
end

handlers["partyleader"] = function(args)
    local raw = args[1]
    if not raw then return Err("Usage: /aa partyleader <player|idx>") end
    local leader = 0
    if raw ~= "player" then
        leader = tonumber(raw)
        if not leader then
            return Err("Usage: /aa partyleader <player|idx>")
        end
    end
    A_Admin.SetPartyLeader(leader)
    Confirm("Party leader: " .. Val(raw == "player" and "player" or leader))
end

handlers["partymember"] = function(args)
    local idx   = tonumber(args[1])
    local name  = args[2]
    local class = tonumber(args[3])
    local level = tonumber(args[4])
    if not idx or not name or not class or not level then
        return Err("Usage: /aa partymember <idx> <name> <class> <level>")
    end
    A_Admin.SetPartyMember(idx, name, class, level)
    Confirm("Party member " .. Val(idx) .. ": " .. Val(name) .. " lv" .. Val(level) .. " class " .. Val(class))
end

handlers["kill"] = function(args)
    local idx = tonumber(args[1])
    if not idx then return Err("Usage: /aa kill <idx>") end
    A_Admin.KillPartyMember(idx)
    Confirm("Party member " .. Val(idx) .. " killed")
end

handlers["res"] = function(args)
    local idx = tonumber(args[1])
    if not idx then return Err("Usage: /aa res <idx>") end
    A_Admin.ResPartyMember(idx)
    Confirm("Party member " .. Val(idx) .. " resurrected")
end

handlers["rotdmg"] = function(args)
    local lvl = tonumber(args[1])
    if not lvl then return Err("Usage: /aa rotdmg <level>") end
    A_Admin.SetRotDamage(lvl)
    Confirm("Rot damage level: " .. Val(lvl))
end

-- Movement

local function MovementHandler(apiName, label)
    return function(args)
        local b = ParseBool(args[1])
        if b == nil then return Err("Usage: /aa " .. label .. " [on|off]") end
        A_Admin[apiName](b)
        Confirm(label:sub(1,1):upper() .. label:sub(2) .. ": " .. Val(b and "on" or "off"))
    end
end

handlers["moving"]   = MovementHandler("SetMoving",    "moving")
handlers["mounted"]  = MovementHandler("SetMounted",   "mounted")
handlers["flying"]   = MovementHandler("SetFlying",    "flying")
handlers["falling"]  = MovementHandler("SetFalling",   "falling")
handlers["swimming"] = MovementHandler("SetSwimming",  "swimming")

-- Spec / Talents

handlers["spec"] = function(args)
    local idx = tonumber(args[1])
    if not idx then return Err("Usage: /aa spec <index>") end
    A_Admin.SetSpec(idx)
    Confirm("Active spec: " .. Val(idx))
end

handlers["talents"] = function(args)
    if args[1] == "reset" then
        A_Admin.ResetTalents()
        Confirm("Talents reset")
    else
        Err("Usage: /aa talents reset")
    end
end

-- Buffs

handlers["buff"] = function(args)
    local spellId  = tonumber(args[1])
    local name     = args[2]
    local icon     = args[3]
    local duration = tonumber(args[4])
    local stacks   = tonumber(args[5])
    if not spellId or not name or not icon or not duration or not stacks then
        return Err("Usage: /aa buff <spellId> <name> <icon> <duration> <stacks>")
    end
    A_Admin.AddBuff(spellId, name, icon, duration, stacks)
    Confirm("Buff added: " .. Val(name) .. " (" .. Val(stacks) .. " stack(s), " .. Val(duration) .. "s)")
end

handlers["debuff"] = function(args)
    local spellId = tonumber(args[1])
    if not spellId then return Err("Usage: /aa debuff <spellId>") end
    A_Admin.RemoveBuff(spellId)
    Confirm("Buff removed: spell " .. Val(spellId))
end

handlers["clearbuffs"] = function(_args)
    A_Admin.ClearBuffs()
    Confirm("All buffs cleared")
end

-- Zone / Instance

handlers["zone"] = function(args)
    local name = args[1]
    local id   = tonumber(args[2])
    if not name or not id then return Err("Usage: /aa zone <name> <id>") end
    A_Admin.SetZone(name, id)
    Confirm("Zone: " .. Val(name) .. " (id " .. Val(id) .. ")")
end

handlers["subzone"] = function(args)
    local name = args[1]
    if not name then return Err("Usage: /aa subzone <name>") end
    A_Admin.SetSubZone(name)
    Confirm("Sub-zone: " .. Val(name))
end

handlers["instance"] = function(args)
    local name       = args[1]
    local itype      = args[2]
    local difficulty = tonumber(args[3])
    local maxPlayers = tonumber(args[4])
    if not name or not itype or not difficulty or not maxPlayers then
        return Err("Usage: /aa instance <name> <type> <difficulty> <maxPlayers>")
    end
    A_Admin.SetInstanceInfo(name, itype, difficulty, maxPlayers)
    Confirm("Instance: " .. Val(name) .. " " .. Val(itype) .. "/" .. Val(difficulty) .. " (" .. Val(maxPlayers) .. " players)")
end

handlers["ininstance"] = function(args)
    local b = ParseBool(args[1])
    if b == nil then return Err("Usage: /aa ininstance [on|off]") end
    A_Admin.SetInInstance(b)
    Confirm("In instance: " .. Val(b and "on" or "off"))
end

-- Economy

handlers["money"] = function(args)
    local copper = tonumber(args[1])
    if not copper then return Err("Usage: /aa money <copper>") end
    A_Admin.SetMoney(copper)
    local gold   = math.floor(copper / 10000)
    local silver = math.floor((copper % 10000) / 100)
    local rem    = copper % 100
    Confirm("Money set to " .. Val(gold .. "g " .. silver .. "s " .. rem .. "c") .. " (" .. copper .. " copper)")
end

handlers["ilvl"] = function(args)
    local lvl = tonumber(args[1])
    if not lvl then return Err("Usage: /aa ilvl <level>") end
    A_Admin.SetItemLevel(lvl)
    Confirm("Item level: " .. Val(lvl))
end

-- PvP / Guild

handlers["pvp"] = function(args)
    local b = ParseBool(args[1])
    if b == nil then return Err("Usage: /aa pvp [on|off]") end
    A_Admin.SetPvPEnabled(b)
    Confirm("PvP: " .. Val(b and "on" or "off"))
end

handlers["honor"] = function(args)
    local lvl = tonumber(args[1])
    if not lvl then return Err("Usage: /aa honor <level>") end
    A_Admin.SetHonorLevel(lvl)
    Confirm("Honor level: " .. Val(lvl))
end

handlers["guild"] = function(args)
    local name    = args[1]
    local rank    = args[2]
    local members = tonumber(args[3])
    if not name or not rank or not members then
        return Err("Usage: /aa guild <name> <rank> <members>")
    end
    A_Admin.SetGuildInfo(name, rank, members)
    Confirm("Guild: " .. Val(name) .. " [" .. rank .. "] (" .. Val(members) .. " members)")
end

handlers["noguild"] = function(_args)
    A_Admin.ClearGuild()
    Confirm("Guild cleared")
end

-- Action Bars

handlers["actionslot"] = function(args)
    local slot    = tonumber(args[1])
    local spellId = tonumber(args[2])
    if not slot or not spellId then return Err("Usage: /aa actionslot <slot> <spellId>") end
    A_Admin.SetActionSlot(slot, spellId)
    Confirm("Action slot " .. Val(slot) .. " = spell " .. Val(spellId))
end

handlers["clearslot"] = function(args)
    local slot = tonumber(args[1])
    if not slot then return Err("Usage: /aa clearslot <slot>") end
    A_Admin.ClearActionSlot(slot)
    Confirm("Action slot " .. Val(slot) .. " cleared")
end

handlers["clearactions"] = function(_args)
    A_Admin.ClearActionBars()
    Confirm("All action bar slots cleared")
end

-- Great Vault

handlers["vault"] = function(args)
    local sub = args[1]
    if not sub then return Err("Usage: /aa vault activity|rewards|clear") end
    sub = sub:lower()
    if sub == "activity" then
        local atype     = tonumber(args[2])
        local idx       = tonumber(args[3])
        local threshold = tonumber(args[4])
        local progress  = tonumber(args[5])
        local level     = tonumber(args[6])
        if not atype or not idx or not threshold or not progress or not level then
            return Err("Usage: /aa vault activity <type> <idx> <threshold> <progress> <level>")
        end
        A_Admin.SetVaultActivity(atype, idx, threshold, progress, level)
        Confirm("Vault slot type=" .. Val(atype) .. " idx=" .. Val(idx) .. ": " .. Val(progress) .. "/" .. Val(threshold) .. " lv" .. Val(level))
    elseif sub == "rewards" then
        local has = ParseBool(args[2])
        if has == nil then return Err("Usage: /aa vault rewards [on|off] [canClaim]") end
        local canClaim = args[3] and ParseBool(args[3])
        A_Admin.SetVaultRewards(has, canClaim)
        Confirm("Vault rewards: " .. Val(has and "on" or "off"))
    elseif sub == "clear" then
        A_Admin.ClearVault()
        Confirm("Vault cleared")
    else
        Err("Unknown vault subcommand: " .. sub)
    end
end

-- Events

handlers["event"] = function(args)
    local name = args[1]
    if not name then return Err("Usage: /aa event <name> [args...]") end
    local eventArgs = {}
    for i = 2, #args do
        eventArgs[#eventArgs + 1] = args[i]
    end
    A_Admin.FireEvent(name, unpack(eventArgs))
    if #eventArgs > 0 then
        Confirm("Event fired: " .. Val(name) .. " (" .. table.concat(eventArgs, ", ") .. ")")
    else
        Confirm("Event fired: " .. Val(name))
    end
end

-- Aliases

handlers["help"] = function(_args) ShowHelp() end

-- ---------------------------------------------------------------------------
-- Argument tokeniser (handles quoted strings)
-- ---------------------------------------------------------------------------

local function Tokenise(input)
    local tokens = {}
    local i = 1
    local len = #input
    while i <= len do
        -- skip whitespace
        while i <= len and input:sub(i, i):match("%s") do i = i + 1 end
        if i > len then break end

        if input:sub(i, i) == '"' then
            -- quoted token
            i = i + 1
            local start = i
            while i <= len and input:sub(i, i) ~= '"' do i = i + 1 end
            tokens[#tokens + 1] = input:sub(start, i - 1)
            i = i + 1
        else
            -- unquoted token
            local start = i
            while i <= len and not input:sub(i, i):match("%s") do i = i + 1 end
            tokens[#tokens + 1] = input:sub(start, i - 1)
        end
    end
    return tokens
end

-- ---------------------------------------------------------------------------
-- Slash command entry point
-- ---------------------------------------------------------------------------

local function OnSlashCommand(input)
    if not A_Admin then
        return Err("A_Admin API is not available in this environment")
    end

    local tokens = Tokenise(input or "")
    local cmd    = tokens[1]

    if not cmd or cmd == "" or cmd == "help" then
        ShowHelp()
        return
    end

    cmd = cmd:lower()
    table.remove(tokens, 1)

    local handler = handlers[cmd]
    if handler then
        handler(tokens)
    else
        Err("Unknown command: " .. cmd .. "  (type /aa help for list)")
    end
end

SLASH_ADMIN1 = "/admin"
SLASH_ADMIN2 = "/aa"
SlashCmdList["ADMIN"] = OnSlashCommand
