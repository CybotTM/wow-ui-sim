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
