//! AddOnPerformance specific-error popup behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn display_specific_error_dialog_uses_addon_name_as_popup_data() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: SpecificErrorDialogProbe = env
            .eval(
                r#"
                local originalStaticPopupShow = StaticPopup_Show
                local callCount = 0
                local capturedWhich = nil
                local capturedTextArg1 = nil
                local capturedTextArg2IsNil = false
                local capturedData = nil
                local addOnName = "SpecificErrorDialogProbe"

                StaticPopup_Show = function(which, textArg1, textArg2, data)
                    callCount = callCount + 1
                    capturedWhich = which
                    capturedTextArg1 = textArg1
                    capturedTextArg2IsNil = textArg2 == nil
                    capturedData = data
                end

                AddOnPerformance:DisplayMessage({
                    type = Enum.AddOnPerformanceMessageType.SpecificAddOnErrorDialog,
                    addOnName = addOnName,
                })

                StaticPopup_Show = originalStaticPopupShow

                return callCount,
                       capturedWhich,
                       capturedTextArg1,
                       capturedTextArg2IsNil,
                       capturedData,
                       addOnName
                "#,
            )
            .expect("AddOnPerformance specific-error dialog probe must run cleanly");

        assert_specific_error_dialog_probe(probe);
    });
}

type SpecificErrorDialogProbe = (i64, String, String, bool, String, String);

fn assert_specific_error_dialog_probe(probe: SpecificErrorDialogProbe) {
    let (
        call_count,
        captured_which,
        captured_text_arg1,
        captured_text_arg2_is_nil,
        captured_data,
        add_on_name,
    ) = probe;

    assert_eq!(
        call_count, 1,
        "`SpecificAddOnErrorDialog` must show exactly one popup"
    );
    assert_eq!(
        captured_which, "ADDON_PERFORMANCE_SPECIFIC_ERROR",
        "`SpecificAddOnErrorDialog` must use the registered popup slug"
    );
    assert_eq!(
        captured_text_arg1, add_on_name,
        "`SpecificAddOnErrorDialog` must pass the addon name as textArg1"
    );
    assert!(
        captured_text_arg2_is_nil,
        "`SpecificAddOnErrorDialog` must pass nil as textArg2"
    );
    assert_eq!(
        captured_data, add_on_name,
        "`SpecificAddOnErrorDialog` must pass the addon name as popup data"
    );
}
