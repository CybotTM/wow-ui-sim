//! Initial AddonList population behavior.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AddOnList";
const TEST_ADDON: &str = "InitialShowProbeAddon";

#[test]
fn initial_show_and_update_populates_scroll_rows_for_registered_addons() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        register_probe_addon(env);

        let (registered_addons, scroll_rows, has_probe_row): (i64, i64, bool) = env
            .eval(
                r#"
                AddonList:Show()
                AddonList_Update()

                local provider = AddonList.ScrollBox:GetDataProvider()
                local hasProbeRow = provider:ContainsByPredicate(function(node)
                    local data = node:GetData()
                    return data.addonIndex
                       and C_AddOns.GetAddOnName(data.addonIndex) == "InitialShowProbeAddon"
                end, TreeDataProviderConstants.ExcludeCollapsed)

                return C_AddOns.GetNumAddOns(),
                       provider:GetSize(TreeDataProviderConstants.ExcludeCollapsed),
                       hasProbeRow
                "#,
            )
            .expect("AddonList initial show population probe must run cleanly");

        assert!(
            registered_addons > 0,
            "`C_AddOns.GetNumAddOns()` must include the probe addon before testing row population"
        );
        assert!(
            scroll_rows >= registered_addons,
            "`AddonList_Update()` must populate at least one ScrollBox row per registered addon. \
             Registered addons: {registered_addons}; ScrollBox rows: {scroll_rows}"
        );
        assert!(
            has_probe_row,
            "`AddonList_Update()` must include the explicitly registered probe addon in the ScrollBox rows"
        );
    });
}

fn register_probe_addon(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(&format!("A_Admin.RegisterTestAddon({TEST_ADDON:?})"))
        .expect("probe addon registration must run cleanly");
}
