//! C_AddOnProfiler: per-addon and application performance metrics.

use crate::lua_api::methods::{borrow_state, create_table, val_to_string};
use crate::lua_bridge::{stack_val, table_set_rust_fn};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use super::set_global_val;

pub fn register_c_addon_profiler(state: &mut LuaState) -> LuaResult<()> {
    let profiler = create_table(state);
    let Val::Table(profiler_ref) = profiler else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(
        state,
        profiler_ref,
        "GetApplicationMetric",
        c_addon_profiler_get_application_metric,
    )?;
    table_set_rust_fn(
        state,
        profiler_ref,
        "GetOverallMetric",
        c_addon_profiler_get_overall_metric,
    )?;
    table_set_rust_fn(
        state,
        profiler_ref,
        "GetAddOnMetric",
        c_addon_profiler_get_addon_metric,
    )?;
    table_set_rust_fn(
        state,
        profiler_ref,
        "CheckForPerformanceMessage",
        c_addon_profiler_check_for_performance_message,
    )?;
    set_global_val(state, "C_AddOnProfiler", profiler);
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

// ── API implementations ───────────────────────────────────────────────────────

fn c_addon_profiler_get_application_metric(state: &mut LuaState) -> LuaResult<u32> {
    let metric = profiler_metric_kind(state, stack_val(state, 1)).unwrap_or(1);
    let value = {
        let sim = borrow_state(state)?;
        application_metric_value(&sim, metric)
    };
    state.push(Val::Num(value));
    Ok(1)
}

fn c_addon_profiler_get_overall_metric(state: &mut LuaState) -> LuaResult<u32> {
    let metric = profiler_metric_kind(state, stack_val(state, 1)).unwrap_or(1);
    let value = {
        let sim = borrow_state(state)?;
        sim.addons
            .iter()
            .filter(|a| a.folder_name != "__BuiltIn")
            .map(|a| addon_metric_value(a, metric))
            .sum::<f64>()
    };
    state.push(Val::Num(value));
    Ok(1)
}

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

fn c_addon_profiler_check_for_performance_message(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
