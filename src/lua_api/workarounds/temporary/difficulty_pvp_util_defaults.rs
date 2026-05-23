//! Temporary `DifficultyUtil` and `PVPUtil` defaults.
//!
//! These utility tables are normally supplied by Blizzard helper Lua. Keep the
//! startup-safe fallback values isolated until the corresponding helper loading
//! path is complete enough to provide them directly.

const DIFFICULTY_PVP_UTIL_DEFAULTS_LUA: &str = r#"
if type(DifficultyUtil) ~= "table" then
    DifficultyUtil = {}
end
if type(DifficultyUtil.ID) ~= "table" then
    DifficultyUtil.ID = {}
end
local difficultyIDs = {
    DungeonNormal = 1,
    DungeonHeroic = 2,
    Raid10Normal = 3,
    Raid25Normal = 4,
    Raid10Heroic = 5,
    Raid25Heroic = 6,
    RaidLFR = 7,
    DungeonChallenge = 8,
    Raid40 = 9,
    PrimaryRaidNormal = 14,
    PrimaryRaidHeroic = 15,
    PrimaryRaidMythic = 16,
    PrimaryRaidLFR = 17,
    DungeonMythic = 23,
    DungeonTimewalker = 24,
    RaidTimewalker = 33,
    RaidStory = 220,
}
for key, value in pairs(difficultyIDs) do
    if DifficultyUtil.ID[key] == nil then
        DifficultyUtil.ID[key] = value
    end
end
if rawget(DifficultyUtil, "GetDifficultyName") == nil then
    local difficultyNames = {
        [DifficultyUtil.ID.DungeonNormal] = PLAYER_DIFFICULTY1 or "Normal",
        [DifficultyUtil.ID.DungeonHeroic] = PLAYER_DIFFICULTY2 or "Heroic",
        [DifficultyUtil.ID.Raid10Normal] = PLAYER_DIFFICULTY1 or "Normal",
        [DifficultyUtil.ID.Raid25Normal] = PLAYER_DIFFICULTY1 or "Normal",
        [DifficultyUtil.ID.Raid10Heroic] = PLAYER_DIFFICULTY2 or "Heroic",
        [DifficultyUtil.ID.Raid25Heroic] = PLAYER_DIFFICULTY2 or "Heroic",
        [DifficultyUtil.ID.RaidLFR] = PLAYER_DIFFICULTY3 or "Raid Finder",
        [DifficultyUtil.ID.DungeonChallenge] = PLAYER_DIFFICULTY_MYTHIC_PLUS or "Mythic+",
        [DifficultyUtil.ID.Raid40] = LEGACY_RAID_DIFFICULTY or "Legacy Raid",
        [DifficultyUtil.ID.PrimaryRaidNormal] = PLAYER_DIFFICULTY1 or "Normal",
        [DifficultyUtil.ID.PrimaryRaidHeroic] = PLAYER_DIFFICULTY2 or "Heroic",
        [DifficultyUtil.ID.PrimaryRaidMythic] = PLAYER_DIFFICULTY6 or "Mythic",
        [DifficultyUtil.ID.PrimaryRaidLFR] = PLAYER_DIFFICULTY3 or "Raid Finder",
        [DifficultyUtil.ID.DungeonMythic] = PLAYER_DIFFICULTY6 or "Mythic",
        [DifficultyUtil.ID.DungeonTimewalker] = PLAYER_DIFFICULTY_TIMEWALKER or "Timewalking",
        [DifficultyUtil.ID.RaidTimewalker] = PLAYER_DIFFICULTY_TIMEWALKER or "Timewalking",
        [DifficultyUtil.ID.RaidStory] = PLAYER_DIFFICULTY_STORY_RAID or "Story",
    }

    function DifficultyUtil.GetDifficultyName(difficultyID)
        return difficultyNames[difficultyID]
    end
end
if rawget(DifficultyUtil, "IsPrimaryRaid") == nil then
    local primaryRaids = {
        [DifficultyUtil.ID.PrimaryRaidLFR] = true,
        [DifficultyUtil.ID.PrimaryRaidNormal] = true,
        [DifficultyUtil.ID.PrimaryRaidHeroic] = true,
        [DifficultyUtil.ID.PrimaryRaidMythic] = true,
    }

    function DifficultyUtil.IsPrimaryRaid(difficultyID)
        return primaryRaids[difficultyID] or false
    end
end
if rawget(DifficultyUtil, "GetMaxPlayers") == nil then
    local maxPlayers = {
        [DifficultyUtil.ID.DungeonNormal] = 5,
        [DifficultyUtil.ID.DungeonHeroic] = 5,
        [DifficultyUtil.ID.DungeonMythic] = 5,
        [DifficultyUtil.ID.DungeonChallenge] = 5,
        [DifficultyUtil.ID.DungeonTimewalker] = 5,
        [DifficultyUtil.ID.Raid10Normal] = 10,
        [DifficultyUtil.ID.Raid10Heroic] = 10,
        [DifficultyUtil.ID.Raid25Normal] = 25,
        [DifficultyUtil.ID.Raid25Heroic] = 25,
        [DifficultyUtil.ID.Raid40] = 40,
    }

    function DifficultyUtil.GetMaxPlayers(difficultyID)
        return maxPlayers[difficultyID]
    end
end

if type(PVPUtil) ~= "table" then
    PVPUtil = {}
end
if rawget(PVPUtil, "GetTierName") == nil then
    function PVPUtil.GetTierName(_tierEnum)
        return ""
    end
end
if rawget(PVPUtil, "GetTierDescription") == nil then
    function PVPUtil.GetTierDescription(_tierEnum)
        return ""
    end
end
if rawget(PVPUtil, "GetBracketName") == nil then
    function PVPUtil.GetBracketName(_bracketIndex)
        return ""
    end
end
if rawget(PVPUtil, "IsInActiveBattlefield") == nil then
    function PVPUtil.IsInActiveBattlefield()
        return false
    end
end
if rawget(PVPUtil, "GetCurrentSeasonNumber") == nil then
    function PVPUtil.GetCurrentSeasonNumber()
        return 0
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(DIFFICULTY_PVP_UTIL_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_difficulty_and_pvp_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("DifficultyUtil = nil; PVPUtil = nil")
            .expect("fixture should clear utility defaults");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("utility defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                if DifficultyUtil.ID.DungeonNormal ~= 1 then return "dungeon_id" end
                if DifficultyUtil.ID.PrimaryRaidMythic ~= 16 then return "raid_id" end
                if DifficultyUtil.GetDifficultyName(DifficultyUtil.ID.DungeonHeroic) ~= "Heroic" then return "name" end
                if DifficultyUtil.IsPrimaryRaid(DifficultyUtil.ID.PrimaryRaidHeroic) ~= true then return "primary_true" end
                if DifficultyUtil.IsPrimaryRaid(DifficultyUtil.ID.DungeonMythic) ~= false then return "primary_false" end
                if DifficultyUtil.GetMaxPlayers(DifficultyUtil.ID.DungeonNormal) ~= 5 then return "dungeon_players" end
                if DifficultyUtil.GetMaxPlayers(DifficultyUtil.ID.Raid25Heroic) ~= 25 then return "raid_players" end

                if PVPUtil.GetTierName(1) ~= "" then return "tier_name" end
                if PVPUtil.GetTierDescription(1) ~= "" then return "tier_description" end
                if PVPUtil.GetBracketName(1) ~= "" then return "bracket" end
                if PVPUtil.IsInActiveBattlefield() ~= false then return "battlefield" end
                if PVPUtil.GetCurrentSeasonNumber() ~= 0 then return "season" end
                return "ok"
                "#,
            )
            .expect("utility default shape probe should run");

        assert_eq!(result, "ok");
    }

    #[test]
    fn preserves_existing_utility_members() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            DifficultyUtil = {
                ID = { DungeonNormal = 99 },
                GetDifficultyName = function() return "custom" end,
            }
            PVPUtil = {
                GetTierName = function() return "rank" end,
            }
            "#,
        )
        .expect("fixture should install existing utility members");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("utility defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                if DifficultyUtil.ID.DungeonNormal ~= 99 then return "id" end
                if DifficultyUtil.GetDifficultyName(99) ~= "custom" then return "name" end
                if DifficultyUtil.IsPrimaryRaid(14) ~= true then return "difficulty_fill" end
                if PVPUtil.GetTierName(1) ~= "rank" then return "pvp_existing" end
                if PVPUtil.GetTierDescription(1) ~= "" then return "pvp_fill" end
                return "ok"
                "#,
            )
            .expect("utility preservation probe should run");

        assert_eq!(result, "ok");
    }
}
