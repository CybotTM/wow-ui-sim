//! AddonList row title color behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const DISABLED_ADDON: &str = "AddonListDisabledColorProbe";
const LOADABLE_ADDON: &str = "AddonListLoadableColorProbe";

#[test]
fn init_addon_uses_gray_for_disabled_and_gold_for_loadable_enabled() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_color_probe_addons(env);

        let (
            disabled_red,
            disabled_green,
            disabled_blue,
            loadable_red,
            loadable_green,
            loadable_blue,
        ): (f64, f64, f64, f64, f64, f64) = env
            .eval(
                r#"
                local function FindAddonIndex(addonName)
                    for index = 1, C_AddOns.GetNumAddOns() do
                        if C_AddOns.GetAddOnName(index) == addonName then
                            return index
                        end
                    end
                end

                local function InitProbeEntry(addonName, frameName)
                    local addonIndex = FindAddonIndex(addonName)
                    local entry = CreateFrame("Button", frameName, UIParent, "AddonListEntryTemplate")
                    local treeNode = {
                        GetData = function()
                            return { addonIndex = addonIndex }
                        end,
                    }

                    AddonList_InitAddon(entry, treeNode)
                    return entry.Title:GetTextColor()
                end

                local disabledRed, disabledGreen, disabledBlue = InitProbeEntry(
                    "AddonListDisabledColorProbe",
                    "AddonListDisabledColorEntry"
                )
                local loadableRed, loadableGreen, loadableBlue = InitProbeEntry(
                    "AddonListLoadableColorProbe",
                    "AddonListLoadableColorEntry"
                )

                return disabledRed,
                       disabledGreen,
                       disabledBlue,
                       loadableRed,
                       loadableGreen,
                       loadableBlue
                "#,
            )
            .expect("AddonList_InitAddon title color probe must run cleanly");

        assert_color_close(
            (disabled_red, disabled_green, disabled_blue),
            (0.5, 0.5, 0.5),
        );
        assert_color_close(
            (loadable_red, loadable_green, loadable_blue),
            (1.0, 0.78, 0.0),
        );
    });
}

fn seed_color_probe_addons(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addons.push(AddonInfo {
        folder_name: DISABLED_ADDON.into(),
        title: "Addon List Disabled Color Probe".into(),
        enabled: false,
        loaded: false,
        ..Default::default()
    });
    state.addons.push(AddonInfo {
        folder_name: LOADABLE_ADDON.into(),
        title: "Addon List Loadable Color Probe".into(),
        enabled: true,
        loaded: false,
        ..Default::default()
    });
}

fn assert_color_close(actual: (f64, f64, f64), expected: (f64, f64, f64)) {
    const TOLERANCE: f64 = 0.001;

    assert!(
        (actual.0 - expected.0).abs() <= TOLERANCE
            && (actual.1 - expected.1).abs() <= TOLERANCE
            && (actual.2 - expected.2).abs() <= TOLERANCE,
        "expected title color {expected:?}, got {actual:?}"
    );
}
