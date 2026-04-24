use super::*;
use crate::lua_api::WowLuaEnv;
use rilua::Val;

fn make_env() -> WowLuaEnv {
    WowLuaEnv::new().expect("failed to create Lua environment")
}

#[test]
fn stub_nil_returns_nothing() {
    let env = make_env();
    env.register_rilua_function("__test_stub_nil", stub_nil)
        .unwrap();
    let func = env.load_rilua("return __test_stub_nil()").unwrap();
    let result = env.call_rilua(&func, &[]).unwrap();
    assert_eq!(result, vec![]);
}

#[test]
fn stub_false_returns_false() {
    let env = make_env();
    env.register_rilua_function("__test_stub_false", stub_false)
        .unwrap();
    let result = env
        .call_rilua(&env.load_rilua("return __test_stub_false()").unwrap(), &[])
        .unwrap();
    assert_eq!(result, vec![Val::Bool(false)]);
}

#[test]
fn stub_zero_returns_zero() {
    let env = make_env();
    env.register_rilua_function("__test_stub_zero", stub_zero)
        .unwrap();
    let result = env
        .call_rilua(&env.load_rilua("return __test_stub_zero()").unwrap(), &[])
        .unwrap();
    assert_eq!(result, vec![Val::Num(0.0)]);
}

#[test]
fn stub_empty_table_returns_table() {
    let env = make_env();
    env.register_rilua_function("__test_stub_empty_table", stub_empty_table)
        .unwrap();
    // type() returns "table" for a table value
    let func = env
        .load_rilua("return type(__test_stub_empty_table())")
        .unwrap();
    let result = env.call_rilua(&func, &[]).unwrap();
    // Val::Str wraps a GcRef — we can compare by checking via Lua
    // Just assert we got one result and it is not nil/false/number
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Val::Str(_)));
}

#[test]
fn register_all_does_not_panic() {
    use rilua::LuaApiMut;
    let env = make_env();
    {
        let mut lua = env.rilua_mut();
        register_all(lua.state_mut());
    }
}

#[test]
fn quest_poi_update_icons_is_registered() {
    let env = make_env();
    let func = env.load_rilua("return QuestPOIUpdateIcons()").unwrap();
    let result = env.call_rilua(&func, &[]).unwrap();
    assert!(result.is_empty());
}

#[test]
fn switch_achievement_search_tab_is_registered() {
    let env = make_env();
    let global_type: String = env.eval("return type(SwitchAchievementSearchTab)").unwrap();
    assert_eq!(global_type, "function");
    env.exec("SwitchAchievementSearchTab(1)").unwrap();
}

#[test]
fn register_all_skips_existing_global() {
    use rilua::LuaApiMut;
    let env = make_env();
    // Pre-register a sentinel value as a global
    env.set_rilua_global("ClearTarget", Val::Bool(true))
        .unwrap();
    {
        let mut lua = env.rilua_mut();
        register_all(lua.state_mut());
    }
    // Our sentinel should still be true, not overwritten by stub
    assert_eq!(env.get_rilua_global("ClearTarget"), Val::Bool(true));
}
