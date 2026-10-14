//! Behavioral fixture helpers; never substitute cached consumers or event handlers.
use wow_ui_sim::lua_api::WowLuaEnv;

pub fn assert_lua(env: &WowLuaEnv, phase: &str, code: &str) {
    env.exec(code)
        .unwrap_or_else(|error| panic!("two-record fixture [{phase}]: {error}"));
    let errors = env.state().borrow().lua_errors.clone();
    assert!(
        errors.is_empty(),
        "[{phase}] recorded Lua errors: {errors:?}"
    );
}

pub fn install_snapshot_assertion(env: &WowLuaEnv, getter: &str, expected: &str) {
    assert_lua(
        env,
        "snapshot assertion setup",
        &format!(
            r#"
            FixtureGetter = C_UIWidgetManager.{getter}
            FixtureExpected = {expected}
            function AssertSuppliedSnapshot(...)
                assert(select('#', ...) == 1, 'getter must return exactly one result')
                local actual = ...
                assert(type(actual) == 'table', 'supplied record must be a table')
                local count = 0
                for key, value in pairs(FixtureExpected) do
                    count = count + 1
                    assert(type(actual[key]) == type(value), 'wrong type: ' .. key)
                    assert(actual[key] == value, 'wrong value: ' .. key)
                end
                local actualCount = 0
                for key in pairs(actual) do
                    actualCount = actualCount + 1
                    assert(FixtureExpected[key] ~= nil, 'unexpected field: ' .. tostring(key))
                end
                assert(actualCount == count, 'missing or extra fields')
                return actual
            end
            "#
        ),
    );
}

pub fn assert_fresh_snapshot(env: &WowLuaEnv, id: f64) {
    assert_lua(
        env,
        "complete fresh snapshots and caller mutation isolation",
        &format!(
            r#"
            FirstSnapshot = AssertSuppliedSnapshot(FixtureGetter({id}))
            SavedSnapshot = AssertSuppliedSnapshot(FixtureGetter({id}))
            SavedExpected = FixtureExpected
            assert(FirstSnapshot ~= SavedSnapshot, 'queries must allocate fresh tables')
            for key in pairs(FirstSnapshot) do FirstSnapshot[key] = nil end
            FirstSnapshot.extra = 'caller mutation'
            AssertSuppliedSnapshot(SavedSnapshot)
            local again = AssertSuppliedSnapshot(FixtureGetter({id}))
            assert(again ~= SavedSnapshot and again ~= FirstSnapshot)
            assert(select('#', FixtureGetter({id} + 1)) == 1)
            assert(FixtureGetter({id} + 1) == nil, 'nearby ID must not infer a record')
            "#
        ),
    );
}

pub fn assert_replacement_snapshot(env: &WowLuaEnv, id: f64) {
    assert_lua(
        env,
        "replacement preserves retained snapshot",
        &format!(
            r#"
            local replacement = AssertSuppliedSnapshot(FixtureGetter({id}))
            assert(replacement ~= SavedSnapshot)
            local newExpected = FixtureExpected
            FixtureExpected = SavedExpected
            AssertSuppliedSnapshot(SavedSnapshot)
            FixtureExpected = newExpected
            AssertSuppliedSnapshot(FixtureGetter({id}))
            "#
        ),
    );
}

pub fn assert_selector_policy(env: &WowLuaEnv, id: f64) {
    assert_lua(
        env,
        "shared numeric selector policy",
        &format!(
            r#"
            local function oneNil(...)
                assert(select('#', ...) == 1 and select(1, ...) == nil)
            end
            -- Match the existing shared required-number boundary, not a new integer policy.
            local invalidCalls = {{
                function(getter) return getter() end,
                function(getter) return getter(nil) end,
                function(getter) return getter('not a number') end,
                function(getter) return getter('{id}') end,
                function(getter) return getter(false) end,
                function(getter) return getter({{}}) end,
                function(getter) return getter(function() end) end,
                function(getter) return getter(coroutine.create(function() end)) end,
            }}
            for index, call in ipairs(invalidCalls) do
                local sharedOK = pcall(call, C_UIWidgetManager.GetWidgetSetInfo)
                local ok, err = pcall(call, FixtureGetter)
                assert(not sharedOK and not ok, 'invalid selector accepted: ' .. index)
                assert(type(err) == 'string' and string.find(err, 'must be a number', 1, true))
            end
            for _, selector in ipairs({{0, -1, 1.5, math.huge, -math.huge, 0/0}}) do
                oneNil(C_UIWidgetManager.GetWidgetSetInfo(selector))
                oneNil(FixtureGetter(selector))
            end
            AssertSuppliedSnapshot(FixtureGetter({id}))
            "#
        ),
    );
}

pub fn flush_counts(env: &WowLuaEnv, count: usize) {
    env.fire_on_update(0.0)
        .expect("flush actual cached deferred container layout");
    assert_lua(
        env,
        "post-OnUpdate pool and visible counts",
        &format!(
            r#"
            assert(FixtureContainer.widgetPools:GetNumActive() == {count})
            assert(FixtureContainer:GetNumWidgetsShowing() == {count})
            assert(FixtureContainer.numTimers == 0 and FixtureContainer.ticker == nil)
            "#
        ),
    );
}
