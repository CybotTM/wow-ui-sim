use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

#[test]
fn get_num_random_dungeons() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local n = GetNumRandomDungeons()
            if n < 1 then return "count=" .. n end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetNumRandomDungeons: {result}");
}

#[test]
fn get_lfg_random_dungeon_info() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local id, name = GetLFGRandomDungeonInfo(1)
            if type(id) ~= "number" then return "id_type=" .. type(id) end
            if type(name) ~= "string" then return "name_type=" .. type(name) end
            if name == "" then return "empty_name" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGRandomDungeonInfo(1): {result}");
}

#[test]
fn raid_finder_dungeon_globals_are_inert_when_no_raids_are_seeded() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(GetNumRFDungeons) ~= "function" then
                return "count_type=" .. type(GetNumRFDungeons)
            end
            if type(GetRFDungeonInfo) ~= "function" then
                return "info_type=" .. type(GetRFDungeonInfo)
            end
            local count = GetNumRFDungeons()
            if count ~= 0 then return "count=" .. tostring(count) end
            if GetRFDungeonInfo(1) ~= nil then return "unexpected_info" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Raid Finder dungeon globals should be inert: {result}"
    );
}

#[test]
fn scenario_finder_globals_are_inert_without_scenario_data() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local order = GetScenariosChoiceOrder()
            if type(order) ~= "table" then return "order_type=" .. type(order) end
            if #order ~= 0 then return "order_count=" .. #order end
            if GetNumRandomScenarios() ~= 0 then return "count" end
            if GetRandomScenarioInfo(1) ~= nil then return "info" end
            if GetRandomScenarioBestChoice() ~= nil then return "best_choice" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Scenario Finder globals should be inert: {result}"
    );
}

#[test]
fn get_random_dungeon_best_choice() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local id = GetRandomDungeonBestChoice()
            if type(id) ~= "number" then return "type=" .. type(id) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetRandomDungeonBestChoice: {result}");
}

#[test]
fn dungeon_appears_in_random_lfd_reports_lfd_category_for_seeded_dungeons() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(DungeonAppearsInRandomLFD) ~= "function" then
                return "type=" .. type(DungeonAppearsInRandomLFD)
            end
            if DungeonAppearsInRandomLFD(1201) ~= LE_LFG_CATEGORY_LFD then
                return "known=" .. tostring(DungeonAppearsInRandomLFD(1201))
            end
            if DungeonAppearsInRandomLFD(-1) ~= nil then
                return "header=" .. tostring(DungeonAppearsInRandomLFD(-1))
            end
            if DungeonAppearsInRandomLFD(1271) ~= nil then
                return "journal_id=" .. tostring(DungeonAppearsInRandomLFD(1271))
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "DungeonAppearsInRandomLFD: {result}");
}

#[test]
fn unit_has_lfg_random_cooldown_is_registered_and_defaults_false() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(UnitHasLFGRandomCooldown) ~= "function" then
                return "type=" .. type(UnitHasLFGRandomCooldown)
            end
            if UnitHasLFGRandomCooldown("player") ~= false then
                return "player=" .. tostring(UnitHasLFGRandomCooldown("player"))
            end
            if UnitHasLFGRandomCooldown("party1") ~= false then
                return "party1=" .. tostring(UnitHasLFGRandomCooldown("party1"))
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "UnitHasLFGRandomCooldown: {result}");
}

#[test]
fn c_lfg_info_is_follower_dungeon() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- 1202 = City of Threads (is_follower_dungeon=true)
            if not C_LFGInfo.IsLFGFollowerDungeon(1202) then return "1202_not_follower" end
            -- 1203 = Mists of Tirna Scithe (is_follower_dungeon=false)
            if C_LFGInfo.IsLFGFollowerDungeon(1203) then return "1203_is_follower" end
            -- unknown id
            if C_LFGInfo.IsLFGFollowerDungeon(9999) then return "9999_is_follower" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "IsLFGFollowerDungeon: {result}");
}

#[test]
fn get_lfd_lock_player_count_returns_zero() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local n = GetLFDLockPlayerCount()
            if type(n) ~= "number" then return "type=" .. type(n) end
            if n ~= 0 then return "count=" .. n end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFDLockPlayerCount: {result}");
}

#[test]
fn get_lfd_lock_info_returns_six_nils() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local a, b, c, d, e, f = GetLFDLockInfo(1203, 1)
            if a ~= nil or b ~= nil or c ~= nil or d ~= nil or e ~= nil or f ~= nil then
                return "non_nil"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFDLockInfo: {result}");
}

#[test]
fn get_lfd_role_lock_info_returns_empty_table() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local t = GetLFDRoleLockInfo(1203, 1)
            if type(t) ~= "table" then return "type=" .. type(t) end
            if #t ~= 0 then return "count=" .. #t end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFDRoleLockInfo: {result}");
}

#[test]
fn lfg_construct_declined_message_does_not_error() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- Mirrors LFGConstructDeclinedMessage's loop; the call must not
            -- raise even when nothing is locked.
            local ok = pcall(function()
                for i = 1, GetLFDLockPlayerCount() do
                    GetLFDLockInfo(1203, i)
                end
            end)
            return ok and "ok" or "error"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn is_lfg_dungeon_joinable_in_range() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- 1205 = Grim Batol, min_level=15, max_level=80; player default level 70
            local all, player, hide, size = IsLFGDungeonJoinable(1205)
            if all ~= true then return "all=" .. tostring(all) end
            if player ~= true then return "player=" .. tostring(player) end
            if hide ~= false then return "hide=" .. tostring(hide) end
            if size ~= 5 then return "size=" .. tostring(size) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "IsLFGDungeonJoinable(1205): {result}");
}

#[test]
fn is_lfg_dungeon_joinable_out_of_range() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- -1 = Random Heroic header, min/max=80; player default level 70
            local all, player, hide, size = IsLFGDungeonJoinable(-1)
            if all ~= true then return "all=" .. tostring(all) end
            if player ~= false then return "player=" .. tostring(player) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "IsLFGDungeonJoinable(-1): {result}");
}

#[test]
fn is_lfg_dungeon_joinable_unknown_id() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local all, player, hide, size = IsLFGDungeonJoinable(9999)
            if all ~= false then return "all=" .. tostring(all) end
            if player ~= false then return "player=" .. tostring(player) end
            if hide ~= true then return "hide=" .. tostring(hide) end
            if size ~= 0 then return "size=" .. tostring(size) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "IsLFGDungeonJoinable(9999): {result}");
}

#[test]
fn get_lfg_dungeon_num_encounters() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local n, c = GetLFGDungeonNumEncounters(1203)
            if type(n) ~= "number" then return "num_type=" .. type(n) end
            if n < 1 then return "count=" .. n end
            if c ~= 0 then return "completed=" .. c end
            local n2, c2 = GetLFGDungeonNumEncounters(9999)
            if n2 ~= 0 then return "unknown_count=" .. n2 end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGDungeonNumEncounters: {result}");
}
