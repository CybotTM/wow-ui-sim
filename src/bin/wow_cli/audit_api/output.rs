use serde::Serialize;
use std::collections::BTreeMap;

use super::{AuditResults, SymbolUsage};
use super::gap::GapReport;

/// Print results in human-readable text format.
pub fn print_text(results: &AuditResults) {
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

    println!();
    println!(
        "=== Global Function Calls ({} unique) ===",
        results.global_functions.len()
    );
    for (sym, usage) in &results.global_functions {
        println!("{} ({} files)", sym, usage.files.len());
    }

    println!();
    println!(
        "=== Inherited Templates ({} unique) ===",
        results.inherited_templates.len()
    );
    for (sym, usage) in &results.inherited_templates {
        println!("{} ({} files)", sym, usage.files.len());
    }

    println!();
    println!(
        "=== LE_* Constants ({} unique) ===",
        results.constants.len()
    );
    for (sym, usage) in &results.constants {
        println!("{} ({} files)", sym, usage.files.len());
    }

    println!();
    println!("=== Enum.* References ({} unique) ===", results.enums.len());
    for (sym, usage) in &results.enums {
        println!("{} ({} files)", sym, usage.files.len());
    }

    println!();
    println!(
        "=== Other Constants ({} unique) ===",
        results.other_constants.len()
    );
    for (sym, usage) in &results.other_constants {
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
    println!(
        "C_* Namespaces: {}/{} registered ({} missing)",
        report.c_namespaces.registered, report.c_namespaces.used, report.c_namespaces.missing
    );
    println!(
        "LE_* Constants: {}/{} registered ({} missing)",
        report.le_constants.registered, report.le_constants.used, report.le_constants.missing
    );
    println!(
        "Enum.* Namespaces: {}/{} registered ({} missing)",
        report.enum_namespaces.registered,
        report.enum_namespaces.used,
        report.enum_namespaces.missing
    );

    if !report.missing_c_namespaces.is_empty() {
        println!();
        println!(
            "--- Missing C_* Namespaces ({}) ---",
            report.missing_c_namespaces.len()
        );
        for entry in &report.missing_c_namespaces {
            println!(
                "  {} ({} calls, {} file{})",
                entry.name,
                entry.calls,
                entry.files.len(),
                if entry.files.len() == 1 { "" } else { "s" }
            );
        }
    }

    if !report.missing_le_constants.is_empty() {
        println!();
        println!(
            "--- Missing LE_* Constants ({}) ---",
            report.missing_le_constants.len()
        );
        for entry in &report.missing_le_constants {
            println!(
                "  {} ({} refs, {} file{})",
                entry.name,
                entry.calls,
                entry.files.len(),
                if entry.files.len() == 1 { "" } else { "s" }
            );
        }
    }

    if !report.missing_enum_namespaces.is_empty() {
        println!();
        println!(
            "--- Missing Enum.* Namespaces ({}) ---",
            report.missing_enum_namespaces.len()
        );
        for entry in &report.missing_enum_namespaces {
            println!(
                "  {} ({} refs, {} file{})",
                entry.name,
                entry.calls,
                entry.files.len(),
                if entry.files.len() == 1 { "" } else { "s" }
            );
        }
    }

    if !report.missing_c_methods.is_empty() {
        let total_missing: usize = report.missing_c_methods.values().map(|v| v.len()).sum();
        println!();
        println!(
            "--- Missing C_* Methods (by namespace, {} total) ---",
            total_missing
        );
        for (ns, methods) in &report.missing_c_methods {
            println!("{} ({} missing):", ns, methods.len());
            for entry in methods {
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
}

/// Print gap report as PLAN.md-ready markdown checkboxes.
pub fn print_gap_plan(report: &GapReport) {
    if !report.missing_c_namespaces.is_empty() {
        println!(
            "### Missing C_* Namespaces ({})\n",
            report.missing_c_namespaces.len()
        );
        for entry in &report.missing_c_namespaces {
            println!("- [ ] `{}` ({} calls)", entry.name, entry.calls);
        }
        println!();
    }

    if !report.missing_c_methods.is_empty() {
        let total: usize = report.missing_c_methods.values().map(|v| v.len()).sum();
        println!("### Missing C_* Methods ({} total)\n", total);
        for (ns, methods) in &report.missing_c_methods {
            let method_list: Vec<String> = methods
                .iter()
                .map(|e| format!("{} ({})", e.name, e.calls))
                .collect();
            println!(
                "- [ ] `{}` ({}): {}",
                ns,
                methods.len(),
                method_list.join(", ")
            );
        }
        println!();
    }

    if !report.missing_le_constants.is_empty() {
        println!(
            "### Missing LE_* Constants ({})\n",
            report.missing_le_constants.len()
        );
        for entry in &report.missing_le_constants {
            println!("- [ ] `{}` ({} refs)", entry.name, entry.calls);
        }
        println!();
    }

    if !report.missing_enum_namespaces.is_empty() {
        println!(
            "### Missing Enum Namespaces ({})\n",
            report.missing_enum_namespaces.len()
        );
        for entry in &report.missing_enum_namespaces {
            println!("- [ ] `{}` ({} refs)", entry.name, entry.calls);
        }
        println!();
    }
}
