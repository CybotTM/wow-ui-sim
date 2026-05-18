//! AddonList glue confirm-disable dialog behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;
use wow_ui_sim::lua_api::{AddonEnableSnapshot, AddonInfo};

const ROOT: &str = "Blizzard_AddOnList";
const OUTDATED_ADDON: &str = "AddonListOutdatedProbe";
const CURRENT_ADDON: &str = "AddonListCurrentProbe";

#[test]
fn confirm_disable_addons_disables_outdated_addons_and_saves() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_dialog_addons(env);

        let probe: ConfirmDisableProbe = env
            .eval(
                r#"
                local originalGetAddOnInfo = C_AddOns.GetAddOnInfo
                local originalDisableAddOn = C_AddOns.DisableAddOn
                local originalSaveAddOns = C_AddOns.SaveAddOns

                local disabledIndices = {}
                local saveCalls = 0

                if not CharacterSelect_CheckDialogStates then
                    CharacterSelect_CheckDialogStates = function() end
                end

                C_AddOns.GetAddOnInfo = function(index)
                    if index == 1 then
                        return "AddonListOutdatedProbe", "Outdated Probe", nil, false, "INTERFACE_VERSION"
                    end
                    if index == 2 then
                        return "AddonListCurrentProbe", "Current Probe", nil, true, nil
                    end
                    return originalGetAddOnInfo(index)
                end
                C_AddOns.DisableAddOn = function(index)
                    table.insert(disabledIndices, index)
                    return originalDisableAddOn(index)
                end
                C_AddOns.SaveAddOns = function()
                    saveCalls = saveCalls + 1
                    return originalSaveAddOns()
                end

                AddonDialog_Show("CONFIRM_DISABLE_ADDONS")
                AddonDialogButton1:Click()

                C_AddOns.ResetAddOns()
                local outdatedStateAfterReset = C_AddOns.GetAddOnEnableState(1, nil)
                local currentStateAfterReset = C_AddOns.GetAddOnEnableState(2, nil)

                C_AddOns.GetAddOnInfo = originalGetAddOnInfo
                C_AddOns.DisableAddOn = originalDisableAddOn
                C_AddOns.SaveAddOns = originalSaveAddOns

                return InGlue(),
                       disabledIndices[1],
                       disabledIndices[2],
                       saveCalls,
                       outdatedStateAfterReset,
                       currentStateAfterReset
                "#,
            )
            .expect("AddOnList glue confirm-disable dialog probe must run cleanly");

        assert_confirm_disable_probe(probe);
    });
}

type ConfirmDisableProbe = (bool, i64, Option<i64>, i64, i64, i64);

fn seed_dialog_addons(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addons.clear();
    state.addons.push(addon_info(OUTDATED_ADDON));
    state.addons.push(addon_info(CURRENT_ADDON));
    state.addon_saved_enable_state = Some(AddonEnableSnapshot::from_addons(&state.addons));
}

fn addon_info(folder_name: &str) -> AddonInfo {
    AddonInfo {
        folder_name: folder_name.into(),
        title: folder_name.into(),
        enabled: true,
        loaded: false,
        ..Default::default()
    }
}

fn assert_confirm_disable_probe(probe: ConfirmDisableProbe) {
    let (
        in_glue,
        first_disabled_index,
        second_disabled_index,
        save_calls,
        outdated_state_after_reset,
        current_state_after_reset,
    ) = probe;

    assert!(
        in_glue,
        "`{ROOT}` glue harness must exercise the glue branch"
    );
    assert_eq!(
        first_disabled_index, 1,
        "accepting `CONFIRM_DISABLE_ADDONS` must disable the outdated addon"
    );
    assert_eq!(
        second_disabled_index, None,
        "`AddonList_DisableOutOfDate` must not disable current addons"
    );
    assert_saved_disable_state(
        save_calls,
        outdated_state_after_reset,
        current_state_after_reset,
    );
}

fn assert_saved_disable_state(
    save_calls: i64,
    outdated_state_after_reset: i64,
    current_state_after_reset: i64,
) {
    assert_eq!(save_calls, 1, "`AddonList_DisableOutOfDate` must save once");
    assert_eq!(
        outdated_state_after_reset, 0,
        "`C_AddOns.SaveAddOns` must persist the disabled outdated addon"
    );
    assert_eq!(
        current_state_after_reset, 2,
        "`C_AddOns.SaveAddOns` must keep current addons enabled"
    );
}
