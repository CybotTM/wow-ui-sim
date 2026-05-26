use std::fs;

#[test]
fn global_placeholder_tables_are_temporary_workaround_not_missing_surface() {
    let missing_surface = fs::read_to_string("src/lua_api/globals/missing_surface.rs")
        .expect("read missing_surface source");
    let workaround =
        fs::read_to_string("src/lua_api/workarounds/temporary/global_placeholder_tables.rs")
            .expect("read global placeholder workaround source");

    for symbol in [
        "StaticPopupDialogs",
        "UIPanelWindows",
        "SOUNDKIT",
        "UISpecialFrames",
        "UI_SPECIAL_FRAMES",
    ] {
        assert!(
            !missing_surface.contains(symbol),
            "`{symbol}` placeholder should not be seeded by globals::missing_surface"
        );
        assert!(
            workaround.contains(symbol),
            "`{symbol}` placeholder should be owned by global_placeholder_tables"
        );
    }
}
