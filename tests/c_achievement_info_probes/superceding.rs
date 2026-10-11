use super::env;
use wow_ui_sim::lua_api::WowLuaEnv;

fn relation_env() -> WowLuaEnv {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievement_superceding.insert(6, vec![42, 558, 42]);
        state.achievement_superceding.insert(42, vec![6]);
    }
    env
}

#[test]
fn get_superceding_achievements_returns_one_empty_table_for_absent_relation() {
    let env = env();
    let empty: bool = env
        .eval(
            r#"
            local function capture(...) return select('#', ...), ... end
            local count, result = capture(C_AchievementInfo.GetSupercedingAchievements(999999))
            assert(count == 1, 'expected exactly one relation table')
            assert(type(result) == 'table', 'expected a relation table')
            return next(result) == nil
            "#,
        )
        .unwrap();
    assert!(empty);
}

#[test]
fn get_superceding_achievements_projects_distinct_supplied_sequences() {
    let env = relation_env();
    env.exec(
        r#"
        local function capture(...)
            assert(select('#', ...) == 1, 'exactly one result')
            return ...
        end
        local first = capture(C_AchievementInfo.GetSupercedingAchievements(6))
        local other = capture(C_AchievementInfo.GetSupercedingAchievements(42))
        assert(type(first) == 'table' and #first == 3)
        assert(first[1] == 42 and first[2] == 558 and first[3] == 42)
        assert(type(other) == 'table' and #other == 1 and other[1] == 6)
        "#,
    )
    .unwrap();
}

#[test]
fn get_superceding_achievements_observes_updates_and_removal() {
    let env = relation_env();
    env.exec("RelationBefore = C_AchievementInfo.GetSupercedingAchievements(6)")
        .unwrap();
    env.state()
        .borrow_mut()
        .achievement_superceding
        .insert(6, vec![558]);
    env.exec(
        r#"
        local now = C_AchievementInfo.GetSupercedingAchievements(6)
        assert(type(now) == 'table' and #now == 1 and now[1] == 558)
        assert(#RelationBefore == 3 and RelationBefore[1] == 42)
        local other = C_AchievementInfo.GetSupercedingAchievements(42)
        assert(#other == 1 and other[1] == 6)
        "#,
    )
    .unwrap();
    env.state().borrow_mut().achievement_superceding.remove(&6);
    env.exec(
        r#"
        local now = C_AchievementInfo.GetSupercedingAchievements(6)
        assert(type(now) == 'table' and next(now) == nil)
        assert(#RelationBefore == 3 and RelationBefore[3] == 42)
        "#,
    )
    .unwrap();
}

#[test]
fn get_superceding_achievements_returns_isolated_tables() {
    let env = relation_env();
    env.exec(
        r#"
        local first = C_AchievementInfo.GetSupercedingAchievements(6)
        local second = C_AchievementInfo.GetSupercedingAchievements(6)
        assert(type(first) == 'table' and type(second) == 'table')
        assert(not rawequal(first, second))
        first[1] = 999999
        first[2] = nil
        assert(#second == 3 and second[1] == 42 and second[2] == 558)
        local empty = C_AchievementInfo.GetSupercedingAchievements(999999)
        empty[1] = 6
        local again = C_AchievementInfo.GetSupercedingAchievements(999999)
        assert(type(again) == 'table' and next(again) == nil)
        "#,
    )
    .unwrap();
    assert_eq!(
        env.state().borrow().achievement_superceding.get(&6),
        Some(&vec![42, 558, 42])
    );
    assert!(!env.state().borrow().achievement_superceding.contains_key(&999999));
}

#[test]
fn get_superceding_achievements_requires_a_number() {
    let env = env();
    env.exec(
        r#"
        local query = C_AchievementInfo.GetSupercedingAchievements
        assert(not pcall(query))
        assert(not pcall(query, nil))
        assert(not pcall(query, false))
        assert(not pcall(query, '6'))
        assert(not pcall(query, {}))
        "#,
    )
    .unwrap();
}

#[test]
fn get_superceding_achievements_does_not_alias_unrepresented_numbers() {
    let env = relation_env();
    env.state()
        .borrow_mut()
        .achievement_superceding
        .insert(i32::MAX, vec![558]);
    env.exec(
        r#"
        local fractional = C_AchievementInfo.GetSupercedingAchievements(6.5)
        local large = C_AchievementInfo.GetSupercedingAchievements(2147483648)
        assert(type(fractional) == 'table' and next(fractional) == nil)
        assert(type(large) == 'table' and next(large) == nil)
        "#,
    )
    .unwrap();
}

#[cfg(feature = "retail-12-1-0")]
fn secret_relation_env() -> WowLuaEnv {
    use rilua::LuaApiMut;
    use rilua::table_security::wrap_host_secret_number;

    let env = relation_env();
    {
        let loader = env.loader_env();
        let mut lua = loader.rilua_mut();
        let secret = wrap_host_secret_number(lua.state_mut(), 6.0);
        lua.state_mut().push(secret);
        lua.set_global_val("AchievementRelationSecret6", secret)
            .expect("root authentic host secret number");
        lua.state_mut().pop();
    }
    env
}

#[test]
#[cfg(feature = "retail-12-1-0")]
fn get_superceding_achievements_accepts_untainted_secret_argument() {
    let env = secret_relation_env();
    env.exec(
        r#"
        assert(issecure())
        local taint = debug.getstacktaint()
        local secret = AchievementRelationSecret6
        assert(issecretvalue(secret))
        local result = C_AchievementInfo.GetSupercedingAchievements(secret)
        assert(type(result) == 'table' and #result == 3 and result[1] == 42)
        assert(rawequal(secret, AchievementRelationSecret6) and issecretvalue(secret))
        assert(debug.getstacktaint() == taint and issecure())
        "#,
    )
    .unwrap();
}

#[test]
#[cfg(feature = "retail-12-1-0")]
fn get_superceding_achievements_rejects_tainted_secret_without_changing_taint() {
    let env = secret_relation_env();
    env.exec(
        r#"
        local parentTaint = debug.getstacktaint()
        local secret = AchievementRelationSecret6
        local function addon()
            assert(debug.getstacktaint() == 'AchievementRelationAddon')
            local result = C_AchievementInfo.GetSupercedingAchievements(6)
            assert(type(result) == 'table' and result[1] == 42)
            assert(not pcall(C_AchievementInfo.GetSupercedingAchievements, secret))
            assert(debug.getstacktaint() == 'AchievementRelationAddon')
            assert(rawequal(secret, AchievementRelationSecret6) and issecretvalue(secret))
        end
        debug.setobjecttaint(addon, 'AchievementRelationAddon')
        addon()
        assert(debug.getstacktaint() == parentTaint and issecure())
        assert(rawequal(secret, AchievementRelationSecret6) and issecretvalue(secret))
        "#,
    )
    .unwrap();
}
