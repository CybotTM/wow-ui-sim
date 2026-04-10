//! Static analysis tool for Blizzard UI API usage.
//!
//! Scans Blizzard UI Lua/XML files and cross-references against the simulator's
//! registered API. Useful for identifying gaps in coverage.

use regex::Regex;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};
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
    /// Path to wowless repo (for C_* namespace allowlist from apis.yaml).
    pub wowless_path: Option<PathBuf>,
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

/// Load valid C_* namespaces and their methods from wowless apis.yaml.
///
/// The apis.yaml file contains flat entries like `C_Timer.After:` at the top level.
/// Returns `(namespace_set, namespace_to_methods)`, or `None` if the file doesn't exist.
pub fn load_valid_c_namespaces(
    wowless_path: &Path,
) -> Option<(HashSet<String>, BTreeMap<String, BTreeSet<String>>)> {
    let apis_path = wowless_path.join("data/products/wow/apis.yaml");
    let content = match std::fs::read_to_string(&apis_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!(
                "Warning: wowless apis.yaml not found at {}; skipping C_* namespace filtering",
                apis_path.display()
            );
            return None;
        }
    };

    let mut namespaces: HashSet<String> = HashSet::new();
    let mut methods: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for line in content.lines() {
        // Top-level entries look like `C_Foo.Bar:` (no leading whitespace, ends with `:`)
        if !line.starts_with("C_") || !line.ends_with(':') {
            continue;
        }
        let entry = &line[..line.len() - 1]; // strip trailing `:`
        if let Some(dot) = entry.find('.') {
            let ns = entry[..dot].to_string();
            let method = entry[dot + 1..].to_string();
            namespaces.insert(ns.clone());
            methods.entry(ns).or_default().insert(method);
        }
    }

    Some((namespaces, methods))
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

    // Filter C_* results against wowless allowlist to remove false positives
    if let Some(wowless_path) = &config.wowless_path {
        if let Some((valid_ns, _)) = load_valid_c_namespaces(wowless_path) {
            let before = results.c_api.len();
            results.c_api.retain(|k, _| valid_ns.contains(k));
            let filtered = before - results.c_api.len();
            if filtered > 0 {
                eprintln!(
                    "Filtered {} false-positive C_* namespace(s) using wowless allowlist ({} kept)",
                    filtered,
                    results.c_api.len()
                );
            }
        }
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
pub fn print_json(results: &AuditResults, gap: Option<&GapReport>) {
    #[derive(Serialize)]
    struct Output<'a> {
        c_api: &'a BTreeMap<String, BTreeMap<String, SymbolUsage>>,
        constants: &'a BTreeMap<String, SymbolUsage>,
        enums: &'a BTreeMap<String, SymbolUsage>,
        other_constants: &'a BTreeMap<String, SymbolUsage>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gap: Option<&'a GapReport>,
    }
    let out = Output {
        c_api: &results.c_api,
        constants: &results.constants,
        enums: &results.enums,
        other_constants: &results.other_constants,
        gap,
    };
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}

// ── Simulator source scanning ────────────────────────────────────────

/// What the simulator has registered, collected from Rust source files.
#[derive(Debug, Default, Serialize)]
pub struct SimRegistered {
    /// C_* namespace names registered via `.set("C_Name", ...)`.
    pub c_namespaces: BTreeSet<String>,
    /// LE_* constant names found in Rust source + Lua data files.
    pub le_constants: BTreeSet<String>,
    /// Enum namespace names registered (e.g. "Enum.SpellBookItemType").
    pub enum_namespaces: BTreeSet<String>,
}

/// A single missing symbol with its usage count.
#[derive(Debug, Serialize)]
pub struct GapEntry {
    pub name: String,
    pub calls: usize,
}

/// Summary counts for one symbol family.
#[derive(Debug, Serialize)]
pub struct GapSummary {
    pub registered: usize,
    pub used: usize,
    pub missing: usize,
}

/// Full gap report comparing Blizzard UI usage against simulator registrations.
#[derive(Debug, Serialize)]
pub struct GapReport {
    pub c_namespaces: GapSummary,
    pub le_constants: GapSummary,
    pub enum_namespaces: GapSummary,
    pub missing_c_namespaces: Vec<GapEntry>,
    pub missing_le_constants: Vec<GapEntry>,
    pub missing_enum_namespaces: Vec<GapEntry>,
}

/// Scan the simulator Rust source tree and Lua data files to collect registered symbols.
pub fn scan_simulator(src_path: &Path) -> SimRegistered {
    let mut reg = SimRegistered::default();

    let set_c_re = Regex::new(r#"\.set\("(C_[A-Za-z][A-Za-z0-9_]*)""#).unwrap();
    let le_rs_re = Regex::new(r#""(LE_[A-Z][A-Z0-9_]+)""#).unwrap();
    let le_lua_re = Regex::new(r"LE_[A-Z][A-Z0-9_]+").unwrap();
    let enum_rs_re = Regex::new(r#"\("([A-Z][A-Za-z0-9]+)",\s*&\["#).unwrap();
    let enum_rs_explicit_re = Regex::new(r#"\("([A-Z][A-Za-z0-9]+)",\s*&\["#).unwrap();
    let enum_lua_block_re = Regex::new(r"Enum\.([A-Za-z][A-Za-z0-9]+)\s*=\s*\{").unwrap();

    for entry in WalkDir::new(src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        match ext {
            "rs" => {
                let content = match std::fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                for cap in set_c_re.captures_iter(&content) {
                    reg.c_namespaces.insert(cap[1].to_string());
                }
                for cap in le_rs_re.captures_iter(&content) {
                    reg.le_constants.insert(cap[1].to_string());
                }
                // Enum names from SeqEnumDef/EnumDef tuple patterns
                for cap in enum_rs_re.captures_iter(&content) {
                    reg.enum_namespaces.insert(format!("Enum.{}", &cap[1]));
                }
                let _ = enum_rs_explicit_re; // same regex used above
            }
            "lua" => {
                let content = match std::fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                // LE_* from missing_constants.lua and similar
                for cap in le_lua_re.find_iter(&content) {
                    reg.le_constants.insert(cap.as_str().to_string());
                }
                // Enum.* namespaces from missing_enums.lua
                for cap in enum_lua_block_re.captures_iter(&content) {
                    reg.enum_namespaces.insert(format!("Enum.{}", &cap[1]));
                }
            }
            _ => {}
        }
    }

    reg
}

/// Build a gap report comparing what Blizzard UI uses vs what the simulator registers.
pub fn build_gap_report(used: &AuditResults, registered: &SimRegistered) -> GapReport {
    // C_* namespaces: compare namespace keys from usage
    let used_c: BTreeSet<String> = used.c_api.keys().cloned().collect();
    let missing_c = collect_missing_entries(&used_c, &registered.c_namespaces, |ns| {
        used.c_api
            .get(ns)
            .map(|m| m.values().map(|u| u.count).sum())
            .unwrap_or(0)
    });

    // LE_* constants
    let used_le: BTreeSet<String> = used.constants.keys().cloned().collect();
    let missing_le = collect_missing_entries(&used_le, &registered.le_constants, |sym| {
        used.constants.get(sym).map(|u| u.count).unwrap_or(0)
    });

    // Enum namespaces: extract "Enum.X" from "Enum.X.Y" usage keys
    let used_enum_ns: BTreeSet<String> = used
        .enums
        .keys()
        .map(|k| {
            // "Enum.Foo.Bar" -> "Enum.Foo"
            let parts: Vec<&str> = k.splitn(3, '.').collect();
            if parts.len() >= 2 {
                format!("Enum.{}", parts[1])
            } else {
                k.clone()
            }
        })
        .collect();
    let missing_enum = collect_missing_entries(&used_enum_ns, &registered.enum_namespaces, |ns| {
        // sum all calls to any Enum.Namespace.* key
        let prefix = format!("{}.", ns);
        used.enums
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix) || *k == ns)
            .map(|(_, u)| u.count)
            .sum()
    });

    GapReport {
        c_namespaces: GapSummary {
            registered: registered.c_namespaces.len(),
            used: used_c.len(),
            missing: missing_c.len(),
        },
        le_constants: GapSummary {
            registered: registered.le_constants.len(),
            used: used_le.len(),
            missing: missing_le.len(),
        },
        enum_namespaces: GapSummary {
            registered: registered.enum_namespaces.len(),
            used: used_enum_ns.len(),
            missing: missing_enum.len(),
        },
        missing_c_namespaces: missing_c,
        missing_le_constants: missing_le,
        missing_enum_namespaces: missing_enum,
    }
}

fn collect_missing_entries(
    used: &BTreeSet<String>,
    registered: &BTreeSet<String>,
    call_count: impl Fn(&str) -> usize,
) -> Vec<GapEntry> {
    let mut entries: Vec<GapEntry> = used
        .iter()
        .filter(|name| !registered.contains(*name))
        .map(|name| GapEntry {
            name: name.clone(),
            calls: call_count(name),
        })
        .collect();
    entries.sort_by(|a, b| b.calls.cmp(&a.calls).then(a.name.cmp(&b.name)));
    entries
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
            println!("  {} ({} calls)", entry.name, entry.calls);
        }
    }

    if !report.missing_le_constants.is_empty() {
        println!();
        println!(
            "--- Missing LE_* Constants ({}) ---",
            report.missing_le_constants.len()
        );
        for entry in &report.missing_le_constants {
            println!("  {} ({} refs)", entry.name, entry.calls);
        }
    }

    if !report.missing_enum_namespaces.is_empty() {
        println!();
        println!(
            "--- Missing Enum.* Namespaces ({}) ---",
            report.missing_enum_namespaces.len()
        );
        for entry in &report.missing_enum_namespaces {
            println!("  {} ({} refs)", entry.name, entry.calls);
        }
    }
}
