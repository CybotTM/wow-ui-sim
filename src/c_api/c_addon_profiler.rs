//! C_AddOnProfiler: per-addon and application performance metrics.

use crate::lua_api::methods::{borrow_state, create_table, table_set_num, val_to_string};
use crate::lua_bridge::{TableBuilder, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use super::helpers::set_global_val;

type LuaTableRef = rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>;
type RustLuaFn = rilua::vm::closure::RustFn;

const ADDON_PROFILER_METRIC_RECENT_AVERAGE_TIME: i32 = 1;
const ADDON_PERFORMANCE_MESSAGE_TYPE_SPECIFIC_CHAT_WARNING: i32 = 0;
const ADDON_PERFORMANCE_MESSAGE_TYPE_SPECIFIC_ERROR_DIALOG: i32 = 1;
const ADDON_PERFORMANCE_MESSAGE_TYPE_OVERALL_ERROR_DIALOG: i32 = 2;
const ADDON_PERFORMANCE_WARNING_CVAR: &str = "addonPerformanceMsgWarning";
const ADDON_PERFORMANCE_ERROR_CVAR: &str = "addonPerformanceMsgError";
const ADDON_PERFORMANCE_OVERALL_CVAR: &str = "addonPerformanceMsgOverall";

const C_ADDON_PROFILER_METHODS: &[(&str, RustLuaFn)] = &[
    (
        "GetApplicationMetric",
        c_addon_profiler_get_application_metric,
    ),
    ("GetOverallMetric", c_addon_profiler_get_overall_metric),
    ("GetAddOnMetric", c_addon_profiler_get_addon_metric),
    (
        "GetTopKAddOnsForMetric",
        c_addon_profiler_get_top_k_addons_for_metric,
    ),
    (
        "AddPerformanceMessageShown",
        c_addon_profiler_add_performance_message_shown,
    ),
    (
        "CheckForPerformanceMessage",
        c_addon_profiler_check_for_performance_message,
    ),
    ("IsEnabled", c_addon_profiler_is_enabled),
];

pub fn register_c_addon_profiler(state: &mut LuaState) -> LuaResult<()> {
    let profiler = create_table(state);
    let Val::Table(profiler_ref) = profiler else {
        unreachable!("create_table must return a table");
    };
    register_c_addon_profiler_methods(state, profiler_ref)?;
    set_global_val(state, "C_AddOnProfiler", profiler);
    Ok(())
}

fn register_c_addon_profiler_methods(state: &mut LuaState, t: LuaTableRef) -> LuaResult<()> {
    for (name, rust_fn) in C_ADDON_PROFILER_METHODS {
        table_set_rust_fn_static(state, t, name, *rust_fn)?;
    }
    Ok(())
}

// ── metric computation helpers ────────────────────────────────────────────────

fn profiler_metric_kind(_state: &LuaState, metric: Val) -> Option<i32> {
    match metric {
        Val::Num(v) if v.is_finite() && v.fract() == 0.0 => Some(v as i32),
        _ => None,
    }
}

fn average(values: impl Iterator<Item = f64>, count: usize) -> f64 {
    if count == 0 {
        0.0
    } else {
        values.sum::<f64>() / count as f64
    }
}

fn addon_metric_value(addon: &crate::lua_api::AddonInfo, metric: i32) -> f64 {
    match metric {
        0 => {
            if addon.runtime.session_frame_count == 0 {
                0.0
            } else {
                addon.runtime.session_total_ms / addon.runtime.session_frame_count as f64
            }
        }
        1 => average(
            addon.runtime.recent_frames.iter().copied(),
            addon.runtime.recent_frames.len(),
        ),
        4 => addon.runtime.peak_ms,
        _ => 0.0,
    }
}

fn application_metric_value(state: &crate::lua_api::SimState, metric: i32) -> f64 {
    match metric {
        0 => {
            if state.app_frame_metrics.session_frame_count == 0 {
                0.0
            } else {
                state.app_frame_metrics.session_total_ms
                    / state.app_frame_metrics.session_frame_count as f64
            }
        }
        1 => average(
            state.app_frame_metrics.recent_frame_ms.iter().copied(),
            state.app_frame_metrics.recent_frame_ms.len(),
        ),
        4 => state.app_frame_metrics.peak_ms,
        _ => 0.0,
    }
}

fn read_cvar_fraction(sim: &crate::lua_api::SimState, name: &str) -> Option<f64> {
    let value = sim.cvars.get(name)?;
    let parsed = value.parse::<f64>().ok()?;
    if parsed > 0.0 && parsed < 1.0 {
        Some(parsed)
    } else {
        None
    }
}

fn overall_metric_value(state: &crate::lua_api::SimState, metric: i32) -> f64 {
    state
        .addons
        .iter()
        .filter(|addon| addon.folder_name != "__BuiltIn")
        .map(|addon| addon_metric_value(addon, metric))
        .sum()
}

#[derive(Debug, Clone)]
struct PerformanceMessage {
    add_on_name: Option<String>,
    metric: i32,
    metric_value: f64,
    threshold_value: f64,
    message_type: i32,
}

fn find_highest_specific_addon_metric(
    sim: &crate::lua_api::SimState,
    metric: i32,
) -> Option<(String, f64, f64, f64)> {
    let app_value = application_metric_value(sim, metric);
    let overall_value = overall_metric_value(sim, metric);

    sim.addons
        .iter()
        .filter(|addon| addon.folder_name != "__BuiltIn")
        .filter_map(|addon| {
            let metric_value = addon_metric_value(addon, metric);
            if metric_value <= 0.0 {
                return None;
            }

            let denominator = app_value - overall_value + metric_value;
            if denominator <= 0.0 {
                return None;
            }

            let percentage = metric_value / denominator;
            Some((
                addon.folder_name.clone(),
                metric_value,
                denominator,
                percentage,
            ))
        })
        .max_by(|left, right| left.3.total_cmp(&right.3))
}

fn threshold_value_for_specific_message(
    sim: &crate::lua_api::SimState,
    metric: i32,
    add_on_name: &str,
    threshold: f64,
) -> f64 {
    let app_value = application_metric_value(sim, metric);
    let overall_value = overall_metric_value(sim, metric);
    let addon_value = sim
        .addons
        .iter()
        .find(|addon| addon.folder_name == add_on_name)
        .map(|addon| addon_metric_value(addon, metric))
        .unwrap_or(0.0);
    let denominator = app_value - overall_value + addon_value;
    denominator * threshold
}

fn find_specific_performance_message(
    sim: &crate::lua_api::SimState,
    metric: i32,
    warning_threshold: Option<f64>,
    error_threshold: Option<f64>,
) -> Option<PerformanceMessage> {
    let (add_on_name, metric_value, _, percentage) =
        find_highest_specific_addon_metric(sim, metric)?;

    if let Some(error_threshold) = error_threshold
        && percentage > error_threshold
    {
        let threshold_value =
            threshold_value_for_specific_message(sim, metric, &add_on_name, error_threshold);
        return Some(PerformanceMessage {
            add_on_name: Some(add_on_name),
            metric,
            metric_value,
            threshold_value,
            message_type: ADDON_PERFORMANCE_MESSAGE_TYPE_SPECIFIC_ERROR_DIALOG,
        });
    }

    if let Some(warning_threshold) = warning_threshold
        && percentage > warning_threshold
    {
        let threshold_value =
            threshold_value_for_specific_message(sim, metric, &add_on_name, warning_threshold);
        return Some(PerformanceMessage {
            add_on_name: Some(add_on_name),
            metric,
            metric_value,
            threshold_value,
            message_type: ADDON_PERFORMANCE_MESSAGE_TYPE_SPECIFIC_CHAT_WARNING,
        });
    }

    None
}

fn find_overall_performance_message(
    sim: &crate::lua_api::SimState,
    metric: i32,
    threshold: f64,
) -> Option<PerformanceMessage> {
    let app_value = application_metric_value(sim, metric);
    if app_value <= 0.0 {
        return None;
    }

    let overall_value = overall_metric_value(sim, metric);
    let percentage = overall_value / app_value;
    if percentage <= threshold {
        return None;
    }

    Some(PerformanceMessage {
        add_on_name: None,
        metric,
        metric_value: overall_value,
        threshold_value: app_value * threshold,
        message_type: ADDON_PERFORMANCE_MESSAGE_TYPE_OVERALL_ERROR_DIALOG,
    })
}

fn find_performance_message(sim: &crate::lua_api::SimState) -> Option<PerformanceMessage> {
    let warning_threshold = read_cvar_fraction(sim, ADDON_PERFORMANCE_WARNING_CVAR);
    let error_threshold = read_cvar_fraction(sim, ADDON_PERFORMANCE_ERROR_CVAR);
    let overall_threshold = read_cvar_fraction(sim, ADDON_PERFORMANCE_OVERALL_CVAR);
    let metric = ADDON_PROFILER_METRIC_RECENT_AVERAGE_TIME;

    if let Some(message) =
        find_specific_performance_message(sim, metric, warning_threshold, error_threshold)
    {
        return Some(message);
    }

    overall_threshold.and_then(|threshold| find_overall_performance_message(sim, metric, threshold))
}

fn push_performance_message(state: &mut LuaState, message: PerformanceMessage) -> LuaResult<u32> {
    let table = TableBuilder::new(state)
        .set("type", message.message_type)?
        .set("metric", message.metric)?
        .set("metricValue", message.metric_value)?
        .set("thresholdValue", message.threshold_value)?
        .set("addOnName", message.add_on_name)?
        .build();
    state.push(table);
    Ok(1)
}

// ── API implementations ───────────────────────────────────────────────────────

fn push_profiler_metric(
    state: &mut LuaState,
    metric_value: fn(&crate::lua_api::SimState, i32) -> f64,
) -> LuaResult<u32> {
    let metric = profiler_metric_kind(state, stack_val(state, 1)).unwrap_or(1);
    let value = {
        let sim = borrow_state(state)?;
        metric_value(&sim, metric)
    };
    state.push(Val::Num(value));
    Ok(1)
}

macro_rules! profiler_metric_method {
    ($name:ident, $metric_value:ident) => {
        fn $name(state: &mut LuaState) -> LuaResult<u32> {
            push_profiler_metric(state, $metric_value)
        }
    };
}

profiler_metric_method!(
    c_addon_profiler_get_application_metric,
    application_metric_value
);
profiler_metric_method!(c_addon_profiler_get_overall_metric, overall_metric_value);

fn c_addon_profiler_get_addon_metric(state: &mut LuaState) -> LuaResult<u32> {
    let addon_name = val_to_string(state, stack_val(state, 1)).unwrap_or_default();
    let metric = profiler_metric_kind(state, stack_val(state, 2)).unwrap_or(1);
    let value = {
        let sim = borrow_state(state)?;
        sim.addons
            .iter()
            .find(|a| a.folder_name == addon_name)
            .map(|a| addon_metric_value(a, metric))
            .unwrap_or(0.0)
    };
    state.push(Val::Num(value));
    Ok(1)
}

fn top_addons_for_metric(
    sim: &crate::lua_api::SimState,
    metric: i32,
    limit: usize,
) -> Vec<(String, f64)> {
    let mut addons = sim
        .addons
        .iter()
        .filter(|addon| addon.folder_name != "__BuiltIn")
        .map(|addon| (addon.folder_name.clone(), addon_metric_value(addon, metric)))
        .collect::<Vec<_>>();

    addons.sort_by(|left, right| {
        right
            .1
            .total_cmp(&left.1)
            .then_with(|| left.0.cmp(&right.0))
    });
    addons.truncate(limit);
    addons
}

fn c_addon_profiler_get_top_k_addons_for_metric(state: &mut LuaState) -> LuaResult<u32> {
    let metric = profiler_metric_kind(state, stack_val(state, 1)).unwrap_or(1);
    let limit = match stack_val(state, 2) {
        Val::Num(k) if k.is_finite() && k > 0.0 => k.floor() as usize,
        _ => 0,
    };
    let addons = {
        let sim = borrow_state(state)?;
        top_addons_for_metric(&sim, metric, limit)
    };

    let results = create_table(state);
    let Val::Table(results_ref) = results else {
        unreachable!("create_table must return a table");
    };
    for (index, (add_on_name, metric_value)) in addons.into_iter().enumerate() {
        let row = TableBuilder::new(state)
            .set("addOnName", add_on_name)?
            .set("metricValue", metric_value)?
            .build();
        table_set_num(state, results_ref, (index + 1) as f64, row);
    }
    state.push(results);
    Ok(1)
}

fn c_addon_profiler_add_performance_message_shown(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_addon_profiler_is_enabled(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_addon_profiler_check_for_performance_message(state: &mut LuaState) -> LuaResult<u32> {
    let message = {
        let sim = borrow_state(state)?;
        find_performance_message(&sim)
    };
    if let Some(message) = message {
        return push_performance_message(state, message);
    }
    Ok(0)
}
