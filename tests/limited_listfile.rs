#[test]
fn bundled_limited_listfile_resolves_common_assets_case_insensitively() {
    assert_eq!(
        wow_ui_sim::limited_listfile::lookup_path("Fonts/frizqt__.ttf"),
        Some(615960)
    );
    assert_eq!(
        wow_ui_sim::limited_listfile::lookup_path("INTERFACE/BUTTONS/UI-PANEL-BUTTON-UP.BLP"),
        Some(130828)
    );
    assert_eq!(
        wow_ui_sim::limited_listfile::lookup_path("Interface/Icons/Trade_Engineering.blp"),
        Some(136243)
    );
}

#[test]
fn bundled_limited_listfile_preserves_font_path_casing() {
    let entry = wow_ui_sim::limited_listfile::lookup_entry("fonts/frizqt__.ttf")
        .expect("FRIZQT__.TTF listfile entry");

    assert_eq!(entry.fdid, 615960);
    assert_eq!(entry.path, "Fonts/FRIZQT__.TTF");
}
