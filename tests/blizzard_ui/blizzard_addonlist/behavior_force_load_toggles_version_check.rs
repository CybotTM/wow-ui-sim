//! Force-load checkbox behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AddOnList";

#[test]
fn force_load_click_toggles_addon_version_check() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let (enabled_after_checked_click, enabled_after_unchecked_click): (bool, bool) = env
            .eval(
                r#"
                C_AddOns.SetAddonVersionCheck(true)
                AddonList.ForceLoad:SetChecked(true)
                AddonList.ForceLoad:Click()
                local enabledAfterCheckedClick = C_AddOns.IsAddonVersionCheckEnabled()

                AddonList.ForceLoad:SetChecked(false)
                AddonList.ForceLoad:Click()
                local enabledAfterUncheckedClick = C_AddOns.IsAddonVersionCheckEnabled()

                return enabledAfterCheckedClick, enabledAfterUncheckedClick
                "#,
            )
            .expect("AddonList ForceLoad click probe must run cleanly");

        assert!(
            !enabled_after_checked_click,
            "Checked `AddonList.ForceLoad` click must disable addon version checking"
        );
        assert!(
            enabled_after_unchecked_click,
            "Unchecked `AddonList.ForceLoad` click must enable addon version checking"
        );
    });
}
