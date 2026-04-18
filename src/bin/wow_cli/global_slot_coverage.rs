//! `wow-cli global-slot-coverage` — Track 3 sub-item 5 reporter.
//!
//! Boots a headless `WowLuaEnv`, settles startup, then reports on the
//! populated slot vector: how many slots resolved to non-nil values,
//! split by category. This is the measurement proxy for "how many
//! `GETGLOBAL` sites a hypothetical fast-path compiler could rewrite
//! before sub-item 3 lands". Wall-time comparison deferred until the
//! VM actually dispatches through `read_slot`.

use std::time::Instant;
use wow_ui_sim::global_slot_coverage::slot_coverage_report;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::startup::settle_headless_startup;

pub fn run() {
    let baseline_ms = measure_startup_ms(true);
    let (slot_ms, report) = measure_startup_with_report();

    println!("startup_elapsed_ms_without_slots={baseline_ms:.3}");
    println!("startup_elapsed_ms_with_slots={slot_ms:.3}");
    println!("startup_delta_ms={:.3}", baseline_ms - slot_ms);
    println!(
        "startup_delta_percent={:.1}",
        percent_delta(baseline_ms, slot_ms),
    );
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
    if !report.unpopulated_globals.is_empty() {
        println!("unpopulated_globals=");
        for name in &report.unpopulated_globals {
            println!("  - {name}");
        }
    }
    if !report.unpopulated_namespaces.is_empty() {
        println!("unpopulated_namespaces=");
        for name in &report.unpopulated_namespaces {
            println!("  - {name}");
        }
    }
}

fn measure_startup_with_report() -> (f64, wow_ui_sim::global_slot_coverage::SlotCoverageReport) {
    with_slot_mode(false, || {
        let started = Instant::now();
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        settle_headless_startup(&env);
        let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
        let report = slot_coverage_report(&env);
        (elapsed_ms, report)
    })
}

fn measure_startup_ms(disabled: bool) -> f64 {
    with_slot_mode(disabled, || {
        let started = Instant::now();
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        settle_headless_startup(&env);
        started.elapsed().as_secs_f64() * 1000.0
    })
}

fn with_slot_mode<T>(disabled: bool, f: impl FnOnce() -> T) -> T {
    const ENV: &str = "WOW_SIM_DISABLE_GLOBAL_SLOTS";

    if disabled {
        // SAFETY: this CLI subcommand runs single-threaded and toggles
        // the environment only around its own bootstrap measurement.
        unsafe { std::env::set_var(ENV, "1") };
    } else {
        // SAFETY: same reasoning as above.
        unsafe { std::env::remove_var(ENV) };
    }

    let result = f();

    // SAFETY: restore the default slot-enabled mode before returning.
    unsafe { std::env::remove_var(ENV) };
    result
}

fn percent(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        (numerator as f64 / denominator as f64) * 100.0
    }
}

fn percent_delta(baseline_ms: f64, slot_ms: f64) -> f64 {
    if baseline_ms == 0.0 {
        0.0
    } else {
        ((baseline_ms - slot_ms) / baseline_ms) * 100.0
    }
}
