#[test]
fn character_creation_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "C_CharacterCreation",
        "__wow_character_create_races",
        "__wow_character_create_classes",
        "__wow_character_create_categories",
    ] {
        assert!(
            !bootstrap.contains(needle),
            "{needle} must live in the explicit temporary character-create workaround boundary, not runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} must live in the explicit temporary character-create workaround boundary, not shared bootstrap"
        );
    }
}
