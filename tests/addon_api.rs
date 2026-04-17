//! Tests for addon API functions (addon_api.rs).

use wow_ui_sim::lua_api::AddonInfo;
use wow_ui_sim::lua_api::WowLuaEnv;

fn env_with_addons() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    {
        let mut state = env.state().borrow_mut();
        state.addons.push(AddonInfo {
            folder_name: "MyAddon".into(),
            title: "My Addon Title".into(),
            notes: "A test addon".into(),
            enabled: true,
            loaded: true,
            load_on_demand: false,
            load_time_secs: 0.0,
            ..Default::default()
        });
        state.addons.push(AddonInfo {
            folder_name: "LODAddon".into(),
            title: "LOD Addon".into(),
            notes: "".into(),
            enabled: false,
            loaded: false,
            load_on_demand: true,
            load_time_secs: 0.0,
            ..Default::default()
        });
    }
    env
}

// ============================================================================
// C_AddOns.GetNumAddOns
// ============================================================================

#[test]
fn test_get_num_addons() {
    let env = env_with_addons();
    // 2 test addons + 1 __BuiltIn pseudo-addon from init_builtin_frames
    let count: i32 = env.eval("return C_AddOns.GetNumAddOns()").unwrap();
    assert_eq!(count, 3);
}

// ============================================================================
// C_AddOns.GetAddOnInfo
// ============================================================================

#[test]
fn test_get_addon_info_by_index() {
    let env = env_with_addons();
    // Index 1 is __BuiltIn, test addons start at index 2
    let (name, title, notes, loadable): (String, String, String, bool) =
        env.eval("return C_AddOns.GetAddOnInfo(2)").unwrap();
    assert_eq!(name, "MyAddon");
    assert_eq!(title, "My Addon Title");
    assert_eq!(notes, "A test addon");
    assert!(loadable);
}

#[test]
fn test_get_addon_info_by_name() {
    let env = env_with_addons();
    let (name, title): (String, String) =
        env.eval("return C_AddOns.GetAddOnInfo('MyAddon')").unwrap();
    assert_eq!(name, "MyAddon");
    assert_eq!(title, "My Addon Title");
}

#[test]
fn test_get_addon_info_not_found() {
    let env = env_with_addons();
    let is_nil: bool = env
        .eval("local n = C_AddOns.GetAddOnInfo(999); return n == nil")
        .unwrap();
    assert!(is_nil);
}

// ============================================================================
// C_AddOns.IsAddOnLoaded
// ============================================================================

#[test]
fn test_is_addon_loaded_by_name() {
    let env = env_with_addons();
    let loaded: bool = env
        .eval("return C_AddOns.IsAddOnLoaded('MyAddon')")
        .unwrap();
    assert!(loaded);
    let not_loaded: bool = env
        .eval("return C_AddOns.IsAddOnLoaded('LODAddon')")
        .unwrap();
    assert!(!not_loaded);
}

#[test]
fn test_is_addon_loaded_by_index() {
    let env = env_with_addons();
    // Index 2 = MyAddon (loaded), Index 3 = LODAddon (not loaded)
    let loaded: bool = env.eval("return C_AddOns.IsAddOnLoaded(2)").unwrap();
    assert!(loaded);
    let not_loaded: bool = env.eval("return C_AddOns.IsAddOnLoaded(3)").unwrap();
    assert!(!not_loaded);
}

// ============================================================================
// C_AddOns.IsAddOnLoadOnDemand
// ============================================================================

#[test]
fn test_is_addon_load_on_demand() {
    let env = env_with_addons();
    let lod: bool = env
        .eval("return C_AddOns.IsAddOnLoadOnDemand('LODAddon')")
        .unwrap();
    assert!(lod);
    let not_lod: bool = env
        .eval("return C_AddOns.IsAddOnLoadOnDemand('MyAddon')")
        .unwrap();
    assert!(!not_lod);
}

// ============================================================================
// C_AddOns.EnableAddOn / DisableAddOn
// ============================================================================

#[test]
fn test_enable_disable_addon_by_name() {
    let env = env_with_addons();
    // LODAddon starts disabled
    let state_before: i32 = env
        .eval("return C_AddOns.GetAddOnEnableState('LODAddon')")
        .unwrap();
    assert_eq!(state_before, 0);

    env.eval::<()>("C_AddOns.EnableAddOn('LODAddon')").unwrap();
    let state_after: i32 = env
        .eval("return C_AddOns.GetAddOnEnableState('LODAddon')")
        .unwrap();
    assert_eq!(state_after, 2);

    env.eval::<()>("C_AddOns.DisableAddOn('LODAddon')").unwrap();
    let state_disabled: i32 = env
        .eval("return C_AddOns.GetAddOnEnableState('LODAddon')")
        .unwrap();
    assert_eq!(state_disabled, 0);
}

#[test]
fn test_enable_disable_addon_by_index() {
    let env = env_with_addons();
    // Index 2 = MyAddon
    env.eval::<()>("C_AddOns.DisableAddOn(2)").unwrap();
    let state: i32 = env.eval("return C_AddOns.GetAddOnEnableState(2)").unwrap();
    assert_eq!(state, 0);

    env.eval::<()>("C_AddOns.EnableAddOn(2)").unwrap();
    let state: i32 = env.eval("return C_AddOns.GetAddOnEnableState(2)").unwrap();
    assert_eq!(state, 2);
}

// ============================================================================
// C_AddOns.EnableAllAddOns / DisableAllAddOns
// ============================================================================

#[test]
fn test_enable_all_disable_all() {
    let env = env_with_addons();
    env.eval::<()>("C_AddOns.DisableAllAddOns()").unwrap();
    // Index 2 = MyAddon, Index 3 = LODAddon
    let s1: i32 = env.eval("return C_AddOns.GetAddOnEnableState(2)").unwrap();
    let s2: i32 = env.eval("return C_AddOns.GetAddOnEnableState(3)").unwrap();
    assert_eq!(s1, 0);
    assert_eq!(s2, 0);

    env.eval::<()>("C_AddOns.EnableAllAddOns()").unwrap();
    let s1: i32 = env.eval("return C_AddOns.GetAddOnEnableState(2)").unwrap();
    let s2: i32 = env.eval("return C_AddOns.GetAddOnEnableState(3)").unwrap();
    assert_eq!(s1, 2);
    assert_eq!(s2, 2);
}

// ============================================================================
// C_AddOns.GetAddOnMetadata
// ============================================================================

#[test]
fn test_get_addon_metadata() {
    let env = env_with_addons();
    let title: String = env
        .eval("return C_AddOns.GetAddOnMetadata('MyAddon', 'Title')")
        .unwrap();
    assert_eq!(title, "My Addon Title");

    let notes: String = env
        .eval("return C_AddOns.GetAddOnMetadata('MyAddon', 'Notes')")
        .unwrap();
    assert_eq!(notes, "A test addon");

    let version: String = env
        .eval("return C_AddOns.GetAddOnMetadata('MyAddon', 'Version')")
        .unwrap();
    assert_eq!(version, "@project-version@");
}

#[test]
fn test_get_addon_metadata_unknown_addon() {
    let env = env_with_addons();
    // For unknown addons, Title returns the addon name itself
    let title: String = env
        .eval("return C_AddOns.GetAddOnMetadata('Unknown', 'Title')")
        .unwrap();
    assert_eq!(title, "Unknown");
}

// ============================================================================
// C_AddOns.DoesAddOnExist
// ============================================================================

#[test]
fn test_does_addon_exist() {
    let env = env_with_addons();
    let exists: bool = env
        .eval("return C_AddOns.DoesAddOnExist('MyAddon')")
        .unwrap();
    assert!(exists);
    let not_exists: bool = env
        .eval("return C_AddOns.DoesAddOnExist('Nonexistent')")
        .unwrap();
    assert!(!not_exists);
}

// ============================================================================
// C_AddOns.GetAddOnName / GetAddOnTitle / GetAddOnNotes
// ============================================================================

#[test]
fn test_get_addon_name_title_notes() {
    let env = env_with_addons();
    // Index 2 = MyAddon
    let name: String = env.eval("return C_AddOns.GetAddOnName(2)").unwrap();
    assert_eq!(name, "MyAddon");
    let title: String = env.eval("return C_AddOns.GetAddOnTitle(2)").unwrap();
    assert_eq!(title, "My Addon Title");
    let notes: String = env.eval("return C_AddOns.GetAddOnNotes(2)").unwrap();
    assert_eq!(notes, "A test addon");
}

#[test]
fn test_get_addon_notes_empty() {
    let env = env_with_addons();
    // Index 3 = LODAddon (empty notes)
    let is_nil: bool = env.eval("return C_AddOns.GetAddOnNotes(3) == nil").unwrap();
    assert!(is_nil, "Empty notes should return nil");
}

// ============================================================================
// C_AddOns.GetAddOnSecurity
// ============================================================================

#[test]
fn test_get_addon_security() {
    let env = env_with_addons();
    // Index 2 = MyAddon
    let sec: String = env.eval("return C_AddOns.GetAddOnSecurity(2)").unwrap();
    assert_eq!(sec, "INSECURE");
}

// ============================================================================
// C_AddOns.IsAddonVersionCheckEnabled / SetAddonVersionCheck
// ============================================================================

#[test]
fn test_version_check_toggle() {
    let env = env_with_addons();
    env.eval::<()>("C_AddOns.SetAddonVersionCheck(true)")
        .unwrap();
    let enabled: bool = env
        .eval("return C_AddOns.IsAddonVersionCheckEnabled()")
        .unwrap();
    assert!(enabled);

    env.eval::<()>("C_AddOns.SetAddonVersionCheck(false)")
        .unwrap();
    let disabled: bool = env
        .eval("return C_AddOns.IsAddonVersionCheckEnabled()")
        .unwrap();
    assert!(!disabled);
}

// ============================================================================
// Legacy global functions
// ============================================================================

#[test]
fn test_legacy_get_num_addons() {
    let env = env_with_addons();
    // 2 test addons + 1 __BuiltIn
    let count: i32 = env.eval("return GetNumAddOns()").unwrap();
    assert_eq!(count, 3);
}

#[test]
fn test_legacy_is_addon_loaded() {
    let env = env_with_addons();
    let loaded: bool = env.eval("return IsAddOnLoaded('MyAddon')").unwrap();
    assert!(loaded);
}

#[test]
fn test_legacy_get_addon_metadata() {
    let env = env_with_addons();
    let title: String = env
        .eval("return GetAddOnMetadata('MyAddon', 'Title')")
        .unwrap();
    assert_eq!(title, "My Addon Title");
}

// ============================================================================
// Global constants
// ============================================================================

#[test]
fn test_addon_actions_blocked_table() {
    let env = env_with_addons();
    let is_table: bool = env
        .eval("return type(ADDON_ACTIONS_BLOCKED) == 'table'")
        .unwrap();
    assert!(is_table);
}

// ============================================================================
// Legacy GetAddOnEnableState (always returns 2)
// ============================================================================

#[test]
fn test_legacy_get_addon_enable_state_always_returns_2() {
    let env = env_with_addons();
    // Even for disabled addons, legacy GetAddOnEnableState always returns 2
    let state: i32 = env
        .eval("return GetAddOnEnableState(2, 'LODAddon')")
        .unwrap();
    assert_eq!(
        state, 2,
        "Legacy GetAddOnEnableState should always return 2"
    );
}

// ============================================================================
// Legacy IsAddOnLoadOnDemand
// ============================================================================

#[test]
fn test_legacy_is_addon_load_on_demand() {
    let env = env_with_addons();
    let lod: bool = env.eval("return IsAddOnLoadOnDemand('LODAddon')").unwrap();
    assert!(lod);
    let not_lod: bool = env.eval("return IsAddOnLoadOnDemand('MyAddon')").unwrap();
    assert!(!not_lod);
}

// ============================================================================
// C_AddOnProfiler runtime metrics
// ============================================================================

/// Verify that GetApplicationMetric and GetOverallMetric return different values
/// so that addon CPU percentages are not 100%.
#[test]
fn test_profiler_app_vs_overall_metric_differ() {
    let env = env_with_addons();
    // Create a frame owned by MyAddon (index 1; index 0 is __BuiltIn).
    {
        let mut state = env.state().borrow_mut();
        state.loading_addon_index = Some(1);
    }
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
    {
        let mut state = env.state().borrow_mut();
        state.loading_addon_index = None;
    }

    // Simulate several frames so metrics accumulate.
    for _ in 0..10 {
        env.fire_on_update(0.016).unwrap(); // ~60fps
    }

    // GetApplicationMetric (total frame time) should be greater than GetOverallMetric
    // (addon-only time), meaning the percentage is not 100%.
    let app_val: f64 = env.eval(
        "return C_AddOnProfiler.GetApplicationMetric(Enum.AddOnProfilerMetric.RecentAverageTime)"
    ).unwrap();
    let overall_val: f64 = env
        .eval("return C_AddOnProfiler.GetOverallMetric(Enum.AddOnProfilerMetric.RecentAverageTime)")
        .unwrap();
    let addon_val: f64 = env.eval(
        "return C_AddOnProfiler.GetAddOnMetric('MyAddon', Enum.AddOnProfilerMetric.RecentAverageTime)"
    ).unwrap();

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

    // The percentage should be less than 100%.
    let pct = overall_val / app_val * 100.0;
    assert!(
        pct < 100.0,
        "Addon CPU percentage should be < 100%, got {pct:.1}%"
    );
}

#[test]
fn test_profiler_check_for_performance_message_reports_specific_addon() {
    let env = env_with_addons();

    {
        let mut state = env.state().borrow_mut();
        state.cvars.set("addonPerformanceMsgWarning", "0.01");
        state.cvars.set("addonPerformanceMsgError", "0.02");
        state.cvars.set("addonPerformanceMsgOverall", "0.75");
        state.app_frame_metrics.recent_frame_ms = std::collections::VecDeque::from([10.0; 10]);
        state.app_frame_metrics.session_total_ms = 100.0;
        state.app_frame_metrics.session_frame_count = 10;
        state.app_frame_metrics.peak_ms = 10.0;

        let addon = state
            .addons
            .iter_mut()
            .find(|addon| addon.folder_name == "MyAddon")
            .expect("MyAddon should exist");
        addon.runtime.recent_frames = std::collections::VecDeque::from([1.0; 10]);
        addon.runtime.session_total_ms = 10.0;
        addon.runtime.session_frame_count = 10;
        addon.runtime.peak_ms = 1.0;
    }

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

    let parts: Vec<_> = encoded.split('|').collect();
    assert_ne!(encoded, "nil", "expected profiler message");
    assert_eq!(parts.len(), 6, "expected 6 encoded profiler fields");

    let msg_type = parts[0].parse::<i32>().unwrap();
    let metric = parts[1].parse::<i32>().unwrap();
    let addon_name = parts[2];
    let exceeds_threshold = parts[3] == "true";
    let positive_metric = parts[4] == "true";
    let positive_threshold = parts[5] == "true";

    env.eval::<()>(
        r#"
        local msg = C_AddOnProfiler.CheckForPerformanceMessage()
        if msg then
            C_AddOnProfiler.AddPerformanceMessageShown(msg)
        end
        "#,
    )
    .unwrap();

    assert_eq!(msg_type, 1);
    assert_eq!(metric, 1);
    assert_eq!(addon_name, "MyAddon");
    assert!(exceeds_threshold);
    assert!(positive_metric);
    assert!(positive_threshold);
}
