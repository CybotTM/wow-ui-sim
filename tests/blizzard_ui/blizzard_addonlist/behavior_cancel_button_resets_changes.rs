//! Cancel-button reset behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AddOnList";

#[test]
fn cancel_button_resets_pending_addon_changes() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let (disabled_before_cancel, enabled_after_cancel, save_flag_cleared): (bool, bool, bool) =
            env.eval(
                r#"
                local function FindToggleableAddonIndex()
                    for index = 1, C_AddOns.GetNumAddOns() do
                        if C_AddOns.GetAddOnName(index) ~= "__BuiltIn" then
                            return index
                        end
                    end
                end

                local addonIndex = FindToggleableAddonIndex()
                C_AddOns.EnableAddOn(addonIndex)
                C_AddOns.SaveAddOns()
                AddonList.startStatus[addonIndex] = true

                AddonList:Show()
                AddonList_Enable(addonIndex, false)
                local disabledBeforeCancel =
                    C_AddOns.GetAddOnEnableState(addonIndex) <= Enum.AddOnEnableState.None

                AddonList.CancelButton:Click()
                local enabledAfterCancel =
                    C_AddOns.GetAddOnEnableState(addonIndex) > Enum.AddOnEnableState.None

                return disabledBeforeCancel,
                       enabledAfterCancel,
                       AddonList.save == false
                "#,
            )
            .expect("AddonList Cancel-button reset probe must run cleanly");

        assert!(
            disabled_before_cancel,
            "test setup must create a pending disabled-addon change before clicking Cancel"
        );
        assert!(
            enabled_after_cancel,
            "`AddonList.CancelButton:Click()` must trigger `C_AddOns.ResetAddOns()` via `OnHide`"
        );
        assert!(
            save_flag_cleared,
            "`AddonListMixin:OnHide()` must clear `AddonList.save` after the cancel path"
        );
    });
}
