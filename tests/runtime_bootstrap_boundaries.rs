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
fn state_backed_namespaces_are_not_generic_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");

    for namespace in ["C_PaperDollInfo", "C_Widget"] {
        let fallback = format!("{namespace} = {namespace} or __wow_namespace()");
        assert!(
            !bootstrap.contains(&fallback),
            "{namespace} must be registered by its Rust C API surface, not generic runtime bootstrap"
        );
    }
}

#[test]
fn profession_spec_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");

    assert!(
        !bootstrap.contains("C_ProfSpecs"),
        "C_ProfSpecs defaults must live in the explicit temporary professions workaround boundary"
    );
}

#[test]
fn legacy_spell_wrappers_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");

    assert!(
        !bootstrap.contains("IsPressHoldReleaseSpell"),
        "legacy spell wrappers must live in the explicit temporary legacy spell workaround boundary"
    );
}

#[test]
fn game_time_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");

    assert!(
        !bootstrap.contains("GameTime_GetTime"),
        "GameTime defaults must live in the explicit temporary GameTime/calendar workaround boundary"
    );
}

#[test]
fn audio_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");

    assert!(
        !bootstrap.contains("C_CombatAudioAlert"),
        "combat audio defaults must live in the explicit temporary sound workaround boundary"
    );
}

#[test]
fn ui_frame_manager_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");

    assert!(
        !bootstrap.contains("UIFrameManager"),
        "UIFrameManager defaults must live in the explicit temporary UI workaround boundary"
    );
}

#[test]
fn recruit_a_friend_surface_is_not_runtime_bootstrap_fallback() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");

    assert!(
        !bootstrap.contains("C_RecruitAFriend"),
        "C_RecruitAFriend must be registered by its Rust missing-surface module, not runtime bootstrap"
    );
}

#[test]
fn prototype_dialog_surface_is_not_runtime_bootstrap_fallback() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");

    assert!(
        !bootstrap.contains("C_PrototypeDialog"),
        "C_PrototypeDialog must live in the explicit temporary C API shim boundary, not runtime bootstrap"
    );
}

#[test]
fn transmog_sets_surface_is_not_runtime_bootstrap_fallback() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");

    assert!(
        !bootstrap.contains("C_TransmogSets"),
        "C_TransmogSets defaults must live in the explicit temporary C API shim boundary, not runtime bootstrap"
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

#[test]
fn state_backed_namespaces_still_have_registered_members() {
    let env = WowLuaEnv::new().expect("lua env should initialize");
    let result: String = env
        .eval(
            r#"
            if type(C_PaperDollInfo) ~= "table" then return "missing_paper_doll" end
            if type(C_PaperDollInfo.GetArmorEffectiveness) ~= "function" then return "missing_armor" end
            if type(C_Widget) ~= "table" then return "missing_widget" end
            if type(C_Widget.IsFrameWidget) ~= "function" then return "missing_widget_fn" end
            if C_Widget.IsFrameWidget({}) ~= false then return "widget_table" end
            return "ok"
            "#,
        )
        .expect("state-backed namespace probe should run");

    assert_eq!(result, "ok");
}
