use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn c_macro_namespace_is_not_generic_runtime_bootstrap_fallback() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");

    assert!(
        !bootstrap.contains("C_Macro = C_Macro or __wow_namespace()"),
        "C_Macro must be registered by Rust or the explicit macro workaround boundary, not generic runtime bootstrap"
    );
}

#[test]
fn c_macro_namespace_still_has_rust_backed_macro_text() {
    let env = WowLuaEnv::new().expect("lua env should initialize");
    let result: String = env
        .eval(
            r#"
            if type(C_Macro) ~= "table" then return "missing_namespace" end
            if type(C_Macro.RunMacroText) ~= "function" then return "missing_run_macro_text" end
            return "ok"
            "#,
        )
        .expect("C_Macro probe should run");

    assert_eq!(result, "ok");
}
