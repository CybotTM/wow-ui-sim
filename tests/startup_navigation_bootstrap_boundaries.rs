#[test]
fn startup_navigation_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "__wow_ensure_startup_navigation_surface",
        "function ToggleMailFrame",
        "function OpenAllBags",
        "function ToggleLFDParentFrame",
        "function UpdateRaidAndPartyFrames",
        "function HelpOpenWebTicketButton_OnUpdate",
        "MarkAllSettingsDirty",
    ] {
        assert!(
            !bootstrap.contains(needle),
            "{needle} defaults must live in the explicit temporary startup navigation workaround boundary, not runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} defaults must live in the explicit temporary startup navigation workaround boundary, not shared bootstrap"
        );
    }
}
