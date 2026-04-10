//! Static analysis tool for Blizzard UI API usage.
//!
//! Scans Blizzard UI Lua/XML files and cross-references against the simulator's
//! registered API. Useful for identifying gaps in coverage.

use regex::Regex;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Per-symbol occurrence data.
#[derive(Debug, Default, Serialize)]
pub struct SymbolUsage {
    pub count: usize,
    pub files: Vec<String>,
}

/// Collected results from scanning Blizzard UI sources.
#[derive(Debug, Default, Serialize)]
pub struct AuditResults {
    /// C_Namespace -> method -> usage
    pub c_api: BTreeMap<String, BTreeMap<String, SymbolUsage>>,
    /// LE_* constants
    pub constants: BTreeMap<String, SymbolUsage>,
    /// Enum.Namespace.Value references
    pub enums: BTreeMap<String, SymbolUsage>,
    /// Other constant families (ITEM_MOD_, MAX_, etc.)
    pub other_constants: BTreeMap<String, SymbolUsage>,
}

/// Configuration for the audit command.
pub struct AuditConfig {
    pub ui_path: PathBuf,
    pub namespace_filter: Option<String>,
    pub filter_startup: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Text,
    Json,
}

/// Script element tag names whose text content is inline Lua.
const SCRIPT_TAGS: &[&str] = &[
    "OnLoad", "OnClick", "OnShow", "OnHide", "OnEvent", "OnUpdate", "OnEnter", "OnLeave",
];

/// All compiled regex patterns (built once, reused across all files).
struct Patterns {
    c_api: Regex,
    le_const: Regex,
    enum_ref: Regex,
    other_const: Regex,
    line_comment: Regex,
    block_comment: Regex,
    kv_global: Regex,
    /// One regex per script tag (no backreference support in the `regex` crate).
    xml_script_tags: Vec<Regex>,
}

impl Patterns {
    fn new() -> Self {
        let xml_script_tags = SCRIPT_TAGS
            .iter()
            .map(|tag| Regex::new(&format!(r"(?s)<{tag}[^>]*>(.*?)</{tag}>")).unwrap())
            .collect();
        Self {
            c_api: Regex::new(r"(C_\w+)[.:](\w+)").unwrap(),
            le_const: Regex::new(r"\bLE_\w+").unwrap(),
            enum_ref: Regex::new(r"\bEnum\.(\w+\.\w+)").unwrap(),
            other_const: Regex::new(r"\b(ITEM_MOD|MAX|NUM|SPELL_SCHOOL|RAID_CLASS|CLASS_SORT)_\w+")
                .unwrap(),
            line_comment: Regex::new(r"--[^\n]*").unwrap(),
            block_comment: Regex::new(r"(?s)--\[\[.*?\]\]").unwrap(),
            kv_global: Regex::new(r#"type="global"[^>]*value="([^"]+)""#).unwrap(),
            xml_script_tags,
        }
    }
}

/// Strip Lua comments from source text using pre-compiled patterns.
fn strip_comments<'a>(src: &'a str, p: &Patterns) -> std::borrow::Cow<'a, str> {
    let without_blocks = p.block_comment.replace_all(src, "");
    // replace_all returns Cow; if no block comments, it borrows — turn into owned for
    // the second pass to avoid lifetime issues.
    let without_blocks = without_blocks.into_owned();
    p.line_comment
        .replace_all(&without_blocks, "")
        .into_owned()
        .into()
}

fn record_symbol_usage(usage: &mut SymbolUsage, file_label: &str) {
    usage.count += 1;
    if !usage.files.iter().any(|f| f == file_label) {
        usage.files.push(file_label.to_string());
    }
}

fn scan_c_api_usage(clean: &str, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    for cap in p.c_api.captures_iter(clean) {
        let ns = cap[1].to_string();
        let method = cap[2].to_string();
        let usage = results
            .c_api
            .entry(ns)
            .or_default()
            .entry(method)
            .or_default();
        record_symbol_usage(usage, file_label);
    }
}

fn scan_constant_usage(clean: &str, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    for cap in p.le_const.captures_iter(clean) {
        let usage = results.constants.entry(cap[0].to_string()).or_default();
        record_symbol_usage(usage, file_label);
    }
}

fn scan_enum_usage(clean: &str, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    for cap in p.enum_ref.captures_iter(clean) {
        let sym = format!("Enum.{}", &cap[1]);
        let usage = results.enums.entry(sym).or_default();
        record_symbol_usage(usage, file_label);
    }
}

fn scan_other_constant_usage(
    clean: &str,
    file_label: &str,
    results: &mut AuditResults,
    p: &Patterns,
) {
    for cap in p.other_const.captures_iter(clean) {
        let usage = results
            .other_constants
            .entry(cap[0].to_string())
            .or_default();
        record_symbol_usage(usage, file_label);
    }
}

/// Scan a chunk of Lua source text and accumulate results.
fn scan_lua_text(text: &str, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    let clean = strip_comments(text, p);
    scan_c_api_usage(&clean, file_label, results, p);
    scan_constant_usage(&clean, file_label, results, p);
    scan_enum_usage(&clean, file_label, results, p);
    scan_other_constant_usage(&clean, file_label, results, p);
}

/// Extract inline Lua from XML script elements and scan them.
fn scan_xml_file(path: &Path, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    // Extract KeyValue globals: type="global" value="SOME_CONSTANT"
    for cap in p.kv_global.captures_iter(&content) {
        let usage = results
            .other_constants
            .entry(cap[1].to_string())
            .or_default();
        record_symbol_usage(usage, file_label);
    }

    // Extract inline script bodies
    for re in &p.xml_script_tags {
        for cap in re.captures_iter(&content) {
            scan_lua_text(&cap[1], file_label, results, p);
        }
    }
}

/// Whether to skip a directory (test suites we don't want to scan).
fn should_skip(path: &Path) -> bool {
    let s = path.to_string_lossy();
    s.contains("Interface/AddOns/Wowless") || s.contains("Interface/AddOns/WowlessData")
}

/// Whether to skip a file because it's LoadOnDemand (when filter_startup is on).
fn is_load_on_demand(addon_path: &Path, addon_dir: &Path) -> bool {
    let toc = addon_path
        .ancestors()
        .find(|p| p.parent() == Some(addon_dir))
        .and_then(|addon| {
            let name = addon.file_name()?;
            Some(addon.join(format!("{}.toc", name.to_string_lossy())))
        });

    if let Some(toc_path) = toc {
        if let Ok(toc_content) = std::fs::read_to_string(&toc_path) {
            return toc_content
                .lines()
                .any(|line| line.trim().eq_ignore_ascii_case("## LoadOnDemand: 1"));
        }
    }
    false
}

/// Run the audit and return results.
pub fn run_audit(config: &AuditConfig) -> AuditResults {
    let p = Patterns::new();
    let mut results = AuditResults::default();
    let addon_dir = &config.ui_path;

    for entry in WalkDir::new(addon_dir)
        .into_iter()
        .filter_entry(|e| !should_skip(e.path()))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "lua" && ext != "xml" {
            continue;
        }

        if config.filter_startup && is_load_on_demand(path, addon_dir) {
            continue;
        }

        let file_label = path
            .strip_prefix(addon_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        match ext {
            "lua" => {
                if let Ok(content) = std::fs::read_to_string(path) {
                    scan_lua_text(&content, &file_label, &mut results, &p);
                }
            }
            "xml" => {
                scan_xml_file(path, &file_label, &mut results, &p);
            }
            _ => {}
        }
    }

    // Apply namespace filter if requested
    if let Some(ns) = &config.namespace_filter {
        results.c_api.retain(|k, _| k == ns);
    }

    results
}

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
pub fn print_json(results: &AuditResults) {
    #[derive(Serialize)]
    struct Output<'a> {
        c_api: &'a BTreeMap<String, BTreeMap<String, SymbolUsage>>,
        constants: &'a BTreeMap<String, SymbolUsage>,
        enums: &'a BTreeMap<String, SymbolUsage>,
        other_constants: &'a BTreeMap<String, SymbolUsage>,
    }
    let out = Output {
        c_api: &results.c_api,
        constants: &results.constants,
        enums: &results.enums,
        other_constants: &results.other_constants,
    };
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
