//! AddOnPerformance overall-error popup accept behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ADDON_LIST_ROOT: &str = "Blizzard_AddOnList";
const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn overall_error_popup_accept_opens_addon_list_panel() {
    with_blizzard_addon_startup_shape(&[ROOT, ADDON_LIST_ROOT], &[], |env, _loaded| {
        let probe: OverallErrorAcceptProbe = env
            .eval(
                r#"
                local originalShowUIPanel = ShowUIPanel
                local callCount = 0
                local targetWasAddonList = false

                AddonList:Hide()
                local shownBefore = AddonList:IsShown()

                ShowUIPanel = function(frame, ...)
                    callCount = callCount + 1
                    targetWasAddonList = frame == AddonList
                    return originalShowUIPanel(frame, ...)
                end

                StaticPopupDialogs.ADDON_PERFORMANCE_OVERALL_ERROR.OnAccept(nil, nil)

                ShowUIPanel = originalShowUIPanel

                return callCount,
                       targetWasAddonList,
                       shownBefore,
                       AddonList:IsShown()
                "#,
            )
            .expect("AddOnPerformance overall-error accept probe must run cleanly");

        assert_overall_error_accept_probe(probe);
    });
}

type OverallErrorAcceptProbe = (i64, bool, bool, bool);

fn assert_overall_error_accept_probe(probe: OverallErrorAcceptProbe) {
    let (call_count, target_was_addon_list, shown_before, shown_after) = probe;

    assert_eq!(
        call_count, 1,
        "`ADDON_PERFORMANCE_OVERALL_ERROR` accept must call `ShowUIPanel` once"
    );
    assert!(
        target_was_addon_list,
        "overall error popup must pass the `AddonList` frame to `ShowUIPanel`"
    );
    assert!(
        !shown_before,
        "test setup must start with the `AddonList` frame hidden"
    );
    assert!(
        shown_after,
        "overall error popup accept must show the `AddonList` frame"
    );
}
