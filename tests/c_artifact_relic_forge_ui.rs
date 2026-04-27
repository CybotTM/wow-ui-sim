//! Integration tests for the `C_ArtifactRelicForgeUI` namespace
//! registered in `src/c_api/c_artifact_relic_forge_ui.rs`. The
//! `IsAtForge` probe is the only function this namespace currently
//! exposes; it is consumed by `Blizzard_ArtifactUI.lua:115` to gate
//! whether `ARTIFACT_UPDATE` should auto-show the relic-forge panel.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn c_artifact_relic_forge_ui_namespace_is_registered() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env.eval("return type(C_ArtifactRelicForgeUI)").unwrap();
    let fn_kind: String = env
        .eval("return type(C_ArtifactRelicForgeUI.IsAtForge)")
        .unwrap();
    assert_eq!(kind, "table");
    assert_eq!(fn_kind, "function");
}

#[test]
fn is_at_forge_defaults_to_false() {
    let env = WowLuaEnv::new().expect("env");
    let at_forge: bool = env
        .eval("return C_ArtifactRelicForgeUI.IsAtForge()")
        .unwrap();
    assert!(!at_forge);
}

#[test]
fn is_at_forge_reflects_state_flag() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().relic_forge_at_forge = true;
    let at_forge: bool = env
        .eval("return C_ArtifactRelicForgeUI.IsAtForge()")
        .unwrap();
    assert!(at_forge);
}
