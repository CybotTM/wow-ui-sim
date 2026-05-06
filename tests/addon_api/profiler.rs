use super::env_with_addons;
use std::collections::VecDeque;
use wow_ui_sim::lua_api::WowLuaEnv;

/// Verify that GetApplicationMetric and GetOverallMetric return different values
/// so that addon CPU percentages are not 100%.
#[test]
fn test_profiler_app_vs_overall_metric_differ() {
    let env = env_with_addons();
    create_profiler_update_frame(&env);
    run_profiler_frames(&env);

    let app_val = app_recent_average_metric(&env);
    let overall_val = overall_recent_average_metric(&env);
    let addon_val = addon_recent_average_metric(&env, "MyAddon");

    assert!(app_val > 0.0, "App metric should be positive after frames");
    assert!(
        overall_val > 0.0,
        "Overall metric should be positive (addon ran)"
    );
    assert!(addon_val > 0.0, "Addon metric should be positive");
    assert!(
        app_val > overall_val,
        "App metric ({app_val:.3}) should exceed overall addon metric ({overall_val:.3})"
    );

    let pct = overall_val / app_val * 100.0;
    assert!(
        pct < 100.0,
        "Addon CPU percentage should be < 100%, got {pct:.1}%"
    );
}

#[test]
fn test_profiler_get_top_k_addons_for_metric_returns_sorted_table() {
    let env = env_with_addons();
    seed_top_k_metric_frames(&env);

    let encoded: String = env
        .eval(
            r#"
            local results = C_AddOnProfiler.GetTopKAddOnsForMetric(Enum.AddOnProfilerMetric.RecentAverageTime, 1)
            return table.concat({
                type(results),
                tostring(#results),
                results[1].addOnName,
                tostring(results[1].metricValue),
            }, "|")
            "#,
        )
        .unwrap();

    assert_eq!(encoded, "table|1|LODAddon|6");
}

#[test]
fn test_profiler_check_for_performance_message_reports_specific_addon() {
    let env = env_with_addons();
    seed_performance_warning_metrics(&env);

    let message = read_performance_message(&env);
    acknowledge_pending_performance_message(&env);

    assert_performance_message(message);
}

fn create_profiler_update_frame(env: &WowLuaEnv) {
    set_loading_addon_index(env, Some(1));
    env.eval::<()>(
        r#"
        local f = CreateFrame("Frame", "ProfTestFrame", UIParent)
        f:SetScript("OnUpdate", function(self, elapsed)
            local x = 0
            for i = 1, 5000 do x = x + i end
        end)
        "#,
    )
    .unwrap();
    set_loading_addon_index(env, None);
}

fn set_loading_addon_index(env: &WowLuaEnv, index: Option<u16>) {
    env.state().borrow_mut().loading_addon_index = index;
}

fn run_profiler_frames(env: &WowLuaEnv) {
    for _ in 0..10 {
        env.fire_on_update(0.016).unwrap();
    }
}

fn app_recent_average_metric(env: &WowLuaEnv) -> f64 {
    env.eval(
        "return C_AddOnProfiler.GetApplicationMetric(Enum.AddOnProfilerMetric.RecentAverageTime)",
    )
    .unwrap()
}

fn overall_recent_average_metric(env: &WowLuaEnv) -> f64 {
    env.eval("return C_AddOnProfiler.GetOverallMetric(Enum.AddOnProfilerMetric.RecentAverageTime)")
        .unwrap()
}

fn addon_recent_average_metric(env: &WowLuaEnv, addon_name: &str) -> f64 {
    let script = format!(
        "return C_AddOnProfiler.GetAddOnMetric('{addon_name}', Enum.AddOnProfilerMetric.RecentAverageTime)"
    );
    env.eval(&script).unwrap()
}

fn seed_top_k_metric_frames(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    for addon in state.addons.iter_mut() {
        match addon.folder_name.as_str() {
            "MyAddon" => addon.runtime.recent_frames = VecDeque::from([2.0, 4.0]),
            "LODAddon" => addon.runtime.recent_frames = VecDeque::from([5.0, 7.0]),
            _ => {}
        }
    }
}

fn seed_performance_warning_metrics(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.cvars.set("addonPerformanceMsgWarning", "0.01");
    state.cvars.set("addonPerformanceMsgError", "0.02");
    state.cvars.set("addonPerformanceMsgOverall", "0.75");
    state.app_frame_metrics.recent_frame_ms = VecDeque::from([10.0; 10]);
    state.app_frame_metrics.session_total_ms = 100.0;
    state.app_frame_metrics.session_frame_count = 10;
    state.app_frame_metrics.peak_ms = 10.0;

    let addon = state
        .addons
        .iter_mut()
        .find(|addon| addon.folder_name == "MyAddon")
        .expect("MyAddon should exist");
    addon.runtime.recent_frames = VecDeque::from([1.0; 10]);
    addon.runtime.session_total_ms = 10.0;
    addon.runtime.session_frame_count = 10;
    addon.runtime.peak_ms = 1.0;
}

fn read_performance_message(env: &WowLuaEnv) -> PerformanceMessage {
    let encoded: String = env
        .eval(
            r#"
            local msg = C_AddOnProfiler.CheckForPerformanceMessage()
            if not msg then
                return "nil"
            end
            C_AddOnProfiler.AddPerformanceMessageShown(msg)
            return table.concat({
                tostring(msg.type),
                tostring(msg.metric),
                msg.addOnName,
                tostring(msg.metricValue > msg.thresholdValue),
                tostring(msg.metricValue > 0),
                tostring(msg.thresholdValue > 0),
            }, "|")
            "#,
        )
        .unwrap();

    PerformanceMessage::from_encoded(&encoded)
}

fn acknowledge_pending_performance_message(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        local msg = C_AddOnProfiler.CheckForPerformanceMessage()
        if msg then
            C_AddOnProfiler.AddPerformanceMessageShown(msg)
        end
        "#,
    )
    .unwrap();
}

#[derive(Debug)]
struct PerformanceMessage {
    msg_type: i32,
    metric: i32,
    addon_name: String,
    exceeds_threshold: bool,
    positive_metric: bool,
    positive_threshold: bool,
}

impl PerformanceMessage {
    fn from_encoded(encoded: &str) -> Self {
        assert_ne!(encoded, "nil", "expected profiler message");
        let parts: Vec<_> = encoded.split('|').collect();
        assert_eq!(parts.len(), 6, "expected 6 encoded profiler fields");

        Self {
            msg_type: parts[0].parse().unwrap(),
            metric: parts[1].parse().unwrap(),
            addon_name: parts[2].to_string(),
            exceeds_threshold: parts[3] == "true",
            positive_metric: parts[4] == "true",
            positive_threshold: parts[5] == "true",
        }
    }
}

fn assert_performance_message(message: PerformanceMessage) {
    assert_eq!(message.msg_type, 1);
    assert_eq!(message.metric, 1);
    assert_eq!(message.addon_name, "MyAddon");
    assert!(message.exceeds_threshold);
    assert!(message.positive_metric);
    assert!(message.positive_threshold);
}
