#[test]
fn objective_tracker_frame_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "ObjectiveTrackerFrame",
        "function objectiveTracker:OnAdded",
        "ObjectiveTrackerContainerMixin.Init",
        "MinimizeButton",
    ] {
        assert!(
            !bootstrap.contains(needle),
            "{needle} must live in the explicit temporary misc global-frame workaround boundary, not runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} must live in the explicit temporary misc global-frame workaround boundary, not shared bootstrap"
        );
    }
}
