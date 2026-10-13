//! Actual host-secret NUMBER wrappers; same policy/rooting boundary as existing widget tests.
use super::fixtures::{ID, assert_initial_snapshot, assert_lua, seed_record, supplied_record};
use rilua::LuaApiMut;
use rilua::table_security::wrap_host_secret_number;
use wow_ui_sim::lua_api::WowLuaEnv;

fn install_secret_ids(env: &WowLuaEnv) {
    let loader = env.loader_env();
    let mut lua = loader.rilua_mut();
    rilua::table_security::register_table_security(&mut lua).expect("VM security helpers");
    for (name, number) in [
        ("DiscreteSecretID", ID),
        ("DiscreteAbsentSecretID", ID + 1.0),
    ] {
        let value = wrap_host_secret_number(lua.state_mut(), number);
        lua.state_mut().push(value);
        let inserted = lua.set_global_val(name, value);
        lua.state_mut().pop();
        inserted.expect("root actual host secret NUMBER");
    }
}

#[test]
fn discrete_secrets_allow_untainted_deny_tainted_and_recover_public_snapshots() {
    let env = WowLuaEnv::new().expect("security discrete environment");
    seed_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    // Publication and supplied-record boundary precede any denied-input assertions.
    install_secret_ids(&env);
    assert_lua(
        &env,
        "actual secret wrappers and caller policy",
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil)
        local get = C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo
        local supplied, absent = DiscreteSecretID, DiscreteAbsentSecretID
        assert(issecretvalue(supplied) and issecretvalue(absent))
        local function AssertPublicSnapshot(row)
            AssertDiscreteSnapshot(row, expectedDiscrete)
            assert(not issecretvalue(row))
            for key, value in pairs(row) do
                assert(not issecretvalue(key) and not issecretvalue(value))
            end
        end
        local retained = get(supplied)
        AssertPublicSnapshot(retained)
        assert(get(absent) == nil)
        local function addon()
            assert(debug.getstacktaint() == 'DiscreteSecurityProbe')
            for _, id in ipairs({supplied, absent}) do
                local ok, message = pcall(get, id)
                assert(not ok and type(message) == 'string')
                assert(string.find(message, 'untainted caller', 1, true),
                    'deny before supplied/absent lookup: ' .. tostring(message))
                assert(issecretvalue(id), 'wrapped input retained')
                assert(not pcall(secretunwrap, id), 'tainted declassification denied')
            end
            AssertPublicSnapshot(get(760301))
            assert(get(760302) == nil)
            AssertPublicSnapshot(retained)
        end
        debug.setobjecttaint(addon, 'DiscreteSecurityProbe')
        addon()
        assert(debug.getstacktaint() == nil, 'outer caller restored')
        local fresh = get(supplied)
        assert(fresh ~= retained)
        fresh.tooltip = 'caller mutation'
        fresh.progressVal = -999
        AssertPublicSnapshot(retained)
        AssertPublicSnapshot(get(supplied))
        assert(get(absent) == nil)
    "#,
    );
}
