//! AddonList performance panel early-return behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AddOnList";

#[test]
fn update_performance_hides_panel_in_glue_or_when_profiler_disabled() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let (hidden_in_glue, hidden_when_profiler_disabled): (bool, bool) = env
            .eval(
                r#"
                local originalInGlue = InGlue
                local originalIsEnabled = C_AddOnProfiler.IsEnabled

                AddonList.Performance:Show()
                InGlue = function() return true end
                C_AddOnProfiler.IsEnabled = function() return true end
                AddonList:UpdatePerformance()
                local hiddenInGlue = not AddonList.Performance:IsShown()

                AddonList.Performance:Show()
                InGlue = function() return false end
                C_AddOnProfiler.IsEnabled = function() return false end
                AddonList:UpdatePerformance()
                local hiddenWhenProfilerDisabled = not AddonList.Performance:IsShown()

                InGlue = originalInGlue
                C_AddOnProfiler.IsEnabled = originalIsEnabled

                return hiddenInGlue, hiddenWhenProfilerDisabled
                "#,
            )
            .expect("AddonList performance early-return probe must run cleanly");

        assert!(
            hidden_in_glue,
            "`UpdatePerformance` must hide the performance panel while in glue"
        );
        assert!(
            hidden_when_profiler_disabled,
            "`UpdatePerformance` must hide the performance panel when the profiler is disabled"
        );
    });
}
