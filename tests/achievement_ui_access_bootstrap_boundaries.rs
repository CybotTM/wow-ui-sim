#[test]
fn achievement_ui_access_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "function HasCompletedAnyAchievement()",
        "function CanShowAchievementUI()",
    ] {
        assert!(
            !bootstrap.contains(needle),
            "{needle} must live in the explicit temporary achievement-ui access workaround boundary, not runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} must live in the explicit temporary achievement-ui access workaround boundary, not shared bootstrap"
        );
    }
}
