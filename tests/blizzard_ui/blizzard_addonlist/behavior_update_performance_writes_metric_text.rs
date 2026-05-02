//! AddonList performance metric text behavior for `Blizzard_AddOnList`.

use std::collections::VecDeque;

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;
use wow_ui_sim::lua_api::state::AddonRuntimeMetrics;

const ROOT: &str = "Blizzard_AddOnList";
const PROFILER_ADDON: &str = "AddonListProfilerMetricProbe";

#[test]
fn update_performance_writes_metric_text_when_profiler_is_enabled() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_profiler_metrics(env);

        let (
            performance_shown,
            current_text,
            expected_current_text,
            average_text,
            expected_average_text,
            peak_text,
            expected_peak_text,
        ): MetricTextProbe = env
            .eval(
                r#"
                local originalInGlue = InGlue
                InGlue = function() return false end
                SetCVar("addonPerformanceMsgWarning", "0")

                AddonList:UpdatePerformance()

                local currentText = AddonList.Performance.Current:GetText()
                local averageText = AddonList.Performance.Average:GetText()
                local peakText = AddonList.Performance.Peak:GetText()

                InGlue = originalInGlue

                return AddonList.Performance:IsShown(),
                       currentText,
                       ADDON_LIST_PERFORMANCE_CURRENT_CPU:format("25%"),
                       averageText,
                       ADDON_LIST_PERFORMANCE_AVERAGE_CPU:format("25%"),
                       peakText,
                       ADDON_LIST_PERFORMANCE_PEAK_CPU:format("25%")
                "#,
            )
            .expect("AddonList performance metric text probe must run cleanly");

        assert!(
            performance_shown,
            "`UpdatePerformance` must show the performance panel when profiling is enabled out of glue"
        );
        assert!(
            !current_text.is_empty(),
            "`Performance.Current` text must be populated"
        );
        assert_eq!(current_text, expected_current_text);
        assert_eq!(average_text, expected_average_text);
        assert_eq!(peak_text, expected_peak_text);
    });
}

type MetricTextProbe = (bool, String, String, String, String, String, String);

fn seed_profiler_metrics(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    for addon in &mut state.addons {
        addon.runtime = AddonRuntimeMetrics::default();
    }
    state.app_frame_metrics.recent_frame_ms = VecDeque::from([100.0]);
    state.app_frame_metrics.session_total_ms = 200.0;
    state.app_frame_metrics.session_frame_count = 1;
    state.app_frame_metrics.peak_ms = 300.0;
    state.addons.push(AddonInfo {
        folder_name: PROFILER_ADDON.into(),
        title: "AddonList Profiler Metric Probe".into(),
        enabled: true,
        loaded: true,
        runtime: AddonRuntimeMetrics {
            recent_frames: VecDeque::from([25.0]),
            session_total_ms: 50.0,
            session_frame_count: 1,
            peak_ms: 75.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
