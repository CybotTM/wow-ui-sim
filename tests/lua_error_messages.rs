use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn global_nil_call_error_matches_active_profile() {
    let env = WowLuaEnv::new().expect("lua env should initialize");
    let err = env
        .exec("CooldownFrame_Set = nil; local function f() CooldownFrame_Set() end; f()")
        .expect_err("calling a nil global should fail");

    let expected = match wow_ui_sim::client_profile::ACTIVE {
        wow_ui_sim::client_profile::ClientProfile::Wrath
        | wow_ui_sim::client_profile::ClientProfile::Mists
        | wow_ui_sim::client_profile::ClientProfile::Era
        | wow_ui_sim::client_profile::ClientProfile::Anniversary => {
            "(string):1: attempt to call global 'CooldownFrame_Set' (a nil value)"
        }
        wow_ui_sim::client_profile::ClientProfile::Retail
        | wow_ui_sim::client_profile::ClientProfile::Ptr
        | wow_ui_sim::client_profile::ClientProfile::WowForever => {
            "(string):1: attempt to call a nil value"
        }
    };
    assert!(
        err.to_string().contains(expected),
        "unexpected error: {err}"
    );
}

#[cfg(feature = "profile-retail")]
#[test]
fn retail_nil_global_pcall_preserves_generic_error_and_two_results() {
    let env = WowLuaEnv::new().expect("lua env should initialize");
    env.exec(
        r#"
        rawset(_G, "RetailMissingGlobal", nil)
        local function callMissingGlobal()
            RetailMissingGlobal()
        end
        local function check(ok, err, ...)
            assert(select('#', ok, err, ...) == 2)
            assert(ok == false)
            assert(type(err) == "string")
            local suffix = "attempt to call a nil value"
            assert(string.sub(err, -#suffix) == suffix, err)
            assert(not string.find(err, "RetailMissingGlobal", 1, true), err)
        end
        check(pcall(callMissingGlobal))
        "#,
    )
    .expect("Retail inner pcall must retain generic nil-call wording and two results");
}
