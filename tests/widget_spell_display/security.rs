//! Cached Retail/PTR SecretArguments=AllowedWhenUntainted; existing common ID validator.
use super::discrete_fixtures::assert_lua;
use super::fixtures::{ID, assert_initial_snapshot, seed_record, supplied_record};
use rilua::LuaApiMut;
use rilua::table_security::wrap_host_secret_number;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn spell_display_real_secret_ids_allow_untainted_deny_tainted_recover() {
    let env = WowLuaEnv::new().expect("SpellDisplay security environment");
    seed_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    {
        let loader = env.loader_env();
        let mut lua = loader.rilua_mut();
        rilua::table_security::register_table_security(&mut lua).expect("VM security helpers");
        for (name, id) in [
            ("SpellDisplaySecretID", ID),
            ("SpellDisplayAbsentSecretID", ID + 1.0),
        ] {
            let value = wrap_host_secret_number(lua.state_mut(), id);
            lua.state_mut().push(value);
            let result = lua.set_global_val(name, value);
            lua.state_mut().pop();
            result.expect("root actual secret NUMBER");
        }
    }
    assert_lua(
        &env,
        "SpellDisplay real secret ID policy",
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil)
        local get = C_UIWidgetManager.GetSpellDisplayVisualizationInfo
        local supplied, absent = SpellDisplaySecretID, SpellDisplayAbsentSecretID
        assert(issecretvalue(supplied) and issecretvalue(absent))
        local function public(row)
            AssertSpellSnapshot(row, expectedSpell)
            assert(not issecretvalue(row))
            for key, value in pairs(row) do
                assert(not issecretvalue(key) and not issecretvalue(value))
            end
            for key, value in pairs(row.spellInfo) do
                assert(not issecretvalue(key) and not issecretvalue(value))
            end
        end
        local retained = get(supplied)
        public(retained)
        assert(get(absent) == nil)
        local function addon()
            assert(debug.getstacktaint() == 'SpellDisplaySecurityProbe')
            for _, id in ipairs({supplied, absent}) do
                local ok, message = pcall(get, id)
                assert(not ok and type(message) == 'string')
                assert(string.find(message, 'untainted caller', 1, true), tostring(message))
                assert(issecretvalue(id))
                assert(not pcall(secretunwrap, id), 'no tainted declassification')
            end
            public(get(761301))
            assert(get(761302) == nil)
            public(retained)
        end
        debug.setobjecttaint(addon, 'SpellDisplaySecurityProbe')
        addon()
        assert(debug.getstacktaint() == nil)
        local fresh = get(supplied)
        assert(fresh ~= retained)
        assert(fresh.spellInfo ~= retained.spellInfo)
        fresh.spellInfo.text = 'caller mutation'
        public(retained)
        public(get(supplied))
        assert(get(absent) == nil)
    "#,
    );
}
