//! Tests for C_Timer.NewTimerID, C_System.GetFrameStack, and
//! C_AddOnProfiler.CheckForPerformanceMessage.

use wow_ui_sim::lua_api::{AddonInfo, WowLuaEnv};
use wow_ui_sim::lua_api::state::AddonRuntimeMetrics;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ── C_Timer.NewTimerID ────────────────────────────────────────────────────────

#[test]
fn new_timer_id_returns_number() {
    let env = env();
    let ty: String = env.eval("return type(C_Timer.NewTimerID())").unwrap();
    assert_eq!(ty, "number");
}

#[test]
fn new_timer_id_returns_monotonically_increasing_ids() {
    let env = env();
    let (id1, id2, id3): (f64, f64, f64) = env
        .eval(
            r#"
            local a = C_Timer.NewTimerID()
            local b = C_Timer.NewTimerID()
            local c = C_Timer.NewTimerID()
            return a, b, c
            "#,
        )
        .unwrap();
    assert!(id1 < id2, "id1={id1} should be less than id2={id2}");
    assert!(id2 < id3, "id2={id2} should be less than id3={id3}");
}

// ── C_System.GetFrameStack ────────────────────────────────────────────────────

#[test]
fn get_frame_stack_returns_empty_when_no_hover() {
    let env = env();
    let count: i32 = env
        .eval("return #C_System.GetFrameStack()")
        .unwrap();
    assert_eq!(count, 0, "no hover → empty stack");
}

#[test]
fn get_frame_stack_returns_table() {
    let env = env();
    let ty: String = env
        .eval("return type(C_System.GetFrameStack())")
        .unwrap();
    assert_eq!(ty, "table");
}

#[test]
fn get_frame_stack_returns_one_frame_when_hovered() {
    let env = env();
    // Create a frame, stash its name, then set hovered_frame via SimState.
    let _: () = env
        .eval(r#"_G.__test_frame = CreateFrame("Frame", "TestHoverFrame", UIParent)"#)
        .unwrap();
    let frame_id: f64 = env
        .eval("return __test_frame:GetID() or 0")
        .unwrap();
    // Use the frame's widget id via name lookup in WidgetRegistry.
    {
        let state = env.state().borrow();
        let id = state.widgets.get_id_by_name("TestHoverFrame");
        drop(state);
        if let Some(id) = id {
            env.state().borrow_mut().hovered_frame = Some(id);
        }
    }
    let _ = frame_id; // suppress unused warning
    let count: i32 = env
        .eval("return #C_System.GetFrameStack()")
        .unwrap();
    assert_eq!(count, 1, "one hovered frame → stack has one element");
}

// ── C_AddOnProfiler.CheckForPerformanceMessage ───────────────────────────────

#[test]
fn check_for_performance_message_returns_nil_by_default() {
    let env = env();
    // No cvars set → thresholds are disabled → no performance message.
    let is_nil: bool = env
        .eval("return C_AddOnProfiler.CheckForPerformanceMessage() == nil")
        .unwrap();
    assert!(is_nil, "no cvars set → nil (no message)");
}

#[test]
fn check_for_performance_message_returns_table_when_threshold_exceeded() {
    let env = env();
    // Seed an addon with high recent-average time and a low cvar threshold.
    {
        let mut state = env.state().borrow_mut();
        // Set a warning threshold of 0.01 (1%) so any non-trivial addon time triggers it.
        state.cvars.set("addonPerformanceMsgWarning", "0.01");
        // Add a fake addon with high metric values so it dominates the average.
        state.app_frame_metrics.session_frame_count = 1;
        state.app_frame_metrics.session_total_ms = 1000.0;
        state.app_frame_metrics.recent_frame_ms.push_back(1000.0);
        state.addons.push(AddonInfo {
            folder_name: "HeavyAddon".into(),
            title: "Heavy".into(),
            enabled: true,
            loaded: true,
            runtime: AddonRuntimeMetrics {
                session_frame_count: 1,
                session_total_ms: 900.0,
                recent_frames: {
                    let mut d = std::collections::VecDeque::new();
                    d.push_back(900.0);
                    d
                },
                ..Default::default()
            },
            ..Default::default()
        });
    }
    let is_table: bool = env
        .eval(
            r#"
            local msg = C_AddOnProfiler.CheckForPerformanceMessage()
            return type(msg) == "table"
            "#,
        )
        .unwrap();
    assert!(is_table, "threshold exceeded → returns a message table");
}
