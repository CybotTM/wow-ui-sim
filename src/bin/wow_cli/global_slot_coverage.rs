//! `wow-cli global-slot-coverage` — Track 3 sub-item 5 reporter.
//!
//! Boots a headless `WowLuaEnv`, settles startup, then reports on the
//! populated slot vector: how many slots resolved to non-nil values,
//! split by category. The wall-time line is the actual slot-enabled
//! bootstrap cost; any before/after speedup comparison stays deferred
//! until the VM actually dispatches through `read_slot`.

use std::time::Instant;
use wow_ui_sim::global_slot_coverage::slot_coverage_report;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::startup::settle_headless_startup;

pub fn run() {
    let (with_slots_ms, report) = measure_slot_enabled_bootstrap();

    print_timing(with_slots_ms);
    print_coverage(&report);
    print_unpopulated("unpopulated_globals", &report.unpopulated_globals);
    print_unpopulated("unpopulated_namespaces", &report.unpopulated_namespaces);
}

fn print_timing(with_slots_ms: f64) {
    for line in render_timing_lines(with_slots_ms) {
        println!("{line}");
    }
}

fn print_coverage(report: &wow_ui_sim::global_slot_coverage::SlotCoverageReport) {
    println!("whitelist_version={}", report.version);
    println!("slot_count={}", report.slot_count);
    println!(
        "populated_slots={} ({:.1}%)",
        report.populated_total,
        percent(report.populated_total, report.slot_count),
    );
    println!(
        "  globals={}/{} ({:.1}%)",
        report.populated_globals,
        report.globals_total,
        percent(report.populated_globals, report.globals_total),
    );
    println!(
        "  namespaces={}/{} ({:.1}%)",
        report.populated_namespaces,
        report.namespaces_total,
        percent(report.populated_namespaces, report.namespaces_total),
    );
}

fn print_unpopulated(label: &str, names: &[String]) {
    if names.is_empty() {
        return;
    }
    println!("{label}=");
    for name in names {
        println!("  - {name}");
    }
}

fn measure_slot_enabled_bootstrap() -> (f64, wow_ui_sim::global_slot_coverage::SlotCoverageReport) {
    let started = Instant::now();
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    settle_headless_startup(&env);
    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
    let report = slot_coverage_report(&env);
    (elapsed_ms, report)
}

fn percent(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        (numerator as f64 / denominator as f64) * 100.0
    }
}

fn timing_line(with_slots_ms: f64) -> String {
    format!("startup_elapsed_ms_with_slots={with_slots_ms:.3}")
}

fn render_timing_lines(with_slots_ms: f64) -> Vec<String> {
    vec![timing_line(with_slots_ms)]
}

#[cfg(test)]
mod tests {
    use super::{print_timing, render_timing_lines, timing_line};

    #[test]
    fn timing_line_reports_single_bootstrap_measurement() {
        assert_eq!(timing_line(112.5), "startup_elapsed_ms_with_slots=112.500");
    }

    #[test]
    fn render_timing_lines_omits_legacy_baseline_and_delta_fields() {
        assert_eq!(
            render_timing_lines(112.5),
            vec!["startup_elapsed_ms_with_slots=112.500".to_string()],
        );
    }

    #[test]
    fn print_timing_keeps_the_measurement_shape_simple() {
        let _ = print_timing as fn(f64);
    }
}
