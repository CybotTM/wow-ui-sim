#[test]
fn glue_character_select_defaults_are_not_shared_bootstrap_fallbacks() {
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "SetWorldFrameStrata",
        "SetCharSelectModelFrame",
        "SetCharSelectMapSceneFrame",
        "InitializeCharacterScreenData",
        "GetMaxWarbandGroupCount",
        "GetActiveTimerunningSeasonID",
        "GetCharacterListUpdate",
    ] {
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} must live in the explicit temporary glue character-select workaround boundary, not shared bootstrap"
        );
    }
}
