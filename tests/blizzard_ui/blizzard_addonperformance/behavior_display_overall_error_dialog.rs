//! AddOnPerformance overall-error popup behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn display_overall_error_dialog_uses_popup_without_addon_data() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: OverallErrorDialogProbe = env
            .eval(
                r#"
                local originalStaticPopupShow = StaticPopup_Show
                local callCount = 0
                local capturedWhich = nil
                local capturedTextArg1IsNil = false
                local capturedTextArg2IsNil = false
                local capturedDataIsNil = false

                StaticPopup_Show = function(which, textArg1, textArg2, data)
                    callCount = callCount + 1
                    capturedWhich = which
                    capturedTextArg1IsNil = textArg1 == nil
                    capturedTextArg2IsNil = textArg2 == nil
                    capturedDataIsNil = data == nil
                end

                AddOnPerformance:DisplayMessage({
                    type = Enum.AddOnPerformanceMessageType.OverallAddOnErrorDialog,
                })

                StaticPopup_Show = originalStaticPopupShow

                return callCount,
                       capturedWhich,
                       capturedTextArg1IsNil,
                       capturedTextArg2IsNil,
                       capturedDataIsNil
                "#,
            )
            .expect("AddOnPerformance overall-error dialog probe must run cleanly");

        assert_overall_error_dialog_probe(probe);
    });
}

type OverallErrorDialogProbe = (i64, String, bool, bool, bool);

fn assert_overall_error_dialog_probe(probe: OverallErrorDialogProbe) {
    let (
        call_count,
        captured_which,
        captured_text_arg1_is_nil,
        captured_text_arg2_is_nil,
        captured_data_is_nil,
    ) = probe;

    assert_eq!(
        call_count, 1,
        "`OverallAddOnErrorDialog` must show exactly one popup"
    );
    assert_eq!(
        captured_which, "ADDON_PERFORMANCE_OVERALL_ERROR",
        "`OverallAddOnErrorDialog` must use the registered popup slug"
    );
    assert!(
        captured_text_arg1_is_nil,
        "`OverallAddOnErrorDialog` must not pass an addon name as textArg1"
    );
    assert!(
        captured_text_arg2_is_nil,
        "`OverallAddOnErrorDialog` must not pass textArg2"
    );
    assert!(
        captured_data_is_nil,
        "`OverallAddOnErrorDialog` must not pass popup data"
    );
}
