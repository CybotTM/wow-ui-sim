use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

#[test]
fn get_lfd_choice_order_returns_ids() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local order = GetLFDChoiceOrder()
            if type(order) ~= "table" then return "type=" .. type(order) end
            if #order < 8 then return "count=" .. #order end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFDChoiceOrder: {result}");
}

#[test]
fn get_lfg_dungeon_info_seeded_id() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local name = GetLFGDungeonInfo(1203)
            if name == nil then return "nil" end
            if type(name) ~= "string" then return "type=" .. type(name) end
            if name == "" then return "empty" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGDungeonInfo(1203): {result}");
}

#[test]
fn get_lfg_dungeon_info_returns_21_values() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local t = {GetLFGDungeonInfo(1203)}
            if #t ~= 21 then return "count=" .. #t end
            -- t[1] = name, t[2] = typeID, t[13] = maxPlayers
            if type(t[1]) ~= "string" then return "name_type=" .. type(t[1]) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGDungeonInfo 21 values: {result}");
}

#[test]
fn get_lfg_dungeon_info_unknown_returns_nil() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local v = GetLFGDungeonInfo(9999)
            return v == nil and "nil" or "not_nil"
            "#,
        )
        .unwrap();
    assert_eq!(result, "nil");
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
