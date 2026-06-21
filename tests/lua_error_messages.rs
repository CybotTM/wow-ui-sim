use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn global_nil_call_error_matches_retail() {
    let env = WowLuaEnv::new().expect("lua env should initialize");
    let err = env
        .exec("CooldownFrame_Set = nil; local function f() CooldownFrame_Set() end; f()")
        .expect_err("calling a nil global should fail");

    assert!(
        err.to_string()
            .contains("(string):1: attempt to call a nil value"),
        "unexpected error: {err}"
    );
}
