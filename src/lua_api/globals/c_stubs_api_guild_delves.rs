//! Guild bank, pet battles, delves, and zone ability stubs.
//!
//! Split from c_stubs_api_social.rs. Contains:
//! - C_GuildBank, C_PetBattles
//! - C_DelvesUI, C_ZoneAbility, C_AutoComplete, C_PhotoSharing
//! - Misc global stubs (totems, parental controls, etc.)

use mlua::{Lua, Result, Value};

/// C_GuildBank, C_PetBattles stubs.
pub(super) fn register_guild_bank_pet_battles(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let guild_bank = lua.create_table()?;
    guild_bank.set(
        "IsGuildBankEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    guild_bank.set("GetCurrentBankTab", lua.create_function(|_, ()| Ok(1i32))?)?;
    guild_bank.set("FetchNumTabs", lua.create_function(|_, ()| Ok(0i32))?)?;
    g.set("C_GuildBank", guild_bank)?;

    lua.load(PET_BATTLES_LUA).exec()?;
    g.set(
        "C_PetBattles",
        lua.globals().get::<mlua::Table>("C_PetBattles")?,
    )?;
    Ok(())
}

const PET_BATTLES_LUA: &str = r#"
    C_PetBattles = C_PetBattles or {}
    local api = C_PetBattles

    local ACTION = Enum and Enum.BattlePetAction or {
        None = 0,
        Ability = 1,
        SwitchPet = 2,
        Trap = 3,
        Skip = 4,
    }

    local OWNER = Enum and Enum.BattlePetOwner or {
        Ally = 0,
        Enemy = 1,
        Weather = 2,
    }

    local QUEUE = Enum and Enum.PetBattleQueueStatus or {
        None = 0,
        Queued = 1,
        MatchAccepted = 8,
        MatchDeclined = 9,
        Matchmaking = 15,
    }

    local STATE = Enum and Enum.PetbattleState or {
        Created = 0,
        WaitingPreBattle = 1,
        RoundInProgress = 2,
        WaitingForFrontPets = 3,
        Finished = 6,
    }

    api._state = api._state or {
        inBattle = true,
        wildBattle = true,
        shouldShowPetSelect = false,
        canActivePetSwapOut = true,
        waitingOnOpponent = false,
        battleState = STATE.WaitingPreBattle,
        forfeitPenalty = 10,
        selectedActionType = nil,
        selectedActionIndex = nil,
        trapAbilityID = 427,
        trapAvailable = true,
        trapError = nil,
        skipAvailable = true,
        pvpQueue = {
            status = nil,
            estimatedTime = 18,
            queuedTime = 4,
            canAccept = false,
        },
        pvpDuel = {
            pending = false,
            exactMatch = false,
            challengedUnit = nil,
        },
        pendingReportBattlePetTarget = nil,
        pendingReportTargetUnit = nil,
        turnTimeInfo = {
            timeRemaining = 18,
            turnTime = 30,
        },
        attackModifiers = {
            ["7:9"] = 1.5,
            ["9:7"] = 0.66,
        },
        effectNames = { "damage", "heal", "variance" },
        namedStates = {
            STATE_Stat_Power = 18,
            STATE_Stat_Speed = 19,
            STATE_Stat_Accuracy = 20,
        },
        abilities = {
            [427] = {
                id = 427,
                name = "Trap",
                icon = "Interface\\Icons\\Ability_Hunter_BeastTaming",
                maxCooldown = 0,
                description = "Attempt to capture a weakened wild pet.",
                numTurns = 1,
                petType = 8,
                noStrongWeakHints = true,
                effects = {
                    [1] = {
                        [1] = { damage = 0, heal = 0, variance = 0 },
                    },
                },
                procTurnIndex = {},
                stateModifications = {},
            },
            [1001] = {
                id = 1001,
                name = "Arcane Bite",
                icon = "Interface\\Icons\\Spell_Arcane_Arcane02",
                maxCooldown = 0,
                description = "Deal Arcane damage to the enemy team.",
                numTurns = 1,
                petType = 7,
                noStrongWeakHints = false,
                effects = {
                    [1] = {
                        [1] = { damage = 280, heal = 0, variance = 10 },
                    },
                },
                procTurnIndex = { ApplyOnHit = 1 },
                stateModifications = { [18] = 12 },
            },
            [1002] = {
                id = 1002,
                name = "Mystic Ward",
                icon = "Interface\\Icons\\Spell_Holy_PowerWordShield",
                maxCooldown = 2,
                description = "Reduce damage taken for 2 rounds.",
                numTurns = 2,
                petType = 6,
                noStrongWeakHints = true,
                effects = {
                    [1] = {
                        [1] = { damage = 0, heal = 75, variance = 0 },
                    },
                },
                procTurnIndex = { OnRoundStart = 1 },
                stateModifications = { [19] = 8 },
            },
            [1003] = {
                id = 1003,
                name = "Tail Sweep",
                icon = "Interface\\Icons\\Ability_Druid_TwilightsWrath",
                maxCooldown = 1,
                description = "Strike twice if you act first.",
                numTurns = 1,
                petType = 2,
                noStrongWeakHints = false,
                effects = {
                    [1] = {
                        [1] = { damage = 190, heal = 0, variance = 8 },
                    },
                },
                procTurnIndex = { OnAttack = 1 },
                stateModifications = {},
            },
            [2001] = {
                id = 2001,
                name = "Sandstorm",
                icon = "Interface\\Icons\\Ability_Rogue_FeignDeath",
                maxCooldown = 3,
                description = "A weather effect that alters the battle.",
                numTurns = 5,
                petType = 4,
                noStrongWeakHints = true,
                effects = {
                    [1] = {
                        [1] = { damage = 0, heal = 0, variance = 0 },
                    },
                },
                procTurnIndex = { OnRoundEnd = 1 },
                stateModifications = {},
            },
        },
        owners = {
            [OWNER.Ally] = {
                isPlayerNPC = false,
                activePet = 1,
                pets = {
                    [1] = {
                        customName = "Brightpaw",
                        speciesName = "Brightpaw Cub",
                        icon = "Interface\\Icons\\INV_Pet_Cat",
                        displayID = 101,
                        level = 25,
                        health = 1420,
                        maxHealth = 1540,
                        power = 325,
                        speed = 289,
                        xp = 480,
                        maxXP = 1000,
                        petType = 7,
                        speciesID = 39,
                        breedQuality = Enum and Enum.BattlePetBreedQuality and Enum.BattlePetBreedQuality.Rare or 3,
                        abilities = { 1001, 1002, 1003 },
                        abilityStates = {
                            [1] = { isUsable = true, cooldown = 0, lockdown = 0 },
                            [2] = { isUsable = true, cooldown = 1, lockdown = 0 },
                            [3] = { isUsable = true, cooldown = 0, lockdown = 0 },
                        },
                        auras = {
                            {
                                auraID = 1002,
                                instanceID = 9001,
                                turnsRemaining = 2,
                                isBuff = true,
                                casterOwner = OWNER.Ally,
                                casterIndex = 1,
                            },
                        },
                        states = {
                            [18] = 325,
                            [19] = 289,
                        },
                    },
                    [2] = {
                        customName = "Shellhide",
                        speciesName = "Shellhide Tortoise",
                        icon = "Interface\\Icons\\INV_Pet_Turtle",
                        displayID = 102,
                        level = 24,
                        health = 1210,
                        maxHealth = 1510,
                        power = 280,
                        speed = 180,
                        xp = 320,
                        maxXP = 1000,
                        petType = 8,
                        speciesID = 67,
                        breedQuality = Enum and Enum.BattlePetBreedQuality and Enum.BattlePetBreedQuality.Uncommon or 2,
                        abilities = { 1002, 1001, 1003 },
                        abilityStates = {},
                        auras = {},
                        states = {
                            [18] = 280,
                            [19] = 180,
                        },
                    },
                    [3] = {
                        customName = "Cliffhopper",
                        speciesName = "Cliffhopper",
                        icon = "Interface\\Icons\\INV_Pet_Bird",
                        displayID = 103,
                        level = 23,
                        health = 0,
                        maxHealth = 1390,
                        power = 301,
                        speed = 312,
                        xp = 0,
                        maxXP = 1000,
                        petType = 2,
                        speciesID = 73,
                        breedQuality = Enum and Enum.BattlePetBreedQuality and Enum.BattlePetBreedQuality.Common or 1,
                        abilities = { 1003, 1001, 1002 },
                        abilityStates = {},
                        auras = {},
                        states = {
                            [18] = 301,
                            [19] = 312,
                        },
                    },
                },
            },
            [OWNER.Enemy] = {
                isPlayerNPC = true,
                activePet = 1,
                pets = {
                    [1] = {
                        customName = "Duststinger",
                        speciesName = "Duststinger Wasp",
                        icon = "Interface\\Icons\\Ability_Hunter_Pet_Wasp",
                        displayID = 201,
                        level = 25,
                        health = 980,
                        maxHealth = 1380,
                        power = 295,
                        speed = 250,
                        xp = 0,
                        maxXP = 0,
                        petType = 9,
                        speciesID = 88,
                        breedQuality = Enum and Enum.BattlePetBreedQuality and Enum.BattlePetBreedQuality.Rare or 3,
                        abilities = { 1001, 1003, 1002 },
                        abilityStates = {},
                        auras = {},
                        states = {
                            [18] = 295,
                            [19] = 250,
                        },
                    },
                    [2] = {
                        customName = "Ashtooth",
                        speciesName = "Ashtooth",
                        icon = "Interface\\Icons\\INV_Pet_Wolf",
                        displayID = 202,
                        level = 24,
                        health = 760,
                        maxHealth = 1440,
                        power = 310,
                        speed = 220,
                        xp = 0,
                        maxXP = 0,
                        petType = 3,
                        speciesID = 91,
                        breedQuality = Enum and Enum.BattlePetBreedQuality and Enum.BattlePetBreedQuality.Uncommon or 2,
                        abilities = { 1003, 1001, 1002 },
                        abilityStates = {},
                        auras = {},
                        states = {
                            [18] = 310,
                            [19] = 220,
                        },
                    },
                },
            },
            [OWNER.Weather] = {
                isPlayerNPC = true,
                activePet = 1,
                pets = {
                    [0] = {
                        customName = "Sandstorm",
                        speciesName = "Sandstorm",
                        icon = "Interface\\Icons\\Ability_Hunter_Pet_Scorpid",
                        displayID = 301,
                        level = 1,
                        health = 1,
                        maxHealth = 1,
                        power = 0,
                        speed = 0,
                        xp = 0,
                        maxXP = 0,
                        petType = 4,
                        speciesID = 0,
                        breedQuality = Enum and Enum.BattlePetBreedQuality and Enum.BattlePetBreedQuality.Common or 1,
                        abilities = { 2001 },
                        abilityStates = {},
                        auras = {
                            {
                                auraID = 2001,
                                instanceID = 9100,
                                turnsRemaining = 3,
                                isBuff = true,
                                casterOwner = OWNER.Weather,
                                casterIndex = 0,
                            },
                        },
                        states = {},
                    },
                },
            },
        },
    }

    local unpack_fn = unpack or table.unpack

    local function normalize_number(value)
        local numericValue = tonumber(value)
        return numericValue
    end

    local function owner_state(petOwner)
        return api._state.owners[normalize_number(petOwner) or OWNER.Ally]
    end

    local function pet_state(petOwner, slot)
        local owner = owner_state(petOwner)
        if owner == nil then
            return nil
        end
        return owner.pets[normalize_number(slot) or 0]
    end

    local function ability_entry_by_id(abilityID)
        return api._state.abilities[normalize_number(abilityID) or -1]
    end

    local function ability_entry_for_pet(petOwner, slot, abilityIndex)
        local pet = pet_state(petOwner, slot)
        if pet == nil then
            return nil
        end
        local abilityID = pet.abilities[normalize_number(abilityIndex) or -1]
        if abilityID == nil then
            return nil
        end
        return ability_entry_by_id(abilityID)
    end

    function api.IsInBattle()
        return api._state.inBattle == true
    end

    function api.IsWildBattle()
        return api._state.wildBattle == true
    end

    function api.IsPlayerNPC(petOwner)
        local owner = owner_state(petOwner)
        return owner ~= nil and owner.isPlayerNPC == true or false
    end

    function api.GetAllEffectNames()
        return unpack_fn(api._state.effectNames)
    end

    function api.GetAllStates(parserEnv)
        local namedStates = api._state.namedStates
        if type(parserEnv) == "table" then
            for key, value in pairs(namedStates) do
                parserEnv[key] = value
            end
        end
        return namedStates
    end

    function api.GetBattleState()
        return api._state.battleState
    end

    function api.GetPVPMatchmakingInfo()
        local queue = api._state.pvpQueue
        if queue.status == nil then
            return nil
        end
        return queue.status, queue.estimatedTime, queue.queuedTime
    end

    function api.AcceptPVPDuel()
        api._state.pvpDuel.pending = false
        api._state.pvpDuel.accepted = true
    end

    function api.CancelPVPDuel()
        api._state.pvpDuel.pending = false
        api._state.pvpDuel.accepted = false
    end

    function api.StartPVPDuel(unit, exactMatch)
        api._state.pvpDuel.pending = true
        api._state.pvpDuel.challengedUnit = unit
        api._state.pvpDuel.exactMatch = exactMatch == true
    end

    function api.StartPVPMatchmaking()
        api._state.pvpQueue.status = QUEUE.Matchmaking
        api._state.pvpQueue.canAccept = true
    end

    function api.StopPVPMatchmaking()
        api._state.pvpQueue.status = nil
        api._state.pvpQueue.canAccept = false
    end

    function api.AcceptQueuedPVPMatch()
        api._state.pvpQueue.status = QUEUE.MatchAccepted
        api._state.pvpQueue.canAccept = false
    end

    function api.DeclineQueuedPVPMatch()
        api._state.pvpQueue.status = QUEUE.MatchDeclined
        api._state.pvpQueue.canAccept = false
    end

    function api.CanAcceptQueuedPVPMatch()
        return api._state.pvpQueue.canAccept == true
    end

    function api.CanActivePetSwapOut()
        return api._state.canActivePetSwapOut == true
    end

    function api.CanPetSwapIn(slot)
        local pet = pet_state(OWNER.Ally, slot)
        return pet ~= nil and pet.health > 0 and api.GetActivePet(OWNER.Ally) ~= (normalize_number(slot) or -1)
    end

    function api.ChangePet(slot)
        if api.CanPetSwapIn(slot) then
            api._state.selectedActionType = ACTION.SwitchPet
            api._state.selectedActionIndex = normalize_number(slot)
        end
    end

    function api.ForfeitGame()
        api._state.inBattle = false
        api._state.battleState = STATE.Finished
    end

    function api.SkipTurn()
        api._state.selectedActionType = ACTION.Skip
        api._state.selectedActionIndex = nil
    end

    function api.UseAbility(abilityIndex)
        api._state.selectedActionType = ACTION.Ability
        api._state.selectedActionIndex = normalize_number(abilityIndex)
    end

    function api.UseTrap()
        api._state.selectedActionType = ACTION.Trap
        api._state.selectedActionIndex = nil
    end

    function api.GetAbilityEffectInfo(abilityID, turnIndex, effectIndex, effectName)
        local ability = ability_entry_by_id(abilityID)
        if ability == nil then
            return nil
        end
        local turnEffects = ability.effects[normalize_number(turnIndex) or -1]
        local effect = turnEffects and turnEffects[normalize_number(effectIndex) or -1]
        return effect and effect[effectName] or nil
    end

    function api.GetAbilityInfo(petOwner, slot, abilityIndex)
        local ability = ability_entry_for_pet(petOwner, slot, abilityIndex)
        if ability == nil then
            return nil
        end
        return ability.id, ability.name, ability.icon, ability.maxCooldown, ability.description, ability.numTurns, ability.petType, ability.noStrongWeakHints
    end

    function api.GetAbilityInfoByID(abilityID)
        local ability = ability_entry_by_id(abilityID)
        if ability == nil then
            return nil
        end
        return ability.id, ability.name, ability.icon, ability.maxCooldown, ability.description, ability.numTurns, ability.petType, ability.noStrongWeakHints
    end

    function api.GetAbilityProcTurnIndex(abilityID, procType)
        local ability = ability_entry_by_id(abilityID)
        if ability == nil then
            return nil
        end
        return ability.procTurnIndex[procType] or 1
    end

    function api.GetAbilityState(petOwner, slot, abilityIndex)
        local pet = pet_state(petOwner, slot)
        if pet == nil then
            return false, 0, 0
        end
        local state = pet.abilityStates[normalize_number(abilityIndex) or -1] or {}
        return state.isUsable ~= false, state.cooldown or 0, state.lockdown or 0
    end

    function api.GetAbilityStateModification(abilityID, stateID)
        local ability = ability_entry_by_id(abilityID)
        if ability == nil then
            return 0
        end
        return ability.stateModifications[normalize_number(stateID) or -1] or 0
    end

    function api.GetActivePet(petOwner)
        local owner = owner_state(petOwner)
        return owner and owner.activePet or 1
    end

    function api.GetAttackModifier(attackType, defenderType)
        local key = string.format("%d:%d", normalize_number(attackType) or 0, normalize_number(defenderType) or 0)
        return api._state.attackModifiers[key] or 1
    end

    function api.GetAuraInfo(petOwner, slot, auraIndex)
        local pet = pet_state(petOwner, slot)
        if pet == nil then
            return nil
        end
        local aura = pet.auras[normalize_number(auraIndex) or -1]
        if aura == nil then
            return nil
        end
        return aura.auraID, aura.instanceID, aura.turnsRemaining, aura.isBuff, aura.casterOwner, aura.casterIndex
    end

    function api.GetBreedQuality(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        return pet and pet.breedQuality or 1
    end

    function api.GetDisplayID(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        return pet and pet.displayID or nil
    end

    function api.GetForfeitPenalty()
        return api._state.forfeitPenalty
    end

    function api.GetHealth(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        return pet and pet.health or 0
    end

    function api.GetIcon(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        return pet and pet.icon or nil
    end

    function api.GetLevel(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        return pet and pet.level or 0
    end

    function api.GetMaxHealth(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        return pet and pet.maxHealth or 0
    end

    function api.GetName(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        if pet == nil then
            return nil
        end
        return pet.customName, pet.speciesName
    end

    function api.GetNumAuras(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        return pet and #pet.auras or 0
    end

    function api.GetNumPets(petOwner)
        local owner = owner_state(petOwner)
        if owner == nil then
            return 0
        end

        local count = 0
        for _ in pairs(owner.pets) do
            count = count + 1
        end
        return count
    end

    function api.GetPetSpeciesID(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        return pet and pet.speciesID or 0
    end

    function api.GetPetType(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        return pet and pet.petType or 0
    end

    function api.GetPlayerTrapAbility()
        return api._state.trapAbilityID
    end

    function api.GetPower(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        return pet and pet.power or 0
    end

    function api.GetSelectedAction()
        return api._state.selectedActionType, api._state.selectedActionIndex
    end

    function api.GetSpeed(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        return pet and pet.speed or 0
    end

    function api.GetStateValue(petOwner, slot, stateID)
        local pet = pet_state(petOwner, slot)
        if pet == nil then
            return 0
        end
        return pet.states[normalize_number(stateID) or -1] or 0
    end

    function api.GetTurnTimeInfo()
        return api._state.turnTimeInfo.timeRemaining, api._state.turnTimeInfo.turnTime
    end

    function api.GetXP(petOwner, slot)
        local pet = pet_state(petOwner, slot)
        if pet == nil then
            return 0, 0
        end
        return pet.xp, pet.maxXP
    end

    function api.IsSkipAvailable()
        return api._state.skipAvailable == true
    end

    function api.IsTrapAvailable()
        return api._state.trapAvailable == true, api._state.trapError
    end

    function api.IsWaitingOnOpponent()
        return api._state.waitingOnOpponent == true
    end

    function api.SetPendingReportBattlePetTarget(slot)
        api._state.pendingReportBattlePetTarget = normalize_number(slot)
    end

    function api.SetPendingReportTargetFromUnit(unit)
        api._state.pendingReportTargetUnit = unit
    end

    function api.ShouldShowPetSelect()
        return api._state.shouldShowPetSelect == true
    end
"#;

/// C_DelvesUI namespace - Delves companion data.
pub(super) fn register_c_delves_ui(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    let delve_entrance_tiers = seeded_delve_entrance_tiers(lua)?;
    let active_delve_tier = lua.create_table()?;
    active_delve_tier.set("tier", 4)?;
    active_delve_tier.set("tierDescription", "Tier 4")?;
    active_delve_tier.set("unlocked", true)?;
    active_delve_tier.set("modifierUIWidgetSetID", 4404)?;
    active_delve_tier.set("suggestedILvl", 603)?;
    active_delve_tier.set("lockedReason", Value::Nil)?;
    t.set("__activeDelveTier", active_delve_tier)?;
    t.set("__delveEntranceBackgroundWidgetSetID", 5501)?;
    t.set(
        "__delveEntranceDescriptionString",
        "The Fungal Folly winds deeper with every tier.",
    )?;
    t.set("__delveEntranceHeaderString", "Fungal Folly")?;
    t.set("__delveEntranceMapID", 2339)?;
    t.set("__delveEntranceTiers", delve_entrance_tiers)?;
    t.set("__selectedDelveEntranceTier", 4)?;
    t.set("__tieredEntranceOptionalAffixTraitTreeID", 77001)?;
    let delve_state = t.clone();
    t.set(
        "GetTraitTreeForCompanion",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetRoleNodeForCompanion",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetRoleSubtreeForCompanion",
        lua.create_function(|_, _role_type: Value| Ok(0i32))?,
    )?;
    t.set(
        "GetCreatureDisplayInfoForCompanion",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetCurioNodeForCompanion",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetCurrentDelvesSeasonNumber",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    t.set(
        "GetDelvesMinRequiredLevel",
        lua.create_function(|_, ()| Ok(80i32))?,
    )?;
    t.set(
        "GetActiveDelveTier",
        lua.create_function(move |_, ()| delve_state.get::<mlua::Table>("__activeDelveTier"))?,
    )?;
    let delve_state = t.clone();
    t.set(
        "GetDelveEntranceBackgroundWidgetSetID",
        lua.create_function(move |_, ()| {
            delve_state.get::<i32>("__delveEntranceBackgroundWidgetSetID")
        })?,
    )?;
    let delve_state = t.clone();
    t.set(
        "GetDelveEntranceDescriptionString",
        lua.create_function(move |_, ()| {
            delve_state.get::<String>("__delveEntranceDescriptionString")
        })?,
    )?;
    let delve_state = t.clone();
    t.set(
        "GetDelveEntranceHeaderString",
        lua.create_function(move |_, ()| delve_state.get::<String>("__delveEntranceHeaderString"))?,
    )?;
    let delve_state = t.clone();
    t.set(
        "GetDelveEntranceMapID",
        lua.create_function(move |_, ()| delve_state.get::<i32>("__delveEntranceMapID"))?,
    )?;
    let delve_state = t.clone();
    t.set(
        "GetDelveEntranceTiers",
        lua.create_function(move |_, ()| delve_state.get::<mlua::Table>("__delveEntranceTiers"))?,
    )?;
    t.set(
        "GetFactionForCompanion",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set("HasActiveDelve", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetUnseenCuriosBySlotType",
        lua.create_function(|lua, _slot_type: Value| lua.create_table())?,
    )?;
    t.set(
        "GetDelvesFactionForSeason",
        lua.create_function(|_, _season: Value| Ok(Value::Nil))?,
    )?;
    let delve_state = t.clone();
    t.set(
        "GetTieredEntranceOptionalAffixTraitTreeID",
        lua.create_function(move |_, ()| {
            delve_state.get::<i32>("__tieredEntranceOptionalAffixTraitTreeID")
        })?,
    )?;
    let delve_state = t.clone();
    t.set(
        "IsDelveEntranceTierEnabled",
        lua.create_function(move |lua, tier: i32| {
            let tiers = delve_state.get::<mlua::Table>("__delveEntranceTiers")?;
            for pair in tiers.sequence_values::<mlua::Table>() {
                let info = pair?;
                if info.get::<i32>("tier")? == tier {
                    let unlocked = info.get::<bool>("unlocked")?;
                    let locked_reason = info.get::<Value>("lockedReason")?;
                    return Ok(mlua::MultiValue::from_vec(vec![
                        Value::Boolean(unlocked),
                        if unlocked { Value::Nil } else { locked_reason },
                    ]));
                }
            }

            Ok(mlua::MultiValue::from_vec(vec![
                Value::Boolean(false),
                Value::String(lua.create_string("Unknown tier")?),
            ]))
        })?,
    )?;
    t.set(
        "RequestPartyEligibilityForDelveTiers",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    let delve_state = t.clone();
    t.set(
        "SelectDelveEntranceTier",
        lua.create_function(move |_, tier: i32| {
            delve_state.set("__selectedDelveEntranceTier", tier)?;
            let tiers = delve_state.get::<mlua::Table>("__delveEntranceTiers")?;
            for pair in tiers.sequence_values::<mlua::Table>() {
                let info = pair?;
                if info.get::<i32>("tier")? == tier {
                    delve_state.set("__activeDelveTier", info)?;
                    break;
                }
            }
            Ok(())
        })?,
    )?;
    t.set(
        "SaveSeenCuriosBySlotType",
        lua.create_function(|_, (_slot_type, _table): (Value, Value)| Ok(()))?,
    )?;
    lua.globals().set("C_DelvesUI", t)?;
    Ok(())
}

fn seeded_delve_entrance_tiers(lua: &Lua) -> Result<mlua::Table> {
    let tiers = lua.create_table()?;
    for (index, (tier, unlocked, widget_set_id, suggested_ilvl, locked_reason)) in [
        (1, true, 4401, 571, None),
        (2, true, 4402, 584, None),
        (3, true, 4403, 597, None),
        (4, true, 4404, 603, None),
        (
            5,
            false,
            4405,
            610,
            Some("Complete Tier 4 to unlock this delve tier."),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let info = lua.create_table()?;
        info.set("tier", tier)?;
        info.set("tierDescription", format!("Tier {tier}"))?;
        info.set("unlocked", unlocked)?;
        info.set("modifierUIWidgetSetID", widget_set_id)?;
        info.set("suggestedILvl", suggested_ilvl)?;
        match locked_reason {
            Some(reason) => info.set("lockedReason", reason)?,
            None => info.set("lockedReason", Value::Nil)?,
        }
        tiers.set(index + 1, info)?;
    }
    Ok(tiers)
}

/// C_ZoneAbility namespace - zone ability data.
pub(super) fn register_c_zone_ability(lua: &Lua) -> Result<()> {
    lua.load(ZONE_ABILITY_LUA).exec()?;
    let zone_ability = lua.globals().get::<mlua::Table>("C_ZoneAbility")?;
    lua.globals().set("C_ZoneAbility", zone_ability)?;
    Ok(())
}

const ZONE_ABILITY_LUA: &str = r#"
    C_ZoneAbility = C_ZoneAbility or {}
    local api = C_ZoneAbility

    api._state = api._state or {
        activeAbilities = {
            {
                zoneAbilityID = 1,
                uiPriority = 1,
                spellID = 372610,
                textureKit = nil,
                tutorialText = "Skyward Ascent",
            },
        },
        iconsBySpellID = {},
        defaultIcon = "Interface\\Icons\\INV_Misc_QuestionMark",
    }

    local function copyAbility(ability)
        if type(ability) ~= "table" then
            return nil
        end

        local copy = {}
        for key, value in pairs(ability) do
            copy[key] = value
        end
        return copy
    end

    local function resolveSpellTexture(spellID)
        if type(C_Spell) ~= "table" or type(C_Spell.GetSpellTexture) ~= "function" then
            return nil
        end

        local ok, texture = pcall(C_Spell.GetSpellTexture, spellID)
        if not ok or texture == nil or texture == "" then
            return nil
        end
        return texture
    end

    api.GetActiveAbilities = api.GetActiveAbilities or function()
        local abilities = api._state.activeAbilities or {}
        local copy = {}
        for index, ability in ipairs(abilities) do
            copy[index] = copyAbility(ability)
        end
        return copy
    end

    api.GetZoneAbilityIcon = api.GetZoneAbilityIcon or function(spellID)
        local iconsBySpellID = api._state.iconsBySpellID or {}
        local seededIcon = iconsBySpellID[spellID]
        if seededIcon ~= nil and seededIcon ~= "" then
            return seededIcon
        end

        local spellTexture = resolveSpellTexture(spellID)
        if spellTexture ~= nil then
            return spellTexture
        end

        return api._state.defaultIcon
    end
"#;

/// C_AutoComplete namespace - player name autocomplete results.
pub(super) fn register_c_auto_complete(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetAutoCompleteResults",
        lua.create_function(|lua, _args: mlua::MultiValue| lua.create_table())?,
    )?;
    t.set(
        "GetAutoCompletePresenceID",
        lua.create_function(|_, _name: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetAutoCompleteRealms",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "IsRecognizedName",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?,
    )?;
    g.set("C_AutoComplete", t)
}

/// C_PhotoSharing namespace - social photo sharing feature.
pub(super) fn register_c_photo_sharing(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsAuthorized", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "BeginAuthorizationFlow",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    t.set(
        "CompleteAuthorizationFlow",
        lua.create_function(|_, _url: Value| Ok(()))?,
    )?;
    t.set("ClearAuthorization", lua.create_function(|_, ()| Ok(()))?)?;
    t.set(
        "GetPhotoSharingAuthURL",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set("GetCropRatio", lua.create_function(|_, ()| Ok(1.0f64))?)?;
    t.set(
        "SetScreenshotPreviewTexture",
        lua.create_function(|_, _frame: Value| Ok(()))?,
    )?;
    t.set(
        "UploadPhotoToService",
        lua.create_function(|_, (_title, _desc): (Value, Value)| Ok(()))?,
    )?;
    t.set("GetStatus", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    g.set("C_PhotoSharing", t)
}

/// AutoComplete-related global function stubs.
pub(super) fn register_auto_complete_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "AutoCompleteEditBox_SetCustomAutoCompleteFunction",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    g.set(
        "AutoCompleteEditBox_SetAutoCompleteSource",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    Ok(())
}

/// Misc global stubs: guild info, totems, parental controls, item text.
pub(super) fn register_misc_global_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("CanEditGuildInfo", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("IsCpuBound", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "GetTotemCannotDismiss",
        lua.create_function(|_, _slot: i32| Ok(false))?,
    )?;
    g.set(
        "GetTotemTimeLeft",
        lua.create_function(|_, _slot: i32| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetSecondsUntilParentalControlsKick",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "ItemTextHasNextPage",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    Ok(())
}
