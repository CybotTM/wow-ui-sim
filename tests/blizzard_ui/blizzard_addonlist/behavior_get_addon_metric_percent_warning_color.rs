//! AddonList per-addon performance warning text for `Blizzard_AddOnList`.

use std::collections::VecDeque;

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;
use wow_ui_sim::lua_api::state::AddonRuntimeMetrics;

const ROOT: &str = "Blizzard_AddOnList";
const WARNING_ADDON: &str = "AddonListMetricWarningProbe";

#[test]
fn addon_metric_percent_wraps_warning_text_and_alert_icon() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_warning_metric(env);

        let (text, show_warning, expected_text, has_alert_icon): MetricWarningProbe = env
            .eval(
                r#"
                SetCVar("addonPerformanceMsgWarning", "0.10")

                local formatString = "Probe CPU: %s - %s"
                local text, showWarning = AddonList:GetAddonMetricPercent(
                    "AddonListMetricWarningProbe",
                    formatString,
                    Enum.AddOnProfilerMetric.RecentAverageTime
                )
                local expectedPlainText = formatString:format("40%", "AddonListMetricWarningProbe")
                local expectedText = RED_FONT_COLOR:WrapTextInColorCode(expectedPlainText)
                    .. CreateSimpleTextureMarkup([[Interface\DialogFrame\DialogIcon-AlertNew-16]], 16, 16)

                return text,
                       showWarning,
                       expectedText,
                       text:find("DialogIcon%-AlertNew%-16") ~= nil
                "#,
            )
            .expect("AddonList addon metric warning probe must run cleanly");

        assert!(
            show_warning,
            "metric percentage over the cvar threshold must warn"
        );
        assert_eq!(
            text, expected_text,
            "warning text must be red-wrapped and append the alert icon"
        );
        assert!(
            has_alert_icon,
            "warning text must include `DialogIcon-AlertNew-16` markup"
        );
    });
}

type MetricWarningProbe = (String, bool, String, bool);

fn seed_warning_metric(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    for addon in &mut state.addons {
        addon.runtime = AddonRuntimeMetrics::default();
    }
    state.app_frame_metrics.recent_frame_ms = VecDeque::from([100.0]);
    state.addons.push(AddonInfo {
        folder_name: WARNING_ADDON.into(),
        title: "AddonList Metric Warning Probe".into(),
        enabled: true,
        loaded: true,
        runtime: AddonRuntimeMetrics {
            recent_frames: VecDeque::from([40.0]),
            ..Default::default()
        },
        ..Default::default()
    });
}
