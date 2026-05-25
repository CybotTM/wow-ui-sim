#[test]
fn dropdown_mixins_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "DropdownSelectionTextMixin",
        "WowStyle1DropdownMixin",
        "WowStyle1FilterDropdownMixin",
        "WowStyle1ArrowDropdownMixin",
        "WowDropdownFilterBehaviorMixin",
        "WowFilterButtonMixin",
        "__wow_copy_mixin_methods",
    ] {
        assert!(
            !bootstrap.contains(needle),
            "{needle} must live in the explicit temporary dropdown workaround boundary, not runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} must live in the explicit temporary dropdown workaround boundary, not shared bootstrap"
        );
    }
}
