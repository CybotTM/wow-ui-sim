use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use wow_ui_sim::lua_api::WowLuaEnv;

/// Template to benchmark: (lua_template_name, widget_type, count).
#[derive(Clone, Copy, Debug)]
pub struct TemplateBench {
    pub template: &'static str,
    pub widget_type: &'static str,
    pub count: usize,
}

pub const ACTION_BUTTON_SPELLFX_BENCH: TemplateBench = TemplateBench {
    template: "ActionButtonSpellFXTemplate",
    widget_type: "CheckButton",
    count: 10,
};

pub const ACTION_BUTTON_TEMPLATE_BENCH: TemplateBench = TemplateBench {
    template: "ActionButtonTemplate",
    widget_type: "CheckButton",
    count: 10,
};

pub const MINIMAL_SCROLLBAR_BENCH: TemplateBench = TemplateBench {
    template: "MinimalScrollBar",
    widget_type: "EventFrame",
    count: 10,
};

pub struct TemplateBenchResult {
    pub template: &'static str,
    pub count: usize,
    pub elapsed: Duration,
    pub frames_created: usize,
}

static ACTION_BAR_BENCH_SERIAL: AtomicUsize = AtomicUsize::new(0);

/// Measure CreateFrame throughput for a specific template.
/// Requires a fully loaded game UI environment (templates must be registered).
pub fn measure_template_create(env: &WowLuaEnv, bench: &TemplateBench) -> TemplateBenchResult {
    let frames_before = env.state().borrow().widgets.iter_ids().count();

    let code = format!(
        r#"
        for i = 1, {} do
            CreateFrame("{}", nil, UIParent, "{}")
        end
    "#,
        bench.count, bench.widget_type, bench.template,
    );

    let started = Instant::now();
    env.exec(&code)
        .unwrap_or_else(|e| panic!("template bench {} failed: {e}", bench.template));
    let elapsed = started.elapsed();

    let frames_after = env.state().borrow().widgets.iter_ids().count();

    TemplateBenchResult {
        template: bench.template,
        count: bench.count,
        elapsed,
        frames_created: frames_after - frames_before,
    }
}

/// Run the profiled startup hot paths against one loaded game UI. Each case
/// mutates the shared env, so action-bar button names use a per-run prefix.
pub fn measure_profiled_startup_hot_paths(
    env: &WowLuaEnv,
    action_bar_count: usize,
) -> Vec<TemplateBenchResult> {
    vec![
        measure_template_create(env, &ACTION_BUTTON_SPELLFX_BENCH),
        measure_template_create(env, &ACTION_BUTTON_TEMPLATE_BENCH),
        measure_template_create(env, &MINIMAL_SCROLLBAR_BENCH),
        measure_action_bar_button_family(env, action_bar_count),
    ]
}

/// Measure the action-bar button family creation pattern:
/// CreateFrame("CheckButton", name, parent, template, id).
/// This mirrors what ActionBarMixin:ActionBar_OnLoad does at startup.
pub fn measure_action_bar_button_family(env: &WowLuaEnv, count: usize) -> TemplateBenchResult {
    let frames_before = env.state().borrow().widgets.iter_ids().count();
    let run_id = ACTION_BAR_BENCH_SERIAL.fetch_add(1, Ordering::Relaxed);

    let code = format!(
        r#"
        local container = CreateFrame("Frame", nil, UIParent)
        for i = 1, {count} do
            CreateFrame("CheckButton", "PerfActionBtn{run_id}_" .. i, container, "ActionBarButtonTemplate", i)
        end
    "#,
    );

    let started = Instant::now();
    env.exec(&code)
        .unwrap_or_else(|e| panic!("action-bar button family bench failed: {e}"));
    let elapsed = started.elapsed();

    let frames_after = env.state().borrow().widgets.iter_ids().count();

    TemplateBenchResult {
        template: "ActionBarButtonTemplate",
        count,
        elapsed,
        frames_created: frames_after - frames_before,
    }
}
