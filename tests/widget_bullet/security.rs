//! Real VM host-secret NUMBER selectors, not mock Lua wrappers.
use super::fixtures::{ID, assert_initial_snapshot, assert_lua, supplied_record, supply_record};
use rilua::LuaApiMut;
use rilua::table_security::wrap_host_secret_number;
use wow_ui_sim::lua_api::WowLuaEnv;

fn install_secret_ids(env: &WowLuaEnv) {
    let loader = env.loader_env();
    let mut lua = loader.rilua_mut();
    rilua::table_security::register_table_security(&mut lua).expect("VM security helpers");
    for (name, number) in [("BulletSecretID", ID), ("BulletAbsentSecretID", ID + 1.0)] {
        let value = wrap_host_secret_number(lua.state_mut(), number);
        lua.state_mut().push(value);
        let inserted = lua.set_global_val(name, value);
        lua.state_mut().pop();
        inserted.expect("root real secret NUMBER");
    }
}

#[test]
fn bullet_secrets_allowed_untainted_denied_tainted_before_present_absent_lookup() {
    let env = WowLuaEnv::new().expect("bullet security environment");
    supply_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    install_secret_ids(&env);
    assert_lua(
        &env,
        "AllowedWhenUntainted real selectors and public snapshots",
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil)
        local get = C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo
        local supplied, absent = BulletSecretID, BulletAbsentSecretID
        assert(issecretvalue(supplied) and issecretvalue(absent))
        local function assertPublic(row)
            AssertBulletSnapshot(row, expectedBullet)
            assert(not issecretvalue(row) and not issecretvalue(row.lines))
            for key, value in pairs(row) do
                assert(not issecretvalue(key) and not issecretvalue(value))
            end
            for index, text in ipairs(row.lines) do
                assert(not issecretvalue(index) and not issecretvalue(text))
            end
        end
        local retained = get(supplied)
        assertPublic(retained)
        assert(select('#', get(absent)) == 1 and get(absent) == nil)
        local function addon()
            assert(debug.getstacktaint() == 'BulletSecurityProbe')
            for _, id in ipairs({supplied, absent}) do
                local ok, message = pcall(get, id)
                assert(not ok and type(message) == 'string')
                assert(string.find(message, 'untainted caller', 1, true),
                    'deny before present/absent lookup: ' .. tostring(message))
                assert(issecretvalue(id))
                assert(not pcall(secretunwrap, id), 'tainted declassification denied')
            end
            assertPublic(get(760401))
            assert(get(760402) == nil)
            assertPublic(retained)
        end
        debug.setobjecttaint(addon, 'BulletSecurityProbe')
        addon()
        assert(debug.getstacktaint() == nil, 'outer stack restored')
        local fresh = get(supplied)
        assert(fresh ~= retained and fresh.lines ~= retained.lines)
        fresh.lines[1] = 'caller mutation'
        assertPublic(retained)
        assertPublic(get(supplied))
    "#,
    );
}
