use super::{assert_ptr_source_omits_qualified_methods, load_game_ui_without_player_choice};

const SNAPSHOT_ONLY_METHODS: &[&str] = &[
    "CreateNarrationInfo",
    "GetCheckboxContext",
    "MakeIndexInfo",
    "MakeNarrationStringForMoney",
    "MakeNarrationStringFromIndexInfo",
    "MakeNarrationStringFromInfo",
    "MakeNarrationString",
    "NarrateCurrentScreen",
    "RegionToNarrationInfo",
    "ResolveForwardedRegion",
    "SetStaticDescription",
    "SetStaticName",
    "ShouldBeEnabled",
    "ShouldRegionNavigationSkipTooltips",
];

/// Proves all proposed NarrationUtil additions are absent from PTR source and runtime.
#[test]
fn snapshot_only_narration_methods_remain_absent() {
    assert_ptr_source_omits_qualified_methods("NarrationUtil", SNAPSHOT_ONLY_METHODS);

    let env = load_game_ui_without_player_choice();
    let (namespace_type, absent_count): (String, i32) = env
        .eval(
            r#"
            local names = {
                "CreateNarrationInfo",
                "GetCheckboxContext",
                "MakeIndexInfo",
                "MakeNarrationStringForMoney",
                "MakeNarrationStringFromIndexInfo",
                "MakeNarrationStringFromInfo",
                "MakeNarrationString",
                "NarrateCurrentScreen",
                "RegionToNarrationInfo",
                "ResolveForwardedRegion",
                "SetStaticDescription",
                "SetStaticName",
                "ShouldBeEnabled",
                "ShouldRegionNavigationSkipTooltips",
            }
            local absentCount = 0
            for _, name in ipairs(names) do
                if NarrationUtil == nil or NarrationUtil[name] == nil then
                    absentCount = absentCount + 1
                end
            end
            return type(NarrationUtil), absentCount
            "#,
        )
        .expect("NarrationUtil runtime probe succeeds");

    assert_eq!(namespace_type, "nil");
    assert_eq!(absent_count, SNAPSHOT_ONLY_METHODS.len() as i32);
}
