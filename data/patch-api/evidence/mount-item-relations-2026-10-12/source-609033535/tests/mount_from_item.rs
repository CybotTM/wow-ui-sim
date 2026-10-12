#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn empty_env() -> WowLuaEnv {
    WowLuaEnv::new().expect("fresh mount-item environment")
}

#[test]
fn mount_item_empty_relation_returns_exactly_one_nil() {
    let env = empty_env();
    env.exec(
        r#"
        local query = rawget(C_MountJournal, 'GetMountFromItem')
        assert(type(query) == 'function', 'missing C_MountJournal.GetMountFromItem')
        local function miss(...)
            assert(select('#', ...) == 1, 'mount miss must return exactly ONE value')
            assert((...) == nil, 'empty supplied relation must return nil')
        end
        miss(query(6948))
        miss(query(901001))
        "#,
    )
    .expect("empty relation misses without normal catalog seeding");
}

#[test]
fn mount_item_requires_one_numeric_identity() {
    let env = empty_env();
    env.exec(
        r#"
        local query = rawget(C_MountJournal, 'GetMountFromItem')
        assert(type(query) == 'function', 'missing C_MountJournal.GetMountFromItem')
        local function numberError(...)
            local ok, message = pcall(query, ...)
            assert(not ok and type(message) == 'string', 'required numeric argument must error')
            assert(message:find('number', 1, true), 'expected number contract: ' .. message)
        end
        numberError()
        numberError(nil)
        for _, value in ipairs({false, true, '6948', 'item:6948', {}, function() end,
            coroutine.create(function() end)}) do
            numberError(value)
        end
        "#,
    )
    .expect("required number contract, not optional filter or inferred numeric domain");
}

#[test]
fn mount_item_authenticates_secret_input_before_type_or_lookup() {
    let env = empty_env();
    env.exec(
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil, 'untainted setup')
        local query = rawget(C_MountJournal, 'GetMountFromItem')
        assert(type(query) == 'function', 'missing C_MountJournal.GetMountFromItem')
        local valid = secretwrap(6948)
        local invalid = {secretwrap('6948'), secretwrap(false), secretwrap({}), secretwrap(nil)}
        assert(issecretvalue(valid), 'VM-owned numeric secret wrapper required')
        local function miss(...)
            assert(select('#', ...) == 1, 'secret/ordinary miss arity')
            assert((...) == nil, 'empty relation miss')
        end
        local function numberError(value)
            local ok, message = pcall(query, value)
            assert(not ok and type(message) == 'string' and message:find('number', 1, true),
                'untainted invalid input must reach number validation')
            assert(not message:find('untainted caller', 1, true),
                'untainted invalid input must not fail permission gate')
        end
        miss(query(valid))
        for _, value in ipairs(invalid) do
            assert(issecretvalue(value), 'VM-owned invalid secret wrapper required')
            numberError(value)
        end
        local function denied(value)
            assert(issecretvalue(value), 'authentic wrapper before permission test')
            local ok, message = pcall(query, value)
            assert(not ok and type(message) == 'string', 'tainted secret input must fail')
            assert(message:find('untainted caller', 1, true),
                'AllowedWhenUntainted permission gate must precede type/lookup: ' .. message)
            assert(not message:find('number', 1, true), 'permission denial is not type validation')
            assert(issecretvalue(value), 'denial must retain secret wrapper')
            assert(not pcall(secretunwrap, value), 'tainted caller cannot unwrap secret')
            assert(debug.getstacktaint() == 'MountItemTest', 'denial preserves caller taint')
        end
        local function addon()
            assert(debug.getstacktaint() == 'MountItemTest', 'actual tainted closure')
            denied(valid)
            for _, value in ipairs(invalid) do denied(value) end
            miss(query(6948))
            numberError('6948')
            numberError(false)
            assert(debug.getstacktaint() == 'MountItemTest', 'ordinary query preserves taint')
        end
        debug.setobjecttaint(addon, 'MountItemTest')
        addon()
        assert(debug.getstacktaint() == nil, 'outer secure caller restored')
        assert(issecretvalue(valid), 'secure/tainted round trip retains wrapper')
        miss(query(valid))
        "#,
    )
    .expect("authentic secret permission before malformed input parsing");
}

fn fresh_mount_item_fixture_env() -> WowLuaEnv {
    let env = empty_env();
    assert!(env.state().borrow().mount_item_relations.is_empty());
    env
}

fn assert_mount_item_query(env: &WowLuaEnv, numeric_argument: &str, expected: Option<i32>) {
    let expected = expected.map_or_else(|| "nil".to_owned(), |id| id.to_string());
    let script = format!(
        r#"
        local query = rawget(C_MountJournal, 'GetMountFromItem')
        assert(type(query) == 'function', 'missing C_MountJournal.GetMountFromItem')
        local function check(...)
            assert(select('#', ...) == 1, 'mount query must return exactly ONE value')
            assert((...) == {expected}, 'wrong mount relation for {numeric_argument}')
        end
        check(query({numeric_argument}))
        "#
    );
    env.exec(&script)
        .expect("real mount-item query must match explicit fixture state");
}

#[test]
fn supplied_mount_item_mapping_needs_no_catalog_target_or_ownership() {
    let env = fresh_mount_item_fixture_env();
    {
        let mut state = env.state().borrow_mut();
        state.world.mounts.clear();
        state.mount_item_relations.insert(901001, 401);
    }
    assert!(env.state().borrow().world.mounts.is_empty());
    assert_mount_item_query(&env, "901001", Some(401));
    assert!(env.state().borrow().world.mounts.is_empty());
}

#[test]
fn supplied_mount_item_mapping_replacement_and_removal_are_live() {
    let env = fresh_mount_item_fixture_env();
    env.state()
        .borrow_mut()
        .mount_item_relations
        .insert(901001, 401);
    assert_mount_item_query(&env, "901001", Some(401));

    env.state()
        .borrow_mut()
        .mount_item_relations
        .insert(901001, 402);
    assert_mount_item_query(&env, "901001", Some(402));

    env.state()
        .borrow_mut()
        .mount_item_relations
        .remove(&901001);
    assert_mount_item_query(&env, "901001", None);
}

#[test]
fn supplied_mount_item_mapping_does_not_leak_into_independent_fresh_environment() {
    let first = fresh_mount_item_fixture_env();
    first
        .state()
        .borrow_mut()
        .mount_item_relations
        .insert(901001, 401);
    assert_mount_item_query(&first, "901001", Some(401));

    let second = fresh_mount_item_fixture_env();
    assert_mount_item_query(&second, "901001", None);
    assert_mount_item_query(&first, "901001", Some(401));
    assert!(second.state().borrow().mount_item_relations.is_empty());
}

#[test]
fn supplied_mount_item_numeric_identities_are_distinct_without_truncation_aliases() {
    let env = fresh_mount_item_fixture_env();
    env.state().borrow_mut().mount_item_relations.extend([
        (901000, 400),
        (901001, 401),
        (901002, 402),
        (i32::MIN, 403),
        (i32::MAX, 404),
        (0, 405),
    ]);
    for (argument, expected) in [
        ("901000", 400),
        ("901001", 401),
        ("901002", 402),
        ("-2147483648", 403),
        ("2147483647", 404),
        ("0", 405),
    ] {
        assert_mount_item_query(&env, argument, Some(expected));
    }
    // Populated neighbors, boundaries and zero expose truncation or saturation.
    for argument in [
        "900999",
        "901003",
        "901000.5",
        "901001.5",
        "901002.5",
        "-0.5",
        "0.5",
        "-2147483648.5",
        "2147483647.5",
        "0/0",
        "1/0",
        "-1/0",
        "-2147483649",
        "2147483648",
    ] {
        assert_mount_item_query(&env, argument, None);
    }
}

#[test]
fn supplied_mount_item_secret_identity_hits_and_recovers_across_callers() {
    let env = fresh_mount_item_fixture_env();
    env.state()
        .borrow_mut()
        .mount_item_relations
        .insert(901001, 401);
    env.exec(
        r#"
        debug.settaintmode(true)
        local query = C_MountJournal.GetMountFromItem
        local key = secretwrap(901001)
        assert(issecretvalue(key), 'authentic mapped secret key')
        local function hit(...)
            assert(select('#', ...) == 1 and (...) == 401, 'mapped identity and exact return arity')
        end
        hit(query(key))
        local function addon()
            assert(debug.getstacktaint() == 'MountMappedSecret', 'genuine tainted caller')
            local ok, message = pcall(query, key)
            assert(not ok and message:find('untainted caller', 1, true),
                'mapped secret input denied before lookup')
            assert(issecretvalue(key), 'denial retains original wrapper')
            hit(query(901001))
            assert(debug.getstacktaint() == 'MountMappedSecret', 'ordinary mapped lookup preserves taint')
        end
        debug.setobjecttaint(addon, 'MountMappedSecret')
        addon()
        assert(debug.getstacktaint() == nil and issecretvalue(key), 'secure caller and wrapper restored')
        hit(query(key))
        "#,
    )
    .expect("secure wrapped hits, tainted denial and ordinary mapped recovery");
    assert_eq!(
        env.state().borrow().mount_item_relations.get(&901001),
        Some(&401)
    );
}
