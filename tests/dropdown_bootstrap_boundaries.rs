#[test]
fn dropdown_list_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "__wow_register_dropdown_globals",
        "__wow_seed_dropdown_button_template_children",
        "__wow_seed_dropdown_list",
        "DropDownList1",
        "DropDownList2",
        "DropDownList3",
    ] {
        assert!(
            !bootstrap.contains(needle),
            "{needle} fallback must live in the explicit temporary dropdown-list workaround, not runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} fallback must live in the explicit temporary dropdown-list workaround, not shared bootstrap"
        );
    }
}
