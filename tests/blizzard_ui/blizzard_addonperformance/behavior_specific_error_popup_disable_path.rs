//! AddOnPerformance specific-error popup accept behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn specific_error_popup_accept_disables_addon_then_reloads_ui() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: SpecificErrorAcceptProbe = env
            .eval(
                r#"
                local originalReloadUI = ReloadUI
                local addOnName = "SpecificErrorAcceptProbeAddon"
                local callOrder = {}

                A_Admin.RegisterTestAddon(addOnName)
                local enabledBefore = C_AddOns.GetAddOnEnableState(addOnName)

                ReloadUI = function()
                    table.insert(callOrder, "reload")
                end

                local originalDisableAddOn = C_AddOns.DisableAddOn
                C_AddOns.DisableAddOn = function(name)
                    table.insert(callOrder, "disable:" .. tostring(name))
                    return originalDisableAddOn(name)
                end

                StaticPopupDialogs.ADDON_PERFORMANCE_SPECIFIC_ERROR.OnAccept(nil, addOnName)

                C_AddOns.DisableAddOn = originalDisableAddOn
                ReloadUI = originalReloadUI

                return enabledBefore,
                       C_AddOns.GetAddOnEnableState(addOnName),
                       callOrder[1],
                       callOrder[2],
                       #callOrder
                "#,
            )
            .expect("AddOnPerformance specific-error accept probe must run cleanly");

        assert_specific_error_accept_probe(probe);
    });
}

type SpecificErrorAcceptProbe = (i64, i64, String, String, i64);

fn assert_specific_error_accept_probe(probe: SpecificErrorAcceptProbe) {
    let (enabled_before, enabled_after, first_call, second_call, call_count) = probe;

    assert_eq!(
        enabled_before, 2,
        "test addon must start enabled before accepting the popup"
    );
    assert_eq!(
        enabled_after, 0,
        "`ADDON_PERFORMANCE_SPECIFIC_ERROR` accept must disable the addon"
    );
    assert_eq!(
        first_call, "disable:SpecificErrorAcceptProbeAddon",
        "popup accept must disable the addon before reloading UI"
    );
    assert_eq!(
        second_call, "reload",
        "popup accept must call `ReloadUI` after disabling the addon"
    );
    assert_eq!(
        call_count, 2,
        "popup accept must only perform disable and reload actions"
    );
}
