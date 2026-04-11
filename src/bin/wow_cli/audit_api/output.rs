use serde::Serialize;
use std::collections::BTreeMap;

use super::gap::{GapEntry, GapReport, GapSummary};
use super::{AuditResults, SymbolUsage};

/// Print results in human-readable text format.
pub fn print_text(results: &AuditResults) {
    print_c_api_usage(results);
    print_symbol_section("Global Function Calls", &results.global_functions);
    print_symbol_section("Inherited Templates", &results.inherited_templates);
    print_symbol_section("LE_* Constants", &results.constants);
    print_symbol_section("Enum.* References", &results.enums);
    print_symbol_section("Other Constants", &results.other_constants);
}

fn print_c_api_usage(results: &AuditResults) {
    let total_ns = results.c_api.len();
    let total_methods: usize = results.c_api.values().map(|v| v.len()).sum();
    println!(
        "=== C_* API Usage ({} namespaces, {} unique methods) ===",
        total_ns, total_methods
    );
    for (ns, methods) in &results.c_api {
        let total: usize = methods.values().map(|u| u.count).sum();
        println!("{} ({} calls)", ns, total);
        let mut sorted: Vec<(&String, &SymbolUsage)> = methods.iter().collect();
        sorted.sort_by(|a, b| b.1.count.cmp(&a.1.count));
        for (method, usage) in sorted {
            println!("  .{} ({})", method, usage.count);
        }
    }
}

fn print_symbol_section(title: &str, symbols: &BTreeMap<String, SymbolUsage>) {
    println!();
    println!("=== {} ({} unique) ===", title, symbols.len());
    for (sym, usage) in symbols {
        println!("{} ({} files)", sym, usage.files.len());
    }
}

/// Print results as JSON.
pub fn print_json(results: &AuditResults, gap: Option<&GapReport>) {
    #[derive(Serialize)]
    struct Output<'a> {
        c_api: &'a BTreeMap<String, BTreeMap<String, SymbolUsage>>,
        global_functions: &'a BTreeMap<String, SymbolUsage>,
        inherited_templates: &'a BTreeMap<String, SymbolUsage>,
        constants: &'a BTreeMap<String, SymbolUsage>,
        enums: &'a BTreeMap<String, SymbolUsage>,
        other_constants: &'a BTreeMap<String, SymbolUsage>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gap: Option<&'a GapReport>,
    }
    let out = Output {
        c_api: &results.c_api,
        global_functions: &results.global_functions,
        inherited_templates: &results.inherited_templates,
        constants: &results.constants,
        enums: &results.enums,
        other_constants: &results.other_constants,
        gap,
    };
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}

/// Print the gap report in human-readable text format.
pub fn print_gap_text(report: &GapReport) {
    println!("=== API Gap Report ===");
    println!();
    print_gap_summary("C_* Namespaces", &report.c_namespaces);
    print_gap_summary("LE_* Constants", &report.le_constants);
    print_gap_summary("Enum.* Namespaces", &report.enum_namespaces);

    print_missing_entries("Missing C_* Namespaces", &report.missing_c_namespaces, "calls");
    print_missing_entries("Missing LE_* Constants", &report.missing_le_constants, "refs");
    print_missing_entries("Missing Enum.* Namespaces", &report.missing_enum_namespaces, "refs");
    print_missing_methods_text(&report.missing_c_methods);
}

fn print_gap_summary(label: &str, summary: &GapSummary) {
    println!(
        "{}: {}/{} registered ({} missing)",
        label, summary.registered, summary.used, summary.missing
    );
}

fn print_missing_entries(title: &str, entries: &[GapEntry], unit: &str) {
    if entries.is_empty() {
        return;
    }
    println!();
    println!("--- {} ({}) ---", title, entries.len());
    for entry in entries {
        println!(
            "  {} ({} {}, {} file{})",
            entry.name,
            entry.calls,
            unit,
            entry.files.len(),
            if entry.files.len() == 1 { "" } else { "s" }
        );
    }
}

fn print_missing_methods_text(methods: &BTreeMap<String, Vec<GapEntry>>) {
    if methods.is_empty() {
        return;
    }
    let total_missing: usize = methods.values().map(|v| v.len()).sum();
    println!();
    println!(
        "--- Missing C_* Methods (by namespace, {} total) ---",
        total_missing
    );
    for (ns, entries) in methods {
        println!("{} ({} missing):", ns, entries.len());
        for entry in entries {
            println!(
                "    .{} ({} refs, {} file{})",
                entry.name,
                entry.calls,
                entry.files.len(),
                if entry.files.len() == 1 { "" } else { "s" }
            );
        }
    }
}

/// Print gap report as PLAN.md-ready markdown checkboxes.
pub fn print_gap_plan(report: &GapReport) {
    print_plan_entries("Missing C_* Namespaces", &report.missing_c_namespaces, "calls");
    print_plan_methods(&report.missing_c_methods);
    print_plan_entries("Missing LE_* Constants", &report.missing_le_constants, "refs");
    print_plan_entries("Missing Enum Namespaces", &report.missing_enum_namespaces, "refs");
}

fn print_plan_entries(title: &str, entries: &[GapEntry], unit: &str) {
    if entries.is_empty() {
        return;
    }
    println!("### {} ({})\n", title, entries.len());
    for entry in entries {
        println!("- [ ] `{}` ({} {})", entry.name, entry.calls, unit);
    }
    println!();
}

fn print_plan_methods(methods: &BTreeMap<String, Vec<GapEntry>>) {
    if methods.is_empty() {
        return;
    }
    let total: usize = methods.values().map(|v| v.len()).sum();
    println!("### Missing C_* Methods ({} total)\n", total);
    for (ns, entries) in methods {
        let method_list: Vec<String> = entries
            .iter()
            .map(|e| format!("{} ({})", e.name, e.calls))
            .collect();
        println!(
            "- [ ] `{}` ({}): {}",
            ns,
            entries.len(),
            method_list.join(", ")
        );
    }
    println!();
}
