#[test]
fn recently_moved_startup_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");

    for (needle, owner) in [
        ("function ReloadUI", "Rust event API"),
        ("HasArtifactEquipped", "temporary inert-global workaround"),
        ("IsPVPTimerRunning", "temporary inert-global workaround"),
        (
            "GetAlternativeDefaultLanguage",
            "temporary inert-global workaround",
        ),
        ("UI_SPECIAL_FRAMES", "Rust global table surface"),
        ("UISpecialFrames =", "Rust global table surface"),
        ("StaticPopup_Show", "temporary StaticPopup workaround"),
        ("StaticPopup_Hide", "temporary StaticPopup workaround"),
        (
            "StaticPopup_AddShowCondition",
            "temporary StaticPopup workaround",
        ),
        (
            "StaticPopupDialogs = StaticPopupDialogs or {}",
            "Rust global table surface",
        ),
        (
            "__wow_ensure_glue_character_select_surface",
            "temporary client-info workaround",
        ),
        (
            "__wow_ensure_spellbook_surface",
            "temporary legacy spell workaround",
        ),
        (
            "function RegisterEventCallback",
            "temporary dispatcher callback workaround",
        ),
        (
            "function UnregisterEventCallback",
            "temporary dispatcher callback workaround",
        ),
        (
            "function RegisterUnitEventCallback",
            "temporary dispatcher callback workaround",
        ),
        (
            "function UnregisterUnitEventCallback",
            "temporary dispatcher callback workaround",
        ),
        (
            "function DevTools_AddMessageHandler",
            "temporary dispatcher callback workaround",
        ),
        (
            "__wow_ensure_dispatcher_surface",
            "temporary Dispatcher surface workaround",
        ),
        ("DISPATCHER_VERSION = 2.0", "temporary Dispatcher surface workaround"),
        ("Dispatcher = dispatcher", "temporary Dispatcher surface workaround"),
    ] {
        assert!(
            !bootstrap.contains(needle),
            "{needle} fallback must live in the explicit {owner}, not runtime bootstrap"
        );
    }
}
