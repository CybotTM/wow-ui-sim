//! AddonList search-box filtering behavior.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AddOnList";
const MATCHING_ADDON: &str = "SearchFilterUniqueNeedle";

#[test]
fn search_box_filters_rows_to_single_matching_addon() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        register_probe_addons(env);

        let (unfiltered_rows, filtered_rows, only_match_is_probe): (i64, i64, bool) = env
            .eval(
                r#"
                AddonList:Show()
                AddonList.SearchBox:SetText("")
                AddonList_Update()

                local unfilteredProvider = AddonList.ScrollBox:GetDataProvider()
                local unfilteredRows =
                    unfilteredProvider:GetSize(TreeDataProviderConstants.ExcludeCollapsed)

                AddonList.SearchBox:SetText("SearchFilterUniqueNeedle")
                AddonList_Update()

                local filteredProvider = AddonList.ScrollBox:GetDataProvider()
                local filteredRows =
                    filteredProvider:GetSize(TreeDataProviderConstants.ExcludeCollapsed)
                local firstNode = filteredProvider:Find(1, TreeDataProviderConstants.ExcludeCollapsed)
                local firstData = firstNode and firstNode:GetData()
                local firstAddonName = firstData
                    and firstData.addonIndex
                    and C_AddOns.GetAddOnName(firstData.addonIndex)

                return unfilteredRows,
                       filteredRows,
                       firstAddonName == "SearchFilterUniqueNeedle"
                "#,
            )
            .expect("AddonList search filter probe must run cleanly");

        assert!(
            unfiltered_rows > filtered_rows,
            "`AddonList.SearchBox` filtering must reduce visible ScrollBox rows. \
             Unfiltered rows: {unfiltered_rows}; filtered rows: {filtered_rows}"
        );
        assert_eq!(
            filtered_rows, 1,
            "Filtering by the unique probe addon name must leave exactly one visible row"
        );
        assert!(
            only_match_is_probe,
            "The single filtered row must be the explicitly registered matching probe addon"
        );
    });
}

fn register_probe_addons(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(&format!(
        r#"
        A_Admin.RegisterTestAddon({MATCHING_ADDON:?})
        A_Admin.RegisterTestAddon("SearchFilterNonMatchingAddon")
        "#
    ))
    .expect("search filter probe addon registration must run cleanly");
}
