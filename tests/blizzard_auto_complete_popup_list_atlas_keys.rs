use wow_ui_sim::atlas::get_atlas_info;

const POPUP_LIST_ATLAS_KEYS: &[&str] = &[
    "search-highlight",
    "talents-search-suggestion-itemborder",
    "_search-rowbg",
    "UI-Frame-BotCornerLeft",
    "UI-Frame-BotCornerRight",
    "_UI-Frame-Bot",
    "!UI-Frame-LeftTile",
    "!UI-Frame-RightTile",
];

#[test]
fn blizzard_auto_complete_popup_list_xml_atlas_keys_resolve() {
    let missing = POPUP_LIST_ATLAS_KEYS
        .iter()
        .copied()
        .filter(|key| get_atlas_info(key).is_none())
        .collect::<Vec<_>>();

    assert!(
        missing.is_empty(),
        "AutoCompletePopupList XML atlas keys must resolve: {missing:?}"
    );
}
